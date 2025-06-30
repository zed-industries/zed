#![allow(unused, dead_code)]
use command_palette_hooks::CommandPaletteFilter;
use feature_flags::FeatureFlagAppExt as _;
use gpui::{Entity, EventEmitter, FocusHandle, Focusable, WeakEntity, actions, prelude::*};
use settings_ui::SettingsUiFeatureFlag;
use ui::prelude::*;
use workspace::{
    Workspace, WorkspaceId,
    item::{Item, ItemEvent},
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
            let onboarding = cx.new(|cx| OnboardingUI::new(workspace, cx));
            workspace.add_item_to_active_pane(Box::new(onboarding), None, true, window, cx);
        });

        workspace.register_action(|_workspace, _: &JumpToBasics, _window, _cx| {
            // Jump to basics implementation
        });

        workspace.register_action(|_workspace, _: &JumpToEditing, _window, _cx| {
            // Jump to editing implementation
        });

        workspace.register_action(|_workspace, _: &JumpToAiSetup, _window, _cx| {
            // Jump to AI setup implementation
        });

        workspace.register_action(|_workspace, _: &JumpToWelcome, _window, _cx| {
            // Jump to welcome implementation
        });

        workspace.register_action(|_workspace, _: &NextPage, _window, _cx| {
            // Next page implementation
        });

        workspace.register_action(|_workspace, _: &PreviousPage, _window, _cx| {
            // Previous page implementation
        });

        workspace.register_action(|_workspace, _: &ToggleFocus, _window, _cx| {
            // Toggle focus implementation
        });

        workspace.register_action(|_workspace, _: &ResetOnboarding, _window, _cx| {
            // Reset onboarding implementation
        });
    })
    .detach();

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

    // Page entities
    basics_page: Entity<BasicsPage>,
    editing_page: Entity<EditingPage>,
    ai_setup_page: Entity<AiSetupPage>,
    welcome_page: Entity<WelcomePage>,

    // Workspace reference for Item trait
    workspace: WeakEntity<Workspace>,
}

impl EventEmitter<ItemEvent> for OnboardingUI {}

impl Focusable for OnboardingUI {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

pub struct BasicsPage {
    focus_handle: FocusHandle,
    parent: WeakEntity<OnboardingUI>,
}

pub struct EditingPage {
    focus_handle: FocusHandle,
    parent: WeakEntity<OnboardingUI>,
}

pub struct AiSetupPage {
    focus_handle: FocusHandle,
    parent: WeakEntity<OnboardingUI>,
}

pub struct WelcomePage {
    focus_handle: FocusHandle,
    parent: WeakEntity<OnboardingUI>,
}

// Event types for communication between pages and main UI
#[derive(Clone)]
pub enum OnboardingEvent {
    PageCompleted(OnboardingPage),
}

// Implement EventEmitter for all entities
impl EventEmitter<OnboardingEvent> for BasicsPage {}
impl EventEmitter<OnboardingEvent> for EditingPage {}
impl EventEmitter<OnboardingEvent> for AiSetupPage {}
impl EventEmitter<OnboardingEvent> for WelcomePage {}

impl Focusable for BasicsPage {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Focusable for EditingPage {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Focusable for AiSetupPage {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Focusable for WelcomePage {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// Placeholder Render implementations
impl Render for OnboardingUI {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        h_flex()
            .id("onboarding-ui")
            .key_context("Onboarding")
            .track_focus(&self.focus_handle)
            .w(px(904.))
            .h(px(500.))
            .gap(px(48.))
            .child(v_flex().h_full().w(px(256.)).child("nav"))
    }
}

impl Render for BasicsPage {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        gpui::div()
    }
}

impl Render for EditingPage {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        gpui::div()
    }
}

impl Render for AiSetupPage {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        gpui::div()
    }
}

impl Render for WelcomePage {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        gpui::div()
    }
}

impl OnboardingUI {
    pub fn new(workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        let parent_handle = cx.entity().downgrade();

        let basics_page = cx.new(|cx| BasicsPage {
            focus_handle: cx.focus_handle(),
            parent: parent_handle.clone(),
        });

        let editing_page = cx.new(|cx| EditingPage {
            focus_handle: cx.focus_handle(),
            parent: parent_handle.clone(),
        });

        let ai_setup_page = cx.new(|cx| AiSetupPage {
            focus_handle: cx.focus_handle(),
            parent: parent_handle.clone(),
        });

        let welcome_page = cx.new(|cx| WelcomePage {
            focus_handle: cx.focus_handle(),
            parent: parent_handle.clone(),
        });

        Self {
            focus_handle: cx.focus_handle(),
            current_page: OnboardingPage::Basics,
            current_focus: OnboardingFocus::Page,
            completed_pages: [false; 4],
            basics_page,
            editing_page,
            ai_setup_page,
            welcome_page,
            workspace: workspace.weak_handle(),
        }
    }

    fn jump_to_page(
        &mut self,
        page: OnboardingPage,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        self.current_page = page;
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
}

impl Item for OnboardingUI {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Onboarding".into()
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        f(event.clone())
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
        if let Some(workspace) = weak_workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                Some(cx.new(|cx| OnboardingUI::new(workspace, cx)))
            })
        } else {
            None
        }
    }
}
