// Eklenti yüklendiğinde sağ tık menüsünü oluştur
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: "analyze-selection",
    title: "Seçili Metni ManipuLens ile Analiz Et",
    contexts: ["selection"]
  });

  ensureClientId();
});

// client_id yoksa oluştur (yedek olarak)
async function ensureClientId() {
  const data = await chrome.storage.local.get(["client_id", "currentUser"]);
  if (!data.client_id) {
    const id = "user_" + Math.random().toString(36).substring(2, 10) + Date.now().toString(36);
    await chrome.storage.local.set({ client_id: id });
    console.log("Yeni client_id oluşturuldu:", id);
  }
}

// ===== PENCERE AÇ =====
// Boyutlar DIP cinsinden başlangıç değeri; popup.js açılışta kendi ekranına
// göre (window.screen) clamp eder — service worker'da screen objesi yok.
async function openOrFocusPopup() {
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
      width: 360,
      height: 660,
      focused: true
    });
  }
}

chrome.action.onClicked.addListener(() => {
  openOrFocusPopup();
});

// Popup'tan gelen analiz isteklerini dinle (analyze-btn bu mesajı gönderiyor)
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message?.action === "startAnalysis" && message.text) {
    startAnalysisInBackground(message.text, message.tabId);
  }
  return false;
});

// Sağ tık menüsüne tıklandığında
chrome.contextMenus.onClicked.addListener(async (info, tab) => {
  if (info.menuItemId === "analyze-selection" && info.selectionText) {
    startAnalysisInBackground(info.selectionText, tab.id);
    await openOrFocusPopup();
  }
});

// ============== ANA ANALİZ FONKSİYONU ==============
const MAX_TEXT_LEN = 1000;

