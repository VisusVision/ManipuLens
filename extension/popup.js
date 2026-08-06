// ============== DİL SİSTEMİ ==============
let currentLang = "tr";
let isAnalyzing = false;

// ============== TEMA (DARK/LIGHT) ==============
let currentTheme = "light";

const EYE_SVG = `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-7 11-7 11 7 11 7-4 7-11 7S1 12 1 12z"/><circle cx="12" cy="12" r="3"/></svg>`;
const EYE_OFF_SVG = `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>`;

function applyTheme() {
  // Güneş/ay ikonları HTML'de sabit; geçiş animasyonunu body.dark
  // üzerinden tamamen CSS yönetir (JS içerik değiştirmez).
  document.body.classList.toggle("dark", currentTheme === "dark");
  const btn = document.getElementById("theme-btn");
  if (btn) {
    const label = currentTheme === "dark"
      ? (currentLang === "en" ? "Switch to light theme" : "Açık temaya geç")
      : (currentLang === "en" ? "Switch to dark theme" : "Koyu temaya geç");
    btn.title = label;
    btn.setAttribute("aria-label", label);
  }
}

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
    },
    // KVKK
    kvkkTitle: "KVKK Onayı",
    kvkkDesc: "ManipuLens'i kullanmadan önce Kullanım Şartları ve Gizlilik Politikası metnini okumanız ve kabul etmeniz gerekmektedir.",
    kvkkLinkText: "Kullanım Şartları ve Gizlilik Politikası",
    kvkkAcceptSuffix: "'nı okudum ve kabul ediyorum.",
    kvkkAcceptBtn: "Kabul Et ve Devam Et",
    kvkkModalTitle: "Kullanım Şartları ve Gizlilik Politikası",
    kvkkFullText: `
      <p><strong>1. TARAFLAR VE HİZMETİN TANIMI</strong><br>
      İşbu "Kullanım Şartları ve Gizlilik Politikası" (bundan böyle "Sözleşme" olarak anılacaktır), ManipuLens eklentisinin geliştiricisi ("Geliştirici") ile eklentiyi indirerek, kurarak veya kullanarak işbu şartları kabul eden gerçek veya tüzel kişi ("Kullanıcı") arasında akdedilmiştir. Eklenti, seçilen metinlerin yapay zeka modelleri aracılığıyla analiz edilmesine olanak tanıyan bir yazılım hizmetidir.</p>
      <p><strong>2. FİKRİ MÜLKİYET HAKLARI</strong><br>
      Eklentinin kaynak kodları, tasarımı, arayüzü, logoları ve eklenti aracılığıyla sunulan tüm içerik, 5846 sayılı Fikir ve Sanat Eserleri Kanunu kapsamında Geliştirici’nin mülkiyetindedir. Kullanıcı, eklentiyi tersine mühendislik işlemine tabi tutmamayı, kaynak kodlarını kopyalamamayı veya ticari amaçlarla çoğaltmamayı kabul ve taahhüt eder.</p>
      <p><strong>3. KULLANIM ŞARTLARI VE KULLANICI SORUMLULUĞU</strong><br>
      <strong>Hizmetin Niteliği:</strong> Eklenti tarafından sağlanan analizler, yalnızca bilgilendirme amaçlıdır. Geliştirici, bu analizlerin %100 doğruluk payına sahip olduğunu veya her türlü manipülasyonu tespit edebileceğini garanti etmez.<br>
      <strong>Yasaklı Kullanım:</strong> Kullanıcı; eklentiyi yasa dışı, ahlaka aykırı veya üçüncü kişilerin haklarını ihlal edecek şekilde kullanamaz. Özellikle; başkasına ait özel verileri, ticari sırları veya kişisel bilgileri eklentiye girerek analiz ettirmek Kullanıcı’nın münhasır sorumluluğundadır.<br>
      <strong>Sistem Bütünlüğü:</strong> Kullanıcı, eklentinin çalışmasını engelleyecek, yavaşlatacak veya sunuculara zarar verecek herhangi bir girişimde bulunmayacaktır.</p>
      <p><strong>4. VERİ GİZLİLİĞİ VE KİŞİSEL VERİLERİN İŞLENMESİ</strong><br>
      <strong>Toplanan Veriler:</strong> İsim, soyisim, e-posta adresi ve analiz edilen metin içerikleri, hizmetin ifası amacıyla sınırlı olarak işlenmektedir.<br>
      <strong>Çerezler ve Takip:</strong> Eklenti, tarayıcı bazlı oturum yönetimi için yerel depolama (local storage) tekniklerini kullanabilir.<br>
      <strong>Üçüncü Taraf İşleyiciler:</strong> Analiz süreçleri, sunucu altyapı hizmeti sağlayıcısı tarafından sağlanmaktadır. Kullanıcı, verilerinin bu sağlayıcıların KVKK uyumlu sunucularında barındırıldığını kabul eder.<br>
      <strong>Veri Güvenliği:</strong> Geliştirici, veri güvenliğini sağlamak için teknik ve idari tüm önlemleri alır. Ancak internet ortamındaki risklerden doğacak veri kayıplarından Geliştirici sorumlu tutulamaz.</p>
      <p><strong>5. SORUMLULUK REDDİ VE GARANTİLERİN SINIRLANDIRILMASI</strong><br>
      Geliştirici, eklentinin "olduğu gibi" sunulduğunu beyan eder. Eklentinin kullanıcının özel amaçlarına uygun olacağı, kesintisiz çalışacağı veya hatalardan tamamen arınmış olacağı yönünde herhangi bir zımni veya sarih garanti verilmemektedir. Kullanıcı, eklentiyi kullanma kararından doğan tüm risklerin kendisine ait olduğunu kabul eder.</p>
      <p><strong>6. HESAP SİLME VE VERİ İMHA POLİTİKASI</strong><br>
      Kullanıcı, hesabını dilediği zaman destek e-posta adresi üzerinden talep ederek silebilir. Hesap silme talebi gerçekleştiğinde, kullanıcının kişisel verileri veritabanından kalıcı olarak silinir; analiz logları ise istatistiksel amaçlarla anonimleştirilerek saklanabilir.</p>
      <p><strong>7. MÜCBİR SEBEPLER VE SÖZLEŞME DEĞİŞİKLİĞİ</strong><br>
      Doğal afetler, altyapı arızaları, siber saldırılar veya yasal düzenlemeler gibi mücbir sebeplerden doğan aksaklıklardan Geliştirici sorumlu değildir. Geliştirici, bu sözleşmeyi dilediği zaman güncelleme hakkını saklı tutar; kullanıcı, eklentiyi kullanmaya devam ederek güncellenmiş şartları kabul etmiş sayılır.</p>
      <p><strong>8. YETKİLİ MAHKEME VE UYUŞMAZLIKLAR</strong><br>
      İşbu sözleşmenin yorumlanması ve uygulanmasından doğan uyuşmazlıklarda İstanbul Mahkemeleri ve İcra Daireleri münhasıran yetkilidir.</p>
    `,
    // Auth
    tabLogin: "Giriş Yap",
    tabRegister: "Kayıt Ol",
    usernamePlaceholder: "E-posta",
    passwordPlaceholder: "Şifre",
    passwordAgainPlaceholder: "Şifre Tekrar",
    loginBtn: "Giriş Yap",
    registerBtn: "Kayıt Ol",
    errEmpty: "Lütfen tüm alanları doldurun.",
    errPasswordMatch: "Şifreler eşleşmiyor.",
    errUserExists: "Bu e-posta adresi zaten kayıtlı.",
    errWrongCredentials: "E-posta veya şifre hatalı.",
    errShortPassword: "Şifre en az 8 karakter olmalı; en az 1 büyük harf, 1 küçük harf ve 1 rakam içermelidir.",
    successRegister: "Kayıt başarılı! Giriş yapabilirsiniz.",
    // Doğrulama + şifre sıfırlama
    forgotLink: "Şifremi unuttum",
    verifyTitle: "E-posta Doğrulama",
    verifyDesc: "{email} adresine gönderilen 6 haneli kodu girin.",
    verifyBtn: "Doğrula",
    resendCode: "Kodu tekrar gönder",
    backToLogin: "Girişe dön",
    forgotTitle: "Şifre Sıfırlama",
    forgotDesc: "Kayıtlı e-posta adresinizi girin, size 6 haneli bir kod gönderelim.",
    sendCodeBtn: "Kod Gönder",
    resetTitle: "Yeni Şifre Belirle",
    resetDesc: "E-postanıza gelen 6 haneli kodu ve yeni şifrenizi girin.",
    resetBtn: "Şifreyi Sıfırla",
    newPasswordPlaceholder: "Yeni Şifre",
    errCodeIncomplete: "Lütfen 6 haneli kodu eksiksiz girin.",
    codeSent: "Kod gönderildi.",
    translatingResult: "Sonuçlar seçilen dile çevriliyor...",
    translateError: "Çeviri yapılamadı. Sunucuya ulaşılamıyor olabilir, lütfen tekrar deneyin.",
    errServerOutdated: "Sunucu bu özelliği desteklemiyor. Backend'i yeniden derleyip başlatın (cargo run)."
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
    },
    // KVKK
    kvkkTitle: "KVKK Consent",
    kvkkDesc: "Before using ManipuLens, you must read and accept the Terms of Use and Privacy Policy.",
    kvkkLinkText: "Terms of Use and Privacy Policy",
    kvkkAcceptSuffix: " – I have read and accept it.",
    kvkkAcceptBtn: "Accept and Continue",
    kvkkModalTitle: "Terms of Use and Privacy Policy",
    kvkkFullText: `
      <p><strong>1. PARTIES AND DEFINITION OF THE SERVICE</strong><br>
      This "Terms of Use and Privacy Policy" is concluded between the developer of the ManipuLens extension ("Developer") and the user who accepts these terms ("User"). The extension enables analysis of selected texts through AI models.</p>
      <p><strong>2. INTELLECTUAL PROPERTY RIGHTS</strong><br>
      All source codes, design, interface and content belong to the Developer under Turkish Law No. 5846. Reverse engineering and commercial copying are prohibited.</p>
      <p><strong>3. TERMS OF USE AND USER RESPONSIBILITY</strong><br>
      Analyses are for informational purposes only. The Developer does not guarantee 100% accuracy. Users are solely responsible for any private or third-party data they submit for analysis.</p>
      <p><strong>4. DATA PRIVACY</strong><br>
      Selected texts and analysis results are processed only for providing the service. Local storage is used for session management. Data is hosted on KVKK-compliant servers.</p>
      <p><strong>5. DISCLAIMER</strong><br>
      The extension is provided "as is". No warranty of uninterrupted or error-free operation is given.</p>
      <p><strong>6. ACCOUNT DELETION</strong><br>
      Users may request account deletion via support email. Personal data will be permanently deleted.</p>
      <p><strong>7. FORCE MAJEURE & CHANGES</strong><br>
      The Developer is not liable for force majeure events and may update this agreement at any time.</p>
      <p><strong>8. JURISDICTION</strong><br>
      Istanbul Courts have exclusive jurisdiction.</p>
    `,
    // Auth
    tabLogin: "Login",
    tabRegister: "Register",
    usernamePlaceholder: "Email",
    passwordPlaceholder: "Password",
    passwordAgainPlaceholder: "Confirm Password",
    loginBtn: "Login",
    registerBtn: "Register",
    errEmpty: "Please fill in all fields.",
    errPasswordMatch: "Passwords do not match.",
    errUserExists: "This email is already registered.",
    errWrongCredentials: "Incorrect email or password.",
    errShortPassword: "Password must be at least 8 characters and include an uppercase letter, a lowercase letter, and a digit.",
    successRegister: "Registration successful! You can now log in.",
    // Verification + password reset
    forgotLink: "Forgot password?",
    verifyTitle: "Email Verification",
    verifyDesc: "Enter the 6-digit code sent to {email}.",
    verifyBtn: "Verify",
    resendCode: "Resend code",
    backToLogin: "Back to login",
    forgotTitle: "Password Reset",
    forgotDesc: "Enter your registered email and we'll send you a 6-digit code.",
    sendCodeBtn: "Send Code",
    resetTitle: "Set New Password",
    resetDesc: "Enter the 6-digit code from your email and your new password.",
    resetBtn: "Reset Password",
    newPasswordPlaceholder: "New Password",
    errCodeIncomplete: "Please enter the full 6-digit code.",
    codeSent: "Code sent.",
    translatingResult: "Translating results into the selected language...",
    translateError: "Translation failed. The server may be unreachable, please try again.",
    errServerOutdated: "The server does not support this feature. Rebuild and restart the backend (cargo run)."
  }
};

