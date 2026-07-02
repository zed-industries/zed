# Building a modal form with text inputs in Zed (GPUI)

# Modal forms with text inputs in Zed — API reference

## 1. Showing a modal: `Workspace::toggle_modal`

`/Users/user/zed/crates/workspace/src/workspace.rs:7907`
```rust
pub fn toggle_modal<V: ModalView, B>(&mut self, window: &mut Window, cx: &mut App, build: B)
where
    B: FnOnce(&mut Window, &mut Context<V>) -> V,
```
Also on `Workspace` (`workspace.rs:7896`, `:7916`):
```rust
pub fn active_modal<V: ManagedView + 'static>(&self, cx: &App) -> Option<Entity<V>>
pub fn hide_modal(&mut self, window: &mut Window, cx: &mut App) -> bool
```
Semantics (`crates/workspace/src/modal_layer.rs:139-161`): if a modal of the same type `V` is active it is toggled off; a different active modal is replaced; the new modal's `focus_handle(cx)` is focused automatically via `cx.defer_in` (`modal_layer.rs:187-189`). Dismissal happens when the modal emits `DismissEvent` (`modal_layer.rs:89-93` subscribes and calls `hide_modal`). Clicking outside the modal also dismisses it (`modal_layer.rs:283-288`).

Real call sites:
- `/Users/user/zed/crates/git_ui/src/git_ui.rs:515-517`:
```rust
workspace.toggle_modal(window, cx, |window, cx| {
    RenameBranchModal::new(current_branch_name, repo, window, cx)
});
```
- `/Users/user/zed/crates/recent_projects/src/remote_connections.rs:259-261`:
```rust
workspace.toggle_modal(window, cx, |window, cx| {
    RemoteConnectionModal::new(&connection_options, paths, window, cx)
});
```
- From async context via `WeakEntity<Workspace>` (`/Users/user/zed/crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs:541-575`): `workspace.update_in(cx, |workspace, window, cx| { workspace.toggle_modal(window, cx, |window, cx| Self { ... }) })`.

## 2. Trait requirements

`ModalView` — `/Users/user/zed/crates/workspace/src/modal_layer.rs:49-65`:
```rust
pub trait ModalView: ManagedView {
    fn on_before_dismiss(&mut self, _window: &mut Window, _: &mut Context<Self>) -> DismissDecision {
        DismissDecision::Dismiss(true)
    }
    fn fade_out_background(&self) -> bool { false }
    fn render_bare(&self) -> bool { false }
}
```
`DismissDecision` — `modal_layer.rs:8-11`:
```rust
pub enum DismissDecision { Dismiss(bool), Pending }
```
`ManagedView` (blanket-implemented) — `/Users/user/zed/crates/gpui/src/window.rs:570-575`:
```rust
pub trait ManagedView: Focusable + EventEmitter<DismissEvent> + Render {}
impl<M: Focusable + EventEmitter<DismissEvent> + Render> ManagedView for M {}
pub struct DismissEvent;
```
So a modal type must impl: `Render`, `Focusable` (`fn focus_handle(&self, cx: &App) -> FocusHandle`), `impl EventEmitter<DismissEvent> for T {}`, and `impl ModalView for T {}`. Emit `cx.emit(DismissEvent)` to close. Example of dismissal-blocking: `RemoteConnectionModal` returns `DismissDecision::Dismiss(self.finished)` and `fade_out_background() -> true` (`/Users/user/zed/crates/remote_connection/src/remote_connection.rs:415-427`).

## 3. Complete minimal example: `RenameBranchModal` (single Editor field)

