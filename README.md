# ManipuLens

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Chrome_Extension-4285F4?style=for-the-badge&logo=google-chrome&logoColor=white" alt="Chrome Extension">
  <img src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker">
  <img src="https://img.shields.io/badge/Ollama-000000?style=for-the-badge&logo=ollama&logoColor=white" alt="Ollama">
</p>

---

<p align="center">
  🌐 <b>Languages:</b> 
  <b>English</b> | <a href="README.tr.md">Türkçe</a>
</p>

---

**ManipuLens** is an AI-powered security and analysis tool designed to detect linguistic, psychological, and statistical manipulation techniques that Large Language Models (LLMs) may employ during user interactions. Supported by a **Multi-Agent LLM** orchestration architecture and a high-performance **Rust** backend service, this privacy-focused advanced browser extension monitors AI chat flows in real time and analyzes selected web text via a right-click context menu to instantly detect and expose cognitive, statistical, and behavioral manipulation.

### Key Focus Areas:
* 🕵️ **Multi-Agent Verdicts:** Specialised sub-agents (Linguistic, Psychological, Behavioral, Perceptual, Social) dissecting texts concurrently.
* 🛡️ **Privacy-First (Local AI):** Runs entirely on your machine using local LLMs via Ollama—your data never leaves your local network.
* 🚀 **Blazing Fast Performance:** Powered by Rust (Axum/Tokio) for near-instant orchestration and evaluation.

---

## 🚀 Key Features

* **⚡ High-Performance Rust Orchestrator:** Uses Axum and Tokio asynchronously to trigger and join specialized sub-agent analysis paths in parallel, ensuring sub-second backend response times.
* **🧠 Brain-Chained Multi-Agent Architecture:** Features 5 specialized domain experts overseen by a Sentezör (Manager) Agent, and an advanced *Consumer Intent Agent* to predict cognitive vulnerabilities.
* **🖱️ Context Menu Integration (Seamless UX):** No manual copy-pasting required. Highlight any text on any webpage, right-click, and select "Analyze with ManipuLens" to automatically trigger the flow.
* **🎨 Dynamic Highlight Injection:** Instead of generic overlays, the extension dynamically changes the DOM `<mark>` style matching the exact color profile of the **dominant manipulation type** found (e.g., Psychological = Magenta, Social = Blue).
* **🐳 Production-Ready DevOps:** Built using Docker Multi-Stage builds with static Linux compilation (`x86_64-unknown-linux-musl`) running on ultra-lightweight Alpine containers.

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

---

## 🗺️ System Architecture

ManipuLens employs a highly optimized asynchronous processing pipeline designed to handle complex multi-agent analysis without blocking the main event loops:

⚙️ Requirements
Before launching the production pipeline, ensure you have the following ecosystem components installed:

Docker Desktop (With Compose support active)

Ollama (Running locally on port 11434)

Google Chrome Browser (Or any Chromium-based browser)

🚀 Quick Start (Backend Deployment)
Thanks to our optimized multi-stage Docker setup, you can compile the entire Rust matrix and spin up the environment with a single command orchestration.
1. Download and Serve the Local Intelligence Model
Open your system terminal and pull down the llama3 core weight infrastructure using Ollama:
```
ollama pull llama3
```
2. Run the Multi-Agent Cluster via Docker Compose
Navigate to the project root directory (/manipulation-detector) and execute the build pattern:
```
docker-compose up --build
```
This handles the static musl-compilation within isolation layers and fires up the Axum server bound to 0.0.0.0:3000 with direct bridging to your native Ollama port.

🧩 Chrome Extension Installation (Frontend Setup)
Since the frontend extension lives directly inside the browser environment, load it manually into your Chromium instance:

Copy the URL chrome://extensions/ and paste it into your Chrome address bar.

Toggle the Developer Mode (Geliştirici Modu) switch located in the upper right-hand corner.

Click the Load Unpacked (Paketlenmemiş öğe yükle) button on the top-left layout.

Select the extension directory inside your local repository folder.

🎉 The ManipuLens icon will appear in your utility bar, fully wired and listening to your right-click triggers!

📄 License
Distributed under the MIT License. See LICENSE for more information.
