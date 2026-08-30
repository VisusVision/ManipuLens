---
title: "Yerel model"
order: 4
kisaca: "Bilgisayarında çalışan dil modeli. Metnin buluta gitmemesinin tek sebebi bu."
sorumluluk: "Altı ajanın da altında duran tek motor. Ajanları ayıran şey model değil, sistem promptu."
dosyalar: "src/agents.rs:37-70 (call_ollama_agent) · sistem promptları agents.rs:72-176"
owner: TODO
needs_reverify: true
---

## Bir örnekle

Altı farklı uzman gibi görünen şey aslında aynı modele sorulan altı farklı soru. "Sen bir
dil uzmanısın, şu cümlede abartı ara" ile "sen bir davranış uzmanısın, aciliyet ara" aynı
motoru farklı yönlere çeviriyor.

## Teknik detay

| Ayar | Değer |
| --- | --- |
| Model | `llama3` |
| Uç nokta | `{OLLAMA_URL}/api/generate` |
| `stream` | `false` |
| `format` | `"json"` |

`format: "json"` katı JSON çıktısı zorlar. Modelin "işte cevabınız:" gibi bir giriş cümlesi
yazıp JSON'u bozması bu ayar sayesinde engelleniyor.

Ajanları birbirinden ayıran tek şey sistem promptudur (`agents.rs:72-176`). Kod tarafında
altı fonksiyon aynı `call_ollama_agent()` yardımcısını çağırır; değişen parametre prompttur.

## Neden böyle?

**Neden yerel model?** Gizlilik iddiasının tamamı buraya dayanıyor. Metin makineden
çıkmıyorsa sızabileceği bir yer de yok. Bedeli açık: kendi işlemcin yorulur ve analiz bulut
modeline göre yavaştır.

**Neden bulut yedeği yok?** Bilinçli. "Ollama kapalıysa buluta düşer" davranışı, gizlilik
iddiasını sessizce çöpe atardı. Kullanıcı hangi durumda metninin nereye gittiğini bilemezdi.

**Neden `llama3`?** Yerelde makul donanımda çalışan, Türkçe çıktısı kabul edilebilir ve
JSON modunu destekleyen bir model. Model adı kodda sabit; değiştirmek `agents.rs` ve
`orchestrator.rs` içinde tek satırlık iş.

## Bilinen sınır

Model yanlış dilde cevap verebiliyor. Bu, orkestratördeki dil onarım katmanının var olma
sebebi. Ayrıca model uydurabiliyor (halüsinasyon); JSON zorunluluğu ve güven puanı bunu
sınırlar, ortadan kaldırmaz. Rapor kanıt değil, işarettir.
