# Embedding a multi-line Editor with syntax highlighting inside a custom view (SQL scratch editor)

# Embedding a multi-line Editor with syntax highlighting in a custom view

## 1. Creating a multi-line `Editor` entity inside another view

### Core constructor and mode enum

`crates/editor/src/editor.rs:1836`:
```rust
pub fn new(
    mode: EditorMode,
    buffer: Entity<MultiBuffer>,
    project: Option<Entity<Project>>,
    window: &mut Window,
    cx: &mut Context<Self>,
) -> Self
```

`EditorMode` — `crates/editor/src/editor.rs:460`:
```rust
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EditorMode {
    SingleLine,
    AutoHeight {
        min_lines: usize,
        max_lines: Option<usize>,
    },
    Full {
        scale_ui_elements_with_buffer_font_size: bool,
        show_active_line_background: bool,
        sizing_behavior: SizingBehavior,
    },
    Minimap { parent: WeakEntity<Editor> },
}
```
`EditorMode::full()` (editor.rs:480) returns `Full { scale_ui_elements_with_buffer_font_size: true, show_active_line_background: true, sizing_behavior: SizingBehavior::Default }`.

### Convenience constructors (all `crates/editor/src/editor.rs`)

```rust
pub fn single_line(window: &mut Window, cx: &mut Context<Self>) -> Self            // :1731
pub fn multi_line(window: &mut Window, cx: &mut Context<Self>) -> Self             // :1737, uses EditorMode::full()
pub fn auto_height(min_lines: usize, max_lines: usize, window: &mut Window, cx: &mut Context<Self>) -> Self  // :1743
pub fn auto_height_unbounded(min_lines: usize, window: &mut Window, cx: &mut Context<Self>) -> Self          // :1765
pub fn for_buffer(buffer: Entity<Buffer>, project: Option<Entity<Project>>, window: &mut Window, cx: &mut Context<Self>) -> Self       // :1784, EditorMode::full()
pub fn for_multibuffer(buffer: Entity<MultiBuffer>, project: Option<Entity<Project>>, window: &mut Window, cx: &mut Context<Self>) -> Self  // :1794
```
`Editor::multi_line` internals (editor.rs:1737-1741) show the buffer chain:
```rust
let buffer = cx.new(|cx| Buffer::local("", cx));
let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
Self::new(EditorMode::full(), buffer, None, window, cx)
```
Signatures: `Buffer::local<T: Into<String>>(base_text: T, cx: &Context<Self>) -> Self` (`crates/language/src/buffer.rs:971`); `MultiBuffer::singleton(buffer: Entity<Buffer>, cx: &mut Context<Self>) -> Self` (`crates/multi_buffer/src/multi_buffer.rs:1227`).

### Real example A — agent panel message editor (`crates/agent_ui/src/message_editor.rs:462-506`)

```rust
let language_registry = project
    .upgrade()
    .map(|project| project.read(cx).languages().clone());

let editor = cx.new(|cx| {
    let buffer = cx.new(|cx| {
        let buffer = Buffer::local("", cx);
        if let Some(language_registry) = language_registry.as_ref() {
            buffer.set_language_registry(language_registry.clone());
        }
        buffer
    });
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

    let mut editor = Editor::new(mode, buffer, None, window, cx);
    editor.set_placeholder_text(placeholder, window, cx);
    editor.set_show_indent_guides(false, cx);
    editor.set_show_completions_on_input(Some(true));
    editor.set_soft_wrap();
    editor.disable_mouse_wheel_zoom();
    editor.set_use_modal_editing(true);
    editor
});
```
The caller passes the mode (`crates/agent_ui/src/conversation_view/thread_view.rs:804-807`):
```rust
editor::EditorMode::AutoHeight {
    min_lines: AgentSettings::get_global(cx).message_editor_min_lines,
    max_lines: Some(AgentSettings::get_global(cx).set_message_editor_max_lines()),
},
```

### Real example B — git commit editor (`crates/git_ui/src/git_panel.rs:816-845`)

