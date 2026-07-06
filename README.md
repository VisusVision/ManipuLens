# ManipuLens

<p align="center">
  <a href="https://github.com/fastapi/fastapi" target="_blank">
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Chrome_Extension-4285F4?style=for-the-badge&logo=google-chrome&logoColor=white" alt="Chrome Extension">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Ollama-000000?style=for-the-badge&logo=ollama&logoColor=white" alt="Ollama">
  </a>
</p>

---

<p align="center">
  🌐 <b>Languages:</b> 
  <b>English</b> | <a href="README.tr.md">Türkçe</a>
</p>

---

**ManipuLens** is an advanced, privacy-first browser extension backed by a high-performance **Rust** backend and a **Multi-Agent Large Language Model (LLM)** orchestrator. It seamlessly analyzes web text right from your context menu to detect, dissect, and expose statistical, linguistic, cognitive, and behavioral manipulations in real-time.

### Key Focus Areas:
* 🕵️ **Multi-Agent Verdicts:** Specialised sub-agents (Linguistic, Psychological, Behavioral, Perceptual, Social) dissecting texts concurrently.
* 🛡️ **Privacy-First (Local AI):** Runs entirely on your machine using local LLMs via Ollama—your data never leaves your local network.
* 🚀 **Blazing Fast Performance:** Powered by Rust (Axum/Tokio) for near-instant orchestration and evaluation.

## 🚀 Key Features

* **⚡ High-Performance Rust Orchestrator:** Uses Axum and Tokio asynchronously to trigger and join specialized sub-agent analysis paths in parallel, ensuring sub-second backend response times.
* **🧠 Brain-Chained Multi-Agent Architecture:** Features 5 specialized domain experts overseen by a Sentezör (Manager) Agent, and an advanced *Consumer Intent Agent* to predict cognitive vulnerabilities.
* **🖱️ Context Menu Integration (Seamless UX):** No manual copy-pasting required. Highlight any text on any webpage, right-click, and select "Analyze with ManipuLens" to automatically trigger the flow.
* **🎨 Dynamic Highlight Enjection:** Instead of generic overlays, the extension dynamically changes the DOM `<mark>` style matching the exact color profile of the **dominant manipulation type** found (e.g., Psychological = Magenta, Social = Blue).
* **🐳 Production-Ready Devops:** Built using Docker Multi-Stage builds with static Linux compilation (`x86_64-unknown-linux-musl`) running on ultra-lightweight Alpine containers.

---

## 🎭 The Expert Agent Squad

ManipuLens relies on a structured hierarchy of local generative agents to parse, dissect, and visualize the semantic integrity of the content:

| Agent Profile | Focus Area | Dynamic UI Color |
| :--- | :--- | :--- |
| **Linguistic (Dilsel)** | Wordplay, fallacies, equivocation, and semantic distortions. | `#4cc9f0` (Light Blue) |
| **Psychological (Psikolojik)** | Gaslighting, guilt-tripping, and fear-mongering (Culture of Fear). | `#f72585` (Magenta) |
| **Behavioral (Davranışsal)** | Creating artificial urgency, FOMO, and impulsive action traps. | `#f8961e` (Orange) |
| **Perceptual (Algısal)** | Cherry-picking facts, biased framing, and selective presentation. | `#7209b7` (Purple) |
| **Social (Sosyal)** | Peer pressure, herd mentality, polarization, and tribal biases. | `#4361ee` (Dark Blue) |

---

## 🔄 UI/UX Workflow

1. **Selection & Trigger:** The user selects a text snippet on a webpage. Right-clicking creates a secure transaction via `background.js` using `chrome.storage.local`.
2. **Asynchronous Handshake:** The extension popup auto-opens, instantly locking the UI interaction (`button.disabled = true`) to prevent race conditions.
3. **Rust Multi-Thread Sifting:** The payload hits `http://127.0.0.1:3000/v1/analyze`, sparking the multi-agent consensus network.
4. **Visual Synthesis:** The extension injects contextually colored markers back into the target web page's active DOM and displays a customized consumer behavior prediction card.

## 🗺️ System Architecture

ManipuLens employs a highly optimized asynchronous processing pipeline designed to handle complex multi-agent analysis without blocking the main event loops:


[ Chrome Extension (Frontend) ]
│
▼ (Context Menu Event / storage.local)
[ background.js (Service Worker) ]
│
▼ HTTP POST (Payload: { text: "..." })
┌─────────────────────────────────────────────────────────┐
│ Rust Backend Orchestrator (Axum + Tokio)                │
│                                                         │
│      ┌──► Linguistic Agent (Dilsel Ajan)   ──┐          │
│      ├──► Psychological Agent (Psikolojik) ──┤          │
│  🛸  ├──► Behavioral Agent (Davranışsal)   ──┼─► [Sentezör]
│      ├──► Perceptual Agent (Algısal)       ──┤  (Manager)### The Async Consensus Protocol (Rust Side)
When a request hits `/v1/analyze`, the Rust backend leverages `tokio::spawn` and `tokio::join!` to parallelize the outbound Ollama API requests. Rather than evaluating agents sequentially, all 5 experts evaluate the text matrix concurrently:

1. **Concurrent Evaluation:** The core metrics are collected using highly efficient non-blocking HTTP pooling via `reqwest`.
2. **Synthesis Strategy:** The **Sentezör Agent (Manager)** acts as a reduction layer, aggregating the active flags, compiling the text `target_sentences`, and picking the absolute `dominant_manipulation`.
3. **Neuromarketing Analysis:** The pipeline feeds the compiled context into the **Consumer Intent Agent**, translating linguistic exploitation into actionable consumer behavior risk profiles.

---

### 📦 Unified API Payload Schema

The communication contract between the Rust runtime and the Chrome infrastructure utilizes a strictly typed JSON structure defined in `types.rs`:

```json
{
  "is_manipulated": true,
  "dominant_manipulation": "Psikolojik",
  "genel_sonuc": "Detailed summary explaining the strategic manipulation attempts found in the text matrix...",
  "predicted_product": "VPN service or high-priced privacy tool endorsement injection.",
  "detailed_analyses": [
    {
      "manipulation_type": "Psikolojik",
      "detected": true,
      "confidence_score": 0.85,
      "aciklama": "Gaslighting and threat vector manipulation observed.",
      "target_sentences": [
        "Your data is leaking right now and you don't even care.",
        "Without this layer, your identity remains completely naked."
      ]
    }
  ]
}
│      └──► Social Agent (Sosyal Ajan)       ──┘          │
│                                                         │
│                               ▼                         │
│                  [Consumer Intent Agent]                │
└─────────────────────────────┬───────────────────────────┘
│
▼ JSON Unified Response
[ Dynamic DOM Injections ]


