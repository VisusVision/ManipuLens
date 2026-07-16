// Eklenti yüklendiğinde sağ tık menüsünü oluştur
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: "analyze-selection",
    title: "Seçili Metni ManipuLens ile Analiz Et",
    contexts: ["selection"]
  });

  ensureClientId();
});

// client_id yoksa oluştur
async function ensureClientId() {
  const data = await chrome.storage.local.get("client_id");
  if (!data.client_id) {
    const id = "user_" + Math.random().toString(36).substring(2, 10) + Date.now().toString(36);
    await chrome.storage.local.set({ client_id: id });
    console.log("Yeni client_id oluşturuldu:", id);
  }
}

// ===== PENCERE AÇ =====
chrome.action.onClicked.addListener(async () => {
  const windows = await chrome.windows.getAll({ populate: true });
  const existing = windows.find(w =>
    w.type === "popup" &&
    w.tabs?.[0]?.url?.includes("popup.html")
  );

  if (existing) {
    chrome.windows.update(existing.id, { focused: true });
  } else {
    chrome.windows.create({
      url: "popup.html",
      type: "popup",
      width: 400,
      height: 660,
      focused: true
    });
  }
});

// Sağ tık menüsüne tıklandığında
chrome.contextMenus.onClicked.addListener(async (info, tab) => {
  if (info.menuItemId === "analyze-selection" && info.selectionText) {
    startAnalysisInBackground(info.selectionText, tab.id);

    const windows = await chrome.windows.getAll({ populate: true });
    const existing = windows.find(w =>
      w.type === "popup" &&
      w.tabs &&
      w.tabs[0] &&
      w.tabs[0].url &&
      w.tabs[0].url.includes("popup.html")
    );

    if (existing) {
      chrome.windows.update(existing.id, { focused: true });
    } else {
      chrome.windows.create({
        url: "popup.html",
        type: "popup",
        width: 400,
        height: 660,
        focused: true
      });
    }
  }
});

// Popup'tan gelen analiz isteği
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.action === "startAnalysis") {
    startAnalysisInBackground(message.text, message.tabId);
  }
});

// ============== ANA ANALİZ FONKSİYONU (LOKAL) ==============
async function startAnalysisInBackground(selectedText, tabId) {
  await chrome.storage.local.set({
    analysisStatus: "running",
    lastAnalysisResult: null,
    analysisError: null
  });

  try {
    const storage = await chrome.storage.local.get("client_id");
    const clientId = storage.client_id || "anonim";

    // LOKAL sunucu
    const baseUrl = "http://localhost:3000";

    const response = await fetch(`${baseUrl}/v1/analyze`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        text: selectedText,
        client_id: clientId
      })
    });

    if (!response.ok) throw new Error("Sunucu hatası: " + response.status);

    const data = await response.json();

    await chrome.storage.local.set({
      analysisStatus: "done",
      lastAnalysisResult: data,
      analysisError: null,
      lastTabId: tabId
    });

    // Vurgulama
    if (tabId && data.is_manipulated && data.detailed_analyses) {
      let allTargetSentences = [];
      data.detailed_analyses.forEach(agent => {
        if (agent.detected) {
          allTargetSentences = allTargetSentences.concat(agent.target_sentences || []);
        }
      });

      if (allTargetSentences.length > 0) {
        const dominantColor = getProgressBarColor(data.dominant_manipulation);
        try {
          await chrome.scripting.executeScript({
            target: { tabId: tabId },
            func: highlightSentencesOnPage,
            args: [allTargetSentences, dominantColor]
          });
        } catch (e) {
          console.log("Vurgulama yapılamadı:", e);
        }
      }
    }

  } catch (error) {
    console.error("Arka plan analiz hatası:", error);
    await chrome.storage.local.set({
      analysisStatus: "error",
      lastAnalysisResult: null,
      analysisError: error.message || "Bilinmeyen hata"
    });
  }
}

function getProgressBarColor(type) {
  const map = {
    "Dilsel": "#4cc9f0",
    "Psikolojik": "#f72585",
    "Davranışsal": "#f8961e",
    "Algısal": "#7209b7",
    "Sosyal": "#4361ee",
    "Pazarlama": "#2a9d8f",
    "Tüketici Manipülasyonu": "#2a9d8f"
  };
  return map[type] || "#4361ee";
}

function highlightSentencesOnPage(sentences, highlightColor) {
  if (!sentences || sentences.length === 0) return;

  const color = highlightColor || "#4361ee";

  const walker = document.createTreeWalker(
    document.body,
    NodeFilter.SHOW_TEXT,
    {
      acceptNode: function (node) {
        const parent = node.parentNode;
        if (!parent) return NodeFilter.FILTER_REJECT;

        const tag = parent.nodeName.toLowerCase();
        if (tag === "script" || tag === "style" || tag === "noscript" || tag === "textarea") {
          return NodeFilter.FILTER_REJECT;
        }
        if (parent.closest && parent.closest("mark")) {
          return NodeFilter.FILTER_REJECT;
        }
        return NodeFilter.FILTER_ACCEPT;
      }
    }
  );

  const textNodes = [];
  let currentNode;
  while ((currentNode = walker.nextNode())) {
    textNodes.push(currentNode);
  }

  sentences.forEach((sentence) => {
    if (!sentence || sentence.trim().length < 5) return;

    const searchText = sentence.trim();

    textNodes.forEach((node) => {
      const text = node.nodeValue;
      if (!text || !text.includes(searchText)) return;

      const parent = node.parentNode;
      if (!parent) return;

      const parts = text.split(searchText);
      if (parts.length === 1) return;

      const fragment = document.createDocumentFragment();

      parts.forEach((part, index) => {
        if (part) {
          fragment.appendChild(document.createTextNode(part));
        }
        if (index < parts.length - 1) {
          const mark = document.createElement("mark");
          mark.textContent = searchText;
          mark.style.backgroundColor = color;
          mark.style.color = "white";
          mark.style.padding = "1px 3px";
          mark.style.borderRadius = "3px";
          mark.style.fontWeight = "500";
          mark.title = "Manipülasyon Şüphesi!";
          fragment.appendChild(mark);
        }
      });

      parent.replaceChild(fragment, node);
    });
  });
}