`/Users/user/zed/crates/git_ui/src/git_ui.rs:404-490` (uses `menu::{Confirm, Cancel}` actions; `menu` crate defines them via `actions!(menu, [Cancel, Confirm, SelectPrevious, SelectNext, ...])` at `/Users/user/zed/crates/menu/src/menu.rs:12`; global keymap binds `enter`→`menu::Confirm`, `escape`→`menu::Cancel`, `tab`→`menu::SelectNext`, `shift-tab`→`menu::SelectPrevious` at `/Users/user/zed/assets/keymaps/default-macos.json:21-33`):
```rust
struct RenameBranchModal {
    current_branch: SharedString,
    editor: Entity<Editor>,
    repo: Entity<Repository>,
}

impl RenameBranchModal {
    fn new(current_branch: String, repo: Entity<Repository>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(current_branch.clone(), window, cx);
            editor
        });
        Self { current_branch: current_branch.into(), editor, repo }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let new_name = self.editor.read(cx).text(cx);
        // ... spawn work ...
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for RenameBranchModal {}
impl ModalView for RenameBranchModal {}
impl Focusable for RenameBranchModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)   // modal layer focuses this on open
    }
}

impl Render for RenameBranchModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RenameBranchModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(rems(34.))
            .child(/* header h_flex with Icon + Headline */)
            .child(div().px_3().pb_3().w_full().child(self.editor.clone()))
    }
}
```
Variant with placeholder + explicit focus + inline `on_action` closures: `GitCloneModal` (`git_ui.rs:1193-1275`):
```rust
let repo_input = cx.new(|cx| {
    let mut editor = Editor::single_line(window, cx);
    editor.set_placeholder_text("Enter repository URL…", window, cx);
    editor
});
let focus_handle = repo_input.focus_handle(cx);
window.focus(&focus_handle, cx);
// render:
.on_action(cx.listener(|_, _: &menu::Cancel, _, cx| { cx.emit(DismissEvent); }))
.on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
    let repo = this.repo_input.read(cx).text(cx);
    /* ... */
    cx.emit(DismissEvent);
}))
```
Standard modal chrome: root `div()/v_flex()` with `.elevation_2(cx)`/`.elevation_3(cx)`, `.w(rems(34.))`, `.key_context("SomeModal")`. The SSH modal (`RemoteServerProjects::render`, `/Users/user/zed/crates/recent_projects/src/remote_servers.rs:3076-3112`) additionally does `.capture_any_mouse_down(cx.listener(|this,_,window,cx| { this.focus_handle(cx).focus(window, cx); }))` and `.on_mouse_down_out(cx.listener(|this,_,_,cx| cx.emit(DismissEvent)))`. An `Entity<Editor>` renders directly as a child (`.child(self.editor.clone())`); wrap it for borders/padding, e.g. `div().p_2().border_b_1().border_color(cx.theme().colors().border_variant).child(state.address_editor.clone())` (`remote_servers.rs:2493-2499`).

## 4. Multi-field form with Tab between fields: `ConfigureMode` in the debugger's NewProcessModal

