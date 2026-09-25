# ManipuLens

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Chrome_Uzantısı-4285F4?style=for-the-badge&logo=google-chrome&logoColor=white" alt="Chrome Extension">
  <img src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker">
  <img src="https://img.shields.io/badge/Azure_OpenAI-0078D4?style=for-the-badge&logo=microsoftazure&logoColor=white" alt="Azure OpenAI">
</p>

---

<p align="center">
  🌐 <b>Diller:</b> 
  <a href="README.md">English</a> | <b>Türkçe</b>
</p>

---

**ManipuLens**, yüksek performanslı bir **Rust** arka plan servisi ve **Çoklu Ajanlı Büyük Dil Modeli (LLM)** orkestratörü tarafından desteklenen, gizlilik odaklı gelişmiş bir tarayıcı uzantısıdır. Web sayfalarında seçtiğiniz metinleri sağ tık menünüz üzerinden sorunsuz bir şekilde analiz ederek istatistiksel, dilsel, bilişsel ve davranışsal manipülasyonları gerçek zamanlı olarak tespit eder ve ifşa eder.

### Temel Odak Alanları:
* 🕵️ **Çoklu Ajan Kararları:** Metin matrisini eşzamanlı olarak inceleyen uzman alt ajanlar (Dilsel, Psikolojik, Davranışsal, Algısal, Sosyal).
* 🛡️ **Önce Gizlilik:** Analiz için seçtiğiniz metin, ManipuLens Azure backend'ine ve yapılandırılmış Azure OpenAI modeline gönderilir. Hesap ve analiz geçmişi PostgreSQL üzerinde saklanır; tam analiz metni audit loglarına yazılmaz.
* 🚀 **Yüksek Performans:** Neredeyse anlık orkestrasyon ve değerlendirme için Rust (Axum/Tokio) mimarisinden güç alır.

---

## 🚀 Öne Çıkan Özellikler

* **⚡ Yüksek Performanslı Rust Orkestratörü:** Uzman alt ajan analiz yollarını paralel olarak tetiklemek ve birleştirmek için Axum ve Tokio'yu asenkron olarak kullanır ve milisaniyeler seviyesinde yanıt süreleri sağlar.
* **🧠 Karar Zincirli Çoklu Ajan Mimarisi:** Bir Sentezör (Yönetici) Ajan tarafından denetlenen 5 uzmanlık alanının yanı sıra, bilişsel kırılganlıkları tahmin eden gelişmiş bir *Tüketici Eğilim Ajanı* içerir.
* **🖱️ Sağ Tık Menü Entegrasyonu (Akıcı UX):** Manuel kopyala-yapıştıra gerek yoktur. Herhangi bir web sayfasındaki metni seçip sağ tıklayarak "ManipuLens ile Analiz Et" seçeneğini seçmeniz akışı başlatmak için yeterlidir.
* **🎨 Dinamik İşaretçi Enjeksiyonu:** Genel ve sabit renkler yerine uzantı, sayfadaki DOM `<mark>` stilini tespit edilen **baskın manipülasyon türünün** renk profiliyle (örn. Psikolojik = Macenta, Sosyal = Mavi) dinamik olarak değiştirir.
* **🐳 Dağıtıma Hazır DevOps:** Ultra hafif Alpine konteynerleri üzerinde çalışan, statik Linux derlemesine (`x86_64-unknown-linux-musl`) sahip Docker Çok Aşamalı (Multi-Stage) yapısıyla kurulmuştur.

---

## 🎭 Uzman Ajan Kadrosu

ManipuLens, içeriklerin anlamsal bütünlüğünü ayrıştırmak, incelemek ve görselleştirmek için yerel üretken ajanların hiyerarşik yapısına güvenir:

| Ajan Profili | Odak Alanı | Dinamik Arayüz Rengi |
| :--- | :--- | :--- |
| **Dilsel (Linguistic)** | Kelime oyunları, mantık hataları, kelime çarpıtmaları ve semantik manipülasyonlar. | `#4cc9f0` (Açık Mavi) |
| **Psikolojik (Psychological)** | Gaslighting, suçluluk psikolojisi yaratma ve korku kültürü aşılama. | `#f72585` (Macenta) |
| **Davranışsal (Behavioral)** | Yapay aciliyet hissi oluşturma, FOMO ve fevri eylem tuzakları. | `#f8961e` (Turuncu) |
| **Algısal (Perceptual)** | Gerçekleri cımbızlama (cherry-picking), taraflı çerçeveleme ve seçici sunum. | `#7209b7` (Mor) |
| **Sosyal (Social)** | Mahalle baskısı, sürü psikolojisi, kutuplaştırma ve kabilecilik önyargıları. | `#4361ee` (Koyu Mavi) |
| **Pazarlama (Marketing)** | Gizli reklam, sorun şişirme, mucize vaadi ve satın alma baskısı. | `#2a9d8f` (Turkuaz) |

