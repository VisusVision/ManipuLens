---
title: "Eski veriyi bir kez SQLite'a taşıma"
order: 16
parca: veri
zorluk: orta
dosya: "src/db.rs"
aralik: [84, 123]
dil: rust
ne_yapiyor: "Eski users.json ve history.jsonl dosyalarını yalnız hedef tablolar boşsa SQLite'a aktarır, kaynak dosyaları silmeden bırakır."
neden_boyle: "Geçiş her açılışta çağrılsa bile tekrar kayıt üretmemelidir. Boş tablo kapısı işlemi fiilen tek seferlik yapar; kaynakları korumak da aktarım yanlışsa geri dönme ve karşılaştırma imkânı verir."
kaldirirsak: "Eski kurulumdaki hesaplar ve analiz geçmişi yeni sürümde görünmez. Koşulsuz içe aktarım yazılırsa her açılışta yinelenen geçmiş kayıtları oluşabilir."
notlar:
  - satirlar: [1, 8]
    metin: "Önce hedef kullanıcı sayısı okunuyor. Sorgu hatasında sıfır varsayılması uygulamayı açar ama bozuk veritabanında içe aktarma denemesine yol açabilecek bilinçli bir toleranstır."
  - satirlar: [9, 20]
    metin: "Dosyanın varlığı, JSON'un geçerliliği ve her kullanıcının eklenmesi ayrı ayrı deneniyor. Tek bozuk kayıt tüm açılışı panikle düşürmüyor."
  - satirlar: [18, 18]
    metin: "Log açıkça dosyanın korunduğunu söylüyor. Bu ayrıntı operatöre geçişin kopyalama olduğunu, taşıma veya silme olmadığını bildirir."
  - satirlar: [23, 39]
    metin: "Geçmiş için aynı boşluk kapısı ayrı uygulanıyor. Kullanıcı tablosu dolu, geçmiş boş olabilir; iki veri türü birbirini gereksiz yere bloke etmiyor."
sina:
  - soru: "Kaynak JSON dosyaları neden başarılı aktarım sonrası silinmiyor?"
    cevap: "Geri dönüş ve veri karşılaştırması için. Otomatik silme, aktarımın eksik olduğu ancak geç fark edildiği durumda tek eski kopyayı yok ederdi."
  - soru: "Bu göç tamamen idempotent mi?"
    cevap: "Normal tamamlanmış durumda evet, çünkü dolu tablo tekrar aktarımı engeller. Kısmi aktarımda tablo artık boş olmadığı için kalan kayıtlar otomatik tamamlanmaz; bilinen sadelik bedeli budur."
owner: TODO
needs_reverify: true
---

Bu parça küçük ama gerçek bir veri göçü örneği: hedefi kontrol et, kaynağı toleranslı oku, eskisini koru ve ne yaptığını logla. Güvenli geçiş çoğu zaman silmemekle başlar.
