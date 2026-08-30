---
title: "Veri ve kayıt"
order: 6
kisaca: "Analizin nerede durduğu, ne saklandığı ve daha önemlisi ne saklanmadığı."
sorumluluk: "SQLite veritabanı ve günlük denetim kaydı. Gizlilik iddiasının en somut kanıtı burada."
dosyalar: "src/db.rs (391 satır) · src/audit.rs (65 satır) · manipulens.db · logs/audit-YYYY-MM-DD.jsonl"
owner: TODO
needs_reverify: true
---

## Bir örnekle

Bir haber metnini analiz ettin. Veritabanına giren şey metnin tamamı değil, **ilk 120
karakteri** ve sonucun özeti. Metnin geri kalanı hiçbir dosyaya yazılmıyor.

## Teknik detay

SQLite, `PRAGMA journal_mode = WAL`. Dosya repo kökünde `manipulens.db`.

```sql
users    (id PK, email UNIQUE, password_hash, created_at, verified DEFAULT 0)
history  (id AUTOINCREMENT, timestamp, client_id, text_preview,
          is_manipulated, dominant_manipulation, genel_sonuc, lang)
sessions (token PK, user_id, email, created_at, expires_at)
```

`history(client_id)` üzerinde `idx_history_client` indeksi var. `lang` sütunu sonradan
`ALTER TABLE` ile eklenmiş; eski kurulumlarda yoksa hata yok sayılıyor.

`migrate_from_json_files()` eski JSON dosyalarını **bir kez** içe aktarır. Tablolar boş
değilse hiçbir şey yapmaz ve kaynak dosyalara dokunmaz.

### Denetim kaydı

`logs/audit-YYYY-MM-DD.jsonl`, günlük dönen, satır başına bir JSON olay. Metnin yalnızca
ilk 120 karakteri (`PREVIEW_CHARS`) ve uzunluğu yazılır.

Eski sistem böyle değildi: `analiz_log.txt` tam metni ve e-postayı düz metin olarak
tutuyordu. Kod yorumunda bu açıkça "gizlilik riski" diye işaretlenip terk edilmiş.

### `client_id` artık istemcinin beyanı değil

Eskiden istemci kendi kimliğini söylüyordu. Şimdi oturumdan geliyor — kod yorumuyla:
kimse başkası adına kayıt oluşturamaz.

## Neden böyle?

**Neden SQLite?** Tek kullanıcılı, yerel, tek dosyalık bir sistem için ayrı bir veritabanı
sunucusu çalıştırmak kurulumu zorlaştırırdı. Kurulum adımı ne kadar azsa proje o kadar
denenir.

**Neden tam metin saklanmıyor?** Saklanan her şey sızabilir. Geçmiş listesini
anlamlandırmak için 120 karakter yetiyor; tamamını tutmak fayda getirmeden risk ekliyordu.

## Bilinen sınır

Veritabanı şifrelenmiyor. Makineye erişimi olan biri `manipulens.db` dosyasını okuyabilir —
önizlemeler ve e-posta adresleri dahil. Yerel çalışan bir araç için kabul edilmiş risk,
ama gizlilik sayfasında yazılı durması gerekiyor.
