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

