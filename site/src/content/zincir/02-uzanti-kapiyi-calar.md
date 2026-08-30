---
title: "Uzantı kapıyı çalar"
order: 2
kicker: "Bilgisayarının içinde"
sade: "Seçtiğin metin, internete değil, kendi bilgisayarında çalışan sunucuya gider. Adres `127.0.0.1` — yani bu makine."
kod: "POST http://127.0.0.1:3000/v1/analyze"
sure: "birkaç milisaniye"
hata: "Sunucu kapalıysa istek hiç ulaşmaz; uzantı bağlantı hatası gösterir."
owner: TODO
needs_reverify: true
---

`chrome.storage.local` içinden `client_id`, `currentUser`, `language` ve `authToken` okunur; istek `Authorization: Bearer <token>` başlığıyla gönderilir. Metin gövdede JSON olarak gider, sorgu dizesinde değil.
