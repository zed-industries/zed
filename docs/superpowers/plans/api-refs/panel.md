# Adding a new dock Panel to Zed

# API Reference: Adding a New Dock Panel to Zed

## 1. The `Panel` trait and `PanelEvent` — `/Users/user/zed/crates/workspace/src/dock.rs`

### `PanelEvent` (lines 27–32)
```rust
pub enum PanelEvent {
    ZoomIn,
    ZoomOut,
    Activate,
    Close,
}
```

### Full `Panel` trait (lines 36–96) — copied verbatim
```rust
pub trait Panel: Focusable + EventEmitter<PanelEvent> + Render + Sized {
    fn persistent_name() -> &'static str;
    fn panel_key() -> &'static str;
    fn position(&self, window: &Window, cx: &App) -> DockPosition;
    fn position_is_valid(&self, position: DockPosition) -> bool;
    fn set_position(&mut self, position: DockPosition, window: &mut Window, cx: &mut Context<Self>);
    fn default_size(&self, window: &Window, cx: &App) -> Pixels;
    fn min_size(&self, _window: &Window, _cx: &App) -> Option<Pixels> {
        None
    }
    fn initial_size_state(&self, _window: &Window, _cx: &App) -> PanelSizeState {
        PanelSizeState::default()
    }
    fn size_state_changed(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}
    fn supports_flexible_size(&self) -> bool {
        false
    }
    fn has_flexible_size(&self, _window: &Window, _cx: &App) -> bool {
        false
    }
    fn set_flexible_size(
        &mut self,
        _flexible: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
    fn icon(&self, window: &Window, cx: &App) -> Option<ui::IconName>;
    fn icon_tooltip(&self, window: &Window, cx: &App) -> Option<&'static str>;
    fn toggle_action(&self) -> Box<dyn Action>;
    fn icon_label(&self, _window: &Window, _: &App) -> Option<String> {
        None
    }
    fn is_zoomed(&self, _window: &Window, _cx: &App) -> bool {
        false
    }
    fn starts_open(&self, _window: &Window, _cx: &App) -> bool {
        false
    }
    fn set_zoomed(&mut self, _zoomed: bool, _window: &mut Window, _cx: &mut Context<Self>) {}
    fn set_active(&mut self, _active: bool, _window: &mut Window, _cx: &mut Context<Self>) {}
    fn pane(&self) -> Option<Entity<Pane>> {
        None
    }
    fn remote_id() -> Option<proto::PanelId> {
        None
    }
    fn activation_priority(&self) -> u32;
    fn enabled(&self, _cx: &App) -> bool {
        true
    }
    fn is_agent_panel(&self) -> bool {
        false
    }
    /// Returns metadata describing how to hide this panel's button from the
    /// status bar by writing to user settings. Implementors should return
    /// `None` if the panel button cannot be hidden through settings.
    fn hide_button_setting(&self, _: &App) -> Option<HideStatusItem> {
        None
    }
}
```
Required (no default): `persistent_name`, `panel_key`, `position`, `position_is_valid`, `set_position`, `default_size`, `icon`, `icon_tooltip`, `toggle_action`, `activation_priority`.

### Supporting types (same file)
```rust
// lines 289–294
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DockPosition {
    Left,
    Bottom,
    Right,
}

// lines 343–348
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PanelSizeState {
    pub size: Option<Pixels>,
    #[serde(default)]
    pub flex: Option<f32>,
}
```
A blanket `impl<T: Panel> PanelHandle for Entity<T>` exists at dock.rs:144, so you never implement `PanelHandle` (dock.rs:98–142) yourself.

## 2. Example implementations

### GitPanel — `/Users/user/zed/crates/git_ui/src/git_panel.rs`

`const GIT_PANEL_KEY: &str = "GitPanel";` (line 94)

