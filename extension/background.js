// Eklenti yüklendiğinde sağ tık menüsünü oluştur
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: "analyze-manipulens",
    title: "ManipuLens ile Analiz Et",
    contexts: ["selection"] // Sadece metin seçildiğinde görünsün
  });
});

// Sağ tık menüsündeki butona tıklandığında çalışacak fonksiyon
chrome.contextMenus.onClicked.addListener((info, tab) => {
  if (info.menuItemId === "analyze-manipulens") {
    const selectedText = info.selectionText;

    // Seçilen metni popup.js'in yakalayabilmesi için yerel hafızaya (storage) geçici olarak kaydediyoruz
    chrome.storage.local.set({ pendingText: selectedText }, () => {
      // Kullanıcıya kolaylık olsun diye uzantının popup ekranını otomatik açıyoruz (Chrome 116+ destekler)
      if (chrome.action && chrome.action.openPopup) {
        chrome.action.openPopup();
      } else {
        console.log("Popup'ı açmak için lütfen eklenti ikonuna tıklayın, metniniz hazır.");
      }
    });
  }
});