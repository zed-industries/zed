# Extraction Plan: `language_core` crate

## Goal

Extract a `language_core` crate from the existing `language` crate. `language_core` contains tree-sitter grammar infrastructure, LSP adapter traits, language configuration, and highlight mapping — everything needed to parse source code and interact with language servers, without any Zed buffer/editor integration.

The existing `language` crate becomes the integration layer: it depends on `language_core` and adds buffer integration (`Buffer`, `SyntaxMap`, `Outline`, `DiagnosticSet`), settings, theme, and proto serialization.

A new `grammars` crate provides the embedded `.scm` query files, `config.toml` files, and native tree-sitter grammar registrations for built-in languages.

### Dependency graph after extraction

```
language_core   ← gpui, lsp, tree-sitter, http_client, parking_lot, regex, serde, util, collections
grammars        ← language_core, rust_embed, tree-sitter-rust, tree-sitter-python, ...
language        ← language_core, text, theme, settings, rpc, task, fs, clock, sum_tree, fuzzy
languages       ← language, grammars

Ex              ← language_core, grammars
Zed             ← language_core, grammars, language, languages
```

### Naming

- The new core crate is named `language_core` (crate root: `src/language_core.rs`).
- The existing integration crate keeps the name `language`.
- `language` re-exports everything from `language_core` so that downstream code using `use language::Grammar` etc. continues to work unchanged.

---

## Phase 0: Pre-cleanup in the existing `language` crate

These are structural fixes that simplify the extraction. Each is an independent commit. All existing tests must continue to pass after each change.

### 0.1 Remove unused `Buffer` parameter from `LspAdapter::process_diagnostics`

The `existing_diagnostics: Option<&Buffer>` parameter is passed through but never read by any implementation. The only override (`RustLspAdapter` in `crates/languages/src/rust.rs` line 265) names the parameter `_` and ignores it. The call site in `crates/project/src/lsp_store.rs` line 826 computes a buffer reference and passes it, but the adapter never uses it.

**Changes:**

1. **`crates/language/src/language.rs`** — `LspAdapter` trait definition (line ~412): Remove the `Option<&'_ Buffer>` parameter from `process_diagnostics`. New signature:
   ```
   fn process_diagnostics(&self, params: &mut lsp::PublishDiagnosticsParams, server_id: LanguageServerId) {}
   ```
2. **`crates/language/src/language.rs`** — `CachedLspAdapter::process_diagnostics` (line ~288): Remove the `existing_diagnostics` parameter. Update the delegation call.
3. **`crates/languages/src/rust.rs`** — `RustLspAdapter::process_diagnostics` (line ~265): Remove the `_: Option<&'_ Buffer>` parameter.
4. **`crates/project/src/lsp_store.rs`** (line ~826): Remove the buffer lookup (`params.uri.to_file_path()...get_buffer(...)`) and pass only `&mut params, server_id` to the adapter. If `get_buffer` becomes unused, remove it too (check for other callers first).
5. **Search for any other callers/overrides**: `grep -rn "process_diagnostics" crates/ --include="*.rs"` — update all sites. Also check `crates/extension_host/` and `crates/extension/` for extension-system wrappers.

### 0.2 Remove unused `&App` from `LspAdapter::retain_old_diagnostic`

No implementation uses the `cx: &App` parameter. The only override (`CLspAdapter` in `crates/languages/src/c.rs`) names it `_: &App`.

**Changes:**

1. **`crates/language/src/language.rs`** — `LspAdapter` trait: Remove `&App` from `retain_old_diagnostic`. New signature:
   ```
   fn retain_old_diagnostic(&self, old: &Diagnostic, new: &Diagnostic) -> bool { false }
   ```
2. **`crates/language/src/language.rs`** — `CachedLspAdapter::retain_old_diagnostic` (line ~298): Remove `cx` parameter, update delegation.
3. **`crates/languages/src/c.rs`** — `CLspAdapter::retain_old_diagnostic`: Remove `_: &App`.
4. **All callers**: Search `grep -rn "retain_old_diagnostic" crates/ --include="*.rs"` and update.

### 0.3 Remove unused `&App` from `LspAdapter::prepare_initialize_params`

No implementation uses the `cx: &App` parameter. Both overrides (`CLspAdapter` and `RustLspAdapter`) name it `_: &App`.

**Changes:**

1. **`crates/language/src/language.rs`** — `LspAdapter` trait: Remove `&App`. New signature:
   ```
   fn prepare_initialize_params(&self, params: InitializeParams) -> Result<InitializeParams> { Ok(params) }
   ```
2. **`crates/language/src/language.rs`** — `CachedLspAdapter::prepare_initialize_params`: Remove `cx`, update delegation.
3. **`crates/languages/src/rust.rs`** and **`crates/languages/src/c.rs`**: Remove `_: &App`.
4. **All callers**: Update.

### 0.4 Remove `fs: &dyn Fs` parameter from `ToolchainLister` trait methods

The `fs` parameter is passed into `list` and `resolve` but could be captured by the implementing struct at construction time. The only real implementation (`PythonToolchainProvider` in `crates/languages/src/python.rs`) uses `fs` in `list` (via `venv_to_toolchain(toolchain, fs)`) and `resolve`. The test mock `PythonMootToolchainLister` captures `Arc<FakeFs>` in its struct and ignores the `fs` parameter.

**Changes:**

1. **`crates/language/src/toolchain.rs`** — `ToolchainLister` trait:
   - Remove `fs: &dyn Fs` from `list`. New signature:
     ```
     async fn list(&self, worktree_root: PathBuf, subroot_relative_path: Arc<RelPath>, project_env: Option<HashMap<String, String>>) -> ToolchainList;
     ```
   - Remove `fs: &dyn Fs` from `resolve`. New signature:
     ```
     async fn resolve(&self, path: PathBuf, project_env: Option<HashMap<String, String>>) -> anyhow::Result<Toolchain>;
     ```
