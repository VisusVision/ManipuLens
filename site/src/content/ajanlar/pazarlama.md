---
title: Pazarlama Ajanı
manipulation_type: Pazarlama
color: "#2a9d8f"
order: 6
source_fn: "src/agents.rs::analyze_marketing"
kisaca: "Metnin arkasında satılan bir ürün/hizmet olup olmadığını çıkarır."
teknik_odak:
  - "Bilgilendirici içerik görünümü altında gizlenen ürün veya hizmet yönlendirmesi"
  - "Tek markayı doğal sonuç gibi öne çıkaran karşılaştırma ve inceleme dili"
  - "Ticari çıkarı açıklamadan satın alma niyeti üreten fayda iddiaları"
prompt_karari: "Bu ajan diğerlerinden farklı olarak yalnız manipülasyon türünü değil, metnin hangi ürün veya hizmete yönelttiğini de tahmin eder. Sonuç sentezörde predicted_product alanına dönüşebilir."
yanlis_pozitif: "Sponsorluğu açıkça belirtilmiş reklam veya özellikleri ölçülebilir biçimde karşılaştıran satış metni, yalnız ticari olduğu için manipülasyon sayılmaz."
owner: TODO
needs_reverify: true
---

## Kısaca
Metnin ticari amaç taşıyıp taşımadığını ve neyin satıldığını tahmin eder.

## Bir örnekle
Tarafsız görünen bir "inceleme" yazısı tek bir markaya yönlendiriyorsa bu ajan işaretler.