```rust
pub(crate) fn commit_message_editor(
    commit_message_buffer: Entity<Buffer>,
    placeholder: Option<SharedString>,
    project: Entity<Project>,
    in_panel: bool,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> Editor {
    let buffer = cx.new(|cx| MultiBuffer::singleton(commit_message_buffer, cx));
    let max_lines = if in_panel { MAX_PANEL_EDITOR_LINES } else { 18 };
    let mut commit_editor = Editor::new(
        EditorMode::AutoHeight { min_lines: max_lines, max_lines: Some(max_lines) },
        buffer, None, window, cx,
    );
    commit_editor.set_use_autoclose(false);
    commit_editor.set_show_gutter(false, cx);
    commit_editor.set_show_wrap_guides(false, cx);
    commit_editor.set_show_indent_guides(false, cx);
    commit_editor.set_placeholder_text(&placeholder.unwrap_or("Enter commit message".into()), window, cx);
    commit_editor
}
```

### Real example C — auto-height JSON editor in a modal (`crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs:84-100`)

```rust
fn create_editor(json: String, jsonc_language: Option<Arc<Language>>, window: &mut Window, cx: &mut App) -> Entity<Editor> {
    cx.new(|cx| {
        let mut editor = Editor::auto_height(4, 16, window, cx);
        editor.set_text(json, window, cx);
        editor.set_show_gutter(false, cx);
        editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
        if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
            buffer.update(cx, |buffer, cx| buffer.set_language(jsonc_language, cx))
        }
        editor
    })
}
```

### Rendering the embedded editor

Two options seen in-tree:

1. **`EditorElement::new(&entity, style)`** — full control over fonts/syntax theme. Signature: `pub fn new(editor: &Entity<Editor>, style: EditorStyle) -> Self` (`crates/editor/src/element.rs:240`). Agent panel (`message_editor.rs:2012-2037`):
```rust
.child({
    let settings = ThemeSettings::get_global(cx);
    let text_style = TextStyle {
        color: cx.theme().colors().text,
        font_family: settings.buffer_font.family.clone(),
        font_fallbacks: settings.buffer_font.fallbacks.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_size: settings.agent_buffer_font_size(cx).into(),
        font_weight: settings.buffer_font.weight,
        line_height: relative(settings.buffer_line_height.value()),
        ..Default::default()
    };
    EditorElement::new(
        &self.editor,
        EditorStyle {
            background: cx.theme().colors().editor_background,
            local_player: cx.theme().players().local(),
            text: text_style,
            syntax: cx.theme().syntax().clone(),   // REQUIRED for syntax highlighting colors
            inlay_hints_style: editor::make_inlay_hints_style(cx),
            ..Default::default()
        },
    )
})
```
Reusable style builder: `git_commit_editor_style(font_size: gpui::Pixels, cx: &App) -> EditorStyle` (`crates/git_ui/src/git_panel.rs:7424-7443`), used at git_panel.rs:5285 as `.child(EditorElement::new(&self.commit_editor, panel_editor_style))`.

2. **`.child(self.editor.clone())`** — `Editor` implements `Render`, so the entity can be a child directly with default styling. Real uses: `crates/collab_ui/src/channel_view.rs:429`, `crates/repl/src/notebook/cell.rs:598`, `crates/keymap_editor/src/keymap_editor.rs:3487`.

Useful setters (all in `crates/editor/src/editor.rs`): `set_placeholder_text` (:3088), `set_read_only(&mut self, read_only: bool)` (:3206), `text(&self, cx: &App) -> String` (:8469), `set_text` (:8488), `set_show_gutter`, `set_soft_wrap_mode`.

## 2. Setting the buffer language by name

Agent panel setting Markdown (`crates/agent_ui/src/message_editor.rs:588-602`) — the canonical async pattern:
```rust
if let Some(language_registry) = language_registry {
    let editor = editor.clone();
    cx.spawn(async move |_, cx| {
        let markdown = language_registry.language_for_name("Markdown").await?;
        editor.update(cx, |editor, cx| {
            if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                buffer.update(cx, |buffer, cx| {
                    buffer.set_language(Some(markdown), cx);
                });
            }
        });
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
```
Get the registry from a project: `project.read(cx).languages().clone()` (message_editor.rs:462-464). Also call `buffer.set_language_registry(language_registry.clone())` at buffer creation (message_editor.rs:470) so injections/highlight queries load.

