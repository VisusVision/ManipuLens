---
title: Sosyal Ajan
manipulation_type: Sosyal
color: "#4361ee"
order: 5
source_fn: "src/agents.rs::analyze_social"
kisaca: "Grup baskısı, otorite ve aidiyet üzerinden kurulan ikna kalıplarını arar."
teknik_odak:
  - "Kaynağı belirsiz uzman veya otorite adına kurulan doğruluk iddiaları"
  - "Çoğunluğa uyma baskısı oluşturan sosyal kanıt ve popülerlik sayıları"
  - "Gruba ait olma veya dışlanma korkusunu karara bağlayan ifadeler"
prompt_karari: "Prompt bir görüşün popüler olmasını değil, popülerlik veya otoritenin kanıt yerine kullanılmasını arar. Bu ayrım yapılmazsa her alıntı ve kullanıcı sayısı şüpheli görünürdü."
yanlis_pozitif: "Kaynağı açık uzman görüşü, yöntemi belirtilmiş anket veya doğrulanabilir kullanıcı sayısı tek başına sosyal manipülasyon değildir."
owner: TODO
needs_reverify: true
---

## Kısaca
"Herkes yapıyor", "uzmanlar öneriyor" türü sosyal kanıt baskısını arar.

## Bir örnekle
"Milyonlarca kişi çoktan geçti" cümlesi ürün hakkında hiçbir şey söylemez.
