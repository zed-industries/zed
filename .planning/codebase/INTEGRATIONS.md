# External Integrations

**Analysis Date:** 2026-03-01

## APIs & External Services

**Language Model Providers:**
- OpenAI (GPT-4, GPT-4 Turbo, GPT-4o, o1, o3)
  - SDK/Client: `crates/open_ai/` with custom HTTP client
  - Auth: API key via settings, configurable per user
  - Base URL: `https://api.openai.com/v1`

- Anthropic (Claude models)
  - SDK/Client: `crates/anthropic/` with custom HTTP client
  - Auth: API key via settings and credentials_provider
  - Base URL: `https://api.anthropic.com`
  - Special: Supports context-1m beta header

- Google AI (Gemini)
  - SDK/Client: `crates/google_ai/` with custom HTTP client
  - Auth: API key via settings

- Mistral
  - SDK/Client: `crates/mistral/` with custom HTTP client
  - Auth: API key via settings

- DeepSeek
  - SDK/Client: `crates/deepseek/` with custom HTTP client
  - Auth: API key via settings

- Vercel (Edge API)
  - SDK/Client: `crates/vercel/` with custom HTTP client

- xAI
  - SDK/Client: `crates/x_ai/` with custom HTTP client

- Open Router
  - SDK/Client: `crates/open_router/` with schemars support

- Ollama (Local)
  - SDK/Client: `crates/ollama/` with custom HTTP client

- LMStudio (Local)
  - SDK/Client: `crates/lmstudio/` with custom HTTP client

- Supermaven (Code Completion)
  - SDK/Client: `crates/supermaven_api/` with custom HTTP client
  - Location: `crates/supermaven/`

- GitHub Copilot
  - SDK/Client: `crates/copilot/` and `crates/copilot_chat/`
  - Implementation: Uses LSP extension model
  - Node Runtime: Via `crates/node_runtime/`

**AWS Services:**
- Bedrock (AWS ML Model Inference)
  - SDK/Client: `crates/bedrock/` and `crates/aws_http_client/`
  - Auth: Multiple methods - Access key/secret, named profiles, SSO profiles, bearer tokens
  - Config env vars: `ZED_ACCESS_KEY_ID`, `ZED_SECRET_ACCESS_KEY`, `ZED_SESSION_TOKEN`, `ZED_AWS_PROFILE`, `ZED_AWS_REGION`, `ZED_AWS_ENDPOINT`, `ZED_BEDROCK_BEARER_TOKEN`
  - Service URL: `https://amazonaws.com`

- S3 (Object Storage)
  - SDK: `aws-sdk-s3` 1.15.0 via collab server
  - Usage: File/asset storage in collab infrastructure

- Kinesis (Event Streaming)
  - SDK: `aws-sdk-kinesis` 1.51.0 via collab server
  - Usage: Event streaming and analytics in collab infrastructure

## Data Storage

**Databases:**
- **SQLite (Local)**
  - Client: `sqlez` crate (custom abstraction with macros)
  - Bindings: `libsqlite3-sys` 0.30.1 (bundled)
  - Location: User local data directory (`.zed/` typically)
  - Schema: Managed by `crates/migrator/`

- **PostgreSQL (Collab Server)**
  - Client: `sqlx` 0.8 with PostgreSQL driver
  - ORM: `sea-orm` 1.1.10
  - Connection: Via `runtime-tokio-rustls` with UUID support
  - Location: Collab server infrastructure (`crates/collab/`)

**File Storage:**
- Local filesystem - Worktree operations via `crates/fs/`
- AWS S3 - Collab server storage via Kinesis/Bedrock infrastructure
- Remote server storage - Via `crates/remote_server/`

**Caching:**
- Moka 0.12.10 - In-memory cache with sync features
- Custom LMDB via `heed` 0.21.0 - Key-value store for embeddings/indexes
- Parking lot-based synchronization for shared state

## Authentication & Identity

**Auth Provider:**
- Custom credential system
- GitHub OAuth (for Git operations)
  - Implementation: `crates/git_hosting_providers/`
  - GitHub API: `https://api.github.com`
  - Token env var: `GITHUB_TOKEN`

