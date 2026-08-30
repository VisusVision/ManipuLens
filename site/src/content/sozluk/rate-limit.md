---
terim: "Rate limit"
alan: manipulens
kume: sistem
kisaca: "Bir kullanıcının belirli sürede kaç kez kapıyı çalabileceğini sınırlayan sayaç. Ceza değildir; ortak işlem gücünün tek kişi tarafından tüketilmesini engeller."
kod_capasi: "src/auth.rs::RateWindow"
ilgili: ["endpoint", "bearer-token", "denetim-kaydi"]
esanlam: ["hız limiti", "istek sınırı"]
owner: TODO
needs_reverify: true
---

Analiz ucu kullanıcı başına 60 saniyede 10 isteğe izin verir. Sınır aşılırsa yalnız red değil, yeniden denemeye kalan saniye döner.
