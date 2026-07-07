// 1. TETİKLEYİCİ: Sağ tık menüsünden bir metin gönderildi mi kontrol et
document.addEventListener('DOMContentLoaded', () => {
  chrome.storage.local.get(["actionTriggeredText"], function(result) {
    if (result.actionTriggeredText) {
      const targetText = result.actionTriggeredText;
      chrome.storage.local.remove(["actionTriggeredText"]);
      startDirectAnalysis(targetText);
    }
  });
});

// 2. TETİKLEYİCİ: Popup içindeki mavi butona basıldığında çalışacak alan
document.getElementById('analyze-btn').addEventListener('click', async () => {
  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
  
  chrome.scripting.executeScript({
    target: { tabId: tab.id },
    func: () => window.getSelection().toString()
  }, async (selection) => {
    if (!selection || !selection[0] || !selection[0].result.trim()) {
      document.getElementById('result').innerText = "Lütfen önce sayfada analiz etmek istediğiniz bir metni fareyle seçin.";
      return;
    }
    // Seçilen metni al ve analiz fonksiyonuna gönder
    startDirectAnalysis(selection[0].result);
  });
});

// ANA ANALİZ MOTORU: Hem sağ tık hem buton analiz için bu fonksiyonu besler
async function startDirectAnalysis(selectedText) {
  const resultDiv = document.getElementById('result');
  resultDiv.innerHTML = "<p style='text-align:center; color:#6c757d;'>🕵️ Ajanlar metni inceliyor, lütfen bekleyin...</p>";

  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });

  try {
    // 1. GitHub'dan güncel linki çekiyoruz
        // Linkin sonuna anlık saat bilgisini (timestamp) ekleyerek Chrome'u kandırıyoruz.
// Böylece her seferinde yepyeni bir dosya aradığını sanıp önbelleği atlıyor.
        const githubRawUrl = `https://raw.githubusercontent.com/VisusVision/ManipuLens/feature/remote-backend-sync/extension/server_config.json?t=${Date.now()}`;
        const configResponse = await fetch(githubRawUrl, { cache: 'no-store' });
        const configData = await configResponse.json();
    const baseUrl = configData.ngrok_url;

    // 2. Çektiğimiz link ile kendi sunucumuza bağlanıyoruz
    const response = await fetch(`${baseUrl}/v1/analyze`, {
      method: 'POST',
      headers: { 
        'Content-Type': 'application/json',
        'ngrok-skip-browser-warning': 'true'
      },
      body: JSON.stringify({ text: selectedText })
    }); 
    
    if (!response.ok) throw new Error("Sunucu hatası");

    const data = await response.json();
    
    if (data.is_manipulated) {
      let html = `<span class="badge danger">Manipülasyon Tespit Edildi!</span>`;
      html += `<p><strong>Baskın Yöntem:</strong> ${data.dominant_manipulation}</p>`;
      html += `<p><strong>Özet:</strong> ${data.genel_sonuc}</p>`;

      if (data.predicted_product) {
        html += `
          <div class="prediction-box">
            <span class="prediction-title">🔮 Satın Alma Eğilimi Tahmini:</span>
            <p class="prediction-text">${data.predicted_product}</p>
          </div>
        `;
      }

      html += `<hr><h4>Ajan Analiz Grafikleri:</h4>`;
      
      let allTargetSentences = [];

      data.detailed_analyses.forEach(agent => {
        if (agent.detected) {
          allTargetSentences = allTargetSentences.concat(agent.target_sentences);
          const typeClass = getAgentClass(agent.manipulation_type);
          const percentage = (agent.confidence_score * 100).toFixed(0);

          html += `
            <div class="card ${typeClass}">
              <p><strong>[${agent.manipulation_type} Manipülasyon]</strong></p>
              <p>${agent.aciklama}</p>
              <div class="progress-container">
                <div class="progress-bar" style="width: ${percentage}%; background-color: ${getProgressBarColor(agent.manipulation_type)};"></div>
              </div>
              <span class="score-text">%${percentage} Güven</span>
            </div>
          `;
        }
      });
      
      resultDiv.innerHTML = html;

      if (allTargetSentences.length > 0) {
        const dominantColor = getProgressBarColor(data.dominant_manipulation);
        chrome.scripting.executeScript({
          target: { tabId: tab.id },
          func: highlightSentencesOnPage,
          args: [allTargetSentences, dominantColor]
        });
      }

    } else {
      resultDiv.innerHTML = `<span class="badge safe">✓ Temiz Metin</span><p>Ajanlar bu metinde herhangi bir manipülasyon taktiği tespit edemedi.</p>`;
    }

  } catch (error) {
    resultDiv.innerText = "Hata oluştu! Rust sunucusunun açık olduğundan emin olun.";
    console.error(error);
  }
}

// YARDIMCI FONKSİYONLAR
function getAgentClass(type) {
  const map = { 
    "Dilsel": "linguistic", 
    "Psikolojik": "psychological", 
    "Davranışsal": "behavioral", 
    "Algısal": "perceptual", 
    "Sosyal": "social",
    "Pazarlama": "marketing" 
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
    "Pazarlama": "#2a9d8f" 
  };
  return map[type] || "#4361ee";
}

function highlightSentencesOnPage(sentences, highlightColor) {
  sentences.forEach(sentence => {
    if (!sentence || sentence.trim().length < 5) return;
    
    const bodyText = document.body.innerHTML;
    const escapedSentence = sentence.replace(/[-\/\\^$*+?.()|[\]{}]/g, '\\$&');
    const regex = new RegExp(`(?<!<mark[^>]*>)${escapedSentence}`, 'g');
    
    if (bodyText.includes(sentence)) {
      document.body.innerHTML = document.body.innerHTML.replace(
        regex, 
        `<mark style="background-color: ${highlightColor}; color: white; padding: 2px; border-radius: 4px; font-weight: 500; box-shadow: 0 1px 3px rgba(0,0,0,0.1);" title="Manipülasyon Şüphesi!">${sentence}</mark>`
      );
    }
  });
}