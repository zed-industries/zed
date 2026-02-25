# OpenRouter Provider Selection Feature Plan

## Overview

Add the ability to select inference providers for OpenRouter models. When hovering over an OpenRouter model in the model selector, a secondary submenu/popover should expand showing available providers with their stats (name, tokens/sec, latency, input/output prices).

## Key Discoveries

### 1. OpenRouter API for Providers (Endpoints)

From the reference implementation at `/Users/bytedance/Projects/Zed/openrouter-select-provider/crates/open_router/src/open_router.rs`:

- **API Endpoint**: `https://openrouter.ai/api/frontend/stats/endpoint?permaslug={model}&variant={variant}`
- **Data Structures**:
  - `Endpoint`: Contains `provider_name`, `provider_display_name`, `pricing`, `stats`
  - `EndpointPricing`: Contains `prompt` and `completion` prices (strings)
  - `EndpointStats`: Contains `p50_throughput` (tokens/sec) and `p50_latency` (ms)

### 2. Existing Provider Setting

The current codebase already has `OpenRouterProvider` settings in:
- `/Users/bytedance/Projects/Zed/zed/crates/settings_content/src/language_model.rs#L404-418`
- Fields: `order`, `allow_fallbacks`, `require_parameters`, `data_collection`, `only`, `ignore`, `quantizations`, `sort`

The `open_router::Model` already has a `provider: Option<Provider>` field, and the request includes `provider` in `/Users/bytedance/Projects/Zed/zed/crates/language_models/src/provider/open_router.rs#L518`.

### 3. Model Fetching Pattern (from research)

Zed uses a consistent async fetching + caching pattern:

```rust
pub struct State {
    api_key_state: ApiKeyState,
    http_client: Arc<dyn HttpClient>,
    available_models: Vec<Model>,  // Cache for fetched models
    fetch_models_task: Option<Task<Result<(), LanguageModelCompletionError>>>,
}

fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<...> {
    cx.spawn(async move |this, cx| {
        let models = list_models(...).await?;
        this.update(cx, |this, cx| {
            this.available_models = models;
            cx.notify();  // Trigger UI re-render
        })?;
        Ok(())
    })
}
```

When rendering, read from cache; if empty, show loading or fallback.

### 4. Model Selector Architecture

**Primary selector**: `/Users/bytedance/Projects/Zed/zed/crates/agent_ui/src/acp/model_selector.rs`
- Uses `Picker<AcpModelPickerDelegate>`
- Renders model entries via `ModelSelectorListItem`
- Has async model refresh task that watches for changes

**UI Component**: `/Users/bytedance/Projects/Zed/zed/crates/agent_ui/src/ui/model_selector_components.rs`
- `ModelSelectorListItem` - The row component for each model
- Already has `end_hover_slot` for favorite toggle button

## Design Decision: Option A - Inline Provider Selector

Add a provider selection button/popover that appears when hovering over OpenRouter models within the existing picker UI. This is non-disruptive and fits the existing UI pattern.

### Data Flow

```
Model selector opens
       │
       ▼
User hovers on OpenRouter model row
       │
       ▼
Trigger async fetch of endpoints (if not cached)
       │
       ▼
Show provider expand icon on row
       │
       ▼
User clicks expand → popover shows provider list
       │
       ▼
User selects provider
       │
       ▼
Update model's provider setting + persist
       │
       ▼
Include provider.order in subsequent API requests
```

## Implementation Plan

### Phase 1: API Layer - Add Endpoint Fetching

**File**: `/Users/bytedance/Projects/Zed/zed/crates/open_router/src/open_router.rs`

1. Add data structures for endpoint response:
   - `ListModelEndpointsResponse`
   - `Endpoint`
   - `EndpointPricing`
   - `EndpointVariablePricing`
   - `EndpointStats`

2. Add `canonical_slug` field to `Model` struct (needed for API call)

3. Add `get_model_endpoints(model: &Model, client: &dyn HttpClient) -> Result<Vec<Endpoint>>` function

