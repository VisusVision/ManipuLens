---
terim: "FinalReport"
alan: manipulens
kume: sistem
kisaca: "Kullanıcıya dönen birleşik analiz raporunun kalıbı. Tek ajanın sözü değil; şefin kararıyla altı ayrıntılı raporu birlikte taşır."
kod_capasi: "src/types.rs::FinalReport"
ilgili: ["agent-analysis", "sentezor", "dominant-manipulation"]
esanlam: ["nihai rapor", "final report"]
owner: TODO
needs_reverify: true
---

Yapıda genel karar, baskın tür, kısa sonuç, olası ürün ve altı `AgentAnalysis` bulunur. Şef çökerse aynı yapı `fallback_summary()` ile eksik ama geçerli biçimde doldurulur.