Focusable / EventEmitter (lines 7313–7325):
```rust
impl Focusable for GitPanel {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        if self.entries.is_empty() || self.commit_editor_expanded {
            self.commit_editor.focus_handle(cx)
        } else {
            self.focus_handle.clone()
        }
    }
}

impl EventEmitter<Event> for GitPanel {}      // panel-specific events (enum Event { Focus }, line 370)
impl EventEmitter<PanelEvent> for GitPanel {} // REQUIRED by Panel trait
```

`impl Panel for GitPanel` (lines 7352–7412):
```rust
impl Panel for GitPanel {
    fn persistent_name() -> &'static str {
        "GitPanel"
    }

    fn panel_key() -> &'static str {
        GIT_PANEL_KEY
    }

    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        GitPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings.git_panel.get_or_insert_default().dock = Some(position.into())
        });
    }

    fn default_size(&self, _: &Window, cx: &App) -> Pixels {
        GitPanelSettings::get_global(cx).default_width
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<ui::IconName> {
        Some(ui::IconName::GitBranch).filter(|_| GitPanelSettings::get_global(cx).button)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Git Panel")
    }

    fn icon_label(&self, _: &Window, cx: &App) -> Option<String> {
        if !GitPanelSettings::get_global(cx).show_count_badge {
            return None;
        }
        let total = self.changes_count;
        (total > 0).then(|| total.to_string())
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _: &Window, cx: &App) -> bool {
        GitPanelSettings::get_global(cx).starts_open
    }

    fn activation_priority(&self) -> u32 {
        3
    }

    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.git_panel.get_or_insert_default().button = Some(false);
        }))
    }
}
```

`GitPanel::load` (lines 7088–7115) — async constructor pattern used by `initialize_panels` (reads serialized state from the KV store on a background thread, then constructs on the UI thread):
```rust
pub async fn load(
    workspace: WeakEntity<Workspace>,
    mut cx: AsyncWindowContext,
) -> anyhow::Result<Entity<Self>> {
    let serialized_panel = match workspace
        .read_with(&cx, |workspace, cx| {
            Self::serialization_key(workspace).map(|key| (key, KeyValueStore::global(cx)))
        })
        .ok()
        .flatten()
    {
        Some((serialization_key, kvp)) => cx
            .background_spawn(async move { kvp.read_kvp(&serialization_key) })
            .await
            .context("loading git panel")
            .log_err()
            .flatten()
            .map(|panel| serde_json::from_str::<SerializedGitPanel>(&panel))
            .transpose()
            .log_err()
            .flatten(),
        None => None,
    };

    workspace.update_in(&mut cx, |workspace, window, cx| {
        GitPanel::new_with_serialized_panel(workspace, serialized_panel, window, cx)
    })
}
```
Constructor signature (git_panel.rs:860–866; `pub fn new` at ~850 delegates to it with `None`):
```rust
fn new_with_serialized_panel(
    workspace: &mut Workspace,
    serialized_panel: Option<SerializedGitPanel>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Entity<Self> {
    let project = workspace.project().clone();
    let fs = workspace.app_state().fs.clone();
    // ... cx.new(|cx| { let focus_handle = cx.focus_handle(); ...; Self { ... } })
```
`impl Render for GitPanel` is at git_panel.rs:7186.

### ProjectPanel — `/Users/user/zed/crates/project_panel/src/project_panel.rs`

Simpler `load` — no serialization (lines 974–981):
```rust
pub async fn load(
    workspace: WeakEntity<Workspace>,
    mut cx: AsyncWindowContext,
) -> Result<Entity<Self>> {
    workspace.update_in(&mut cx, |workspace, window, cx| {
        ProjectPanel::new(workspace, window, cx)
    })
}
```

