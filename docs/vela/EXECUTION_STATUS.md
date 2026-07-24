# Execution status

Updated: 2026-07-23

## Active workstreams

| Stream | Scope | Status | Current result |
|---|---|---|---|
| Editor/Agent architecture | Zed agent, Agent UI, Diff, Project/LSP | Validated | Native Agent fake-model stream, cancellation-aware tool, Agent Diff, and compaction budget tests pass |
| Settings/configuration | Zed Settings Store, Settings UI, TOML facade | Designed | ADR-002 accepted; JSON/JSONC coupling identified; facade maps into typed Settings Store incrementally |
| Git/Diff | Turn Diff, Branch Diff, base picker | Validated | Agent Diff and merge-base Branch Diff/base-picker tests pass; extend rather than rebuild |
| Local-first availability | Remove account gates from locally implemented features | In progress | Required local flags are always enabled; Agent trial/upgrade overlays and Zed Pro call-to-action paths are removed |
| Build/platform | Toolchain, Xcode, cmake, reproducible build | Ready | Xcode 26.5, Metal Toolchain 17F42, cmake 4.4.0, and Rust 1.95.0 are ready; `cargo check -p zed` passes |

## Completed

- Created `upstream.lock` with exact Zed and Pi reference commits.
- Created ADR-001 for the in-process Rust agent decision.
- Added reproducible development environment and Zed Cargo wrappers.
- Installed Rust 1.95.0, cmake 4.4.0, and Metal Toolchain 17F42.
- Validated the full Zed crate with `cargo check -p zed`.
- Built the 1.1 GB debug executable and smoke-launched it with isolated user data.
- Validated native Agent streaming, cancellation-aware tools, and compaction budgeting with fake models.
- Validated existing Agent Diff and Branch Diff/base picker tests.
- Accepted ADR-002 for the incremental TOML settings facade.
- Added and validated Rust, Go, Python, and TypeScript P0 fixtures.
- Made six local CodeIDE capabilities independent of Zed staff flags and server-delivered feature flags.
- Rebuilt and launched Zed fork commit `0652116bf1` with the local-first policy.
- Removed Agent onboarding/trial-expiration overlays, reset actions, and the Zed Pro upgrade button.
- Replaced hosted payment-limit messaging with BYOK/local-provider configuration actions; rebuilt and launched fork commit `0966a52559`.
- Swapped the primary panels for new CodeIDE profiles: project tree on the left and Agent conversation on the right; applied it to the development profile and launched fork commit `15b9019eb5`.
- Unified Chat and searchable Thread History into one default-open right Agent panel with `Chat`/`Threads` switching; launched fork commit `01ed0b645c`.
- Disabled the legacy standalone Workspace Threads Sidebar and its status toggle so persisted sidebar state cannot obscure the unified Agent panel; launched fork commit `c36ba15606`.
- Named the product Vela, configured `github.com/bkcarlos/vela` as `origin`, renamed the product branch to `vela/main`, pushed all fork changes, and made official Zed a fetch-only `upstream`.
- Adopted the Fold/Indigo application identity and replaced visible native Agent, panel, welcome, onboarding, Diff, diagnostics, and quick-action marks with the 16px Vela symbol (`71be9d362c`).
- Renamed the executable, macOS bundle metadata, URL scheme, release-channel display names, and user-data roots to Vela; the ad-hoc-signed development bundle is installed as `~/Applications/Vela.app` with Bundle ID `app.vela.Vela-Dev` for Launchpad discovery (`a8e8a26f08`).
- Began native multi-provider authentication: added a Settings UI mode that offers account sign-in or API key, implemented OpenRouter browser PKCE sign-in with Keychain persistence, and rebranded shared OAuth callback pages (`ea98fa4708`).
- Surfaced ChatGPT OAuth as the first provider in LLM Provider settings, renamed it from `ChatGPT Subscription` to `ChatGPT`, and added a direct `Connect Providers` action to the Agent model selector (`8852430ebe`).
- Diagnosed a real ChatGPT OAuth rejection as OpenAI `unsupported_country_region_territory`; added safe, actionable provider-specific UI messaging without attempting to bypass OpenAI policy (`d6a1f9de12`).
- Added automatic macOS SystemConfiguration proxy inheritance (explicit Vela setting → environment → macOS proxy), verified the running app selected the local proxy, and replaced the top-right Zed account sign-in with a Vela `Connect` entry that opens ChatGPT/Copilot/OpenRouter/API-key providers (`d6fe3be8e0`).
- Initialized the CodeIDE Git repository.

## Important findings

1. `crates/agent_ui/src/agent_diff.rs` already implements substantial Agent Diff behavior.
2. `crates/git_ui/src/branch_diff.rs` already supports merge-base comparison and a base branch picker.
3. `crates/agent/src/tools/` already contains definition, references, diagnostics, rename, and code-action tools.
4. `crates/language_models/src/provider/` already contains OpenAI-compatible and Anthropic-compatible providers.
5. Compatible-provider model settings already require per-model `max_tokens` and support `max_output_tokens`, capabilities, Base URL, headers, and Keychain-backed API keys.
6. Native Agent already implements queueing, compaction, retry, permissions, subagents, token accounting, and context-window tests.
7. Zed Settings UI and persistence are strongly coupled to `settings.json`; TOML should begin as a facade rather than a full rewrite.
8. Debug builds already receive most staff flags, but LSP tools and several review/security features were still off; CodeIDE now enables its required local features explicitly.
9. Zed Cloud inference and hosted edit-prediction quotas are enforced server-side. CodeIDE must use BYOK/local providers instead of pretending a hosted subscription exists.

These findings reduce the amount of new Diff, Git, and semantic-tool code, but increase the importance of integration tests and careful extension of existing crates.

## Blockers

No development dependency blocker remains. The global `xcode-select` points to Command Line Tools, so Zed commands must run through `scripts/with-zed-environment.sh` unless the user later changes the global developer directory with administrator authentication.

## Next executable tasks

1. Implement the first TOML facade vertical slice for custom model providers.
2. Connect the Agent empty state directly to CodeIDE provider configuration.
3. Connect one model-provider Settings UI flow to TOML persistence.
4. Add comment-preserving round-trip and invalid-config tests.
5. Exercise P0 fixtures through Zed LSP integration tests.
6. Define the remaining gaps between current Agent Diff/context UI and the required ChangeSet workflow.
