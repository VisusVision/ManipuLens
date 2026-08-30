---
title: "Analiz kapısı: kimlik ve hız limiti"
order: 3
parca: sunucu
zorluk: orta
dosya: "src/main.rs"
aralik: [775, 800]
dil: rust
ne_yapiyor: "Analiz isteği modele ulaşmadan önce kimliği ve hız limitini kontrol eder."
neden_boyle: "Sıra bilinçli: önce kimlik, sonra limit. Limit kullanıcı başına uygulanıyor ve kullanıcı ancak kimlik doğrulandıktan sonra biliniyor. Ters sırada limit uygulanacak bir kimlik olmazdı."
kaldirirsak: "Uç yeniden herkese açılır. Kod yorumunun anlattığı eski duruma dönülür: adresi bilen herkes sınırsız analiz tetikleyebilir ve her analiz yedi model çağrısı demektir."
notlar:
  - satirlar: [8, 12]
    metin: "`let ... else` deseni: oturum yoksa fonksiyon burada biter. Rust'ta erken çıkışın okunaklı hâli."
  - satirlar: [9, 9]
    metin: "Reddedilen istek de denetim kaydına yazılıyor. Başarısız denemeler görünmezse saldırı da görünmez."
  - satirlar: [16, 18]
    metin: "Hız sayacı kullanıcının e-postasına göre tutuluyor — IP'ye göre değil. Aynı makinedeki iki kullanıcı ayrı sayılıyor."
  - satirlar: [19, 20]
    metin: "Limit aşılırsa kalan saniye hesaplanıp kullanıcıya söyleniyor. “Sonra dene” demek yerine ne kadar sonra olduğunu söylemek."
sina:
  - soru: "Hız limiti neden IP yerine e-postaya bağlı?"
    cevap: "Aynı makinedeki farklı kullanıcıları ayırt edebilmek için; IP tabanlı sayaç ikisini tek kullanıcı sayardı."
  - soru: "Kimlik kontrolü limitin önüne neden konuldu?"
    cevap: "Limit kullanıcı başına uygulanıyor; kullanıcının kim olduğu ancak kimlik doğrulandıktan sonra biliniyor."
owner: TODO
needs_reverify: true
---

Bu bölümdeki yorum satırı projenin en dürüst cümlelerinden biri: uç eskiden tamamen açıktı ve bu bilinçli olarak kapatıldı. Kodun kendi geçmişini anlatması nadir ama değerli.
