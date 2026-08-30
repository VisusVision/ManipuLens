---
title: "Rehber sitesinin temeli kuruldu"
date: 2026-08-29
version: "site-0.1.0"
component: "site"
owner: "Enes"
status: "draft"
visibility: "team"
simple_summary: "Projeyi anlatan rehber sitenin ilk iskeleti hazır: ana sayfa, nasıl çalışır, parçalar, ajan atlası, kurulum ve güncellemeler bölümleri açıldı. Sayfaların üstündeki anahtarla anlatımı sade ya da teknik seçebiliyorsunuz."
technical_summary: "Astro 5 statik site, site/ altında. Üç content collection (rehber, ajanlar, guncellemeler) zod şemalarıyla tanımlandı; güncelleme şeması plandaki YAML sözleşmesini birebir karşılıyor. Sade/teknik anahtarı <html data-ml-mode> + localStorage ile çalışıyor, CSS görünürlük kuralı olduğu için sayfa yeniden yüklenmiyor. Tasarım jetonları ve altı ajan rengi extension/background.js::getProgressBarColor haritasından alındı."
why_changed: "Altı kişilik ekibin aynı doğrulukta bilgiye ulaşması ve değişiklikleri iki anlatım katmanıyla takip edebilmesi için."
impact: "Henüz yayın hattı yok; site yerelde npm run dev / npm run build ile çalışıyor."
tests: "npm run build (astro check dahil) temiz geçti."
known_issues:
  - "İçerik sahipleri (owner) TODO olarak duruyor; ekip dağılımı yapılmadı."
  - "Hiçbir sayfa henüz verified_at_commit taşımıyor, tümü 'yeniden doğrulama gerekli' işaretli."
  - "Trello köprüsü ve GitHub Pages yayını kurulmadı."
---