function t(key) {
  return i18n[currentLang][key] || key;
}

function translateAgentName(name) {
  return i18n[currentLang].agentNames[name] || name;
}

function applyLanguage() {
  // KRİTİK: CSS text-transform:uppercase, sayfanın lang niteliğine göre çalışır.
  // lang="tr" sabit kalırsa İngilizce arayüzde "Dominant" → "DOMİNANT" olur (Türkçe i→İ kuralı).
  document.documentElement.lang = currentLang;

  document.getElementById("sub-title").innerText = t("subTitle");
  document.getElementById("analyze-btn").innerText = t("analyzeBtn");
  document.getElementById("history-btn").innerText = t("historyBtn");
  // Segmentli TR|EN anahtarı: aktif dili kayan vurgu (thumb) gösterir,
  // metin JS'ten değişmez — yalnızca .en sınıfı yönetilir.
  document.getElementById("lang-btn").classList.toggle("en", currentLang === "en");
  applyTheme(); // tema tuşunun erişilebilirlik etiketi de seçili dile uysun

  // KVKK
  document.getElementById("kvkk-title").innerText = t("kvkkTitle");
  document.getElementById("kvkk-desc").innerText = t("kvkkDesc");
  document.getElementById("kvkk-accept-btn").innerText = t("kvkkAcceptBtn");
  document.getElementById("kvkk-modal-title").innerText = t("kvkkModalTitle");

  const label = document.querySelector(".kvkk-check-row label");
  if (label) {
    label.innerHTML = `<a href="#" id="kvkk-link">${t("kvkkLinkText")}</a>${t("kvkkAcceptSuffix")}`;
    document.getElementById("kvkk-link").addEventListener("click", (e) => {
      e.preventDefault();
      openKvkkModal();
    });
  }

  // Auth
  document.getElementById("tab-login").innerText = t("tabLogin");
  document.getElementById("tab-register").innerText = t("tabRegister");
  document.getElementById("login-username").placeholder = t("usernamePlaceholder");
  document.getElementById("login-password").placeholder = t("passwordPlaceholder");
  document.getElementById("reg-username").placeholder = t("usernamePlaceholder");
  document.getElementById("reg-password").placeholder = t("passwordPlaceholder");
  document.getElementById("reg-password2").placeholder = t("passwordAgainPlaceholder");
  document.getElementById("login-btn").innerText = t("loginBtn");
  document.getElementById("register-btn").innerText = t("registerBtn");

  // Doğrulama + şifre sıfırlama ekranları
  document.getElementById("forgot-link").innerText = t("forgotLink");
  document.getElementById("verify-title").innerText = t("verifyTitle");
  document.getElementById("verify-desc").innerText = t("verifyDesc").replace("{email}", pendingEmail || "");
  document.getElementById("verify-btn").innerText = t("verifyBtn");
  // Geri sayım aktifken resend metnini ezme (sayaç kendi metnini yönetiyor)
  if (!resendTimer) document.getElementById("resend-link").innerText = t("resendCode");
  document.getElementById("verify-back-link").innerText = t("backToLogin");
  document.getElementById("forgot-title").innerText = t("forgotTitle");
  document.getElementById("forgot-desc").innerText = t("forgotDesc");
  document.getElementById("forgot-email").placeholder = t("usernamePlaceholder");
  document.getElementById("forgot-btn").innerText = t("sendCodeBtn");
  document.getElementById("forgot-back-link").innerText = t("backToLogin");
  document.getElementById("reset-title").innerText = t("resetTitle");
  document.getElementById("reset-desc").innerText = t("resetDesc");
  document.getElementById("reset-password").placeholder = t("newPasswordPlaceholder");
  document.getElementById("reset-password2").placeholder = t("passwordAgainPlaceholder");
  document.getElementById("reset-btn").innerText = t("resetBtn");
  document.getElementById("reset-back-link").innerText = t("backToLogin");

  const resultDiv = document.getElementById("result");
  if (resultDiv.innerText.includes("metin seçin") || resultDiv.innerText.includes("Select a text")) {
    resultDiv.innerText = t("selectText");
  }
}

