use command_palette_hooks::CommandPaletteFilter;
use db::kvp::KEY_VALUE_STORE;
use feature_flags::{FeatureFlag, FeatureFlagViewExt as _};
use fs::Fs;
use gpui::{
    AnyElement, App, AppContext, Context, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, Render, SharedString, Subscription, Task, WeakEntity, Window, actions,
};
use settings::{Settings, SettingsStore, update_settings_file};
use std::sync::Arc;
use theme::{ThemeMode, ThemeSettings};
use ui::{
    ButtonCommon as _, ButtonSize, ButtonStyle, Clickable as _, Color, Divider, FluentBuilder,
    Headline, InteractiveElement, KeyBinding, Label, LabelCommon, ParentElement as _,
    StatefulInteractiveElement, Styled, ToggleButton, Toggleable as _, Vector, VectorName, div,
    h_flex, rems, v_container, v_flex,
};
use workspace::{
    AppState, Workspace, WorkspaceId,
    dock::DockPosition,
    item::{Item, ItemEvent},
    open_new, with_active_or_new_workspace,
};

pub struct OnBoardingFeatureFlag {}

impl FeatureFlag for OnBoardingFeatureFlag {
    const NAME: &'static str = "onboarding";
}

pub const FIRST_OPEN: &str = "first_open";

actions!(
    zed,
    [
        /// Opens the onboarding view.
        OpenOnboarding
    ]
);

pub fn init(cx: &mut App) {
    cx.on_action(|_: &OpenOnboarding, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            workspace
                .with_local_workspace(window, cx, |workspace, window, cx| {
                    let existing = workspace
                        .active_pane()
                        .read(cx)
                        .items()
                        .find_map(|item| item.downcast::<Onboarding>());

                    if let Some(existing) = existing {
                        workspace.activate_item(&existing, true, true, window, cx);
                    } else {
                        let settings_page = Onboarding::new(workspace.weak_handle(), cx);
                        workspace.add_item_to_active_pane(
                            Box::new(settings_page),
                            None,
                            true,
                            window,
                            cx,
                        )
                    }
                })
                .detach();
        });
    });
    cx.observe_new::<Workspace>(|_, window, cx| {
        let Some(window) = window else {
            return;
        };

        let onboarding_actions = [std::any::TypeId::of::<OpenOnboarding>()];

        CommandPaletteFilter::update_global(cx, |filter, _cx| {
            filter.hide_action_types(&onboarding_actions);
        });

        cx.observe_flag::<OnBoardingFeatureFlag, _>(window, move |is_enabled, _, _, cx| {
            if is_enabled {
                CommandPaletteFilter::update_global(cx, |filter, _cx| {
                    filter.show_action_types(onboarding_actions.iter());
                });
            } else {
                CommandPaletteFilter::update_global(cx, |filter, _cx| {
                    filter.hide_action_types(&onboarding_actions);
                });
            }
        })
        .detach();
    })
    .detach();
}

pub fn show_onboarding_view(app_state: Arc<AppState>, cx: &mut App) -> Task<anyhow::Result<()>> {
    open_new(
        Default::default(),
        app_state,
        cx,
        |workspace, window, cx| {
            {
                workspace.toggle_dock(DockPosition::Left, window, cx);
                let onboarding_page = Onboarding::new(workspace.weak_handle(), cx);
                workspace.add_item_to_center(Box::new(onboarding_page.clone()), window, cx);

                window.focus(&onboarding_page.focus_handle(cx));

                cx.notify();
            };
            db::write_and_log(cx, || {
                KEY_VALUE_STORE.write_kvp(FIRST_OPEN.to_string(), "false".to_string())
            });
        },
    )
}

fn read_theme_selection(cx: &App) -> ThemeMode {
    let settings = ThemeSettings::get_global(cx);
    settings
        .theme_selection
        .as_ref()
        .and_then(|selection| selection.mode())
        .unwrap_or_default()
}

