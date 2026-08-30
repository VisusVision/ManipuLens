---
title: "Metni seçersin"
order: 1
kicker: "Sende başlıyor"
sade: "Şüphelendiğin yazıyı fareyle seç, sağ tıkla, ManipuLens'i seç. Zincirin tamamı buradan tetikleniyor; başka hiçbir şey yapman gerekmiyor."
kod: "extension/background.js::contextMenus.onClicked"
sure: "anında"
hata: "Sağ tık menüsü görünmüyorsa uzantı yüklenmemiş ya da sayfa yenilenmemiştir."
owner: TODO
needs_reverify: true
---

Uzantı kurulduğunda `onInstalled` ile sağ tık menüsü kaydedilir. Tıklama `startAnalysisInBackground` fonksiyonunu çağırır; seçili metin buradan alınır. Manifest V3 olduğu için kalıcı arka plan sayfası yok, service worker uyanır ve işini bitirince uyur.