`ProjectPanel::new` (lines 654–972, essential shape):
```rust
fn new(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Entity<Self> {
    let project = workspace.project().clone();
    let project_panel = cx.new(|cx| {
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, window, Self::focus_in).detach();

        cx.subscribe_in(&project, window, |this, project, event, window, cx| match event {
            project::Event::ActivateProjectPanel => {
                cx.emit(PanelEvent::Activate);
            }
            // ... many more project events
            _ => {}
        })
        .detach();
        // ... builds Self { focus_handle, project, ... }
    });
    project_panel
}
```

`impl Panel for ProjectPanel` (lines 7503–7573):
```rust
impl EventEmitter<Event> for ProjectPanel {}
impl EventEmitter<PanelEvent> for ProjectPanel {}

impl Panel for ProjectPanel {
    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        match ProjectPanelSettings::get_global(cx).dock {
            DockSide::Left => DockPosition::Left,
            DockSide::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            let dock = match position {
                DockPosition::Left | DockPosition::Bottom => DockSide::Left,
                DockPosition::Right => DockSide::Right,
            };
            settings.project_panel.get_or_insert_default().dock = Some(dock);
        });
    }

    fn default_size(&self, _: &Window, cx: &App) -> Pixels {
        ProjectPanelSettings::get_global(cx).default_width
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<IconName> {
        ProjectPanelSettings::get_global(cx)
            .button
            .then_some(IconName::FileTree)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Project Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn persistent_name() -> &'static str {
        "Project Panel"
    }

    fn panel_key() -> &'static str {
        PROJECT_PANEL_KEY   // const PROJECT_PANEL_KEY: &str = "ProjectPanel"; (line 92)
    }

    fn starts_open(&self, _: &Window, cx: &App) -> bool { /* checks settings + worktrees */ }

    fn activation_priority(&self) -> u32 {
        1
    }

    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.project_panel.get_or_insert_default().button = Some(false);
        }))
    }
}

impl Focusable for ProjectPanel {   // lines 7594–7598
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
```
Imports used (project_panel.rs:75–85):
```rust
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};
use zed_actions::project_panel::{Toggle, ToggleFocus};
```

## 3. Panel registration — two separate steps

### Step A: `<crate>::init(cx)` in `/Users/user/zed/crates/zed/src/main.rs` (registers actions; lines 745–773)
```rust
project_panel::init(cx);      // line 745
outline_panel::init(cx);      // line 746
terminal_view::init(cx);      // line 761
collab_ui::init(&app_state, cx); // line 772
git_ui::init(cx);             // line 773
debugger_ui::init(cx);        // line 593
```
(The same calls are repeated in test setup inside zed.rs, lines 5561–5608.)

### Step B: panel instances added per-workspace-window in `/Users/user/zed/crates/zed/src/zed.rs`

`initialize_panels` is called from window init (zed.rs:625–626: `let panels_task = initialize_panels(window, cx); workspace.set_panels_task(panels_task);`). Definition at zed.rs:748–785:
```rust
fn initialize_panels(window: &mut Window, cx: &mut Context<Workspace>) -> Task<anyhow::Result<()>> {
    cx.spawn_in(window, async move |workspace_handle, cx| {
        let project_panel = ProjectPanel::load(workspace_handle.clone(), cx.clone());
        let outline_panel = OutlinePanel::load(workspace_handle.clone(), cx.clone());
        let terminal_panel = TerminalPanel::load(workspace_handle.clone(), cx.clone());
        let git_panel = GitPanel::load(workspace_handle.clone(), cx.clone());
        let channels_panel =
            collab_ui::collab_panel::CollabPanel::load(workspace_handle.clone(), cx.clone());
        let debug_panel = DebugPanel::load(workspace_handle.clone(), cx);

        async fn add_panel_when_ready(
            panel_task: impl Future<Output = anyhow::Result<Entity<impl workspace::Panel>>> + 'static,
            workspace_handle: WeakEntity<Workspace>,
            mut cx: gpui::AsyncWindowContext,
        ) {
            if let Some(panel) = panel_task.await.context("failed to load panel").log_err()
            {
                workspace_handle
                    .update_in(&mut cx, |workspace, window, cx| {
                        workspace.add_panel(panel, window, cx);
                    })
                    .log_err();
            }
        }

        futures::join!(
            add_panel_when_ready(project_panel, workspace_handle.clone(), cx.clone()),
            add_panel_when_ready(outline_panel, workspace_handle.clone(), cx.clone()),
            add_panel_when_ready(terminal_panel, workspace_handle.clone(), cx.clone()),
            add_panel_when_ready(git_panel, workspace_handle.clone(), cx.clone()),
            add_panel_when_ready(channels_panel, workspace_handle.clone(), cx.clone()),
            add_panel_when_ready(debug_panel, workspace_handle.clone(), cx.clone()),
            initialize_agent_panel(workspace_handle, cx.clone()).map(|r| r.log_err()),
        );

        anyhow::Ok(())
    })
}
```
Imports at zed.rs:35 `use git_ui::git_panel::GitPanel;` and zed.rs:61 `use project_panel::ProjectPanel;`. So a NEW panel needs: (1) `my_panel::init(cx);` in main.rs; (2) `MyPanel::load(...)` + an `add_panel_when_ready(...)` entry in `initialize_panels` in zed.rs; (3) a `my_panel.workspace = true` dep in `crates/zed/Cargo.toml`.

