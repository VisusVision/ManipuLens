---
title: Dilsel Ajan
manipulation_type: Dilsel
color: "#4cc9f0"
order: 1
source_fn: "src/agents.rs::analyze_linguistic"
kisaca: "Cümlenin kendisine bakar: kelime seçimi, abartı, muğlaklık, yönlendiren dil."
teknik_odak:
  - "Sorumluluktan kaçan muğlak kaynaklar: ‘uzmanlar söylüyor’ gibi gelincik sözcükler"
  - "Yargıyı gerçek gibi taşıyan yüklü kelimeler ve örtmeceler"
  - "Bir anahtar kelimenin anlamını cümle içinde kaydıran veya varsayımı gizleyen yapılar"
prompt_karari: "Prompt yalnız kelime seçiminin inancı değiştirip değiştirmediğini test eder. Genel ikna, güçlü görüş ve haber üslubu ayrı bırakılır; aksi hâlde ajan diğer beş uzmanla aynı şeyleri işaretlerdi."
yanlis_pozitif: "Tarafsız kelimelerle yeniden yazıldığında okuyucunun inancı değişmiyorsa metin güçlü veya abartılı olsa bile manipülasyon sayılmaz."
owner: TODO
needs_reverify: true
---

## Kısaca
Metnin dilini inceler. Hangi kelimeler seçilmiş, hangi iddia ölçülemez biçimde bırakılmış,
hangi cümle abartıyla şişirilmiş.

## Bir örnekle
"Piyasadaki **tek** gerçek çözüm" cümlesinde ölçülebilir bir bilgi yok; abartı var.
