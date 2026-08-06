---
name: otomasyon-onerisi
description: Proje icin eksik otomasyonlari (CI, pre-commit hook, dependency bot, release/build workflow, tekrar eden elle islemler) tespit edip onem sirasina gore onerir. "recommend automations for this project", "bu proje icin otomasyon oner", "otomasyon eksikligi var mi" gibi isteklerde tetiklenir - kullanici "skill" kelimesini soylemese bile.
---

# Otomasyon Onerisi

Amac: genel gecer otomasyon listesi degil, BU repoda gercekten eksik ve BU repoda kanitlanmis (tekrar eden elle islem, dokumante edilmis ama denetlenmeyen kural, savunmasiz kod yolu) otomasyonlari onermek.

## Adim 1: Repo tara

Paralel bak:
- `.github/workflows/` var mi, ne kapsiyor (build/test/lint hangileri eksik)
- `.githooks/` + `git config --get core.hooksPath` — hook kurulu mu, aktif mi
- `.github/dependabot.yml` / renovate config var mi
- `scripts/` klasoru — hangi elle-tekrarlanan islemler script'e donusturulmemis
- Dockerfile / docker-compose.yml — build/publish otomasyonu var mi
- CLAUDE.md (proje ve global) — zorunlu tutulan ama denetlenmeyen kurallar (orn. "shellcheck kullan", "sg ile refactor yap") — bunlar CI'da enforce ediliyor mu
- `git log --oneline -30` — ayni islemin tekrar tekrar elle commitlendigine dair pattern (orn. "Update X.json" gibi tekrarlayan commit basliklari)

## Adim 2: Kontrol listesiyle kiyasla

- CI (build + test + lint)
- Pre-commit / pre-push hook
- Dependency guncelleme botu (Dependabot/Renovate)
- Release/packaging otomasyonu (tag push -> build + paketleme)
- Container build/smoke-test otomasyonu
- Git log'da tekrar eden elle islem (script'e donusturulebilir)
- CLAUDE.md'de zorunlu ama denetlenmeyen kurallar (shellcheck, format, vb.)

Sadece GERCEKTEN eksik olanlari listele — var olani tekrar onerme.

## Adim 3: Onem sirasina diz

Siralama kistasi: (boyle bir hata/tekrar gecmiste kac kez oldu) x (yakalanmadiginda ne kadar riskli/kaybettirici). Once "kirilirsa hemen fark edilir" seyler (CI, hook), sonra bakim/verimlilik (dependency bot, script), en sona nice-to-have (release paketleme, stil denetimi).

## Adim 4: Sun

Turkce, kisa, her madde: **isim** — bu repoda bulunan somut kanit (dosya/commit/kural adiyla). Genel "iyi pratik" cumlesi yazma, kanita bagla.

Sonda sor: hangisini/hangilerini kursun. Onaylanmadan hicbir dosya/config degistirme.

## Uygularken

- Yeni CI/hook eklerken mevcut kod o kontrolden (clippy/fmt/test) GECIYOR MU once dogrula (`cargo clippy`, `cargo fmt --check` calistir). Gecmiyorsa: ya once kirilan seyi duzelt (kullaniciya sorup) ya da o adimi disari birak — ilk gunden kirmizi CI kurmanin anlami yok.
- `git config` degisikligi (orn. `core.hooksPath`) dahil git config'e dokunan hicbir komutu kullanici acikca soylemeden calistirma — sadece komutu ver, kullanici calistirsin ya da acikca "calistir" desin.
- Push/otomatik commit iceren script'lerde push adimini varsayilan kapali birak (`-Commit` gibi flag'le ac, push'u yine de elle birak).
