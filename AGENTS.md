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
- Docker: `docker-compose up --build` (brings up `postgres:16` on host port 5433 plus the backend).
- Tests need a live PostgreSQL: each test opens its own isolated schema via `Db::connect_test()`; address from `DATABASE_URL_TEST` (default `postgres://postgres:postgres@127.0.0.1:5433/manipulens_test`).
- Extension: no build step — `chrome://extensions` → Developer Mode → Load unpacked → `extension/`.

## Architecture
Two runtimes joined by HTTP:
- **`src/`** — Rust backend (Axum/Tokio) at `http://127.0.0.1:3000`.
- **`extension/`** — Chrome MV3 (background service worker + popup); calls backend, injects into page DOM.
- Backend calls local **Ollama** (`OLLAMA_URL`, default `http://localhost:11434`); no cloud LLM.

### Backend modules (all route handlers live in `main.rs`, not split out)
- `main.rs` — Axum router; all HTTP handlers (register/login/verify/resend/forgot/reset/analyze/translate-report/history/profile/consent/ads/healthz); `authenticate()` middleware; SMTP sending; `.env` loader.
- `orchestrator.rs` — core of `/v1/analyze`: runs 6 expert agents in parallel (`tokio::join!`), reduces via "Synthesizer" (manager) LLM call into `FinalReport`; on synthesizer failure `fallback_summary()` builds local summary from highest-confidence agent. Also `repair_language` and `translate_report`.
- `agents.rs` — one Ollama prompt fn per manipulation type (Linguistic/Psychological/Behavioral/Perceptual/Social/Marketing); all share `call_ollama_agent()` and a `reqwest::Client` in `OnceLock`. Also the pre-filter gate `needs_full_analysis()`, which decides whether the 6 agents run at all. Three stages, cheapest first: (1) `looks_like_sales_copy()` / `looks_like_personal_pressure()` — keyword rules, no LLM; (2) `llm_genre_gate()` — "what kind of text is this?"; (3) `commercial_intent_gate()` — "does the writer steer the reader to something they provide?", asked only when stage 2 dropped the text. Stage 3 exists because stage 2 missed 8/12 ads disguised as news, reviews or personal stories (measured 2026-09-12); the keyword rules caught 0 of those 8, so patterns alone do not generalise.
- `auth.rs` — in-memory rate limiting (`RateWindow`), brute-force lock (`LoginGuard`), session token generation (`new_token`, 128 hex chars).
- `db.rs` — PostgreSQL via `sqlx` (`PgPool`, runtime query API — no compile-time DB needed): `users`, `history`, `sessions`, `user_profiles`. Schema lives in `migrations/`, applied on startup. Connection from `DATABASE_URL` (default `postgres://postgres:postgres@127.0.0.1:5433/manipulens`). One-time auto-import from legacy `users.json`/`history.jsonl` (`migrate_from_json_files`); source files not deleted.
- `ads.rs` — ad targeting agent. Two layers: `score_candidates()` is a pure, LLM-free scoring/exclusion function (testable, auditable) and `explain_top()` makes ONE Ollama call only to phrase the "why this ad?" line; if it fails the rule labels are used. Hard exclusions: no consent, campaign language the user never reads, sensitive category without a reliable adult age signal, and urgency-framed campaigns for users whose dominant manipulation type is Davranışsal (the tool does not run the trap it exposes).
- `import_sqlite.rs` — one-shot migration `--import-sqlite manipulens.db`: reads the old SQLite file into PostgreSQL, skips any table that already has rows, never touches the source.
- `audit.rs` — daily-rotating JSONL audit log (`logs/audit-YYYY-MM-DD.jsonl`); full analyzed text is NEVER logged, only first 120 chars preview.
- `types.rs` — all serde DTOs; single source of truth for the backend↔extension JSON contract.

### `/v1/analyze` flow
1. Extension: right-click → `background.js` POSTs selected text with `Authorization: Bearer <token>`.
2. `main.rs::handle_analyze` — session check (401 without token), then per-user rate limit (10/min).
3. `orchestrator::run_orchestrator` — pre-filter gate first (`agents::needs_full_analysis`); if it says no, a clean report is returned after 1-2 Ollama calls instead of 7. Otherwise 6 agents parallel, then synthesizer (sequential).
4. Result written to PostgreSQL `history` (agent verdicts as `jsonb`) + JSONL audit; returned as `FinalReport` JSON.
5. Extension: `background.js::highlightSentencesOnPage` finds `target_sentences` via flexible regex, wraps them in `<mark>` colored per detecting agent (written with `textContent` — XSS-safe).

### Measurement (labelled sets, no server needed)
Two offline modes read the same `ETIKET|text` file format (`MANIP` / `TEMIZ`, `#` comments):
- `--gate-file <set.txt>` — pre-filter only. Prints, per text, which rule fired, what the genre gate said, what the intent gate said, and the final decision; then the miss rate on `MANIP` texts (a dropped ad never gets a report — the most expensive error) and how many `TEMIZ` texts passed (cost, not an error).
- `--analyze-file <set.txt>` — full pipeline: accuracy, false positives/negatives, per-agent trigger counts.

Sets in the repo: `dogrulama-seti.txt` (21, used for prompt tuning — regression set, not independent), `kapi-olcum-seti.txt` (30: 10 short ads, 10 disguised ads, 10 hard clean texts), `kapi-dogrulama-seti-2.txt` (18, written after the rule layer was hardened). Tuning against a set forfeits its independence — when that happens, write a new one and keep the old as a regression set.

### Auth
Sessions travel in `Authorization: Bearer <token>` header, not cookies (token in PostgreSQL `sessions`, 30-day TTL). Passwords bcrypt-hashed inside `spawn_blocking` (CPU-heavy). Login verifies against a fixed dummy hash even when the user doesn't exist — closes the timing side channel.

### Extension ↔ backend URL resolution (gotcha)
Backend address is not hardcoded: extension fetches `ngrok_url` from `extension/server_config.json` via GitHub raw (ngrok tunnel URL is unstable). `popup.js::getBaseUrl()` caches it 5 min in `chrome.storage.local`; but `background.js::startAnalysisInBackground` (the main analyze path) does NOT use that cache — fresh GitHub fetch every time. The two resolution paths are independent: changing one may require changing the other.

### i18n (TR/EN)
`lang` field ("tr"/"en") selects LLM prompts and UI messages (`norm_lang`, `pick` helpers in `main.rs`). Wrong-language LLM output trips the `orchestrator::wrong_language` heuristic → `repair_language` fixes it in one batch translation call. `HistoryEntry.lang` records generation language; on UI language switch only wrong-language entries get translated, and the translation is persisted to DB (no repeat Ollama call for the same entry).

### Ads (`/v1/ads`)
Targeting reads the inferred profile, so it is gated on explicit consent: `users.ads_consent` defaults to **false**, and with consent off the profile is never read. Revoking consent deletes that user's `ad_decisions` (events cascade); deleting the profile deletes them too. `POST /v1/ads/inventory` is the admin endpoint and stays CLOSED unless `ADS_ADMIN_TOKEN` is set (header `x-ads-admin-token`). Every served ad carries a user-visible reason line.
