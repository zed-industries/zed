# Language Model Provider Extensions - Implementation Guide

## Purpose

This document provides a detailed guide for completing the implementation of Language Model Provider Extensions in Zed. It explains what has been done, what remains, and how to complete the work.

For the full design and rationale, see [language_model_provider_extensions_plan.md](./language_model_provider_extensions_plan.md).

## Core Design Principle

**Extensions handle ALL provider-specific logic.** This means:
- Thought signatures (Anthropic)
- Reasoning effort parameters (OpenAI o-series)
- Cache control markers
- Parallel tool calls
- SSE/streaming format parsing
- Any other provider-specific features

Zed's core should have **zero knowledge** of these details. The extension API must be generic enough that extensions can implement any provider without Zed changes.

---

## Current Status: STREAMING API COMPLETE ✅

The core plumbing and streaming API are now complete. Extensions can:
1. Declare LLM providers in their manifest
2. Be queried for providers and models at load time
3. Have their providers registered with the `LanguageModelRegistry`
4. Have their providers unregistered when the extension is unloaded
5. Stream completions using the new polling-based API

**What's NOT done yet:**
- Credential UI prompt support (`llm_request_credential` returns false)
- Model refresh mechanism
- A working test extension that demonstrates the feature (requires WASM build)
- End-to-end testing with a real extension

---

## What Has Been Completed

### 1. WIT Interface Definition ✅

**Location:** `crates/extension_api/wit/since_v0.7.0/`

Created all WIT files for v0.7.0:
- `llm-provider.wit` - Core LLM types (ProviderInfo, ModelInfo, CompletionRequest, CompletionEvent, etc.)
- `extension.wit` - Updated with LLM exports/imports

Key types in `llm-provider.wit`:
```wit
record provider-info {
    id: string,
    name: string,
    icon: option<string>,
}

record model-info {
    id: string,
    name: string,
    max-token-count: u64,
    max-output-tokens: option<u64>,
    capabilities: model-capabilities,
    is-default: bool,
    is-default-fast: bool,
}

variant completion-event {
    started,
    text(string),
    thinking(thinking-content),
    redacted-thinking(string),
    tool-use(tool-use),
    tool-use-json-parse-error(tool-use-json-parse-error),
    stop(stop-reason),
    usage(token-usage),
    reasoning-details(string),
}
```

Key exports in `extension.wit`:
```wit
export llm-providers: func() -> list<provider-info>;
export llm-provider-models: func(provider-id: string) -> result<list<model-info>, string>;
export llm-provider-is-authenticated: func(provider-id: string) -> bool;
export llm-provider-authenticate: func(provider-id: string) -> result<_, string>;
export llm-stream-completion-start: func(provider-id: string, model-id: string, request: completion-request) -> result<string, string>;
export llm-stream-completion-next: func(stream-id: string) -> result<option<completion-event>, string>;
export llm-stream-completion-close: func(stream-id: string);
```

Note: The streaming API uses a polling-based approach with explicit stream IDs instead of a resource handle.
This avoids complexity with cross-boundary resource ownership in the WASM component model.

Key imports in `extension.wit`:
```wit
import llm-get-credential: func(provider-id: string) -> option<string>;
import llm-store-credential: func(provider-id: string, value: string) -> result<_, string>;
import llm-delete-credential: func(provider-id: string) -> result<_, string>;
import llm-get-env-var: func(name: string) -> option<string>;
```

### 2. Extension Manifest Changes ✅

**Location:** `crates/extension/src/extension_manifest.rs`

Added these types:
```rust
pub struct LanguageModelProviderManifestEntry {
    pub name: String,
    pub icon: Option<String>,
    pub models: Vec<LanguageModelManifestEntry>,
    pub auth: Option<LanguageModelAuthConfig>,
}

pub struct LanguageModelManifestEntry {
    pub id: String,
    pub name: String,
    pub max_token_count: u64,
    pub max_output_tokens: Option<u64>,
    pub supports_images: bool,
    pub supports_tools: bool,
    pub supports_thinking: bool,
}

pub struct LanguageModelAuthConfig {
    pub env_var: Option<String>,
    pub credential_label: Option<String>,
}
```

