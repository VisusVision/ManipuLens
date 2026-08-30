---
title: "Model çağrısı ve JSON zorlaması"
order: 6
parca: model
zorluk: orta
dosya: "src/agents.rs"
aralik: [37, 70]
dil: rust
ne_yapiyor: "Bir ajanın sorusunu yerel modele gönderir ve cevabı katı JSON olarak geri alır."
neden_boyle: "`format: \"json\"` modelin serbest metin yazmasını engelliyor. Bu ayar olmadan model “İşte analiziniz:” gibi bir giriş cümlesi ekleyip JSON'u bozabiliyor ve ayrıştırma her seferinde hata veriyordu."
kaldirirsak: "Model bazen geçerli JSON, bazen açıklama cümlesiyle sarılmış JSON döndürür. `serde_json::from_str` ikincisinde hata verir ve o ajan yedek yola düşer — yani ajan sessizce körleşir."
notlar:
  - satirlar: [2, 15]
    metin: "İstek gövdesi bir JSON nesnesi. Model adı, sistem promptu, kullanıcı metni ve davranış ayarları burada."
  - satirlar: [6, 6]
    metin: "`stream: false` — cevabı parça parça değil, tek seferde iste. Analiz için akış gerekmiyor."
  - satirlar: [7, 7]
    metin: "`format: \"json\"` bu parçanın en önemli satırı: model artık serbest metin değil, geçerli bir JSON üretmek zorunda."
  - satirlar: [9, 9]
    metin: "`keep_alive: \"30m\"` modeli bellekte sıcak tutuyor. Olmasaydı her analizde model yeniden yüklenir, ilk çağrı çok yavaş olurdu."
  - satirlar: [11, 12]
    metin: "`temperature: 0.2` düşük tutulmuş: analiz işinde yaratıcılık değil tutarlılık isteniyor."
  - satirlar: [30, 32]
    metin: "Modelin döndürdüğü dize burada `AgentAnalysis` yapısına ayrıştırılıyor. Şema tutmazsa hata dönüyor — sessizce yanlış veri geçmiyor."
sina:
  - soru: "`temperature` neden düşük?"
    cevap: "Aynı metne her seferinde benzer sonuç verilmesi isteniyor; yaratıcı çeşitlilik analiz işinde gürültüdür."
  - soru: "`keep_alive` kaldırılırsa ne değişir?"
    cevap: "Model her çağrıdan sonra bellekten düşer; sonraki analizin ilk çağrısı model yükleme süresi kadar gecikir."
owner: TODO
needs_reverify: true
---

Altı ajanın altısı da bu tek fonksiyondan geçiyor. Ajanları birbirinden ayıran şey bu koddaki bir fark değil, fonksiyona verilen sistem promptu.