2. **`crates/language/src/toolchain.rs`** — Remove the `use fs::Fs;` import if no longer needed.
3. **`crates/languages/src/python.rs`** — `PythonToolchainProvider`:
   - Add an `fs: Arc<dyn Fs>` field to the struct.
   - Add a constructor: `pub fn new(fs: Arc<dyn Fs>) -> Self`.
   - In `list` and `resolve`, use `&*self.fs` instead of the parameter.
4. **`crates/languages/src/lib.rs`** — Where `PythonToolchainProvider` is constructed (around line 100-150), change to `PythonToolchainProvider::new(fs.clone())`.
5. **`crates/project/tests/integration/project_tests.rs`** — `PythonMootToolchainLister`: Remove `_: &dyn Fs` from the `list` and `resolve` method signatures.
6. **All callers of `list` and `resolve`**: Search `grep -rn "\.list(" crates/ --include="*.rs"` and `grep -rn "\.resolve(" crates/ --include="*.rs"` — but scope to callers that pass an `fs` argument to a `ToolchainLister`. Update call sites to drop the `fs` argument.

### 0.5 Move `Diagnostic` and `DiagnosticSourceKind` out of `buffer.rs`

These are pure data types with no buffer dependencies. They currently live in `crates/language/src/buffer.rs` but only depend on `lsp::DiagnosticSeverity`, `lsp::CodeDescription`, `serde`, `serde_json::Value`, and standard library types. Moving them to their own file makes the later extraction cleaner.

**Changes:**

1. **Create `crates/language/src/diagnostic.rs`** containing:
   - `Diagnostic` struct (currently `buffer.rs` around line 255)
   - `DiagnosticSourceKind` enum (currently `buffer.rs` around line 298)
   - Their `impl` blocks (trait impls, methods)
   - Required imports: `lsp`, `serde`, `serde_json`
2. **`crates/language/src/language.rs`** — Add `mod diagnostic;` and `pub use diagnostic::{Diagnostic, DiagnosticSourceKind};`.
3. **`crates/language/src/buffer.rs`** — Remove `Diagnostic` and `DiagnosticSourceKind` definitions. Add `use crate::{Diagnostic, DiagnosticSourceKind};` where needed.
4. **Verify** that `cargo test -p language` passes.

### 0.6 Verify all changes

Run:
```
cargo test -p language
cargo test -p languages
cargo build -p zed
```

All must pass before proceeding.

---

## Phase 1: Create the `language_core` crate

### 1.1 Create crate skeleton

Create `crates/language_core/Cargo.toml`:

```toml
[package]
name = "language_core"
version = "0.1.0"
edition = "2024"
publish = false

[lib]
path = "src/language_core.rs"

[dependencies]
anyhow.workspace = true
async-trait.workspace = true
collections.workspace = true
futures.workspace = true
gpui.workspace = true
http_client.workspace = true
log.workspace = true
lsp.workspace = true
parking_lot.workspace = true
regex.workspace = true
schemars.workspace = true
semver.workspace = true
serde.workspace = true
serde_json.workspace = true
smol.workspace = true
tree-sitter.workspace = true
util.workspace = true

[dev-dependencies]
gpui = { workspace = true, features = ["test-support"] }

[features]
test-support = []
```

Add to workspace `Cargo.toml`:
- Add `"crates/language_core"` to the `members` array
- Add `language_core = { path = "crates/language_core" }` to `[workspace.dependencies]`

Create empty `crates/language_core/src/language_core.rs`.

### 1.2 Verify skeleton builds

```
cargo check -p language_core
```

---

## Phase 2: Move types from `language` to `language_core`

This is the largest phase. Each step moves a cohesive group of types. After each step, both `language_core` and `language` should compile (language re-exports from language_core).

The general pattern for each move:
1. Copy code to `language_core`
2. Adapt imports (replace `SharedString` usage that stays fine since GPUI is allowed; replace `text::` types with local definitions where needed)
3. In `language`, replace the moved code with `pub use language_core::MovedType;`
4. Verify compilation

### 2.1 Move `HighlightMap` and `HighlightId`

**Source:** `crates/language/src/highlight_map.rs`

**Create `crates/language_core/src/highlight_map.rs`** with the contents of the current file, with these modifications:

- Remove `use gpui::HighlightStyle;` and `use theme::SyntaxTheme;`.
- Change `HighlightMap::new` signature from `(capture_names: &[&str], theme: &SyntaxTheme)` to `(capture_names: &[&str], highlight_names: &[&str])`. The matching algorithm stays identical — instead of iterating `theme.highlights.iter().enumerate().filter_map(|(i, (key, _))| ...)`, iterate `highlight_names.iter().enumerate().filter_map(|(i, key)| ...)`. The logic is pure string matching.
- Remove `HighlightId::style` method (returns `Option<HighlightStyle>` — theme-dependent).
- Remove `HighlightId::name` method (returns `Option<&str>` from theme — theme-dependent).
- Change visibility of `HighlightMap::new` from `pub(crate)` to `pub`.
- Change visibility of `HighlightId::is_default` from `pub(crate)` to `pub`.
- Move the test to `language_core`. Rewrite it to use `&[&str]` for highlight names instead of `SyntaxTheme`. The test assertions should check `map.get(N)` returns the expected `HighlightId` indices, not call `.name()` or `.style()`.

