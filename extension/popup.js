// ============== DİL SİSTEMİ ==============
let currentLang = "tr";
let isAnalyzing = false;

const i18n = {
  tr: {
    subTitle: "Yapay Zeka Analizi",
    analyzeBtn: "Seçili Metni Analiz Et",
    historyBtn: "Geçmişim",
    selectText: "Sayfadan bir metin seçin ve ajanları tetikleyin.",
    selectTextError: "Lütfen önce sayfada analiz etmek istediğiniz bir metni fareyle seçin.",
    loadingHistory: "Geçmiş yükleniyor...",
    noHistory: "Henüz geçmiş kaydın yok.",
    historyTitle: "Geçmiş Analizlerin",
    historyError: "Geçmiş yüklenirken hata oluştu.",
    connectionError: "Bağlantı Hatası: Sunucuya ulaşılamadı.",
    technicalDetail: "Teknik Detay",
    dominantMethod: "Baskın Yöntem",
    detected: "Tespit Edildi",
    cleanText: "✓ Temiz Metin",
    noManipulation: "Ajanlar bu metinde herhangi bir manipülasyon taktiği tespit edemedi.",
    agentAnalyses: "Ajan Analizleri",
    manipulationLevel: "Manipülasyon Seviyesi",
    noDetection: "Tespit Yok",
    upperLevel: "Üst Seviye",
    midLevel: "Orta Seviye",
    lowLevel: "Az Seviye",
    predictionTitle: "Satın Alma Eğilimi Tahmini:",
    manipulation: "Manipülasyon",
    clean: "Temiz",
    loadingStages: [
      "Ajanlar başlatılıyor...",
      "Dilsel ajan inceliyor...",
      "Psikolojik ajan inceliyor...",
      "Davranışsal ajan inceliyor...",
      "Algısal ajan inceliyor...",
      "Sosyal ajan inceliyor...",
      "Pazarlama ajanı inceliyor...",
      "Sonuçlar birleştiriliyor...",
      "Rapor hazırlanıyor..."
    ],
    agentNames: {
      "Dilsel": "Dilsel",
      "Psikolojik": "Psikolojik",
      "Davranışsal": "Davranışsal",
      "Algısal": "Algısal",
      "Sosyal": "Sosyal",
      "Pazarlama": "Pazarlama",
      "Tüketici Manipülasyonu": "Tüketici"
    }
  },
  en: {
    subTitle: "AI Analysis",
    analyzeBtn: "Analyze Selected Text",
    historyBtn: "My History",
    selectText: "Select a text on the page and trigger the agents.",
    selectTextError: "Please first select the text you want to analyze on the page.",
    loadingHistory: "Loading history...",
    noHistory: "No history records yet.",
    historyTitle: "Your Analysis History",
    historyError: "Error loading history.",
    connectionError: "Connection Error: Could not reach the server.",
    technicalDetail: "Technical Detail",
    dominantMethod: "Dominant Method",
    detected: "Detected",
    cleanText: "✓ Clean Text",
    noManipulation: "The agents did not detect any manipulation tactics in this text.",
    agentAnalyses: "Agent Analyses",
    manipulationLevel: "Manipulation Level",
    noDetection: "Not Detected",
    upperLevel: "High Level",
    midLevel: "Medium Level",
    lowLevel: "Low Level",
    predictionTitle: "Purchase Intent Prediction:",
    manipulation: "Manipulation",
    clean: "Clean",
    loadingStages: [
      "Starting agents...",
      "Linguistic agent analyzing...",
      "Psychological agent analyzing...",
      "Behavioral agent analyzing...",
      "Perceptual agent analyzing...",
      "Social agent analyzing...",
      "Marketing agent analyzing...",
      "Merging results...",
      "Preparing report..."
    ],
    agentNames: {
      "Dilsel": "Linguistic",
      "Psikolojik": "Psychological",
      "Davranışsal": "Behavioral",
      "Algısal": "Perceptual",
      "Sosyal": "Social",
      "Pazarlama": "Marketing",
      "Tüketici Manipülasyonu": "Consumer"
    }
  }
};

function t(key) {
  return i18n[currentLang][key] || key;
}

function translateAgentName(name) {
  return i18n[currentLang].agentNames[name] || name;
}

function applyLanguage() {
  document.getElementById("sub-title").innerText = t("subTitle");
  document.getElementById("analyze-btn").innerText = t("analyzeBtn");
  document.getElementById("history-btn").innerText = t("historyBtn");
  document.getElementById("lang-btn").innerText = currentLang === "tr" ? "EN" : "TR";

  const resultDiv = document.getElementById("result");
  if (resultDiv.innerText.includes("metin seçin") || resultDiv.innerText.includes("Select a text")) {
    resultDiv.innerText = t("selectText");
  }
}

