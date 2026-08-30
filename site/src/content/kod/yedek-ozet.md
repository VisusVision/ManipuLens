---
title: "Yedek özet: şefe ulaşılamazsa"
order: 7
parca: orkestrator
zorluk: orta
dosya: "src/orchestrator.rs"
aralik: [339, 360]
dil: rust
ne_yapiyor: "Sentezör çağrısı başarısız olursa altı rapordan en yüksek güven puanlısını seçip yerel bir özet kurar."
neden_boyle: "Sentezör tek hata noktası olmasın diye. Yedek yol yeni bir model çağrısı yapmıyor — zaten elde olan raporlardan en güçlüsünü seçiyor, yani ağ ya da model tamamen çökse bile çalışıyor."
kaldirirsak: "Şef çağrısı her başarısız olduğunda kullanıcı boş ekran görür. Altı ajanın yaptığı iş çöpe gider, oysa elde kullanılabilir sonuç vardır."
notlar:
  - satirlar: [2, 5]
    metin: "Önce yalnız `detected` olanlar süzülüyor, sonra en yüksek güven puanlısı seçiliyor. `total_cmp` ondalık sayıları güvenle karşılaştırmak için."
  - satirlar: [7, 8]
    metin: "`match` iki durumu ayırıyor: bir bulgu var mı, yok mu. Rust'ta “boş olabilir” durumu unutulamıyor — derleyici iki dalı da yazdırıyor."
  - satirlar: [9, 20]
    metin: "Özet metni kullanıcıya durumu açıkça söylüyor: yönetici ajana ulaşılamadı. Eksik yoldan gelen sonuç eksik olduğunu itiraf ediyor."
sina:
  - soru: "Yedek yol neden yeni bir model çağrısı yapmıyor?"
    cevap: "Yedek yolun devreye girme sebebi zaten model/ağ tarafındaki bir arıza; yeni bir çağrı aynı sebeple başarısız olurdu."
  - soru: "Hiçbir ajan bir şey bulamamışsa ne dönüyor?"
    cevap: "`None` dalı çalışır ve “uzman ajanlar manipülasyon tespit etmedi” özeti üretilir."
owner: TODO
needs_reverify: true
---

Bu fonksiyon birim testli: `fallback_summary_picks_highest_confidence`. Yedek yolların test edilmesi kolay unutulur, çünkü mutlu yol zaten çalışıyordur.
