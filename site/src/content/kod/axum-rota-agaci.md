---
title: "Axum rota ağacını kurma"
order: 12
parca: sunucu
zorluk: baslangic
dosya: "src/main.rs"
aralik: [1007, 1024]
dil: rust
ne_yapiyor: "On HTTP yolunu işleyicilerine bağlar, ortak CORS ve uygulama durumunu ekler, sonra sunucunun yalnız yerel adreste dinleyeceğini belirler."
neden_boyle: "Rotalar tek zincirde toplandığında dışarı açılan bütün yüzey bir bakışta denetlenebilir. Ortak katman ve durum her işleyicide yeniden kurulmaz; aynı güvenlik ve veri nesneleri paylaşılır."
kaldirirsak: "İlgili `.route` satırı silinen özellik artık 404 döner. CORS veya state katmanı kaldırılırsa uçlar derlense bile tarayıcı istekleri ya da veritabanı erişimi çalışmaz."
notlar:
  - satirlar: [1, 11]
    metin: "Her satır yöntem + yol + işleyici eşleşmesidir. Yalnız geçmiş `GET`, veri değiştiren veya işlem başlatan uçların tamamı `POST` kullanıyor."
  - satirlar: [12, 13]
    metin: "CORS katmanı ve `AppState` bütün rotalara bir kez ekleniyor. Bu iki satır zincirin sonunda olsa da üzerindeki bütün rotaları kapsıyor."
  - satirlar: [15, 15]
    metin: "`127.0.0.1:3000` sunucuyu yalnız aynı makineden erişilebilir kılıyor. `0.0.0.0` olsaydı yerel ağ arayüzlerinden de dinlerdi."
  - satirlar: [16, 18]
    metin: "Açılış logu hangi uçların Bearer istediğini görünür kılıyor. Bu log koruma uygulamaz; gerçek kontrol işleyicilerin içindedir."
sina:
  - soru: "Bir rotanın tabloda görünmesi onun korumalı olduğunu kanıtlar mı?"
    cevap: "Hayır. Router yalnız yolu işleyiciye bağlar; Bearer kontrolünün işleyici içinde gerçekten çağrıldığını ayrıca okumak gerekir."
  - soru: "Adres neden `0.0.0.0` değil?"
    cevap: "Araç yerel kullanım için tasarlandığı için; loopback adresi başka makinelerin sunucuya doğrudan bağlanmasını engeller."
owner: TODO
needs_reverify: true
---

Sunucunun dış yüzeyi on sekiz satırda görülebiliyor. Bir API'yi anlamaya başlarken önce iş mantığına değil, hangi kapıların gerçekten açık olduğuna bakmak çoğu zaman daha hızlıdır.
