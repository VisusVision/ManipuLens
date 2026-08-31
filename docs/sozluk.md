<!-- URETILMIS DOSYA - ELLE DUZENLEME.
     Kaynak: Site/src/content/sozluk/*.md · Uretici: Site/scripts/build-sozluk.mjs
     Yeniden uretmek icin: cd Site && npm run sozluk -->

# ManipuLens Sözlüğü

42 terim · son üretim 2026-08-31

Ekibin ortak dil sözleşmesi. Sitedeki karşılığı: `/sozluk`.

## Genel terimler (29)

ManipuLens'ten bağımsız olarak her yerde geçerli kavramlar.

### Manipülasyon dili (11)

#### Aciliyet

Karar için zamanını kısaltma tekniği. Düşünürsen vazgeçeceğini bilir.

> **Örnek:** “Bu fiyat bugün gece yarısı bitiyor!”

Test basit: süre kalkarsa aynı kararı verir miydin? Cevap hayırsa karar senin değildi.

İlgili: `davranissal-manipulasyon`, `kitlik`

#### Algısal manipülasyon

Gerçeği değil, gerçeğin çerçevesini değiştirme. Aynı sayı “yüzde 10 yağlı” ya da “yüzde 90 yağsız” olabilir.

> **Örnek:** “Ameliyatta hayatta kalma oranı %90” ile “ölüm oranı %10” aynı veridir.

Kodda: `src/agents.rs::analyze_perceptual`

Veri doğru olduğu için yakalaması zor. Yalan yok; seçilmiş bir kutu var.

İlgili: `cerceveleme`, `manipulasyon`

#### Çerçeveleme

Bilgiyi hangi kutunun içinde sunduğun. Kutu, çoğu zaman içerikten daha çok karar verir.

> **Örnek:** “Vergi indirimi” ile “bütçe kesintisi” aynı kalemi anlatabilir.

Çerçevesiz cümle yoktur; sorun çerçevenin varlığı değil, gizlenmesidir.

İlgili: `algisal-manipulasyon`

#### Davranışsal manipülasyon

Seni hemen bir şey yapmaya iten kurgu: geri sayım, kıtlık, son şans.

> **Örnek:** “Sepetindeki ürün 9 dakika sonra silinecek.”

Kodda: `src/agents.rs::analyze_behavioral`

Hedefi fikrin değil, parmağın. Düşünme süresini kısaltmak tek başına bir manipülasyon tekniğidir.

İlgili: `aciliyet`, `kitlik`

#### Dilsel manipülasyon

Cümlenin kuruluşuyla yapılan yönlendirme: belirsiz özne, edilgen çatı, ölçülemez abartı.

> **Örnek:** “Bilim insanları söylüyor.” — hangi bilim insanı, hangi çalışma?

Kodda: `src/agents.rs::analyze_linguistic`

En kolay yakalanan tür, çünkü izi cümlenin kendisinde durur; duyguya değil dilbilgisine bakılır.

İlgili: `manipulasyon`, `ajan`

#### Kıtlık

Az olduğunu söyleyerek değeri şişirme. Rakam çoğu zaman uydurmadır.

> **Örnek:** “Son 3 ürün kaldı!”

Gerçek kıtlık da vardır; farkı, sayının doğrulanabilir olup olmadığıdır. Sayfayı yenileyince rakam değişiyorsa cevabın var.

İlgili: `aciliyet`, `davranissal-manipulasyon`

#### Manipülasyon

Seni ikna etmek değil, yönlendirmek için kurulmuş dil. Fikrini değiştirir gibi görünür, aslında kararını senin yerine verir.

> **Örnek:** “Akıllı insanlar bu fırsatı kaçırmaz.”

İkna argüman sunar ve seni tartışmaya davet eder; manipülasyon argümanı atlar, doğrudan karara zıplar. ManipuLens'in altı ajanı da bu farkı arar.

İlgili: `cerceveleme`, `guven-puani`

#### Pazarlama manipülasyonu

Satış amacını gizleyen anlatı. Tavsiye gibi duran reklam.

> **Örnek:** “Ben de yıllarca denedim, sonunda bunu buldum — linki açıklamada.”

Kodda: `src/agents.rs::analyze_marketing`

Sorun ürünün satılması değil, satıcı ile tanık rolünün aynı ağızda birleşmesi.

İlgili: `manipulasyon`, `sosyal-kanit`

#### Psikolojik manipülasyon

Duyguya basma: korku, suçluluk, gurur. Argüman yerine his bırakır.

> **Örnek:** “Ailesini gerçekten seven biri bunu ertelemez.”

Kodda: `src/agents.rs::analyze_psychological`

Cümle mantıklı görünebilir; ölçüt cümlenin doğruluğu değil, seni hangi duyguyla baş başa bıraktığıdır.

İlgili: `manipulasyon`, `aciliyet`

#### Sosyal kanıt

“Bunu 10.000 kişi aldı.” Doğru olabilir; yine de senin için doğru olduğunu göstermez.

> **Örnek:** “4.8 yıldız, 12.000 değerlendirme — tereddüt etmeyin.”

Bir seçimi çok kişinin yapması, o seçimin senin koşullarına uyduğunu değil, popüler olduğunu kanıtlar.

İlgili: `sosyal-manipulasyon`

#### Sosyal manipülasyon

Kalabalığı kanıt gibi gösterme. “Herkes alıyor” bir veri değil, bir baskı.

> **Örnek:** “Senin yaşındaki herkes çoktan geçti bu aşamayı.”

Kodda: `src/agents.rs::analyze_social`

Yalnız kalma korkusunu kullanır. Kalabalığın büyüklüğü iddianın doğruluğu hakkında hiçbir şey söylemez.

İlgili: `sosyal-kanit`, `manipulasyon`

### Yapay zekâ (10)

#### Context

Modelin cevap verirken aynı anda önünde tuttuğu bilgi masası. Kalıcı hafıza değildir; masadan düşen bilgiyi model artık göremez.

Sistem promptu, kullanıcı metni ve önceki mesajlar bu sınırlı alana birlikte sığar. Alan dolunca eski veya düşük öncelikli bilgi dışarıda kalır.

İlgili: `token`, `prompt`, `sistem-promptu`

#### Halüsinasyon

Modelin emin bir tonla uydurması. Yanlış cevap her zaman yanlış görünmez.

ManipuLens bunu JSON zorunluluğu ve güven puanıyla sınırlar; tamamen ortadan kaldırmaz. Rapor kanıt değil, işaret.

İlgili: `llm`, `fallback`, `guven-puani`

#### JSON modu

Modelden serbest yazı yerine makinenin okuyabileceği düzenli bir nesne isteme biçimi. Cevabın doğru olduğu değil, biçiminin denetlenebilir olduğu anlamına gelir.

Kodda: `src/agents.rs:45-47`

ManipuLens Ollama'ya `format: "json"` gönderir. Böylece modelin cevabın başına açıklama ekleyip ayrıştırmayı bozma ihtimali azalır; alanların anlamı yine ayrıca doğrulanmalıdır.

İlgili: `llm`, `agent-analysis`, `final-report`

#### llama3

ManipuLens'in yerelde çalıştırdığı dil modelinin adı. Ajanlardan biri değildir; altı uzmanın da kullandığı ortak motordur.

Kodda: `src/agents.rs::call_ollama_agent`

Kod, Ollama isteklerinde model adını `llama3` olarak gönderir. Altı farklı uzman görünmesinin sebebi altı model değil, aynı modele verilen farklı sistem promptlarıdır.

İlgili: `yerel-model`, `ollama`, `ajan`

#### LLM

Çok miktarda metinle eğitilmiş, bir sonraki kelimeyi tahmin ederek cümle kuran model. Bilgi tabanı değil, dil makinesidir.

“Anlıyor” demek yerine “örüntüye bakıp devamını kestiriyor” demek daha doğru. ManipuLens'te altı ajanın da altında aynı LLM var; farkı sistem promptu yapar.

İlgili: `prompt`, `halusinasyon`, `yerel-model`

#### Ollama

Yerel modeli indirip çalıştıran program. ManipuLens modele doğrudan değil, Ollama üzerinden seslenir.

Kodda: `localhost:11434`

Ollama kapalıysa analiz başlamaz; bu bir hata değil, tasarımın sonucudur — bulut yedeği yok.

İlgili: `yerel-model`, `llm`

#### Prompt

Modele verdiğin iş emri. Sihirli kelime değil, net tarif: ne isteniyor, hangi biçimde, hangi sınırla.

Kötü sonuçların çoğu modelin yeteneğinden değil, tarifin belirsizliğinden gelir.

İlgili: `sistem-promptu`, `llm`

#### Sistem promptu

Modele her konuşmadan önce verilen sabit rol tarifi. Ajanın kim olduğunu ve neye bakacağını o belirler.

Kodda: `src/agents.rs:72-176`

ManipuLens'in altı ajanı aynı modeli kullanır; birbirinden yalnız sistem promptuyla ayrılır. Ajan farkı kodda değil, tarifte.

İlgili: `prompt`, `ajan`

#### Token

Metnin model tarafından işlenen küçük parçalarından biri. Kelimeyle aynı şey değildir; tek kelime birkaç parçaya ayrılabilir.

Model metni doğrudan kelimeler halinde okumaz; önce sayısal parçalara ayırır. İstek sınırları ve kullanım maliyeti çoğunlukla bu parçaların sayısıyla ölçülür.

İlgili: `llm`, `context`

#### Yerel model

Bilgisayarının içinde çalışan model. İnternete çıkmaz, veriyi kimseye göndermez. Karşılığı: kendi işlemcin yorulur.

Kodda: `OLLAMA_URL, src/agents.rs:37`

Gizlilik iddiasının temeli budur: metin makineden çıkmıyorsa sızabileceği bir yer de yoktur.

İlgili: `ollama`, `llm`

### Web, uzantı, altyapı (8)

#### <mark> vurgusu

Bir metin parçasını sayfa üzerinde anlamlı biçimde işaretleyen HTML etiketi. Sadece renk değildir; tarayıcıya bu parçanın özellikle öne çıkarıldığını söyler.

Kodda: `extension/background.js::makeMark`

Uzantı eşleşen cümleyi ajan rengi, alt çizgi ve açıklama etiketi taşıyan bir `<mark>` ile sarar. Temel stil doğrudan öğeye yazıldığı için sıkı sayfa güvenlik kurallarında da görünür kalır.

İlgili: `dom`, `target-sentences`

#### Bearer token

Bir isteği yapanın geçerli oturuma sahip olduğunu gösteren geçici anahtar. Parola değildir; ele geçiren kişi süresi dolana kadar kullanıcı gibi davranabilir.

Kodda: `extension/background.js:113`

Uzantı anahtarı `Authorization: Bearer <token>` başlığında yollar. Sunucu kullanıcı kimliğini istemcinin beyanından değil, bu anahtarın SQLite oturum kaydından türetir.

İlgili: `endpoint`, `rate-limit`

#### CORS

Tarayıcının “bu sunucuya kim seslenebilir” kuralı. Gevşek bırakılırsa senin sunucunla başkası da konuşabilir.

Kodda: `src/main.rs`

Bilinen risk olarak açık duruyor; ayrıntısı Gizlilik ve güvenlik sayfasında.

İlgili: `endpoint`

#### Docker

Bir uygulamayı bağımlılıklarıyla birlikte taşınabilir bir kutuda çalıştırma aracı. Sanal makine değildir; ana işletim sisteminin çekirdeğini paylaşır.

ManipuLens'in bugünkü kurulum yolu Docker kullanmıyor; Rust sunucusu ve Ollama ayrı çalıştırılıyor. Terim yol haritasındaki paketleme seçeneğini anlamak için sözlükte yer alır, çalışan özellik gibi sunulmaz.

İlgili: `endpoint`, `yerel-model`

#### DOM

Tarayıcının sayfayı başlık, paragraf ve düğme gibi düğümlerden oluşan canlı bir ağaç olarak göstermesi. HTML dosyasının kendisi değil, kodun değiştirebildiği çalışan hâlidir.

Kodda: `extension/background.js::highlightTextInPage`

ManipuLens bu ağacın metin düğümlerini `TreeWalker` ile gezer. Script, stil ve zaten işaretlenmiş düğümleri atlayarak sayfanın davranışını bozmamaya çalışır.

İlgili: `mark-vurgusu`, `service-worker`

#### Endpoint

Sunucunun dinlediği adreslerden biri. Her adres tek bir işi yapar: analiz et, giriş yap, geçmişi getir.

Kodda: `src/main.rs:1007-1018`

ManipuLens sunucusunda on adet var. Uzantı bunlardan yalnız birkaçını çağırır; kalanı hesap ve geçmiş içindir.

İlgili: `cors`

#### Manifest V3

Chrome uzantısının izinlerini ve çalışma parçalarını tarif eden güncel paket kuralı. Özellik değildir; uzantının tarayıcıya sunduğu kimlik kartıdır.

Kodda: `extension/manifest.json`

ManipuLens burada sağ tık, betik çalıştırma, aktif sekme, depolama ve pencere izinlerini bildirir. V3 kalıcı arka plan sayfası yerine gerektiğinde uyanan service worker kullanır.

İlgili: `service-worker`, `dom`

#### Service worker

Uzantının işi gelince uyanan, bitince uyuyan arka plan görevlisi. Sürekli açık bir pencere değildir; bu yüzden durumunu ayrıca saklaması gerekir.

Kodda: `extension/background.js`

Sağ tık olayını dinler, yerel sunucuya isteği yollar ve boyama kodunu aktif sekmeye enjekte eder. Oturum gibi kalıcı bilgiler `chrome.storage.local` içinde tutulur.

İlgili: `manifest-v3`, `dom`, `endpoint`

## ManipuLens'e özel terimler (13)

Yalnız bu projede anlamı olan terimler; her birinin kodda karşılığı var.

### ManipuLens sistemi (13)

#### AgentAnalysis

Bir uzman ajanın doldurduğu standart rapor kalıbı. Serbest paragraf değildir; bulgu, güven, açıklama ve hedef cümleler aynı kutuda taşınır.

Kodda: `src/types.rs::AgentAnalysis`

Rust yapısı `manipulation_type`, `detected`, `confidence_score`, `aciklama` ve `target_sentences` alanlarını zorunlu tutar. Sentezör altı uzmanı bu ortak biçim sayesinde karşılaştırabilir.

İlgili: `uzman-ajan`, `guven-puani`, `target-sentences`, `final-report`

#### Ajan

Tek bir soruyu soran uzman. ManipuLens'te altı tane var, her biri metne farklı bir açıdan bakar.

Kodda: `src/agents.rs`

Ajan ayrı bir program değil; aynı modele farklı rol tarifiyle sorulan ayrı bir sorudur.

İlgili: `orkestrator`, `sistem-promptu`

#### Denetim kaydı

Sistemde ne zaman hangi olayın yaşandığını satır satır tutan iz defteri. Analiz geçmişi değildir; hata ve güvenlik olaylarını sonradan incelemek içindir.

Kodda: `src/audit.rs`

ManipuLens günlük JSONL dosyası kullanır ve analiz metninin yalnız ilk 120 karakterini kaydeder. Tam metni yazmamak, kayıt yararlılığı ile veri sızıntısı riskini dengeler.

İlgili: `rate-limit`, `endpoint`

#### dominant_manipulation

Raporun en baskın bulduğu yönlendirme türü. Metindeki bütün teknikleri saymaz; güvenilir bulgular arasından öne çıkanı adlandırır.

Kodda: `src/types.rs::FinalReport`

Sentezör güven puanı ve hedef cümlenin metinde gerçekten bulunmasını tartar. Güvenilir bulgu yoksa alan `Yok` olur; boş bırakılmaz.

İlgili: `final-report`, `manipulation-type`, `guven-puani`

#### Fallback (yedek yol)

Asıl yol tıkanırsa devreye giren yedek yol. Şef ajana ulaşılamazsa altı rapordan en yüksek güvenlisi seçilir; sistem boş dönmez.

Kodda: `src/orchestrator.rs::fallback_summary`

Yedek yolun devreye girdiği rapor, tam rapordan daha zayıftır. Arayüzün bunu gizlememesi gerekir.

İlgili: `sentezor`, `guven-puani`

#### FinalReport

Kullanıcıya dönen birleşik analiz raporunun kalıbı. Tek ajanın sözü değil; şefin kararıyla altı ayrıntılı raporu birlikte taşır.

Kodda: `src/types.rs::FinalReport`

Yapıda genel karar, baskın tür, kısa sonuç, olası ürün ve altı `AgentAnalysis` bulunur. Şef çökerse aynı yapı `fallback_summary()` ile eksik ama geçerli biçimde doldurulur.

İlgili: `agent-analysis`, `sentezor`, `dominant-manipulation`

#### Güven puanı

Ajanın “bundan ne kadar eminim” değeri, 0 ile 1 arası. Kesinlik değil, ajanın kendi tahmini.

> **Örnek:** 0.91 güven, “bu cümle kesin manipülatif” demek değil; “ajan çok emin” demek.

Kodda: `src/agents.rs::AgentAnalysis.confidence_score`

Yüksek puan modelin kendine güvenini ölçer, gerçeği değil. Rapor okunurken puan tek başına delil sayılmaz.

İlgili: `ajan`, `fallback`

#### manipulation_type

Raporda “bu hangi tür manipülasyon” sorusunun cevabını taşıyan alan. Ajanın etiketi.

Kodda: `src/agents.rs::AgentAnalysis`

Sitedeki ajan renkleri ve uzantıdaki ilerleme çubuğu bu alanın değerine göre eşlenir; değer değişirse iki yer birden bozulur.

İlgili: `ajan`, `target-sentences`

#### Orkestratör

İşi kendisi yapmayan yönetici. Altı uzmanı aynı anda çalıştırır, raporları toplar, şefe verir.

Kodda: `src/orchestrator.rs::run_orchestrator`

Altı ajanı sırayla değil paralel çağırır; toplam süre en yavaş ajanın süresi kadardır, altısının toplamı kadar değil.

İlgili: `ajan`, `sentezor`

#### Rate limit

Bir kullanıcının belirli sürede kaç kez kapıyı çalabileceğini sınırlayan sayaç. Ceza değildir; ortak işlem gücünün tek kişi tarafından tüketilmesini engeller.

Kodda: `src/auth.rs::RateWindow`

Analiz ucu kullanıcı başına 60 saniyede 10 isteğe izin verir. Sınır aşılırsa yalnız red değil, yeniden denemeye kalan saniye döner.

İlgili: `endpoint`, `bearer-token`, `denetim-kaydi`

#### Sentezör

Altı raporu tek karara indiren şef. Uzmanları dinler, çelişkileri çözer, son raporu yazar.

Kodda: `src/orchestrator.rs`

Şef de bir LLM çağrısıdır; ulaşılamazsa sistem boş dönmez, yedek yola geçer.

İlgili: `orkestrator`, `fallback`

#### target_sentences

Ajanın “suçlu” dediği cümleler. Sayfada renkli boyanan tam olarak bunlardır.

Kodda: `src/agents.rs::AgentAnalysis`

Boş dönerse uzantı hiçbir şey boyamaz; kullanıcı “çalışmadı” sanır. Boş sonuç da bir sonuçtur, arayüzde söylenmelidir.

İlgili: `manipulation-type`

#### Uzman ajan

Metne yalnız bir bakış açısından bakan altı okuyucudan biri. Ayrı bir model değildir; ortak modele dar bir görev veren iş rolüdür.

Kodda: `src/agents.rs:72-176`

Dilsel, psikolojik, davranışsal, algısal, sosyal ve pazarlama uzmanlarının her biri aynı çağrı yardımcısını kullanır. Aralarındaki sınırı sistem promptu ve döndürdükleri `manipulation_type` değeri kurar.

İlgili: `ajan`, `orkestrator`, `sentezor`