// ============== ANA KOD ==============
document.addEventListener("DOMContentLoaded", async () => {
  const stored = await chrome.storage.local.get("language");
  currentLang = stored.language || "tr";
  applyLanguage();

  document.getElementById("lang-btn").addEventListener("click", async () => {
    if (isAnalyzing) return;

    currentLang = currentLang === "tr" ? "en" : "tr";
    await chrome.storage.local.set({ language: currentLang });
    applyLanguage();

    const data = await chrome.storage.local.get(["analysisStatus", "lastAnalysisResult"]);
    if (data.analysisStatus === "done" && data.lastAnalysisResult) {
      renderResult(data.lastAnalysisResult);
    }
  });

  const analyzeBtn = document.getElementById("analyze-btn");
  if (analyzeBtn) {
    analyzeBtn.addEventListener("click", async () => {
      const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });

      chrome.scripting.executeScript({
        target: { tabId: tab.id },
        func: () => window.getSelection().toString()
      }, async (selection) => {
        const selectedText = selection?.[0]?.result?.trim();

        if (!selectedText) {
          const data = await chrome.storage.local.get(["analysisStatus", "lastAnalysisResult"]);
          
          if (data.analysisStatus === "done" && data.lastAnalysisResult) {
            renderResult(data.lastAnalysisResult);
          } else {
            document.getElementById("result").innerText = t("selectTextError");
          }
          return;
        }

        isAnalyzing = true;
        document.getElementById("lang-btn").style.opacity = "0.4";
        document.getElementById("lang-btn").style.pointerEvents = "none";

        chrome.runtime.sendMessage({
          action: "startAnalysis",
          text: selectedText,
          tabId: tab.id
        });

        startLoadingAnimation();
      });
    });
  }

  const historyBtn = document.getElementById("history-btn");
  if (historyBtn) {
    historyBtn.addEventListener("click", async () => {
      const resultDiv = document.getElementById("result");
      resultDiv.innerHTML = `<p style="text-align:center; color:#6c757d;">${t("loadingHistory")}</p>`;

      try {
        // LOKAL sunucu
        const response = await fetch("http://localhost:3000/v1/history", {
          headers: { "Content-Type": "application/json" }
        });

        if (!response.ok) throw new Error("error");

        const history = await response.json();

        if (!history || history.length === 0) {
          resultDiv.innerHTML = `<p style="text-align:center; color:#6c757d;">${t("noHistory")}</p>`;
          return;
        }

        let html = `<h4 style="margin-top:0;">${t("historyTitle")}</h4>`;

        history.forEach((item) => {
          const date = new Date(item.timestamp).toLocaleString(currentLang === "tr" ? "tr-TR" : "en-US");
          const typeClass = getAgentClass(item.dominant_manipulation);
          const borderColor = getProgressBarColor(item.dominant_manipulation);

          const badgeColor = item.is_manipulated ? borderColor : "#2a9d8f";
          const badgeText = item.is_manipulated ? t("manipulation") : t("clean");

          html += `
            <div class="card ${typeClass}" style="margin-bottom:12px; padding:10px; border-left-color: ${borderColor};">
              <div style="font-size:11px; color:#888; margin-bottom:6px;">${date}</div>
              <span class="badge" style="background-color: ${badgeColor}; margin-bottom:8px;">${badgeText}</span>
              <p style="margin:6px 0 4px 0; font-size:13px; font-weight:700; color:${borderColor};">
                ${escapeHTML(translateAgentName(item.dominant_manipulation) || "-")}
              </p>
              <p style="margin:0 0 6px 0; font-size:12px; color:#555;">
                ${escapeHTML(item.genel_sonuc || "")}
              </p>
              <p style="margin:0; font-size:11px; color:#999; font-style:italic;">
                "${escapeHTML(item.text_preview)}..."
              </p>
            </div>
          `;
        });

        resultDiv.innerHTML = html;

      } catch (error) {
        console.error(error);
        resultDiv.innerHTML = `<p style="text-align:center; color:#e63946;">${t("historyError")}</p>`;
      }
    });
  }

  checkAndShowLastResult();

  chrome.storage.onChanged.addListener((changes) => {
    if (changes.analysisStatus || changes.lastAnalysisResult || changes.analysisError) {
      checkAndShowLastResult();
    }
  });
});

let loadingInterval = null;

function startLoadingAnimation() {
  if (loadingInterval) clearInterval(loadingInterval);
  let stageIndex = 0;
  const stages = t("loadingStages");
  updateLoadingUI(stages[0]);
  loadingInterval = setInterval(() => {
    stageIndex = (stageIndex + 1) % stages.length;
    updateLoadingUI(stages[stageIndex]);
  }, 2400);
}

function stopLoadingAnimation() {
  if (loadingInterval) {
    clearInterval(loadingInterval);
    loadingInterval = null;
  }
  isAnalyzing = false;
  const langBtn = document.getElementById("lang-btn");
  if (langBtn) {
    langBtn.style.opacity = "1";
    langBtn.style.pointerEvents = "auto";
  }
}

function updateLoadingUI(text) {
  const resultDiv = document.getElementById("result");
  if (!resultDiv) return;
  resultDiv.innerHTML = `
    <div class="loading-container">
      <div class="loading-icon">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="10.5" cy="10.5" r="6.5" stroke="url(#grad)" stroke-width="2.4"/>
          <path d="M19.5 19.5L15.2 15.2" stroke="url(#grad)" stroke-width="2.4" stroke-linecap="round"/>
          <defs>
            <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stop-color="#4361ee"/>
              <stop offset="50%" stop-color="#7209b7"/>
              <stop offset="100%" stop-color="#f72585"/>
            </linearGradient>
          </defs>
        </svg>
      </div>
      <div class="loading-text">${text}</div>
      <div class="progress-track">
        <div class="progress-fill"></div>
      </div>
    </div>`;
}

