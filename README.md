\# ManipuLens: Çoklu Ajan Tabanlı LLM Manipülasyon Denetim Hattı (Audit Pipeline) 🕵️‍♂️⚖️



\[!\[License](https://img.shields.io/badge/License-Apache\_2.0-blue.svg)](LICENSE)

\[!\[Rust](https://img.shields.io/badge/backend-Rust-orange.svg)](src/)

\[!\[Ollama](https://img.shields.io/badge/LLM-Ollama-lightgrey.svg)](https://ollama.com)



\### "Büyük Dil Modelleri bizi manipüle ediyor mu?"



Günümüzde yapay zeka asistanları, belirli bir tercihe veya karara yönlendirmek için kanıtları seçici bir şekilde sunma (karanlık kalıplar) eğilimi gösterebilmektedir. Bu manipülatif kalıplar nadiren MMLU, HumanEval veya GSM8K gibi standart kıyaslama (benchmark) testlerinde ortaya çıkar. Ancak kullanıcı zarar raporlarında, düzenleyici incelemelerde (\*\*AB Yapay Zeka Yasası Madde 5\*\*, FTC karanlık kalıp kılavuzu) ve toplu dava şikayetlerinde kendilerini gösterirler.



\*\*ManipuLens\*\*, bu kritik açığı kapatmak için tasarlanmış, çoklu ajanlı (multi-agent) bir değerlendirme ve denetim çerçevesidir. Web üzerinde okuduğunuz veya seçtiğiniz herhangi bir metni asenkron uzman ajanların filtresinden geçirerek yapılandırılmış kanıt izleri üretir.



\---



\## ⚖️ Çalışma Mantığı ve Mahkeme Salonu Metaforu



Kavramsal olarak, her değerlendirme bir mahkeme salonu gibi ele alınır:



\* \*\*Savcılık (İnceleme Hattı):\*\* Web sayfasından seçilen veya sorgulanan hedef metin, potansiyel manipülasyon iddialarını toplar.

\* \*\*Jüri (Critic Agent Ensemble):\*\* 5 farklı manipülasyon kategorisine göre ayarlanmış uzman eleştirmen ajanlar metni bağımsız ve asenkron olarak değerlendirir.

\* \*\*Hakim \& Karar (Orchestrator/Aggregator):\*\* Hakemler arası uyum ölçütlerini ve sentez yeteneğini kullanarak hiçbir ajanın karara tek başına hükmetmesini engeller; nihai, adil ve makine tarafından okunabilir (\*\*JSON\*\*) bir denetim raporu üretir.



\---



\## 🤖 Manipülasyon Boyutları (Uzman Ajanlar)



| Ajan Kategorisi | Odak Noktası ve İncelediği Taktikler |

| :--- | :--- |

| \*\*Dilsel (Linguistic) Ajanı\*\* | Sözcük seçimi, belirsizlik, ikna edici ifadeler ve dil kalıpları yoluyla çerçeveleme hileleri. |

| \*\*Psikolojik (Psychological) Ajanı\*\* | Duygusal yönlendirme, gaslighting, suçluluk duygusu uyandırma ve korku pompalama (Fearmongering). |

| \*\*Davranışsal (Behavioral) Ajanı\*\* | Karar alma sürecini fevri şekilde hızlandırmayı (FOMO) veya kullanıcı eylemlerini kısıtlamayı amaçlayan yönlendirmeler. |

| \*\*Algısal (Perceptual) Ajanı\*\* | Gerçekleri seçici sunma (Cherry-picking) ve bilgiyi önyargılı yorumlatma girişimleri. |

| \*\*Sosyal (Social) Ajanı\*\* | Mahalle baskısı, sahte otorite ipuçları, fikir birliği iddiaları ve sürü psikolojisi mekanizmaları. |



\---



\## 🛠️ Sistem Mimarisi ve Teknolojik Altyapı



Sistem, maksimum yerel gizlilik (privacy-first) ve sıfır gecikme hedeflenerek iki ana parçadan kurulmuştur:

\[Tarayıcı Eklentisi (UI/UX)] ──▶ \[Rust Async Backend (Axum/Tokio)] ──▶ \[Yerel LLM (Ollama)]



\* \*\*Rust Sunucusu (`axum` \& `tokio`):\*\* Python'daki GIL (Global Interpreter Lock) bariyerini aşarak 5 uzman ajanın analiz isteklerini yerel LLM'e \*\*gerçek zamanlı paralel\*\* olarak fırlatır. Sıfır maliyetli soyutlama ile minimum RAM tüketir.

\* \*\*Tarayıcı Eklentisi (Manifest V3):\*\* Kullanıcının seçtiği metni yakalar, backend sonuçlarını dinamik ilerleme çubukları (Progress Bar) ile görselleştirir marketinden bağımsız ve manipülatif cümleleri \*\*web sayfasında doğrudan fosforlu kalemle boyayarak (Highlighting)\*\* kanıt izlerini gösterir.



\---



\## 📦 Kurulum ve Lokalde Çalıştırma



Ortak depodaki arkadaşlarınızın sistemi kendi laptoplarında çalıştırabilmesi için aşağıdaki adımları takip etmesi yeterlidir:



\### 1. Yerel LLM Yapılandırması (Ollama)

Sistem verilerinizin dışarı sızmaması için yerel modellerle çalışır.

\* \[Ollama](https://ollama.com) uygulamasını indirin ve başlatın.

\* Terminalden denetim modelini çekin:

&#x20;   ```bash

&#x20;   ollama run llama3

&#x20;   ```



\### 2. Backend Servisini Başlatma (Rust)

\* Projenin kök dizinine gelin ve yüksek performanslı yerel sunucuyu çalıştırın:

&#x20;   ```bash

&#x20;   cargo run

&#x20;   ```

\* Servis `127.0.0.1:3000` adresinde eklentiden gelecek analiz isteklerini dinlemeye başlayacaktır.



\### 3. Tarayıcı Eklentisini Aktif Etme

\* Chrome veya tabanlı tarayıcınızdan `chrome://extensions/` sayfasına gidin.

\* \*\*Geliştirici Modu\*\*'nu (Developer Mode) açın.

\* \*\*Paketlenmemiş öğe yükle\*\* (Load unpacked) diyerek projenin içindeki `extension` klasörünü seçip yükleyin.



\---



\## 🇪🇺 Düzenleyici Bağlam (Regulatory Context)



ManipuLens, \*\*AB Yapay Zeka Yasası (EU AI Act)\*\* göz önünde bulundurularak tasarlanmıştır:

\* \*\*Madde 5(1)(a):\*\* Önemli zarara neden olan bilinçaltı veya manipülatif teknikler kullanan yapay zeka sistemlerini yasaklar. ManipuLens, bu madde kapsamındaki teknik ihlalleri denetlemek adına nesnel kanıtlar üretir.

\* \*Yasal Uyarı:\* ManipuLens yasal bir uyumluluk sertifikası vermez; teknik kanıt izleri üretir. Yasal yorumlama kullanıcının sorumluluğundadır.



\---



\## 📝 Alıntı (Citation)



Bu projeyi akademik veya düzenleyici çalışmalarda kullanıyorsanız lütfen dökümantasyonunuza ekleyin:



```bibtex

@software{vural2026manipulens,

&#x20; author  = {Vural, Ulas},

&#x20; title   = {{ManipuLens}: A Multi-Agent Audit Pipeline for {LLM} Manipulation Detection},

&#x20; year    = {2026},

&#x20; url     = {\[https://github.com/uvural/manipulens](https://github.com/uvural/manipulens)},

&#x20; license = {Apache-2.0}

}





Teşekkürler

ManipuLens, Dr. Ulaş Vural liderliğinde yürütülen çoklu ajan (multi-agent) araştırma vizyonunun bir parçası olarak geliştirilmiştir. Akademik ve teknik yönlendirmelerinden ötürü hocamıza teşekkür ederiz.





\---



\## Adım 2: Projenin Kök Dizinindeki `LICENSE` Dosyası



Ulaş Hocanın akademik dökümanında lisans türü Apache 2.0 olarak belirtilmiş. Projenin kurumsal ve akademik gücünü korumak için kök dizinde (`Cargo.toml` dosyasının yanında) uzantısız, düz bir metin dosyası açıp adını `LICENSE` koy ve içine şu standart metni yapıştır:



```text

Apache License

Version 2.0, January 2004

http://www.apache.org/licenses/



TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

(This project is licensed under the Apache License, Version 2.0)

