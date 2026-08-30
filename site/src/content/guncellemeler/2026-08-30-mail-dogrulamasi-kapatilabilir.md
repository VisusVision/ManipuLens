---
title: "Geliştirme sırasında e-posta doğrulaması kapatılabiliyor"
date: 2026-08-30
version: "backend-0.3.0"
component: "backend"
owner: "Enes"
status: "released"
visibility: "team"
simple_summary: "Ekip test ederken her kayıt ve girişte e-postaya gelen 6 haneli kodu beklemek zorunda değil. Bir ayar açıldığında kayıt olur olmaz oturum açılıyor, kod adımı hiç görünmüyor. Ayar kapalıyken her şey eskisi gibi çalışıyor."
ne_ise_yarar: "Bir özelliği denemek için harcanan süreyi kısaltıyor: eskiden her yeni test hesabı için posta kutusu açıp kod kopyalamak gerekiyordu, şimdi kayıt düğmesine basmak yeterli."
technical_summary: "AUTH_MAIL_DISABLED ortam değişkeni eklendi. Açıkken /v1/register oturum token'ını doğrudan döndürür, /v1/verify ve /v1/resend uçları devre dışı yanıt verir, /v1/reset kod istemez. SMTP gönderim kodu silinmedi; bayrak yalnızca çağrı yolunu atlar. /v1/health yanıtı mail_disabled alanını taşır, uzantı doğrulama ekranlarını buna göre gizler — sunucu ile arayüz aynı modda kalır. Varsayılan kapalı; açıkken sunucu başlangıçta uyarı basar."
why_changed: "Altı kişilik ekip geliştirme yaparken her kayıt ve giriş denemesinde e-posta kodu beklemek doğrudan zaman kaybıydı."
impact: "Yerel geliştirmede kayıt akışı tek adıma iniyor. Üretimde bayrak kapalı kalır."
tests: "39/39 test yeşil; bayrak açık ve kapalı iki durumda kayıt, giriş ve şifre sıfırlama akışı elle koşuldu."
karsilastirma:
  - alan: "Kayıt akışı"
    once: "Kayıt → e-posta bekle → 6 haneli kodu gir → oturum aç"
    sonra: "Kayıt → oturum açık (bayrak açıkken)"
  - alan: "Şifre sıfırlama"
    once: "E-posta ile gönderilen kod zorunlu"
    sonra: "Bayrak açıkken kod istenmiyor, doğrudan yeni şifre"
  - alan: "SMTP kodu"
    once: "Tek yol, atlanamıyor"
    sonra: "Yerinde duruyor, bayrak yalnızca çağrıyı atlıyor"
  - alan: "Arayüz eşlemesi"
    once: "Uzantı sunucunun mail modunu bilmiyor"
    sonra: "/v1/health üzerinden okuyup doğrulama ekranlarını gizliyor"
known_issues:
  - "Bayrak açıkken /v1/reset, e-postayı bilen herkesin şifreyi değiştirmesine izin verir. Yalnızca yerel geliştirme içindir; üretime bu bayrakla çıkılmaz."
commit_or_pr: "https://github.com/VisusVision/ManipuLens/commit/25e65af"
verified_at_commit: "25e65af"
verified_at: 2026-08-30
---
