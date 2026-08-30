---
title: "Chrome uzantısı"
order: 1
kisaca: "Sağ tık menüsünü ekleyen, seçtiğin metni sunucuya gönderen ve sonucu sayfaya boyayan parça."
sorumluluk: "Kullanıcıyla temas eden tek yüz. Analiz yapmaz, karar vermez; taşır ve gösterir."
dosyalar: "extension/manifest.json · extension/background.js (442 satır) · extension/popup.js (1454 satır)"
owner: TODO
needs_reverify: true
---

## Bir örnekle

Bir haber sitesinde "Uzmanlar bu fırsatın son gün olduğunu söylüyor" cümlesini seçip sağ
tıklarsın. Birkaç saniye sonra o cümle sayfanın kendi üzerinde renklenir ve üzerine gelince
hangi ajanın yakaladığı yazar. Sekme değiştirmedin, hiçbir yere kopyalamadın.

## Teknik detay

Manifest V3 uzantısı. İzinler: `contextMenus`, `scripting`, `activeTab`, `storage`, `windows`
ve `host_permissions: ["http://*/*", "https://*/*"]`. Arka planda kalıcı bir sayfa yok;
service worker (`background.js`) iş geldiğinde uyanır, bitince uyur.

Akış: `onInstalled` sağ tık menüsünü kaydeder, `contextMenus.onClicked`
`startAnalysisInBackground` fonksiyonunu çağırır. `chrome.storage.local` içinden `client_id`,
`currentUser`, `language` ve `authToken` okunur; istek `Authorization: Bearer <token>`
başlığıyla `http://127.0.0.1:3000/v1/analyze` adresine gider. `401` dönerse depoya
`isLoggedIn: false` ve `authToken: null` yazılır — oturum sessizce ölmez.

Sonuç `chrome.storage.local`'a yazılır, popup oradan okur. Ardından
`chrome.scripting.executeScript` ile boyama fonksiyonu sayfaya enjekte edilir.

### Sayfa boyamada üç sağlamlık kararı

1. **Esnek regex.** Eski kod `text.includes()` kullanıyordu. Modelin döndürdüğü cümle
   sayfadaki metinden tek karakter farklıysa hiçbir şey işaretlenmiyordu. Artık harf
   duyarsız, esnek boşluklu, baş/son noktalama toleranslı bir regex kuruluyor.
2. **CSSOM ile stil.** Temel görsel stiller doğrudan `element.style` üzerinden uygulanıyor.
   Katı CSP'li siteler `<style>` enjeksiyonunu engellese bile işaret görünür kalıyor.
3. **Sayaç dönüyor.** Boyama fonksiyonu kaç işaret koyduğunu döndürüyor; hiç eşleşme yoksa
   seçili metnin tamamı baskın manipülasyon renginde işaretleniyor.

## Neden böyle?

**Neden Manifest V3?** Chrome Web Store artık V2 kabul etmiyor. Bedeli: kalıcı arka plan
sayfası yok, durumu `chrome.storage` üzerinden taşımak zorundasın.

**Neden her siteye izin?** Metin herhangi bir sayfada seçilebilir ve işaret o sayfaya
konur. Dar bir izin listesi uzantıyı işlevsiz bırakırdı. Bunun ürkütücü göründüğü kabul
ediliyor; gizlilik sayfası nedenini açıkça yazıyor.

## Bilinen sınır

Sunucu adresi `background.js` içinde **sabit** yazılı (`127.0.0.1:3000`). `popup.js` içinde
bir `getBaseUrl()` fonksiyonu var ama o da aynı sabiti döndürüyor — uzaktan erişim desteği
yarım kalmış. Kurulum sayfası yalnız yerel adresi anlatıyor; uzaktan erişim yol haritasında.
