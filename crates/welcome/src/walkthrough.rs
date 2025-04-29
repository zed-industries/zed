use client::telemetry::Telemetry;

use fs::Fs;
use gpui::{
    App, ClickEvent, Context, Entity, EventEmitter, FocusHandle, Focusable, ListSizingBehavior,
    ListState, ParentElement, Render, Styled, Subscription, WeakEntity, Window, list, svg,
};
use persistence::WALKTHROUGH_DB;
use settings::SettingsStore;
use std::sync::Arc;
use theme::{Appearance, ThemeSettings};
use ui::prelude::*;
use workspace::{
    SerializableItem, Workspace, WorkspaceId, delete_unloaded_items,
    item::{Item, ItemEvent},
    register_serializable_item,
};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _cx| {
        workspace.register_action(|workspace, _: &workspace::Walkthrough, window, cx| {
            let walkthrough = Walkthrough::new(workspace, cx);
            workspace.add_item_to_active_pane(Box::new(walkthrough), None, true, window, cx)
        });
    })
    .detach();

    register_serializable_item::<Walkthrough>(cx);
}

const STEPS: [&'static dyn WalkthroughStep; 5] = [
    &ThemeStep,
    &SettingsStep,
    &AiIntegrations,
    &DataSharing,
    &OpenProject,
];

pub struct Walkthrough {
    active_step: usize,
    workspace: WeakEntity<Workspace>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    _telemetry: Arc<Telemetry>,
    steps: ListState,
    _settings_subscription: Subscription,
}

impl Walkthrough {
    pub fn new(workspace: &Workspace, cx: &mut Context<Workspace>) -> Entity<Self> {
        let this = cx.new(|cx| {
            let this = cx.weak_entity();
            Walkthrough {
                focus_handle: cx.focus_handle(),
                workspace: workspace.weak_handle(),
                fs: workspace.app_state().fs.clone(),
                _telemetry: workspace.client().telemetry().clone(),
                _settings_subscription: cx
                    .observe_global::<SettingsStore>(move |_: &mut Walkthrough, cx| cx.notify()),
                steps: ListState::new(
                    STEPS.len(),
                    gpui::ListAlignment::Top,
                    px(1000.),
                    move |ix, window, cx| {
                        this.update(cx, |this, cx| {
                            STEPS[ix].render_checkbox(ix, this, window, cx)
                        })
                        .unwrap_or_else(|_| div().into_any())
                    },
                ),
                active_step: 0,
            }
        });

        this
    }
}

impl Render for Walkthrough {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .justify_center()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .key_context("Walkthrough")
            .track_focus(&self.focus_handle(cx))
            .child(
                v_flex()
                    .child(
                        v_flex()
                            .w_full()
                            .child(
                                svg()
                                    .path("icons/logo_96.svg")
                                    .text_color(cx.theme().colors().icon_disabled)
                                    .w(px(40.))
                                    .h(px(40.))
                                    .mx_auto()
                                    .mb_4(),
                            )
                            .child(
                                h_flex()
                                    .w_full()
                                    .justify_center()
                                    .child(Headline::new("Welcome to Zed")),
                            )
                            .child(
                                h_flex().w_full().justify_center().child(
                                    Label::new("The editor for what's next")
                                        .color(Color::Muted)
                                        .italic(),
                                ),
                            ),
                    )
                    .child(
                        h_flex()
                            .flex_wrap()
                            .justify_center()
                            .child(
                                list(self.steps.clone())
                                    .with_sizing_behavior(ListSizingBehavior::Infer),
                            )
                            .child(STEPS[self.active_step].render_subpane(
                                self.active_step,
                                self,
                                window,
                                cx,
                            )),
                    ),
            )
    }
}

trait WalkthroughStep {
    fn render_checkbox(
        &self,
        ix: usize,
        walkthrough: &mut Walkthrough,
        window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement;
    fn render_subpane(
        &self,
        ix: usize,
        walkthrough: &mut Walkthrough,
        window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement;
}

fn select_step(
    ix: usize,
    cx: &mut Context<Walkthrough>,
) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    cx.listener(move |walkthrough, _, _, cx| {
        walkthrough.active_step = ix;
        cx.notify();
    })
}

struct ThemeStep;
impl WalkthroughStep for ThemeStep {
    fn render_checkbox(
        &self,
        ix: usize,
        _walkthrough: &mut Walkthrough,
        window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        div()
            .child(Label::new("Pick a Theme").render(window, cx))
            .id(ix)
            .on_click(select_step(ix, cx))
            .into_any_element()
    }

    fn render_subpane(
        &self,
        _ix: usize,
        walkthrough: &mut Walkthrough,
        _window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        let current = cx.theme().name.clone();
        let fs = walkthrough.fs.clone();
        // let registry = ThemeRegistry::global(cx);
        // let mut themes = registry.list();

        let make_button = |name: &'static str, fs: Arc<dyn Fs>| {
            Button::new(name, name)
                .when(current == name, |this| this.toggle_state(true))
                .on_click(move |_event, window, cx| {
                    let name = name.to_string();
                    // TODO: filter click event?
                    telemetry::event!("Settings Changed", setting = "theme", value = &name);
                    let appearance = Appearance::from(window.appearance());
                    let fs = fs.clone();
                    settings::update_settings_file::<ThemeSettings>(fs, cx, move |settings, _| {
                        settings.set_theme(name, appearance);
                    });
                })
        };
        div()
            .children(["One Light", "One Dark"].map(|name| make_button(name, fs.clone())))
            .into_any()
    }
}

