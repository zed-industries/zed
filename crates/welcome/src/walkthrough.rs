use client::telemetry::Telemetry;

use fs::Fs;
use gpui::{
    App, ClickEvent, Context, Entity, EventEmitter, FocusHandle, Focusable, ListSizingBehavior,
    ListState, ParentElement, Render, Styled, Subscription, TextOverflow, WeakEntity, Window, list,
    svg,
};
use persistence::WALKTHROUGH_DB;
use settings::SettingsStore;
use std::sync::Arc;
use theme::ThemeRegistry;
use theme::{Appearance, ThemeSettings};
use ui::prelude::*;
use workspace::{
    SerializableItem, Workspace, WorkspaceId, delete_unloaded_items,
    item::{Item, ItemEvent},
    register_serializable_item,
};
use zed_actions::{ExtensionCategoryFilter, Extensions};

use crate::welcome_ui::theme_preview::ThemePreviewTile;
use crate::welcome_ui::transparent_tabs::TransparentTabs;

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

pub struct Walkthrough {
    active_step: usize,
    workspace: WeakEntity<Workspace>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    _telemetry: Arc<Telemetry>,
    list: ListState,
    steps: Vec<Box<dyn WalkthroughStep>>,
    _settings_subscription: Subscription,
}

impl Walkthrough {
    pub fn checkbox_section(
        &mut self,
        ix: usize,
        title: &'static str,
        description: &'static str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let active = ix == self.active_step;
        let theme = cx.theme().clone();

        div()
            .size_full()
            .p_2()
            .child(
                h_flex()
                    .rounded_md()
                    .size_full()
                    .p_4()
                    .border_1()
                    .when(active, |div| div.bg(theme.colors().element_background))
                    .id(title)
                    .on_click(select_step(ix, cx))
                    .border_color(theme.colors().border)
                    .child(v_flex().child(Label::new(title)).when(active, |div| {
                        div.text_sm()
                            .size_full()
                            .text_color(theme.colors().text_muted)
                            .child(description)
                    })),
            )
            .into_any()
    }