Added to `ExtensionManifest`:
```rust
pub language_model_providers: BTreeMap<Arc<str>, LanguageModelProviderManifestEntry>,
```

### 3. Host-Side Provider/Model Structs ✅

**Location:** `crates/extension_host/src/wasm_host/llm_provider.rs`

Created `ExtensionLanguageModelProvider` implementing `LanguageModelProvider`:
- Wraps a `WasmExtension` and `LlmProviderInfo`
- Delegates to extension calls for authentication, model listing, etc.
- Returns `ExtensionLanguageModel` instances
- Implements `LanguageModelProviderState` for UI observation

Created `ExtensionLanguageModel` implementing `LanguageModel`:
- Wraps extension + model info
- Implements `stream_completion` by calling extension's `llm-stream-completion`
- Converts between Zed's `LanguageModelRequest` and WIT's `CompletionRequest`
- Handles streaming via polling-based approach with explicit stream IDs

**Key implementation details:**
- The `stream_completion` method uses a polling loop that calls `llm_stream_completion_start`, then repeatedly calls `llm_stream_completion_next` until the stream is complete, and finally calls `llm_stream_completion_close` to clean up
- Credential storage uses gpui's `cx.read_credentials()`, `cx.write_credentials()`, and `cx.delete_credentials()` APIs
- The `new()` method now accepts a `models: Vec<LlmModelInfo>` parameter to populate available models at registration time

### 4. Extension Host Proxy ✅

**Location:** `crates/extension/src/extension_host_proxy.rs`

Added `ExtensionLanguageModelProviderProxy` trait:
```rust
pub type LanguageModelProviderRegistration = Box<dyn FnOnce(&mut App) + Send + Sync + 'static>;

pub trait ExtensionLanguageModelProviderProxy: Send + Sync + 'static {
    fn register_language_model_provider(
        &self,
        provider_id: Arc<str>,
        register_fn: LanguageModelProviderRegistration,
        cx: &mut App,
    );

    fn unregister_language_model_provider(&self, provider_id: Arc<str>, cx: &mut App);
}
```

The proxy uses a boxed closure pattern. This allows `extension_host` to create the `ExtensionLanguageModelProvider` (which requires `WasmExtension`), while letting `language_models` handle the actual registry registration.

### 5. Proxy Implementation ✅

**Location:** `crates/language_models/src/extension.rs`

```rust
pub struct ExtensionLanguageModelProxy {
    registry: Entity<LanguageModelRegistry>,
}

impl ExtensionLanguageModelProviderProxy for ExtensionLanguageModelProxy {
    fn register_language_model_provider(
        &self,
        _provider_id: Arc<str>,
        register_fn: LanguageModelProviderRegistration,
        cx: &mut App,
    ) {
        register_fn(cx);
    }

    fn unregister_language_model_provider(&self, provider_id: Arc<str>, cx: &mut App) {
        self.registry.update(cx, |registry, cx| {
            registry.unregister_provider(LanguageModelProviderId::from(provider_id), cx);
        });
    }
}
```

The proxy is registered during `language_models::init()`.

### 6. Extension Loading Wiring ✅

**Location:** `crates/extension_host/src/extension_host.rs`

In `extensions_updated()`:

**Unloading (around line 1217):**
```rust
for provider_id in extension.manifest.language_model_providers.keys() {
    let full_provider_id: Arc<str> = format!("{}:{}", extension_id, provider_id).into();
    self.proxy.unregister_language_model_provider(full_provider_id, cx);
}
```

**Loading (around line 1383):**
After loading a wasm extension, we query for LLM providers and models:
```rust
if !extension.manifest.language_model_providers.is_empty() {
    let providers_result = wasm_extension
        .call(|ext, store| {
            async move { ext.call_llm_providers(store).await }.boxed()
        })
        .await;

    if let Ok(Ok(providers)) = providers_result {
        for provider_info in providers {
            // Query for models...
            let models_result = wasm_extension.call(...).await;
            // Store provider_info and models for registration
        }
    }
}
```