### Key `Workspace` methods — `/Users/user/zed/crates/workspace/src/workspace.rs`
```rust
pub fn add_panel<T: Panel>(&mut self, panel: Entity<T>, window: &mut Window, cx: &mut Context<Self>)   // line 2532
pub fn focus_panel<T: Panel>(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Option<Entity<T>> // line 4298
pub fn toggle_panel_focus<T: Panel>(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool    // line 4311
pub fn open_panel<T: Panel>(&mut self, window: &mut Window, cx: &mut Context<Self>)                    // line 4416
pub fn close_panel<T: Panel>(&self, window: &mut Window, cx: &mut Context<Self>)                       // line 4438
pub fn panel<T: Panel>(&self, cx: &App) -> Option<Entity<T>>                                           // line 4448
```

## 4. Actions: `actions!` + `register_action` + `toggle_action`

### Where `ToggleFocus` is DEFINED — two patterns:

Pattern 1 (in-crate) — git_ui defines it locally, `/Users/user/zed/crates/git_ui/src/git_panel.rs:99–143`:
```rust
actions!(
    git_panel,
    [
        /// Closes the git panel.
        Close,
        /// Toggles the git panel.
        Toggle,
        /// Toggles focus on the git panel.
        ToggleFocus,
        // ... more actions
    ]
);
```

Pattern 2 (shared crate, only needed to avoid dep cycles) — project_panel's actions live in `/Users/user/zed/crates/zed_actions/src/lib.rs:399–411`:
```rust
pub mod project_panel {
    use gpui::actions;

    actions!(
        project_panel,
        [
            /// Toggles the project panel.
            Toggle,
            /// Toggles focus on the project panel.
            ToggleFocus
        ]
    );
}
```
For a new panel, Pattern 1 (define `ToggleFocus` in your own crate) is the default choice. The action namespace string (first `actions!` arg) is what appears in keymaps, e.g. `"git_panel::ToggleFocus"`.

### Registering the handler

project_panel's `init` — `/Users/user/zed/crates/project_panel/src/project_panel.rs:463–472` (uses `cx.observe_new` directly):
```rust
pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<ProjectPanel>(window, cx);
        });
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            if !workspace.toggle_panel_focus::<ProjectPanel>(window, cx) {
                workspace.close_panel::<ProjectPanel>(window, cx);
            }
        });
        // ... more register_action calls; NOTE: init must end with .detach() on the observe_new
    })
    .detach();
}
```

