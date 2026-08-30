---
title: "Rust sunucusu"
order: 2
kisaca: "Uzantı ile model arasındaki aracı katman: kimlik doğrular, limit uygular, sonucu saklar."
sorumluluk: "Kapı ve kayıt memuru. Analizi kendisi yapmaz, orkestratöre devreder."
dosyalar: "src/main.rs (1227 satır) · src/auth.rs (162 satır) · src/types.rs (122 satır)"
owner: TODO
needs_reverify: true
---

## Bir örnekle

Uzantı isteği gönderir. Sunucu önce "sen kimsin" der, sonra "bu dakika içinde kaç kez
sordun" der, sonra "metin ne kadar uzun" der. Üçü de geçilirse model çalışmaya başlar.
Geçilmezse model **hiç** çalışmaz — boşa dönen yedi model çağrısı olmaz.

## Teknik detay

Axum + Tokio. Router `main.rs:1007-1018` içinde, on uç nokta tanımlı:

| Yöntem | Yol | Kimlik | Ne yapar |
| --- | --- | --- | --- |
| POST | `/v1/register` | açık | Kayıt, doğrulama kodu gönderir |
| POST | `/v1/login` | açık | Giriş, oturum token'ı döner |
| POST | `/v1/verify` | açık | Doğrulama kodunu kontrol eder |
| POST | `/v1/resend` | açık | Kodu yeniden gönderir |
| POST | `/v1/forgot` | açık | Parola sıfırlama kodu |
| POST | `/v1/reset` | açık | Kod ile yeni parola |
| POST | `/v1/analyze` | **Bearer** | Ana analiz |
| POST | `/v1/translate-report` | **Bearer** | Raporu TR↔EN çevirir |
| GET | `/v1/history` | **Bearer** | Geçmiş analizler |
| GET | `/healthz` | açık | `{"status":"ok"}` |

`/v1/analyze` sırası sabittir: dil normalize (`norm_lang`), oturum kontrolü, hız limiti,
boş metin kontrolü, uzunluk kontrolü (en fazla 1000 karakter), orkestratör, geçmişe yazma.

### CORS

`AllowOrigin::mirror_request()` kullanılıyor — `Any` değil. İzinli yöntemler
GET/POST/OPTIONS; başlıklar `Content-Type`, `Authorization`, `Accept`,
`ngrok-skip-browser-warning`; `max_age` 24 saat.

Nüans önemli: `mirror_request()` pratikte gelen her origin'i yansıtır, yani kapı `Any`
kadar geniştir. Asıl korumayı CORS değil, `/v1/analyze` üzerindeki Bearer zorunluluğu
sağlıyor.

## Neden böyle?

**Neden araya bir sunucu?** Uzantı doğrudan Ollama'ya da konuşabilirdi. O zaman hesap,
limit, geçmiş ve denetim kaydı diye bir şey olmazdı; her sekme kendi başına yedi model
çağrısı tetikleyebilirdi.

**Neden kapı sonradan eklendi?** Kod yorumu açıkça söylüyor: bu uç eskiden tamamen açıktı,
adresi bilen herkes sınırsız analiz tetikleyebiliyordu. Her analiz yedi model çağrısı
demek — kapısız bırakmak makineyi başkasına açmaktı.

## Bilinen sınır

`ngrok-skip-browser-warning` başlığı hem CORS ayarında hem istemcide duruyor ama ngrok
kullanılmıyor. Hazırlanmış ama kapalı bir ayar; kararlar sayfasında böyle geçiyor.
