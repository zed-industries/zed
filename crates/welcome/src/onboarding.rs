use anyhow::anyhow;
use editor::Editor;
use gpui::{
    AnyElement, Bounds, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ListAlignment, ListState, ScrollHandle, SharedString, Task, UniformListScrollHandle,
    WeakEntity, canvas, div,
};
use language::LanguageRegistry;
use persistence::WALKTHROUGH_DB;
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use std::sync::Arc;
use ui::prelude::*;
use util::ResultExt;

use workspace::{
    AppState, Item, SerializableItem, Workspace, WorkspaceId, delete_unloaded_items,
    item::ItemEvent,
};

pub fn init(app_state: Arc<AppState>, cx: &mut App) {
    workspace::register_serializable_item::<OnboardingWalkthrough>(cx);

    let app_state = app_state.clone();

    cx.observe_new(move |workspace: &mut Workspace, _window, cx| {
        let app_state = app_state.clone();
        let weak_workspace = cx.entity().downgrade();

        workspace.register_action(
            move |workspace, _: &workspace::OnboardingWalkthrough, window, cx| {
                let app_state = app_state.clone();
                let language_registry = app_state.languages.clone();

                let walkthrough = cx.new(|cx| {
                    OnboardingWalkthrough::new(
                        weak_workspace.clone(),
                        language_registry.clone(),
                        window,
                        cx,
                    )
                    // todo!("fail more graceefully")
                    .expect("Failed to create OnboardingWalkthrough")
                });
                workspace.add_item_to_active_pane(Box::new(walkthrough), None, true, window, cx)
            },
        );
    })
    .detach();
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum WalkthroughPage {
    #[default]
    Theme,
    KeyBindings,
    Extensions,
    Settings,
}

pub struct OnboardingWalkthrough {
    active_page: WalkthroughPage,
    focus_handle: FocusHandle,
    language_registry: Arc<LanguageRegistry>,
    weak_handle: WeakEntity<Workspace>,
    nav_picker: Entity<Picker<OnboardingNavDelegate>>,
    nav_scroll_handle: UniformListScrollHandle,
    page_scroll_handle: ScrollHandle,
    page_list: ListState,
    last_bounds: Option<Bounds<Pixels>>,
}

impl OnboardingWalkthrough {
    pub fn new(
        weak_handle: WeakEntity<Workspace>,
        language_registry: Arc<LanguageRegistry>,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> anyhow::Result<Self> {
        let nav_delegate = OnboardingNavDelegate::new(cx.entity().downgrade(), 0);
        let nav_picker = cx.new(|cx| {
            let picker = Picker::uniform_list(nav_delegate, window, cx);
            picker.focus(window, cx);
            picker
        });
        let entity = cx.entity().downgrade();
        let list_state = ListState::new(
            0,
            gpui::ListAlignment::Bottom,
            px(1000.),
            move |ix, _window, cx| {
                if let Some(entity) = entity.upgrade() {
                    entity.update(cx, |this: &mut Self, cx| {
                        this.render_page(ix, cx).into_any_element()
                    })
                } else {
                    div().into_any()
                }
            },
        );

        let welcome = Self {
            active_page: WalkthroughPage::default(),
            focus_handle: cx.focus_handle(),
            language_registry,
            weak_handle,
            nav_picker,
            nav_scroll_handle: UniformListScrollHandle::new(),
            page_scroll_handle: ScrollHandle::new(),
            page_list: list_state,
            last_bounds: None,
        };

        Ok(welcome)
    }

    fn set_active_page(&mut self, page: WalkthroughPage, cx: &mut gpui::Context<Self>) {
        if self.active_page != page {
            self.active_page = page;
            cx.emit(ItemEvent::UpdateTab);
            cx.notify();
        }
    }

    fn render_page(&mut self, ix: usize, cx: &mut Context<Self>) -> AnyElement {
        // todo!("render page based on ix")
        div().child(format!("Page {}", ix)).into_any_element()
    }

    fn render_theme_page(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div().child("Theme")
    }

    fn render_keybindings_page(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div().child("Keybindings")
    }

    fn render_extensions_page(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div().child("Extensions")
    }

    fn render_settings_page(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div().child("Settings")
    }

    fn render_active_page(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        match self.active_page {
            WalkthroughPage::Theme => self.render_theme_page(window, cx).into_any_element(),
            WalkthroughPage::KeyBindings => {
                self.render_keybindings_page(window, cx).into_any_element()
            }
            WalkthroughPage::Extensions => {
                self.render_extensions_page(window, cx).into_any_element()
            }
            WalkthroughPage::Settings => self.render_settings_page(window, cx).into_any_element(),
        }
    }

    fn render_navigation(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div()
            .w(rems(20.))
            .h_full()
            // .border_r_1()
            // .border_color(ui::Color::Muted)
            .child(self.nav_picker.clone())
    }
}

impl Render for OnboardingWalkthrough {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let this = cx.entity();
        // this whole canvas bit is so we can get the bounds so we can
        // react to smaller sizes
        canvas(
            move |bounds, window, cx| {
                this.update(cx, |this, cx| {
                    this.last_bounds = Some(dbg!(bounds));

                    let mut elements = h_flex()
                        .debug_below()
                        .h_full()
                        .w_full()
                        .child(
                            h_flex()
                                .w_full()
                                .max_w(px(800.))
                                .h_full()
                                .gap_6()
                                .child(this.render_navigation(window, cx))
                                .child(
                                    div()
                                        .flex_1()
                                        .h_full()
                                        .child(this.render_active_page(window, cx)),
                                ),
                        )
                        .into_any();

                    elements.prepaint_as_root(
                        bounds.origin,
                        bounds.size.map(Into::into),
                        window,
                        cx,
                    );
                    elements
                })
            },
            |_, mut elements, window, cx| {
                elements.paint(window, cx);
            },
        )
        .size_full()
    }
}

struct OnboardingNavDelegate {
    welcome: WeakEntity<OnboardingWalkthrough>,
    selected_ix: usize,
    nav_items: Vec<(SharedString, WalkthroughPage)>,
}

impl OnboardingNavDelegate {
    fn new(welcome: WeakEntity<OnboardingWalkthrough>, selected_ix: usize) -> Self {
        Self {
            welcome,
            selected_ix,
            nav_items: vec![
                ("Theme".into(), WalkthroughPage::Theme),
                ("Key Bindings".into(), WalkthroughPage::KeyBindings),
                ("Extensions".into(), WalkthroughPage::Extensions),
                ("Settings".into(), WalkthroughPage::Settings),
            ],
        }
    }
}

impl PickerDelegate for OnboardingNavDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.nav_items.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Picker<Self>>,
    ) {
        if ix < self.nav_items.len() {
            self.selected_ix = ix;

            // Update the active page in Welcome2
            if let Some(welcome) = self.welcome.upgrade() {
                welcome.update(cx, |welcome, cx| {
                    let page = self.nav_items[ix].1.clone();
                    welcome.set_active_page(page, cx);
                });
            }
        }
    }

    fn placeholder_text(&self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> Arc<str> {
        "Navigation".into()
    }

    fn update_matches(
        &mut self,
        _query: String,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Picker<Self>>,
    ) -> Task<()> {
        // We don't filter nav items, so just return a completed task
        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Picker<Self>>,
    ) {
        // Just set the active page again to ensure it's set
        if let Some(welcome) = self.welcome.upgrade() {
            welcome.update(cx, |welcome, cx| {
                let page = self.nav_items[self.selected_ix].1.clone();
                welcome.set_active_page(page, cx);
            });
        }
    }

    fn dismissed(&mut self, _window: &mut gpui::Window, _cx: &mut gpui::Context<Picker<Self>>) {
        // No-op
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        if ix >= self.nav_items.len() {
            return None;
        }

        let (name, _) = &self.nav_items[ix];

        Some(
            div()
                .px_4()
                .py_2()
                // .bg(if selected {
                //     Color::Accent
                // } else {
                //     Color::default_bg().with_alpha_factor(0.0)
                // })
                // .text_color(if selected {
                //     Color::White
                // } else {
                //     Color::Default
                // })
                .rounded_md()
                // .when(selected, |div| div.font_weight(FontWeight::BOLD))
                .child(name.clone())
                .into_any_element(),
        )
    }

    fn editor_position(&self) -> PickerEditorPosition {
        // Hide the editor since we're just using this for navigation
        PickerEditorPosition::End
    }

    fn render_editor(
        &self,
        _editor: &gpui::Entity<Editor>,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Picker<Self>>,
    ) -> Div {
        // Return an empty div to hide the editor
        div()
    }
}

