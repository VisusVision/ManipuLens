---
terim: "Denetim kaydı"
alan: manipulens
kume: sistem
kisaca: "Sistemde ne zaman hangi olayın yaşandığını satır satır tutan iz defteri. Analiz geçmişi değildir; hata ve güvenlik olaylarını sonradan incelemek içindir."
kod_capasi: "src/audit.rs"
ilgili: ["rate-limit", "endpoint"]
esanlam: ["audit log", "denetim günlüğü"]
owner: TODO
needs_reverify: true
---

ManipuLens günlük JSONL dosyası kullanır ve analiz metninin yalnız ilk 120 karakterini kaydeder. Tam metni yazmamak, kayıt yararlılığı ile veri sızıntısı riskini dengeler.
