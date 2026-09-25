# ManipuLens

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Chrome_Extension-4285F4?style=for-the-badge&logo=google-chrome&logoColor=white" alt="Chrome Extension">
  <img src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white" alt="Docker">
  <img src="https://img.shields.io/badge/Azure_OpenAI-0078D4?style=for-the-badge&logo=microsoftazure&logoColor=white" alt="Azure OpenAI">
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
* 🛡️ **Privacy-Focused:** Text selected for analysis is sent to the ManipuLens Azure backend and the configured Azure OpenAI model. Account and analysis-history data are stored in PostgreSQL; the full analyzed text is not written to audit logs.
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
| **Marketing (Pazarlama)** | Disguised advertising, problem inflation, miracle claims, and purchase pressure. | `#2a9d8f` (Teal) |

---

## 🚪 Pre-filter Gate and Measurement

Not every text reaches the six agents. A **pre-filter gate** runs first - it cuts false
alarms and prevents unnecessary full multi-agent analysis. Clean texts can stop after the Azure OpenAI pre-filter stage. Three stages, cheapest first:

1. **Rule layer (no LLM)** - sales-copy signals (urgency, scarcity, price, call to action,
   social proof; at least two distinct groups) and personal-pressure patterns. A match goes
   straight to full analysis without calling the model.
2. **Genre question** - "is this text informing or persuading?"
3. **Commercial-intent question** - asked only when the genre question dropped the text:
   "does the writer steer the reader toward something they provide?" The decisive test is
   **who gains**: a writer who sells or represents the thing, yes; a writer who merely used
   or reported it, no.

Stage 3 came out of measurement: 8 of 12 ads disguised as news, reviews or personal stories
were dropped by the genre question, and the keyword rules caught **none** of them
(2026-09-12). A pattern list memorises; a purpose question generalises.

Two offline commands, no server or extension needed:

```bash
cargo run --release -- --gate-file kapi-olcum-seti.txt      # gate only
cargo run --release -- --analyze-file kapi-olcum-seti.txt   # full pipeline
```

File format is `LABEL|text` (`MANIP` / `TEMIZ`, `#` for comments). Sets in the repo:
`dogrulama-seti.txt` (21, used for prompt tuning - regression set),
`kapi-olcum-seti.txt` (30: short ads / disguised ads / hard clean texts),
`kapi-dogrulama-seti-2.txt` (18, written after the rule layer was hardened).
Tuning against a set forfeits its independence - write a new one and keep the old for
regression.

---

## 🔄 UI/UX Workflow

1. **Selection & Trigger:** The user selects a text snippet on a webpage. Right-clicking creates a secure transaction via `background.js` using `chrome.storage.local`.
2. **Asynchronous Handshake:** The extension popup auto-opens, instantly locking the UI interaction (`button.disabled = true`) to prevent race conditions.
3. **Rust Multi-Thread Sifting:** In production, the payload reaches the `/v1/analyze` endpoint hosted on Azure Container Apps, starting the multi-agent analysis pipeline.
4. **Visual Synthesis:** The extension injects contextually colored markers back into the target web page's active DOM and displays a customized consumer behavior prediction card.

---

## 📊 User Profile and Dataset

Every analysis is written to the PostgreSQL `history` table together with the six
expert agents' verdicts (not the full text — a 120-character preview). A
two-layer user profile is built on top of it:

- **Counter layer** — refreshed after every analysis, outside the request
  (`tokio::spawn`) and without any LLM call. Totals, manipulated ratio,
  dominant-type distribution, per-agent detection counts, language mix,
  product/sector predictions, average text length.
- **Inference layer** — the demographic agent (`analyze_demographic`); reads
  the user's counters plus their last 30 text previews and estimates age
  band, education level, consumer tendency and interests. Refreshed every 5
  analyses (or when the inference is older than 24 hours) rather than on
  every one — a full analysis already requires multiple Azure OpenAI calls, and an additional inference call
  would land on the user's wait time.

