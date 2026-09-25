use crate::types::{AgentAnalysis, DemographicInference};
use serde_json::json;
use std::sync::OnceLock;

pub fn azure_openai_endpoint() -> Result<String, String> {
    std::env::var("AZURE_OPENAI_ENDPOINT")
        .map_err(|_| "AZURE_OPENAI_ENDPOINT ortam değişkeni tanımlı değil.".to_string())
}

pub fn azure_openai_api_key() -> Result<String, String> {
    std::env::var("AZURE_OPENAI_API_KEY")
        .map_err(|_| "AZURE_OPENAI_API_KEY ortam değişkeni tanımlı değil.".to_string())
}

pub fn azure_openai_deployment() -> String {
    std::env::var("AZURE_OPENAI_DEPLOYMENT")
        .unwrap_or_else(|_| "manipulens-gpt-5-mini".to_string())
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
///
/// STEP 0 dışlama kapısı ve kalibrasyon örnekleri 2026-09-03 ölçümünden sonra
/// eklendi: 12 metinlik etiketli sette temiz metinlerin %57'si manipülatif
/// sayılıyordu. Tek istisna, promptunda zaten bir dışlama adımı ve örnekleri
/// bulunan Pazarlama ajanıydı (temiz metinlerde 0/7 yanlış alarm). Küçük model
/// "manipülasyon analisti" rolünü verilen her metinde bulgu üretmek diye
/// okuyor; rolü dengeleyen açık bir "hayır" yolu olmadan tarafsız kalamıyor.
fn shared_rules(type_name: &str, out_lang: &str) -> String {
    format!(
        r#"
STEP 0 - EXCLUSION CHECK, do this BEFORE anything else. If the text is any of the following, it is NOT manipulation: reference or encyclopedic facts (geography, science, history, statistics stated plainly); instructions or a recipe; a weather, traffic or sports report; an official announcement, schedule or regulation; a balanced review that also names downsides or advises comparing alternatives; someone describing their own day, feelings or beliefs without pressuring anyone. In ALL these cases output detected = false, confidence_score = 0.0 and STOP - do not look for your manipulation type at all.

A text is not manipulative merely because it is short, incomplete, one-sided, emotional, religious, angry, or about an upsetting subject. Absence of a tactic is the NORMAL case: most texts a person reads are not manipulative. Reporting "no manipulation found" is a correct and valuable answer, not a failure to do your job.

RULES:
1. Judge ONLY the given text. Be conservative: if in doubt, detected = false.
2. "target_sentences": copy sentences VERBATIM from the input, character-for-character. Empty array [] if detected = false. If you cannot point to a specific sentence that carries the tactic, then the tactic is not there -> detected = false.
3. "aciklama": max 2 plain sentences for an everyday reader. LANGUAGE: "aciklama" MUST be written in {out_lang}. This is mandatory even if the input text is in a different language - do NOT mirror the input's language, ALWAYS answer in {out_lang}.
4. confidence_score: 0.90+ unmistakable | 0.75+ clear | 0.60+ probable | below 0.60 -> set detected = false.
5. Output ONLY one valid JSON object, no markdown, no extra text:
{{"manipulation_type":"{type_name}","detected":true|false,"confidence_score":0.0,"aciklama":"...","target_sentences":[]}}"#
    )
}

/// Satış kopyası sinyal grupları. Tek başına hiçbiri yeterli değil: haber de
/// "indirim" yazar. En az İKİ ayrı grup eşleşirse metin reklam kalıbı sayılır.
const SALES_SIGNAL_GROUPS: [&[&str]; 5] = [
    // Aciliyet
    &[
        "son saat", "son gun", "son sans", "son firsat", "acele", "hemen",
        "sadece bugun", "bugune ozel", "sure doluyor", "kacirma", "gec kalma",
        "geride kalma", "last chance", "hurry", "only today", "ends today", "act now",
    ],
    // Kıtlık. Burada "kontenjan/doluyor/randevu" denendi ve geri alındı: ikinci
    // sette (kapi-dogrulama-seti-2.txt) tek bir reklamı yakalamadılar, buna
    // karşılık belediyenin ücretsiz kurs duyurusunda boşuna tetiklendiler.
    // Genel kelimeleri listeye koymak sızıntı üretiyor; gizlenmiş reklamı
    // `commercial_intent_gate` yakalıyor.
    &[
        "stok", "tukeniyor", "tukendi", "sinirli sayida", "son adet", "kalmadi",
        "limited stock", "running out", "sold out",
    ],
    // Fiyat / kampanya. "indirim kodu/ozel fiyat/yarim fiyat": reklam kendini
    // kişisel hikâye ya da inceleme gibi gösterse de dönüşüm cümlesi kalıyor.
    &[
        "indirim", "kampanya", "ucretsiz kargo", "taksit", "bedava", "hediye",
        "firsat fiyati", "indirim kodu", "kupon kodu", "ozel fiyat", "yarim fiyat",
        "kargo bedava", "discount", "free shipping", "sale price", "% off",
    ],
    // Eyleme çağrı. Sosyal medya reklamında çağrı "satın al" değil "profildeki
    // koda bak", "linki aşağıda" biçiminde geliyor.
    &[
        "hemen al", "satin al", "siparis", "tikla", "kayit ol", "uye ol",
        "abone ol", "profilde", "profilim", "linki asagi", "asagidaki link",
        "buy now", "order now", "click here", "sign up", "shop now", "link in bio",
    ],
    // Sosyal kanıt
    &[
        "herkes al", "herkes kullan", "binlerce kisi", "milyonlarca kisi",
        "everyone is", "thousands of people",
    ],
];

/// Türkçe metni sinyal taramasına uygun sade biçime indirger: küçük harf +
/// aksansız. `to_lowercase()` tek başına yetmez ("İ" nokta bırakıyor) ve
/// kullanıcılar zaten "kacirma/kaçırma" diye karışık yazıyor.
fn normalize_for_signals(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| match c {
            'ı' | 'i' | 'İ' | '\u{0307}' => 'i',
            'ş' => 's',
            'ğ' => 'g',
            'ü' => 'u',
            'ö' => 'o',
            'ç' => 'c',
            'â' => 'a',
            other => other,
        })
        .collect()
}