impl EventEmitter<ItemEvent> for OnboardingWalkthrough {}

impl Focusable for OnboardingWalkthrough {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for OnboardingWalkthrough {
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>> {
        let weak_handle = self.weak_handle.clone();
        let language_registry = self.language_registry.clone();

        let onboarding_result =
            OnboardingWalkthrough::new(weak_handle, language_registry, window, cx).log_err();

        if let Some(onboarding) = onboarding_result {
            Some(cx.new(|_| onboarding))
        } else {
            None
        }
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}

impl SerializableItem for OnboardingWalkthrough {
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
            &WALKTHROUGH_DB,
            cx,
        )
    }

    fn deserialize(
        project: Entity<project::Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<Entity<Self>>> {
        // todo!("update")
        let has_walkthrough = WALKTHROUGH_DB.get_walkthrough(item_id, workspace_id);
        let language_registry = project.read(cx).languages().clone();

        let weak_handle = workspace.clone();
        if has_walkthrough.is_ok() {
            Task::ready(Ok(cx.new(|cx| {
                // todo!{"can we do this more safely?"}
                OnboardingWalkthrough::new(weak_handle, language_registry, window, cx)
                    .expect("failed to create onboarding walkthrough")
            })))
        } else {
            Task::ready(Err(anyhow!("No walkthrough for itemID {}", item_id)))
        }
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
