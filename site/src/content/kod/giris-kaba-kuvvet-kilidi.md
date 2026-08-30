---
title: "Giriş kaba-kuvvet kilidi"
order: 15
parca: hesap
zorluk: orta
dosya: "src/auth.rs"
aralik: [46, 90]
dil: rust
ne_yapiyor: "Aynı hesaba art arda yapılan başarısız girişleri sayar, eşik aşılınca hesabı geçici kilitler ve başarılı girişte sayacı temizler."
neden_boyle: "Parola karması tek denemeyi pahalılaştırır ama sınırsız denemeyi durdurmaz. Bellek içi küçük sayaç çevrim içi tahmini yavaşlatır; başarılı kullanıcıyı eski hataları yüzünden cezalandırmamak için başarıda sıfırlanır."
kaldirirsak: "Bir saldırgan aynı e-posta için durmadan parola deneyebilir. Doğru parolayı bulamasa bile bcrypt hesapları işlemciyi tüketip giriş hizmetini yavaşlatır."
notlar:
  - satirlar: [3, 9]
    metin: "Koruma yalnız üç değer tutuyor: pencere başlangıcı, hata sayısı ve kilidin biteceği an. Sunucu yeniden başlarsa sıfırlanması dosyanın başındaki tasarım notunda kabul edilmiş."
  - satirlar: [11, 13]
    metin: "Sınırlar kodda sabit: 15 dakikada 8 hata, ardından 15 dakika kilit. Bu sayılar arayüz metninin gerçek kaynağı."
  - satirlar: [20, 27]
    metin: "`check` yalnız mevcut kilidi okuyor ve kalan saniyeyi döndürüyor. Başarısızlığı kaydetmek ayrı fonksiyonun işi; kontrolün kendisi sayacı değiştirmiyor."
  - satirlar: [29, 38]
    metin: "Eski pencere dolduysa sayaç önce sıfırlanıyor. Sekizinci hatada bitiş zamanı yazılıp hata sayacı temizleniyor; kilit durumunu artık `locked_until` taşıyor."
  - satirlar: [40, 45]
    metin: "Başarılı giriş üç alanın üçünü de temizliyor. Yalnız hata sayısını sıfırlamak eski kilit zamanını bırakıp doğru parolayı yine reddedebilirdi."
sina:
  - soru: "Bu sayaç neden veritabanında değil?"
    cevap: "Kısa ömürlü kötüye kullanım durumudur; yeniden başlatmada sıfırlanması kabul edilmiş ve bellek erişimi her girişte daha ucuzdur. Bedeli, yeniden başlatmanın kilidi kaldırmasıdır."
  - soru: "`check` neden başarısız denemeyi kendi içinde kaydetmiyor?"
    cevap: "Fonksiyon parolanın doğru olup olmadığını bilmiyor. Denetleme ile sonucu kaydetmek ayrılınca çağıran taraf yalnız gerçek başarısızlıkta sayacı artırır."
owner: TODO
needs_reverify: true
---

Hız limiti bir uç noktasını korur; bu kilit tek bir hesabı korur. İkisi benzer sayaçlar kullansa da saldırı yüzeyleri ve sıfırlanma kuralları farklıdır.
