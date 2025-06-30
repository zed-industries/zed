#![allow(unused, dead_code)]
mod persistence;

use client::Client;
use command_palette_hooks::CommandPaletteFilter;
use feature_flags::FeatureFlagAppExt as _;
use gpui::{
    Entity, EventEmitter, FocusHandle, Focusable, KeyBinding, Task, WeakEntity, actions, prelude::*,
};
use persistence::ONBOARDING_DB;

use project::Project;
use settings_ui::SettingsUiFeatureFlag;
use std::sync::Arc;
use ui::{ListItem, Vector, VectorName, prelude::*};
use util::ResultExt;
use workspace::{
    Workspace, WorkspaceId,
    item::{Item, ItemEvent, SerializableItem},
    notifications::NotifyResultExt,
};

actions!(
    onboarding,
    [
        ShowOnboarding,
        JumpToBasics,
        JumpToEditing,
        JumpToAiSetup,
        JumpToWelcome,
        NextPage,
        PreviousPage,
        ToggleFocus,
        ResetOnboarding,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _cx| {
        workspace.register_action(|workspace, _: &ShowOnboarding, window, cx| {
            let client = workspace.client().clone();
            let onboarding = cx.new(|cx| OnboardingUI::new(workspace, client, cx));
            workspace.add_item_to_active_pane(Box::new(onboarding), None, true, window, cx);
        });
    })
    .detach();

    workspace::register_serializable_item::<OnboardingUI>(cx);

    feature_gate_onboarding_ui_actions(cx);
}

fn feature_gate_onboarding_ui_actions(cx: &mut App) {
    const ONBOARDING_ACTION_NAMESPACE: &str = "onboarding";

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_namespace(ONBOARDING_ACTION_NAMESPACE);
    });

    cx.observe_flag::<SettingsUiFeatureFlag, _>({
        move |is_enabled, cx| {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                if is_enabled {
                    filter.show_namespace(ONBOARDING_ACTION_NAMESPACE);
                } else {
                    filter.hide_namespace(ONBOARDING_ACTION_NAMESPACE);
                }
            });
        }
    })
    .detach();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnboardingPage {
    Basics,
    Editing,
    AiSetup,
    Welcome,
}

impl OnboardingPage {
    fn next(&self) -> Option<Self> {
        match self {
            Self::Basics => Some(Self::Editing),
            Self::Editing => Some(Self::AiSetup),
            Self::AiSetup => Some(Self::Welcome),
            Self::Welcome => None,
        }
    }

    fn previous(&self) -> Option<Self> {
        match self {
            Self::Basics => None,
            Self::Editing => Some(Self::Basics),
            Self::AiSetup => Some(Self::Editing),
            Self::Welcome => Some(Self::AiSetup),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnboardingFocus {
    Navigation,
    Page,
}

pub struct OnboardingUI {
    focus_handle: FocusHandle,
    current_page: OnboardingPage,
    current_focus: OnboardingFocus,
    completed_pages: [bool; 4],

    // Workspace reference for Item trait
    workspace: WeakEntity<Workspace>,
    workspace_id: Option<WorkspaceId>,
    client: Arc<Client>,
}

impl EventEmitter<ItemEvent> for OnboardingUI {}

impl Focusable for OnboardingUI {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Clone)]
pub enum OnboardingEvent {
    PageCompleted(OnboardingPage),
}

impl Render for OnboardingUI {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .bg(cx.theme().colors().editor_background)
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .child(
                h_flex()
                    .id("onboarding-ui")
                    .key_context("Onboarding")
                    .track_focus(&self.focus_handle)
                    .on_action(cx.listener(Self::handle_jump_to_basics))
                    .on_action(cx.listener(Self::handle_jump_to_editing))
                    .on_action(cx.listener(Self::handle_jump_to_ai_setup))
                    .on_action(cx.listener(Self::handle_jump_to_welcome))
                    .on_action(cx.listener(Self::handle_next_page))
                    .on_action(cx.listener(Self::handle_previous_page))
                    .w(px(904.))
                    .h(px(500.))
                    .gap(px(48.))
                    .child(self.render_navigation(window, cx))
                    .child(
                        v_flex()
                            .h_full()
                            .flex_1()
                            .child(div().flex_1().child(self.render_active_page(window, cx))),
                    ),
            )
    }
}

impl OnboardingUI {
    pub fn new(workspace: &Workspace, client: Arc<Client>, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            current_page: OnboardingPage::Basics,
            current_focus: OnboardingFocus::Page,
            completed_pages: [false; 4],
            workspace: workspace.weak_handle(),
            workspace_id: workspace.database_id(),
            client,
        }
    }

