---
terim: "Service worker"
alan: genel
kume: web
kisaca: "Uzantının işi gelince uyanan, bitince uyuyan arka plan görevlisi. Sürekli açık bir pencere değildir; bu yüzden durumunu ayrıca saklaması gerekir."
kod_capasi: "extension/background.js"
ilgili: ["manifest-v3", "dom", "endpoint"]
esanlam: ["arka plan görevlisi", "background service worker"]
owner: TODO
needs_reverify: true
---

Sağ tık olayını dinler, yerel sunucuya isteği yollar ve boyama kodunu aktif sekmeye enjekte eder. Oturum gibi kalıcı bilgiler `chrome.storage.local` içinde tutulur.