`/Users/user/zed/crates/debugger_ui/src/new_process_modal.rs:820-957`. Uses `InputField` (crate `ui_input`) rather than raw `Editor`:
```rust
pub(super) struct ConfigureMode {
    program: Entity<InputField>,
    cwd: Entity<InputField>,
    stop_on_entry: ToggleState,
    save_to_debug_json: ToggleState,
}

impl ConfigureMode {
    pub(super) fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let program = cx.new(|cx| {
            InputField::new(window, cx, "ENV=Zed ~/bin/program --option")
                .label("Program")
                .tab_stop(true)
                .tab_index(1)
        });
        let cwd = cx.new(|cx| {
            InputField::new(window, cx, "Ex: $ZED_WORKTREE_ROOT")
                .label("Working Directory")
                .tab_stop(true)
                .tab_index(2)
        });
        cx.new(|_| Self { program, cwd, stop_on_entry: ToggleState::Unselected, save_to_debug_json: ToggleState::Unselected })
    }

    fn on_tab(&mut self, _: &menu::SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
    }
    fn on_tab_prev(&mut self, _: &menu::SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_prev(cx);
    }

    fn render(&mut self, ..., cx: &mut ui::Context<Self>) -> impl IntoElement {
        v_flex()
            .tab_group()
            .track_focus(&self.program.focus_handle(cx))
            .on_action(cx.listener(Self::on_tab))
            .on_action(cx.listener(Self::on_tab_prev))
            .p_2().w_full().gap_3()
            .child(self.program.clone())
            .child(self.cwd.clone())
            .child(Switch::new("debugger-stop-on-entry", self.stop_on_entry).tab_index(3_isize) /* ... */)
    }
}
```
Why Tab works: single-line editors propagate the `Tab` action (`Editor::tab`, `/Users/user/zed/crates/editor/src/editor.rs:5061-5065`: `if self.mode.is_single_line() { cx.propagate(); return; }`), the global keymap maps `tab`→`menu::SelectNext` / `shift-tab`→`menu::SelectPrevious` (`assets/keymaps/default-macos.json:21,24`), and the form container handles those with `window.focus_next(cx)` / `window.focus_prev(cx)` (`gpui/src/window.rs:1950,1961` — `pub fn focus_next(&mut self, cx: &mut App)`, `pub fn focus_prev(&mut self, cx: &mut App)`), which walk tab stops inside the `.tab_group()` by `tab_index`. Tab-order plumbing: `FocusHandle::tab_index(isize)` and `FocusHandle::tab_stop(bool)` builder methods (`gpui/src/window.rs:429,440`); element-level `.tab_index()`, `.tab_stop()`, `.tab_group()` on `div` (`gpui/src/elements/div.rs:715,724,735`).

Reading the values on confirm (`new_process_modal.rs:860-866`):
```rust
let cwd_text = self.cwd.read(cx).text(cx);          // InputField::text
let program = self.program.read(cx).text(cx);
```
`NewProcessModal` itself: `impl EventEmitter<DismissEvent> for NewProcessModal {}`, `impl ModalView for NewProcessModal {}` (`new_process_modal.rs:802,809`), `Focusable::focus_handle` delegates to the active field's handle (`:803-806`), cancel via `.on_action(cx.listener(|_, _: &menu::Cancel, _, cx| { cx.emit(DismissEvent); }))` (`:582`), opened with `workspace.toggle_modal(window, cx, |window, cx| { ... })` (`:101`).

## 5. `InputField` component (recommended for form fields)

Crate `ui_input` (add `ui_input.workspace = true`; the `Arc<dyn ErasedEditor>` factory is installed by `editor::init` — `editor.rs:392` sets `ui_input::ERASED_EDITOR_FACTORY`). `/Users/user/zed/crates/ui_input/src/input_field.rs`:
```rust
impl InputField {                                             // input_field.rs:55
    pub fn new(window: &mut Window, cx: &mut App, placeholder_text: &str) -> Self  // :56
    pub fn start_icon(mut self, icon: IconName) -> Self                            // :77
    pub fn label(mut self, label: impl Into<SharedString>) -> Self                 // :82
    pub fn label_size(mut self, size: LabelSize) -> Self                           // :87
    pub fn tab_index(mut self, index: isize) -> Self                               // :97
    pub fn tab_stop(mut self, tab_stop: bool) -> Self                              // :102
    pub fn masked(mut self, masked: bool) -> Self          // password-style; adds eye toggle button  // :108
    pub fn set_error(&mut self, error: Option<impl Into<SharedString>>, cx: &mut Context<Self>)       // :116 red border + hint text
    pub fn is_empty(&self, cx: &App) -> bool                                        // :121
    pub fn text(&self, cx: &App) -> String                                          // :129
    pub fn clear(&self, window: &mut Window, cx: &mut App)                          // :133
    pub fn set_text(&self, text: &str, window: &mut Window, cx: &mut App)           // :137
    pub fn set_masked(&self, masked: bool, window: &mut Window, cx: &mut App)       // :141
}
impl Focusable for InputField { fn focus_handle(&self, cx: &App) -> FocusHandle }   // :49 (delegates to inner editor)
impl Render for InputField { ... }  // :146 renders label + bordered h_flex + editor, focused border color
```
Hold as `Entity<InputField>`, add as `.child(self.field.clone())`.