Signatures:
- `LanguageRegistry::language_for_name(self: &Arc<Self>, name: &str) -> impl Future<Output = Result<Arc<Language>>>` — `crates/language/src/language_registry.rs:538`. Name match is **case-insensitive** (`UniCase`), so `"SQL"` matches a language named `"sql"`.
- `LanguageRegistry::language_for_name_or_extension(self: &Arc<Self>, string: &str) -> impl Future<Output = Result<Arc<Language>>>` — language_registry.rs:590 (matches name OR path suffix, e.g. `"sql"`).
- `LanguageRegistry::available_language_for_name(self: &Arc<Self>, name: &str) -> Option<AvailableLanguage>` — language_registry.rs:617 (sync existence check, exact name).
- `Buffer::set_language(&mut self, language: Option<Arc<Language>>, cx: &mut Context<Self>)` — `crates/language/src/buffer.rs:1489`.
- `Buffer::set_language_registry(&self, language_registry: Arc<LanguageRegistry>)` — buffer.rs:1515.

Sync fetch + await variant (jsonc, `crates/agent_ui/src/agent_configuration/configure_context_server_modal.rs:540`):
```rust
let jsonc_language = language_registry.language_for_name("jsonc").await.ok();
```

## 3. SQL is NOT built into Zed — it is an extension

Verified in the tree:
- `crates/languages/src/` contains only: `bash.rs, c.rs, cpp.rs, css.rs, eslint.rs, go.rs, json.rs, lib.rs, package_json.rs, python.rs, rust.rs, tailwind.rs, tailwindcss.rs, typescript.rs, vtsls.rs, yaml.rs`. No SQL.
- The full built-in language list (`crates/languages/src/lib.rs:88-220`): bash, c, cpp, css, diff, go, gomod, gowork, json, jsonc, markdown, markdown-inline, python, rust, tsx, typescript, javascript, jsdoc, regex, yaml, gitcommit, zed-keybind-context.
- No `tree-sitter-sql` dependency anywhere in workspace `Cargo.toml`s.
- In-tree `extensions/` dir contains only: glsl, html, proto, test-extension.
- SQL ships as an **external extension** named `"sql"`: `crates/extensions_ui/src/extension_suggest.rs:69` has `("sql", &["sql"])` — Zed suggests installing the `sql` extension when a `.sql` file is opened.

Consequence for a SQL scratch editor: `language_registry.language_for_name("SQL")` returns `Err` unless the user has the SQL extension installed (extension languages register into the same `LanguageRegistry` at runtime). Handle the error and fall back to plain text, e.g. `let sql = language_registry.language_for_name("SQL").await.ok();` then `buffer.set_language(sql, cx)` (set_language takes `Option<Arc<Language>>`, `None` = plain text). You can check availability synchronously with `available_language_for_name("SQL")` (language_registry.rs:617) and prompt the user to install the extension.

## 4. Key bindings in the embedded editor (cmd-enter to submit)

### Pattern: wrap the editor in a div with `key_context` + `on_action`

Agent panel (`crates/agent_ui/src/message_editor.rs:1999-2010`):
```rust
impl Render for MessageEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("MessageEditor")
            .on_action(cx.listener(Self::chat))
            .on_action(cx.listener(Self::send_immediately))
            .on_action(cx.listener(Self::chat_with_follow))
            .on_action(cx.listener(Self::cancel))
            .capture_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::paste_raw))
            .flex_1()
            .child(EditorElement::new(&self.editor, /* style */ ...))
    }
}
```
Handler signature (message_editor.rs:1003):
```rust
fn chat(&mut self, _: &Chat, _: &mut Window, cx: &mut Context<Self>) {
    self.send(cx);
}
```
`Chat` action is defined via the `actions!` macro in `crates/zed_actions/src/lib.rs:539-566` (`actions!(agent, [ ... /// Starts a chat conversation with the agent. Chat, ... ])`). Define your own with `actions!(sql_scratch, [Run])` — doc comments become user-visible descriptions (see `crates/agent_ui/src/agent_ui.rs:184`).