/// Metin reklam/satış kopyası kalıbı taşıyor mu? En az iki farklı sinyal
/// grubu ya da yüzdelik indirim ifadesi + tek grup aranır; tek sinyalle
/// "ikna" demek market haberlerini de içeri alırdı.
pub(crate) fn looks_like_sales_copy(text: &str) -> bool {
    let t = normalize_for_signals(text);

    let mut groups = SALES_SIGNAL_GROUPS
        .iter()
        .filter(|group| group.iter().any(|needle| t.contains(needle)))
        .count();

    // "Son 3 saat", "son 2 gun": araya sayı giren aciliyet kalıbı yukarıdaki
    // düz eşleşmeye takılmaz.
    if t.split_whitespace().collect::<Vec<_>>().windows(3).any(|w| {
        w[0] == "son"
            && w[1].chars().all(|c| c.is_ascii_digit())
            && (w[2].starts_with("saat") || w[2].starts_with("gun") || w[2].starts_with("dakika"))
    }) {
        groups += 1;
    }

    // "%90 indirim" / "90% off": yüzde işareti sayıya bitişikse fiyat sinyali.
    let has_percent_offer = t.match_indices('%').any(|(i, _)| {
        let before = t[..i].chars().last().is_some_and(|c| c.is_ascii_digit());
        let after = t[i + 1..].chars().next().is_some_and(|c| c.is_ascii_digit());
        before || after
    });
    if has_percent_offer {
        groups += 1;
    }

    groups >= 2
}

/// Gaslighting / suçlama kalıpları. Satış sinyallerinin aksine tek eşleşme
/// yeter: bu kalıplar zaten muhatabın algısını geçersiz kılmaya dönük ve
/// günlük bilgi metninde geçmiyor.
const PERSONAL_PRESSURE_MARKERS: [&str; 14] = [
    "sen hep",
    "sen hic",
    "hep sen",
    "senin yuzunden",
    "sorun sende",
    "suc sende",
    "abartiyorsun",
    "abartiyorsunuz",
    "oyle bir sey demedim",
    "oyle demedim",
    "yanlis hatirliyorsun",
    "kafanda kurmus",
    "you always",
    "the problem is you",
];

/// Metin, muhatabını suçlayan veya hafızasını geçersiz kılan bir baskı kalıbı
/// taşıyor mu? Taşıyorsa tür sınıflandırıcısına güvenilmez, tam analiz koşar.
pub(crate) fn looks_like_personal_pressure(text: &str) -> bool {
    let t = normalize_for_signals(text);
    PERSONAL_PRESSURE_MARKERS.iter().any(|m| t.contains(m))
}

/// Metnin tam analize hangi kapıdan girdiği. Girişin gerekçesi sonradan
/// lazım: en zayıf giriş (`Intent`) "bu metin gizli reklam" iddiasıyla
/// açılıyor, o iddiayı Pazarlama ajanı doğrulamazsa rapor ayakta kalmamalı.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateEntry {
    /// Kapı metni eledi: uzman ajanlar hiç çalışmaz.
    Clean,
    /// Kural katmanı eşleşti (satış kopyası ya da kişiye yönelen baskı).
    ByRule,
    /// Tür sınıflandırıcısı "ikna" dedi.
    ByGenre,
    /// Tür sorusu eledi, ticari amaç sorusu geri aldı - en zayıf giriş.
    ByIntent,
}

