---
title: "Hız limiti: sabit pencereli sayaç"
order: 4
parca: hesap
zorluk: baslangic
dosya: "src/auth.rs"
aralik: [18, 44]
dil: rust
ne_yapiyor: "Belirli bir süre içinde kaç istek yapıldığını sayar ve sınır aşılırsa kalan süreyi döndürür."
neden_boyle: "Sabit pencere (fixed window) en basit hız limiti algoritması: tek bir zaman damgası ve tek bir sayaç. Kayan pencere daha adil olurdu ama her isteğin zamanını saklamayı gerektirirdi — bu ölçekte gereksiz karmaşıklık."
kaldirirsak: "Tek kullanıcı arka arkaya yüzlerce analiz tetikleyebilir. Her analiz yedi model çağrısı olduğu için makine birkaç dakikada kilitlenir."
notlar:
  - satirlar: [3, 6]
    metin: "Yapının tuttuğu tek şey iki sayı: pencerenin ne zaman başladığı ve kaç istek geldiği. Bellekte neredeyse yer kaplamıyor."
  - satirlar: [8, 9]
    metin: "Sabitler kodda açıkça duruyor: 60 saniyede 10 analiz. Sitedeki tablo bu iki satırdan yazıldı."
  - satirlar: [16, 19]
    metin: "Pencere dolduysa sayaç sıfırlanıyor. “Sabit pencere” adı buradan geliyor — süre bitince her şey yeniden başlıyor."
  - satirlar: [20, 22]
    metin: "Limit aşıldıysa hata değil, **kalan saniye** dönüyor. Çağıran taraf bunu doğrudan kullanıcıya gösterebiliyor."
sina:
  - soru: "Sabit pencerenin bilinen zayıflığı nedir?"
    cevap: "Pencere sınırında yığılma: kullanıcı pencerenin sonunda 10, hemen ardından yeni pencerede 10 istek daha yapabilir, yani kısa sürede 20 istek geçebilir."
  - soru: "`allow` neden `Result<(), i64>` döndürüyor?"
    cevap: "Başarıda taşınacak veri yok; başarısızlıkta kullanıcıya söylenecek tek bir sayı var — kalan saniye."
owner: TODO
needs_reverify: true
---

Yirmi yedi satırda tam çalışan bir koruma. Kodun kısalığı yanıltmasın: bu parça olmasaydı analiz ucu kendi kendini yiyen bir kaynak deliğine dönerdi.
