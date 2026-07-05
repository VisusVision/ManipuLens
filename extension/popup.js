document.getElementById('analyze-btn').addEventListener('click', async () => {
  const resultDiv = document.getElementById('result');
  resultDiv.innerHTML = "<p style='text-align:center; color:#6c757d;'>🕵️ Ajanlar metni inceliyor, lütfen bekleyin...</p>";

  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
  
  chrome.scripting.executeScript({
    target: { tabId: tab.id },
    func: () => window.getSelection().toString()
  }, async (selection) => {
    
    if (!selection || !selection[0] || !selection[0].result.trim()) {
      resultDiv.innerText = "Lütfen önce sayfada analiz etmek istediğiniz bir metni fareyle seçin.";
      return;
    }

    const selectedText = selection[0].result;

    try {
      const response = await fetch('http://localhost:3000/v1/analyze', { 
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: selectedText })
      });

      if (!response.ok) throw new Error("Sunucu hatası");

      const data = await response.json();
      
      if (data.is_manipulated) {
        let html = `<span class="badge danger">Manipülasyon Tespit Edildi!</span>`;
        html += `<p><strong>Baskın Yöntem:</strong> ${data.dominant_manipulation}</p>`;
        html += `<p><strong>Özet:</strong> ${data.genel_sonuc}</p><hr>`;
        html += `<h4>Ajan Analiz Grafikleri:</h4>`;
        
        let allTargetSentences = [];

        data.detailed_analyses.forEach(agent => {
          if (agent.detected) {
            // Sayfada çizilecek cümleleri topla
            allTargetSentences = allTargetSentences.concat(agent.target_sentences);

            // Ajan tipine göre dinamik CSS sınıfı belirle
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

        // Sayfa üzerinde renklendirme fonksiyonunu tetikle
        if (allTargetSentences.length > 0) {
          chrome.scripting.executeScript({
            target: { tabId: tab.id },
            func: highlightSentencesOnPage,
            args: [allTargetSentences]
          });
        }

      } else {
        resultDiv.innerHTML = `<span class="badge safe">✓ Temiz Metin</span><p>Ajanlar bu metinde herhangi bir manipülasyon taktiği tespit edemedi.</p>`;
      }

    } catch (error) {
      resultDiv.innerText = "Hata oluştu! Rust sunucusunun açık olduğundan emin olun.";
      console.error(error);
    }
  });
});

// Ajan türüne göre renk sınıfları yardımcı fonksiyonları
function getAgentClass(type) {
  const map = { "Dilsel": "linguistic", "Psikolojik": "psychological", "Davranışsal": "behavioral", "Algısal": "perceptual", "Sosyal": "social" };
  return map[type] || "";
}

function getProgressBarColor(type) {
  const map = { "Dilsel": "#4cc9f0", "Psikolojik": "#f72585", "Davranışsal": "#f8961e", "Algısal": "#7209b7", "Sosyal": "#4361ee" };
  return map[type] || "#4361ee";
}

// --- SAYFA ÜZERİNDE RENKLENDİRME YAPAN ENJEKSİYON FONKSİYONU ---
function highlightSentencesOnPage(sentences) {
  sentences.forEach(sentence => {
    if (!sentence || sentence.trim().length < 5) return;
    
    // Sayfa dökümanında (DOM) eşleşen metinleri bulup etrafını sarmak için regex kullanıyoruz
    const bodyText = document.body.innerHTML;
    
    // Özel karakterleri kaçırıyoruz (escape)
    const escapedSentence = sentence.replace(/[-\/\\^$*+?.()|[\]{}]/g, '\\$&');
    
    // Eğer cümle zaten renklendirilmemişse etrafını <mark> ile sar
    const regex = new RegExp(`(?<!<mark[^>]*>)${escapedSentence}`, 'g');
    
    if (bodyText.includes(sentence)) {
      document.body.innerHTML = document.body.innerHTML.replace(
        regex, 
        `<mark style="background-color: #ffeb3b; color: black; padding: 2px; border-radius: 4px; font-weight: 500; box-shadow: 0 1px 3px rgba(0,0,0,0.1);" title="Manipülasyon Şüphesi!">${sentence}</mark>`
      );
    }
  });
}