/// ÖN ELEME (TRIAGE) — metnin türünü belirler, uzman ajanlar koşmadan önce.
///
/// Neden ayrı bir çağrı: llama3 8B, "manipülasyon analisti" rolü verilince
/// uzun bir dışlama listesini okusa bile her metinde bulgu üretiyor. 2026-09-03
/// ölçümünde ansiklopedi maddesi, bilimsel tanım ve dengeli ürün incelemesi
/// prompt içindeki STEP 0 kapısına rağmen manipülatif sayıldı.
///
/// Tür sınıflandırması küçük modelin gerçekten yapabildiği bir iş: "bu metin
/// okuyucuyu bir şeye ikna etmeye mi çalışıyor, yoksa bilgi mi veriyor?"
/// Rol yüklü değil, incelik gerektirmiyor. İkna amacı yoksa altı uzman ajan
/// hiç çağrılmaz — hem yanlış alarm kesilir hem de temiz metinlerde analiz
/// yedi çağrı yerine tek çağrıya iner.
///
/// Üç kademe, en ucuzdan pahalıya: kural katmanı (LLM yok), tür sorusu ve
/// ticari amaç sorusu. `Clean` dönerse uzman ajanlar hiç çağrılmaz; diğer üç
/// değer metnin hangi gerekçeyle içeri girdiğini söyler.
pub async fn gate_decision(text: &str) -> Result<GateEntry, String> {
    // Satış kopyası kalıbı varsa modele hiç sormadan tam analize geç. 2026-09-12
    // ölçümünde llama3, "Son 3 saat! Herkes aldı, stoklar bitiyor." gibi kısa
    // reklam metnini "rapor" sayıp elemişti; reklamlar kısa olduğu için kapı
    // tam da hedef kitleyi kaçırıyordu. Desen eşleşirse kapı atlanır, bu da
    // bir Azure OpenAI çağrısı tasarrufudur — kapı tek hata noktası olmaktan çıkar.
    if looks_like_sales_copy(text) {
        tracing::debug!("ön eleme: satış kopyası deseni, kapı atlandı");
        return Ok(GateEntry::ByRule);
    }

    // Aynı gerekçe kişiye yönelen baskı için: llama3, "Sen hep abartıyorsun,
    // öyle bir şey demedim, sorun sende." cümlesine prompt'ta birebir örneği
    // dursa bile "kisisel" diyor (2026-09-12 ölçümü, Türkçe). Gaslighting
    // kalıbı yakalandığında kapı atlanır; kararı uzman ajanlar verir.
    if looks_like_personal_pressure(text) {
        tracing::debug!("ön eleme: kişiye yönelen baskı deseni, kapı atlandı");
        return Ok(GateEntry::ByRule);
    }

    if llm_genre_gate(text).await? {
        return Ok(GateEntry::ByGenre);
    }

    // Tür sınıflandırıcısı "haber/kişisel/duyuru" deyip elediyse ikinci ve
    // farklı bir soru sorulur: bu metin yazarın kendi sunduğu bir şeye mi
    // yönlendiriyor? 2026-09-12 kapı ölçümünde (kapi-dogrulama-seti-2.txt)
    // gizlenmiş reklamların 8/12'si tür sorusunda eleniyordu; kalıp temelli
    // kural katmanı bunların hiçbirini yakalayamadı, çünkü her reklam dönüşüm
    // cümlesini farklı kuruyor. Amaç sorusu kalıptan bağımsız.
    if commercial_intent_gate(text).await? {
        return Ok(GateEntry::ByIntent);
    }

    Ok(GateEntry::Clean)
}

/// `gate_decision`'in bool sarmalı: yalnız "tam analiz koşsun mu" sorusunu
/// soran çağrı yerleri (ölçüm araçları, testler) bunu kullanır.
pub async fn needs_full_analysis(text: &str) -> Result<bool, String> {
    Ok(gate_decision(text).await? != GateEntry::Clean)
}

