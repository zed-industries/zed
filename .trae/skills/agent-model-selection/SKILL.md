---
name: "agent-model-selection"
description: "Guide for understanding and modifying agent model selection in Zed. Invoke when working on model picker UI, provider registration, or LLM request handling."
---

# Agent Model Selection Architecture

## Data Flow

1. **Startup**: Providers registered in `language_models/src/language_models.rs` → stored in `LanguageModelRegistry`
2. **UI Display**: `language_model_selector.rs` calls `registry.visible_providers()` → `provider.provided_models(cx)` → grouped by favorites/recommended/provider
3. **Selection**: User clicks model → `confirm()` → `update_settings_file()` → `settings.json` updated → settings observer → `registry.select_default_model()`
4. **LLM Request**: `model.stream_completion(request)` → provider-specific transform (e.g., `into_open_ai()`) → HTTP request → response stream

## Key Files

### Core Abstractions
- `crates/language_model/src/language_model.rs` - `LanguageModel` trait
- `crates/language_model/src/registry.rs` - `LanguageModelRegistry`
- `crates/language_model/src/provider.rs` - `LanguageModelProvider` trait

### Provider Implementations
- `crates/language_models/src/provider/open_ai.rs`
- `crates/language_models/src/provider/anthropic.rs`
- `crates/language_models/src/provider/open_router.rs`
- `crates/language_models/src/provider/google.rs`
- `crates/language_models/src/provider/cloud.rs`
- `crates/language_models/src/provider/ollama.rs`

### UI Components
- `crates/agent_ui/src/language_model_selector.rs` - Main model picker
- `crates/agent_ui/src/acp/model_selector.rs` - ACP model selector
- `crates/agent_ui/src/ui/provider_selector.rs` - Provider selection UI

### Settings
- `crates/settings_content/src/agent.rs` - `LanguageModelSelection` struct
- `crates/agent_ui/src/agent_ui.rs` - `update_active_language_model_from_settings()`

## Key Types

### LanguageModel Trait
- `id()`, `name()`, `provider_id()`, `provider_name()`
- `supports_tools()`, `supports_images()`, `max_token_count()`
- `permaslug()` - stable identifier for API calls
- `stream_completion(LanguageModelRequest)` - main entry point for LLM requests

### LanguageModelProvider Trait
- `provided_models(cx)` - returns all models
- `recommended_models(cx)` - returns recommended models
- `is_authenticated(cx)`, `authenticate(cx)`

### AgentModelInfo (for ACP)
- `id: acp::ModelId` - model identifier
- `providers_callback: Option<ProvidersCallback>` - callback for fetching provider options

### LanguageModelSelection (Settings)
- `provider: LanguageModelProviderSetting` - e.g., "openai", "openrouter"
- `model: String` - e.g., "gpt-5", "claude-sonnet-4"
- `enable_thinking: bool`, `effort: Option<String>`

## OpenRouter Specifics

### Model ID vs Permaslug
- **slug**: Stable identifier but can point to different model versions over time (e.g., `anthropic/claude-sonnet` may update to newer versions)
- **permaslug**: Immutable identifier that always points to the exact same model version

### Provider/Endpoint Selection
- Endpoint data: `GET https://openrouter.ai/api/frontend/stats/endpoint?permaslug={permaslug}`
- Selection stored: `OpenRouterState::selected_providers: HashMap<String, String>`
- Applied in: `into_open_router()` sets `provider.order` field

### Key Files
- `crates/open_router/src/open_router.rs` - API client, `Model`, `Endpoint` structs
- `crates/language_models/src/provider/open_router.rs` - `OpenRouterState`, `fetch_endpoints()`
