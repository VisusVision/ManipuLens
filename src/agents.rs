use crate::types::AgentAnalysis;
use serde_json::json;
use std::sync::OnceLock;

/// Ollama sunucu adresi: OLLAMA_URL env değişkeni ile değiştirilebilir
/// (Docker içinde http://host.docker.internal:11434 gerekir).
pub fn ollama_url() -> String {
    std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".to_string())
}

/// Tek paylaşımlı HTTP client: her istekte yeni bağlantı havuzu kurmayı önler.
pub fn http_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(reqwest::Client::new)
}

fn output_language(lang: &str) -> &'static str {
    if lang == "en" { "English" } else { "Turkish" }
}

/// Ortak kurallar — kısa tutuldu: uzun promptlar 7 LLM çağrısında ciddi
/// gecikme yaratıyor. target_sentences'ın BİREBİR kopyalanması kritik:
/// eklenti bu cümleleri sayfada exact-match ile arayıp vurguluyor.
fn shared_rules(type_name: &str, out_lang: &str) -> String {
    format!(
        r#"
RULES:
1. Judge ONLY the given text. Be conservative: if in doubt, detected = false.
2. "target_sentences": copy sentences VERBATIM from the input, character-for-character. Empty array [] if detected = false.
3. "aciklama": max 2 plain sentences for an everyday reader. LANGUAGE: "aciklama" MUST be written in {out_lang}. This is mandatory even if the input text is in a different language - do NOT mirror the input's language, ALWAYS answer in {out_lang}.
4. confidence_score: 0.90+ unmistakable | 0.75+ clear | 0.60+ probable | below 0.60 -> set detected = false.
5. Output ONLY one valid JSON object, no markdown, no extra text:
{{"manipulation_type":"{type_name}","detected":true|false,"confidence_score":0.0,"aciklama":"...","target_sentences":[]}}"#
    )
}

async fn call_ollama_agent(system_prompt: &str, user_text: &str) -> Result<AgentAnalysis, String> {
    let payload = json!({
        "model": "llama3",
        "system": system_prompt,
        "prompt": user_text,
        "stream": false,
        "format": "json",
        // Model 30 dk boyunca VRAM'de sıcak kalsın; tekrar yükleme gecikmesi olmasın.
        "keep_alive": "30m",
        "options": {
            "temperature": 0.2,
            "top_p": 0.9
        }
    });

    let response = http_client()
        .post(format!("{}/api/generate", ollama_url()))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let res_body: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

        if let Some(response_str) = res_body.get("response").and_then(|r| r.as_str()) {
            let analysis: AgentAnalysis =
                serde_json::from_str(response_str).map_err(|e| e.to_string())?;
            return Ok(analysis);
        }
    }

    Err("Ollama'dan geçerli bir yanıt alınamadı.".to_string())
}

pub async fn analyze_linguistic(text: &str, lang: &str) -> Result<AgentAnalysis, String> {
    let out_lang = output_language(lang);
    let prompt = format!(
        r#"You are a LINGUISTIC MANIPULATION analyst: deception through word choice itself.

DETECT only: weasel words dodging accountability ("experts say"); loaded wording smuggling judgments as facts; euphemisms hiding reality; shifting a key word's meaning mid-argument; presupposition traps.
NOT manipulation: ordinary persuasion, strong honest opinions, satire, news/academic tone, simple exaggeration.
TEST: would neutral wording of the same facts change the reader's belief? If not -> detected = false.
{shared}"#,
        shared = shared_rules("Dilsel", out_lang)
    );
    call_ollama_agent(&prompt, text).await
}

pub async fn analyze_psychological(text: &str, lang: &str) -> Result<AgentAnalysis, String> {
    let out_lang = output_language(lang);
    let prompt = format!(
        r#"You are a PSYCHOLOGICAL MANIPULATION analyst: emotional coercion aimed at controlling the reader.

DETECT only: gaslighting (making readers doubt their own perception); guilt-tripping; fear-mongering beyond evidence; emotional blackmail; manufactured shame/inadequacy to sell a "fix".
NOT manipulation: honest warnings, motivational language, expressed concern, ordinary empathy appeals.
TEST: is emotion weaponized to bypass rational judgment rather than honestly inform? If not -> detected = false.
{shared}"#,
        shared = shared_rules("Psikolojik", out_lang)
    );
    call_ollama_agent(&prompt, text).await
}

pub async fn analyze_behavioral(text: &str, lang: &str) -> Result<AgentAnalysis, String> {
    let out_lang = output_language(lang);
    let prompt = format!(
        r#"You are a BEHAVIORAL MANIPULATION analyst: artificial pressure engineered to trigger impulsive action.

DETECT only: fake scarcity ("Only 3 left!") with no verifiable basis; artificial deadlines/countdowns that exist purely to prevent deliberation; FOMO engineering; stacked act-now pressure loops.
NOT manipulation: ordinary calls to action, genuine verifiable time limits, informational deadlines.
TEST: does the urgency exist only to stop the reader from thinking? If not -> detected = false.
{shared}"#,
        shared = shared_rules("Davranışsal", out_lang)
    );
    call_ollama_agent(&prompt, text).await
}

