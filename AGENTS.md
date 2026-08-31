# AGENTS.md

Ortak ajan sözleşmesi. Claude Code, Codex ve bu depoda çalışan diğer ajanlar bunu
okur; `CLAUDE.md` bu dosyayı içe aktarır. Kural buraya yazılır, ikinci kopya açılmaz.

## Communication
- Make ALL permission requests in TURKISH: 1-2 sentences — what and why. No filler.

## Discipline
- Read only relevant files; never scan the whole project.
- Rust (Axum): prioritize type safety. Extension: strict Chrome MV3.
- Output only diffs or results.
- Structural refactors via `sg` (ast-grep), not text search-replace. Scripts checked with `shellcheck`.

## Commands
Backend (Rust), from repo root:
```
cargo build [--release]        # release is what Dockerfile uses
cargo run                      # serves 0.0.0.0:3000; needs Ollama at localhost:11434
cargo test                     # tests are #[cfg(test)] modules inside src/*.rs
cargo test auth::tests::token_is_long_and_unique   # single test
cargo clippy --all-targets     # some rules are #[deny(...)]: clippy fails, build still passes
```
- Pull model first: `ollama pull llama3` (name hardcoded in `src/agents.rs`, `src/orchestrator.rs`).
- SMTP optional; without `.env`, codes print to console (dev mode). Setup: [SETUP_MAIL.md](SETUP_MAIL.md).
- Docker: `docker-compose up --build` (uses `OLLAMA_URL=http://host.docker.internal:11434`).
- Extension: no build step — `chrome://extensions` → Developer Mode → Load unpacked → `extension/`.

## Architecture
Two runtimes joined by HTTP:
- **`src/`** — Rust backend (Axum/Tokio) at `http://127.0.0.1:3000`.
- **`extension/`** — Chrome MV3 (background service worker + popup); calls backend, injects into page DOM.
- Backend calls local **Ollama** (`OLLAMA_URL`, default `http://localhost:11434`); no cloud LLM.

### Backend modules (all route handlers live in `main.rs`, not split out)
- `main.rs` — Axum router; all HTTP handlers (register/login/verify/resend/forgot/reset/analyze/translate-report/history/healthz); `authenticate()` middleware; SMTP sending; `.env` loader.
- `orchestrator.rs` — core of `/v1/analyze`: runs 6 expert agents in parallel (`tokio::join!`), reduces via "Synthesizer" (manager) LLM call into `FinalReport`; on synthesizer failure `fallback_summary()` builds local summary from highest-confidence agent. Also `repair_language` and `translate_report`.
- `agents.rs` — one Ollama prompt fn per manipulation type (Linguistic/Psychological/Behavioral/Perceptual/Social/Marketing); all share `call_ollama_agent()` and a `reqwest::Client` in `OnceLock`.
- `auth.rs` — in-memory rate limiting (`RateWindow`), brute-force lock (`LoginGuard`), session token generation (`new_token`, 128 hex chars).
- `db.rs` — SQLite (rusqlite, bundled): `users`, `history`, `sessions` tables. One-time auto-import from legacy `users.json`/`history.jsonl` (`migrate_from_json_files`); source files not deleted.
- `audit.rs` — daily-rotating JSONL audit log (`logs/audit-YYYY-MM-DD.jsonl`); full analyzed text is NEVER logged, only first 120 chars preview.
- `types.rs` — all serde DTOs; single source of truth for the backend↔extension JSON contract.

### `/v1/analyze` flow
1. Extension: right-click → `background.js` POSTs selected text with `Authorization: Bearer <token>`.
2. `main.rs::handle_analyze` — session check (401 without token), then per-user rate limit (10/min).
3. `orchestrator::run_orchestrator` — 6 agents parallel, then synthesizer (sequential).
4. Result written to SQLite `history` + JSONL audit; returned as `FinalReport` JSON.
5. Extension: `background.js::highlightSentencesOnPage` finds `target_sentences` via flexible regex, wraps them in `<mark>` colored per detecting agent (written with `textContent` — XSS-safe).

### Auth
Sessions travel in `Authorization: Bearer <token>` header, not cookies (token in SQLite `sessions`, 30-day TTL). Passwords bcrypt-hashed inside `spawn_blocking` (CPU-heavy). Login verifies against a fixed dummy hash even when the user doesn't exist — closes the timing side channel.

### Extension ↔ backend URL resolution (gotcha)
Backend address is not hardcoded: extension fetches `ngrok_url` from `extension/server_config.json` via GitHub raw (ngrok tunnel URL is unstable). `popup.js::getBaseUrl()` caches it 5 min in `chrome.storage.local`; but `background.js::startAnalysisInBackground` (the main analyze path) does NOT use that cache — fresh GitHub fetch every time. The two resolution paths are independent: changing one may require changing the other.

### i18n (TR/EN)
`lang` field ("tr"/"en") selects LLM prompts and UI messages (`norm_lang`, `pick` helpers in `main.rs`). Wrong-language LLM output trips the `orchestrator::wrong_language` heuristic → `repair_language` fixes it in one batch translation call. `HistoryEntry.lang` records generation language; on UI language switch only wrong-language entries get translated, and the translation is persisted to DB (no repeat Ollama call for the same entry).