    fn completed_pages_to_string(&self) -> String {
        self.completed_pages
            .iter()
            .map(|&completed| if completed { '1' } else { '0' })
            .collect()
    }

    fn completed_pages_from_string(s: &str) -> [bool; 4] {
        let mut result = [false; 4];
        for (i, ch) in s.chars().take(4).enumerate() {
            result[i] = ch == '1';
        }
        result
    }

    fn jump_to_page(
        &mut self,
        page: OnboardingPage,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        self.current_page = page;
        cx.emit(ItemEvent::UpdateTab);
        cx.notify();
    }

    fn next_page(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) {
        if let Some(next) = self.current_page.next() {
            self.current_page = next;
            cx.notify();
        }
    }

    fn previous_page(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) {
        if let Some(prev) = self.current_page.previous() {
            self.current_page = prev;
            cx.notify();
        }
    }

    fn toggle_focus(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) {
        self.current_focus = match self.current_focus {
            OnboardingFocus::Navigation => OnboardingFocus::Page,
            OnboardingFocus::Page => OnboardingFocus::Navigation,
        };
        cx.notify();
    }

    fn reset(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) {
        self.current_page = OnboardingPage::Basics;
        self.current_focus = OnboardingFocus::Page;
        self.completed_pages = [false; 4];
        cx.notify();
    }

    fn mark_page_completed(
        &mut self,
        page: OnboardingPage,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        let index = match page {
            OnboardingPage::Basics => 0,
            OnboardingPage::Editing => 1,
            OnboardingPage::AiSetup => 2,
            OnboardingPage::Welcome => 3,
        };
        self.completed_pages[index] = true;
        cx.notify();
    }

    fn handle_jump_to_basics(
        &mut self,
        _: &JumpToBasics,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.jump_to_page(OnboardingPage::Basics, window, cx);
    }

    fn handle_jump_to_editing(
        &mut self,
        _: &JumpToEditing,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.jump_to_page(OnboardingPage::Editing, window, cx);
    }

    fn handle_jump_to_ai_setup(
        &mut self,
        _: &JumpToAiSetup,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.jump_to_page(OnboardingPage::AiSetup, window, cx);
    }

    fn handle_jump_to_welcome(
        &mut self,
        _: &JumpToWelcome,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.jump_to_page(OnboardingPage::Welcome, window, cx);
    }

    fn handle_next_page(&mut self, _: &NextPage, window: &mut Window, cx: &mut Context<Self>) {
        self.next_page(window, cx);
    }

    fn handle_previous_page(
        &mut self,
        _: &PreviousPage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.previous_page(window, cx);
    }

    fn render_navigation(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let client = self.client.clone();

        v_flex()
            .h_full()
            .w(px(256.))
            .gap_2()
            .justify_between()
            .child(
                v_flex()
                    .w_full()
                    .gap_px()
                    .child(
                        h_flex()
                            .w_full()
                            .justify_between()
                            .py(px(24.))
                            .pl(px(24.))
                            .pr(px(12.))
                            .child(Vector::new(VectorName::ZedLogo, rems(2.), rems(2.)))
                            .child(
                                Button::new("sign_in", "Sign in")
                                    .label_size(LabelSize::Small)
                                    .on_click(cx.listener(move |_, _, window, cx| {
                                        let client = client.clone();
                                        window
                                            .spawn(cx, async move |cx| {
                                                client
                                                    .authenticate_and_connect(true, &cx)
                                                    .await
                                                    .into_response()
                                                    .notify_async_err(cx);
                                            })
                                            .detach();
                                    })),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_px()
                            .py(px(16.))
                            .child(self.render_nav_item(
                                OnboardingPage::Basics,
                                "The Basics",
                                "1",
                                cx,
                            ))
                            .child(self.render_nav_item(
                                OnboardingPage::Editing,
                                "Editing Experience",
                                "2",
                                cx,
                            ))
                            .child(self.render_nav_item(
                                OnboardingPage::AiSetup,
                                "AI Setup",
                                "3",
                                cx,
                            ))
                            .child(self.render_nav_item(
                                OnboardingPage::Welcome,
                                "Welcome",
                                "4",
                                cx,
                            )),
                    ),
            )
            .child(self.render_bottom_controls(window, cx))
    }

    fn render_nav_item(
        &mut self,
        page: OnboardingPage,
        label: impl Into<SharedString>,
        shortcut: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let selected = self.current_page == page;
        let label = label.into();
        let shortcut = shortcut.into();
        let id = ElementId::Name(label.clone());

        h_flex()
            .id(id)
            .h(rems(1.5))
            .w_full()
            .child(
                div()
                    .w(px(3.))
                    .h_full()
                    .when(selected, |this| this.bg(cx.theme().status().info)),
            )
            .child(
                h_flex()
                    .pl(px(23.))
                    .flex_1()
                    .justify_between()
                    .items_center()
                    .child(Label::new(label))
                    .child(Label::new(format!("⌘{}", shortcut.clone())).color(Color::Muted)),
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.jump_to_page(page, window, cx);
            }))
    }

    fn render_bottom_controls(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        h_flex().w_full().p_4().child(
            Button::new(
                "next",
                if self.current_page == OnboardingPage::Welcome {
                    "Get Started"
                } else {
                    "Next"
                },
            )
            .style(ButtonStyle::Filled)
            .key_binding(ui::KeyBinding::for_action_in(
                &NextPage,
                &self.focus_handle,
                window,
                cx,
            ))
            .on_click(cx.listener(|this, _, window, cx| {
                this.next_page(window, cx);
            })),
        )
    }

    fn render_active_page(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut Context<Self>,
    ) -> AnyElement {
        match self.current_page {
            OnboardingPage::Basics => self.render_basics_page(),
            OnboardingPage::Editing => self.render_editing_page(),
            OnboardingPage::AiSetup => self.render_ai_setup_page(),
            OnboardingPage::Welcome => self.render_welcome_page(),
        }
    }

    fn render_basics_page(&self) -> AnyElement {
        v_flex()
            .h_full()
            .w_full()
            .child("Basics Page")
            .into_any_element()
    }

    fn render_editing_page(&self) -> AnyElement {
        v_flex()
            .h_full()
            .w_full()
            .child("Editing Page")
            .into_any_element()
    }

    fn render_ai_setup_page(&self) -> AnyElement {
        v_flex()
            .h_full()
            .w_full()
            .child("AI Setup Page")
            .into_any_element()
    }

    fn render_welcome_page(&self) -> AnyElement {
        v_flex()
            .h_full()
            .w_full()
            .child("Welcome Page")
            .into_any_element()
    }
}