The agent's limits are enforced in code: any estimate below 0.60 confidence
is forced to "unknown" (the model's compliance is not trusted — the output is
re-checked in Rust), and **ethnicity, religion, health, sexual orientation
and political opinion** are both forbidden in the prompt and absent from the
output schema, since they are special-category personal data under GDPR/KVKK.
If the agent fails, the existing profile is left untouched.

No profile is produced below 5 analyses. A user can read and delete their own
profile only; identity is derived solely from the session token.

```
GET  /v1/profile          → your own profile (exists:false if not built yet)
POST /v1/profile/delete   → delete your own profile (history untouched)
```

## 📣 Ad Targeting

The collected profile feeds a targeting agent (`src/ads.rs`) that decides which
suggestion a user sees. Two layers: a rule layer scores and excludes without any
LLM call, and the LLM only phrases the "why this ad?" line — so the decision stays
deterministic and auditable.

Limits are baked into the code:
- **Consent required.** `users.ads_consent` defaults to `false`; with consent off
  the profile is never read. Revoking consent deletes the decisions made under it.
- Sensitive categories (gambling, alcohol, credit) need a reliable **adult age
  signal**; an unknown age does not count as adult.
- Campaigns framed with artificial **urgency** are withheld from the users most
  exposed to behavioural manipulation. ManipuLens exposes that trap; it will not
  run it in its own panel.
- Demographic signals below 0.60 confidence never reach the score.
- Every ad carries a visible "why this ad?" line and can be dismissed.

```
GET  /v1/ads               → suggestions for you (+ decision_id)
POST /v1/ads/feedback      → impression | click | dismiss
POST /v1/consent           → turn ad personalisation on/off
POST /v1/ads/inventory     → add/update a campaign (requires ADS_ADMIN_TOKEN)
```

The inventory starts empty; with no campaigns `/v1/ads` returns an empty list.
`ornek-reklam-envanteri.json` in the repo holds seven sample campaigns (one in
English, one `sensitive`); load them with:

```bash
ADS_ADMIN_TOKEN=... python envanter-yukle.py ornek-reklam-envanteri.json
```

Re-loading the same `id` updates that campaign instead of creating a duplicate.

**Dataset export** — runs without starting the server, one analysis per line:

```
cargo run -- --export-dataset dataset.jsonl
```

Privacy: the export contains no email addresses; users are separated by UUID.

## 🗺️ System Architecture

ManipuLens employs a highly optimized asynchronous processing pipeline designed to handle complex multi-agent analysis without blocking the main event loops:

⚙️ Requirements

For local development, make sure you have:

- Docker Desktop with Compose support
- Google Chrome or another Chromium-based browser
- Access to Azure OpenAI / Microsoft Foundry
- The required Azure OpenAI environment variables configured in `.env`

Required core environment variables:

AZURE_OPENAI_ENDPOINT=
AZURE_OPENAI_DEPLOYMENT=
AZURE_OPENAI_API_KEY=
DATABASE_URL=

If email verification and password reset are enabled, SMTP settings must also be configured.

🚀 Quick Start (Local Backend)

Create your own `.env` file based on `.env.example`. Never commit real API keys or passwords to the repository.

Then run:

docker-compose up --build

Docker Compose starts the local PostgreSQL database and the Rust/Axum backend. The backend listens on `0.0.0.0:3000` and sends analysis requests to the configured Azure OpenAI deployment.

In production, the ManipuLens backend runs on Azure Container Apps, while PostgreSQL is hosted on Azure Database for PostgreSQL Flexible Server.

🧩 Chrome Extension Installation (Frontend Setup)
Since the frontend extension lives directly inside the browser environment, load it manually into your Chromium instance:

Copy the URL chrome://extensions/ and paste it into your Chrome address bar.

Toggle the Developer Mode (Geliştirici Modu) switch located in the upper right-hand corner.

Click the Load Unpacked (Paketlenmemiş öğe yükle) button on the top-left layout.

Select the extension directory inside your local repository folder.

🎉 The ManipuLens icon will appear in your utility bar, fully wired and listening to your right-click triggers!

📄 License
Distributed under the MIT License. See LICENSE for more information.
