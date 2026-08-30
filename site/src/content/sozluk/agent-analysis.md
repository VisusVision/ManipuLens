---
terim: "AgentAnalysis"
alan: manipulens
kume: sistem
kisaca: "Bir uzman ajanın doldurduğu standart rapor kalıbı. Serbest paragraf değildir; bulgu, güven, açıklama ve hedef cümleler aynı kutuda taşınır."
kod_capasi: "src/types.rs::AgentAnalysis"
ilgili: ["uzman-ajan", "guven-puani", "target-sentences", "final-report"]
esanlam: ["ajan analizi", "Agent Analysis"]
owner: TODO
needs_reverify: true
---

Rust yapısı `manipulation_type`, `detected`, `confidence_score`, `aciklama` ve `target_sentences` alanlarını zorunlu tutar. Sentezör altı uzmanı bu ortak biçim sayesinde karşılaştırabilir.