struct SettingsStep;
impl WalkthroughStep for SettingsStep {
    fn render_checkbox(
        &self,
        ix: usize,
        _walkthrough: &mut Walkthrough,
        window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        div()
            .child(Label::new("Configure Zed").render(window, cx))
            .id(ix)
            .on_click(select_step(ix, cx))
            .into_any_element()
    }

    fn render_subpane(
        &self,
        _ix: usize,
        _walkthrough: &mut Walkthrough,
        _window: &mut Window,
        _cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        // keymap picker dropdown
        // vim mode checkbox
        // buttons for
        //   - open keymap
        //   - open settings
        //   - extensions
        //   - open https://zed.dev/docs/configuring-zed in browser
        div().size_20().bg(gpui::red()).into_any()
    }
}

struct AiIntegrations;
impl WalkthroughStep for AiIntegrations {
    fn render_checkbox(
        &self,
        ix: usize,
        _walkthrough: &mut Walkthrough,
        window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        div()
            .child(Label::new("AI Setup").render(window, cx))
            .id(ix)
            .on_click(select_step(ix, cx))
            .into_any_element()
    }

    fn render_subpane(
        &self,
        _ix: usize,
        _walkthrough: &mut Walkthrough,
        _window: &mut Window,
        _cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        // agentic setup section
        // inline "try-edit-predictions" section
        div().size_20().bg(gpui::green()).into_any()
    }
}

struct DataSharing;
impl WalkthroughStep for DataSharing {
    fn render_checkbox(
        &self,
        ix: usize,
        _walkthrough: &mut Walkthrough,
        window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        div()
            .child(Label::new("Data Sharing").render(window, cx))
            .id(ix)
            .on_click(select_step(ix, cx))
            .into_any_element()
    }

    fn render_subpane(
        &self,
        _ix: usize,
        _walkthrough: &mut Walkthrough,
        _window: &mut Window,
        _cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        // checkboxes:
        // Send Crash Reports
        // Send Telemetry
        // Share Training Data (disabled)
        div().size_20().bg(gpui::yellow()).into_any()
    }
}

struct OpenProject;
impl WalkthroughStep for OpenProject {
    fn render_checkbox(
        &self,
        ix: usize,
        _walkthrough: &mut Walkthrough,
        window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        div()
            .child(Label::new("Open a Project").render(window, cx))
            .id(ix)
            .on_click(select_step(ix, cx))
            .into_any_element()
    }

    fn render_subpane(
        &self,
        _ix: usize,
        _walkthrough: &mut Walkthrough,
        _window: &mut Window,
        _cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        // spinner "searching for recent projects" while running:

        div().size_20().bg(gpui::rgba(0x87ceeb)).into_any()
    }
}

impl EventEmitter<ItemEvent> for Walkthrough {}

impl Focusable for Walkthrough {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for Walkthrough {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Walkthrough".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Walkthrough Page Opened")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>> {
        self.workspace
            .update(cx, |workspace, cx| Walkthrough::new(workspace, cx))
            .ok()
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}

impl SerializableItem for Walkthrough {
    fn serialized_item_kind() -> &'static str {
        "Walkthrough"
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<workspace::ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<()>> {
        delete_unloaded_items(
            alive_items,
            workspace_id,
            "walkthroughs",
            &*WALKTHROUGH_DB,
            cx,
        )
    }

    fn deserialize(
        _project: Entity<project::Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: workspace::ItemId,
        _window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<Entity<Self>>> {
        let has_walkthrough = WALKTHROUGH_DB.get_walkthrough(item_id, workspace_id);
        cx.spawn(async move |cx| {
            has_walkthrough?;
            workspace.update(cx, |workspace, cx| Walkthrough::new(workspace, cx))
        })
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: workspace::ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<gpui::Task<gpui::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        Some(cx.background_spawn(async move {
            WALKTHROUGH_DB.save_walkthrough(item_id, workspace_id).await
        }))
    }

    fn should_serialize(&self, _event: &Self::Event) -> bool {
        false
    }
}

mod persistence {
    use db::{define_connection, query, sqlez_macros::sql};
    use workspace::{ItemId, WorkspaceDb};

    define_connection! {
        pub static ref WALKTHROUGH_DB: WalkthroughDb<WorkspaceDb> =
            &[sql!(
                CREATE TABLE walkthroughs (
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,
                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
            )];
    }

    impl WalkthroughDb {
        query! {
            pub async fn save_walkthrough(item_id: ItemId, workspace_id: workspace::WorkspaceId) -> Result<()> {
                INSERT INTO walkthroughs(item_id, workspace_id)
                VALUES (?1, ?2)
                ON CONFLICT DO UPDATE SET
                  item_id = ?1,
                  workspace_id = ?2
            }
        }

        query! {
            pub fn get_walkthrough(item_id: ItemId, workspace_id: workspace::WorkspaceId) -> Result<ItemId> {
                SELECT item_id
                FROM walkthroughs
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