**Update `crates/language/src/highlight_map.rs`:** Replace the entire file with:
```rust
pub use language_core::highlight_map::{HighlightMap, HighlightId};
```
Plus add back the theme-dependent methods as extension functions or a trait impl:
```rust
use theme::SyntaxTheme;
use gpui::HighlightStyle;

// Bridge functions for code that needs theme resolution
impl HighlightId {
    pub fn style(&self, theme: &SyntaxTheme) -> Option<HighlightStyle> {
        theme.highlights.get(self.0 as usize).map(|entry| entry.1)
    }

    pub fn name<'a>(&self, theme: &'a SyntaxTheme) -> Option<&'a str> {
        theme.highlights.get(self.0 as usize).map(|e| e.0.as_str())
    }
}
```
Note: Rust allows adding inherent methods to a type in downstream crates only if the type is in the same crate. Since `HighlightId` moves to `language_core`, the `language` crate can't add inherent methods. Options:
- **Option A**: Define an extension trait `HighlightIdExt` in `language` with `style()` and `name()` methods. Downstream code adds `use language::HighlightIdExt;`.
- **Option B**: Keep `style()` and `name()` as free functions in `language`: `pub fn highlight_style(id: HighlightId, theme: &SyntaxTheme) -> Option<HighlightStyle>`.
- **Option C**: Keep `style()` and `name()` on `HighlightId` in `language_core` but behind a `theme` feature flag that adds the `theme` dependency. When `language` enables this feature, the methods are available.

**Recommendation: Option A** (extension trait). It's idiomatic Rust and the blast radius is tiny — only 2 crates import `HighlightId` directly.

Also update `Language::set_theme` in `crates/language/src/language.rs` (line ~2160) to bridge between `SyntaxTheme` and the new `HighlightMap::new`:
```rust
pub fn set_theme(&self, theme: &SyntaxTheme) {
    if let Some(grammar) = self.grammar.as_ref()
        && let Some(highlights_config) = &grammar.highlights_config
    {
        let highlight_names: Vec<&str> = theme.highlights.iter().map(|(name, _)| name.as_str()).collect();
        *grammar.highlight_map.lock() = HighlightMap::new(highlights_config.query.capture_names(), &highlight_names);
    }
}
```

**Update `crates/language_core/src/language_core.rs`:**
```rust
pub mod highlight_map;
pub use highlight_map::{HighlightMap, HighlightId};
```

### 2.2 Move `Diagnostic` and `DiagnosticSourceKind`

**Source:** `crates/language/src/diagnostic.rs` (created in Phase 0.5)

**Move** the file to `crates/language_core/src/diagnostic.rs`. Update imports to use `lsp` types directly.

**Update `crates/language_core/src/language_core.rs`:**
```rust
mod diagnostic;
pub use diagnostic::{Diagnostic, DiagnosticSourceKind};
```

**Update `crates/language/src/language.rs`:** Change `mod diagnostic; pub use diagnostic::*;` to `pub use language_core::{Diagnostic, DiagnosticSourceKind};`. Remove the local `diagnostic.rs` file.

### 2.3 Move `LanguageName` and `LanguageId`

**Source:** `LanguageName` is in `crates/language/src/language_registry.rs` (line 39). `LanguageId` is in `crates/language/src/language.rs` (line 1339).

**Add to `crates/language_core/src/language_core.rs`:**

`LanguageName` — copy the struct and all its impls. It uses `SharedString` from GPUI (allowed). Copy all trait impls: `From`, `AsRef`, `Borrow`, `PartialEq`, `Display`, `Hash`, `Ord`, etc. Keep `lsp_id()` method.

`LanguageId` — copy the struct. Move `NEXT_LANGUAGE_ID` static to `language_core`. The `new()` method uses `AtomicUsize`.

**Update `crates/language/src/language_registry.rs`:** Remove `LanguageName` definition, add `use language_core::LanguageName;`.

**Update `crates/language/src/language.rs`:** Remove `LanguageId` definition and `NEXT_LANGUAGE_ID` static, add `use language_core::LanguageId;`. Add `pub use language_core::{LanguageName, LanguageId};` to re-exports.

### 2.4 Move `LanguageConfig`, `LanguageMatcher`, and all config sub-types

**Source:** `crates/language/src/language.rs` lines ~833-1250

This is a large block. Create `crates/language_core/src/language_config.rs` containing:

- `LanguageConfig` struct (all ~40 fields)
- `LanguageMatcher` struct
- `BracketPairConfig`, `BracketPair`, `BracketPairContent` and their impls (including `Deserialize` impls)
- `BlockCommentConfig` and its custom `Deserialize` impl
- `DecreaseIndentConfig`
- `OrderedListConfig`
- `TaskListConfig`
- `JsxTagAutoCloseConfig`
- `WrapCharactersConfig`
- `LanguageConfigOverride` and `Override<T>` enum
- `LanguageScope` struct and its `impl` block (methods like `line_comment_prefixes`, `block_comment`, `brackets`, `word_characters`, etc.)
- Helper functions: `deserialize_regex`, `serialize_regex`, `deserialize_regex_vec`, `regex_json_schema`, `regex_vec_json_schema`, `auto_indent_using_last_non_empty_line_default`
- `LanguageConfig::default()` impl
- `LanguageConfig::load()` method
- `LanguageConfig::FILE_NAME` constant

**Dependencies to handle:**

- `SoftWrap` — currently re-exported from `settings`. Define a `SoftWrap` enum in `language_core` with the same variants and `Serialize`/`Deserialize` representation:
  ```rust
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
  #[serde(rename_all = "snake_case")]
  pub enum SoftWrap { None, EditorWidth, PreferredLineLength, Bounded }
  ```
  Omit `PreferLine` (deprecated) — or include it with `#[serde(alias)]` for backwards compat. Check the settings_content definition for the exact serde representation.
  In `crates/language/src/language_settings.rs`, change the re-export to reference `language_core::SoftWrap` instead of `settings::SoftWrap`. If the `settings` crate has a different `SoftWrap` type, provide `From` conversions in `language`.