fn write_theme_selection(theme_mode: ThemeMode, cx: &App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file::<ThemeSettings>(fs, cx, move |settings, _| {
        settings.set_mode(theme_mode);
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectedPage {
    Basics,
    Editing,
    AiSetup,
}

struct Onboarding {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    selected_page: SelectedPage,
    _settings_subscription: Subscription,
}

impl Onboarding {
    fn new(workspace: WeakEntity<Workspace>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            workspace,
            focus_handle: cx.focus_handle(),
            selected_page: SelectedPage::Basics,
            _settings_subscription: cx.observe_global::<SettingsStore>(move |_, cx| cx.notify()),
        })
    }

    fn render_page_nav(
        &mut self,
        page: SelectedPage,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let text = match page {
            SelectedPage::Basics => "Basics",
            SelectedPage::Editing => "Editing",
            SelectedPage::AiSetup => "AI Setup",
        };
        let binding = match page {
            SelectedPage::Basics => {
                KeyBinding::new(vec![gpui::Keystroke::parse("cmd-1").unwrap()], cx)
            }
            SelectedPage::Editing => {
                KeyBinding::new(vec![gpui::Keystroke::parse("cmd-2").unwrap()], cx)
            }
            SelectedPage::AiSetup => {
                KeyBinding::new(vec![gpui::Keystroke::parse("cmd-3").unwrap()], cx)
            }
        };
        let selected = self.selected_page == page;
        h_flex()
            .id(text)
            .rounded_sm()
            .child(text)
            .child(binding)
            .h_8()
            .gap_2()
            .px_2()
            .py_0p5()
            .w_full()
            .justify_between()
            .map(|this| {
                if selected {
                    this.bg(Color::Selected.color(cx))
                        .border_l_1()
                        .border_color(Color::Accent.color(cx))
                } else {
                    this.text_color(Color::Muted.color(cx))
                }
            })
            .hover(|style| {
                if selected {
                    style.bg(Color::Selected.color(cx).opacity(0.6))
                } else {
                    style.bg(Color::Selected.color(cx).opacity(0.3))
                }
            })
            .on_click(cx.listener(move |this, _, _, cx| {
                this.selected_page = page;
                cx.notify();
            }))
    }

    fn render_page(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self.selected_page {
            SelectedPage::Basics => self.render_basics_page(window, cx).into_any_element(),
            SelectedPage::Editing => self.render_editing_page(window, cx).into_any_element(),
            SelectedPage::AiSetup => self.render_ai_setup_page(window, cx).into_any_element(),
        }
    }

    fn render_basics_page(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_mode = read_theme_selection(cx);

        v_container().child(
            h_flex()
                .items_center()
                .justify_between()
                .child(Label::new("Theme"))
                .child(
                    h_flex()
                        .rounded_md()
                        .child(
                            ToggleButton::new("light", "Light")
                                .style(ButtonStyle::Filled)
                                .size(ButtonSize::Large)
                                .toggle_state(theme_mode == ThemeMode::Light)
                                .on_click(|_, _, cx| write_theme_selection(ThemeMode::Light, cx))
                                .first(),
                        )
                        .child(
                            ToggleButton::new("dark", "Dark")
                                .style(ButtonStyle::Filled)
                                .size(ButtonSize::Large)
                                .toggle_state(theme_mode == ThemeMode::Dark)
                                .on_click(|_, _, cx| write_theme_selection(ThemeMode::Dark, cx))
                                .last(),
                        )
                        .child(
                            ToggleButton::new("system", "System")
                                .style(ButtonStyle::Filled)
                                .size(ButtonSize::Large)
                                .toggle_state(theme_mode == ThemeMode::System)
                                .on_click(|_, _, cx| write_theme_selection(ThemeMode::System, cx))
                                .middle(),
                        ),
                ),
        )
    }

    fn render_editing_page(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        // div().child("editing page")
        "Right"
    }

    fn render_ai_setup_page(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child("ai setup page")
    }
}

impl Render for Onboarding {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .image_cache(gpui::retain_all("onboarding-page"))
            .key_context("onboarding-page")
            .px_24()
            .py_12()
            .items_start()
            .child(
                v_flex()
                    .w_1_3()
                    .h_full()
                    .child(
                        h_flex()
                            .pt_0p5()
                            .child(Vector::square(VectorName::ZedLogo, rems(2.)))
                            .child(
                                v_flex()
                                    .left_1()
                                    .items_center()
                                    .child(Headline::new("Welcome to Zed"))
                                    .child(
                                        Label::new("The editor for what's next")
                                            .color(Color::Muted)
                                            .italic(),
                                    ),
                            ),
                    )
                    .p_1()
                    .child(Divider::horizontal_dashed())
                    .child(
                        v_flex().gap_1().children([
                            self.render_page_nav(SelectedPage::Basics, window, cx)
                                .into_element(),
                            self.render_page_nav(SelectedPage::Editing, window, cx)
                                .into_element(),
                            self.render_page_nav(SelectedPage::AiSetup, window, cx)
                                .into_element(),
                        ]),
                    ),
            )
            // .child(Divider::vertical_dashed())
            .child(div().w_2_3().h_full().child(self.render_page(window, cx)))
    }
}

impl EventEmitter<ItemEvent> for Onboarding {}

impl Focusable for Onboarding {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for Onboarding {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Onboarding".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Onboarding Page Opened")
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
        Some(Onboarding::new(self.workspace.clone(), cx))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