Then during registration (around line 1511):
```rust
for (provider_info, models) in llm_providers_with_models {
    let provider_id: Arc<str> = format!("{}:{}", manifest.id, provider_info.id).into();
    this.proxy.register_language_model_provider(
        provider_id,
        Box::new(move |cx: &mut App| {
            let provider = Arc::new(ExtensionLanguageModelProvider::new(
                wasm_ext, pinfo, mods, cx,
            ));
            language_model::LanguageModelRegistry::global(cx).update(
                cx,
                |registry, cx| {
                    registry.register_provider(provider, cx);
                },
            );
        }),
        cx,
    );
}
```

### 7. Extension API Updates ✅

**Location:** `crates/extension_api/src/extension_api.rs`

- Updated `wit_bindgen::generate!` to use `./wit/since_v0.7.0`
- Added LLM type re-exports (prefixed with `Llm` for clarity)
- Added LLM methods to `Extension` trait with default implementations
- Added `wit::Guest` implementations for LLM functions

The default implementations ensure backward compatibility:
```rust
fn llm_providers(&self) -> Vec<LlmProviderInfo> {
    Vec::new()  // Extensions without LLM providers return empty
}

fn llm_provider_models(&self, _provider_id: &str) -> Result<Vec<LlmModelInfo>, String> {
    Ok(Vec::new())
}

fn llm_stream_completion_start(...) -> Result<String, String> {
    Err("`llm_stream_completion_start` not implemented".to_string())
}
fn llm_stream_completion_next(stream_id: &str) -> Result<Option<LlmCompletionEvent>, String> {
    Err("`llm_stream_completion_next` not implemented".to_string())
}
fn llm_stream_completion_close(stream_id: &str) { /* cleanup */ }
```

### 8. Test Files Updated ✅

Added `language_model_providers: BTreeMap::default()` to all test manifests:
- `crates/extension/src/extension_manifest.rs` (test module)
- `crates/extension_host/src/extension_store_test.rs`
- `crates/extension_host/src/capability_granter.rs` (test module)
- `crates/extension_host/benches/extension_compilation_benchmark.rs`

---

## What Remains To Be Done

### Task 1: Test the Streaming Completion Flow (HIGH PRIORITY) - ARCHITECTURE UPDATED ✅

The streaming API has been updated to use a polling-based approach instead of a resource handle pattern.
This was necessary because the original design had a fundamental issue: the `completion-stream` resource
was defined in an imported interface but returned from an exported function, creating ownership ambiguity.

**New API:**
- `llm-stream-completion-start` - Returns a stream ID (string)
- `llm-stream-completion-next` - Poll for the next event using the stream ID
- `llm-stream-completion-close` - Clean up the stream when done

**Still needs testing:**
1. Create a test extension that implements a simple LLM provider
2. Verify the polling-based streaming works correctly through the WASM boundary
3. Test error handling and edge cases

**Location to test:** `crates/extension_host/src/wasm_host/llm_provider.rs` - the `stream_completion` method on `ExtensionLanguageModel`.

### Task 2: Credential UI Prompt Support (MEDIUM PRIORITY)

**Location:** `crates/extension_host/src/wasm_host/wit/since_v0_7_0.rs`

The `llm_request_credential` host function currently returns `Ok(Ok(false))`:
```rust
async fn llm_request_credential(
    &mut self,
    _provider_id: String,
    _credential_type: llm_provider::CredentialType,
    _label: String,
    _placeholder: String,
) -> wasmtime::Result<Result<bool, String>> {
    // TODO: Implement actual UI prompting
    Ok(Ok(false))
}
```

**What needs to happen:**
1. Show a dialog to the user asking for the credential
2. Wait for user input
3. Return `true` if provided, `false` if cancelled
4. The extension can then use `llm_store_credential` to save it

