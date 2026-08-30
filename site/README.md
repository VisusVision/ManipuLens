# ManipuLens Rehber Sitesi

Altı kişilik ekip için yaşayan proje rehberi. Astro 5, statik çıktı, çalışma zamanı yok.

```bash
npm install
npm run dev      # http://localhost:4321/ManipuLens
npm run build    # once sozlugu uretir, sonra dist/ olusturur
npm run sozluk   # sadece docs/sozluk.md uret
npm run pano     # sadece Trello panosunu tazele (anahtar gerekir)
```

## Kurallar

1. **Kaynak doğrusu çalışan koddur.** README tek başına kaynak sayılmaz. Her sayfa
   `verified_at_commit` taşır; taşımayan sayfa `needs_reverify: true` ile işaretlenir.
2. **Her içerik iki katman anlatır.** Sade anlatım varsayılan; teknik katman `ml-technical`
   sınıfı veya `<Layer kind="technical">` ile eklenir ve sade metni tekrar etmez, genişletir.
3. **Sır girmez.** Trello anahtarı, SMTP parolası, token bu klasöre veya çıktıya yazılmaz.
4. **Ajan renkleri kopyadır, karar değil.** Kaynak: `extension/background.js::getProgressBarColor`.
   Orada değişirse `src/styles/tokens.css` ve ajan içerik dosyaları da değişir.

## Yerleşim

| Yol | İş |
| --- | --- |
| `src/content.config.ts` | rehber / ajanlar / guncellemeler koleksiyon şemaları (zod) |
| `src/content/ajanlar/` | altı uzman ajanın sayfası |
| `src/content/guncellemeler/` | güncelleme kayıtları; şema plandaki YAML sözleşmesi |
| `src/components/Layer.astro` | sade/teknik anlatım bloğu |
| `src/components/ModeToggle.astro` | anlatım anahtarı (localStorage `manipulens:mode`) |
| `src/components/EvidenceBadge.astro` | kanıt ve güncellik rozeti |
| `src/content/sozluk/` | sözlük terimleri; `alan` yarıyı, `kume` filtreyi belirler |
| `src/components/Section.astro` | bölüm iskeleti: numara + kicker + başlık + lede |
| `src/components/TermLink.astro` | metinde geçen terimi otomatik sözlüğe bağlar |
| `src/data/pano.json` | **üretilmiş**: Trello panosunun anlık kopyası |
| `scripts/build-sozluk.mjs` | `docs/sozluk.md` üreticisi |
| `scripts/fetch-trello.mjs` | Trello çekimi |
| `src/fonts/` | yerel woff2 (Newsreader, Instrument Sans, JetBrains Mono) |
| `src/styles/tokens.css` | renk, tipografi, ölçek jetonları |

## Güncelleme eklemek

`src/content/guncellemeler/YYYY-AA-GG-kisa-ad.md` dosyası aç. Zorunlu alanlar:
`title`, `date`, `component`, `owner`, `simple_summary`, `technical_summary`.
`status` varsayılan `draft`; bir ekip üyesi doğrulamadan `released` yapılmaz.

## Sözlük eklemek

`src/content/sozluk/<slug>.md` aç. Zorunlu: `terim`, `alan` (`genel` | `manipulens`),
`kume` (`manipulasyon` | `yapay-zeka` | `sistem` | `web`), `kisaca` (≤ 280 karakter).
Manipülasyon kümesindeki her terim `ornek` cümlesi taşımak **zorunda** — şema derlemede
durdurur. Kod karşılığı varsa `kod_capasi` yaz; yalnız teknik modda görünür.
`docs/sozluk.md` elle düzenlenmez, `npm run build` her seferinde yeniden üretir.

## Trello panosu (kurulum)

Pano özel olduğu için çekim bir API anahtarı ister. **Anahtar bu depoya yazılmaz.**

1. https://trello.com/power-ups/admin adresinden bir Power-Up aç, API anahtarını al.
2. Aynı sayfadaki `Token` bağlantısıyla token üret — **okuma yetkisi yeterli**.
3. Yerelde: `cp .env.example .env`, değerleri `.env` içine yaz. `.env` git'e girmez.
4. CI'da: repo `Settings > Secrets and variables > Actions` altına `TRELLO_KEY`,
   `TRELLO_TOKEN`, `TRELLO_BOARD` ekle. Filtreyi değiştirmek için `TRELLO_FILTRE`
   değişkenini (variable) tanımla.
5. `npm run pano` ile dene; `src/data/pano.json` dolar, `/pano` sayfası kartları gösterir.

Görünürlük varsayılanı `beyaz`: **yalnız `site` etiketli kartlar** siteye çıkar. Kart
açıklamasında `<!-- gizli -->` işaretinden sonrası kesilir. Anahtar tanımlı değilse çekim
hata vermez, boş pano yazar ve sayfa "veri yok" durumunu dürüstçe gösterir.

`.github/workflows/trello-sync.yml` her yarım saatte bir tazeler, değiştiyse commit eder.
Elle tazelemek için Actions sekmesinden `Trello panosunu tazele` iş akışını çalıştır.

## Açık işler

- GitHub Pages iş akışı (`astro.config.mjs` içindeki `site`/`base` yayın hedefine göre ayarlanır).
- Trello webhook köprüsü (ayrı Cloudflare Worker; bu repoya sır girmez).
- Destek rotası sayfaları: katkı rehberi, mimari kararlar, gizlilik, sözlük/SSS.
- İçerik sahipleri: tüm `owner` alanları şu an `TODO`.
