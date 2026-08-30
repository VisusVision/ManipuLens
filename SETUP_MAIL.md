# ManipuLens — E-posta (SMTP) Kurulum Rehberi

Doğrulama ve şifre sıfırlama kodlarının maile gidebilmesi için **gönderici bir e-posta hesabı** gerekir. Sistem bu hesabı SMTP üzerinden kullanır. Hesap tanımlanmazsa mail gönderilemez; kodlar backend konsoluna yazılır (geliştirici modu).

## Geliştirme sırasında maili tamamen kapatmak

Ekip geliştirme yaparken her kayıt/giriş denemesinde 6 haneli kod girmek zaman
kaybı. `.env` dosyasına şunu ekleyip backend'i yeniden başlatmak yeterli:

```
AUTH_MAIL_DISABLED=1
```

Bu modda:

- `/v1/register` kullanıcıyı **doğrulanmış** oluşturur ve oturum token'ını
  doğrudan döner — uzantı kayıt sonrası hemen içeri alır.
- Daha önce doğrulanmamış kalmış hesaplar `/v1/login` ile giriş yapabilir;
  kayıt bir kez doğrulanmış işaretlenir.
- `/v1/verify` ve `/v1/resend` "devre dışı" mesajı döner.
- `/v1/forgot` kod göndermez; `/v1/reset` kod istemeden şifreyi değiştirir.
- `/healthz` yanıtına `"mail_disabled": true` eklenir; uzantı sıfırlama
  ekranındaki kod kutularını buna bakarak gizler.
- Sunucu açılışta uyarı satırı basar.

**Güvenlik uyarısı:** bu mod açıkken `/v1/reset`, e-postayı bilen herkesin o
hesabın şifresini değiştirmesine izin verir. Yalnızca yerel geliştirmede
kullanın; üretimde `AUTH_MAIL_DISABLED` tanımsız veya `0` olmalıdır
(varsayılan davranış zaten kapalıdır). SMTP kodu silinmedi — bayrak
kaldırıldığında eski akış birebir geri döner.

## Gmail ile kurulum (önerilen, ~5 dakika)

1. **2 Adımlı Doğrulamayı aç:** https://myaccount.google.com/security → "2 Adımlı Doğrulama" → etkinleştir. (Uygulama şifresi için zorunlu.)
2. **Uygulama şifresi oluştur:** https://myaccount.google.com/apppasswords → uygulama adı olarak `ManipuLens` yaz → oluştur. Google sana 16 haneli bir şifre verir (örn. `abcd efgh ijkl mnop`).
3. **Proje kök dizininde `.env` dosyası oluştur** (`Cargo.toml` ile aynı klasör) ve doldur:

```
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=seninadresin@gmail.com
SMTP_PASS=abcdefghijklmnop
SMTP_FROM=seninadresin@gmail.com
```

> ÖNEMLİ: `SMTP_PASS` normal Gmail şifren DEĞİL, 2. adımda aldığın uygulama şifresidir. Aradaki **boşlukları silerek** yaz (16 karakter bitişik).

4. **Backend'i yeniden başlat:** `cargo run`

Açılışta şu satırı görmelisin:

```
✓ SMTP bağlantısı doğrulandı: smtp.gmail.com:587 (seninadresin@gmail.com)
```

`✗ UYARI` görüyorsan mesaj sana tam olarak neyin yanlış olduğunu söyler (şifre hatalı, bağlantı yok vb.).

## Diğer sağlayıcılar

| Sağlayıcı | SMTP_HOST | SMTP_PORT |
|---|---|---|
| Outlook / Hotmail | smtp.office365.com | 587 |
| Yandex | smtp.yandex.com | 465 |
| Yahoo | smtp.mail.yahoo.com | 587 |

Port 465 girilirse sistem otomatik olarak implicit TLS kullanır, 587'de STARTTLS kullanır.

## Sorun giderme

- **Mail gelmiyor, konsolda `[DEV MODE]` yazıyor:** `.env` dosyası yok veya eksik → yukarıdaki adımları uygula, backend'i yeniden başlat.
- **`✗ SMTP bağlantı/kimlik testi başarısız`:** Şifre yanlış (uygulama şifresi mi? boşluklar silindi mi?) veya sağlayıcı SMTP erişimini kapatmış.
- **Mail spam klasörüne düşüyor:** Kişisel Gmail hesabından kendine/kullanıcılara giden ilk mailler bazen spam'e düşebilir; "Spam değil" işaretledikten sonra düzelir. Kalıcı çözüm için özel domain + SPF/DKIM kayıtlı bir adres kullanılır.
- **Docker kullanıyorsan:** `.env` dosyası `docker-compose.yml` üzerinden container'a aktarılır (`env_file` satırı hazır).

## Güvenlik notu

`.env` dosyası `.gitignore`'a eklidir; şifren asla git deposuna girmez.