pub async fn analyze_perceptual(text: &str, lang: &str) -> Result<AgentAnalysis, String> {
    let out_lang = output_language(lang);
    let prompt = format!(
        r#"You are a PERCEPTUAL MANIPULATION analyst: framing and selective information distorting the reader's picture of reality.

DETECT only: cherry-picked data; omitted context that reverses a claim's meaning; technically-true-but-misleading framing; statistical distortion (no baseline, cropped scales); false dichotomy.
NOT manipulation: one-sided but honest advocacy, simplified explanations, merely incomplete informative text.
TEST: is information curated so the reader reliably reaches a FALSE conclusion? Incompleteness alone is not enough -> detected = false.
{shared}"#,
        shared = shared_rules("Algısal", out_lang)
    );
    call_ollama_agent(&prompt, text).await
}

pub async fn analyze_social(text: &str, lang: &str) -> Result<AgentAnalysis, String> {
    let out_lang = output_language(lang);
    let prompt = format!(
        r#"You are a SOCIAL MANIPULATION analyst: deceptive use of group pressure and identity.

DETECT only: bandwagon coercion ("everyone already does this") used to shame; fabricated/unverifiable social proof; us-vs-them polarization framing disagreement as betrayal; false consensus on contested claims.
NOT manipulation: plausible popularity claims ("best-selling"), community/belonging language without coercion.
TEST: is group belonging or social fear substituting for evidence? If not -> detected = false.
{shared}"#,
        shared = shared_rules("Sosyal", out_lang)
    );
    call_ollama_agent(&prompt, text).await
}

pub async fn analyze_marketing(text: &str, lang: &str) -> Result<AgentAnalysis, String> {
    let out_lang = output_language(lang);
    // UI "Satın Alma Eğilimi Tahmini" paneli bu iskelet cümleye bağlı
    // (orchestrator aciklama'yı predicted_product'a kopyalıyor) — iskelet
    // değişmez, ama [X] slotu metindeki SPESİFİK hedefle doldurulmak zorunda.
    let aciklama_template = if lang == "en" {
        r#"If detected = true, "aciklama" MUST be exactly one sentence of the form "The reader may be inclined to purchase or turn toward X." where you replace X with the SPECIFIC product, service, brand or sector THIS text pushes - take the name from the text itself. NEVER write generic fillers like "a product" or "a service"; if the text names no product, use the narrowest sector it implies. If detected = false, write one short neutral English sentence saying no manipulative commercial push was found."#
    } else {
        r#"If detected = true, "aciklama" MUST be exactly one sentence of the form "Kişi X satın almaya veya yönelmeye meyilli olabilir." where you replace X with the SPECIFIC product, service, brand or sector THIS text pushes - take the name from the text itself. NEVER write generic fillers like "bir ürün" or "bir hizmet"; if the text names no product, use the narrowest sector it implies. If detected = false, write one short neutral Turkish sentence saying no manipulative commercial push was found."#
    };

    let prompt = format!(
        r#"You are a CONSUMER MANIPULATION & COMMERCIAL INTENT analyst: text that covertly engineers purchase desire instead of honestly informing a buying decision.

Decide in this exact order:

STEP 1 - HONESTY CHECK (do this FIRST): if the text mentions ANY flaw, downside or limitation of the product, OR advises comparing alternatives before buying, OR is a clearly labelled ad, OR is neutral market/price news, then it is honest -> detected = false, confidence_score = 0.0. STOP, skip STEP 2.

STEP 2 - only if STEP 1 did not fire, DETECT tactics serving a COMMERCIAL goal (selling a product/service/brand/sector):
1. Disguised advertising: editorial, news or personal-story tone hiding a sales agenda; hidden affiliate promotion; astroturfing (fake "ordinary user" praise).
2. Problem inflation: inventing or exaggerating a problem/inadequacy so the promoted product becomes the necessary "solution".
3. Miracle claims: guaranteed, effortless or scientifically implausible results ("lose 10 kg in a week", "double your money in a month").
4. Purchase pressure: fake scarcity or countdowns tied to buying; buy-now FOMO; deceptive price anchoring ("was 5000, today only 499"); fabricated testimonials or invented user counts pushing a sale.
5. Covert steering: repeatedly nudging the reader toward one specific product, brand or sector without declaring the commercial interest.
Mere mention of a product is NOT manipulation. TEST: does the text CREATE purchase desire through deception or pressure, rather than honestly inform? If it honestly informs -> detected = false.
CONFIDENCE: 0.90+ several tactics plus an unmistakable hidden sales agenda | 0.75+ one clear tactic aimed at a purchase | 0.60+ probable covert commercial steering | anything weaker -> detected = false.

CALIBRATION (decisions only, never copy their wording):
- Personal story praising one product as life-changing + discount countdown + "stocks running out" -> detected = true, 0.90.
- Review listing both strengths and weaknesses, suggesting to compare alternatives -> detected = false (STEP 1).
- News article reporting price changes in a market -> detected = false (STEP 1).

{aciklama_template}
{shared}"#,
        shared = shared_rules("Pazarlama", out_lang)
    );
    call_ollama_agent(&prompt, text).await
}
