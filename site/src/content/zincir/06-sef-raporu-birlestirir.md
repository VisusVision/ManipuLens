---
title: "Şef raporu birleştirir"
order: 6
kicker: "Altı rapor, tek karar"
sade: "Yedinci çağrı şefe gider: altı raporu okur, çelişkileri çözer, tek bir karar yazar. Baskın manipülasyon türü burada belirlenir."
kod: "src/orchestrator.rs:78 — sentezör promptu"
sure: "tek model çağrısı"
hata: "Şefe ulaşılamazsa `fallback_summary()` devreye girer: altı rapordan en yüksek güven puanlısını seçer. Sistem boş dönmez."
owner: TODO
needs_reverify: true
---

Sentezörün sistem promptu şefin işini tek cümlede tarif eder: altı uzman raporunu tek bir kalibre edilmiş karara indirmek. Çıktı `FinalReport`. Yedek yol birim testlidir (`fallback_summary_picks_highest_confidence`). Yedek yolun ürettiği rapor tam rapordan zayıftır; arayüzün bunu gizlememesi gerekir.
