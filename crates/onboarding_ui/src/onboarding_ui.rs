#![allow(unused, dead_code)]
mod juicy_button;
mod persistence;
mod theme_preview;

use self::juicy_button::JuicyButton;
use client::{Client, TelemetrySettings};
use command_palette_hooks::CommandPaletteFilter;
use feature_flags::FeatureFlagAppExt as _;
use gpui::{
    Entity, EventEmitter, FocusHandle, Focusable, KeyBinding, Task, UpdateGlobal, WeakEntity,
    actions, prelude::*, svg,
};
use menu;
use persistence::ONBOARDING_DB;

use project::Project;
use serde_json;
use settings::{Settings, SettingsStore};
use settings_ui::SettingsUiFeatureFlag;
use std::sync::Arc;
use theme::{Theme, ThemeRegistry, ThemeSettings};
use ui::{ListItem, ToggleState, Vector, VectorName, prelude::*};
use util::ResultExt;
use vim_mode_setting::VimModeSetting;
use welcome::BaseKeymap;
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
    const ONBOARDING_ACTION_NAMESPACE: &str = "onboarding_ui";

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
pub enum NavigationFocusItem {
    SignIn,
    Basics,
    Editing,
    AiSetup,
    Welcome,
    Next,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFocusItem(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusArea {
    Navigation,
    PageContent,
}

pub struct OnboardingUI {
    focus_handle: FocusHandle,
    current_page: OnboardingPage,
    nav_focus: NavigationFocusItem,
    page_focus: [PageFocusItem; 4],
    completed_pages: [bool; 4],
    focus_area: FocusArea,

    // Workspace reference for Item trait
    workspace: WeakEntity<Workspace>,
    workspace_id: Option<WorkspaceId>,
    client: Arc<Client>,
}

impl OnboardingUI {}

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
            .key_context("OnboardingUI")
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::toggle_focus))
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
                    .gap(px(24.))
                    .child(
                        h_flex()
                            .h(px(500.))
                            .w_full()
                            .gap(px(48.))
                            .child(self.render_navigation(window, cx))
                            .child(
                                v_flex()
                                    .h_full()
                                    .flex_1()
                                    .when(self.focus_area == FocusArea::PageContent, |this| {
                                        this.border_2()
                                            .border_color(cx.theme().colors().border_focused)
                                    })
                                    .rounded_lg()
                                    .p_4()
                                    .child(
                                        div().flex_1().child(self.render_active_page(window, cx)),
                                    ),
                            ),
                    ),
            )
    }
}

