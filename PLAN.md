# Plan: Correctly Send `SweepSuggestionType` for Edit Predictions

## Goal

For jump predictions, send **two notifications** to Sweep:
1. `SweepSuggestionType::JumpToEdit` - when the jump indicator is shown
2. `SweepSuggestionType::GhostText` or `Popup` - when the user accepts the jump and sees the actual edit

For non-jump predictions, send **one notification** with the appropriate type.

**Important**: This should ONLY affect Sweep. Zeta1/Zeta2 and other providers should not receive double notifications or have any behavior changes.

---

## Context Summary

### Current State

1. **Prediction Types**: The system distinguishes between:
   - `BufferEditPrediction::Local` - edit in the current buffer (shown as ghost text or popup)
   - `BufferEditPrediction::Jump` - edit in a different buffer (shown as "Jump to Edit" indicator)

2. **Editor Display Modes**: When rendering predictions:
   - `EditPrediction::MoveWithin` / `MoveOutside` - jump indicators
   - `EditPrediction::Edit` with `EditDisplayMode::Inline` - ghost text
   - `EditPrediction::Edit` with `EditDisplayMode::DiffPopover` - popup

3. **Current Metrics**: Both `edit_prediction_shown` and `edit_prediction_accepted` in `sweep_ai.rs` hardcode `SweepSuggestionType::GhostText`

4. **`did_show` implementations**: Only `ZedEditPredictionDelegate` overrides `did_show`. All other providers (Copilot, Supermaven, Codestral, etc.) use the default empty implementation.

---

## Implementation Plan

### Step 1: Add Shared Display Type Enum

**File: `crates/edit_prediction_types/src/edit_prediction_types.rs`**

Add a new enum to represent what the user sees:

```rust
/// The display mode used when showing an edit prediction to the user.
/// Used for metrics tracking.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuggestionDisplayType {
    GhostText,
    DiffPopover,
    Jump,
}
```

### Step 2: Update Delegate Trait Signatures

**File: `crates/edit_prediction_types/src/edit_prediction_types.rs`**

Modify `did_show` to accept the display type in all three places:

```rust
// In EditPredictionDelegate trait:
fn did_show(&mut self, _display_type: SuggestionDisplayType, _cx: &mut Context<Self>) {}

// In EditPredictionDelegateHandle trait:
fn did_show(&self, display_type: SuggestionDisplayType, cx: &mut App);

// In impl<T> EditPredictionDelegateHandle for Entity<T>:
fn did_show(&self, display_type: SuggestionDisplayType, cx: &mut App) {
    self.update(cx, |this, cx| this.did_show(display_type, cx))
}
```

### Step 3: Update `CurrentEditPrediction`

**File: `crates/edit_prediction/src/edit_prediction.rs`**

Add field to track display type:

```rust
struct CurrentEditPrediction {
    pub requested_by: PredictionRequestedBy,
    pub prediction: EditPrediction,
    pub was_shown: bool,
    pub shown_at: Option<Instant>,
    /// The display type used when showing this prediction (for Sweep metrics)
    pub shown_with: Option<SuggestionDisplayType>,
}
```

Initialize `shown_with: None` wherever `CurrentEditPrediction` is constructed.

### Step 4: Update `did_show_current_prediction` in Store

**File: `crates/edit_prediction/src/edit_prediction.rs`**

Modify signature and implementation:

```rust
fn did_show_current_prediction(
    &mut self,
    project: &Entity<Project>,
    display_type: SuggestionDisplayType,
    cx: &mut Context<Self>
) {
    let Some(project_state) = self.projects.get_mut(&project.entity_id()) else {
        return;
    };

    let Some(current_prediction) = project_state.current_prediction.as_mut() else {
        return;
    };

    // Track the display type for acceptance metrics
    // Update to non-jump type if we're showing the actual edit (overwrite jump with ghost/popup)
    if current_prediction.shown_with.is_none()
        || display_type != SuggestionDisplayType::Jump
    {
        current_prediction.shown_with = Some(display_type);
    }

    let is_first_non_jump_show = !current_prediction.was_shown
        && display_type != SuggestionDisplayType::Jump;

    if is_first_non_jump_show {
        current_prediction.was_shown = true;
        current_prediction.shown_at = Some(Instant::now());
    }

    // Send metrics - Sweep gets display-type-aware notifications
    if self.edit_prediction_model == EditPredictionModel::Sweep {
        sweep_ai::edit_prediction_shown(
            &self.sweep_ai,
            self.client.clone(),
            &current_prediction.prediction,
            display_type,
            cx,
        );
    }
    // Zeta doesn't send shown notifications, so nothing to do here

    // Only track in shown_predictions on first non-jump show
    if is_first_non_jump_show {
        self.shown_predictions
            .push_front(current_prediction.prediction.clone());
        if self.shown_predictions.len() > 50 {
            self.shown_predictions.pop_back();
        }
    }
}
```

### Step 5: Update `ZedEditPredictionDelegate`

**File: `crates/edit_prediction/src/zed_edit_prediction_delegate.rs`**

Update signature to match trait:

```rust
fn did_show(&mut self, display_type: SuggestionDisplayType, cx: &mut Context<Self>) {
    self.store.update(cx, |store, cx| {
        store.did_show_current_prediction(&self.project, display_type, cx);
    });
}
```

### Step 6: Update `sweep_ai.rs` Functions

**File: `crates/edit_prediction/src/sweep_ai.rs`**

Modify `edit_prediction_shown` to accept `SuggestionDisplayType` and map internally:

```rust
pub fn edit_prediction_shown(
    sweep_ai: &SweepAi,
    client: Arc<Client>,
    prediction: &EditPrediction,
    display_type: SuggestionDisplayType,
    cx: &App,
) {
    let Some(api_token) = sweep_ai.api_token.read(cx).key(&SWEEP_CREDENTIALS_URL) else {
        return;
    };
    let debug_info = sweep_ai.debug_info.clone();

    let (additions, deletions) = compute_edit_metrics(&prediction.edits, &prediction.snapshot);
    let autocomplete_id = prediction.id.to_string();

    let suggestion_type = match display_type {
        SuggestionDisplayType::GhostText => SweepSuggestionType::GhostText,
        SuggestionDisplayType::DiffPopover => SweepSuggestionType::Popup,
        SuggestionDisplayType::Jump => SweepSuggestionType::JumpToEdit,
    };

    let request_body = AutocompleteMetricsRequest {
        event_type: SweepEventType::AutocompleteSuggestionShown,
        suggestion_type,
        additions,
        deletions,
        autocomplete_id,
        edit_tracking: String::new(),
        edit_tracking_line: None,
        lifespan: 0,
        debug_info,
        device_id: String::new(),
        privacy_mode_enabled: false,
    };

    send_autocomplete_metrics_request(cx, client, api_token, request_body);
}
```

Update `edit_prediction_accepted` to use `shown_with`:

```rust
pub(crate) fn edit_prediction_accepted(
    store: &EditPredictionStore,
    current_prediction: CurrentEditPrediction,
    cx: &App,
) {
    // ... existing setup code ...

    let suggestion_type = match current_prediction.shown_with {
        Some(SuggestionDisplayType::DiffPopover) => SweepSuggestionType::Popup,
        Some(SuggestionDisplayType::Jump) => SweepSuggestionType::GhostText, // fallback if only jump was shown
        Some(SuggestionDisplayType::GhostText) | None => SweepSuggestionType::GhostText,
    };

    let request_body = AutocompleteMetricsRequest {
        event_type: SweepEventType::AutocompleteSuggestionAccepted,
        suggestion_type,
        // ... rest unchanged
    };

    send_autocomplete_metrics_request(cx, store.client.clone(), api_token, request_body);
}
```

### Step 7: Update Editor Call Sites

**File: `crates/editor/src/editor.rs`**

**In `update_edit_prediction_preview`** (around line 8079):

```rust
if let Some(provider) = self.edit_prediction_provider.as_ref() {
    // Determine display type based on current active prediction
    let display_type = self.active_edit_prediction
        .as_ref()
        .map(|p| match &p.completion {
            EditPrediction::MoveWithin { .. } | EditPrediction::MoveOutside { .. } => {
                SuggestionDisplayType::Jump
            }
            EditPrediction::Edit { display_mode, .. } => match display_mode {
                EditDisplayMode::DiffPopover => SuggestionDisplayType::DiffPopover,
                EditDisplayMode::Inline | EditDisplayMode::TabAccept => SuggestionDisplayType::GhostText,
            },
        })
        .unwrap_or(SuggestionDisplayType::GhostText);

    provider.provider.did_show(display_type, cx)
}
```

**In `update_visible_edit_prediction`** (around line 8269):

When showing a move/jump:
```rust
let completion = if is_move {
    if let Some(provider) = &self.edit_prediction_provider {
        provider.provider.did_show(SuggestionDisplayType::Jump, cx);
    }
    // ... rest of move handling
    EditPrediction::MoveWithin { target, snapshot }
} else {
    // ...
}
```

When showing an edit:
```rust
if show_completions_in_buffer {
    let display_type = if all_edits_insertions_or_deletions(&edits, &multibuffer) {
        SuggestionDisplayType::GhostText
    } else {
        SuggestionDisplayType::DiffPopover
    };

    if let Some(provider) = &self.edit_prediction_provider {
        provider.provider.did_show(display_type, cx);
    }

    // ... rest of edit rendering
}
```

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/edit_prediction_types/src/edit_prediction_types.rs` | Add `SuggestionDisplayType` enum, update `did_show` signatures (3 places) |
| `crates/edit_prediction/src/edit_prediction.rs` | Add `shown_with` to `CurrentEditPrediction`, update `did_show_current_prediction` signature and implementation |
| `crates/edit_prediction/src/zed_edit_prediction_delegate.rs` | Update `did_show` signature |
| `crates/edit_prediction/src/sweep_ai.rs` | Add `display_type` parameter to `edit_prediction_shown`, update `edit_prediction_accepted` to use `shown_with` |
| `crates/editor/src/editor.rs` | Update 2 `did_show` call sites to compute and pass display type |

---

## Notification Flow

### Jump Prediction Flow
1. User sees jump indicator → `did_show(Jump)` → Sweep gets `shown(JumpToEdit)`
2. User accepts jump, cursor moves, sees edit → `did_show(GhostText|DiffPopover)` → Sweep gets `shown(GhostText|Popup)`
3. User accepts edit → Sweep gets `accepted(GhostText|Popup)` (uses `shown_with`)

### Direct Edit Flow (no jump)
1. User sees ghost/popup → `did_show(GhostText|DiffPopover)` → Sweep gets `shown(GhostText|Popup)`
2. User accepts → Sweep gets `accepted(GhostText|Popup)`

### Other Providers (Zeta, Copilot, Supermaven, etc.)
- No `shown` notifications sent (Zeta only sends accept)
- Default empty `did_show` implementation ignores the parameter
- No behavior change

---

## Testing Considerations

1. **Unit tests**: Verify that jump predictions trigger two Sweep notifications with correct types
2. **Integration tests**: Verify non-Zed providers don't receive any notifications from `did_show`
3. **Manual testing**: Use Sweep provider to verify metrics are sent correctly for:
   - Direct ghost text edit
   - Direct diff popover edit
   - Jump → ghost text
   - Jump → diff popover
