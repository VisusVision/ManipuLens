---
title: Psikolojik Ajan
manipulation_type: Psikolojik
color: "#f72585"
order: 2
source_fn: "src/agents.rs::analyze_psychological"
kisaca: "Duygu üzerinden baskı kurulup kurulmadığına bakar: korku, suçluluk, aciliyet."
teknik_odak:
  - "Bir seçeneği değerlendirmek yerine korku veya pişmanlık üreten tehdit dili"
  - "Okuyucuya borçluluk ya da suçluluk yükleyen duygusal baskı"
  - "Kanıt sunmadan kaygı ve aciliyet oluşturan sonuç tahminleri"
prompt_karari: "Bu ajan metnin duygulu olmasını değil, duygunun karar verme kapasitesini daraltmak için araç olarak kullanılmasını arar. Dar tanım, her dramatik cümleyi manipülasyon saymasını engeller."
yanlis_pozitif: "Gerçek bir tehlikeyi ölçülebilir kanıtla anlatmak veya kişinin duygusunu açıkça ifade etmesi tek başına psikolojik manipülasyon değildir."
owner: TODO
needs_reverify: true
---

## Kısaca
Metnin okuyucunun duygusunu hedef alıp almadığını arar.

## Bir örnekle
"Bunu kaçırırsan sonra pişman olursun" cümlesi bilgi vermez, korku üretir.
