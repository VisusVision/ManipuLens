---
title: "Sağ tık menüsü kaydı"
order: 1
parca: uzanti
zorluk: baslangic
dosya: "extension/background.js"
aralik: [1, 10]
dil: js
ne_yapiyor: "Uzantı kurulduğunda tarayıcıya “bu metni ManipuLens ile analiz et” menü maddesini ekler."
neden_boyle: "Menü her açılışta değil, yalnız kurulumda bir kez kaydedilir. `onInstalled` tam olarak bunun için var; her başlatmada yeniden kaydetmek aynı menüyü çoğaltma riski taşırdı."
kaldirirsak: "Uzantı yüklenir, çalışır, ama kullanıcı onu hiçbir yerden tetikleyemez. Sistemin tek giriş kapısı bu on satır."
notlar:
  - satirlar: [2, 2]
    metin: "`onInstalled` yalnız kurulumda ve güncellemede çalışır. Tarayıcı her açıldığında değil."
  - satirlar: [4, 4]
    metin: "`id` sabit bir dize. Tıklama olayı geldiğinde hangi menüye basıldığını bu kimlikten anlıyoruz."
  - satirlar: [6, 6]
    metin: "`contexts: [\"selection\"]` menünün yalnız metin seçiliyken görünmesini sağlar. Boş sayfada sağ tıklayınca çıkmaz."
  - satirlar: [9, 9]
    metin: "Menü kaydından sonra istemci kimliği hazırlanıyor — kurulum anında yapılması gereken ikinci iş."
sina:
  - soru: "Menü neden boş bir sayfaya sağ tıklayınca çıkmıyor?"
    cevap: "`contexts` alanı `selection` olarak ayarlı; menü yalnız seçili metin varken gösteriliyor."
  - soru: "`onInstalled` yerine her başlatmada kaydetseydik ne olurdu?"
    cevap: "Aynı kimlikle ikinci kez menü oluşturulmaya çalışılır ve hata alınırdı; menü çoğalma riski doğardı."
owner: TODO
needs_reverify: true
---

Bu, sistemdeki en küçük tam örnek: bir olayı dinle, tek bir şey kaydet. Kodun tamamı on satır ama uzantının varlık sebebi burada başlıyor.