async function checkAndShowLastResult() {
  const resultDiv = document.getElementById("result");
  if (!resultDiv) return;

  const data = await chrome.storage.local.get([
    "analysisStatus",
    "lastAnalysisResult",
    "analysisError"
  ]);

  if (data.analysisStatus === "running") {
    isAnalyzing = true;
    document.getElementById("lang-btn").style.opacity = "0.4";
    document.getElementById("lang-btn").style.pointerEvents = "none";
    if (!loadingInterval) startLoadingAnimation();
    return;
  }

  stopLoadingAnimation();

  if (data.analysisStatus === "error") {
    resultDiv.innerHTML = `
      <p style="color: #e63946; font-weight: bold;">${t("connectionError")}</p>
      <small style="color: #6c757d;">${t("technicalDetail")}: ${escapeHTML(data.analysisError || "Unknown error")}</small>
    `;
    return;
  }

  if (data.analysisStatus === "done" && data.lastAnalysisResult) {
    renderResult(data.lastAnalysisResult);
    return;
  }

  resultDiv.innerHTML = t("selectText");
}

function renderResult(data) {
  const resultDiv = document.getElementById("result");
  if (!resultDiv) return;

  let html = "";

  if (data.is_manipulated) {
    const dominantColor = getProgressBarColor(data.dominant_manipulation);

    html += `
      <div class="dominant-box">
        <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:10px;">
          <div class="dominant-label" style="margin:0;">${t("dominantMethod")}</div>
          <span class="badge danger" style="margin:0; font-size:11px; padding:3px 9px;">${t("detected")}</span>
        </div>

        <div class="dominant-value" style="color: ${dominantColor}; font-size:16px; margin-bottom:10px;">
          ${escapeHTML(translateAgentName(data.dominant_manipulation))}
        </div>

        <p class="summary-text">${escapeHTML(data.genel_sonuc)}</p>
      </div>
    `;

    if (data.predicted_product) {
      html += `
        <div class="prediction-box">
          <span class="prediction-title">🔮 ${t("predictionTitle")}</span>
          <p class="prediction-text">${escapeHTML(data.predicted_product)}</p>
        </div>
      `;
    }
  } else {
    html += `<span class="badge safe">${t("cleanText")}</span>`;
    html += `<p style="margin-top:10px;">${t("noManipulation")}</p>`;
  }

  html += `<hr><h4 style="margin-bottom:14px;">${t("agentAnalyses")}</h4>`;

  data.detailed_analyses.forEach((agent, index) => {
    const typeClass = getAgentClass(agent.manipulation_type);
    const percentage = Math.round(agent.confidence_score * 100);
    const confidenceLabel = getConfidenceLabel(agent.confidence_score);
    const barColor = agent.detected ? getProgressBarColor(agent.manipulation_type) : "#adb5bd";
    const delay = index * 0.08;

    const titleStyle = agent.detected ? `background:${barColor}` : `background:#adb5bd`;
    const descText = agent.detected ? escapeHTML(agent.aciklama) : t("noDetection");
    const levelText = agent.detected ? confidenceLabel : t("noDetection");

    html += `
      <div class="card ${typeClass}" style="animation-delay: ${delay}s; ${!agent.detected ? 'opacity:0.75;' : ''}">
        <div class="agent-title">
          <span style="${titleStyle}">${escapeHTML(translateAgentName(agent.manipulation_type))}</span>
          ${t("manipulation")}
        </div>
        <p class="agent-desc">${descText}</p>
        <div class="level-row">
          <span class="level-label">${t("manipulationLevel")}</span>
          <span class="level-value" style="color:${barColor}">${levelText}</span>
        </div>
        <div class="progress-container">
          <div class="progress-bar" style="width: ${agent.detected ? percentage : 0}%; background-color: ${barColor};"></div>
        </div>
      </div>
    `;
  });

  resultDiv.innerHTML = html;

  setTimeout(() => {
    document.querySelectorAll('.progress-bar').forEach(bar => {
      const width = bar.style.width;
      bar.style.width = '0%';
      setTimeout(() => { bar.style.width = width; }, 30);
    });
  }, 80);
}

function escapeHTML(str) {
  if (!str) return "";
  return String(str)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

function getAgentClass(type) {
  const map = {
    "Dilsel": "linguistic",
    "Psikolojik": "psychological",
    "Davranışsal": "behavioral",
    "Algısal": "perceptual",
    "Sosyal": "social",
    "Pazarlama": "marketing",
    "Tüketici Manipülasyonu": "marketing"
  };
  return map[type] || "";
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

function getConfidenceLabel(score) {
  const percentage = score * 100;
  if (percentage >= 75) return t("upperLevel");
  if (percentage >= 40) return t("midLevel");
  return t("lowLevel");
}