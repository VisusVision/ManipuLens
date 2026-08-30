---
title: "Model yerelde düşünür"
order: 5
kicker: "Metin makineden çıkmıyor"
sade: "Her ajan aynı modele farklı bir soru sorar. Model senin bilgisayarında çalışır; metin hiçbir buluta gitmez."
kod: "src/agents.rs:37 — call_ollama_agent, llama3"
sure: "donanıma göre saniyeler"
hata: "Ollama kapalıysa ajanlar boş döner; bulut yedeği yoktur, bu bilinçli bir karardır."
owner: TODO
needs_reverify: true
---

`{OLLAMA_URL}/api/generate` çağrılır, `stream: false` ve `format: "json"` ile katı JSON çıktısı zorlanır. Ajanları birbirinden ayıran tek şey sistem promptudur (`agents.rs:72-176`); altında hepsi aynı modeldir. Model yanlış dilde cevap verirse `wrong_language()` bunu sezer ve `repair_language()` alanları yeniden çevirir.
