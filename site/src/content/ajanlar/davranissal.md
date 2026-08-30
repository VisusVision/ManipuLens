---
title: Davranışsal Ajan
manipulation_type: Davranışsal
color: "#f8961e"
order: 3
source_fn: "src/agents.rs::analyze_behavioral"
kisaca: "Okuyucuyu belirli bir eyleme itmeye çalışan kalıpları arar."
teknik_odak:
  - "Sahte kıtlık ve geri sayımla düşünme süresini bilinçli olarak kısaltma"
  - "Varsayılan seçenek, sürtünme veya tekrar yoluyla belirli eyleme yöneltme"
  - "Ödül ve kayıp çerçevesiyle kararın mimarisini değiştirme"
prompt_karari: "Prompt niyetten çok davranış mekanizmasını sorar: metin okuyucuya bilgi mi veriyor, yoksa seçim ortamını belirli bir tıklamayı daha olası kılacak şekilde mi kuruyor?"
yanlis_pozitif: "Gerçek stok sayısı, gerçek son tarih veya açık bir eylem çağrısı doğrulanabilir ve baskısızsa davranışsal manipülasyon değildir."
owner: TODO
needs_reverify: true
---

## Kısaca
Metnin bir davranışı tetiklemek için kurgulanıp kurgulanmadığına bakar.

## Bir örnekle
"Son 3 ürün kaldı, hemen tıkla" cümlesi kıtlık kurgusuyla hızlı karar dayatır.
