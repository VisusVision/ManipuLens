---
title: "DOM ağacını güvenli tarama"
order: 11
parca: uzanti
zorluk: ileri
dosya: "extension/background.js"
aralik: [342, 380]
dil: js
ne_yapiyor: "Sayfadaki kullanılabilir metin düğümlerini toplar ve hepsini konumları korunarak tek bir arama metninde birleştirir."
neden_boyle: "Gerçek bir cümle bağlantı, kalın yazı veya span yüzünden birkaç DOM düğümüne bölünebilir. Her düğümü tek başına aramak görünürde aynı olan cümleyi bulamaz; birleşik metin ve konum haritası etiket sınırlarını aşan eşleşmeyi mümkün kılar."
kaldirirsak: "Uzantı yalnız tek HTML düğümüne sığan basit cümleleri boyar. Haber sitelerindeki bağlantılı veya biçimlendirilmiş cümlelerin çoğu sessizce atlanır."
notlar:
  - satirlar: [1, 19]
    metin: "`TreeWalker` yalnız metin düğümlerini geziyor. Script, stil, form metni ve mevcut `<mark>` öğeleri reddediliyor; aksi hâlde kod veya eski vurgu yeniden boyanabilirdi."
  - satirlar: [21, 25]
    metin: "Düğümler önce ayrı bir diziye alınıyor. DOM, boyama sırasında değişeceği için canlı gezgini aynı anda değiştirip okumak düğüm atlamaya yol açardı."
  - satirlar: [27, 32]
    metin: "Yorum, eski yaklaşımın neden bozulduğunu kodun yanında saklıyor: cümlenin görünürde tek olması DOM'da tek düğüm olduğu anlamına gelmiyor."
  - satirlar: [33, 39]
    metin: "Her düğümün birleşik metindeki başlangıç ve bitiş konumu tutuluyor. Sonraki adım bir eşleşmeyi yeniden gerçek DOM düğümlerine bu haritayla dağıtıyor."
sina:
  - soru: "Neden yalnız `document.body.textContent` üzerinde arama yapmak yetmez?"
    cevap: "Metni buldurur ama eşleşmenin hangi DOM düğümlerine ait olduğunu kaybettirir; `<mark>` yerleştirmek için düğüm konumları gerekir."
  - soru: "Zaten işaretlenmiş `<mark>` öğeleri neden atlanıyor?"
    cevap: "Aynı analiz veya sonraki analiz eski vurgunun içine yeni vurgu koyup iç içe, bozuk DOM üretmesin diye."
owner: TODO
needs_reverify: true
---

Bu parça, ekranda gördüğümüz düz metin ile tarayıcının çalıştırdığı ağaç arasındaki farkı gösteriyor. Vurgulamanın zor kısmı rengi seçmek değil, cümleyi yapısını bozmadan yeniden bulmak.
