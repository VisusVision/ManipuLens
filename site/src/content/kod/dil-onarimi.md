---
title: "Model yanlış dilde cevap verirse"
order: 8
parca: orkestrator
zorluk: ileri
dosya: "src/orchestrator.rs"
aralik: [194, 215]
dil: rust
ne_yapiyor: "Modelin istenen dil yerine başka bir dilde cevap verip vermediğini sezgisel olarak tespit eder."
neden_boyle: "Modelin talimata uymaması kesin bir hata değil, olasılıksal bir davranış. Prompt'a “Türkçe yaz” yazmak yardımcı oluyor ama garanti etmiyor; garantiyi koda koymak gerekiyor."
kaldirirsak: "Kullanıcı Türkçe arayüzde İngilizce açıklama görür. Görünüşte küçük bir kusur ama güveni doğrudan zedeler: sistem kendi dilini kontrol edemiyor demektir."
notlar:
  - satirlar: [1, 1]
    metin: "Fonksiyon `bool` döndürüyor: “bu metin yanlış dilde mi”. Kararı veren tarafı basit tutmak için."
  - satirlar: [2, 10]
    metin: "Sezgisel yaklaşım: dile özgü işaretler aranıyor. Kesin bir dil tespiti değil, ucuz ve yeterince iyi bir tahmin."
sina:
  - soru: "Neden tam bir dil tespiti kütüphanesi kullanılmamış?"
    cevap: "Karar ikili ve düşük riskli: yanlış tespit edilirse en fazla gereksiz bir çeviri yapılır. Kütüphane bağımlılığı bu fayda için pahalı kalıyor."
  - soru: "Bu fonksiyon hangi noktada çağrılıyor?"
    cevap: "Rapor üretildikten sonra, kullanıcıya gönderilmeden önce; yanlış dil bulunursa `repair_language` alanları yeniden çeviriyor."
owner: TODO
needs_reverify: true
---

Bu katman hiçbir eski planda geçmiyordu; envanter çıkarılırken bulundu. Kod, dokümantasyonun bilmediği şeyi biliyordu — kod okumanın en somut faydası tam olarak bu.
