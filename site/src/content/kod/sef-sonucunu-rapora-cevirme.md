---
title: "Şef sonucunu nihai rapora çevirme"
order: 13
parca: orkestrator
zorluk: orta
dosya: "src/orchestrator.rs"
aralik: [127, 154]
dil: rust
ne_yapiyor: "Şef modelinin dar JSON cevabını okur, başarısızlıkta yedek özeti seçer ve sonucu altı uzman raporuyla birlikte FinalReport yapısına yerleştirir."
neden_boyle: "Şef yalnız üç alanı kararlaştırır; ayrıntılı uzman raporları modelin yeniden yazmasına bırakılmaz. Küçük ara yapı ayrıştırmayı dar tutar, `Option` ise ağ, durum ve JSON hatalarının hepsini aynı yedek yola bağlar."
kaldirirsak: "Şefin tek bozuk cevabı tüm analizi düşürür veya ayrıntılı raporlar kaybolur. Kullanıcı altı başarılı uzmanın ürettiği kanıtı göremez."
notlar:
  - satirlar: [1, 6]
    metin: "`ManagerOutput` yalnız şefin üretmesi gereken üç alanı içeriyor. Altı uzmanın ayrıntıları bu modele tekrar yazdırılmıyor."
  - satirlar: [8, 13]
    metin: "JSON ayrıştırılamazsa `.ok()?` akışı sessizce `None` yapıyor. Buradaki sessizlik kayıp değil; hemen aşağıdaki yedek yol tarafından bilinçli olarak ele alınıyor."
  - satirlar: [17, 20]
    metin: "`match` başarıyı olduğu gibi alıyor, her tür başarısızlığı `fallback_summary` ile yerelde tamamlıyor. Yeni bir model çağrısı yapılmıyor."
  - satirlar: [22, 28]
    metin: "Nihai rapor şefin üç alanına ek olarak ürün tahmini ve altı özgün uzman analizini taşıyor. Özet ile kanıt aynı veri yapısında ama ayrı alanlarda kalıyor."
sina:
  - soru: "Şef neden doğrudan tam `FinalReport` üretmiyor?"
    cevap: "Uzman raporlarını zaten kod güvenilir biçimde elinde tutuyor; modele yeniden yazdırmak bilgi kaybı ve uydurma riski eklerdi."
  - soru: "`serde_json::from_str` başarısız olduğunda kullanıcı neden boş rapor görmüyor?"
    cevap: "Hata `None` olur ve hemen ardından `fallback_summary` mevcut uzman raporlarından yerel bir sonuç kurar."
owner: TODO
needs_reverify: true
---

Bu kesit orkestrasyonun yalnız çağrı sırası olmadığını gösteriyor. Asıl sorumluluk, olasılıksal model çıktısını kayıp üretmeden kesin bir uygulama sözleşmesine çevirmek.
