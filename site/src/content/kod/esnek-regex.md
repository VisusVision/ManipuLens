---
title: "Esnek regex ile cümle eşleştirme"
order: 9
parca: uzanti
zorluk: ileri
dosya: "extension/background.js"
aralik: [303, 317]
dil: js
ne_yapiyor: "Modelin döndürdüğü cümleyi sayfadaki metinde bulmak için toleranslı bir arama kalıbı kurar."
neden_boyle: "Eski kod `text.includes()` kullanıyordu. Model cümleyi tek bir tırnak veya boşluk farkıyla döndürdüğünde hiçbir şey işaretlenmiyordu — kullanıcı için sistem “çalışmamış” görünüyordu."
kaldirirsak: "Vurgulama çoğu zaman sessizce başarısız olur. En kötü hata türü bu: sistem hata vermiyor, sadece hiçbir şey yapmıyor."
notlar:
  - satirlar: [2, 5]
    metin: "Baştaki ve sondaki tırnak, üç nokta ve boşluklar temizleniyor. Modelin cümleyi alıntı içinde döndürmesi çok yaygın."
  - satirlar: [6, 6]
    metin: "Beş karakterden kısa parçalar reddediliyor. Kısa kalıp sayfadaki her yere uyar ve yanlış yerleri boyar."
  - satirlar: [7, 9]
    metin: "Önce regex'in özel karakterleri kaçırılıyor, sonra boşluklar esnek hâle getiriliyor — normal boşluk da kırılmaz boşluk da eşleşiyor."
  - satirlar: [10, 14]
    metin: "`try/catch` içinde derleniyor: geçersiz bir kalıp tüm boyamayı düşürmesin diye. Hata durumunda `null` dönüyor ve o cümle atlanıyor."
sina:
  - soru: "Neden beş karakter alt sınırı var?"
    cevap: "Kısa kalıplar sayfadaki pek çok yere uyar ve alakasız metinleri boyar; alt sınır yanlış pozitifleri engelliyor."
  - soru: '`\s+` yerine neden `[\s\u00A0]+` yazılmış?'
    cevap: 'Web sayfalarında kırılmaz boşluk (`&nbsp;`, U+00A0) çok yaygın ve normal `\s` her ortamda onu kapsamıyor.'
owner: TODO
needs_reverify: true
---

On beş satırlık bu fonksiyon bir hata düzeltmesinin izini taşıyor. Kodun neden böyle olduğunu anlamak için önce neyin bozulduğunu bilmek gerekiyor.