This requires UI work and async coordination with gpui windows.

### Task 3: Handle Model Refresh (LOW PRIORITY - can be follow-up)

Currently models are only queried once at registration time. Options for improvement:

1. Add a refresh mechanism that re-queries `call_llm_provider_models`
2. Add a notification mechanism where extensions can signal that models have changed
3. Automatic refresh on authentication

**Recommendation:** Start with refresh-on-authentication as a fast-follow.

### Task 4: Create a Test Extension (LOW PRIORITY - but very useful)

**Note:** Creating a working test extension requires building a WASM component, which needs:
1. The `wasm32-wasip1` Rust target: `rustup target add wasm32-wasip1`
2. Building with: `cargo build --target wasm32-wasip1 --release`
3. The resulting `.wasm` file must be placed in the extension directory

The existing `extensions/test-extension` has a pre-built WASM file checked in. To test LLM
provider functionality, either:
- Rebuild the test-extension WASM with LLM provider code
- Create a new extension and build it locally

Example test extension that demonstrates the LLM provider API:

```
extensions/test-llm-provider/
├── extension.toml
├── Cargo.toml
└── src/
    └── lib.rs
```

**extension.toml:**
```toml
id = "test-llm-provider"
name = "Test LLM Provider"
version = "0.1.0"
schema_version = 1

[language_model_providers.test-provider]
name = "Test Provider"
```

**src/lib.rs:**
```rust
use zed_extension_api::{self as zed, *};

use std::collections::HashMap;
use std::sync::Mutex;

struct TestExtension {
    streams: Mutex<HashMap<String, Vec<LlmCompletionEvent>>>,
    next_stream_id: Mutex<u64>,
}

impl zed::Extension for TestExtension {
    fn new() -> Self {
        Self {
            streams: Mutex::new(HashMap::new()),
            next_stream_id: Mutex::new(0),
        }
    }
    
    fn llm_providers(&self) -> Vec<LlmProviderInfo> {
        vec![LlmProviderInfo {
            id: "test-provider".into(),
            name: "Test Provider".into(),
            icon: None,
        }]
    }
    
    fn llm_provider_models(&self, _provider_id: &str) -> Result<Vec<LlmModelInfo>, String> {
        Ok(vec![LlmModelInfo {
            id: "test-model".into(),
            name: "Test Model".into(),
            max_token_count: 4096,
            max_output_tokens: Some(1024),
            capabilities: LlmModelCapabilities {
                supports_images: false,
                supports_tools: false,
                supports_tool_choice_auto: false,
                supports_tool_choice_any: false,
                supports_tool_choice_none: false,
                supports_thinking: false,
                tool_input_format: LlmToolInputFormat::JsonSchema,
            },
            is_default: true,
            is_default_fast: true,
        }])
    }
    
    fn llm_stream_completion_start(
        &mut self,
        _provider_id: &str,
        _model_id: &str,
        _request: &LlmCompletionRequest,
    ) -> Result<String, String> {
        // Create a simple response with test events
        let events = vec![
            LlmCompletionEvent::Started,
            LlmCompletionEvent::Text("Hello, ".into()),
            LlmCompletionEvent::Text("world!".into()),
            LlmCompletionEvent::Stop(LlmStopReason::EndTurn),
        ];
        
        let mut id = self.next_stream_id.lock().unwrap();
        let stream_id = format!("stream-{}", *id);
        *id += 1;
        
        self.streams.lock().unwrap().insert(stream_id.clone(), events);
        Ok(stream_id)
    }
    
    fn llm_stream_completion_next(
        &mut self,
        stream_id: &str,
    ) -> Result<Option<LlmCompletionEvent>, String> {
        let mut streams = self.streams.lock().unwrap();
        if let Some(events) = streams.get_mut(stream_id) {
            if events.is_empty() {
                Ok(None)
            } else {
                Ok(Some(events.remove(0)))
            }
        } else {
            Err(format!("Unknown stream: {}", stream_id))
        }
    }
    
    fn llm_stream_completion_close(&mut self, stream_id: &str) {
        self.streams.lock().unwrap().remove(stream_id);
    }
}

zed::register_extension!(TestExtension);
```

