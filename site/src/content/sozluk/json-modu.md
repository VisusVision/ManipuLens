---
terim: "JSON modu"
alan: genel
kume: yapay-zeka
kisaca: "Modelden serbest yazı yerine makinenin okuyabileceği düzenli bir nesne isteme biçimi. Cevabın doğru olduğu değil, biçiminin denetlenebilir olduğu anlamına gelir."
kod_capasi: "src/agents.rs:45-47"
ilgili: ["llm", "agent-analysis", "final-report"]
esanlam: ["JSON mode", "format json"]
owner: TODO
needs_reverify: true
---

ManipuLens Ollama'ya `format: "json"` gönderir. Böylece modelin cevabın başına açıklama ekleyip ayrıştırmayı bozma ihtimali azalır; alanların anlamı yine ayrıca doğrulanmalıdır.
