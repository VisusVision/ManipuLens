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

**ManipuLens**, büyük dil modellerinin (LLM) kullanıcı etkileşimleri sırasında başvurabileceği dilbilimsel, psikolojik ve istatistiksel manipülasyon tekniklerine karşı geliştirilmiş, yapay zeka tabanlı bir güvenlik duvarıdır. Çoklu Ajanlı (Multi-Agent) LLM orkestrasyon mimarisi ve yüksek performanslı Rust backend servisiyle desteklenen bu gizlilik odaklı gelişmiş tarayıcı uzantısı, hem yapay zeka sohbetlerini hem de web üzerindeki metinleri sağ tık entegrasyonuyla gerçek zamanlı inceleyerek bilişsel ve davranışsal manipülasyonları anında tespit eder ve maskeler.

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