// ============== KVKK ==============
function openKvkkModal() {
  document.getElementById("kvkk-modal-body").innerHTML = t("kvkkFullText");
  document.getElementById("kvkk-modal").classList.add("show");
}

function closeKvkkModal() {
  document.getElementById("kvkk-modal").classList.remove("show");
}

async function checkKvkkAccepted() {
  const data = await chrome.storage.local.get("kvkkAccepted");
  return !!data.kvkkAccepted;
}

async function acceptKvkk() {
  await chrome.storage.local.set({ kvkkAccepted: true });
  document.getElementById("kvkk-overlay").style.display = "none";
  // KVKK sonrası auth ekranını göster
  showAuthScreen();
}

// ============== AUTH ==============
// NOT: Eskiden e-posta/şifre çiftleri chrome.storage.local içinde düz metin
// olarak saklanıyordu ve backend'deki bcrypt tabanlı /v1/register ve
// /v1/login uçları hiç kullanılmıyordu (ölü kod + güvenlik açığı: şifreler
// tarayıcıda düz metin duruyordu). Artık gerçek kimlik doğrulaması backend
// üzerinden yapılıyor; istemcide şifre saklanmıyor.
function showAuthScreen() {
  document.getElementById("auth-overlay").style.display = "flex";
}

function hideAuthScreen() {
  document.getElementById("auth-overlay").style.display = "none";
}

async function checkIsLoggedIn() {
  // Token da zorunlu: eski sürümden kalan token'sız oturumlar geçersizdir
  // (backend artık tüm korumalı uçlarda Bearer token istiyor).
  const data = await chrome.storage.local.get(["isLoggedIn", "authToken"]);
  return !!data.isLoggedIn && !!data.authToken;
}

async function getAuthToken() {
  const data = await chrome.storage.local.get("authToken");
  return data.authToken || null;
}

// 401 dönen her korumalı istekte çağrılır: oturum düşmüştür, girişe dön
async function handleSessionExpired() {
  await chrome.storage.local.set({ isLoggedIn: false, authToken: null });
  showAuthScreen();
  showAuthView("login");
}

// PERFORMANS: server_config.json her istekte GitHub'dan (cache-bust + no-store)
// indiriliyordu; geçmiş/çeviri/auth çağrılarının en yavaş kısmı buydu.
// Artık URL 5 dk bellek + chrome.storage önbelleğinde tutulur; popup her
// açılışta yeniden indirmez. ngrok adresi değişirse forceRefresh ile tazelenir.
let cachedBaseUrl = null;
let cachedBaseUrlAt = 0;
const BASE_URL_TTL = 5 * 60 * 1000;

