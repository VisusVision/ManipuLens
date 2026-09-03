# ManipuLens

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Chrome_Uzantısı-4285F4?style=for-the-badge&logo=google-chrome&logoColor=white" alt="Chrome Extension">
  <img src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker">
  <img src="https://img.shields.io/badge/Ollama-000000?style=for-the-badge&logo=ollama&logoColor=white" alt="Ollama">
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
* 🛡️ **Önce Gizlilik (Yerel Yapay Zeka):** Ollama aracılığıyla tamamen kendi yerel makinenizde çalışır; verileriniz asla yerel ağınızın dışına çıkmaz.
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

---

## 🔄 UI/UX İş Akışı

1. **Seçim ve Tetikleme:** Kullanıcı web sayfasında bir metin seçer. Sağ tıklama, `background.js` aracılığıyla `chrome.storage.local` kullanarak güvenli bir işlem alanı oluşturur.
2. **Asenkron El Sıkışma:** Uzantı popup penceresi otomatik olarak açılır ve olası çakışmaları (race conditions) önlemek için arayüz etkileşimini anında kilitler (`button.disabled = true`).
3. **Rust Çok Kanallı Süzme:** İstek verisi `http://127.0.0.1:3000/v1/analyze` adresine ulaşarak çoklu ajan konsensüs ağını ateşler.
4. **Görsel Sentez:** Uzantı, bağlamsal olarak renklendirilmiş işaretçileri web sayfasının aktif DOM yapısına enjekte eder ve özelleştirilmiş tüketici davranışı tahmin kartını görüntüler.

---

## 📊 Kullanıcı Profili ve Veri Seti

Her analiz, 6 uzman ajanın kararıyla birlikte SQLite `history` tablosuna yazılır
(tam metin değil, 120 karakterlik önizleme). Bunun üstünde iki katmanlı bir
kullanıcı profili durur:

- **Sayaç katmanı** — her analizden sonra, isteğin dışında (`tokio::spawn`) ve
  LLM çağrısı olmadan güncellenir. Toplam analiz, manipülatif oran, baskın
  tip dağılımı, ajan bazlı tespit sayıları, dil dağılımı, ürün/sektör
  tahminleri, ortalama metin uzunluğu.
- **Çıkarım katmanı** — demografi ajanı (`analyze_demographic`); kullanıcının
  sayaçlarını ve son 30 metin önizlemesini okuyup yaş aralığı, eğitim
  seviyesi, tüketici eğilimi ve ilgi alanları tahmin eder. Her analizde
  değil, 5 analizde bir (ya da çıkarım 24 saatten eskiyse) tazelenir — bir
  analiz zaten 7 Ollama çağrısı yapıyor, 8.'si kullanıcının bekleme süresine
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
İstek `/v1/analyze` uç noktasına ulaştığında, Rust arka planı Ollama API isteklerini paralel hale getirmek için `tokio::spawn` ve `tokio::join!` mimarisinden yararlanır. Ajanları sırayla çalıştırmak yerine, 5 uzmanın tamamı metin matrisini eşzamanlı olarak değerlendirir:

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
Sistemi ayağa kaldırmadan önce bilgisayarınızda aşağıdaki bileşenlerin kurulu olduğundan emin olun:

Docker Desktop (Compose desteği aktif olmalı)

Ollama (11434 portunda yerel olarak çalışmalı)

Google Chrome Tarayıcı (Veya Chromium tabanlı herhangi bir tarayıcı)

🚀 Hızlı Başlangıç (Arka Plan Dağıtımı)
Optimize edilmiş çok aşamalı Docker kurulumumuz sayesinde, tüm Rust matrisini derleyebilir ve ortamı tek bir komutla ayağa kaldırabilirsiniz.

1. Yerel Yapay Zeka Modelini İndirin ve Başlatın
Sistem terminalinizi açın ve Ollama kullanarak llama3 çekirdek modelini bilgisayarınıza çekin:
```
ollama pull llama3
```
Proje `llama3` üzerine kuruludur ve varsayılan model odur. Model adı `.env`
içindeki `OLLAMA_MODEL` ile değiştirilebilir; bu yalnızca karşılaştırma
ölçümü içindir, üretim yapılandırması `llama3` kalır.

**Kalite değişikliği ölçümle kanıtlanır.** Etiketli doğrulama seti üzerinde tam
analiz akışını koşturup doğruluk ve yanlış pozitif oranını basan bir mod var:
```
cargo run --release -- --analyze-file dogrulama-seti.txt
```
Prompt veya akış değiştiğinde bu seti önce ve sonra koşturun; tek metne bakıp
"düzeldi" demek ölçüm değildir. Yeni bir şikâyet türü çıktığında sete etiketli
örnek olarak eklenir.

`dogrulama-seti-2.txt` ayrık (held-out) settir: prompt ayarı ona bakılmadan
yapılır, yalnız genellemeyi ölçmek için koşulur. Ayrık setteki bir metin
düzeltme için kullanıldıysa ayrık olmaktan çıkar; ana sete taşıyın.

2026-09-03 ölçümü, llama3:

| Aşama | Doğruluk | Yanlış pozitif |
| --- | --- | --- |
| Başlangıç | %67 | 4/7 temiz metin |
| Prompt dışlama + kanıt doğrulaması | %75 | 3/7 |
| Ön eleme (triage) eklendi | %100 | 0/7 |

Ön eleme temiz metinlerde altı ajanı hiç çağırmadığı için analiz süresi de
düştü: ikna metni ~12 sn, ikna olmayan metin ~0,4 sn.
2. Çoklu Ajan Kümesini Docker Compose ile Çalıştırın
Proje ana dizinine (/manipulation-detector) gidin ve derleme komutunu çalıştırın:
```
docker-compose up --build
```
Bu komut, izolasyon katmanları içinde statik musl-derlemesini yönetir ve yerel Ollama portunuza doğrudan köprü kurarak Axum sunucusunu 0.0.0.0:3000 adresine bağlar.

🧩 Chrome Uzantısı Kurulumu (Frontend Kurulumu)
Frontend uzantısı doğrudan tarayıcı ortamında yaşadığı için, bunu tarayıcınıza manuel olarak yükleyin:

chrome://extensions/ adresini kopyalayın ve Chrome adres çubuğunuza yapıştırın.

Sağ üst köşede bulunan Geliştirici Modu (Developer Mode) anahtarını aktif hale getirin.

Sol üstteki Paketlenmemiş öğe yükle (Load Unpacked) butonuna tıklayın.

Yerel depo klasörünüzün içindeki extension dizinini seçin.

🎉 ManipuLens simgesi araç çubuğunuzda görünecek, tamamen bağlanmış ve sağ tık tetikleyicilerinizi dinlemeye hazır olacaktır!

📄 Lisans
MIT Lisansı altında dağıtılmaktadır. Daha fazla bilgi için LICENSE dosyasına bakın.





