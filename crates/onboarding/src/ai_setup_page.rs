use std::sync::Arc;

use ai_onboarding::{AiUpsellCard, SignInStatus};
use gpui::{
    Action, AnyView, App, DismissEvent, EventEmitter, FocusHandle, Focusable, WeakEntity, Window,
    prelude::*,
};
use itertools;

use language_model::{LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry};
use ui::{ButtonLike, Divider, Modal, ModalFooter, ModalHeader, Section, SwitchField, prelude::*};
use workspace::{ModalView, Workspace};

use util::ResultExt;
use zed_actions::agent::OpenSettings;

const FEATURED_PROVIDERS: [&'static str; 4] = ["anthropic", "google", "openai", "ollama"];

pub(crate) struct AiConfigurationPage {
    workspace: WeakEntity<Workspace>,
}

impl AiConfigurationPage {
    pub(crate) fn new(workspace: WeakEntity<Workspace>) -> Self {
        Self { workspace }
    }

    fn open_configuration_modal(
        &mut self,
        selected_provider: Arc<dyn LanguageModelProvider>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    let modal = AiConfigurationModal::new(selected_provider, window, cx);
                    window.focus(&modal.focus_handle(cx));
                    modal
                });
            })
            .log_err();
    }

    fn render_llm_provider_card(
        &mut self,
        _: &mut Window,
        cx: &mut Context<Self>,
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
                        let group_name =
                            SharedString::new(format!("onboarding-hover-group-{}", index));

                        ButtonLike::new(("onboarding-ai-setup-buttons", index))
                            .size(ButtonSize::Large)
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
                                    // TODO: Change this element when the provider is configured
                                    .child(
                                        h_flex()
                                            .visible_on_hover(group_name)
                                            .gap_1()
                                            .child(
                                                Icon::new(IconName::Settings)
                                                    .color(Color::Muted)
                                                    .size(IconSize::XSmall),
                                            )
                                            .child(
                                                Label::new("Configure")
                                                    .color(Color::Muted)
                                                    .size(LabelSize::Small),
                                            ),
                                    ),
                            )
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.open_configuration_modal(provider.clone(), window, cx)
                            }))
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
                    }),
            )
    }

    fn render_llm_provider_section(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
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
            .child(self.render_llm_provider_card(window, cx))
    }

    fn render_privacy_card(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .relative()
            .pt_2()
            .pb_2p5()
            .pl_3()
            .pr_2()
            .border_1()
            .border_dashed()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().surface_background.opacity(0.3))
            .rounded_lg()
            .overflow_hidden()
            .child(
                h_flex()
                    .gap_2()
                    .justify_between()
                    .child(Label::new("We don't train models using your data"))
                    .child(
                        Button::new("learn_more", "Learn More")
                            .icon(IconName::ArrowUpRight)
                            .icon_size(IconSize::XSmall)
                            .color(Color::Muted),
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
}

impl Render for AiConfigurationPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(SwitchField::new(
                "enable_ai",
                "Enable AI features",
                None,
                ui::ToggleState::Selected,
                |_, _, _| {},
            ))
            .child(self.render_privacy_card(window, cx))
            .child(AiUpsellCard {
                sign_in_status: SignInStatus::SignedIn,
                sign_in: Arc::new(|_, _| {}),
            })
            .child(self.render_llm_provider_section(window, cx))
    }
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
                                .child(
                                    Button::new("save-btn", "Configure Provider")
                                        // TODO: save configuration
                                        .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                                ),
                        ),
                    ),
            )
    }
}