async function getBaseUrl(forceRefresh = false) {
  const now = Date.now();

  if (!forceRefresh) {
    if (cachedBaseUrl && now - cachedBaseUrlAt < BASE_URL_TTL) return cachedBaseUrl;

    const stored = await chrome.storage.local.get(["cachedBaseUrl", "cachedBaseUrlAt"]);
    if (stored.cachedBaseUrl && now - (stored.cachedBaseUrlAt || 0) < BASE_URL_TTL) {
      cachedBaseUrl = stored.cachedBaseUrl;
      cachedBaseUrlAt = stored.cachedBaseUrlAt;
      return cachedBaseUrl;
    }
  }

  const configResponse = await fetch(
    `https://raw.githubusercontent.com/VisusVision/ManipuLens/feature/remote-backend-sync/extension/server_config.json?t=${now}`,
    { cache: "no-store" }
  );
  const configData = await configResponse.json();
  cachedBaseUrl = configData.ngrok_url;
  cachedBaseUrlAt = now;
  await chrome.storage.local.set({ cachedBaseUrl, cachedBaseUrlAt });
  return cachedBaseUrl;
}

// Auth API çağrılarına arayüz dili otomatik eklenir (mesajlar seçili dilde gelir).
// Sağlamlaştırma: 15 sn timeout + JSON olmayan yanıtlarda (ör. 404 = eski backend)
// anlaşılır hata fırlatılır.
async function authApi(path, body) {
  const baseUrl = await getBaseUrl();
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), 15000);

  try {
    const response = await fetch(`${baseUrl}${path}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "ngrok-skip-browser-warning": "true"
      },
      body: JSON.stringify({ ...body, lang: currentLang }),
      signal: controller.signal
    });

    const text = await response.text();
    try {
      return JSON.parse(text);
    } catch (parseError) {
      // Backend eski sürümse yeni endpoint 404 döner ve JSON gelmez
      throw new Error(`HTTP ${response.status}`);
    }
  } finally {
    clearTimeout(timer);
  }
}

// API hatasını kullanıcı dostu mesaja çevir
function apiErrorMessage(e) {
  if (e && /HTTP 404/.test(e.message || "")) return t("errServerOutdated");
  return t("connectionError");
}

// ============== ÇEVİRİ (/v1/translate-report) ==============
// REGRESYON DÜZELTMESİ: baseUrl önbelleği eklendikten sonra ngrok adresi
// değiştiğinde çeviri isteği sessizce başarısız olup eski (çevrilmemiş)
// görünümü geri koyuyordu — "AI çıktıları aynı kalıyor" bunun sonucuydu.
// Artık: 60 sn timeout (Ollama çevirisi uzun sürebilir) + ilk deneme
// başarısızsa config önbelleği yenilenip BİR kez daha denenir + yine de
// olmazsa kullanıcıya görünür bir hata mesajı gösterilir.
async function requestTranslationOnce(report, lang, forceBaseRefresh = false) {
  const baseUrl = await getBaseUrl(forceBaseRefresh);
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), 60000);

  try {
    const token = await getAuthToken();
    const response = await fetch(`${baseUrl}/v1/translate-report`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "ngrok-skip-browser-warning": "true",
        ...(token ? { "Authorization": `Bearer ${token}` } : {})
      },
      body: JSON.stringify({ report, lang }),
      signal: controller.signal
    });
    if (response.status === 401) {
      await handleSessionExpired();
      throw new Error("HTTP 401");
    }
    if (!response.ok) throw new Error("HTTP " + response.status);
    return await response.json();
  } finally {
    clearTimeout(timer);
  }
}

async function requestTranslation(report, lang) {
  try {
    return await requestTranslationOnce(report, lang);
  } catch (firstError) {
    console.warn("Çeviri ilk deneme başarısız, config yenilenip tekrar deneniyor:", firstError);
    return await requestTranslationOnce(report, lang, true);
  }
}

// ============== DİL TUTARLI SONUÇ GÖSTERİMİ ==============
// BUG FİXİ: Dil değiştirdikten sonra Geçmiş ekranından sonuç ekranına
// dönünce (veya popup yeniden açılınca) sonuç, çevirisi önbellekte yoksa
// ORİJİNAL dilde basılıyordu — "arayüz değişiyor ama sonuç eski dilde
// kalıyor" sorununun kaynağı buydu. Artık TÜM sonuç gösterimleri bu ortak
// fonksiyondan geçer: dil uyuşmuyorsa önbelleğe bakılır, yoksa çeviri
// istenir; çeviri başarısızsa orijinal gösterilir + görünür hata verilir.
let translationInFlight = false;

async function showResultInCurrentLang(data) {
  if (!data || !data.lastAnalysisResult) return;

  // Sonuç zaten arayüz dilinde üretilmiş → direkt göster
  if (!data.analysisLang || data.analysisLang === currentLang) {
    renderResult(data.lastAnalysisResult);
    return;
  }

  // Bu dile daha önce çevrilmiş → önbellekten göster (ağa çıkma)
  const cache = data.translatedResults || {};
  if (cache[currentLang]) {
    renderResult(cache[currentLang]);
    return;
  }

  // Çeviri gerekiyor. Aynı anda ikinci çeviri başlatma (ör. storage.onChanged
  // ile checkAndShowLastResult üst üste tetiklenirse).
  if (translationInFlight) return;
  translationInFlight = true;

  const resultDiv = document.getElementById("result");
  resultDiv.innerHTML = `<p style="text-align:center; color:var(--muted);">${t("translatingResult")}</p>`;

  try {
    const translated = await requestTranslation(data.lastAnalysisResult, currentLang);
    cache[currentLang] = translated;
    await chrome.storage.local.set({ translatedResults: cache });
    renderResult(translated);
  } catch (e) {
    console.error("Çeviri hatası:", e);
    // Çeviri yapılamadıysa orijinal dilde göster + görünür hata bildir
    // (boş/yarım ekran bırakmak yerine en azından içerik kalır)
    renderResult(data.lastAnalysisResult);
    showTransientError(t("translateError"));
  } finally {
    translationInFlight = false;
  }
}

// Sonuç alanının üstünde 4 sn görünen, kaybolan hata bildirimi
function showTransientError(message) {
  const resultDiv = document.getElementById("result");
  if (!resultDiv) return;
  const note = document.createElement("p");
  note.style.cssText =
    "text-align:center; color:var(--danger); font-size:12px; font-weight:600; margin:0 0 8px 0;";
  note.textContent = message;
  resultDiv.prepend(note);
  setTimeout(() => note.remove(), 4000);
}

// ============== GEÇMİŞ (fetch + render) ==============
// Geçmiş 60 sn önbelleklenir: butona tekrar basmak veya dil değiştirdikten
// sonra açmak ağa çıkmadan anında render edilir.
const HISTORY_CACHE_TTL = 60 * 1000;

async function fetchHistory(clientId, forceBaseRefresh = false) {
  const baseUrl = await getBaseUrl(forceBaseRefresh);
  const controller = new AbortController();
  // 60 sn: dil değişiminden sonraki İLK geçmiş açılışında backend eski
  // kayıtları Ollama ile çevirebilir (yavaş olabilir). Çeviri artık
  // veritabanına kalıcı yazıldığı için sonraki açılışlar anında döner.
  const timer = setTimeout(() => controller.abort(), 60000);

  try {
    // Kimlik artık oturum token'ından: backend geçmişi token sahibinin
    // hesabıyla filtreler (client_id parametresi kaldırıldı — IDOR kapandı).
    // lang: kayıtlar farklı dilde üretilmişse backend özetleri arayüz diline çevirir.
    const token = await getAuthToken();
    const response = await fetch(`${baseUrl}/v1/history?lang=${currentLang}`, {
      headers: {
        "ngrok-skip-browser-warning": "true",
        ...(token ? { "Authorization": `Bearer ${token}` } : {})
      },
      signal: controller.signal
    });
    if (response.status === 401) {
      await handleSessionExpired();
      throw new Error("HTTP 401");
    }
    if (!response.ok) throw new Error("HTTP " + response.status);
    return await response.json();
  } finally {
    clearTimeout(timer);
  }
}

function renderHistoryList(history) {
  const resultDiv = document.getElementById("result");
  if (!resultDiv) return;

  if (!history || history.length === 0) {
    resultDiv.innerHTML = `<p style="text-align:center; color:var(--muted);">${t("noHistory")}</p>`;
    return;
  }

  let html = `<h4 style="margin-top:0;" id="history-heading">${t("historyTitle")}</h4>`;

  history.forEach((item) => {
    const date = new Date(item.timestamp).toLocaleString(currentLang === "tr" ? "tr-TR" : "en-US");
    const typeClass = getAgentClass(item.dominant_manipulation);
    const borderColor = getProgressBarColor(item.dominant_manipulation);

    const badgeColor = item.is_manipulated ? borderColor : "var(--safe)";
    const badgeText = item.is_manipulated ? t("manipulation") : t("clean");

    html += `
      <div class="card ${typeClass}" style="margin-bottom:12px; padding:10px; border-left-color: ${borderColor};">
        <div style="font-size:11px; color:var(--muted2); margin-bottom:6px;">${date}</div>
        <span class="badge" style="background-color: ${badgeColor}; margin-bottom:8px;">${badgeText}</span>
        <p style="margin:6px 0 4px 0; font-size:13px; font-weight:700; color:${borderColor};">
          ${escapeHTML(translateAgentName(item.dominant_manipulation) || "-")}
        </p>
        <p style="margin:0 0 6px 0; font-size:12px; color:var(--text-soft);">
          ${escapeHTML(item.genel_sonuc || "")}
        </p>
        <p style="margin:0; font-size:11px; color:var(--muted2); font-style:italic;">
          "${escapeHTML(item.text_preview)}..."
        </p>
      </div>
    `;
  });

  resultDiv.innerHTML = html;
}

// Doğrulama/sıfırlama akışlarında bekleyen e-posta
let pendingEmail = "";
let resetEmail = "";

async function doLogin(email, password) {
  try {
    const data = await authApi("/v1/login", { email, password });

    if (data.success) {
      await chrome.storage.local.set({
        isLoggedIn: true,
        currentUser: data.email,
        client_id: data.client_id,
        authToken: data.token || null
      });
      return { success: true };
    }

    return {
      success: false,
      needsVerification: !!data.needs_verification,
      error: data.message || t("errWrongCredentials")
    };
  } catch (error) {
    console.error("Login hatası:", error);
    return { success: false, error: apiErrorMessage(error) };
  }
}

async function doRegister(email, password) {
  try {
    const data = await authApi("/v1/register", { email, password });
    if (!data.success) {
      return { success: false, error: data.message || t("errUserExists") };
    }
    return {
      success: true,
      needsVerification: !!data.needs_verification,
      message: data.message
    };
  } catch (error) {
    console.error("Register hatası:", error);
    return { success: false, error: apiErrorMessage(error) };
  }
}

// ============== AUTH EKRAN YÖNETİMİ ==============
function showAuthView(view) {
  const forms = ["login-form", "register-form", "verify-form", "forgot-form", "reset-form"];
  forms.forEach((f) => {
    document.getElementById(f).style.display = "none";
  });
  ["login-error", "register-error", "verify-error", "forgot-error", "reset-error"].forEach((id) => {
    const e = document.getElementById(id);
    if (e) {
      e.innerText = "";
      e.className = "auth-error";
    }
  });

  const tabs = document.querySelector(".auth-tabs");
  const isTabbed = view === "login" || view === "register";
  tabs.style.display = isTabbed ? "flex" : "none";
  document.getElementById(view + "-form").style.display = "block";

  if (isTabbed) {
    document.getElementById("tab-login").classList.toggle("active", view === "login");
    document.getElementById("tab-register").classList.toggle("active", view === "register");
  }

  if (view === "verify") {
    document.getElementById("verify-desc").innerText =
      t("verifyDesc").replace("{email}", pendingEmail || "");
    clearCode("verify-codes");
    const first = document.querySelector("#verify-codes .code-box");
    if (first) first.focus();
    // Ekrana girildiğinde kod zaten yeni gönderilmiş olur → geri sayımı başlat
    startResendCooldown(30);
  }
  if (view === "reset") {
    clearCode("reset-codes");
    document.getElementById("reset-password").value = "";
    document.getElementById("reset-password2").value = "";
  }
}

// ============== 6 HANELİ KOD KUTUCUKLARI ==============
function setupCodeBoxes(containerId) {
  const boxes = Array.from(document.querySelectorAll(`#${containerId} .code-box`));
  boxes.forEach((box, i) => {
    box.addEventListener("input", () => {
      box.value = box.value.replace(/\D/g, "").slice(0, 1);
      if (box.value && i < boxes.length - 1) boxes[i + 1].focus();
    });
    box.addEventListener("keydown", (e) => {
      if (e.key === "Backspace" && !box.value && i > 0) boxes[i - 1].focus();
    });
    box.addEventListener("paste", (e) => {
      e.preventDefault();
      const digits = (e.clipboardData.getData("text") || "").replace(/\D/g, "").slice(0, boxes.length);
      digits.split("").forEach((d, j) => {
        if (boxes[j]) boxes[j].value = d;
      });
      const next = Math.min(digits.length, boxes.length - 1);
      boxes[next].focus();
    });
  });
}

function getCode(containerId) {
  return Array.from(document.querySelectorAll(`#${containerId} .code-box`))
    .map((b) => b.value)
    .join("");
}

function clearCode(containerId) {
  document.querySelectorAll(`#${containerId} .code-box`).forEach((b) => {
    b.value = "";
  });
}

function isStrongPassword(password) {
  return (
    password.length >= 8 &&
    /[A-Z]/.test(password) &&
    /[a-z]/.test(password) &&
    /[0-9]/.test(password)
  );
}

// ============== RESEND GERİ SAYIMI (30 sn) ==============
let resendTimer = null;

function startResendCooldown(secs = 30) {
  const link = document.getElementById("resend-link");
  if (!link) return;

  let remaining = secs;
  const base = t("resendCode");
  link.classList.add("disabled");
  link.innerText = `${base} (${remaining})`;

  if (resendTimer) clearInterval(resendTimer);
  resendTimer = setInterval(() => {
    remaining--;
    if (remaining <= 0) {
      clearInterval(resendTimer);
      resendTimer = null;
      link.classList.remove("disabled");
      link.innerText = base;
    } else {
      link.innerText = `${base} (${remaining})`;
    }
  }, 1000);
}

// Pencereyi açıldığı ekranın çalışma alanına sığdır. Service worker'da
// screen objesi olmadığı için clamp burada, popup bağlamında yapılır.
async function fitWindowToDisplay() {
  try {
    const win = await chrome.windows.getCurrent();
    // Kullanıcının maximize ettiği pencereyle savaşma
    if (win.type !== "popup" || win.state !== "normal") return;

    const availW = screen.availWidth;
    const availH = screen.availHeight;
    const update = {};

    const targetW = Math.max(320, Math.min(win.width, availW));
    const targetH = Math.min(660, Math.max(400, availH - 20));
    if (targetW !== win.width) update.width = targetW;
    if (targetH !== win.height) update.height = targetH;

    // Alt/sağ kenar ekrandan taşıyorsa içeri çek
    const w = update.width ?? win.width;
    const h = update.height ?? win.height;
    if (win.top + h > availH) update.top = Math.max(0, availH - h);
    if (win.left + w > availW) update.left = Math.max(0, availW - w);

    if (Object.keys(update).length > 0) {
      await chrome.windows.update(win.id, update);
    }
  } catch (e) {
    // Boyutlandırma başarısız olsa da popup çalışmaya devam etsin
    console.warn("fitWindowToDisplay:", e);
  }
}

// ============== ANA KOD ==============
document.addEventListener("DOMContentLoaded", async () => {
  fitWindowToDisplay();

  const stored = await chrome.storage.local.get(["language", "theme"]);
  currentLang = stored.language || "tr";
  currentTheme = stored.theme === "dark" ? "dark" : "light";
  applyLanguage();
  applyTheme();

  // Tema tuşu
  document.getElementById("theme-btn").addEventListener("click", async () => {
    currentTheme = currentTheme === "dark" ? "light" : "dark";
    await chrome.storage.local.set({ theme: currentTheme });
    applyTheme();
  });

  // Şifre göster/gizle (göz) tuşları
  document.querySelectorAll(".pw-eye").forEach((btn) => {
    btn.innerHTML = EYE_SVG;
    btn.addEventListener("click", () => {
      const input = document.getElementById(btn.dataset.target);
      if (!input) return;
      const show = input.type === "password";
      input.type = show ? "text" : "password";
      btn.innerHTML = show ? EYE_OFF_SVG : EYE_SVG;
    });
  });

  // Ekran kontrolü
  const kvkkOk = await checkKvkkAccepted();
  const loggedIn = await checkIsLoggedIn();

  if (!kvkkOk) {
    // KVKK göster
  } else if (!loggedIn) {
    document.getElementById("kvkk-overlay").style.display = "none";
    showAuthScreen();
  } else {
    document.getElementById("kvkk-overlay").style.display = "none";
    hideAuthScreen();
  }

  // KVKK checkbox
  const checkbox = document.getElementById("kvkk-checkbox");
  const acceptBtn = document.getElementById("kvkk-accept-btn");

  checkbox.addEventListener("change", () => {
    if (checkbox.checked) {
      acceptBtn.classList.add("enabled");
    } else {
      acceptBtn.classList.remove("enabled");
    }
  });

  acceptBtn.addEventListener("click", async () => {
    if (checkbox.checked) {
      await acceptKvkk();
    }
  });

  // KVKK link + modal
  document.getElementById("kvkk-link").addEventListener("click", (e) => {
    e.preventDefault();
    openKvkkModal();
  });
  document.getElementById("kvkk-modal-close").addEventListener("click", closeKvkkModal);
  document.getElementById("kvkk-modal").addEventListener("click", (e) => {
    if (e.target.id === "kvkk-modal") closeKvkkModal();
  });

  // Auth tab switching
  document.getElementById("tab-login").addEventListener("click", () => showAuthView("login"));
  document.getElementById("tab-register").addEventListener("click", () => showAuthView("register"));

  // Kod kutucuklarını hazırla
  setupCodeBoxes("verify-codes");
  setupCodeBoxes("reset-codes");

  // Login
  document.getElementById("login-btn").addEventListener("click", async () => {
    const username = document.getElementById("login-username").value.trim().toLowerCase();
    const password = document.getElementById("login-password").value;

    const errEl = document.getElementById("login-error");
    errEl.innerText = "";

    if (!username || !password) {
      errEl.innerText = t("errEmpty");
      return;
    }

    const result = await doLogin(username, password);
    if (result.success) {
      hideAuthScreen();
    } else if (result.needsVerification) {
      // Şifre doğru ama e-posta doğrulanmamış → doğrulama ekranına geç
      pendingEmail = username;
      showAuthView("verify");
      const vErr = document.getElementById("verify-error");
      vErr.className = "auth-success";
      vErr.innerText = result.error;
    } else {
      errEl.innerText = result.error;
    }
  });

  // Register
  document.getElementById("register-btn").addEventListener("click", async () => {
    const username = document.getElementById("reg-username").value.trim().toLowerCase();
    const password = document.getElementById("reg-password").value;
    const password2 = document.getElementById("reg-password2").value;

    const errEl = document.getElementById("register-error");
    errEl.innerText = "";
    errEl.className = "auth-error";

    if (!username || !password || !password2) {
      errEl.innerText = t("errEmpty");
      return;
    }

    if (!isStrongPassword(password)) {
      errEl.innerText = t("errShortPassword");
      return;
    }

    if (password !== password2) {
      errEl.innerText = t("errPasswordMatch");
      return;
    }

    const result = await doRegister(username, password);
    if (result.success) {
      // Kayıt sonrası e-posta doğrulama ekranına geç
      pendingEmail = username;
      showAuthView("verify");
      const vErr = document.getElementById("verify-error");
      vErr.className = "auth-success";
      vErr.innerText = result.message || t("codeSent");
    } else {
      errEl.innerText = result.error;
    }
  });

  // Doğrula
  document.getElementById("verify-btn").addEventListener("click", async () => {
    const errEl = document.getElementById("verify-error");
    errEl.className = "auth-error";
    errEl.innerText = "";

    if (!pendingEmail) {
      showAuthView("login");
      return;
    }

    const code = getCode("verify-codes");
    if (code.length !== 6) {
      errEl.innerText = t("errCodeIncomplete");
      return;
    }

    try {
      const data = await authApi("/v1/verify", { email: pendingEmail, code });
      if (data.success) {
        // Doğrulama başarılı → otomatik giriş (token dahil)
        await chrome.storage.local.set({
          isLoggedIn: true,
          currentUser: data.email,
          client_id: data.client_id,
          authToken: data.token || null
        });
        hideAuthScreen();
      } else {
        errEl.innerText = data.message;
        clearCode("verify-codes");
      }
    } catch (e) {
      errEl.innerText = apiErrorMessage(e);
    }
  });

  // Kodu tekrar gönder (30 sn geri sayım korumalı)
  document.getElementById("resend-link").addEventListener("click", async () => {
    const link = document.getElementById("resend-link");
    if (link.classList.contains("disabled")) return;

    const errEl = document.getElementById("verify-error");
    if (!pendingEmail) return;

    try {
      const data = await authApi("/v1/resend", { email: pendingEmail });
      errEl.className = data.success ? "auth-success" : "auth-error";
      errEl.innerText = data.message || t("codeSent");
      if (data.success) startResendCooldown(30);
    } catch (e) {
      errEl.className = "auth-error";
      errEl.innerText = apiErrorMessage(e);
    }
  });

  // Şifremi unuttum
  document.getElementById("forgot-link").addEventListener("click", () => showAuthView("forgot"));

  document.getElementById("forgot-btn").addEventListener("click", async () => {
    const email = document.getElementById("forgot-email").value.trim().toLowerCase();
    const errEl = document.getElementById("forgot-error");
    errEl.className = "auth-error";
    errEl.innerText = "";

    if (!email) {
      errEl.innerText = t("errEmpty");
      return;
    }

    try {
      const data = await authApi("/v1/forgot", { email });
      resetEmail = email;
      showAuthView("reset");
      const rErr = document.getElementById("reset-error");
      rErr.className = "auth-success";
      rErr.innerText = data.message || t("codeSent");
    } catch (e) {
      errEl.innerText = apiErrorMessage(e);
    }
  });

  // Yeni şifre belirle
  document.getElementById("reset-btn").addEventListener("click", async () => {
    const errEl = document.getElementById("reset-error");
    errEl.className = "auth-error";
    errEl.innerText = "";

    if (!resetEmail) {
      showAuthView("forgot");
      return;
    }

    const code = getCode("reset-codes");
    const password = document.getElementById("reset-password").value;
    const password2 = document.getElementById("reset-password2").value;

    if (code.length !== 6) {
      errEl.innerText = t("errCodeIncomplete");
      return;
    }
    if (!password || !password2) {
      errEl.innerText = t("errEmpty");
      return;
    }
    if (!isStrongPassword(password)) {
      errEl.innerText = t("errShortPassword");
      return;
    }
    if (password !== password2) {
      errEl.innerText = t("errPasswordMatch");
      return;
    }

    try {
      const data = await authApi("/v1/reset", { email: resetEmail, code, new_password: password });
      if (data.success) {
        showAuthView("login");
        const lErr = document.getElementById("login-error");
        lErr.className = "auth-success";
        lErr.innerText = data.message;
        document.getElementById("login-username").value = resetEmail;
      } else {
        errEl.innerText = data.message;
      }
    } catch (e) {
      errEl.innerText = apiErrorMessage(e);
    }
  });

  // Geri dön linkleri
  document.getElementById("verify-back-link").addEventListener("click", () => showAuthView("login"));
  document.getElementById("forgot-back-link").addEventListener("click", () => showAuthView("login"));
  document.getElementById("reset-back-link").addEventListener("click", () => showAuthView("login"));

  // Dil değiştirme
  document.getElementById("lang-btn").addEventListener("click", async () => {
    if (isAnalyzing) return;

    currentLang = currentLang === "tr" ? "en" : "tr";
    await chrome.storage.local.set({ language: currentLang });
    applyLanguage();

    // Ekranda GEÇMİŞ açıksa: yeni dilde yeniden yükle (backend özetleri
    // arayüz diline çevirerek döndürür; karışık dil sorunu böyle çözülür)
    if (document.getElementById("history-heading")) {
      document.getElementById("history-btn").click();
      return;
    }

    const data = await chrome.storage.local.get([
      "analysisStatus",
      "lastAnalysisResult",
      "analysisLang",
      "translatedResults"
    ]);
    if (data.analysisStatus !== "done" || !data.lastAnalysisResult) return;

    // Ortak dil-tutarlı gösterim: önbellek varsa anında, yoksa tek çeviri.
    // Çeviri sırasında dil butonu kilitlenir (üst üste istek olmasın).
    const langBtn = document.getElementById("lang-btn");
    langBtn.style.opacity = "0.4";
    langBtn.style.pointerEvents = "none";
    try {
      await showResultInCurrentLang(data);
    } finally {
      langBtn.style.opacity = "1";
      langBtn.style.pointerEvents = "auto";
    }
  });

  // Analiz butonu
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
            // Dil/çeviri farkındalıklı ortak render yolunu kullan
            checkAndShowLastResult();
          } else {
            document.getElementById("result").innerText = t("selectTextError");
          }
          return;
        }

        isAnalyzing = true;
        document.getElementById("lang-btn").style.opacity = "0.4";
        document.getElementById("lang-btn").style.pointerEvents = "none";

        // Progress bar geçen süreye göre dolduğu için başlangıç anını kaydet
        await chrome.storage.local.set({ analysisStartTime: Date.now() });

        chrome.runtime.sendMessage({
          action: "startAnalysis",
          text: selectedText,
          tabId: tab.id
        });

        startLoadingAnimation();
      });
    });
  }

  // Geçmiş butonu
  const historyBtn = document.getElementById("history-btn");
  if (historyBtn) {
    historyBtn.addEventListener("click", async () => {
      const resultDiv = document.getElementById("result");

      // background.js ile aynı kimlik mantığı: giriş yapılmışsa e-posta, yoksa client_id
      const idData = await chrome.storage.local.get(["client_id", "currentUser", "historyCache"]);
      const clientId = idData.currentUser || idData.client_id || "anonim";

      // Taze önbellek varsa ANINDA göster. Önbellek DİLE ÖZGÜdür: içerik artık
      // backend'de arayüz diline çevrildiği için TR önbelleği EN arayüzde
      // kullanılamaz — dil değişince yeni dilde taze veri çekilir.
      const cache = idData.historyCache;
      const cacheFresh =
        cache &&
        cache.clientId === clientId &&
        cache.lang === currentLang &&
        Date.now() - cache.timestamp < HISTORY_CACHE_TTL;

      if (cacheFresh) {
        renderHistoryList(cache.items);
        return;
      }

      resultDiv.innerHTML = `<p style="text-align:center; color:var(--muted);">${t("loadingHistory")}</p>`;

      try {
        let history;
        try {
          history = await fetchHistory(clientId);
        } catch (firstError) {
          // ngrok adresi değişmiş olabilir: config önbelleğini yenileyip bir kez daha dene
          console.warn("Geçmiş ilk deneme başarısız, config yenilenip tekrar deneniyor:", firstError);
          history = await fetchHistory(clientId, true);
        }

        await chrome.storage.local.set({
          historyCache: { clientId, lang: currentLang, timestamp: Date.now(), items: history }
        });

        renderHistoryList(history);
      } catch (error) {
        console.error(error);
        resultDiv.innerHTML = `<p style="text-align:center; color:var(--danger);">${t("historyError")}</p>`;
      }
    });
  }

  checkAndShowLastResult();

  chrome.storage.onChanged.addListener((changes) => {
    // Yeni analiz tamamlandığında geçmiş önbelleği bayatlar: temizle ki
    // "Geçmişim" bir sonraki açılışta yeni kaydı da göstersin.
    if (changes.lastAnalysisResult) {
      chrome.storage.local.remove("historyCache");
    }
    if (changes.analysisStatus || changes.lastAnalysisResult || changes.analysisError) {
      checkAndShowLastResult();
    }
  });
});