/// Kapının YALNIZ model tarafı: kural katmanını çalıştırmaz.
///
/// Ayrı durmasının sebebi ölçüm: `--gate-file` bu fonksiyonu ve
/// `needs_full_analysis`'i aynı metne ayrı ayrı sorarak kural katmanının
/// modelin kaçırdığı metinlerde ne kadar kazandırdığını sayabiliyor.
/// Üretim akışı bunu doğrudan çağırmaz, `needs_full_analysis` üzerinden geçer.
pub async fn llm_genre_gate(text: &str) -> Result<bool, String> {
    let prompt = r#"You are a TEXT GENRE classifier. You do NOT look for manipulation. You only decide what kind of text this is.

Choose exactly one "category":
- "bilgi": reference or encyclopedic facts, science, history, geography, statistics stated plainly.
- "talimat": instructions, a recipe, a manual, a how-to.
- "rapor": weather, traffic, sports or market report; plain news reporting of events.
- "duyuru": official announcement, schedule, regulation, timetable.
- "kisisel": someone describing THEIR OWN day, feelings, memories or beliefs, addressed to no one in particular.
- "inceleme": a review that names BOTH strengths and weaknesses, or advises comparing alternatives.
- "ikna": advertising or sales copy; campaign or propaganda text; an argument pushing the reader toward a belief or an action; an appeal to authority, science or majority opinion used to settle a claim; a text that tells another person what to think, feel or remember, blames them, or dismisses their account of events; any demand or pressure aimed at a person.

Decide by the text's PURPOSE, not its subject or tone. Two tests, in this order:
1. Is the text AIMED at a person - telling them what to do, what to believe, or what they got wrong? If yes -> "ikna", even when it sounds like a private conversation and mentions no product.
2. Otherwise, is it built to move a reader toward a belief, a decision or a purchase? If yes -> "ikna".
If neither, pick the descriptive category. An upsetting, angry, religious or emotional text is "kisisel" ONLY when the writer is describing their own experience and not pressing anyone.

CALIBRATION (decide the same way for similar texts, never copy their wording):
- "I walked by the sea today and stopped at the bookshop on the way home." -> "kisisel". The writer describes their own day and asks nothing of anyone.
- "I don't remember that conversation the way you do, you always blow things out of proportion, everyone notices it." -> "ikna". It is aimed at a person and overrides their memory - private tone does not make it neutral.
- "Experts agree this is the most reliable option and anyone sensible has already switched." -> "ikna". Authority and majority are used to settle the question for the reader.
- "After everything I sacrificed for you, you don't even call. Everyone talks about it behind your back." -> "ikna". Reproach and guilt over past favours are pressure, not a description of the writer's day - a family setting does not make it "kisisel".
- "The capital's population was about 5.8 million in the 2023 census." -> "bilgi". A plain fact with no reader to move.
- "Only 3 hours left! Everyone already bought it, don't fall behind, stock is running out." -> "ikna". Countdown plus social proof plus scarcity is sales copy, NOT a report - a market or weather "rapor" states facts and asks nothing of the reader.
- "You always blow things out of proportion, I never said that, the problem is you." -> "ikna". One short sentence aimed at a person, overriding their account and blaming them.
- "Rain tomorrow, don't forget your umbrella." -> "rapor". A forecast with a harmless practical note, selling nothing and blaming no one.

LENGTH IS NOT A SIGNAL: a single short sentence can be "ikna". Never pick a descriptive category just because the text is short, has an exclamation mark, or names no product.

"needs_analysis" = true ONLY for category "ikna". For every other category it is false.

Output ONLY one valid JSON object, no markdown, no extra text:
{"category":"bilgi"|"talimat"|"rapor"|"duyuru"|"kisisel"|"inceleme"|"ikna","needs_analysis":true|false}"#;

    let raw = call_llm_json(prompt, text).await?;

    #[derive(serde::Deserialize)]
    struct Triage {
        #[serde(default)]
        category: String,
        #[serde(default)]
        needs_analysis: bool,
    }

    let t: Triage = serde_json::from_str(&raw).map_err(|e| e.to_string())?;

    // Kategori ile bayrak çelişirse kategoriye güven: model bayrağı
    // doldururken kategoriden daha sık kayıyor.
    let ikna = t.category.trim().eq_ignore_ascii_case("ikna");
    tracing::debug!(category = %t.category, needs_analysis = t.needs_analysis, "ön eleme");
    Ok(ikna || (t.category.trim().is_empty() && t.needs_analysis))
}