---

## 🚪 Ön Eleme Kapısı ve Ölçüm

Her metin altı ajana gitmez. Önce **ön eleme kapısı** çalışır; amacı hem yanlış alarmı
azaltmak hem de temiz metinleri 1–2 Azure OpenAI ön eleme çağrısından sonra durdurmaktır. Ön elemeden geçen metinler 6 uzman ajan ve sentezleyici ile tam analize devam eder. Kapı üç kademeli,
en ucuzdan başlar:

1. **Kural katmanı (LLM yok)** — satış kopyası sinyalleri (aciliyet, kıtlık, fiyat, eyleme
   çağrı, sosyal kanıt; en az iki farklı grup) ve kişiye yönelen baskı kalıpları. Eşleşirse
   model hiç çağrılmadan tam analize geçilir.
2. **Tür sorusu** — "bu metin bilgi mi veriyor, ikna mı ediyor?"
3. **Ticari amaç sorusu** — yalnız tür sorusu elediğinde sorulur: "yazar okuyucuyu kendi
   sunduğu bir şeye mi yönlendiriyor?" Belirleyici test **kim kazanıyor**: ürünü satan
   yazar evet, yalnız kullanan/anlatan yazar hayır.

Üçüncü kademe ölçümle geldi: haber, inceleme ya da kişisel hikâye kılığına girmiş
reklamların 12'de 8'i tür sorusunda eleniyordu ve kelime tabanlı kural katmanı bunların
**hiçbirini** yakalamıyordu (2026-09-12). Kalıp listesi ezberliyor, amaç sorusu genelliyor.

Ölçüm iki komutla yapılır, sunucu ve uzantı gerekmez:

```bash
cargo run --release -- --gate-file kapi-olcum-seti.txt      # yalnız kapı
cargo run --release -- --analyze-file kapi-olcum-seti.txt   # tam akış
```

Dosya biçimi `ETIKET|metin` (`MANIP` / `TEMIZ`, `#` yorum). Depodaki setler:
`dogrulama-seti.txt` (21, prompt ayarında kullanıldı — regresyon seti),
`kapi-olcum-seti.txt` (30: kısa reklam / gizlenmiş reklam / zor temiz),
`kapi-dogrulama-seti-2.txt` (18, kural katmanı sıkılaştırıldıktan sonra yazıldı).
Bir sete bakıp kural düzeltirsen o set ayrıklığını kaybeder; yenisini yaz, eskisini
regresyon seti olarak sakla.

---

## 🔄 UI/UX İş Akışı

1. **Seçim ve Tetikleme:** Kullanıcı web sayfasında bir metin seçer. Sağ tıklama, `background.js` aracılığıyla `chrome.storage.local` kullanarak güvenli bir işlem alanı oluşturur.
2. **Asenkron El Sıkışma:** Uzantı popup penceresi otomatik olarak açılır ve olası çakışmaları (race conditions) önlemek için arayüz etkileşimini anında kilitler (`button.disabled = true`).
3. **Rust Çok Kanallı Süzme:** İstek verisi production ortamında Azure Container Apps üzerindeki `/v1/analyze` uç noktasına ulaşır ve çoklu ajan analiz akışını başlatır.
4. **Görsel Sentez:** Uzantı, bağlamsal olarak renklendirilmiş işaretçileri web sayfasının aktif DOM yapısına enjekte eder ve özelleştirilmiş tüketici davranışı tahmin kartını görüntüler.

---

## 📊 Kullanıcı Profili ve Veri Seti

Her analiz, 6 uzman ajanın kararıyla birlikte PostgreSQL `history` tablosuna yazılır
(tam metin değil, 120 karakterlik önizleme). Bunun üstünde iki katmanlı bir
kullanıcı profili durur:

- **Sayaç katmanı** — her analizden sonra, isteğin dışında (`tokio::spawn`) ve
  LLM çağrısı olmadan güncellenir. Toplam analiz, manipülatif oran, baskın
  tip dağılımı, ajan bazlı tespit sayıları, dil dağılımı, ürün/sektör
  tahminleri, ortalama metin uzunluğu.