- `LanguageServerName` — used in `LanguageConfig::scope_opt_in_language_servers` and `LanguageConfigOverride::opt_into_language_servers`. This type is from the `lsp` crate (allowed dependency). No change needed.

- `SharedString` — used in `LanguageConfig::debuggers: IndexSet<SharedString>`. GPUI is allowed. No change needed.

- `schemars::JsonSchema` — add `schemars` to `language_core` deps. No issue.

- `default_true` — imported from `util::serde`. Already available since `util` is a dep.

- `LanguageScope` references `Arc<Language>` — but `Language` is NOT in `language_core`. **Solution**: `LanguageScope` stays in `language` (the integration crate). It depends on the full `Language` struct which has `context_provider` etc. The `LanguageScope` methods read from `self.language.config` and `self.language.grammar` — both types are in core — but the struct itself holds `Arc<Language>` which is integration-layer. So: move all config/matcher types to core, keep `LanguageScope` in `language`.

**Update `crates/language_core/src/language_core.rs`:**
```rust
mod language_config;
pub use language_config::*;
```

**Update `crates/language/src/language.rs`:** Remove the moved types, add `pub use language_core::{LanguageConfig, LanguageMatcher, BracketPairConfig, ...};`. Keep `LanguageScope` locally.

### 2.5 Move `Grammar`, `GrammarId`, and all query config types

**Source:** `crates/language/src/language.rs` lines ~1357-1532 (types) and ~1648-2070 (query builder methods) and ~2372-2420 (Grammar impl)

**Create `crates/language_core/src/grammar.rs`** containing:

**Types:**
- `Grammar` struct (with all fields)
- `GrammarId` struct and `NEXT_GRAMMAR_ID` static
- `HighlightsConfig`
- `OutlineConfig`
- `IndentConfig` (currently private — keep private in core)
- `InjectionConfig` (private)
- `RedactionConfig` (private)
- `RunnableConfig` (private)
- `RunnableCapture` enum (private — change `Named(SharedString)` to `Named(SharedString)` since GPUI allowed, or use `Arc<str>` for decoupling)
- `OverrideConfig` (private)
- `OverrideEntry` (private)
- `BracketsConfig` (private)
- `BracketsPatternConfig` (private)
- `InjectionPatternConfig` (private)
- `TextObjectConfig`
- `TextObject` enum and its `from_capture_name`, `around` methods
- `DebuggerTextObject` enum and its `from_capture_name` method
- `DebugVariablesConfig`
- `ImportsConfig`
- `Capture` enum (private helper)
- `populate_capture_indices` function (private)

**Query builder methods** — currently on `impl Language`, move to `impl Grammar`:

Each method currently does `let grammar = self.grammar_mut()?;` or `let query = Query::new(&self.expect_grammar()?.ts_language, source)?;`. In the core crate, these become methods on `Grammar` directly (since we have `&mut self`).

Transform each method. Example for `with_highlights_query`:
```rust
impl Grammar {
    pub fn with_highlights_query(mut self, source: &str) -> Result<Self> {
        let query = Query::new(&self.ts_language, source)?;
        let mut identifier_capture_indices = Vec::new();
        for name in ["variable", "constant", "constructor", "function", ...] {
            identifier_capture_indices.extend(query.capture_index_for_name(name));
        }
        self.highlights_config = Some(HighlightsConfig { query, identifier_capture_indices });
        Ok(self)
    }
}
```

For methods that need `language_name` for error logging (outline, brackets, indents, injection, imports, redaction, text_object, debug_variables), add a `language_name: &str` parameter:
```rust
pub fn with_outline_query(mut self, source: &str, language_name: &str) -> Result<Self> { ... }
```

For `with_override_query` — this is the most complex. It reads `config.overrides`, `config.brackets.disabled_scopes_by_bracket_ix`, and `config.scope_opt_in_language_servers`. Pass these as parameters:
```rust
pub fn with_override_query(
    mut self,
    source: &str,
    language_name: &str,
    overrides: &HashMap<String, LanguageConfigOverride>,
    brackets: &mut BracketPairConfig,
    scope_opt_in_language_servers: &[LanguageServerName],
) -> Result<Self> { ... }
```
Note: `with_override_query` currently mutates `self.config.brackets.disabled_scopes_by_bracket_ix` (clears it at the end). The caller needs to handle this. Pass `brackets: &mut BracketPairConfig` so the method can clear the disabled scopes.

Add a convenience method that takes `LanguageQueries` and a `LanguageConfig`:
```rust
pub fn with_queries(mut self, queries: LanguageQueries, config: &mut LanguageConfig) -> Result<Self> {
    let name = config.name.as_ref();
    if let Some(q) = queries.highlights { self = self.with_highlights_query(&q)?; }
    if let Some(q) = queries.brackets { self = self.with_brackets_query(&q, name)?; }
    if let Some(q) = queries.indents { self = self.with_indents_query(&q, name)?; }
    if let Some(q) = queries.outline { self = self.with_outline_query(&q, name)?; }
    if let Some(q) = queries.injections { self = self.with_injection_query(&q, name)?; }
    if let Some(q) = queries.overrides {
        self = self.with_override_query(
            &q, name, &config.overrides, &mut config.brackets, &config.scope_opt_in_language_servers,
        )?;
    }
    if let Some(q) = queries.redactions { self = self.with_redaction_query(&q, name)?; }
    if let Some(q) = queries.runnables { self = self.with_runnable_query(&q)?; }
    if let Some(q) = queries.text_objects { self = self.with_text_object_query(&q, name)?; }
    if let Some(q) = queries.debugger { self = self.with_debug_variables_query(&q, name)?; }
    if let Some(q) = queries.imports { self = self.with_imports_query(&q, name)?; }
    Ok(self)
}
```

