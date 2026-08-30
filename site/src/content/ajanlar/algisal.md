---
title: Algısal Ajan
manipulation_type: Algısal
color: "#7209b7"
order: 4
source_fn: "src/agents.rs::analyze_perceptual"
kisaca: "Gerçeğin nasıl çerçevelendiğine bakar: eksik bağlam, seçilmiş karşılaştırma."
teknik_odak:
  - "Payda, zaman aralığı veya karşılaştırma tabanı saklanan istatistikler"
  - "Doğru parçaları seçerek bütün hakkında yanıltıcı izlenim oluşturan çerçeveler"
  - "Ölçek ve görsel oranlarla farkı olduğundan büyük ya da küçük gösterme"
prompt_karari: "Ajan tek tek iddiaların doğruluğundan çok hangi bağlamın dışarıda bırakıldığını sorgular. Böylece dilsel ajanın kelime analizini tekrar etmek yerine sunum biçimini inceler."
yanlis_pozitif: "Kısa bir özet bağlamın tamamını vermese bile temel karşılaştırma, payda ve zaman aralığı açıksa otomatik olarak algısal manipülasyon sayılmaz."
owner: TODO
needs_reverify: true
---

## Kısaca
Doğru bilgilerin bile yanlış izlenim üretecek biçimde çerçevelenip çerçevelenmediğine bakar.

## Bir örnekle
"%50 daha hızlı" cümlesi neyle karşılaştırıldığını söylemezse ölçü değil, algı yönetimidir.
