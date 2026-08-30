---
title: "Sunucu kimliği doğrular"
order: 3
kicker: "Kapıda kontrol"
sade: "Hesabın var mı, hakkın kaldı mı, metin makul uzunlukta mı? Üç kontrol geçilmeden model hiç çalışmaz."
kod: "src/main.rs:775-880, src/auth.rs:25-26"
sure: "1 milisaniyenin altında"
hata: "Oturum yoksa 401, limit aşıldıysa 429 (kalan saniye ile), boş veya 1000 karakterden uzun metinde 400."
owner: TODO
needs_reverify: true
---

Sıra sabittir: dil normalize (`norm_lang`, varsayılan `tr`), oturum kontrolü, hız limiti (kullanıcı başına 60 saniyede 10 analiz), boş metin kontrolü, uzunluk kontrolü (en fazla 1000 karakter). Bu kapı sonradan eklendi — kod yorumu eskiden ucun tamamen açık olduğunu ve URL'i bilen herkesin sınırsız analiz tetikleyebildiğini söylüyor. Her analiz 7 model çağrısı demek; kapısız bırakmak makineyi başkasına açmak olurdu.