let loadingInterval = null;

// YÜKLEME EKRANI:
// Bar bir yüzde göstermez; tamamen CSS ile çalışan, üzerinden sürekli ışık
// süpürmesi geçen dekoratif bir animasyondur (progressGlare/progressShimmer).
// JS yalnızca aşama METNİNİ yönetir: analiz başlangıç zamanına göre sırayla
// ilerler, SON aşamada sabit kalır (eski modulo döngüsü başa sarıyordu).
// Popup kapatılıp açılsa bile aşama kaldığı yerden devam eder.
const LOADING_STAGE_MS = 7000; // her aşama ~7 sn (9 aşama ≈ 60 sn'yi kapsar)

async function startLoadingAnimation() {
  if (loadingInterval) clearInterval(loadingInterval);

  const stored = await chrome.storage.local.get("analysisStartTime");
  const startTime = stored.analysisStartTime || Date.now();

  const tick = () => {
    const stages = t("loadingStages");
    const elapsed = Date.now() - startTime;

    const stageIndex = Math.min(
      Math.floor(elapsed / LOADING_STAGE_MS),
      stages.length - 1
    );
    updateLoadingUI(stages[stageIndex]);
  };

  tick();
  loadingInterval = setInterval(tick, 1000);
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

  // BUG FİXİ: Eskiden her aşama değişiminde tüm yükleme ekranı yeniden
  // kuruluyordu; bu, progress bar animasyonunu (2.2s döngü) her 2.4s'de
  // sıfırlayıp barın zıplamasına neden oluyordu. Artık ekran bir kez
  // kurulur, sonraki çağrılarda yalnızca metin güncellenir.
  const existingText = document.getElementById("loading-stage-text");
  if (existingText) {
    existingText.textContent = text;
    return;
  }

  resultDiv.innerHTML = `
    <div class="loading-container">
      <div class="scan-wrap">
        <span class="scan-halo"></span>
        <span class="scan-ring"></span>
        <span class="scan-core">
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="10.5" cy="10.5" r="6.5" stroke="url(#scanGrad)" stroke-width="2.2"/>
            <path d="M19.5 19.5L15.2 15.2" stroke="url(#scanGrad)" stroke-width="2.2" stroke-linecap="round"/>
            <defs>
              <linearGradient id="scanGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stop-color="#4361ee"/>
                <stop offset="50%" stop-color="#7209b7"/>
                <stop offset="100%" stop-color="#f72585"/>
              </linearGradient>
            </defs>
          </svg>
        </span>
      </div>
      <div class="loading-text" id="loading-stage-text">${text}</div>
      <div class="progress-track">
        <div class="progress-fill" id="loading-progress-fill"></div>
      </div>
    </div>`;
}

