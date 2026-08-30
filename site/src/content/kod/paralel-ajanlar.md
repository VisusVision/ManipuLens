---
title: "Altı ajanı paralel çalıştırma"
order: 5
parca: orkestrator
zorluk: orta
dosya: "src/orchestrator.rs"
aralik: [5, 29]
dil: rust
ne_yapiyor: "Altı ajanı aynı anda başlatır, hepsinin bitmesini bekler ve çöken ajan için boş bir rapor hazırlar."
neden_boyle: "`tokio::join!` altı işi eşzamanlı yürütür. Sırayla çağırmak altı kat beklemek demekti ve ajanlar birbirinin çıktısına muhtaç olmadığı için bunun hiçbir karşılığı yoktu."
kaldirirsak: "`join!` yerine altı ayrı `await` yazılsaydı toplam süre yaklaşık altı katına çıkardı. `fallback` kapanışı kaldırılsaydı tek bir ajanın hatası tüm analizi düşürürdü."
notlar:
  - satirlar: [3, 11]
    metin: "`join!` altı çağrıyı aynı anda başlatıyor ve altısı da bitene kadar bekliyor. Toplam süre en yavaş ajanın süresi kadar."
  - satirlar: [13, 18]
    metin: "Hata mesajı dile göre önceden seçiliyor. Hata anında dil kararı vermek yerine baştan hazırlamak."
  - satirlar: [19, 25]
    metin: "`fallback` bir kapanış (closure): çöken ajanın yerine geçecek boş raporu üretiyor. `detected: false`, `confidence_score: 0.0` — yani “bir şey bulamadım” değil, “konuşamadım”."
sina:
  - soru: "Toplam analiz süresi neyle belirleniyor?"
    cevap: "En yavaş ajanın süresiyle. Altısı paralel çalıştığı için süreleri toplanmıyor."
  - soru: "Bir ajan çökerse kullanıcı ne görür?"
    cevap: "Analiz tamamlanır; çöken ajan boş rapor döndürür, kalan beş ajanın bulguları raporda yer alır."
owner: TODO
needs_reverify: true
---

Bu parça iki fikri aynı anda gösteriyor: eşzamanlılık ve hata yalıtımı. İkisi de bir satırlık sözdizimiyle değil, kasıtlı bir tasarımla geliyor.