## 6. Reading/writing text on a raw `Editor`

`/Users/user/zed/crates/editor/src/editor.rs`:
```rust
pub fn single_line(window: &mut Window, cx: &mut Context<Self>) -> Self            // :1731
pub fn multi_line(window: &mut Window, cx: &mut Context<Self>) -> Self             // :1737
pub fn set_placeholder_text(&mut self, placeholder_text: &str, window: &mut Window, cx: &mut Context<Self>)  // :3088
pub fn text(&self, cx: &App) -> String                                             // :8469
pub fn is_empty(&self, cx: &App) -> bool                                           // :8473
pub fn text_option(&self, cx: &App) -> Option<String>   // trimmed, None if empty  // :8477
pub fn set_text(&mut self, text: impl Into<Arc<str>>, window: &mut Window, cx: &mut Context<Self>)  // :8488
pub fn set_masked(&mut self, masked: bool, cx: &mut Context<Self>)                 // :8569
```
Usage: create `cx.new(|cx| { let mut e = Editor::single_line(window, cx); e.set_placeholder_text("...", window, cx); e })`; read with `self.editor.read(cx).text(cx)`; prefill with `editor.update(cx, |e, cx| e.set_text("...", window, cx))`. To focus a field manually: `self.editor.focus_handle(cx).focus(window, cx)` (e.g. `EditNicknameState::new`, `remote_servers.rs:2960-2980`) or `window.focus(&focus_handle, cx)`.

## 7. Masked / password input — yes, it exists

- Raw editor: `editor.set_masked(true, cx)` (renders bullets). Real use: client-secret field in `ConfigureContextServerModal` (`/Users/user/zed/crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs:562-571`):
```rust
secret_editor: cx.new(|cx| {
    let mut editor = Editor::single_line(window, cx);
    editor.set_placeholder_text("Enter client secret (leave empty for public clients)", window, cx);
    editor.set_masked(true, cx);
    editor
}),
```
- `InputField::masked(true)` gives the same plus a built-in show/hide eye `IconButton` (`input_field.rs:108,206-229`).
- SSH password prompt with manual eye toggle: `RemoteConnectionPrompt` (`/Users/user/zed/crates/remote_connection/src/remote_connection.rs:24-119,167-176`) — toggles via `this.editor.set_masked(this.is_masked, window, cx)` in an `on_click` listener, reads the password with `self.editor.text(cx)` then `self.editor.clear(window, cx)`.

## 8. Checklist for a new modal form

1. Struct holding `Entity<InputField>` (or `Entity<Editor>`) per field.
2. `impl Focusable` returning the first field's `focus_handle(cx)` (modal layer auto-focuses it).
3. `impl EventEmitter<DismissEvent> for T {}` and `impl ModalView for T {}` (override `on_before_dismiss` only to block Esc/click-out dismissal).
4. In `render`: root `v_flex().key_context("MyModal").elevation_3(cx).w(rems(34.)).tab_group().on_action(cx.listener(Self::confirm)).on_action(cx.listener(Self::cancel))` plus `menu::SelectNext`/`menu::SelectPrevious` handlers calling `window.focus_next(cx)`/`focus_prev(cx)`; give each field ascending `tab_index`.
5. `confirm(&mut self, _: &menu::Confirm, window, cx)` reads `field.read(cx).text(cx)`, does work, `cx.emit(DismissEvent)`; `cancel` just emits `DismissEvent`.
6. Open with `workspace.toggle_modal(window, cx, |window, cx| MyModal::new(..., window, cx))`.
