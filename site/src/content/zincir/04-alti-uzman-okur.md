---
title: "Altı uzman aynı anda okur"
order: 4
kicker: "Paralel, sırayla değil"
sade: "Altı ajan metni altı farklı açıdan okur: dil, duygu, davranış, algı, sosyal baskı, satış. Hepsi aynı anda çalışır, biri diğerini beklemez."
kod: "src/orchestrator.rs:7 — tokio::join!"
sure: "en yavaş ajan kadar"
hata: "Bir ajan çökerse analiz çökmez: o ajan boş rapor döndürür, kalan beşi devam eder."
owner: TODO
needs_reverify: true
---

`tokio::join!` altı çağrıyı aynı anda başlatır, dolayısıyla toplam süre altısının toplamı değil en yavaşının süresidir. Her sonuç `unwrap_or_else(|_| fallback(<ad>))` ile sarılıdır; bu yüzden tek bir ajanın hatası zinciri kırmaz.