### Phase 2: State Management - Cache Endpoints

**File**: `/Users/bytedance/Projects/Zed/zed/crates/language_models/src/provider/open_router.rs`

1. Extend `State` struct:
   ```rust
   pub struct State {
       // ... existing fields
       endpoint_cache: HashMap<String, Vec<open_router::Endpoint>>,
       endpoint_fetch_tasks: HashMap<String, Task<Result<Vec<open_router::Endpoint>>>>,
   }
   ```

2. Add methods:
   ```rust
   fn fetch_endpoints(&mut self, model_id: &str, cx: &mut Context<Self>) -> Task<Result<Vec<Endpoint>>>
   fn get_cached_endpoints(&self, model_id: &str) -> Option<&Vec<Endpoint>>
   ```

3. Follow the existing pattern:
   - Spawn async task for API call
   - Update cache with `cx.notify()` on completion
   - Store task reference to prevent duplicate fetches

### Phase 3: UI - Provider Selection

**Files**:
- `/Users/bytedance/Projects/Zed/zed/crates/agent_ui/src/ui/model_selector_components.rs`
- `/Users/bytedance/Projects/Zed/zed/crates/agent_ui/src/acp/model_selector.rs`

#### 3.1 Modify ModelSelectorListItem

Add optional provider data and expand trigger:

```rust
pub struct ModelSelectorListItem {
    // ... existing fields
    provider_info: Option<ProviderInfo>,  // For display (current provider, if set)
    on_expand_providers: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

pub struct ProviderInfo {
    current_provider: Option<SharedString>,
    has_multiple_providers: bool,
}
```

Render an expand button when `on_expand_providers` is set:
```rust
.end_hover_slot(
    // ... existing favorite toggle
    // Add provider expand button for OpenRouter models
    .when_some(self.on_expand_providers, |this, handler| {
        this.child(
            IconButton::new("expand-providers", IconName::ChevronRight)
                .tooltip(Tooltip::text("Select Provider"))
                .on_click(handler)
        )
    })
)
```

#### 3.2 Create ProviderSelectorPopover Component

New file: `/Users/bytedance/Projects/Zed/zed/crates/agent_ui/src/ui/provider_selector.rs`

```rust
pub struct ProviderSelectorPopover {
    endpoints: Vec<open_router::Endpoint>,
    selected_provider: Option<String>,
    on_select: Box<dyn Fn(&str, &mut App)>,
}

impl ProviderSelectorPopover {
    fn render_endpoint(&self, endpoint: &Endpoint) -> impl IntoElement {
        let stats = endpoint.stats.as_ref();
        let throughput = stats.map(|s| s.p50_throughput).unwrap_or(0.0);
        let latency = stats.map(|s| s.p50_latency).unwrap_or(0.0);
        
        // Parse price strings (they're like "0.0000001" per token)
        let input_price = format_price(&endpoint.pricing.prompt);
        let output_price = format_price(&endpoint.pricing.completion);
        
        ListItem::new(endpoint.id.clone())
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(
                        v_flex()
                            .child(Label::new(endpoint.provider_display_name.clone()))
                            .when_some(endpoint.quantization.as_ref(), |this, q| {
                                this.child(Label::new(q).size(LabelSize::XSmall).color(Color::Muted))
                            })
                    )
                    .child(
                        h_flex()
                            .gap_3()
                            .child(
                                v_flex()
                                    .items_end()
                                    .child(Label::new(format!("{:.0} tok/s", throughput)).size(LabelSize::Small))
                                    .child(Label::new(format!("{:.0}ms", latency)).size(LabelSize::XSmall).color(Color::Muted))
                            )
                            .child(
                                v_flex()
                                    .items_end()
                                    .child(Label::new(format!("${}/M", input_price)).size(LabelSize::Small))
                                    .child(Label::new(format!("${}/M", output_price)).size(LabelSize::XSmall).color(Color::Muted))
                            )
                    )
            )
    }
}
```

#### 3.3 Integrate into AcpModelSelector

In `render_match` for OpenRouter models, wire up the provider expansion:

