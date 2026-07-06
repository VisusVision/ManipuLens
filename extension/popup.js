// ANA ANALİZ MOTORU: Hem sağ tık hem buton analiz için bu fonksiyonu besler
async function startDirectAnalysis(selectedText) {
  const resultDiv = document.getElementById('result');
  resultDiv.innerHTML = "<p style='text-align:center; color:#6c757d;'>🕵️ Ajanlar metni inceliyor, lütfen bekleyin...</p>";

  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });

  try {
    // --- BURASI EKLENDİ ---
    const githubRawUrl = 'https://raw.githubusercontent.com/VisusVision/ManipuLens/feature/remote-backend-sync/extension/server_config.json';
    const configResponse = await fetch(githubRawUrl);
    const configData = await configResponse.json();
    const baseUrl = configData.ngrok_url;
    // ----------------------

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
    resultDiv.innerText = "Hata oluştu! Rust sunucusunun veya ngrok'un açık olduğundan emin olun.";
    console.error(error);
  }
}