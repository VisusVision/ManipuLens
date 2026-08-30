---
terim: "Orkestratör"
alan: manipulens
kume: sistem
kisaca: "İşi kendisi yapmayan yönetici. Altı uzmanı aynı anda çalıştırır, raporları toplar, şefe verir."
kod_capasi: "src/orchestrator.rs::run_orchestrator"
ilgili: ["ajan", "sentezor"]
esanlam: ["orchestrator"]
owner: TODO
needs_reverify: true
---

Altı ajanı sırayla değil paralel çağırır; toplam süre en yavaş ajanın süresi kadardır, altısının toplamı kadar değil.