```rust
fn render_match(&self, ix: usize, selected: bool, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<Self::ListItem> {
    // ... existing code
    AcpModelPickerEntry::Model(model_info, is_favorite) => {
        let is_openrouter = model_info.id.0.starts_with("openrouter/");
        
        // ... existing ModelSelectorListItem construction
        ModelSelectorListItem::new(ix, model_info.name.clone())
            // ... existing chain
            .when(is_openrouter, |this| {
                this.on_expand_providers(cx.listener(move |picker, _, window, cx| {
                    picker.show_provider_selector(model_info.id.clone(), window, cx);
                }))
            })
    }
}
```

Add state for provider popover:
```rust
pub struct AcpModelPickerDelegate {
    // ... existing fields
    provider_popover: Option<(ModelId, Entity<ProviderSelectorPopover>)>,
    endpoint_cache: Arc<RwLock<HashMap<String, Vec<open_router::Endpoint>>>>,
}
```

### Phase 4: Persist Provider Selection

**File**: `/Users/bytedance/Projects/Zed/zed/crates/settings_content/src/language_model.rs`

The existing `OpenRouterProvider` struct already supports `order: Option<Vec<String>>` which is exactly what we need. When user selects a provider:

1. Set `model.provider.order = vec![selected_provider_name]`
2. This automatically flows through to API requests via existing code path

**For per-model persistence**, we need to track which provider is selected for which model. Options:
- Store in agent settings under a new field
- Store alongside model selection in thread/profile settings

## Detailed Task List

1. **open_router.rs additions** (~100 lines)
   - Add `Endpoint`, `EndpointPricing`, `EndpointStats` structs
   - Add `canonical_slug` field to `Model`
   - Implement `get_model_endpoints()` function

2. **open_router provider state** (~50 lines)
   - Add endpoint cache HashMap
   - Add endpoint fetch task tracking
   - Implement fetch/cache methods

3. **ModelSelectorListItem enhancement** (~30 lines)
   - Add `provider_info` field
   - Add `on_expand_providers` callback
   - Render expand button in hover slot

4. **ProviderSelectorPopover component** (~150 lines)
   - New component for provider list
   - Price formatting utilities
   - Selection handling

5. **AcpModelSelector integration** (~80 lines)
   - Detect OpenRouter models
   - Manage provider popover state
   - Wire up endpoint fetching
   - Handle provider selection

6. **Provider persistence** (~40 lines)
   - Store selected provider per model
   - Load on startup

## Testing Plan

1. **Unit tests** for:
   - Endpoint API response parsing
   - Price formatting
   - Provider selection logic

2. **Integration tests** for:
   - Endpoint caching
   - Cache invalidation

3. **Manual testing**:
   - Hover on OpenRouter model → expand button appears
   - Click expand → provider list popover opens
   - Verify stats display correctly
   - Select provider → verify API requests include provider
   - Restart → provider selection persists

## Risks & Mitigations

1. **API Stability**: The `api/frontend/stats/endpoint` endpoint is undocumented
   - Mitigation: Graceful degradation if API fails; don't block model selection

2. **Performance**: Endpoint fetches could slow down UI
   - Mitigation: Cache aggressively; prefetch on model list load; show loading state

3. **UX Complexity**: Provider selection adds cognitive load
   - Mitigation: Only show for OpenRouter; sensible default; remember last selection

## Files Changed Summary

| File | Changes |
|------|---------|
| `crates/open_router/src/open_router.rs` | +structs, +get_model_endpoints() |
| `crates/language_models/src/provider/open_router.rs` | +endpoint cache in State |
| `crates/agent_ui/src/ui/model_selector_components.rs` | +provider fields in ModelSelectorListItem |
| `crates/agent_ui/src/ui/provider_selector.rs` | NEW: ProviderSelectorPopover |
| `crates/agent_ui/src/ui/mod.rs` | +export provider_selector |
| `crates/agent_ui/src/acp/model_selector.rs` | +endpoint integration, +popover management |