---

## File-by-File Checklist

### Completed ✅

- [x] `crates/extension_api/wit/since_v0.7.0/llm-provider.wit` - LLM types defined
- [x] `crates/extension_api/wit/since_v0.7.0/extension.wit` - LLM exports/imports added
- [x] `crates/extension_api/src/extension_api.rs` - Extension trait + Guest impl updated for v0.7.0
- [x] `crates/extension/src/extension_manifest.rs` - Manifest types added
- [x] `crates/extension/src/extension_host_proxy.rs` - Proxy trait added
- [x] `crates/extension_host/src/wasm_host/llm_provider.rs` - Provider/Model structs created
- [x] `crates/extension_host/src/wasm_host/wit.rs` - LLM types exported, Extension enum updated
- [x] `crates/extension_host/src/wasm_host/wit/since_v0_7_0.rs` - Host trait implementations
- [x] `crates/extension_host/src/wasm_host/wit/since_v0_6_0.rs` - Rewritten to use latest types
- [x] `crates/extension_host/src/extension_host.rs` - Wired up LLM provider registration/unregistration
- [x] `crates/extension_host/Cargo.toml` - Dependencies added
- [x] `crates/language_models/src/extension.rs` - Proxy implementation
- [x] `crates/language_models/src/language_models.rs` - Proxy registration
- [x] `crates/language_models/Cargo.toml` - Extension dependency added

### Should Implement (Follow-up PRs)

- [ ] `llm_request_credential` UI implementation
- [ ] Model refresh mechanism
- [ ] Test extension for validation
- [ ] Documentation for extension authors

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Extension Host                               │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │                    extensions_updated()                       │    │
│  │                                                               │    │
│  │  1. Load WasmExtension                                        │    │
│  │  2. Query llm_providers() and llm_provider_models()          │    │
│  │  3. Call proxy.register_language_model_provider()            │    │
│  └───────────────────────────┬───────────────────────────────────┘    │
│                              │                                        │
│  ┌───────────────────────────▼───────────────────────────────────┐    │
│  │              ExtensionLanguageModelProvider                    │    │
│  │  - Wraps WasmExtension                                        │    │
│  │  - Implements LanguageModelProvider                           │    │
│  │  - Creates ExtensionLanguageModel instances                   │    │
│  └───────────────────────────┬───────────────────────────────────┘    │
│                              │                                        │
│  ┌───────────────────────────▼───────────────────────────────────┐    │
│  │                ExtensionLanguageModel                          │    │
│  │  - Implements LanguageModel                                    │    │
│  │  - stream_completion() calls extension via WASM               │    │
│  └───────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              │ Proxy (boxed closure)
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                       Language Models Crate                          │
│  ┌───────────────────────────────────────────────────────────────┐   │
│  │              ExtensionLanguageModelProxy                       │   │
│  │  - Implements ExtensionLanguageModelProviderProxy             │   │
│  │  - Calls register_fn closure                                  │   │
│  │  - Unregisters from LanguageModelRegistry                     │   │
│  └───────────────────────────┬───────────────────────────────────┘   │
│                              │                                       │
│  ┌───────────────────────────▼───────────────────────────────────┐   │
│  │                LanguageModelRegistry                           │   │
│  │  - Stores all providers (built-in + extension)                │   │
│  │  - Provides models to UI                                      │   │
│  └───────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Key Code Patterns

### 1. Provider ID Format

Provider IDs are formatted as `{extension_id}:{provider_id}` to ensure uniqueness:

```rust
let provider_id: Arc<str> = format!("{}:{}", manifest.id, provider_info.id).into();
```

### 2. Triple-Nested Result Handling

When calling extension methods, results are nested:
- Outer `Result`: from channel operations (anyhow error)
- Middle `Result`: from WASM call (anyhow error)  
- Inner `Result<T, String>`: from extension logic