**Other Grammar methods to move:**
- `Grammar::id()` → move as-is
- `Grammar::highlight_map()` → move as-is
- `Grammar::highlight_id_for_name()` → move as-is
- `Grammar::debug_variables_config()` → move as-is
- `Grammar::imports_config()` → move as-is
- `Grammar::parse_text()` — this takes `&Rope` (from `text` crate). This method CANNOT move to core because `Rope` is not available. It stays in `language`. Add a more generic method to core:
  ```rust
  pub fn parse_with<F>(&self, callback: F, old_tree: Option<Tree>) -> Tree
  where F: FnMut(usize, tree_sitter::Point) -> &[u8]
  ```
  Or just leave `parse_text` in `language` where it's only used by `Language::highlight_text`.

**Parser and QueryCursor pools** — move to `language_core/src/grammar.rs`:
- `PARSERS` static
- `QUERY_CURSORS` static
- `WASM_ENGINE` static
- `with_parser` function
- `with_query_cursor` function

**Update `crates/language_core/src/language_core.rs`:**
```rust
mod grammar;
pub use grammar::*;
// Selectively re-export pub types; keep private types private
```

**Update `crates/language/src/language.rs`:**
- Remove all moved types and functions
- Add `pub use language_core::{Grammar, GrammarId, HighlightsConfig, OutlineConfig, TextObjectConfig, DebugVariablesConfig, ImportsConfig, TextObject, DebuggerTextObject, with_parser, with_query_cursor};`
- Update `Language::with_queries` to delegate to `Grammar::with_queries`:
  ```rust
  pub fn with_queries(mut self, queries: LanguageQueries) -> Result<Self> {
      if let Some(grammar) = self.grammar.take() {
          let mut grammar = Arc::try_unwrap(grammar).context("cannot mutate grammar")?;
          grammar = grammar.with_queries(queries, &mut self.config)?;
          self.grammar = Some(Arc::new(grammar));
      }
      Ok(self)
  }
  ```
- Keep `Language::highlight_text` and `Grammar::parse_text` in `language` (they use `Rope`).

### 2.6 Move `LanguageQueries` and `QUERY_FILENAME_PREFIXES`

**Source:** `crates/language/src/language_registry.rs` lines ~235-270

**Move to `crates/language_core/src/language_core.rs`** (or a dedicated file if preferred):
- `QUERY_FILENAME_PREFIXES` constant
- `LanguageQueries` struct

These have no external dependencies beyond `std::borrow::Cow`.

**Update `crates/language/src/language_registry.rs`:** Remove definitions, add `use language_core::{LanguageQueries, QUERY_FILENAME_PREFIXES};`.

**Update `crates/language/src/language.rs` re-exports:** Change the existing `pub use language_registry::{..., LanguageQueries, QUERY_FILENAME_PREFIXES};` to `pub use language_core::{LanguageQueries, QUERY_FILENAME_PREFIXES};`.

### 2.7 Move `CodeLabel`, `CodeLabelBuilder`, and `Symbol`

**Source:** `crates/language/src/language.rs` lines ~202-206 (Symbol) and ~813-830 (CodeLabel, CodeLabelBuilder) and ~2430-2490 (CodeLabel impls)

**Move to `crates/language_core/src/language_core.rs`** (or a file like `code_label.rs`):

- `Symbol` struct — uses `String`, `lsp::SymbolKind`, `Option<String>`. Clean.
- `CodeLabel` struct — uses `String`, `Vec<(Range<usize>, HighlightId)>`, `Range<usize>`. Clean (HighlightId is already in core).
- `CodeLabelBuilder` struct and its `impl` (push_str, respan_filter_range, build methods).

**Do NOT move `CodeLabel::fallback_for_completion`** — it takes `&lsp::CompletionItem` and `Option<&Language>`. `Language` is not in core. This method stays in `language`. Move it to a separate `impl CodeLabel` block in `crates/language/src/language.rs`.

**Update `crates/language/src/language.rs`:** Remove moved definitions, add re-exports from `language_core`.

### 2.8 Move `Diagnostic` to `language_core`

Already prepared in Phase 0.5. Move `crates/language/src/diagnostic.rs` to `crates/language_core/src/diagnostic.rs`.

Update both crate roots accordingly.

### 2.9 Move LSP adapter traits and types

**Source:** `crates/language/src/language.rs` lines ~220-700

**Create `crates/language_core/src/lsp_adapter.rs`** containing:

- `LspAdapter` trait (with all methods, including the modified signatures from Phase 0)
- `LspAdapterDelegate` trait
- `LspInstaller` trait
- `DynLspInstaller` trait
- Blanket impl `DynLspInstaller for LI where LI: LspInstaller`
- `CachedLspAdapter` struct and its full `impl`
- `FakeLspAdapter` struct (behind `#[cfg(any(test, feature = "test-support"))]`)
- `PromptResponseContext` struct
- `LanguageServerBinaryLocations` type alias
- `ToLspPosition` trait
- `BinaryStatus` enum
- `LanguageServerStatusUpdate` enum
- `ServerHealth` enum

**Dependencies to handle:**