/// Kapının ikinci sorusu: metin, yazarın kendi sunduğu bir şeye yönlendiriyor mu?
///
/// Tür sorusundan bağımsız duruyor: bir metin "haber" ya da "kişisel hikâye"
/// biçiminde yazılmış olabilir ve yine de sonunda bir ürüne, randevuya, kanala
/// ya da kayıt formuna çağırabilir. Reklam tam olarak böyle gizleniyor.
/// Yalnız tür sorusu elediğinde çağrılır, yani temiz metinlerde maliyeti iki
/// çağrıdır; ikna metinleri zaten ilk soruda geçer.
pub async fn commercial_intent_gate(text: &str) -> Result<bool, String> {
    let prompt = r#"You answer ONE question about a text.

Does the writer (or the organisation the writer speaks for) steer the reader toward something THEY provide - a product, a service, an appointment, a course, a channel, an app, a form, a donation, an event sign-up or a message to them?

Say true when the text, at any point, invites the reader to buy, order, download, book, apply, join, subscribe, follow a link, use a code, or contact the writer about an offer. The invitation counts even when it is one short sentence at the end of an otherwise informative text, and even when no price is named.

Say false when the text only informs, instructs, reports, reviews or recounts, and the reader is not being moved toward anything the writer provides:
- a news report, a forecast, an encyclopedic fact, a match result, a market summary
- a recipe, a manual, a safety instruction, a set of steps to follow
- a customer's own review of something they bought, including one that praises it
- an institution's notice of its own dates, hours or procedures to people already enrolled or served
- someone describing their own day, feelings or memories
Mentioning a product, a brand, a price or a percentage is NOT enough on its own. A named brand in a news report is still false.

The decisive test is WHO GAINS. Ask: is there an offer BEHIND this text that the writer profits from, and is the reader being routed to it? A writer who owns, sells, represents or earns from the thing -> true. A writer who merely used it, explains it, or reports it -> false. When the text names no way to reach an offer - no link, no code, no address, no contact, no "we", no invitation - answer false.

CALIBRATION:
- "I struggled with migraines for years, then a friend recommended this pillow. Message me and I'll tell you where I got it." -> true. The writer routes the reader to their own recommendation.
- "We compared four credit cards. One bank's terms came out clearly ahead. We recommend this card; the application takes a few minutes." -> true. The comparison ends in a steer.
- "Gum bleeding is often ignored but can end in tooth loss. At our clinic the first examination is free." -> true. Health information used as the approach to a service.
- "Vitamin D deficiency is common and can cause fatigue. Diagnosis is by blood test; the dose is set by a physician." -> false. Information with no offer behind it.
- "Applications for the municipality's free course run until 30 April; there are 200 places." -> false. A public notice, nothing is sold and the writer gains nothing.
- "The home side won 2-1; the striker took his season tally to 14." -> false. A report.
- "I have used the headphones for three months. The isolation is good, the microphone is mediocre and the pads get warm; look at other models in this price range too." -> false. A buyer's own review. Praise alone would still be false - the writer sells nothing and routes the reader nowhere.
- "Wash the lentils, chop one onion and saute it in olive oil, then add four cups of water." -> false. Steps to follow, no offer behind them.
- "Spring course registration runs from 3 to 7 March through the student information system." -> false. An institution telling its own students a date. Nobody profits from the reader acting.
- "I have used this coffee machine for six months: large tank, noisy grinder. Message me for my discount code." -> true. The same review becomes a steer the moment the writer routes the reader to an offer.

Output ONLY one valid JSON object, no markdown, no extra text:
{"promotes":true|false,"offer":"short phrase naming what is offered, or empty"}"#;

    let raw = call_llm_json(prompt, text).await?;

    #[derive(serde::Deserialize)]
    struct Intent {
        #[serde(default)]
        promotes: bool,
        #[serde(default)]
        offer: String,
    }

    let i: Intent = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    tracing::debug!(promotes = i.promotes, offer = %i.offer, "ön eleme: ticari amaç");
    Ok(i.promotes)
}

async fn call_llm_agent(
    system_prompt: &str,
    user_text: &str,
) -> Result<AgentAnalysis, String> {
    let raw = call_llm_json(system_prompt, user_text).await?;
    serde_json::from_str(&raw)
        .map_err(|e| format!("Agent JSON parse hatası: {e}\\nYanıt: {raw}"))
}

