---
title: "İstek gövdesi ve Bearer başlığı"
order: 2
parca: uzanti
zorluk: baslangic
dosya: "extension/background.js"
aralik: [107, 126]
dil: js
ne_yapiyor: "Seçilen metni yerel sunucuya gönderir ve oturum token'ını istekle birlikte taşır."
neden_boyle: "Token gövdede değil `Authorization` başlığında gidiyor. Başlık, kimlik bilgisi taşımak için tasarlanmış standart yer; gövdeye koymak onu log'lara ve önbelleklere sızdırma riski yaratır."
kaldirirsak: "Sunucu isteği 401 ile geri çevirir. Kimlik olmadan hız limiti uygulanamaz, limit olmadan makine herkesin hesap makinesine döner."
notlar:
  - satirlar: [1, 3]
    metin: "`fetch` bir Promise döndürür; `await` ile cevabın gelmesi bekleniyor. Bu satır ağ üzerinden değil, kendi makinene gidiyor."
  - satirlar: [7, 8]
    metin: "Token şablon dizesiyle `Bearer <token>` biçimine sokuluyor. Sunucu tam olarak bu biçimi bekliyor."
  - satirlar: [17, 18]
    metin: "401 gelirse oturum bilgisi yerel depodan siliniyor. Oturumun ölmesi sessizce değil, görünür biçimde ele alınıyor."
sina:
  - soru: "Token neden gövdede değil başlıkta?"
    cevap: "`Authorization` başlığı kimlik taşımak için standart yer; gövdeye konsa log ve önbelleklere sızma riski artar."
  - soru: "401 dönerse kod ne yapıyor?"
    cevap: "Yerel depodaki `isLoggedIn` ve `authToken` alanlarını temizliyor, böylece arayüz kullanıcıdan yeniden giriş isteyebiliyor."
owner: TODO
needs_reverify: true
---

HTTP isteğinin anatomisi: adres, yöntem, başlıklar, gövde. Dördü de burada tek bir çağrıda görünüyor.