- `WorktreeId` — currently from `settings`. Define in `language_core`:
  ```rust
  #[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord, Serialize)]
  pub struct WorktreeId(usize);

  impl WorktreeId {
      pub fn from_usize(id: usize) -> Self { Self(id) }
      pub fn to_usize(self) -> usize { self.0 }
      pub fn from_proto(id: u64) -> Self { Self(id as usize) }
      pub fn to_proto(self) -> u64 { self.0 as u64 }
  }
  ```
  Update `crates/settings/src/settings.rs` to re-export `language_core::WorktreeId` instead of defining its own — OR keep both and provide `From` conversions. The simpler approach: have `settings` depend on `language_core` and re-export its `WorktreeId`. But check if this creates a circular dependency. If `language_core` does NOT depend on `settings`, then `settings → language_core` is fine.

  **Alternative:** Define `WorktreeId` in `util` (which both `settings` and `language_core` already depend on) and have both re-export it. This avoids any new inter-crate dependency.

- `Entity<Buffer>` in `Location` — `Location` does NOT move. It stays in `language`.
- `Buffer` in `process_diagnostics` — removed in Phase 0.1.
- `Diagnostic` in `retain_old_diagnostic` — moved to core in 2.8.
- `Language` in `label_for_completion`, `label_for_symbol` — the `LspAdapter` trait methods reference `Arc<Language>`. If `Language` stays in `language`, these methods can't be in core. **Solution**: These methods take `Arc<Language>` where `Language` is not in core. Make the trait generic over the language type, OR change these methods to not take `Language` but instead take `Arc<Grammar>` + `LanguageName` (which is what they actually need for constructing CodeLabels). Check what the implementations actually use from `Language` — likely just `language.grammar()` and `language.name()`. If so, change the parameter to `(&Arc<Grammar>, &LanguageName)` or define a small trait `LanguageInfo { fn grammar() -> &Grammar; fn name() -> LanguageName; }` that both core and integration can implement.

  **Check concrete implementations:** The default `labels_for_completions` calls `self.label_for_completion(item, language)` for each item. The `label_for_completion` default returns `None`. Real implementations (e.g., RustLspAdapter) use `language` to get the grammar for highlight_id_for_name, which is on `Grammar`. So changing the parameter to `&Grammar` is likely sufficient. Audit all overrides to confirm.

  **Pragmatic approach if the above is too invasive:** Keep `LspAdapter` in `language` (not core). The trait is implemented by types in the `languages` crate which already depends on `language`. Ex doesn't need to implement `LspAdapter` — it would interact with language servers directly via the `lsp` crate. Moving adapter traits is a nice-to-have but not essential for the extraction.

  **Recommendation:** Start by moving all adapter types EXCEPT the `LspAdapter` trait itself and `CachedLspAdapter` to `language_core`. Move the data types (`BinaryStatus`, `ServerHealth`, `LanguageServerStatusUpdate`, `PromptResponseContext`, `LanguageServerBinaryLocations`). Move `LspAdapterDelegate`, `LspInstaller`, `DynLspInstaller` (they don't reference `Language`). Move `FakeLspAdapter`. Then assess whether `LspAdapter` can follow by changing `Arc<Language>` to `Arc<Grammar>` in the label methods.

### 2.10 Move `Toolchain` and related types

**Source:** `crates/language/src/toolchain.rs`

**Move to `crates/language_core/src/toolchain.rs`:**
- `Toolchain` struct
- `ToolchainScope` enum (uses `Arc<Path>`, `Arc<RelPath>` — both from util/std)
- `ToolchainList` struct
- `ToolchainMetadata` struct
- `LanguageToolchainStore` trait (uses `AsyncApp`, `WorktreeId`, `RelPath`)
- `LocalLanguageToolchainStore` trait
- The blanket impl `LanguageToolchainStore for T where T: LocalLanguageToolchainStore`

**Keep in `language/src/toolchain.rs`:**
- `ToolchainLister` trait (uses `fs::Fs` which we're keeping out of core)

**Update `crates/language/src/toolchain.rs`:** Remove moved types, re-export from `language_core`. Keep `ToolchainLister` locally.

**Update `crates/language/src/language.rs` re-exports:** Update to re-export from `language_core` for the moved types.

### 2.11 Move `ManifestName` and related types

**Source:** `crates/language/src/manifest.rs`

**Move to `crates/language_core/src/manifest.rs`:**
- `ManifestName` struct and all impls
- `ManifestQuery` struct
- `ManifestProvider` trait
- `ManifestDelegate` trait

These depend on: `SharedString` (GPUI, allowed), `WorktreeId` (now in core), `RelPath` (util).

**Update `crates/language/src/manifest.rs`:** Replace with re-exports.

### 2.12 Move point conversion functions

**Source:** `crates/language/src/language.rs` lines ~2755-2790

The functions `point_to_lsp`, `point_from_lsp`, `range_to_lsp`, `range_from_lsp` use `PointUtf16` and `Unclipped<PointUtf16>` from the `text` crate.

**Option A (recommended):** Define a minimal `PointUtf16` in `language_core`:
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PointUtf16 {
    pub row: u32,
    pub column: u32,
}

impl PointUtf16 {
    pub fn new(row: u32, column: u32) -> Self { Self { row, column } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unclipped<T>(pub T);
```

Move the four conversion functions to `language_core`. The `language` crate provides `From` conversions between `text::PointUtf16` / `language_core::PointUtf16`.

**Option B:** Keep these 4 functions in `language`. They're small and only used in integration contexts. Ex can define its own trivially.

**Recommendation:** Option B for now. These are 30 lines of trivial code. Not worth the type-bridging complexity.

### 2.13 Move `PLAIN_TEXT` static

**Source:** `crates/language/src/language.rs` lines ~133-186

`PLAIN_TEXT` creates a `Language` struct. If `Language` stays in `language`, then `PLAIN_TEXT` stays in `language`.

If we want a core equivalent, add a `PLAIN_TEXT_CONFIG: LazyLock<LanguageConfig>` to `language_core` that just provides the config. The integration layer creates the `Language` from it.

**Recommendation:** Keep `PLAIN_TEXT` in `language`. Not essential for core.

### 2.14 Final `language_core` crate root

After all moves, `crates/language_core/src/language_core.rs` should look approximately like:

```rust
mod diagnostic;
mod grammar;
mod highlight_map;
mod language_config;
mod lsp_adapter;
mod manifest;
mod toolchain;

// Re-exports
pub use diagnostic::{Diagnostic, DiagnosticSourceKind};
pub use grammar::*;  // Grammar, GrammarId, query configs, with_parser, with_query_cursor, etc.
pub use highlight_map::{HighlightMap, HighlightId};
pub use language_config::*;  // LanguageConfig, LanguageMatcher, BracketPairConfig, etc.
pub use lsp_adapter::*;  // adapter traits, BinaryStatus, ServerHealth, etc.
pub use manifest::{ManifestName, ManifestQuery, ManifestProvider, ManifestDelegate};
pub use toolchain::{
    Toolchain, ToolchainScope, ToolchainList, ToolchainMetadata,
    LanguageToolchainStore, LocalLanguageToolchainStore,
};

// Top-level items
mod language_name;
pub use language_name::{LanguageName, LanguageId};

// LanguageQueries and QUERY_FILENAME_PREFIXES
mod queries;
pub use queries::{LanguageQueries, QUERY_FILENAME_PREFIXES};

// CodeLabel and Symbol
mod code_label;
pub use code_label::{CodeLabel, CodeLabelBuilder, Symbol};

// Point types (if we go with Option A from 2.12)
// mod point;
// pub use point::{PointUtf16, Unclipped, point_to_lsp, point_from_lsp, range_to_lsp, range_from_lsp};

// WorktreeId (if defined here rather than in util)
// mod worktree_id;
// pub use worktree_id::WorktreeId;

// SoftWrap enum
// Defined in language_config.rs alongside LanguageConfig
```

The exact file organization can be adjusted by the implementing agent. The key constraint is: no file in `language_core` imports from `text`, `theme`, `settings`, `rpc`, `task`, `fs`, `clock`, `sum_tree`, or `fuzzy`.

### 2.15 Update `language` crate to depend on `language_core`

**`crates/language/Cargo.toml`:** Add `language_core.workspace = true` to `[dependencies]`.

**`crates/language/src/language.rs`:** Add comprehensive re-exports at the top:
```rust
pub use language_core::*;
```
Or be selective to avoid conflicts. The goal is that all existing `use language::SomeType` statements across the workspace continue to resolve. Go through every type that moved and ensure it's re-exported.

**Remove moved code** from all files in `crates/language/src/`.

**Update internal imports** within `language` to use `language_core::` or `crate::` as appropriate.

### 2.16 Verify everything compiles

```bash
cargo check -p language_core
cargo check -p language
cargo test -p language_core
cargo test -p language
cargo build -p zed
```

Fix any compilation errors. Common issues:
- Visibility: types that were `pub(crate)` in `language` and are now in `language_core` need to be `pub` in core and may need visibility adjustments in `language`.
- Orphan rule: `language` can't add inherent methods to `language_core` types. Use extension traits where needed.
- Feature flags: `test-support` features need to be coordinated between the two crates.

---

## Phase 3: Create the `grammars` crate

### 3.1 Create crate skeleton

Create `crates/grammars/Cargo.toml`:

```toml
[package]
name = "grammars"
version = "0.1.0"
edition = "2024"
publish = false

[lib]
path = "src/grammars.rs"

[dependencies]
language_core.workspace = true
rust-embed.workspace = true
anyhow.workspace = true
log.workspace = true
toml.workspace = true
serde.workspace = true

[features]
load-grammars = [
    "dep:tree-sitter-bash",
    "dep:tree-sitter-c",
    "dep:tree-sitter-cpp",
    "dep:tree-sitter-css",
    "dep:tree-sitter-diff",
    "dep:tree-sitter-go",
    "dep:tree-sitter-go-mod",
    "dep:tree-sitter-gowork",
    "dep:tree-sitter-jsdoc",
    "dep:tree-sitter-json",
    "dep:tree-sitter-md",
    "dep:tree-sitter-python",
    "dep:tree-sitter-regex",
    "dep:tree-sitter-rust",
    "dep:tree-sitter-typescript",
    "dep:tree-sitter-yaml",
    "dep:tree-sitter-gitcommit",
]
test-support = ["load-grammars"]

[dependencies.tree-sitter-bash]
workspace = true
optional = true
# ... repeat for all tree-sitter grammar crates
```

Add to workspace `Cargo.toml`:
- Add `"crates/grammars"` to `members`
- Add `grammars = { path = "crates/grammars" }` to `[workspace.dependencies]`

### 3.2 Move language data files

Move the language subdirectories (`.scm` files + `config.toml` files) from `crates/languages/src/` to `crates/grammars/src/`.

The directories to move (only the non-`.rs` data files):
`bash/`, `c/`, `cpp/`, `css/`, `diff/`, `gitcommit/`, `go/`, `gomod/`, `gowork/`, `javascript/`, `jsdoc/`, `json/`, `jsonc/`, `markdown/`, `markdown-inline/`, `python/`, `regex/`, `rust/`, `tsx/`, `typescript/`, `yaml/`, `zed-keybind-context/`

For each directory, copy only: `config.toml`, `*.scm` files, and any other non-`.rs` data files (like `semantic_token_rules.json`).

### 3.3 Implement the grammars crate

Create `crates/grammars/src/grammars.rs`:

```rust
use rust_embed::RustEmbed;
use std::{borrow::Cow, str};

#[derive(RustEmbed)]
#[folder = "src/"]
#[exclude = "*.rs"]
struct GrammarDir;

/// Register all built-in native tree-sitter grammars.
/// Call this before loading language configs/queries.
pub fn register_native_grammars(
    register: &mut dyn FnMut(&str, tree_sitter::Language),
) {
    #[cfg(feature = "load-grammars")]
    {
        register("bash", tree_sitter_bash::LANGUAGE.into());
        register("c", tree_sitter_c::LANGUAGE.into());
        // ... all grammars, same list as languages/src/lib.rs
    }
}

/// Load the config.toml for a given language name.
pub fn load_config(name: &str) -> anyhow::Result<language_core::LanguageConfig> {
    let config_path = format!("{}/config.toml", name);
    let config_bytes = GrammarDir::get(&config_path)
        .ok_or_else(|| anyhow::anyhow!("no config for language {name}"))?;
    let config_str = str::from_utf8(&config_bytes.data)?;
    toml::from_str(config_str).map_err(Into::into)
}

/// Load all .scm query files for a given language name.
pub fn load_queries(name: &str) -> language_core::LanguageQueries {
    let mut queries = language_core::LanguageQueries::default();
    for (prefix, field_fn) in language_core::QUERY_FILENAME_PREFIXES {
        for path in GrammarDir::iter() {
            let path_str = path.as_ref();
            if let Some(rest) = path_str.strip_prefix(name) {
                let rest = rest.strip_prefix('/').unwrap_or(rest);
                if rest.starts_with(prefix) && rest.ends_with(".scm") {
                    if let Some(content) = GrammarDir::get(path_str) {
                        let source = str::from_utf8(&content.data).unwrap();
                        let field = field_fn(&mut queries);
                        match field {
                            Some(existing) => {
                                *field = Some(Cow::Owned(format!("{existing}\n{source}")));
                            }
                            None => {
                                *field = Some(Cow::Owned(source.to_string()));
                            }
                        }
                    }
                }
            }
        }
    }
    queries
}
```

Note: The exact implementation of `load_queries` should match the existing logic in `crates/languages/src/lib.rs` (around line 282). Read that function carefully and replicate its behavior, especially the handling of multiple query files with the same prefix (they get concatenated with newlines).

### 3.4 Update `languages` crate to depend on `grammars`

**`crates/languages/Cargo.toml`:** Add `grammars.workspace = true`.

**`crates/languages/src/lib.rs`:**
- Remove the `LanguageDir` RustEmbed struct
- Remove the `load_queries` and `load_config` functions
- Remove the `.scm`/`.toml` data directories from `src/` (they now live in `grammars`)
- In `init()`:
  - Replace `languages.register_native_grammars([...])` with a call to `grammars::register_native_grammars(...)` or keep the explicit list (since `languages` already has the tree-sitter deps).
  - Replace calls to `load_queries(name)` with `grammars::load_queries(name)`
  - Replace calls to `load_config(name)` with `grammars::load_config(name)` (if used)

### 3.5 Verify

```bash
cargo test -p grammars
cargo test -p languages
cargo build -p zed
```

---

## Phase 4: Verification and cleanup

### 4.1 Full test suite

```bash
cargo test -p language_core
cargo test -p language
cargo test -p languages
cargo test -p grammars
cargo build -p zed
```

### 4.2 Check for unused dependencies

In `crates/language/Cargo.toml`, check if any dependencies are now unused since their types moved to `language_core`. For example, `parking_lot` might only have been used for `Grammar::highlight_map` which moved. Remove unused deps.

### 4.3 Check for dead code warnings

```bash
cargo check -p language_core 2>&1 | grep "warning"
cargo check -p language 2>&1 | grep "warning"
```

Fix any warnings (unused imports, dead code, etc.).

### 4.4 Verify downstream crates

Build the full workspace:
```bash
cargo build --workspace
```

If any crate fails because it was importing a type from `language` that's now in `language_core` and the re-export is missing, add the re-export to `language`.

---

## Phase 5 (Future): Registry split for shared memory

This phase is NOT part of the initial extraction but documents the approach for when Ex integrates into Zed.

The core registry would live in `language_core`:

```rust
pub struct LanguageRegistryCore {
    state: RwLock<LanguageRegistryCoreState>,
    executor: BackgroundExecutor,
}

struct LanguageRegistryCoreState {
    languages_by_name: HashMap<LanguageName, LanguageEntry>,
    grammars: HashMap<Arc<str>, AvailableGrammar>,
    subscription: (watch::Sender<()>, watch::Receiver<()>),
    version: usize,
}
```

The Zed `LanguageRegistry` in `language` wraps this:

```rust
pub struct LanguageRegistry {
    core: Arc<LanguageRegistryCore>,
    lsp_adapters: RwLock<LspAdapterState>,
    theme: RwLock<Option<Arc<Theme>>>,
    language_settings: RwLock<AllLanguageSettingsContent>,
}
```

Ex accesses `Arc<LanguageRegistryCore>` directly. When Ex runs inside Zed, both Ex and Zed share the same `LanguageRegistryCore` instance — same grammars, same compiled queries, same memory.

This split is straightforward once the types are extracted (Phase 2 is the prerequisite). It can be done in a follow-up PR.

---

## Execution notes

- Work on a branch from the commit Ex is pinned to: `0f1f0f9272b2e963f48736fc7e62dd4ec5d8d9e7`.
- Each phase should be a separate commit (or set of commits) for reviewability.
- Phase 0 changes are safe to land independently — they're pure cleanups.
- Phase 2 is the bulk of the work. Steps 2.1-2.8 are straightforward type moves. Step 2.9 (LSP adapters) is the most complex and may need to be done partially.
- If compilation gets stuck, the escape hatch is always: keep the problematic type in `language` and re-export it. The goal is progress, not perfection.
- After Phase 2, run `./script/clippy` (not `cargo clippy`) per the project rules.