- **Çıkarım katmanı** — demografi ajanı (`analyze_demographic`); kullanıcının
  sayaçlarını ve son 30 metin önizlemesini okuyup yaş aralığı, eğitim
  seviyesi, tüketici eğilimi ve ilgi alanları tahmin eder. Her analizde
  değil, 5 analizde bir (ya da çıkarım 24 saatten eskiyse) tazelenir — tam analiz birden fazla Azure OpenAI çağrısı yaptığı için, 8.'si kullanıcının bekleme süresine
  binerdi.

Demografi ajanının sınırları koda gömülüdür: güveni 0.60'ın altındaki her
tahmin "bilinmiyor"a çekilir (modelin uyumuna güvenilmez, çıktı Rust
tarafında da denetlenir) ve **etnik köken, din, sağlık, cinsel yönelim,
siyasi görüş** alanları hem prompt'ta yasaklıdır hem de çıktı şemasında yer
almaz — bunlar KVKK/GDPR'da özel nitelikli kişisel veridir. Ajan başarısız
olursa mevcut profil olduğu gibi korunur.

En az 5 analiz olmadan profil üretilmez. Kullanıcı kendi profilini görür ve
silebilir; başkasının profiline erişim yoktur (kimlik yalnızca oturum
token'ından türetilir).

```
GET  /v1/profile          → kendi profilin (yoksa exists:false)
POST /v1/profile/delete   → kendi profilini sil (geçmişe dokunmaz)
```

## 📣 Reklam Hedefleme

Toplanan profil, kullanıcıya hangi önerinin gösterileceğine karar veren hedefleme
ajanını besler (`src/ads.rs`). Ajan iki katmanlıdır: kural katmanı LLM'siz skorlar
ve hariç tutar, LLM yalnız "neden bu reklam?" cümlesini yazar — karar
deterministik ve denetlenebilir kalsın diye.

Sınırlar koda gömülüdür:
- **Rıza zorunlu.** `users.ads_consent` varsayılan `false`; rıza yokken profil hiç
  okunmaz. Rıza geri alınınca o rızayla üretilmiş kararlar silinir.
- Hassas kategoriler (kumar, alkol, kredi) güvenilir bir **yetişkin yaş sinyali**
  olmadan gösterilmez; yaş "bilinmiyor" ise yetişkin sayılmaz.
- **Aciliyet kurgusu** kullanan kampanya, davranışsal manipülasyona en açık
  kullanıcıya gösterilmez. ManipuLens bu tuzağı gösteren araçtır; aynı tuzağı
  kendi panelinde kurmaz.
- Güveni 0.60 altındaki demografi sinyali skora girmez.
- Her reklamın yanında "neden bu reklam?" satırı durur ve reklam gizlenebilir.

```
GET  /v1/ads               → sana uygun öneriler (+ decision_id)
POST /v1/ads/feedback      → impression | click | dismiss
POST /v1/consent           → reklam rızasını aç/kapa
POST /v1/ads/inventory     → kampanya ekle/güncelle (ADS_ADMIN_TOKEN gerekir)
```

Envanter başlangıçta boştur; kampanya yoksa `/v1/ads` boş liste döner. Depodaki
`ornek-reklam-envanteri.json` yedi örnek kampanya taşır (biri İngilizce, biri
`sensitive`), yükleyici ile yazılır:

```bash
ADS_ADMIN_TOKEN=... python envanter-yukle.py ornek-reklam-envanteri.json
```

Aynı `id` ile tekrar yüklemek kampanyayı günceller, kopya oluşturmaz.

**Veri seti dışa aktarımı** — sunucu açmadan çalışır, satır başına bir analiz:

```
cargo run -- --export-dataset dataset.jsonl
```

Gizlilik: dışa aktarımda e-posta yer almaz; kullanıcı ayrımı UUID ile yapılır.

## 🗺️ Sistem Mimarisi

ManipuLens, ana olay döngülerini engellemeden karmaşık çoklu ajan analizlerini yönetmek için optimize edilmiş asenkron bir işlem hattı kullanır:

[ Chrome Uzantısı (Frontend) ]
│
▼ (Sağ Tık Menü Olayı / storage.local)
[ background.js (Service Worker) ]
│
▼ HTTP POST (Payload: { text: "..." })
┌─────────────────────────────────────────────────────────┐
│ Rust Arka Plan Orkestratörü (Axum + Tokio)              │
│                                                         │
│      ┌──► Dilsel Ajan (Linguistic Agent)   ──┐          │
│      ├──► Psikolojik Ajan (Psychological)  ──┤          │
│  🛸  ├──► Davranışsal Ajan (Behavioral)    ──┼─► [Sentezör]
│      ├──► Algısal Ajan (Perceptual)        ──┤  (Manager)
│      └──► Sosyal Ajan (Social Agent)       ──┘          │
│                                                         │
│                               ▼                         │
│                    [Tüketici Eğilim Ajanı]              │
└─────────────────────────────┬───────────────────────────┘
│
▼ JSON Birleşik Yanıt
[ Dinamik DOM Enjeksiyonları ]

### Asenkron Konsensüs Protokolü (Rust Tarafı)
İstek `/v1/analyze` uç noktasına ulaştığında, Rust arka planı Azure OpenAI isteklerini paralel hale getirmek için `tokio::spawn` ve `tokio::join!` mimarisinden yararlanır. Ajanları sırayla çalıştırmak yerine, 6 uzmanın tamamı metin matrisini eşzamanlı olarak değerlendirir:

1. **Eşzamanlı Değerlendirme:** Çekirdek metrikler, `reqwest` aracılığıyla engellenmeyen (non-blocking) HTTP havuzlama kullanılarak toplanır.
2. **Sentez Stratejisi:** **Sentezör Ajan (Manager)** aktif bayrakları toplayarak, `target_sentences` listesini derleyerek ve mutlak `dominant_manipulation` türünü seçerek bir indirgeme katmanı görevi görür.
3. **Nöromarketing Analizi:** Elde edilen veriler **Tüketici Eğilim Ajanı**'na beslenir ve dilsel sömürü adımları somut tüketici davranışı risk profillerine dönüştürülür.

---

### 📦 Birleşik API Veri Şeması

Rust çalışma zamanı ile Chrome altyapısı arasındaki iletişim sözleşmesi, `types.rs` içinde tanımlanan kesin tipli bir JSON yapısını kullanır:

```json
{
  "is_manipulated": true,
  "dominant_manipulation": "Psikolojik",
  "genel_sonuc": "Metin matrisinde bulunan stratejik manipülasyon girişimlerini açıklayan detaylı özet...",
  "predicted_product": "VPN hizmeti veya yüksek fiyatlı gizlilik aracı onayı enjeksiyonu.",
  "detailed_analyses": [
    {
      "manipulation_type": "Psikolojik",
      "detected": true,
      "confidence_score": 0.85,
      "aciklama": "Gaslighting ve tehdit vektörü manipülasyonu gözlemlendi.",
      "target_sentences": [
        "Verileriniz şu anda sızıyor ve sizin umurunuzda bile değil.",
        "Bu katman olmadan kimliğiniz tamamen çıplak kalır."
      ]
    }
  ]
}
```
⚙️ Gereksinimler

Yerel geliştirme ortamı için:

- Docker Desktop (Compose desteği açık)
- Google Chrome veya Chromium tabanlı bir tarayıcı
- Azure OpenAI / Microsoft Foundry erişimi
- `.env` dosyasında gerekli Azure OpenAI değişkenleri

Gerekli temel ortam değişkenleri:

```env
AZURE_OPENAI_ENDPOINT=
AZURE_OPENAI_DEPLOYMENT=
AZURE_OPENAI_API_KEY=
DATABASE_URL=
🧩 Chrome Uzantısı Kurulumu (Frontend Kurulumu)
Frontend uzantısı doğrudan tarayıcı ortamında yaşadığı için, bunu tarayıcınıza manuel olarak yükleyin:

chrome://extensions/ adresini kopyalayın ve Chrome adres çubuğunuza yapıştırın.

Sağ üst köşede bulunan Geliştirici Modu (Developer Mode) anahtarını aktif hale getirin.

Sol üstteki Paketlenmemiş öğe yükle (Load Unpacked) butonuna tıklayın.

Yerel depo klasörünüzün içindeki extension dizinini seçin.

🎉 ManipuLens simgesi araç çubuğunuzda görünecek, tamamen bağlanmış ve sağ tık tetikleyicilerinizi dinlemeye hazır olacaktır!

📄 Lisans
MIT Lisansı altında dağıtılmaktadır. Daha fazla bilgi için LICENSE dosyasına bakın.





