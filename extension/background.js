// Eklenti yüklendiğinde sağ tık menüsünü oluştur
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: "analyze-selection",
    title: "Seçili Metni ManipuLens ile Analiz Et",
    contexts: ["selection"] // Sadece metin seçildiğinde sağ tıkta görünsün
  });
});

// Sağ tık menüsüne tıklandığında çalışacak alan
chrome.contextMenus.onClicked.addListener((info, tab) => {
  if (info.menuItemId === "analyze-selection" && info.selectionText) {
    // Popup'ın açık olup olmadığını kontrol etmek ve veriyi aktarmak için 
    // yerel depolama (storage) veya mesajlaşma kullanabiliriz.
    // En güvenli yol, seçilen metni geçici olarak storage'a atmak ve popup'ı açmaktır.
    chrome.storage.local.set({ "actionTriggeredText": info.selectionText }, () => {
      // Kullanıcının uzantı popup ekranını programatik olarak açıyoruz
      chrome.action.openPopup();
    });
  }
});