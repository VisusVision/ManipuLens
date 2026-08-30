---
title: "Orkestratör"
order: 3
kisaca: "Altı uzmanı aynı anda çalıştıran, raporlarını toplayıp şefe veren yönetici."
sorumluluk: "İşi kendisi yapmaz. Paralellik, hata yalıtımı ve yedek yol onun sorumluluğunda."
dosyalar: "src/orchestrator.rs (441 satır) · src/agents.rs (179 satır)"
owner: TODO
needs_reverify: true
---

## Bir örnekle

Altı uzmana aynı anda telefon açan bir yönetici düşün. Sırayla arasaydı altı kat beklerdi.
Biri telefonu açmazsa toplantı iptal olmuyor; o uzman "görüş bildirmedi" olarak geçiyor.

## Teknik detay

`run_orchestrator` altı ajanı `tokio::join!` ile **aynı anda** başlatır. Toplam süre
altısının toplamı değil, en yavaşının süresidir.

Her ajanın sonucu `unwrap_or_else(|_| fallback(<ad>))` ile sarılır: bir ajan çökerse analiz
çökmez, o ajan boş rapor döndürür ve kalan beşi devam eder.

| Ajan | Fonksiyon | `manipulation_type` |
| --- | --- | --- |
| Dilsel | `analyze_linguistic` | `Dilsel` |
| Psikolojik | `analyze_psychological` | `Psikolojik` |
| Davranışsal | `analyze_behavioral` | `Davranışsal` |
| Algısal | `analyze_perceptual` | `Algısal` |
| Sosyal | `analyze_social` | `Sosyal` |
| Pazarlama | `analyze_marketing` | `Pazarlama` |

### Sentezör ve yedek yol

Yedinci çağrı şefe gider: altı uzman raporunu tek kalibre edilmiş karara indirir, çıktı
`FinalReport`. Şefe ulaşılamazsa `fallback_summary()` devreye girer ve altı rapordan en
yüksek güven puanlısını seçer. Bu yol birim testlidir:
`fallback_summary_picks_highest_confidence`.

### Dil onarımı — planların hiçbirinde olmayan katman

Orkestratörde üç fonksiyon daha var ve hiçbir eski planda geçmiyor:

- `wrong_language()` — modelin yanlış dilde cevap verdiğini sezgisel olarak tespit eder.
- `repair_language()` — yanlış dildeki alanları yeniden çevirir.
- `strip_quoted()` — alıntı işaretlerini temizler.
- `translate_report()` — `/v1/translate-report` ucunun motoru.

Dördü de birim testli. Bu katman şunu kabul ediyor: model bazen istenen dilde cevap
vermez, ve bunu düzeltmek modelin değil kodun işidir.

## Neden böyle?

**Neden altı ayrı ajan, tek büyük prompt yerine?** Tek prompt altı şeyi aynı anda sorunca
model hepsini yüzeysel yapıyor. Ayrı ajanların her biri tek soruya odaklanıyor ve çıktı
şeması dar kalıyor, bu da JSON'un bozulma ihtimalini düşürüyor.

**Neden paralel?** Ajanlar birbirinin çıktısına muhtaç değil. Sentezör ise **bilerek
sıralı** — girdisi zaten altı ajanın çıktısı.

## Bilinen sınır

Yedek yolun ürettiği rapor tam rapordan zayıftır, ama arayüz şu an bu ikisini birbirinden
ayırmıyor. Kullanıcı "bu sonuç eksik yoldan geldi" bilgisini görmüyor.
