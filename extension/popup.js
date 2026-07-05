document.addEventListener('DOMContentLoaded', async () => {
  // Arka plandan (sağ tık menüsünden) gelen bekleyen bir metin var mı kontrol et
  chrome.storage.local.get(['pendingText'], async (result) => {
    if (result.pendingText) {
      const selectedText = result.pendingText;
      chrome.storage.local.remove('pendingText');
      startAnalysisWithText(selectedText);
    }
  });
});

// Ajanları ve buton durumunu yöneten ana fonksiyon
async function startAnalysisWithText(selectedText) {
  const resultDiv = document.getElementById('result');
  const analyzeBtn = document.getElementById('analyze-btn');
  
  // 1. Butonu Pasif Yap ve Yüklenme Yazısını Koy
  analyzeBtn.disabled = true;
  analyzeBtn.innerText = "🕵️ Ajanlar İnceliyor...";
  resultDiv.innerHTML = "<p style='text-align:center; color:#6c757d;'>🕵️ Ajanlar metni inceliyor, lütfen bekleyin...</p>";

  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });

  try {
    const response = await fetch('http://127.0.0.1:3000/v1/analyze', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
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
            <div class="intent-box">
              🎯 <strong>Tüketici Eğilim Tahmini:</strong>
              <p style="margin: 4px 0 0 0; font-style: italic;">"${data.predicted_product}"</p>
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
      // Yenilik: Şık ve yeşil temiz rapor kartı tasarımı yansıtılıyor
      resultDiv.innerHTML = `
        <span class="badge safe">✓ Temiz Metin</span>
        <div class="card clean-report">
          <p>🛡️ <strong>Güvenli İçerik:</strong></p>
          <p style="color: #1e3a1e; margin-top: 4px;">ManipuLens uzman yapay zeka ajanları bu metin üzerinde herhangi bir dilsel, psikolojik, algısal veya davranışsal manipülasyon taktiği tespit etmedi.</p>
        </div>
      `;
    }

  } catch (error) {
    resultDiv.innerText = "Hata oluştu! Rust sunucusunun açık olduğundan emin olun.";
    console.error(error);
  } finally {
    // 2. İşlem bittiğinde (Başarılı veya Başarısız) Butonu Eski Haline Döndür
    analyzeBtn.disabled = false;
    analyzeBtn.innerText = "Seçili Metni Analiz Et";
  }
}

// Mavi buton tıklama olayı (Tek ve temizlenmiş hali)
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
    startAnalysisWithText(selection[0].result);
  });
});

function getAgentClass(type) {
  const map = { "Dilsel": "linguistic", "Psikolojik": "psychological", "Davranışsal": "behavioral", "Algısal": "perceptual", "Sosyal": "social" };
  return map[type] || "";
}

function getProgressBarColor(type) {
  const map = { "Dilsel": "#4cc9f0", "Psikolojik": "#f72585", "Davranışsal": "#f8961e", "Algısal": "#7209b7", "Sosyal": "#4361ee" };
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