impl OnboardingUI {
    pub fn new(workspace: &Workspace, client: Arc<Client>, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            current_page: OnboardingPage::Basics,
            nav_focus: NavigationFocusItem::Basics,
            page_focus: [PageFocusItem(0); 4],
            completed_pages: [false; 4],
            focus_area: FocusArea::Navigation,
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

    fn reset(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) {
        self.current_page = OnboardingPage::Basics;
        self.focus_area = FocusArea::Navigation;
        self.completed_pages = [false; 4];
        cx.notify();
    }

    fn select_next(&mut self, _: &menu::SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        match self.focus_area {
            FocusArea::Navigation => {
                self.nav_focus = match self.nav_focus {
                    NavigationFocusItem::SignIn => NavigationFocusItem::Basics,
                    NavigationFocusItem::Basics => NavigationFocusItem::Editing,
                    NavigationFocusItem::Editing => NavigationFocusItem::AiSetup,
                    NavigationFocusItem::AiSetup => NavigationFocusItem::Welcome,
                    NavigationFocusItem::Welcome => NavigationFocusItem::Next,
                    NavigationFocusItem::Next => NavigationFocusItem::SignIn,
                };
            }
            FocusArea::PageContent => {
                let page_index = match self.current_page {
                    OnboardingPage::Basics => 0,
                    OnboardingPage::Editing => 1,
                    OnboardingPage::AiSetup => 2,
                    OnboardingPage::Welcome => 3,
                };
                // Bounds checking for page items
                let max_items = match self.current_page {
                    OnboardingPage::Basics => 14, // 4 themes + 7 keymaps + 3 checkboxes
                    OnboardingPage::Editing => 3, // 3 buttons
                    OnboardingPage::AiSetup => 2, // Will have 2 items
                    OnboardingPage::Welcome => 1, // Will have 1 item
                };

                if self.page_focus[page_index].0 < max_items - 1 {
                    self.page_focus[page_index].0 += 1;
                } else {
                    // Wrap to start
                    self.page_focus[page_index].0 = 0;
                }
            }
        }
        cx.notify();
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.focus_area {
            FocusArea::Navigation => {
                self.nav_focus = match self.nav_focus {
                    NavigationFocusItem::SignIn => NavigationFocusItem::Next,
                    NavigationFocusItem::Basics => NavigationFocusItem::SignIn,
                    NavigationFocusItem::Editing => NavigationFocusItem::Basics,
                    NavigationFocusItem::AiSetup => NavigationFocusItem::Editing,
                    NavigationFocusItem::Welcome => NavigationFocusItem::AiSetup,
                    NavigationFocusItem::Next => NavigationFocusItem::Welcome,
                };
            }
            FocusArea::PageContent => {
                let page_index = match self.current_page {
                    OnboardingPage::Basics => 0,
                    OnboardingPage::Editing => 1,
                    OnboardingPage::AiSetup => 2,
                    OnboardingPage::Welcome => 3,
                };
                // Bounds checking for page items
                let max_items = match self.current_page {
                    OnboardingPage::Basics => 14, // 4 themes + 7 keymaps + 3 checkboxes
                    OnboardingPage::Editing => 3, // 3 buttons
                    OnboardingPage::AiSetup => 2, // Will have 2 items
                    OnboardingPage::Welcome => 1, // Will have 1 item
                };

                if self.page_focus[page_index].0 > 0 {
                    self.page_focus[page_index].0 -= 1;
                } else {
                    // Wrap to end
                    self.page_focus[page_index].0 = max_items - 1;
                }
            }
        }
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        match self.focus_area {
            FocusArea::Navigation => {
                match self.nav_focus {
                    NavigationFocusItem::SignIn => {
                        // Handle sign in action
                        // TODO: Implement sign in action
                    }
                    NavigationFocusItem::Basics => {
                        self.jump_to_page(OnboardingPage::Basics, window, cx)
                    }
                    NavigationFocusItem::Editing => {
                        self.jump_to_page(OnboardingPage::Editing, window, cx)
                    }
                    NavigationFocusItem::AiSetup => {
                        self.jump_to_page(OnboardingPage::AiSetup, window, cx)
                    }
                    NavigationFocusItem::Welcome => {
                        self.jump_to_page(OnboardingPage::Welcome, window, cx)
                    }
                    NavigationFocusItem::Next => {
                        // Handle next button action
                        self.next_page(window, cx);
                    }
                }
                // After confirming navigation item (except Next), switch focus to page content
                if self.nav_focus != NavigationFocusItem::Next {
                    self.focus_area = FocusArea::PageContent;
                }
            }
            FocusArea::PageContent => {
                // Handle page-specific item selection
                let page_index = match self.current_page {
                    OnboardingPage::Basics => 0,
                    OnboardingPage::Editing => 1,
                    OnboardingPage::AiSetup => 2,
                    OnboardingPage::Welcome => 3,
                };
                let item_index = self.page_focus[page_index].0;

                // Trigger the action for the focused item
                match self.current_page {
                    OnboardingPage::Basics => {
                        match item_index {
                            0..=3 => {
                                // Theme selection
                                cx.notify();
                            }
                            4..=10 => {
                                // Keymap selection
                                cx.notify();
                            }
                            11..=13 => {
                                // Checkbox toggles (handled by their own listeners)
                                cx.notify();
                            }
                            _ => {}
                        }
                    }
                    OnboardingPage::Editing => {
                        // Similar handling for editing page
                        cx.notify();
                    }
                    _ => {
                        cx.notify();
                    }
                }
            }
        }
        cx.notify();
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        match self.focus_area {
            FocusArea::PageContent => {
                // Switch focus back to navigation
                self.focus_area = FocusArea::Navigation;
            }
            FocusArea::Navigation => {
                // If already in navigation, maybe close the onboarding?
                // For now, just stay in navigation
            }
        }
        cx.notify();
    }

    fn toggle_focus(&mut self, _: &ToggleFocus, _window: &mut Window, cx: &mut Context<Self>) {
        self.focus_area = match self.focus_area {
            FocusArea::Navigation => FocusArea::PageContent,
            FocusArea::PageContent => FocusArea::Navigation,
        };
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
                            .child(
                                Vector::new(VectorName::ZedLogo, rems(2.), rems(2.))
                                    .color(Color::Custom(cx.theme().colors().icon.opacity(0.5))),
                            )
                            .child(
                                Button::new("sign_in", "Sign in")
                                    .color(Color::Muted)
                                    .label_size(LabelSize::Small)
                                    .when(
                                        self.focus_area == FocusArea::Navigation
                                            && self.nav_focus == NavigationFocusItem::SignIn,
                                        |this| this.color(Color::Accent),
                                    )
                                    .size(ButtonSize::Compact)
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
                            .gap(px(12.))
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

        let is_focused = match page {
            OnboardingPage::Basics => self.nav_focus == NavigationFocusItem::Basics,
            OnboardingPage::Editing => self.nav_focus == NavigationFocusItem::Editing,
            OnboardingPage::AiSetup => self.nav_focus == NavigationFocusItem::AiSetup,
            OnboardingPage::Welcome => self.nav_focus == NavigationFocusItem::Welcome,
        };

        let area_focused = self.focus_area == FocusArea::Navigation;

        h_flex()
            .id(id)
            .h(rems(1.5))
            .w_full()
            .when(is_focused, |this| {
                this.bg(if area_focused {
                    cx.theme().colors().border_focused.opacity(0.16)
                } else {
                    cx.theme().colors().border.opacity(0.24)
                })
            })
            .child(
                div()
                    .w(px(3.))
                    .h_full()
                    .when(selected, |this| this.bg(cx.theme().colors().border_focused)),
            )
            .child(
                h_flex()
                    .pl(px(23.))
                    .flex_1()
                    .justify_between()
                    .items_center()
                    .child(Label::new(label).when(is_focused, |this| this.color(Color::Default)))
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
        h_flex().w_full().p(px(12.)).pl(px(24.)).child(
            JuicyButton::new(if self.current_page == OnboardingPage::Welcome {
                "Get Started"
            } else {
                "Next"
            })
            .keybinding(ui::KeyBinding::for_action_in(
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

    fn render_active_page(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self.current_page {
            OnboardingPage::Basics => self.render_basics_page(cx),
            OnboardingPage::Editing => self.render_editing_page(cx),
            OnboardingPage::AiSetup => self.render_ai_setup_page(cx),
            OnboardingPage::Welcome => self.render_welcome_page(cx),
        }
    }

    fn render_basics_page(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let page_index = 0; // Basics page index
        let focused_item = self.page_focus[page_index].0;
        let is_page_focused = self.focus_area == FocusArea::PageContent;

        use theme_preview::ThemePreviewTile;

        // Get available themes
        let theme_registry = ThemeRegistry::default_global(cx);
        let theme_names = theme_registry.list_names();
        let current_theme = cx.theme().clone();

        // For demo, we'll show 4 themes

        v_flex()
            .id("theme-selector")
            .h_full()
            .w_full()
            .p_6()
            .gap_6()
            .overflow_y_scroll()
            // Theme selector section
            .child(
                v_flex()
                    .gap_3()
                    .child(
                        h_flex()
                            .justify_between()
                            .child(Label::new("Pick a Theme").size(LabelSize::Large))
                            .child(
                                Button::new("more_themes", "More Themes")
                                    .style(ButtonStyle::Subtle)
                                    .color(Color::Muted)
                                    .on_click(cx.listener(|_, _, window, cx| {
                                        // TODO: Open theme selector
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_3()
                            .children(
                                vec![
                                    ("One Dark", "one-dark"),
                                    ("Gruvbox Dark", "gruvbox-dark"),
                                    ("One Light", "one-light"),
                                    ("Gruvbox Light", "gruvbox-light"),
                                ]
                                .into_iter()
                                .enumerate()
                                .map(|(i, (label, theme_name))| {
                                    let is_selected = current_theme.name == *theme_name;
                                    let is_focused = is_page_focused && focused_item == i;

                                    v_flex()
                                        .gap_2()
                                        .child(
                                            div()
                                                .id("theme-item")
                                                .when(is_focused, |this| {
                                                    this.border_2().border_color(
                                                        cx.theme().colors().border_focused,
                                                    )
                                                })
                                                .rounded_md()
                                                .p_1()
                                                .id(("theme", i))
                                                .child(
                                                    if let Ok(theme) =
                                                        theme_registry.get(theme_name)
                                                    {
                                                        ThemePreviewTile::new(
                                                            theme,
                                                            is_selected,
                                                            0.5,
                                                        )
                                                        .into_any_element()
                                                    } else {
                                                        div()
                                                            .w(px(200.))
                                                            .h(px(120.))
                                                            .bg(cx
                                                                .theme()
                                                                .colors()
                                                                .surface_background)
                                                            .rounded_md()
                                                            .into_any_element()
                                                    },
                                                )
                                                .on_click(cx.listener(
                                                    move |this, _, window, cx| {
                                                        SettingsStore::update_global(cx, move |store, cx| {
                                                            let mut settings = store.raw_user_settings().clone();
                                                            settings["theme"] = serde_json::json!(theme_name);
                                                            store.set_user_settings(&settings.to_string(), cx).ok();
                                                        });
                                                        cx.notify();
                                                    },
                                                )),
                                        )
                                        .child(Label::new(label).size(LabelSize::Small).color(
                                            if is_selected {
                                                Color::Default
                                            } else {
                                                Color::Muted
                                            },
                                        ))
                                },
                            )),
                    ),
            )
            // Keymap selector section
            .child(
                v_flex()
                    .gap_3()
                    .mt_4()
                    .child(Label::new("Pick a Keymap").size(LabelSize::Large))
                    .child(
                        h_flex().gap_2().children(
                            vec![
                                ("Zed", VectorName::ZedLogo, 4),
                                ("Atom", VectorName::ZedLogo, 5),
                                ("JetBrains", VectorName::ZedLogo, 6),
                                ("Sublime", VectorName::ZedLogo, 7),
                                ("VSCode", VectorName::ZedLogo, 8),
                                ("Emacs", VectorName::ZedLogo, 9),
                                ("TextMate", VectorName::ZedLogo, 10),
                            ]
                            .into_iter()
                            .map(|(label, icon, index)| {
                                let is_focused = is_page_focused && focused_item == index;
                                let current_keymap = BaseKeymap::get_global(cx).to_string();
                                let is_selected = current_keymap == label;

                                v_flex()
                                    .gap_1()
                                    .items_center()
                                    .child(
                                        div()
                                            .id(("keymap", index))
                                            .p_3()
                                            .rounded_md()
                                            .bg(cx.theme().colors().element_background)
                                            .border_1()
                                            .border_color(if is_selected {
                                                cx.theme().colors().border_selected
                                            } else {
                                                cx.theme().colors().border
                                            })
                                            .when(is_focused, |this| {
                                                this.border_color(
                                                    cx.theme().colors().border_focused,
                                                )
                                            })
                                            .when(is_selected, |this| {
                                                this.bg(cx.theme().colors().element_selected)
                                            })
                                            .hover(|this| {
                                                this.bg(cx.theme().colors().element_hover)
                                            })
                                            .child(
                                                Vector::new(icon, rems(2.), rems(2.))
                                                    .color(Color::Muted),
                                            )
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                SettingsStore::update_global(cx, move |store, cx| {
                                                    let base_keymap = match label {
                                                        "Zed" => "None",
                                                        "Atom" => "Atom",
                                                        "JetBrains" => "JetBrains",
                                                        "Sublime" => "SublimeText",
                                                        "VSCode" => "VSCode",
                                                        "Emacs" => "Emacs",
                                                        "TextMate" => "TextMate",
                                                        _ => "VSCode",
                                                    };
                                                    let mut settings = store.raw_user_settings().clone();
                                                    settings["base_keymap"] = serde_json::json!(base_keymap);
                                                    store.set_user_settings(&settings.to_string(), cx).ok();
                                                });
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        Label::new(label)
                                            .size(LabelSize::Small)
                                            .color(if is_selected {
                                                Color::Default
                                            } else {
                                                Color::Muted
                                            }),
                                    )
                            })
                        ),
                    ),
            )
            // Settings checkboxes
            .child(
                v_flex()
                    .gap_3()
                    .mt_6()
                    .child({
                        let vim_enabled = VimModeSetting::get_global(cx).0;
                        h_flex()
                            .id("vim_mode_container")
                            .gap_2()
                            .items_center()
                            .p_1()
                            .rounded_md()
                            .when(is_page_focused && focused_item == 11, |this| {
                                this.border_2()
                                    .border_color(cx.theme().colors().border_focused)
                            })
                            .child(
                                div()
                                    .id("vim_mode_checkbox")
                                    .w_4()
                                    .h_4()
                                    .rounded_sm()
                                    .border_1()
                                    .border_color(cx.theme().colors().border)
                                    .when(vim_enabled, |this| {
                                        this.bg(cx.theme().colors().element_selected)
                                            .border_color(cx.theme().colors().border_selected)
                                    })
                                    .hover(|this| this.bg(cx.theme().colors().element_hover))
                                    .child(
                                        div().when(vim_enabled, |this| {
                                            this.size_full()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .child(
                                                    svg()
                                                        .path("M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z")
                                                        .size_3()
                                                        .text_color(cx.theme().colors().icon),
                                                )
                                        })
                                    ),
                            )
                            .child(Label::new("Enable Vim Mode"))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                let current = VimModeSetting::get_global(cx).0;
                                SettingsStore::update_global(cx, move |store, cx| {
                                    let mut settings = store.raw_user_settings().clone();
                                    settings["vim_mode"] = serde_json::json!(!current);
                                    store.set_user_settings(&settings.to_string(), cx).ok();
                                });
                            }))
                    })
                    .child({
                        let crash_reports_enabled = TelemetrySettings::get_global(cx).diagnostics;
                        h_flex()
                            .id("crash_reports_container")
                            .gap_2()
                            .items_center()
                            .p_1()
                            .rounded_md()
                            .when(is_page_focused && focused_item == 12, |this| {
                                this.border_2()
                                    .border_color(cx.theme().colors().border_focused)
                            })
                            .child(
                                div()
                                    .id("crash_reports_checkbox")
                                    .w_4()
                                    .h_4()
                                    .rounded_sm()
                                    .border_1()
                                    .border_color(cx.theme().colors().border)
                                    .when(crash_reports_enabled, |this| {
                                        this.bg(cx.theme().colors().element_selected)
                                            .border_color(cx.theme().colors().border_selected)
                                    })
                                    .hover(|this| this.bg(cx.theme().colors().element_hover))
                                    .child(
                                        div().when(crash_reports_enabled, |this| {
                                            this.size_full()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .child(
                                                    svg()
                                                        .path("M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z")
                                                        .size_3()
                                                        .text_color(cx.theme().colors().icon),
                                                )
                                        })
                                    ),
                            )
                            .child(Label::new("Send Crash Reports"))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                let current = TelemetrySettings::get_global(cx).diagnostics;
                                SettingsStore::update_global(cx, move |store, cx| {
                                    let mut settings = store.raw_user_settings().clone();
                                    if settings.get("telemetry").is_none() {
                                        settings["telemetry"] = serde_json::json!({});
                                    }
                                    settings["telemetry"]["diagnostics"] = serde_json::json!(!current);
                                    store.set_user_settings(&settings.to_string(), cx).ok();
                                });
                            }))
                    })
                    .child({
                        let telemetry_enabled = TelemetrySettings::get_global(cx).metrics;
                        h_flex()
                            .id("telemetry_container")
                            .gap_2()
                            .items_center()
                            .p_1()
                            .rounded_md()
                            .when(is_page_focused && focused_item == 13, |this| {
                                this.border_2()
                                    .border_color(cx.theme().colors().border_focused)
                            })
                            .child(
                                div()
                                    .id("telemetry_checkbox")
                                    .w_4()
                                    .h_4()
                                    .rounded_sm()
                                    .border_1()
                                    .border_color(cx.theme().colors().border)
                                    .when(telemetry_enabled, |this| {
                                        this.bg(cx.theme().colors().element_selected)
                                            .border_color(cx.theme().colors().border_selected)
                                    })
                                    .hover(|this| this.bg(cx.theme().colors().element_hover))
                                    .child(
                                        div().when(telemetry_enabled, |this| {
                                            this.size_full()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .child(
                                                    svg()
                                                        .path("M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z")
                                                        .size_3()
                                                        .text_color(cx.theme().colors().icon),
                                                )
                                        })
                                    ),
                            )
                            .child(Label::new("Send Telemetry"))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                let current = TelemetrySettings::get_global(cx).metrics;
                                SettingsStore::update_global(cx, move |store, cx| {
                                    let mut settings = store.raw_user_settings().clone();
                                    if settings.get("telemetry").is_none() {
                                        settings["telemetry"] = serde_json::json!({});
                                    }
                                    settings["telemetry"]["metrics"] = serde_json::json!(!current);
                                    store.set_user_settings(&settings.to_string(), cx).ok();
                                });
                            }))
                    }),
            )
            .into_any_element()
    }

    fn render_editing_page(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let page_index = 1; // Editing page index
        let focused_item = self.page_focus[page_index].0;
        let is_page_focused = self.focus_area == FocusArea::PageContent;

        v_flex()
            .h_full()
            .w_full()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                Label::new("Editing Features")
                    .size(LabelSize::Large)
                    .color(Color::Default),
            )
            .child(
                v_flex()
                    .gap_2()
                    .mt_4()
                    .child(
                        Button::new("try_multi_cursor", "Try Multi-cursor Editing")
                            .style(ButtonStyle::Filled)
                            .when(is_page_focused && focused_item == 0, |this| {
                                this.color(Color::Accent)
                            })
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("learn_shortcuts", "Learn Keyboard Shortcuts")
                            .style(ButtonStyle::Filled)
                            .when(is_page_focused && focused_item == 1, |this| {
                                this.color(Color::Accent)
                            })
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("explore_actions", "Explore Command Palette")
                            .style(ButtonStyle::Filled)
                            .when(is_page_focused && focused_item == 2, |this| {
                                this.color(Color::Accent)
                            })
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.notify();
                            })),
                    ),
            )
            .into_any_element()
    }

    fn render_ai_setup_page(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let page_index = 2; // AI Setup page index
        let focused_item = self.page_focus[page_index].0;
        let is_page_focused = self.focus_area == FocusArea::PageContent;

        v_flex()
            .h_full()
            .w_full()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                Label::new("AI Assistant Setup")
                    .size(LabelSize::Large)
                    .color(Color::Default),
            )
            .child(
                v_flex()
                    .gap_2()
                    .mt_4()
                    .child(
                        Button::new("configure_ai", "Configure AI Provider")
                            .style(ButtonStyle::Filled)
                            .when(is_page_focused && focused_item == 0, |this| {
                                this.color(Color::Accent)
                            })
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("try_ai_chat", "Try AI Chat")
                            .style(ButtonStyle::Filled)
                            .when(is_page_focused && focused_item == 1, |this| {
                                this.color(Color::Accent)
                            })
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.notify();
                            })),
                    ),
            )
            .into_any_element()
    }

    fn render_welcome_page(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let page_index = 3; // Welcome page index
        let focused_item = self.page_focus[page_index].0;
        let is_page_focused = self.focus_area == FocusArea::PageContent;

        v_flex()
            .h_full()
            .w_full()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                Label::new("Welcome to Zed!")
                    .size(LabelSize::Large)
                    .color(Color::Default),
            )
            .child(
                Label::new("You're all set up and ready to code")
                    .size(LabelSize::Default)
                    .color(Color::Muted),
            )
            .child(
                Button::new("finish_onboarding", "Start Coding!")
                    .style(ButtonStyle::Filled)
                    .size(ButtonSize::Large)
                    .when(is_page_focused && focused_item == 0, |this| {
                        this.color(Color::Accent)
                    })
                    .on_click(cx.listener(|_, _, _, cx| {
                        // TODO: Close onboarding and start coding
                        cx.notify();
                    })),
            )
            .into_any_element()
    }

    fn render_keyboard_help(&self, cx: &mut Context<Self>) -> AnyElement {
        let help_text = match self.focus_area {
            FocusArea::Navigation => {
                "Use ↑/↓ to navigate • Enter to select page • Tab to switch to page content"
            }
            FocusArea::PageContent => {
                "Use ↑/↓ to navigate • Enter to activate • Esc to return to navigation"
            }
        };

        h_flex()
            .w_full()
            .justify_center()
            .p_2()
            .child(
                Label::new(help_text)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
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