The outer thread view sets an enclosing context + focus tracking (`crates/agent_ui/src/conversation_view/thread_view.rs:11618-11620`):
```rust
v_flex()
    .key_context("AcpThread")
    .track_focus(&self.focus_handle)
    .on_action(cx.listener(|this, _: &menu::Cancel, _, cx| { ... }))
```

### Keymap side — bindings target `<Context> > Editor` because focus is inside the inner Editor

`assets/keymaps/default-macos.json:423-437`:
```json
{
  "context": "AcpThread > Editor && !use_modifier_to_send",
  "use_key_equivalents": true,
  "bindings": { "enter": "agent::Chat" }
},
{
  "context": "AcpThread > Editor && use_modifier_to_send",
  "use_key_equivalents": true,
  "bindings": { "cmd-enter": "agent::Chat", "enter": "editor::Newline" }
}
```
The action dispatches from the focused inner `Editor`, bubbles up the element tree, and is handled by the wrapping div's `on_action`.

The `use_modifier_to_send` context flag is injected via an editor `Addon` (`crates/agent_ui/src/message_editor.rs:2049-2064`), registered with `editor.register_addon(MessageEditorAddon::new())` (message_editor.rs:488):
```rust
impl Addon for MessageEditorAddon {
    fn to_any(&self) -> &dyn std::any::Any { self }
    fn to_any_mut(&mut self) -> Option<&mut dyn std::any::Any> { Some(self) }
    fn extend_key_context(&self, key_context: &mut KeyContext, cx: &App) {
        let settings = agent_settings::AgentSettings::get_global(cx);
        if settings.use_modifier_to_send {
            key_context.add("use_modifier_to_send");
        }
    }
}
```

### Alternative: dynamic KeyContext (git panel commit editor)

`crates/git_ui/src/git_panel.rs:1260-1272`:
```rust
fn dispatch_context(&self, window: &mut Window, cx: &Context<Self>) -> KeyContext {
    let mut dispatch_context = KeyContext::new_with_defaults();
    dispatch_context.add("GitPanel");
    if self.commit_editor.read(cx).is_focused(window) {
        dispatch_context.add("CommitEditor");
    } else if self.focus_handle.contains_focused(window, cx) {
        dispatch_context.add("menu");
        dispatch_context.add("ChangesList");
    }
    dispatch_context
}
```
Applied in render (git_panel.rs:7211-7216): `.key_context(self.dispatch_context(window, cx))` followed by `.on_action(cx.listener(GitPanel::on_commit))` etc. Keymap (`assets/keymaps/default-macos.json:1124-1128`):
```json
{
  "context": "CommitEditor > Editor",
  "use_key_equivalents": true,
  "bindings": { "enter": "editor::Newline", "cmd-enter": "git::Commit" }
}
```

### Focus delegation

Delegate the wrapping view's focus to the inner editor so keymap contexts resolve (`crates/agent_ui/src/message_editor.rs:1993-1997`):
```rust
impl Focusable for MessageEditor {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}
```

## Minimal SQL scratch editor recipe (assembled from the above real code)

```rust
// in YourView::new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>)
let language_registry = project.read(cx).languages().clone();
let editor = cx.new(|cx| {
    let buffer = cx.new(|cx| {
        let buffer = Buffer::local("", cx);
        buffer.set_language_registry(language_registry.clone());
        buffer
    });
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
    editor.set_placeholder_text("Enter SQL...", window, cx);
    editor
});
cx.spawn(async move |_, cx| {
    // SQL comes from the "sql" extension; Err if not installed -> stays plain text
    let sql = language_registry.language_for_name("SQL").await.ok();
    editor.update(cx, |editor, cx| {
        if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
            buffer.update(cx, |buffer, cx| buffer.set_language(sql, cx));
        }
    })
})
.detach();
// render(): div().key_context("SqlScratch").on_action(cx.listener(Self::run_query)).child(EditorElement::new(&self.editor, style))
// keymap: { "context": "SqlScratch > Editor", "bindings": { "cmd-enter": "sql_scratch::Run", "enter": "editor::Newline" } }
```
