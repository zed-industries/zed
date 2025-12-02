# Language Model Provider Extensions Plan

## Executive Summary

This document outlines a comprehensive plan to introduce **Language Model Provider Extensions** to Zed. This feature will allow third-party developers to create extensions that register new language model providers, enabling users to select and use custom language models in Zed's AI features (Agent, inline assist, commit message generation, etc.).

## Table of Contents

1. [Current Architecture Overview](#current-architecture-overview)
2. [Goals and Requirements](#goals-and-requirements)
3. [Proposed Architecture](#proposed-architecture)
4. [Implementation Phases](#implementation-phases)
5. [WIT Interface Design](#wit-interface-design)
6. [Extension Manifest Changes](#extension-manifest-changes)
7. [Migration Plan for Built-in Providers](#migration-plan-for-built-in-providers)
8. [Testing Strategy](#testing-strategy)
9. [Security Considerations](#security-considerations)
10. [Appendix: Provider-Specific Requirements](#appendix-provider-specific-requirements)

---

## Current Architecture Overview

### Key Components

#### `language_model` crate (`crates/language_model/`)
- **`LanguageModel` trait** (`src/language_model.rs:580-718`): Core trait defining model capabilities
  - `id()`, `name()`, `provider_id()`, `provider_name()`
  - `supports_images()`, `supports_tools()`, `supports_tool_choice()`
  - `max_token_count()`, `max_output_tokens()`
  - `count_tokens()` - async token counting
  - `stream_completion()` - the main completion streaming method
  - `cache_configuration()` - optional prompt caching config

- **`LanguageModelProvider` trait** (`src/language_model.rs:743-764`): Provider registration
  - `id()`, `name()`, `icon()`
  - `default_model()`, `default_fast_model()`
  - `provided_models()`, `recommended_models()`
  - `is_authenticated()`, `authenticate()`
  - `configuration_view()` - UI for provider configuration
  - `reset_credentials()`

- **`LanguageModelRegistry`** (`src/registry.rs`): Global registry for providers
  - `register_provider()` / `unregister_provider()`
  - Model selection and configuration
  - Event emission for UI updates

#### `language_models` crate (`crates/language_models/`)
Contains all built-in provider implementations:
- `provider/anthropic.rs` - Anthropic Claude models
- `provider/cloud.rs` - Zed Cloud (proxied models)
- `provider/google.rs` - Google Gemini models
- `provider/open_ai.rs` - OpenAI GPT models
- `provider/ollama.rs` - Local Ollama models
- `provider/deepseek.rs` - DeepSeek models
- `provider/open_router.rs` - OpenRouter aggregator
- `provider/bedrock.rs` - AWS Bedrock
- And more...

#### Extension System (`crates/extension_host/`, `crates/extension_api/`)
- **WIT interface** (`extension_api/wit/since_v0.6.0/`): WebAssembly Interface Types definitions
- **WASM host** (`extension_host/src/wasm_host.rs`): Executes extension WASM modules
- **Extension trait** (`extension/src/extension.rs`): Rust trait for extensions
- **HTTP client** (`extension_api/src/http_client.rs`): Existing HTTP capability for extensions

### Request/Response Flow

```
User Request
    ↓
LanguageModelRequest (crates/language_model/src/request.rs)
    ↓
Provider-specific conversion (e.g., into_anthropic(), into_open_ai())
    ↓
HTTP API call (provider-specific crate)
    ↓
Stream of provider-specific events
    ↓
Event mapping to LanguageModelCompletionEvent
    ↓
Consumer (Agent, Inline Assist, etc.)
```

### Key Data Structures

```rust
// Request
pub struct LanguageModelRequest {
    pub thread_id: Option<String>,
    pub prompt_id: Option<String>,
    pub intent: Option<CompletionIntent>,
    pub mode: Option<CompletionMode>,
    pub messages: Vec<LanguageModelRequestMessage>,
    pub tools: Vec<LanguageModelRequestTool>,
    pub tool_choice: Option<LanguageModelToolChoice>,
    pub stop: Vec<String>,
    pub temperature: Option<f32>,
    pub thinking_allowed: bool,
}

// Completion Events
pub enum LanguageModelCompletionEvent {
    Queued { position: usize },
    Started,
    UsageUpdated { amount: usize, limit: usize },
    ToolUseLimitReached,
    Stop(StopReason),
    Text(String),
    Thinking { text: String, signature: Option<String> },
    RedactedThinking { data: String },
    ToolUse(LanguageModelToolUse),
    ToolUseJsonParseError { ... },
    StartMessage { message_id: Option<String> },
    ReasoningDetails(serde_json::Value),
    UsageUpdate(TokenUsage),
}
```

---

## Goals and Requirements

### Primary Goals

1. **Extensibility**: Allow any developer to add new LLM providers via extensions
2. **Parity**: Extension-based providers should have feature parity with built-in providers
3. **Performance**: Minimize overhead from WASM boundary crossings during streaming
4. **Security**: Sandbox API key handling and network access appropriately
5. **User Experience**: Seamless integration with existing model selectors and configuration UI

### Functional Requirements

1. Extensions can register one or more language model providers
2. Extensions can define multiple models per provider
3. Extensions handle authentication (API keys, OAuth, etc.)
4. Extensions implement the streaming completion API
5. Extensions can specify model capabilities (tools, images, thinking, etc.)
6. Extensions can provide token counting logic
7. Extensions can provide configuration UI components
8. Extensions receive full request context for API customization

### Non-Functional Requirements

1. Streaming should feel as responsive as built-in providers
2. Extension crashes should not crash Zed
3. API keys should never be logged or exposed
4. Extensions should be able to make arbitrary HTTP requests
5. Settings should persist across sessions

---

## Proposed Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────────────────┐
│                         Zed Application                          │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                  LanguageModelRegistry                       ││
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  ││
│  │  │ Built-in     │  │ Extension    │  │ Extension        │  ││
│  │  │ Providers    │  │ Provider A   │  │ Provider B       │  ││
│  │  │ (Anthropic,  │  │ (WASM)       │  │ (WASM)           │  ││
│  │  │  OpenAI...)  │  │              │  │                  │  ││
│  │  └──────────────┘  └──────────────┘  └──────────────────┘  ││
│  └─────────────────────────────────────────────────────────────┘│
│                              ↑                                   │
│                              │                                   │
│  ┌───────────────────────────┴─────────────────────────────────┐│
│  │              ExtensionLanguageModelProvider                  ││
│  │  ┌─────────────────────────────────────────────────────────┐││
│  │  │ • Bridges WASM extension to LanguageModelProvider trait │││
│  │  │ • Manages streaming across WASM boundary                │││
│  │  │ • Handles credential storage via credentials_provider   │││
│  │  │ • Provides configuration UI scaffolding                 │││
│  │  └─────────────────────────────────────────────────────────┘││
│  └─────────────────────────────────────────────────────────────┘│
│                              ↑                                   │
│  ┌───────────────────────────┴─────────────────────────────────┐│
│  │                    WasmHost / WasmExtension                  ││
│  │  • Executes WASM module                                      ││
│  │  • Provides WIT interface for LLM operations                 ││
│  │  • HTTP client for API calls                                 ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

### New Components

#### 1. `ExtensionLanguageModelProvider`

A new struct in `extension_host` that implements `LanguageModelProvider` and wraps a WASM extension:

```rust
pub struct ExtensionLanguageModelProvider {
    extension: WasmExtension,
    provider_info: ExtensionLlmProviderInfo,
    state: Entity<ExtensionLlmProviderState>,
}

struct ExtensionLlmProviderState {
    is_authenticated: bool,
    available_models: Vec<ExtensionLanguageModel>,
}
```

#### 2. `ExtensionLanguageModel`

Implements `LanguageModel` trait, delegating to WASM calls:

```rust
pub struct ExtensionLanguageModel {
    extension: WasmExtension,
    model_info: ExtensionLlmModelInfo,
    provider_id: LanguageModelProviderId,
}
```

#### 3. WIT Interface Extensions

New WIT definitions for LLM provider functionality (see [WIT Interface Design](#wit-interface-design)).

---

## Implementation Phases

### Phase 1: Foundation (2-3 weeks)

**Goal**: Establish the core infrastructure for extension-based LLM providers.

#### Tasks

1. **Define WIT interface for LLM providers** (`extension_api/wit/since_v0.7.0/llm-provider.wit`)
   - Provider metadata (id, name, icon)
   - Model definitions (id, name, capabilities, limits)
   - Credential management hooks
   - Completion request/response types

2. **Create `ExtensionLanguageModelProvider`** (`extension_host/src/wasm_host/llm_provider.rs`)
   - Implement `LanguageModelProvider` trait
   - Handle provider registration/unregistration
   - Basic authentication state management

3. **Create `ExtensionLanguageModel`** (`extension_host/src/wasm_host/llm_model.rs`)
   - Implement `LanguageModel` trait
   - Simple synchronous completion (non-streaming initially)

4. **Update `ExtensionManifest`** (`extension/src/extension_manifest.rs`)
   - Add `language_model_providers` field
   - Parse provider configuration from `extension.toml`

5. **Update extension loading** (`extension_host/src/extension_host.rs`)
   - Detect LLM provider declarations in manifest
   - Register providers with `LanguageModelRegistry`

#### Deliverables
- Extensions can register a provider that appears in model selector
- Basic (non-streaming) completions work
- Manual testing with a test extension

### Phase 2: Streaming Support (2-3 weeks)

**Goal**: Enable efficient streaming completions across the WASM boundary.

#### Tasks

1. **Design streaming protocol**
   - Option A: Chunked responses via repeated WASM calls
   - Option B: Callback-based streaming (preferred)
   - Option C: Shared memory buffer with polling

2. **Implement streaming in WIT**
   ```wit
   resource completion-stream {
       next-event: func() -> result<option<completion-event>, string>;
   }
   
   export stream-completion: func(
       provider-id: string,
       model-id: string,
       request: completion-request
   ) -> result<completion-stream, string>;
   ```

3. **Implement `http-response-stream` integration**
   - Extensions already have access to `fetch-stream`
   - Need to parse SSE/chunked responses in WASM
   - Map to completion events

4. **Update `ExtensionLanguageModel::stream_completion`**
   - Bridge WASM completion-stream to Rust BoxStream
   - Handle backpressure and cancellation

5. **Performance optimization**
   - Batch small events to reduce WASM boundary crossings
   - Consider using shared memory for large payloads

#### Deliverables
- Streaming completions work with acceptable latency
- Performance benchmarks vs built-in providers

### Phase 3: Full Feature Parity (2-3 weeks)

**Goal**: Support all advanced features that built-in providers have.

#### Tasks

1. **Tool/Function calling support**
   - Add tool definitions to request
   - Parse tool use events from response
   - Handle tool results in follow-up requests

2. **Image support**
   - Pass image data in messages
   - Handle base64 encoding/size limits

3. **Thinking/reasoning support** (for Claude-like models)
   - `Thinking` and `RedactedThinking` events
   - Thought signatures for tool calls

4. **Token counting**
   - WIT interface for `count_tokens`
   - Allow extensions to provide custom tokenizers or call API

5. **Prompt caching configuration**
   - Cache control markers in messages
   - Cache configuration reporting

6. **Rate limiting and error handling**
   - Standard error types in WIT
   - Retry-after headers
   - Rate limit events

#### Deliverables
- Extension providers can use tools
- Extension providers can process images
- Full error handling parity

### Phase 4: Credential Management & Configuration UI (1-2 weeks)

**Goal**: Secure credential storage and user-friendly configuration.

#### Tasks

1. **Credential storage integration**
   - Use existing `credentials_provider` crate
   - Extensions request credentials via WIT
   - Credentials never exposed to WASM directly (only "is_authenticated" status)

2. **API key input flow**
   ```wit
   import request-credential: func(
       credential-type: credential-type,
       label: string,
       placeholder: string
   ) -> result<bool, string>;
   ```

3. **Configuration view scaffolding**
   - Generic configuration view that works for most providers
   - Extensions can provide additional settings via JSON schema
   - Settings stored in extension-specific namespace

4. **Environment variable support**
   - Allow specifying env var names for API keys
   - Read from environment on startup

#### Deliverables
- Secure API key storage
- Configuration UI for extension providers
- Environment variable fallback

### Phase 5: Testing & Documentation (1-2 weeks)

**Goal**: Comprehensive testing and developer documentation.

#### Tasks

1. **Integration tests**
   - Test extension loading and registration
   - Test streaming completions
   - Test error handling
   - Test credential management

2. **Performance tests**
   - Latency benchmarks
   - Memory usage under load
   - Comparison with built-in providers

3. **Example extensions**
   - Simple OpenAI-compatible provider
   - Provider with custom authentication
   - Provider with tool support

4. **Documentation**
   - Extension developer guide
   - API reference
   - Migration guide for custom providers

#### Deliverables
- Full test coverage
- Published documentation
- Example extensions in `extensions/` directory

### Phase 6: Migration of Built-in Providers (Optional, Long-term)

**Goal**: Prove the extension system by migrating one or more built-in providers.

#### Tasks

1. **Select candidate provider** (suggest: Ollama or LM Studio - simplest API)
2. **Create extension version**
3. **Feature parity testing**
4. **Performance comparison**
5. **Gradual rollout (feature flag)

---

## WIT Interface Design

### New File: `extension_api/wit/since_v0.7.0/llm-provider.wit`

```wit
interface llm-provider {
    /// Information about a language model provider
    record provider-info {
        /// Unique identifier for the provider (e.g., "my-extension.my-provider")
        id: string,
        /// Display name for the provider
        name: string,
        /// Icon name from Zed's icon set (optional)
        icon: option<string>,
    }

    /// Capabilities of a language model
    record model-capabilities {
        /// Whether the model supports image inputs
        supports-images: bool,
        /// Whether the model supports tool/function calling
        supports-tools: bool,
        /// Whether the model supports tool choice (auto/any/none)
        supports-tool-choice-auto: bool,
        supports-tool-choice-any: bool,
        supports-tool-choice-none: bool,
        /// Whether the model supports extended thinking
        supports-thinking: bool,
        /// The format for tool input schemas
        tool-input-format: tool-input-format,
    }

    /// Format for tool input schemas
    enum tool-input-format {
        json-schema,
        simplified,
    }

    /// Information about a specific model
    record model-info {
        /// Unique identifier for the model
        id: string,
        /// Display name for the model
        name: string,
        /// Maximum input token count
        max-token-count: u64,
        /// Maximum output tokens (optional)
        max-output-tokens: option<u64>,
        /// Model capabilities
        capabilities: model-capabilities,
        /// Whether this is the default model for the provider
        is-default: bool,
        /// Whether this is the default fast model
        is-default-fast: bool,
    }

    /// A message in a completion request
    record request-message {
        role: message-role,
        content: list<message-content>,
        cache: bool,
    }

    enum message-role {
        user,
        assistant,
        system,
    }

    /// Content within a message
    variant message-content {
        text(string),
        image(image-data),
        tool-use(tool-use),
        tool-result(tool-result),
        thinking(thinking-content),
        redacted-thinking(string),
    }

    record image-data {
        /// Base64-encoded image data
        source: string,
        /// Estimated dimensions
        width: option<u32>,
        height: option<u32>,
    }

    record tool-use {
        id: string,
        name: string,
        input: string, // JSON string
        thought-signature: option<string>,
    }

    record tool-result {
        tool-use-id: string,
        tool-name: string,
        is-error: bool,
        content: tool-result-content,
    }

    variant tool-result-content {
        text(string),
        image(image-data),
    }

    record thinking-content {
        text: string,
        signature: option<string>,
    }

    /// A tool definition
    record tool-definition {
        name: string,
        description: string,
        /// JSON Schema for input parameters
        input-schema: string,
    }

    /// Tool choice preference
    enum tool-choice {
        auto,
        any,
        none,
    }

    /// A completion request
    record completion-request {
        messages: list<request-message>,
        tools: list<tool-definition>,
        tool-choice: option<tool-choice>,
        stop-sequences: list<string>,
        temperature: option<f32>,
        thinking-allowed: bool,
        /// Maximum tokens to generate
        max-tokens: option<u64>,
    }

    /// Events emitted during completion streaming
    variant completion-event {
        /// Completion has started
        started,
        /// Text content
        text(string),
        /// Thinking/reasoning content
        thinking(thinking-content),
        /// Redacted thinking (encrypted)
        redacted-thinking(string),
        /// Tool use request
        tool-use(tool-use),
        /// Completion stopped
        stop(stop-reason),
        /// Token usage update
        usage(token-usage),
    }

    enum stop-reason {
        end-turn,
        max-tokens,
        tool-use,
    }

    record token-usage {
        input-tokens: u64,
        output-tokens: u64,
        cache-creation-input-tokens: option<u64>,
        cache-read-input-tokens: option<u64>,
    }

    /// A streaming completion response
    resource completion-stream {
        /// Get the next event from the stream.
        /// Returns None when the stream is complete.
        next-event: func() -> result<option<completion-event>, string>;
    }

    /// Credential types that can be requested
    enum credential-type {
        api-key,
        oauth-token,
    }
}
```

### Updates to `extension_api/wit/since_v0.7.0/extension.wit`

```wit
world extension {
    // ... existing imports ...
    import llm-provider;
    
    use llm-provider.{
        provider-info, model-info, completion-request, 
        completion-stream, credential-type
    };

    /// Returns information about language model providers offered by this extension
    export llm-providers: func() -> list<provider-info>;

    /// Returns the models available for a provider
    export llm-provider-models: func(provider-id: string) -> result<list<model-info>, string>;

    /// Check if the provider is authenticated
    export llm-provider-is-authenticated: func(provider-id: string) -> bool;

    /// Attempt to authenticate the provider
    export llm-provider-authenticate: func(provider-id: string) -> result<_, string>;

    /// Reset credentials for the provider
    export llm-provider-reset-credentials: func(provider-id: string) -> result<_, string>;

    /// Count tokens for a request
    export llm-count-tokens: func(
        provider-id: string, 
        model-id: string, 
        request: completion-request
    ) -> result<u64, string>;

    /// Stream a completion
    export llm-stream-completion: func(
        provider-id: string,
        model-id: string,
        request: completion-request
    ) -> result<completion-stream, string>;

    /// Request a credential from the user
    import llm-request-credential: func(
        provider-id: string,
        credential-type: credential-type,
        label: string,
        placeholder: string
    ) -> result<bool, string>;

    /// Get a stored credential
    import llm-get-credential: func(provider-id: string) -> option<string>;

    /// Store a credential
    import llm-store-credential: func(provider-id: string, value: string) -> result<_, string>;

    /// Delete a stored credential
    import llm-delete-credential: func(provider-id: string) -> result<_, string>;
}
```

---

## Extension Manifest Changes

### Updated `extension.toml` Schema

```toml
id = "my-llm-extension"
name = "My LLM Provider"
description = "Adds support for My LLM API"
version = "1.0.0"
schema_version = 1
authors = ["Developer <dev@example.com>"]
repository = "https://github.com/example/my-llm-extension"

[lib]
kind = "rust"
version = "0.7.0"

# New section for LLM providers
[language_model_providers.my-provider]
name = "My LLM"
icon = "sparkle"  # Optional, from Zed's icon set

# Optional: Default models to show even before API connection
[[language_model_providers.my-provider.models]]
id = "my-model-large"
name = "My Model Large"
max_token_count = 200000
max_output_tokens = 8192
supports_images = true
supports_tools = true

[[language_model_providers.my-provider.models]]
id = "my-model-small"
name = "My Model Small"
max_token_count = 100000
max_output_tokens = 4096
supports_images = false
supports_tools = true

# Optional: Environment variable for API key
[language_model_providers.my-provider.auth]
env_var = "MY_LLM_API_KEY"
credential_label = "API Key"
```

### `ExtensionManifest` Changes

```rust
// In extension/src/extension_manifest.rs

#[derive(Clone, Default, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct LanguageModelProviderManifestEntry {
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub models: Vec<LanguageModelManifestEntry>,
    #[serde(default)]
    pub auth: Option<LanguageModelAuthConfig>,
}

#[derive(Clone, Default, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct LanguageModelManifestEntry {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub max_token_count: u64,
    #[serde(default)]
    pub max_output_tokens: Option<u64>,
    #[serde(default)]
    pub supports_images: bool,
    #[serde(default)]
    pub supports_tools: bool,
    #[serde(default)]
    pub supports_thinking: bool,
}

#[derive(Clone, Default, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct LanguageModelAuthConfig {
    pub env_var: Option<String>,
    pub credential_label: Option<String>,
}

// Add to ExtensionManifest struct:
pub struct ExtensionManifest {
    // ... existing fields ...
    #[serde(default)]
    pub language_model_providers: BTreeMap<Arc<str>, LanguageModelProviderManifestEntry>,
}
```

---

## Migration Plan for Built-in Providers

This section analyzes each built-in provider and what would be required to implement them as extensions.

### Provider Comparison Matrix

| Provider | API Style | Auth Method | Special Features | Migration Complexity |
|----------|-----------|-------------|------------------|---------------------|
| Anthropic | REST/SSE | API Key | Thinking, Caching, Tool signatures | High |
| OpenAI | REST/SSE | API Key | Reasoning effort, Prompt caching | Medium |
| Google | REST/SSE | API Key | Thinking, Tool signatures | High |
| Ollama | REST/SSE | None (local) | Dynamic model discovery | Low |
| DeepSeek | REST/SSE | API Key | Reasoning mode | Medium |
| OpenRouter | REST/SSE | API Key | Reasoning details, Model routing | Medium |
| LM Studio | REST/SSE | None (local) | OpenAI-compatible | Low |
| Bedrock | AWS SDK | AWS Credentials | Multiple underlying providers | High |
| Zed Cloud | Zed Auth | Zed Account | Proxied providers | N/A (keep built-in) |

### Provider-by-Provider Analysis

#### Anthropic (`provider/anthropic.rs`)

**Current Implementation Highlights:**
- Uses `anthropic` crate for API types and streaming
- Custom event mapper (`AnthropicEventMapper`) for SSE → completion events
- Supports thinking/reasoning with thought signatures
- Prompt caching with cache control markers
- Beta headers for experimental features

**Extension Requirements:**
- Full SSE parsing in WASM
- Complex event mapping logic
- Thinking content with signatures
- Cache configuration reporting

**Unique Challenges:**
```rust
// Thought signatures in tool use
pub struct LanguageModelToolUse {
    pub thought_signature: Option<String>, // Anthropic-specific
}

// Thinking events with signatures
Thinking { text: String, signature: Option<String> }
```

**Migration Approach:**
1. Port `anthropic` crate types to extension-compatible structures
2. Implement SSE parser in extension (can use existing `fetch-stream`)
3. Map Anthropic events to generic completion events
4. Handle beta headers via custom HTTP headers

#### OpenAI (`provider/open_ai.rs`)

**Current Implementation Highlights:**
- Uses `open_ai` crate for API types
- Tiktoken-based token counting
- Parallel tool calls support
- Reasoning effort parameter (o1/o3 models)

**Extension Requirements:**
- SSE parsing (standard format)
- Token counting (could call API or use simplified estimate)
- Tool call aggregation across chunks

**Unique Challenges:**
```rust
// Reasoning effort for o-series models
pub reasoning_effort: Option<String>, // "low", "medium", "high"

// Prompt cache key (preview feature)
pub prompt_cache_key: Option<String>,
```

**Migration Approach:**
1. Standard SSE parsing
2. Token counting via API or tiktoken WASM port
3. Support reasoning_effort as model-specific config

#### Google/Gemini (`provider/google.rs`)

**Current Implementation Highlights:**
- Uses `google_ai` crate
- Different API structure from OpenAI/Anthropic
- Thinking support similar to Anthropic
- Tool signatures in function calls

**Extension Requirements:**
- Different request/response format
- Thinking content handling
- Tool signature preservation

**Unique Challenges:**
```rust
// Google uses different content structure
enum ContentPart {
    Text { text: String },
    InlineData { mime_type: String, data: String },
    FunctionCall { name: String, args: Value },
    FunctionResponse { name: String, response: Value },
}
```

**Migration Approach:**
1. Implement Google-specific request building
2. Map Google events to generic completion events
3. Handle thinking/function call signatures

#### Ollama (`provider/ollama.rs`)

**Current Implementation Highlights:**
- Local-only, no authentication needed
- Dynamic model discovery via API
- OpenAI-compatible chat endpoint
- Simple streaming format

**Extension Requirements:**
- API URL configuration
- Model list fetching
- Basic streaming

**Why This is a Good First Migration Target:**
- No authentication complexity
- Simple API format
- Dynamic model discovery is isolated
- Good test case for local provider pattern

**Migration Approach:**
1. Configuration for API URL
2. Model discovery endpoint call
3. OpenAI-compatible streaming

#### DeepSeek (`provider/deepseek.rs`)

**Current Implementation Highlights:**
- OpenAI-compatible API with extensions
- Reasoner model support
- Different handling for reasoning vs standard models

**Extension Requirements:**
- API key authentication
- Model-specific request modifications
- Reasoning content handling

**Migration Approach:**
1. Standard OpenAI-compatible base
2. Special handling for reasoner model
3. Temperature disabled for reasoning

#### OpenRouter (`provider/open_router.rs`)

**Current Implementation Highlights:**
- Aggregates multiple providers
- Dynamic model fetching
- Reasoning details preservation
- Tool call signatures

**Extension Requirements:**
- API key authentication
- Model list from API
- Reasoning details in responses

**Migration Approach:**
1. Model discovery from API
2. Standard OpenAI-compatible streaming
3. Preserve reasoning_details in events

#### LM Studio (`provider/lmstudio.rs`)

**Current Implementation Highlights:**
- Local-only, OpenAI-compatible
- Model discovery from API
- Simple configuration

**Why This is a Good First Migration Target:**
- No authentication
- OpenAI-compatible (reusable streaming code)
- Similar to Ollama

#### Bedrock (`provider/bedrock.rs`)

**Current Implementation Highlights:**
- AWS SDK-based authentication
- Multiple authentication methods (IAM, Profile, etc.)
- Proxies to Claude, Llama, etc.

**Extension Requirements:**
- AWS credential handling (complex)
- AWS Signature V4 signing
- Region configuration

**Why This Should Stay Built-in (Initially):**
- AWS credential management is complex
- SDK dependency not easily portable to WASM
- Security implications of AWS credentials in extensions

---

## Testing Strategy

### Unit Tests

```rust
// extension_host/src/wasm_host/llm_provider_tests.rs

#[gpui::test]
async fn test_extension_provider_registration(cx: &mut TestAppContext) {
    // Load test extension with LLM provider
    // Verify provider appears in registry
    // Verify models are listed correctly
}

#[gpui::test]
async fn test_extension_streaming_completion(cx: &mut TestAppContext) {
    // Create mock HTTP server
    // Load extension
    // Send completion request
    // Verify streaming events received correctly
}

#[gpui::test]
async fn test_extension_tool_calling(cx: &mut TestAppContext) {
    // Test tool definitions are passed correctly
    // Test tool use events are parsed
    // Test tool results can be sent back
}

#[gpui::test]
async fn test_extension_credential_management(cx: &mut TestAppContext) {
    // Test credential storage
    // Test credential retrieval
    // Test authentication state
}

#[gpui::test]
async fn test_extension_error_handling(cx: &mut TestAppContext) {
    // Test API errors are propagated correctly
    // Test rate limiting is handled
    // Test network errors are handled
}
```

### Integration Tests

```rust
// crates/extension_host/src/extension_store_test.rs (additions)

#[gpui::test]
async fn test_llm_extension_lifecycle(cx: &mut TestAppContext) {
    // Install extension with LLM provider
    // Verify provider registered
    // Configure credentials
    // Make completion request
    // Uninstall extension
    // Verify provider unregistered
}
```

### Manual Testing Checklist

1. **Provider Discovery**
   - [ ] Extension provider appears in model selector
   - [ ] Provider icon displays correctly
   - [ ] Models list correctly

2. **Authentication**
   - [ ] API key prompt appears when not authenticated
   - [ ] API key is stored securely
   - [ ] Environment variable fallback works
   - [ ] "Reset credentials" works

3. **Completions**
   - [ ] Basic text completion works
   - [ ] Streaming is smooth (no jank)
   - [ ] Long responses complete successfully
   - [ ] Cancellation works

4. **Advanced Features**
   - [ ] Tool calling works (Agent panel)
   - [ ] Image inputs work (if supported)
   - [ ] Thinking/reasoning displays correctly

5. **Error Handling**
   - [ ] Invalid API key shows error
   - [ ] Rate limiting shows appropriate message
   - [ ] Network errors are handled gracefully

6. **Performance**
   - [ ] First token latency acceptable (<500ms overhead)
   - [ ] Memory usage reasonable
   - [ ] No memory leaks on repeated requests

---

## Security Considerations

### Credential Handling

1. **Never expose raw credentials to WASM**
   - Extensions request credentials via import function
   - Zed stores credentials in secure storage (keychain/credential manager)
   - Extensions receive only "authenticated: true/false" status

2. **Credential scope isolation**
   - Each extension has its own credential namespace
   - Extensions cannot access other extensions' credentials
   - Provider ID is prefixed with extension ID

3. **Audit logging**
   - Log when credentials are accessed (not the values)
   - Log when credentials are modified

### Network Access

1. **HTTP request validation**
   - Extensions already have HTTP access via `fetch` / `fetch-stream`
   - Consider domain allowlisting for LLM providers
   - Log outbound requests for debugging

2. **Request/Response inspection**
   - API keys in headers should be redacted in logs
   - Response bodies may contain sensitive data

### Extension Sandbox

1. **WASM isolation**
   - Extensions run in WASM sandbox
   - Cannot access filesystem outside work directory
   - Cannot access other extensions' data

2. **Resource limits**
   - Memory limits per extension
   - CPU time limits (epoch-based interruption already exists)
   - Concurrent request limits

### Capability Requirements

```toml
# Extensions with LLM providers should declare:
[[capabilities]]
kind = "network:http"
domains = ["api.example.com"]  # Optional domain restriction

[[capabilities]]
kind = "credential:store"
```

---

## Appendix: Provider-Specific Requirements

### A. Anthropic Implementation Details

**Request Format:**
```json
{
  "model": "claude-sonnet-4-20250514",
  "max_tokens": 8192,
  "messages": [
    {"role": "user", "content": [{"type": "text", "text": "Hello"}]}
  ],
  "system": [{"type": "text", "text": "You are helpful"}],
  "tools": [...],
  "thinking": {"type": "enabled", "budget_tokens": 10000}
}
```

**SSE Events:**
- `message_start` - Contains message ID, model, usage
- `content_block_start` - Starts text/tool_use/thinking block
- `content_block_delta` - Incremental content (text_delta, input_json_delta, thinking_delta)
- `content_block_stop` - Block complete
- `message_delta` - Stop reason, final usage
- `message_stop` - End of message

**Special Considerations:**
- Beta headers for thinking: `anthropic-beta: interleaved-thinking-2025-05-14`
- Cache control markers in messages
- Thought signatures on tool uses

### B. OpenAI Implementation Details

**Request Format:**
```json
{
  "model": "gpt-4o",
  "messages": [
    {"role": "system", "content": "You are helpful"},
    {"role": "user", "content": "Hello"}
  ],
  "stream": true,
  "tools": [...],
  "max_completion_tokens": 4096
}
```

**SSE Events:**
```
data: {"choices":[{"delta":{"content":"Hello"}}]}
data: {"choices":[{"delta":{"tool_calls":[...]}}]}
data: [DONE]
```

**Special Considerations:**
- `reasoning_effort` for o-series models
- `parallel_tool_calls` option
- Token counting via tiktoken

### C. Google/Gemini Implementation Details

**Request Format:**
```json
{
  "contents": [
    {"role": "user", "parts": [{"text": "Hello"}]}
  ],
  "generationConfig": {
    "maxOutputTokens": 8192,
    "temperature": 0.7
  },
  "tools": [...]
}
```

**Response Format:**
```json
{
  "candidates": [{
    "content": {
      "parts": [
        {"text": "Response"},
        {"functionCall": {"name": "...", "args": {...}}}
      ]
    }
  }]
}
```

**Special Considerations:**
- Different streaming format (not SSE, line-delimited JSON)
- Tool signatures in function calls
- Thinking support similar to Anthropic

### D. OpenAI-Compatible Providers (Ollama, LM Studio, DeepSeek)

These providers can share common implementation:

**Shared Code:**
```rust
// In extension
fn stream_openai_compatible(
    api_url: &str,
    api_key: Option<&str>,
    request: CompletionRequest,
) -> Result<CompletionStream, String> {
    let request_body = build_openai_request(request);
    let stream = http_client::fetch_stream(HttpRequest {
        method: HttpMethod::Post,
        url: format!("{}/v1/chat/completions", api_url),
        headers: build_headers(api_key),
        body: Some(serde_json::to_vec(&request_body)?),
        redirect_policy: RedirectPolicy::NoFollow,
    })?;
    
    Ok(OpenAiStreamParser::new(stream))
}
```

### E. Example Extension: Simple OpenAI-Compatible Provider

```rust
// src/my_provider.rs
use zed_extension_api::{self as zed, Result};
use zed_extension_api::http_client::{HttpMethod, HttpRequest, RedirectPolicy};

struct MyLlmExtension {
    api_key: Option<String>,
}

impl zed::Extension for MyLlmExtension {
    fn new() -> Self {
        Self { api_key: None }
    }

    fn llm_providers(&self) -> Vec<zed::LlmProviderInfo> {
        vec![zed::LlmProviderInfo {
            id: "my-provider".into(),
            name: "My LLM Provider".into(),
            icon: Some("sparkle".into()),
        }]
    }

    fn llm_provider_models(&self, provider_id: &str) -> Result<Vec<zed::LlmModelInfo>> {
        Ok(vec![
            zed::LlmModelInfo {
                id: "my-model".into(),
                name: "My Model".into(),
                max_token_count: 128000,
                max_output_tokens: Some(4096),
                capabilities: zed::LlmModelCapabilities {
                    supports_images: true,
                    supports_tools: true,
                    ..Default::default()
                },
                is_default: true,
                is_default_fast: false,
            }
        ])
    }

    fn llm_provider_is_authenticated(&self, _provider_id: &str) -> bool {
        self.api_key.is_some() || std::env::var("MY_API_KEY").is_ok()
    }

    fn llm_provider_authenticate(&mut self, provider_id: &str) -> Result<()> {
        if let Some(key) = zed::llm_get_credential(provider_id)? {
            self.api_key = Some(key);
            return Ok(());
        }
        
        if zed::llm_request_credential(
            provider_id,
            zed::CredentialType::ApiKey,
            "API Key",
            "Enter your API key",
        )? {
            self.api_key = zed::llm_get_credential(provider_id)?;
        }
        
        Ok(())
    }

    fn llm_stream_completion(
        &self,
        provider_id: &str,
        model_id: &str,
        request: zed::LlmCompletionRequest,
    ) -> Result<zed::LlmCompletionStream> {
        let api_key = self.api_key.as_ref()
            .or_else(|| std::env::var("MY_API_KEY").ok().as_ref())
            .ok_or("Not authenticated")?;

        let body = serde_json::json!({
            "model": model_id,
            "messages": self.convert_messages(&request.messages),
            "stream": true,
            "max_tokens": request.max_tokens.unwrap_or(4096),
        });

        let stream = HttpRequest::builder()
            .method(HttpMethod::Post)
            .url("https://api.my-provider.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .body(serde_json::to_vec(&body)?)
            .build()?
            .fetch_stream()?;

        Ok(zed::LlmCompletionStream::new(OpenAiStreamParser::new(stream)))
    }
}

zed::register_extension!(MyLlmExtension);
```

---

## Timeline Summary

| Phase | Duration | Key Deliverables |
|-------|----------|------------------|
| 1. Foundation | 2-3 weeks | WIT interface, basic provider registration |
| 2. Streaming | 2-3 weeks | Efficient streaming across WASM boundary |
| 3. Full Features | 2-3 weeks | Tools, images, thinking support |
| 4. Credentials & UI | 1-2 weeks | Secure credentials, configuration UI |
| 5. Testing & Docs | 1-2 weeks | Tests, documentation, examples |
| 6. Migration (optional) | Ongoing | Migrate built-in providers |

**Total estimated time: 8-13 weeks**

---

## Open Questions

1. **Streaming efficiency**: Is callback-based streaming feasible in WASM, or should we use polling?

2. **Token counting**: Should we require extensions to implement token counting, or provide a fallback estimation?

3. **Configuration UI**: Should extensions be able to provide custom UI components, or just JSON schema-driven forms?

4. **Provider priorities**: Should extension providers appear before or after built-in providers in the selector?

5. **Backward compatibility**: How do we handle extensions built against older WIT versions when adding new LLM features?

6. **Rate limiting**: Should the host help with rate limiting, or leave it entirely to extensions?

---

## Conclusion

This plan provides a comprehensive roadmap for implementing Language Model Provider Extensions in Zed. The phased approach allows for incremental delivery of value while building toward full feature parity with built-in providers.

The key architectural decisions are:
1. **WIT-based interface** for WASM interop, consistent with existing extension patterns
2. **Streaming via resources** to minimize WASM boundary crossing overhead
3. **Host-managed credentials** for security
4. **Manifest-based discovery** for static model information

The migration analysis shows that simpler providers (Ollama, LM Studio) can be migrated first as proof of concept, while more complex providers (Anthropic, Bedrock) may remain built-in initially.