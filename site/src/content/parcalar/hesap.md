---
title: "Hesap ve oturum"
order: 5
kisaca: "Neden giriş yapman gerektiği ve sistemin kötüye kullanıma karşı koyduğu sınırlar."
sorumluluk: "Kimlik, oturum ömrü ve bütün hız limitleri burada tanımlı."
dosyalar: "src/auth.rs (162 satır) · src/main.rs kayıt/giriş/doğrulama uçları"
owner: TODO
needs_reverify: true
---

## Bir örnekle

Parolanı yanlış girdin. Sekizinci denemeden sonra hesap on beş dakika kilitleniyor.
Kilitleme parolayı bilmeyeni yavaşlatmak için var; deneme sayısını sonsuz bırakmak
parolayı zamanla tahmin edilebilir yapardı.

## Teknik detay

Bütün sabitler kodda tanımlı ve sitede birebir yazılıyor:

| Sabit | Değer | Anlamı |
| --- | --- | --- |
| `SESSION_TTL_SECS` | 30 gün | Oturum ömrü |
| `ANALYZE_WINDOW_SECS` / `ANALYZE_MAX_PER_WINDOW` | 60 sn / 10 | Dakikada 10 analiz |
| `LOGIN_WINDOW_SECS` | 15 dk | Giriş deneme penceresi |
| `LOGIN_MAX_FAILS` | 8 | Kilitlemeden önceki hatalı giriş |
| `LOGIN_LOCK_SECS` | 15 dk | Kilit süresi |
| `CODE_TTL_SECS` | 600 sn | Doğrulama kodu ömrü |
| `RESEND_COOLDOWN_SECS` | 30 sn | Kod yeniden gönderme beklemesi |
| `MAX_CODE_ATTEMPTS` | 5 | Kod başına yanlış deneme; aşılırsa kod iptal |
| `HISTORY_LIMIT` | 200 | `/v1/history` en fazla kayıt |
| `MAX_TEXT_LEN` | 1000 karakter | Analiz edilebilir metin |

Parola bcrypt ile saklanıyor. Zamanlama saldırısına karşı bir `DUMMY_HASH` sabiti var:
kullanıcı bulunamasa bile hash doğrulaması çalıştırılıyor, böylece "bu e-posta kayıtlı mı"
sorusu cevap süresinden okunamıyor.

## Neden böyle?

**Neden hesap zorunlu?** Analiz ucu eskiden açıktı. Her analiz yedi model çağrısı demek;
kimlik olmadan limit uygulanamaz, limit olmadan makine başkasının hesap makinesine döner.

**Neden 30 günlük oturum?** Uzantı her gün giriş isteseydi kimse kullanmazdı. 30 gün,
kullanılabilirlik ile risk arasında verilmiş bir karar — token çalınırsa geçerlilik süresi
de o kadar uzun demektir.

## Bilinen sınır

Oturum token'ı `chrome.storage.local` içinde duruyor. Bu, makineye erişimi olan birinin
token'ı okuyabileceği anlamına gelir. Yerel çalışan bir sistem için kabul edilmiş bir risk.