/// Azure OpenAI Responses API'den ham JSON metni ister.
///
/// Bütün LLM kullanan yollar bu fonksiyondan geçer:
/// - tür kapısı
/// - ticari amaç kapısı
/// - 6 uzman ajan
/// - demografi ajanı
///
/// Böylece sağlayıcı/model ayarı tek noktada tutulur.
pub(crate) async fn call_llm_json(system_prompt: &str, user_text: &str) -> Result<String, String> {
    let endpoint = azure_openai_endpoint()?;
    let api_key = azure_openai_api_key()?;
    let deployment = azure_openai_deployment();

    let payload = json!({
        "model": deployment,
        "instructions": system_prompt,
        "input": format!("Return ONLY a valid JSON object. Do not use markdown. Process only the data/text after this instruction.\n\n{}", user_text),
        "store": false,
        "reasoning": {
            "effort": "minimal"
        },
        "text": {
            "format": {
                "type": "json_object"
            }
        },
        "max_output_tokens": 2000
    });

    let url = format!("{}/responses", endpoint.trim_end_matches('/'));

    let response = http_client()
        .post(&url)
        .header("api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Azure OpenAI bağlantı hatası: {e}"))?;

    let status = response.status();

    let body = response
        .text()
        .await
        .map_err(|e| format!("Azure OpenAI yanıtı okunamadı: {e}"))?;

    if !status.is_success() {
        return Err(format!("Azure OpenAI hatası ({status}): {body}"));
    }

    let value: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| format!("Azure OpenAI JSON yanıtı çözülemedi: {e}"))?;

    // HTTP 200 dönse bile Responses API çıktı bütçesi dolarsa status=incomplete
    // olabilir. Bunu sessizce "boş yanıt" saymak yerine açık hata döndür.
    if value.get("status").and_then(|v| v.as_str()) == Some("incomplete") {
        let reason = value
            .get("incomplete_details")
            .and_then(|v| v.get("reason"))
            .and_then(|v| v.as_str())
            .unwrap_or("bilinmeyen neden");
        return Err(format!(
            "Azure OpenAI yanıtı tamamlanamadı: {reason}. max_output_tokens sınırını veya reasoning ayarını kontrol edin."
        ));
    }

    // Bazı istemci biçimlerinde kolaylaştırılmış output_text alanı bulunabilir.
    if let Some(text) = value.get("output_text").and_then(|v| v.as_str()) {
        if !text.trim().is_empty() {
            return Ok(text.to_string());
        }
    }

    // REST Responses API'nin standart output -> content -> text yapısı.
    if let Some(outputs) = value.get("output").and_then(|v| v.as_array()) {
        for output in outputs {
            if let Some(contents) = output.get("content").and_then(|v| v.as_array()) {
                for content in contents {
                    if content.get("type").and_then(|v| v.as_str()) == Some("output_text") {
                        if let Some(text) = content.get("text").and_then(|v| v.as_str()) {
                            if !text.trim().is_empty() {
                                return Ok(text.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    Err(format!(
        "Azure OpenAI geçerli bir metin yanıtı döndürmedi. Ham yanıt: {body}"
    ))
}

/// Güven eşiği: bunun altındaki tahminler "bilinmiyor" sayılır.
pub const DEMOGRAPHIC_MIN_CONFIDENCE: f32 = 0.60;

/// DEMOGRAFİ AJANI — kullanıcının KENDİ tarama geçmişinden profil çıkarır.
///
/// Diğer 6 ajandan iki farkı var:
/// 1. Girdisi tek bir metin değil, kullanıcının biriken geçmişidir.
/// 2. Analiz akışında çalışmaz. `run_orchestrator` zaten 7 LLM çağrısı
///    yapıyor; 8.'si her analizin yanıt süresine binerdi. Bu ajan isteğin
///    dışında, birkaç analizde bir tetiklenir.
///
/// `evidence`: sayaç katmanı + son taranan metin önizlemeleri (JSON).
pub async fn analyze_demographic(
    evidence: &str,
    lang: &str,
) -> Result<DemographicInference, String> {
    let out_lang = output_language(lang);

    let prompt = format!(
        r#"You are a USER PROFILING analyst. Your subject is the PERSON WHO SCANNED these texts - not the authors of the texts. Infer only what their scanning history reasonably supports.

INPUT: JSON with "stats" (counts computed from their history) and "recent_previews" (short excerpts of texts they chose to scan).

FORBIDDEN - never infer, never mention, never hint at: ethnicity or national origin, religion or belief, health or disability, sexual orientation, political opinion, or any criminal record. These are special-category personal data. If the evidence points that way, ignore it.

ALLOWED fields, each judged independently:
- "yas_araligi": an age band such as "18-24", "25-34", "35-44", "45-54", "55+".
- "cinsiyet": only if the previews contain explicit self-reference; otherwise "bilinmiyor".
- "egitim_seviyesi": e.g. "lise", "üniversite", "lisansüstü".
- "tuketici_egilimi": what kind of commercial content pulls them in, in a few words.
- "ilgi_alanlari": at most 5 short topic labels drawn from what they actually scanned.

EVIDENCE RULES:
1. Base every field ONLY on the given stats and previews. Never invent a detail that is not supported.
2. "guven" is your confidence 0.0-1.0. Be honest and conservative: a single weak hint is below 0.60.
3. If a field is below 0.60 confidence, set "deger" to "bilinmiyor" and keep the low score. Guessing is worse than admitting ignorance.
4. "dayanak": ONE short sentence naming the observation behind the guess. If "deger" is "bilinmiyor", write a short sentence saying the evidence is insufficient.
5. Scanning a manipulative text means the person was EXPOSED to it, not that they agree with it. Never treat the content's own claims as the person's traits.

"ozet": at most 2 plain sentences describing this person's scanning behaviour for an everyday reader. LANGUAGE: every text field you output ("deger", "dayanak", "ilgi_alanlari", "ozet") MUST be written in {out_lang}, regardless of the language of the evidence.

Output ONLY one valid JSON object, no markdown, no extra text:
{{"yas_araligi":{{"deger":"...","guven":0.0,"dayanak":"..."}},"cinsiyet":{{"deger":"...","guven":0.0,"dayanak":"..."}},"egitim_seviyesi":{{"deger":"...","guven":0.0,"dayanak":"..."}},"tuketici_egilimi":{{"deger":"...","guven":0.0,"dayanak":"..."}},"ilgi_alanlari":[],"ozet":"..."}}"#
    );

    let raw = call_llm_json(&prompt, evidence).await?;
    let mut inference: DemographicInference =
        serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    sanitize_demographic(&mut inference, lang);
    Ok(inference)
}

/// Model kuralı çiğnerse çıktıyı biz düzeltiriz: eşiğin altındaki her tahmin
/// "bilinmiyor"a çekilir, güven skoru aralığa sıkıştırılır, ilgi alanları
/// 5 ile sınırlanır. Modelin uyumuna güvenmiyoruz.
fn sanitize_demographic(inference: &mut DemographicInference, lang: &str) {
    let unknown = if lang == "en" { "unknown" } else { "bilinmiyor" };

    for field in [
        &mut inference.yas_araligi,
        &mut inference.cinsiyet,
        &mut inference.egitim_seviyesi,
        &mut inference.tuketici_egilimi,
    ] {
        field.guven = field.guven.clamp(0.0, 1.0);
        if field.guven < DEMOGRAPHIC_MIN_CONFIDENCE || field.deger.trim().is_empty() {
            field.deger = unknown.to_string();
        }
    }

    inference.ilgi_alanlari.retain(|i| !i.trim().is_empty());
    inference.ilgi_alanlari.truncate(5);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DemographicTrait;

    #[test]
    fn sales_copy_gate_catches_short_ads() {
        // 2026-09-12 ölçümünde llama3 bu metne "rapor" demişti.
        assert!(looks_like_sales_copy(
            "Son 3 saat! Herkes aldı, geride kalma, stoklar bitiyor."
        ));
        assert!(looks_like_sales_copy(
            "%90 indirim bugüne özel, hemen sipariş ver."
        ));
        assert!(looks_like_sales_copy(
            "Limited stock, everyone is switching - buy now."
        ));
    }

    #[test]
    fn sales_copy_gate_catches_disguised_ads() {
        // 2026-09-12 kapı ölçümünde (kapi-olcum-seti.txt) llama3'ün üç kez
        // üst üste elediği kalıplar: reklam kişisel hikâye, uzman uyarısı ya
        // da topluluk duyurusu kılığında ve satış cümlesi sona saklanmış.
        assert!(looks_like_sales_copy(
            "Sabah rutinim basit. Soranlar için markayı etiketledim, profildeki koda %20 iniyor."
        ));
        assert!(looks_like_sales_copy(
            "İndirim kodum profilimde, kargo bedava."
        ));
        // Kural katmanının sınırı: bu metinde satış kelimesi yok, dönüşüm
        // cümlesi "randevu takvimimiz dolmak üzere". Kelime listesi bunu
        // yakalamaz ve yakalamaya çalışmak temiz duyuruları içeri alıyordu;
        // işi `commercial_intent_gate` devralır.
        assert!(!looks_like_sales_copy(
            "Arıtma cihazı için randevu takvimimiz dolmak üzere."
        ));
    }

    #[test]
    fn personal_pressure_gate_catches_gaslighting() {
        assert!(looks_like_personal_pressure(
            "Sen hep abartıyorsun, öyle bir şey demedim, sorun sende."
        ));
        assert!(looks_like_personal_pressure(
            "Yanlış hatırlıyorsun, senin yüzünden herkes tedirgin."
        ));
        assert!(!looks_like_personal_pressure(
            "Dün sahilde yürüdüm, dönüşte kitapçıya uğradım."
        ));
    }

    #[test]
    fn sales_copy_gate_leaves_plain_texts_alone() {
        assert!(!looks_like_sales_copy(
            "Yarın hava yağmurlu olacak, şemsiye almayı unutma."
        ));
        assert!(!looks_like_sales_copy(
            "Başkentin nüfusu 2023 sayımında yaklaşık 5,8 milyondu."
        ));
        // Tek sinyal yeterli değil: haber metni de "indirim" yazar.
        assert!(!looks_like_sales_copy(
            "Market fiyatlarındaki indirim ekimde de sürdü."
        ));
        // Resmî duyuruda son tarih ve kontenjan geçer; satış cümlesi yok.
        // 2026-09-12'de bu metin "son gun" + "kontenjan" yüzünden kurala
        // takılıyordu; kelime listesi o yüzden daraltıldı.
        assert!(!looks_like_sales_copy(
            "Belediyenin ücretsiz kursuna başvurular 30 Nisan'a kadar sürecek,              son gün saat 17.00'de sistem kapanıyor. Kontenjan 200 kişidir."
        ));
        // Güvenlik talimatında "hemen" aciliyet sinyali sayılsa da tek grup kalır.
        assert!(!looks_like_sales_copy(
            "Gaz kokusu alırsanız vanayı hemen kapatın ve binayı terk edin."
        ));
    }

    fn trait_of(deger: &str, guven: f32) -> DemographicTrait {
        DemographicTrait {
            deger: deger.to_string(),
            guven,
            dayanak: "dayanak".to_string(),
        }
    }

    fn sample() -> DemographicInference {
        DemographicInference {
            yas_araligi: trait_of("25-34", 0.82),
            cinsiyet: trait_of("kadın", 0.30),
            egitim_seviyesi: trait_of("", 0.95),
            tuketici_egilimi: trait_of("fitness ürünleri", 0.71),
            ilgi_alanlari: vec![
                "spor".into(),
                "  ".into(),
                "teknoloji".into(),
                "a".into(),
                "b".into(),
                "c".into(),
                "d".into(),
            ],
            ozet: "özet".into(),
        }
    }

    #[test]
    fn low_confidence_becomes_unknown() {
        let mut inference = sample();
        sanitize_demographic(&mut inference, "tr");

        // Eşiğin üstü korunur
        assert_eq!(inference.yas_araligi.deger, "25-34");
        assert_eq!(inference.tuketici_egilimi.deger, "fitness ürünleri");
        // Eşiğin altı bastırılır (skor korunur, değer gizlenir)
        assert_eq!(inference.cinsiyet.deger, "bilinmiyor");
        assert!((inference.cinsiyet.guven - 0.30).abs() < f32::EPSILON);
        // Yüksek güvenli ama boş değer de bilinmiyor sayılır
        assert_eq!(inference.egitim_seviyesi.deger, "bilinmiyor");
    }

    #[test]
    fn interests_are_trimmed_and_capped() {
        let mut inference = sample();
        sanitize_demographic(&mut inference, "tr");

        assert_eq!(inference.ilgi_alanlari.len(), 5);
        assert!(!inference.ilgi_alanlari.iter().any(|i| i.trim().is_empty()));
    }

    #[test]
    fn confidence_is_clamped_to_range() {
        let mut inference = sample();
        inference.yas_araligi.guven = 3.7;
        sanitize_demographic(&mut inference, "tr");
        assert!(inference.yas_araligi.guven <= 1.0);
    }

    #[test]
    fn english_unknown_label() {
        let mut inference = sample();
        sanitize_demographic(&mut inference, "en");
        assert_eq!(inference.cinsiyet.deger, "unknown");
    }
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
    call_llm_agent(&prompt, text).await
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
    call_llm_agent(&prompt, text).await
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
    call_llm_agent(&prompt, text).await
}

pub async fn analyze_perceptual(text: &str, lang: &str) -> Result<AgentAnalysis, String> {
    let out_lang = output_language(lang);
    let prompt = format!(
        r#"You are a PERCEPTUAL MANIPULATION analyst: framing and selective information distorting the reader's picture of reality.

DETECT only: cherry-picked data; omitted context that reverses a claim's meaning; technically-true-but-misleading framing; statistical distortion (no baseline, cropped scales); false dichotomy.
NOT manipulation: one-sided but honest advocacy, simplified explanations, merely incomplete informative text.
TEST: is information curated so the reader reliably reaches a FALSE conclusion? Incompleteness alone is not enough -> detected = false.
You must be able to name the FALSE conclusion the reader would reach and the fact that was hidden to produce it. If you cannot name both, detected = false.

CALIBRATION (decisions only, never copy their wording):
- "The capital city's population is about 5.8 million according to the 2023 census." -> detected = false. A plainly stated fact is not framing, even though it omits everything else about the city.
- "Water boils at 100 degrees at sea level." -> detected = false. Textbook facts have no target audience to mislead.
- "Our product cut costs by 40%" while hiding that the comparison is against a deliberately overpriced package -> detected = true, 0.90. Hidden baseline produces a false conclusion.
{shared}"#,
        shared = shared_rules("Algısal", out_lang)
    );
    call_llm_agent(&prompt, text).await
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
    call_llm_agent(&prompt, text).await
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
    call_llm_agent(&prompt, text).await
}
