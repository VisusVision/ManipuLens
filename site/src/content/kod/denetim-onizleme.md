---
title: "Denetim kaydı: neden 120 karakter"
order: 10
parca: veri
zorluk: baslangic
dosya: "src/audit.rs"
aralik: [24, 30]
dil: rust
ne_yapiyor: "Kayda geçecek metni ilk 120 karaktere kırpar."
neden_boyle: "Saklanan her şey sızabilir. Geçmişte bir kaydı tanımak için 120 karakter yeterli; metnin tamamını tutmak fayda getirmeden risk ekliyordu. Eski sistem tam metni yazıyordu ve kod yorumunda bu açıkça gizlilik riski diye işaretlenmiş."
kaldirirsak: "Log dosyaları kullanıcının analiz ettiği her metnin tam kopyasını içerir. Gizlilik iddiasının tamamı çöker."
notlar:
  - satirlar: [1, 1]
    metin: "Sabit tek yerde tanımlı. Sitedeki “120 karakter” cümlesi bu satırdan geliyor."
  - satirlar: [3, 4]
    metin: "`chars().take(...)` bayt değil **karakter** sayıyor. Türkçe harflerde bayt bazlı kırpma harfin ortasından bölerdi."
sina:
  - soru: "Neden `chars()` kullanılmış, dizi dilimi değil?"
    cevap: "UTF-8'de bir karakter birden çok bayt olabilir; bayt bazlı kırpma çok baytlı harfleri ortadan bölüp geçersiz metin üretir."
  - soru: "Bu üç satır kaldırılsa hangi iddia çürür?"
    cevap: "“Metnin tamamı hiçbir yere yazılmıyor” iddiası; gizlilik sayfasının en somut kanıtı bu fonksiyon."
owner: TODO
needs_reverify: true
---

Üç satır, bir güvenlik kararı. Bu parça kod okumanın en net dersini veriyor: bir sistemin en önemli satırları çoğu zaman en kısa olanlar.
