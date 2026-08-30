---
terim: "llama3"
alan: genel
kume: yapay-zeka
kisaca: "ManipuLens'in yerelde çalıştırdığı dil modelinin adı. Ajanlardan biri değildir; altı uzmanın da kullandığı ortak motordur."
kod_capasi: "src/agents.rs::call_ollama_agent"
ilgili: ["yerel-model", "ollama", "ajan"]
esanlam: ["Llama 3"]
owner: TODO
needs_reverify: true
---

Kod, Ollama isteklerinde model adını `llama3` olarak gönderir. Altı farklı uzman görünmesinin sebebi altı model değil, aynı modele verilen farklı sistem promptlarıdır.
