---
terim: "Bearer token"
alan: genel
kume: web
kisaca: "Bir isteği yapanın geçerli oturuma sahip olduğunu gösteren geçici anahtar. Parola değildir; ele geçiren kişi süresi dolana kadar kullanıcı gibi davranabilir."
kod_capasi: "extension/background.js:113"
ilgili: ["endpoint", "rate-limit"]
esanlam: ["Bearer", "oturum tokenı", "Authorization token"]
owner: TODO
needs_reverify: true
---

Uzantı anahtarı `Authorization: Bearer <token>` başlığında yollar. Sunucu kullanıcı kimliğini istemcinin beyanından değil, bu anahtarın SQLite oturum kaydından türetir.
