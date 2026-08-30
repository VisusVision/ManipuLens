---
terim: "Güven puanı"
alan: manipulens
kume: sistem
kisaca: "Ajanın “bundan ne kadar eminim” değeri, 0 ile 1 arası. Kesinlik değil, ajanın kendi tahmini."
ornek: "0.91 güven, “bu cümle kesin manipülatif” demek değil; “ajan çok emin” demek."
kod_capasi: "src/agents.rs::AgentAnalysis.confidence_score"
ilgili: ["ajan", "fallback"]
esanlam: ["confidence score", "confidence_score"]
owner: TODO
needs_reverify: true
---

Yüksek puan modelin kendine güvenini ölçer, gerçeği değil. Rapor okunurken puan tek başına delil sayılmaz.