async function checkAndShowLastResult() {
  const resultDiv = document.getElementById("result");
  if (!resultDiv) return;

  const data = await chrome.storage.local.get([
    "analysisStatus",
    "lastAnalysisResult",
    "analysisError",
    "analysisLang",
    "translatedResults"
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
      <p style="color: var(--danger); font-weight: bold;">${t("connectionError")}</p>
      <small style="color: var(--muted);">${t("technicalDetail")}: ${escapeHTML(data.analysisError || "Unknown error")}</small>
    `;
    return;
  }

  if (data.analysisStatus === "done" && data.lastAnalysisResult) {
    // Dil-tutarlı ortak gösterim: arayüz dili analiz dilinden farklıysa
    // önbellekten gösterir, önbellek yoksa ÇEVİRİ İSTER. (Eski davranış:
    // önbellek yoksa sessizce orijinal dilde basıyordu — geçmişten sonuca
    // dönüşte "dil bozuluyor" sorununun kaynağı.)
    await showResultInCurrentLang(data);
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
    const barColor = agent.detected ? getProgressBarColor(agent.manipulation_type) : "var(--disabled)";
    const delay = index * 0.08;

    const titleStyle = agent.detected ? `background:${barColor}` : `background:var(--disabled)`;
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

// Tema bazlı CSS değişkenleri döner: dark modda otomatik olarak daha parlak,
// okunabilir varyantlar kullanılır (ör. koyu mor #7209b7 koyu zeminde okunmuyordu).
function getProgressBarColor(type) {
  const map = {
    "Dilsel": "var(--c-linguistic)",
    "Psikolojik": "var(--c-psychological)",
    "Davranışsal": "var(--c-behavioral)",
    "Algısal": "var(--c-perceptual)",
    "Sosyal": "var(--c-social)",
    "Pazarlama": "var(--c-marketing)",
    "Tüketici Manipülasyonu": "var(--c-marketing)"
  };
  return map[type] || "var(--c-social)";
}

function getConfidenceLabel(score) {
  const percentage = score * 100;
  if (percentage >= 75) return t("upperLevel");
  if (percentage >= 40) return t("midLevel");
  return t("lowLevel");
}