impl Item for OnboardingUI {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Onboarding".into()
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        f(event.clone())
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.workspace_id = workspace.database_id();
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
        let weak_workspace = self.workspace.clone();
        let client = self.client.clone();
        if let Some(workspace) = weak_workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                Some(cx.new(|cx| OnboardingUI::new(workspace, client, cx)))
            })
        } else {
            None
        }
    }
}

impl SerializableItem for OnboardingUI {
    fn serialized_item_kind() -> &'static str {
        "OnboardingUI"
    }

    fn deserialize(
        _project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: u64,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<anyhow::Result<Entity<Self>>> {
        window.spawn(cx, async move |cx| {
            let (current_page, completed_pages) = if let Some((page_str, completed_str)) =
                ONBOARDING_DB.get_state(item_id, workspace_id)?
            {
                let page = match page_str.as_str() {
                    "basics" => OnboardingPage::Basics,
                    "editing" => OnboardingPage::Editing,
                    "ai_setup" => OnboardingPage::AiSetup,
                    "welcome" => OnboardingPage::Welcome,
                    _ => OnboardingPage::Basics,
                };
                let completed = OnboardingUI::completed_pages_from_string(&completed_str);
                (page, completed)
            } else {
                (OnboardingPage::Basics, [false; 4])
            };

            cx.update(|window, cx| {
                let workspace = workspace
                    .upgrade()
                    .ok_or_else(|| anyhow::anyhow!("workspace dropped"))?;

                workspace.update(cx, |workspace, cx| {
                    let client = workspace.client().clone();
                    Ok(cx.new(|cx| {
                        let mut onboarding = OnboardingUI::new(workspace, client, cx);
                        onboarding.current_page = current_page;
                        onboarding.completed_pages = completed_pages;
                        onboarding
                    }))
                })
            })?
        })
    }

    fn serialize(
        &mut self,
        _workspace: &mut Workspace,
        item_id: u64,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<anyhow::Result<()>>> {
        let workspace_id = self.workspace_id?;
        let current_page = match self.current_page {
            OnboardingPage::Basics => "basics",
            OnboardingPage::Editing => "editing",
            OnboardingPage::AiSetup => "ai_setup",
            OnboardingPage::Welcome => "welcome",
        }
        .to_string();
        let completed_pages = self.completed_pages_to_string();

        Some(cx.background_spawn(async move {
            ONBOARDING_DB
                .save_state(item_id, workspace_id, current_page, completed_pages)
                .await
        }))
    }

    fn cleanup(
        _workspace_id: WorkspaceId,
        _item_ids: Vec<u64>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<anyhow::Result<()>> {
        Task::ready(Ok(()))
    }

    fn should_serialize(&self, _event: &ItemEvent) -> bool {
        true
    }
}
