---
title: "Sayfa boyanır"
order: 7
kicker: "Sonuç gözünün önünde"
sade: "Şüpheli cümleler sayfanın kendi üzerinde işaretlenir. Renk hangi ajanın yakaladığını söyler; üzerine gelince ajanın adı çıkar."
kod: "chrome.scripting.executeScript + highlightSentencesOnPage"
sure: "anında"
hata: "Hiçbir cümle eşleşmezse seçtiğin metnin tamamı baskın manipülasyon renginde işaretlenir."
owner: TODO
needs_reverify: true
---

Sonuç önce `chrome.storage.local`'a yazılır, popup oradan okur. Boyama betiği sayfaya enjekte edilir. Eşleştirme `includes()` değil esnek regex ile yapılır — model cümleyi tek karakter farklı döndürdüğünde eskiden hiçbir şey işaretlenmiyordu. Temel stiller `element.style` ile uygulanır, böylece katı CSP'li sitelerde de işaret görünür kalır.