async function startAnalysisInBackground(selectedText, tabId) {
  // Önce giriş yapmış kullanıcının oturum token'ını ve arayüz dilini al
  const storage = await chrome.storage.local.get(["client_id", "currentUser", "language", "authToken"]);
  const lang = storage.language === "en" ? "en" : "tr";

  // GÜVENLİK: /v1/analyze artık oturum token'ı istiyor. Token yoksa
  // (giriş yapılmamış / eski sürümden kalma oturum) kullanıcıyı girişe yönlendir.
  if (!storage.authToken) {
    await chrome.storage.local.set({
      analysisStatus: "error",
      lastAnalysisResult: null,
      analysisError:
        lang === "en"
          ? "Please log in first (open the ManipuLens window and sign in)."
          : "Lütfen önce giriş yapın (ManipuLens penceresini açıp oturum açın)."
    });
    return;
  }

  // İstemci tarafı uzunluk kontrolü: sunucuya boşuna istek atma
  if (selectedText.length > MAX_TEXT_LEN) {
    await chrome.storage.local.set({
      analysisStatus: "error",
      lastAnalysisResult: null,
      analysisError:
        lang === "en"
          ? `Selected text is too long (maximum ${MAX_TEXT_LEN} characters).`
          : `Seçilen metin çok uzun (maksimum ${MAX_TEXT_LEN} karakter).`
    });
    return;
  }

  await chrome.storage.local.set({
    analysisStatus: "running",
    lastAnalysisResult: null,
    analysisError: null
  });

  try {
    const githubRawUrl = `https://raw.githubusercontent.com/VisusVision/ManipuLens/feature/remote-backend-sync/extension/server_config.json?t=${Date.now()}`;
    const configResponse = await fetch(githubRawUrl, { cache: "no-store" });
    const configData = await configResponse.json();
    const baseUrl = configData.ngrok_url;

    const response = await fetch(`${baseUrl}/v1/analyze`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "ngrok-skip-browser-warning": "true",
        "Authorization": `Bearer ${storage.authToken}`
      },
      body: JSON.stringify({
        text: selectedText,
        lang: lang
      })
    });

    // Oturum düşmüş (token süresi doldu / şifre değişti): girişe yönlendir
    if (response.status === 401) {
      await chrome.storage.local.set({ isLoggedIn: false, authToken: null });
      throw new Error(
        lang === "en"
          ? "Your session has expired. Please log in again."
          : "Oturumunuzun süresi doldu. Lütfen tekrar giriş yapın."
      );
    }
    if (response.status === 429) {
      const msg = await response.text();
      throw new Error(msg || (lang === "en" ? "Too many requests." : "Çok fazla istek."));
    }
    if (!response.ok) throw new Error("Sunucu hatası: " + response.status);

    const data = await response.json();

    await chrome.storage.local.set({
      analysisStatus: "done",
      lastAnalysisResult: data,
      analysisLang: lang,        // Bu sonuç hangi dilde üretildi (dil değişiminde çeviri kararı için)
      translatedResults: {},     // Yeni analiz geldi -> eski çeviri önbelleği geçersiz
      analysisError: null,
      lastTabId: tabId
    });

    // Vurgulama: her cümle, onu tespit eden AJANIN renginde işaretlenir.
    // Aynı cümleyi birden fazla ajan hedeflediyse güven skoru yüksek olan kazanır.
    if (tabId && data.is_manipulated && data.detailed_analyses) {
      const agentNames = {
        tr: {
          "Dilsel": "Dilsel", "Psikolojik": "Psikolojik", "Davranışsal": "Davranışsal",
          "Algısal": "Algısal", "Sosyal": "Sosyal", "Pazarlama": "Pazarlama",
          "Tüketici Manipülasyonu": "Tüketici"
        },
        en: {
          "Dilsel": "Linguistic", "Psikolojik": "Psychological", "Davranışsal": "Behavioral",
          "Algısal": "Perceptual", "Sosyal": "Social", "Pazarlama": "Marketing",
          "Tüketici Manipülasyonu": "Consumer"
        }
      };
      const suffix = lang === "en" ? "Manipulation" : "Manipülasyon";

      const seen = new Map(); // cümle -> { sentence, color, label, score }
      data.detailed_analyses.forEach(agent => {
        if (!agent.detected) return;
        const color = getProgressBarColor(agent.manipulation_type);
        const name = agentNames[lang][agent.manipulation_type] || agent.manipulation_type;
        const label = `${name} ${suffix}`;

        (agent.target_sentences || []).forEach(s => {
          const key = (s || "").trim();
          if (key.length < 5) return;
          const prev = seen.get(key);
          if (!prev || agent.confidence_score > prev.score) {
            seen.set(key, { sentence: key, color, label, score: agent.confidence_score });
          }
        });
      });

      const targets = Array.from(seen.values());

      // GÜVENCE: Ajan cümleleri sayfada hiç eşleşmezse, analiz edilen SEÇİLİ
      // METNİN kendisi baskın ajanın renginde işaretlenir (seçim sayfadan
      // geldiği için eşleşmesi garantiye en yakın olandır).
      const domName = agentNames[lang][data.dominant_manipulation] || data.dominant_manipulation;
      const fallbackTarget = {
        sentence: selectedText,
        color: getProgressBarColor(data.dominant_manipulation),
        label: `${domName} ${suffix}`
      };

      if (targets.length > 0 || fallbackTarget.sentence) {
        try {
          const results = await chrome.scripting.executeScript({
            target: { tabId: tabId },
            func: highlightSentencesOnPage,
            args: [targets, fallbackTarget]
          });
          const marked = results?.[0]?.result ?? 0;
          console.log(`ManipuLens: ${targets.length} hedef cümleden ${marked} işaret kondu.`);
          if (marked === 0) {
            console.warn("ManipuLens: Hiçbir cümle sayfada eşleşmedi. Hedefler:", targets.map(t => t.sentence));
          }
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

// Sayfada cümleleri, tespit eden AJANIN renginde işaretler.
// Tasarım: metni ezmeyen yumuşak renk tonu + ajan renginde alt çizgi
// (fosforlu kalem hissi), hover'da tonun koyulaşması ve ajan adını gösteren
// sade bir tooltip.
//
// SAĞLAMLIK NOTLARI (işaretlemenin "çalışmama" sorununa karşı):
// 1) ESKİ KOD birebir text.includes() ile arıyordu; LLM'in döndürdüğü cümle
//    sayfadaki metinden TEK karakter bile (tırnak, boşluk, satır sonu, harf
//    büyüklüğü) farklıysa hiçbir şey işaretlenmiyordu. Artık esnek regex
//    kullanılır: harf duyarsız, boşluklar esnek, baş/sondaki tırnak ve
//    noktalama tolere edilir.
// 2) Temel görsel stiller CSSOM ile (element.style) uygulanır — katı CSP'li
//    siteler <style> enjeksiyonunu engellese bile işaret görünür kalır;
//    stylesheet yalnızca hover/tooltip için ek süsleme sağlar.
// 3) Fonksiyon kaç işaret koyduğunu döndürür; background bunu loglar.
// targets: [{ sentence, color, label }], fallbackTarget: seçili metnin tamamı
function highlightSentencesOnPage(targets, fallbackTarget) {
  targets = targets || [];
  if (targets.length === 0 && !fallbackTarget) return 0;

  // Hover + tooltip stilleri (bir kez enjekte edilir; CSP engellese de
  // temel görünüm aşağıdaki inline stillerle korunur)
  if (!document.getElementById("manipulens-highlight-style")) {
    try {
      const style = document.createElement("style");
      style.id = "manipulens-highlight-style";
      style.textContent = `
        mark.manipulens-mark:hover {
          filter: saturate(1.2) brightness(1.03);
        }
        mark.manipulens-mark::after {
          content: attr(data-ml-label);
          position: absolute;
          left: 50%;
          bottom: calc(100% + 7px);
          transform: translateX(-50%) translateY(4px);
          background: #10141f;
          color: #f5f7fb;
          border-bottom: 2px solid rgb(var(--mlc, 67, 97, 238));
          font: 500 11.5px/1.4 'Segoe UI', system-ui, sans-serif;
          letter-spacing: 0.2px;
          padding: 5px 11px;
          border-radius: 7px;
          white-space: nowrap;
          opacity: 0;
          pointer-events: none;
          transition: opacity 0.18s ease, transform 0.18s ease;
          z-index: 2147483647;
          box-shadow: 0 6px 18px rgba(0, 0, 0, 0.3);
        }
        mark.manipulens-mark:hover::after {
          opacity: 1;
          transform: translateX(-50%) translateY(0);
        }
      `;
      document.head.appendChild(style);
    } catch (e) { /* CSP engelledi — inline stiller yeterli */ }
  }

  const hexToRgb = (hex) => {
    const m = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex || "");
    return m
      ? `${parseInt(m[1], 16)}, ${parseInt(m[2], 16)}, ${parseInt(m[3], 16)}`
      : "67, 97, 238";
  };

  // Esnek eşleşme deseni: baş/sondaki tırnak-noktalama atılır, regex özel
  // karakterleri kaçırılır, her boşluk dizisi "herhangi bir boşluk" olur.
  const buildFlexibleRegex = (sentence) => {
    const cleaned = (sentence || "")
      .trim()
      .replace(/^["'‘’“”\s]+/, "")
      .replace(/["'‘’“”.…\s]+$/, "");
    if (cleaned.length < 5) return null;
    const pattern = cleaned
      .replace(/[.*+?^${}()|[\]\\]/g, "\\$&")
      .replace(/\s+/g, "[\\s\\u00A0]+");
    try {
      return new RegExp(pattern, "iu");
    } catch (e) {
      return null;
    }
  };

  const makeMark = (text, rgb, label) => {
    const mark = document.createElement("mark");
    mark.className = "manipulens-mark";
    mark.textContent = text;
    mark.setAttribute("data-ml-label", label || "ManipuLens");
    // CSP'ye dayanıklı temel görünüm (CSSOM üzerinden)
    const s = mark.style;
    s.setProperty("--mlc", rgb);
    s.backgroundColor = `rgba(${rgb}, 0.16)`;
    s.borderBottom = `2px solid rgb(${rgb})`;
    s.borderRadius = "4px";
    s.padding = "1px 3px";
    s.margin = "0 1px";
    s.color = "inherit";
    s.font = "inherit";
    s.position = "relative";
    s.cursor = "help";
    s.transition = "background-color 0.2s ease";
    s.webkitBoxDecorationBreak = "clone";
    s.boxDecorationBreak = "clone";
    return mark;
  };

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

  // KRİTİK DÜZELTME: Eski kod her text düğümünü TEK BAŞINA arıyordu.
  // Gerçek sitelerde cümleler <a>, <b>, <span> gibi etiketlerle birden çok
  // düğüme bölündüğü için tek düğümde arama neredeyse hiç eşleşmiyordu.
  // Yeni yaklaşım: tüm düğüm metinleri TEK metinde birleştirilir (düğüm
  // haritasıyla), eşleşme bu bütünde bulunur ve kesişen her düğüm parçası
  // Range ile ayrı ayrı sarılır — cümle etiket sınırlarını aşsa da işaretlenir.
  let fullText = "";
  const spans = []; // { node, start, end } — fullText içindeki konumlar
  textNodes.forEach((node) => {
    const len = node.nodeValue.length;
    spans.push({ node, start: fullText.length, end: fullText.length + len });
    fullText += node.nodeValue;
  });

  const takenIntervals = []; // çakışan işaretlemeleri önle
  const overlapsTaken = (s, e) =>
    takenIntervals.some((iv) => s < iv.e && e > iv.s);

  // Bir hedef için eşleşmeleri bul, düğüm parçalarına çevir
  const collectSegments = (target, acc) => {
    const re = buildFlexibleRegex(target.sentence);
    if (!re) return;
    const gre = new RegExp(re.source, "giu");
    const rgb = hexToRgb(target.color);

    let m;
    let guard = 0;
    while ((m = gre.exec(fullText)) && guard++ < 20) {
      const startIdx = m.index;
      const endIdx = m.index + m[0].length;
      if (m[0].length === 0) { gre.lastIndex++; continue; }
      if (overlapsTaken(startIdx, endIdx)) continue;
      takenIntervals.push({ s: startIdx, e: endIdx });

      for (const sp of spans) {
        if (sp.end <= startIdx) continue;
        if (sp.start >= endIdx) break;
        const s = Math.max(startIdx, sp.start) - sp.start;
        const e = Math.min(endIdx, sp.end) - sp.start;
        if (e > s) {
          acc.push({ node: sp.node, s, e, rgb, label: target.label, key: sp.start + s });
        }
      }
    }
  };

  const segments = [];
  targets.forEach((t) => collectSegments(t, segments));

  // Hiçbir ajan cümlesi eşleşmediyse seçili metnin tamamını işaretle
  if (segments.length === 0 && fallbackTarget && fallbackTarget.sentence) {
    collectSegments(fallbackTarget, segments);
  }

  // Belgedeki konuma göre TERSTEN uygula: bir düğümün sonunu sarmak,
  // aynı düğümün başındaki offsetleri geçersiz kılmaz.
  segments.sort((a, b) => b.key - a.key);

  let markedCount = 0;
  segments.forEach((seg) => {
    try {
      if (!seg.node.isConnected) return;
      const range = document.createRange();
      range.setStart(seg.node, seg.s);
      range.setEnd(seg.node, seg.e);
      // Yalnızca boşluktan oluşan parçaları sarma (görsel gürültü olmasın)
      if (!range.toString().trim()) return;
      const mark = makeMark(range.toString(), seg.rgb, seg.label);
      mark.textContent = "";
      range.surroundContents(mark);
      markedCount++;
    } catch (e) { /* tek parça sarılamadıysa kalanlar denenmeye devam eder */ }
  });

  return markedCount;
}