```rust
let models_result = wasm_extension.call(...).await;

let models: Vec<LlmModelInfo> = match models_result {
    Ok(Ok(Ok(models))) => models,
    Ok(Ok(Err(e))) => { /* extension returned error */ }
    Ok(Err(e)) => { /* WASM call failed */ }
    Err(e) => { /* channel operation failed */ }
};
```

### 3. Polling-Based Streaming Pattern

The streaming API uses explicit stream IDs with polling instead of resource handles:

```rust
// Start the stream and get an ID
let stream_id = ext.call_llm_stream_completion_start(store, provider_id, model_id, request).await?;

// Poll for events in a loop
loop {
    match ext.call_llm_stream_completion_next(store, &stream_id).await? {
        Ok(Some(event)) => { /* process event */ }
        Ok(None) => break,  // Stream complete
        Err(e) => { /* handle error */ }
    }
}

// Clean up
ext.call_llm_stream_completion_close(store, &stream_id).await;
```

This pattern avoids the complexity of cross-boundary resource ownership in the WASM component model.

### 4. Default Trait Implementations

All LLM methods in the `Extension` trait have defaults so existing extensions continue to work:

```rust
fn llm_providers(&self) -> Vec<LlmProviderInfo> {
    Vec::new()  // No providers by default
}
```

---

## Common Pitfalls

1. **Type confusion:** WIT bindgen creates NEW types for each version. `Completion` from v0.6.0 bindgen is different from v0.7.0. This is why we map older interfaces to `latest::`.

2. **Import paths:** After `pub use self::zed::extension::*;`, types are available without prefix. Types in sub-interfaces (like `lsp::CompletionKind`) need explicit imports.

3. **Async closures:** Extension calls use `extension.call(|ext, store| async move { ... }.boxed())` pattern. The closure must be `'static + Send`.

4. **Stream ID management:** Extensions must track their active streams using the stream IDs returned from `llm_stream_completion_start`. The host will call `llm_stream_completion_close` when done.

5. **Result nesting:** `extension.call(...)` wraps the closure's return type in `Result<T>`, so if the closure returns `Result<Result<X, String>>`, you get `Result<Result<Result<X, String>>>`. Unwrap carefully!

6. **Proxy type boundaries:** The `extension` crate shouldn't depend on `extension_host`. The proxy trait uses a boxed closure to pass the registration logic without needing to share types.

7. **Resource ownership in WIT:** Be careful when defining resources in imported interfaces but returning them from exported functions. This creates ownership ambiguity. The streaming API was changed to use polling to avoid this issue.

---

## Testing

All existing tests pass:
```bash
cargo test -p extension_host --lib
# 3 tests pass

./script/clippy
# No warnings
```

To test the full flow manually:
1. Create a test extension with LLM provider
2. Build and install it
3. Check if it appears in the model selector
4. Try making a completion request

---

## Relevant Files for Reference

### How providers are registered
- `crates/language_model/src/registry.rs` - `LanguageModelRegistry::register_provider`

### How other extension proxies work
- `crates/extension/src/extension_host_proxy.rs` - the proxy pattern
- `crates/project/src/context_server_store/extension.rs` - context server proxy implementation

### How extensions are loaded
- `crates/extension_host/src/extension_host.rs` - `extensions_updated` method

### WasmExtension call pattern
- `crates/extension_host/src/wasm_host.rs` - `WasmExtension::call` method

---

## Questions for Follow-up

1. **Where should configuration UI live?** The current implementation uses an empty config view. Should extension providers have configurable settings?

2. **How to handle extension reload?** Currently, in-flight completions will fail if the extension is unloaded. Should we add graceful handling?

3. **Should there be rate limiting?** If an extension's provider misbehaves, should Zed throttle or disable it?

4. **Icon support:** The `provider_info.icon` field exists but `icon()` on the provider returns `ui::IconName::ZedAssistant`. Should we add custom icon support?