- Zed Cloud Authentication
  - SDK/Client: `crates/cloud_api_client/`
  - Types: `crates/cloud_api_types/`

- Credentials Provider
  - Location: `crates/credentials_provider/`
  - Integration: System keychain/credential storage

## Monitoring & Observability

**Error Tracking:**
- Sentry integration available (referenced in `.factory/prompts/crash/`)
- Crash handling via `crash-handler` 0.6
- Custom minidump generation with `minidumper` 0.8

**Logs:**
- `log` 0.4.16 crate with structured logging
- `env_logger` 0.11 for local development
- `tracing` 0.1.40 for async tracing (in collab server)
- `tracing-subscriber` 0.3.18 for JSON formatting in production

**Telemetry:**
- Custom telemetry system in `crates/telemetry/`
- Telemetry events in `crates/telemetry_events/`
- Prometheus 0.14 metrics in collab server (`crates/collab/`)

## CI/CD & Deployment

**Hosting:**
- Desktop application: Self-contained binary distribution
- Collab Server: Container-ready (postgres/sqlite support)
- Remote Server: Runs as daemon (`crates/remote_server/`)

**CI Pipeline:**
- GitHub Actions (inferred from `GITHUB_RUN_NUMBER` env var)
- Protobuf formatting checks
- Clippy linting enforcement

**Build Artifacts:**
- Native binaries: macOS (universal/arm64/x86_64), Windows, Linux
- Remote server: Linux musl binary
- Extensions: WASM binaries

## Environment Configuration

**Required env vars:**
- Collab server: Database URL, AWS credentials/config
- Language models: API keys for desired providers (ANTHROPIC_API_KEY pattern inferred)
- Optional: `GITHUB_TOKEN` for elevated GitHub API limits
- AWS: ZED-prefixed vars for bedrock (`ZED_ACCESS_KEY_ID`, `ZED_SECRET_ACCESS_KEY`, etc.)

**Secrets location:**
- System keychain: Via `credentials_provider` crate
- Environment variables: Loaded via `dotenvy` 0.15.0
- Settings files: `~/.zed/settings.json` (contains API key configurations)
- Never committed: `.env` files excluded from version control

## Webhooks & Callbacks

**Incoming:**
- Collab server WebSocket endpoints - Real-time collaboration
- Language server LSP endpoints - Editor language features
- Git push/commit hooks - Custom tooling

**Outgoing:**
- LiveKit webhooks (for voice/video events)
- GitHub API webhooks (repository events)
- AWS SQS/SNS via Kinesis (event delivery)
- Telemetry events (custom backend)

## Real-Time Communication

**Voice & Video:**
- LiveKit 0.7.32 - WebRTC infrastructure
  - Client: `crates/livekit_client/`
  - API: `crates/livekit_api/`
  - Features: Audio recording/playback via rodio, screen capture support
  - Location: Collaboration infrastructure

**Collaboration:**
- WebSocket protocol via `async-tungstenite` 0.31.0
- Collab server: `crates/collab/`
- RPC protocol: `crates/rpc/`

## Extensions & Plugin System

**WASM Extensions:**
- Runtime: `wasmtime` 33 with component model support
- Extensions: `wasm32-wasip2` target (standard)
- Extension API: `crates/extension_api/`
- Extension Host: `crates/extension_host/`

**Language Extensions:**
- Tree-Sitter grammars: 30+ language parsers built-in
- LSP-based language support: Extensible via language servers

## Code Formatting & Linting

**Providers:**
- Prettier - JavaScript/TypeScript formatting via Node runtime
- Location: `crates/prettier/`
- Custom language server integration

## Search & Indexing

**Full-Text Search:**
- Tree-sitter based syntax highlighting
- Custom search implementation in `crates/search/`
- Web search providers in `crates/web_search_providers/`

## Version Control

**Git Integration:**
- `git2` 0.20.1 with vendored libgit2
- Location: `crates/git/`
- GitHub integration: `crates/git_hosting_providers/`

## Diagnostics & Language Features

**Language Server Protocol:**
- Custom LSP implementation in `crates/lsp/`
- Debug Adapter Protocol (DAP): `crates/dap/` with multiple adapters
- Language tools: `crates/language_tools/`

---

*Integration audit: 2026-03-01*
