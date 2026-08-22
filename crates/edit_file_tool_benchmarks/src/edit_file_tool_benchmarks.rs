//! Benchmark for the agent's `edit_file` tool.
//!
//! This crate is split out from `benchmarks` because its harness drives the
//! tool through `TestAppContext`/`FakeFs`/`FakeLspAdapter` and `Project::test`,
//! which pull `test-support` into `agent`, `editor`, `language`, `lsp`,
//! `project`, and `settings`. Cargo's feature unification would otherwise
//! apply those test-only builds to every other benchmark target in the same
//! package, including the ones that render production code paths. Keeping
//! this benchmark in its own package confines that contamination to the one
//! benchmark that actually needs it.
//!
//! It no longer needs `language_model`'s `test-support` feature: `edit_file`
//! never reads a thread's configured model (see `EditSessionContext`/
//! `authorize_file_edit` in `agent::tools`), so the harness leaves it unset
//! instead of constructing a `FakeLanguageModel`, which measures the same
//! tool path while dropping that fake object entirely. It also no longer
//! directly requests `lsp`'s `test-support` feature: everything it uses from
//! `lsp`'s test-only surface (`FakeLanguageServer::handle_notification`/
//! `notify`, reached through `language::LanguageRegistry::register_fake_lsp`)
//! is already pulled in by the direct request of `language`'s `test-support`
//! feature, which requests `lsp/test-support` itself; the direct request was
//! redundant manifest debt, not a distinct capability this benchmark needed.
//!
//! This is a staging step, not a destination: the long-term goal is a harness
//! with no `test-support` dependency at all. See `benches/edit_file_tool.rs`
//! for the remaining test-only APIs this benchmark still relies on.