git_ui splits it: `git_panel::register(workspace)` (git_panel.rs:336–344):
```rust
pub fn register(workspace: &mut Workspace) {
    workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
        workspace.toggle_panel_focus::<GitPanel>(window, cx);
    });
    workspace.register_action(|workspace, _: &Toggle, window, cx| {
        if !workspace.toggle_panel_focus::<GitPanel>(window, cx) {
            workspace.close_panel::<GitPanel>(window, cx);
        }
    });
}
```
called from `git_ui::init` — `/Users/user/zed/crates/git_ui/src/git_ui.rs:82–96`:
```rust
pub fn init(cx: &mut App) {
    // ...
    cx.observe_new(|workspace: &mut Workspace, _, cx| {
        git_panel::register(workspace);
        // ...
    })
    .detach();
}
```

### `toggle_action` links the dock button to the action
The `Panel::toggle_action` impl returns the same action, so clicking the dock's status-bar button dispatches it (git_panel.rs:7395–7397, project_panel.rs:7539–7541):
```rust
fn toggle_action(&self) -> Box<dyn Action> {
    Box::new(ToggleFocus)
}
```

## 5. Cargo.toml for a minimal panel crate

From `/Users/user/zed/crates/project_panel/Cargo.toml`. Minimal viable subset for a new panel crate (all `workspace = true` deps):
```toml
[package]
name = "my_panel"
version = "0.1.0"
edition.workspace = true
publish.workspace = true
license = "GPL-3.0-or-later"

[lints]
workspace = true

[lib]
path = "src/my_panel.rs"   # Zed convention: named lib root, no lib.rs
doctest = false

[dependencies]
anyhow.workspace = true
db.workspace = true         # only if serializing panel state (KeyValueStore)
gpui.workspace = true
project.workspace = true    # if you need workspace.project()
serde.workspace = true
serde_json.workspace = true # only for serialized panel state
settings.workspace = true
ui.workspace = true
util.workspace = true       # for .log_err()
workspace.workspace = true  # Panel trait, DockPosition, PanelEvent, Workspace
```
project_panel's full dep list (Cargo.toml:19–54) additionally includes: `collections, command_palette_hooks, editor, file_icons, futures, git_ui, git, itertools, menu, pretty_assertions, schemars, search, smallvec, theme, theme_settings, rayon, client, worktree, language, markdown_preview, zed_actions, telemetry, notifications, feature_flags, fs, log` — most are feature-specific, not required by the Panel plumbing. `zed_actions` is only needed if actions are defined there instead of in your crate. `fs` + `settings` are needed if `set_position` writes to the settings file (`settings::update_settings_file(self.fs.clone(), cx, ...)`); `schemars`/`settings` if you add a `MyPanelSettings` type.

## Checklist for a new panel
1. New crate `crates/my_panel` with `[lib] path = "src/my_panel.rs"`, deps as in section 5.
2. Struct with `focus_handle: FocusHandle` field; `impl Focusable`, `impl EventEmitter<PanelEvent>`, `impl Render`, `impl Panel` (10 required methods, see section 1).
3. `actions!(my_panel, [Toggle, ToggleFocus]);` in your crate; `Panel::toggle_action` returns `Box::new(ToggleFocus)`.
4. `pub fn init(cx: &mut App)` with `cx.observe_new(|workspace: &mut Workspace, _, _| { workspace.register_action(...) }).detach();`.
5. `pub async fn load(workspace: WeakEntity<Workspace>, mut cx: AsyncWindowContext) -> Result<Entity<Self>>` calling `workspace.update_in(&mut cx, |workspace, window, cx| Self::new(workspace, window, cx))`.
6. Wire into the zed crate: `my_panel::init(cx);` in `crates/zed/src/main.rs` (~line 745), `MyPanel::load` + `add_panel_when_ready(...)` in `initialize_panels` in `crates/zed/src/zed.rs` (lines 748–785), and add `my_panel.workspace = true` to `crates/zed/Cargo.toml` plus a `[workspace.dependencies]` entry in the root `Cargo.toml` and the member path in `[workspace] members`.