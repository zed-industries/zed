use std::sync::Arc;

use ai_onboarding::{AiUpsellCard, SignInStatus};
use client::UserStore;
use fs::Fs;
use gpui::{
    Action, AnyView, App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, WeakEntity,
    Window, prelude::*,
};
use itertools;
use language_model::{LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry};
use project::DisableAiSettings;
use settings::{Settings, update_settings_file};
use ui::{
    Badge, ButtonLike, Divider, Modal, ModalFooter, ModalHeader, Section, SwitchField, ToggleState,
    prelude::*, tooltip_container,
};
use util::ResultExt;
use workspace::{ModalView, Workspace};
use zed_actions::agent::OpenSettings;

const FEATURED_PROVIDERS: [&'static str; 4] = ["anthropic", "google", "openai", "ollama"];

fn render_llm_provider_section(
    tab_index: &mut isize,
    workspace: WeakEntity<Workspace>,
    disabled: bool,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    v_flex()
        .gap_4()
        .child(
            v_flex()
                .child(Label::new("Or use other LLM providers").size(LabelSize::Large))
                .child(
                    Label::new("Bring your API keys to use the available providers with Zed's UI for free.")
                        .color(Color::Muted),
                ),
        )
        .child(render_llm_provider_card(tab_index, workspace, disabled, window, cx))
}

fn render_privacy_card(tab_index: &mut isize, disabled: bool, cx: &mut App) -> impl IntoElement {
    let privacy_badge = || {
        Badge::new("Privacy")
            .icon(IconName::ShieldCheck)
            .tooltip(move |_, cx| cx.new(|_| AiPrivacyTooltip::new()).into())
    };

    v_flex()
        .relative()
        .pt_2()
        .pb_2p5()
        .pl_3()
        .pr_2()
        .border_1()
        .border_dashed()
        .border_color(cx.theme().colors().border.opacity(0.5))
        .bg(cx.theme().colors().surface_background.opacity(0.3))
        .rounded_lg()
        .overflow_hidden()
        .map(|this| {
            if disabled {
                this.child(
                    h_flex()
                        .gap_2()
                        .justify_between()
                        .child(
                            h_flex()
                                .gap_1()
                                .child(Label::new("AI is disabled across Zed"))
                                .child(
                                    Icon::new(IconName::Check)
                                        .color(Color::Success)
                                        .size(IconSize::XSmall),
                                ),
                        )
                        .child(privacy_badge()),
                )
                .child(
                    Label::new("Re-enable it any time in Settings.")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
            } else {
                this.child(
                    h_flex()
                        .gap_2()
                        .justify_between()
                        .child(Label::new("We don't train models using your data"))
                        .child(
                            h_flex().gap_1().child(privacy_badge()).child(
                                Button::new("learn_more", "Learn More")
                                    .style(ButtonStyle::Outlined)
                                    .label_size(LabelSize::Small)
                                    .icon(IconName::ArrowUpRight)
                                    .icon_size(IconSize::XSmall)
                                    .icon_color(Color::Muted)
                                    .on_click(|_, _, cx| {
                                        cx.open_url("https://zed.dev/docs/ai/privacy-and-security");
                                    })
                                    .tab_index({
                                        *tab_index += 1;
                                        *tab_index - 1
                                    }),
                            ),
                        ),
                )
                .child(
                    Label::new(
                        "Feel confident in the security and privacy of your projects using Zed.",
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
            }
        })
}

fn render_llm_provider_card(
    tab_index: &mut isize,
    workspace: WeakEntity<Workspace>,
    disabled: bool,
    _: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let registry = LanguageModelRegistry::read_global(cx);

    v_flex()
        .border_1()
        .border_color(cx.theme().colors().border)
        .bg(cx.theme().colors().surface_background.opacity(0.5))
        .rounded_lg()
        .overflow_hidden()
        .children(itertools::intersperse_with(
            FEATURED_PROVIDERS
                .into_iter()
                .flat_map(|provider_name| {
                    registry.provider(&LanguageModelProviderId::new(provider_name))
                })
                .enumerate()
                .map(|(index, provider)| {
                    let group_name = SharedString::new(format!("onboarding-hover-group-{}", index));
                    let is_authenticated = provider.is_authenticated(cx);

                    ButtonLike::new(("onboarding-ai-setup-buttons", index))
                        .size(ButtonSize::Large)
                        .tab_index({
                            *tab_index += 1;
                            *tab_index - 1
                        })
                        .child(
                            h_flex()
                                .group(&group_name)
                                .px_0p5()
                                .w_full()
                                .gap_2()
                                .justify_between()
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            Icon::new(provider.icon())
                                                .color(Color::Muted)
                                                .size(IconSize::XSmall),
                                        )
                                        .child(Label::new(provider.name().0)),
                                )
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .when(!is_authenticated, |el| {
                                            el.visible_on_hover(group_name.clone())
                                                .child(
                                                    Icon::new(IconName::Settings)
                                                        .color(Color::Muted)
                                                        .size(IconSize::XSmall),
                                                )
                                                .child(
                                                    Label::new("Configure")
                                                        .color(Color::Muted)
                                                        .size(LabelSize::Small),
                                                )
                                        })
                                        .when(is_authenticated && !disabled, |el| {
                                            el.child(
                                                Icon::new(IconName::Check)
                                                    .color(Color::Success)
                                                    .size(IconSize::XSmall),
                                            )
                                            .child(
                                                Label::new("Configured")
                                                    .color(Color::Muted)
                                                    .size(LabelSize::Small),
                                            )
                                        }),
                                ),
                        )
                        .on_click({
                            let workspace = workspace.clone();
                            move |_, window, cx| {
                                workspace
                                    .update(cx, |workspace, cx| {
                                        workspace.toggle_modal(window, cx, |window, cx| {
                                            let modal = AiConfigurationModal::new(
                                                provider.clone(),
                                                window,
                                                cx,
                                            );
                                            window.focus(&modal.focus_handle(cx));
                                            modal
                                        });
                                    })
                                    .log_err();
                            }
                        })
                        .into_any_element()
                }),
            || Divider::horizontal().into_any_element(),
        ))
        .child(Divider::horizontal())
        .child(
            Button::new("agent_settings", "Add Many Others")
                .size(ButtonSize::Large)
                .icon(IconName::Plus)
                .icon_position(IconPosition::Start)
                .icon_color(Color::Muted)
                .icon_size(IconSize::XSmall)
                .on_click(|_event, window, cx| {
                    window.dispatch_action(OpenSettings.boxed_clone(), cx)
                })
                .tab_index({
                    *tab_index += 1;
                    *tab_index - 1
                }),
        )
}

pub(crate) fn render_ai_setup_page(
    workspace: WeakEntity<Workspace>,
    user_store: Entity<UserStore>,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let mut tab_index = 0;
    let is_ai_disabled = DisableAiSettings::get_global(cx).disable_ai;

    v_flex()
        .gap_2()
        .child(
            SwitchField::new(
                "enable_ai",
                "Enable AI features",
                None,
                if is_ai_disabled {
                    ToggleState::Unselected
                } else {
                    ToggleState::Selected
                },
                |&toggle_state, _, cx| {
                    let fs = <dyn Fs>::global(cx);
                    update_settings_file::<DisableAiSettings>(
                        fs,
                        cx,
                        move |ai_settings: &mut Option<bool>, _| {
                            *ai_settings = match toggle_state {
                                ToggleState::Indeterminate => None,
                                ToggleState::Unselected => Some(true),
                                ToggleState::Selected => Some(false),
                            };
                        },
                    );
                },
            )
            .tab_index({
                tab_index += 1;
                tab_index - 1
            }),
        )
        .child(render_privacy_card(&mut tab_index, is_ai_disabled, cx))
        .child(
            v_flex()
                .mt_2()
                .gap_6()
                .child(AiUpsellCard {
                    sign_in_status: SignInStatus::SignedIn,
                    sign_in: Arc::new(|_, _| {}),
                    user_plan: user_store.read(cx).plan(),
                    tab_index: Some({
                        tab_index += 1;
                        tab_index - 1
                    }),
                })
                .child(render_llm_provider_section(
                    &mut tab_index,
                    workspace,
                    is_ai_disabled,
                    window,
                    cx,
                ))
                .when(is_ai_disabled, |this| {
                    this.child(
                        div()
                            .id("backdrop")
                            .size_full()
                            .absolute()
                            .inset_0()
                            .bg(cx.theme().colors().editor_background)
                            .opacity(0.8)
                            .block_mouse_except_scroll(),
                    )
                }),
        )
}

struct AiConfigurationModal {
    focus_handle: FocusHandle,
    selected_provider: Arc<dyn LanguageModelProvider>,
    configuration_view: AnyView,
}

impl AiConfigurationModal {
    fn new(
        selected_provider: Arc<dyn LanguageModelProvider>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let configuration_view = selected_provider.configuration_view(window, cx);

        Self {
            focus_handle,
            configuration_view,
            selected_provider,
        }
    }
}

impl ModalView for AiConfigurationModal {}

impl EventEmitter<DismissEvent> for AiConfigurationModal {}

impl Focusable for AiConfigurationModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AiConfigurationModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(34.))
            .elevation_3(cx)
            .track_focus(&self.focus_handle)
            .child(
                Modal::new("onboarding-ai-setup-modal", None)
                    .header(
                        ModalHeader::new()
                            .icon(
                                Icon::new(self.selected_provider.icon())
                                    .color(Color::Muted)
                                    .size(IconSize::Small),
                            )
                            .headline(self.selected_provider.name().0),
                    )
                    .section(Section::new().child(self.configuration_view.clone()))
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_1()
                                .child(
                                    Button::new("onboarding-closing-cancel", "Cancel")
                                        .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                                )
                                .child(Button::new("save-btn", "Done").on_click(cx.listener(
                                    |_, _, window, cx| {
                                        window.dispatch_action(menu::Confirm.boxed_clone(), cx);
                                        cx.emit(DismissEvent);
                                    },
                                ))),
                        ),
                    ),
            )
    }
}

pub struct AiPrivacyTooltip {}

impl AiPrivacyTooltip {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for AiPrivacyTooltip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const DESCRIPTION: &'static str = "One of Zed's most important principles is transparency. This is why we are and value open-source so much. And it wouldn't be any different with AI.";

        tooltip_container(window, cx, move |this, _, _| {
            this.child(
                h_flex()
                    .gap_1()
                    .child(
                        Icon::new(IconName::ShieldCheck)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(Label::new("Privacy Principle")),
            )
            .child(
                div().max_w_64().child(
                    Label::new(DESCRIPTION)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
            )
        })
    }
}