    pub fn new(workspace: &Workspace, cx: &mut Context<Workspace>) -> Entity<Self> {
        let this = cx.new(|cx| {
            let this = cx.weak_entity();
            let steps = vec![
                Box::new(ThemeStep {
                    selection: cx.new(|_| 0),
                }) as Box<dyn WalkthroughStep>,
                Box::new(SettingsStep),
                Box::new(AiIntegrations),
                Box::new(DataSharing),
                Box::new(OpenProject),
            ];
            Walkthrough {
                focus_handle: cx.focus_handle(),
                workspace: workspace.weak_handle(),
                fs: workspace.app_state().fs.clone(),
                _telemetry: workspace.client().telemetry().clone(),
                _settings_subscription: cx
                    .observe_global::<SettingsStore>(move |_: &mut Walkthrough, cx| cx.notify()),
                steps,
                list: ListState::new(
                    steps.len(),
                    gpui::ListAlignment::Top,
                    px(1000.),
                    move |ix, window, cx| {
                        this.update(cx, |this, cx| {
                            let current_step = &this.steps[ix];
                            current_step.render_checkbox(ix, this, window, cx)
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
        div()
            .size_full()
            .key_context("Walkthrough")
            .bg(cx.theme().colors().editor_background)
            .track_focus(&self.focus_handle(cx))
            .p_5()
            .child(
                v_flex()
                    .size_full()
                    .items_center()
                    .justify_center()
                    .relative()
                    .child(
                        v_flex()
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
                            .w(px(768.))
                            .h_full()
                            .child(
                                list(self.list.clone())
                                    .with_sizing_behavior(ListSizingBehavior::Infer)
                                    .h_full()
                                    .w_96(),
                            )
                            .child(div().w_96().h_full().child(
                                self.steps[self.active_step].render_subpane(
                                    self.active_step,
                                    self,
                                    window,
                                    cx,
                                ),
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

struct ThemeStep {
    selection: Entity<usize>,
}
impl WalkthroughStep for ThemeStep {
    fn render_checkbox(
        &self,
        ix: usize,
        walkthrough: &mut Walkthrough,
        window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        walkthrough.checkbox_section(
            ix,
            "Pick a Theme",
            "Select one of our built-in themes, or download one from the extensions page",
            cx,
        )
    }

    //             .child(

    fn render_subpane(
        &self,
        _ix: usize,
        walkthrough: &mut Walkthrough,
        window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        const THEME_PREVIEW_SEED: f32 = 0.42;

        let fs = walkthrough.fs.clone();

        let mut theme_preview_tile = |theme_name: &str, fs: Arc<dyn Fs>| -> Option<AnyElement> {
            let theme_registry = ThemeRegistry::global(cx);
            let theme = theme_registry.clone().get(theme_name).ok()?;
            let current_theme = cx.theme().clone();
            let is_selected = current_theme.id == theme.id;
            Some(
                v_flex()
                    .items_center()
                    .id(theme.name.clone())
                    .child(
                        div().w(px(200.)).h(px(120.)).child(
                            ThemePreviewTile::new(theme.clone(), is_selected, THEME_PREVIEW_SEED)
                                .render(window, cx)
                                .into_any_element(),
                        ),
                    )
                    .text_ui_sm(cx)
                    .child(theme.name.clone())
                    .on_click(move |_event, window, cx| {
                        let name = theme_name.to_string();
                        // TODO: filter click event?
                        telemetry::event!("Settings Changed", setting = "theme", value = &name);
                        settings::update_settings_file::<ThemeSettings>(
                            fs.clone(),
                            cx,
                            move |settings, _| {
                                settings.set_static_theme(name);
                            },
                        );
                    })
                    .into_any(),
            )
        };

        v_flex()
            .size_full()
            .child(
                TransparentTabs::new(self.selection.clone())
                    .tab("Dark", |_window, _app| {
                        v_flex()
                            .child(theme_preview_tile("One Dark", fs.clone()).unwrap())
                            .child(theme_preview_tile("Ayu Dark", fs.clone()).unwrap())
                            .child(theme_preview_tile("Gruvbox Dark", fs.clone()).unwrap())
                    })
                    .tab("Light", |_window, _app| {
                        v_flex()
                            .child(theme_preview_tile("One Light", fs.clone()).unwrap())
                            .child(theme_preview_tile("Ayu Light", fs.clone()).unwrap())
                            .child(theme_preview_tile("Gruvbox Light", fs.clone()).unwrap())
                    })
                    // TODO: picking a theme in the system tab should set both your light and dark themes
                    .tab("System", |window, _app| {
                        let current = match window.appearance() {
                            gpui::WindowAppearance::Light
                            | gpui::WindowAppearance::VibrantLight => "light",
                            gpui::WindowAppearance::Dark | gpui::WindowAppearance::VibrantDark => {
                                "dark"
                            }
                        };
                        v_flex()
                            .child(
                                theme_preview_tile(&format!("One {current}"), fs.clone()).unwrap(),
                            )
                            .child(
                                theme_preview_tile(&format!("Ayu {current}"), fs.clone()).unwrap(),
                            )
                            .child(
                                theme_preview_tile(&format!("Gruvbox {current}"), fs.clone())
                                    .unwrap(),
                            )
                    }),
            )
            .child(
                h_flex().justify_between().children([Button::new(
                    "install-theme",
                    "Browse More Themes",
                )
                .icon(IconName::SwatchBook)
                .icon_size(IconSize::XSmall)
                .icon_color(Color::Muted)
                .icon_position(IconPosition::Start)
                .on_click(cx.listener(|this, _, window, cx| {
                    telemetry::event!("Welcome Theme Changed");
                    this.workspace
                        .update(cx, |_workspace, cx| {
                            window.dispatch_action(
                                Box::new(Extensions {
                                    category_filter: Some(ExtensionCategoryFilter::Themes),
                                }),
                                cx,
                            );
                        })
                        .ok();
                }))]),
            )
            .into_any()
    }
}

struct SettingsStep;
impl WalkthroughStep for SettingsStep {
    fn render_checkbox(
        &self,
        ix: usize,
        walkthrough: &mut Walkthrough,
        _window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        walkthrough.checkbox_section(
            ix,
            "Configure Zed",
            "Set initial settings and/or import from other editors",
            cx,
        )
    }

    fn render_subpane(
        &self,
        _ix: usize,
        _walkthrough: &mut Walkthrough,
        _window: &mut Window,
        _cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        v_flex()
            .items_center()
            .justify_center()
            .child(h_flex().children([
                "VS Code",
                "Atom",
                "Sublime",
                "Jetbrains",
                "Text Mate",
                "Emacs (beta)",
            ]))
            .child("vim mode checkbox")
            .child("browse extensions")
            .when(cfg!(macos), |this| {
                this.child(
                    h_flex()
                        .child(Button::new("install-cli", "Install cli"))
                        .child("Install a `zed` binary that\ncan be run from the command line"),
                )
            })
            .when(
                true,
                // todo!("when this path exists: {}", paths::vscode_settings_file()),
                |this| {
                    this.child(
                        h_flex()
                            .child(Button::new("import-vscode", "Import VsCode settings"))
                            .child(format!("settings file last modified {}", "TODO ago",)),
                    )
                },
            )
            .child(h_flex().children(["open settings", "open keymap", "open config docs"]))
            .into_any()
    }
}

struct AiIntegrations;
impl WalkthroughStep for AiIntegrations {
    fn render_checkbox(
        &self,
        ix: usize,
        walkthrough: &mut Walkthrough,
        _window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        walkthrough.checkbox_section(
            ix,
            "AI Setup",
            "Log in and pick providers for agentic editing and edit predictions",
            cx,
        )
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
        walkthrough: &mut Walkthrough,
        _window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        walkthrough.checkbox_section(
            ix,
            "Data Sharing",
            "Pick which data you send to the zed team",
            cx,
        )
    }

    fn render_subpane(
        &self,
        _ix: usize,
        _walkthrough: &mut Walkthrough,
        _window: &mut Window,
        _cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        v_flex()
            .items_center()
            .justify_center()
            .children([
                "Send Crash Reports",
                "Send Telemetry",
                "---",
                "Help Improve completions",
                "Rate agentic edits",
                // TODO: add note about how zed never shares your code/data by default
            ])
            .into_any()
    }
}

struct OpenProject;
impl WalkthroughStep for OpenProject {
    fn render_checkbox(
        &self,
        ix: usize,
        walkthrough: &mut Walkthrough,
        _window: &mut Window,
        cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        walkthrough.checkbox_section(
            ix,
            "Open a Project",
            "Pick a recent project you had open in another editor",
            cx,
        )
    }

    fn render_subpane(
        &self,
        _ix: usize,
        _walkthrough: &mut Walkthrough,
        _window: &mut Window,
        _cx: &mut Context<Walkthrough>,
    ) -> AnyElement {
        // spinner "searching for recent projects" while running? or just have them pop-in
        // single list sorted by mtime (with source editor icons) or separate into separate lists by source?
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
