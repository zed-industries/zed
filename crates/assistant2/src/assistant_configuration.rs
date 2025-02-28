use std::sync::Arc;

use collections::HashMap;
use gpui::{Action, AnyView, App, EventEmitter, FocusHandle, Focusable, Subscription};
use language_model::{LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry};
use ui::{prelude::*, Divider, DividerColor, ElevationIndex};
use zed_actions::assistant::DeployPromptLibrary;

pub struct AssistantConfiguration {
    focus_handle: FocusHandle,
    configuration_views_by_provider: HashMap<LanguageModelProviderId, AnyView>,
    _registry_subscription: Subscription,
}

impl AssistantConfiguration {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let registry_subscription = cx.subscribe_in(
            &LanguageModelRegistry::global(cx),
            window,
            |this, _, event: &language_model::Event, window, cx| match event {
                language_model::Event::AddedProvider(provider_id) => {
                    let provider = LanguageModelRegistry::read_global(cx).provider(provider_id);
                    if let Some(provider) = provider {
                        this.add_provider_configuration_view(&provider, window, cx);
                    }
                }
                language_model::Event::RemovedProvider(provider_id) => {
                    this.remove_provider_configuration_view(provider_id);
                }
                _ => {}
            },
        );

        let mut this = Self {
            focus_handle,
            configuration_views_by_provider: HashMap::default(),
            _registry_subscription: registry_subscription,
        };
        this.build_provider_configuration_views(window, cx);
        this
    }

    fn build_provider_configuration_views(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let providers = LanguageModelRegistry::read_global(cx).providers();
        for provider in providers {
            self.add_provider_configuration_view(&provider, window, cx);
        }
    }

    fn remove_provider_configuration_view(&mut self, provider_id: &LanguageModelProviderId) {
        self.configuration_views_by_provider.remove(provider_id);
    }

    fn add_provider_configuration_view(
        &mut self,
        provider: &Arc<dyn LanguageModelProvider>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let configuration_view = provider.configuration_view(window, cx);
        self.configuration_views_by_provider
            .insert(provider.id(), configuration_view);
    }
}

impl Focusable for AssistantConfiguration {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub enum AssistantConfigurationEvent {
    NewThread(Arc<dyn LanguageModelProvider>),
}

impl EventEmitter<AssistantConfigurationEvent> for AssistantConfiguration {}

impl AssistantConfiguration {
    fn render_provider_configuration(
        &mut self,
        provider: &Arc<dyn LanguageModelProvider>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let provider_id = provider.id().0.clone();
        let provider_name = provider.name().0.clone();
        let configuration_view = self
            .configuration_views_by_provider
            .get(&provider.id())
            .cloned();

        v_flex()
            .gap_1p5()
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Icon::new(provider.icon())
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(Label::new(provider_name.clone())),
                    )
                    .when(provider.is_authenticated(cx), |parent| {
                        parent.child(
                            Button::new(
                                SharedString::from(format!("new-thread-{provider_id}")),
                                "Start New Thread",
                            )
                            .icon_position(IconPosition::Start)
                            .icon(IconName::Plus)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Filled)
                            .layer(ElevationIndex::ModalSurface)
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener({
                                let provider = provider.clone();
                                move |_this, _event, _window, cx| {
                                    cx.emit(AssistantConfigurationEvent::NewThread(
                                        provider.clone(),
                                    ))
                                }
                            })),
                        )
                    }),
            )
            .child(
                div()
                    .p(DynamicSpacing::Base08.rems(cx))
                    .bg(cx.theme().colors().editor_background)
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .rounded_md()
                    .map(|parent| match configuration_view {
                        Some(configuration_view) => parent.child(configuration_view),
                        None => parent.child(div().child(Label::new(format!(
                            "No configuration view for {provider_name}",
                        )))),
                    }),
            )
    }
}

impl Render for AssistantConfiguration {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let providers = LanguageModelRegistry::read_global(cx).providers();

        v_flex()
            .id("assistant-configuration")
            .track_focus(&self.focus_handle(cx))
            .bg(cx.theme().colors().panel_background)
            .size_full()
            .overflow_y_scroll()
            .child(
                v_flex()
                    .p(DynamicSpacing::Base16.rems(cx))
                    .gap_2()
                    .child(
                        v_flex()
                            .gap_0p5()
                            .child(Headline::new("Prompt Library").size(HeadlineSize::Small))
                            .child(
                                Label::new("Create reusable prompts and tag which ones you want sent in every LLM interaction.")
                                    .color(Color::Muted),
                            ),
                    )
                    .child(
                        Button::new("open-prompt-library", "Open Prompt Library")
                            .style(ButtonStyle::Filled)
                            .layer(ElevationIndex::ModalSurface)
                            .full_width()
                            .icon(IconName::Book)
                            .icon_size(IconSize::Small)
                            .icon_position(IconPosition::Start)
                            .on_click(|_event, window, cx| {
                                window.dispatch_action(DeployPromptLibrary.boxed_clone(), cx)
                            }),
                    ),
            )
            .child(Divider::horizontal().color(DividerColor::Border))
            .child(
                v_flex()
                    .p(DynamicSpacing::Base16.rems(cx))
                    .mt_1()
                    .gap_6()
                    .flex_1()
                    .child(
                        v_flex()
                            .gap_0p5()
                            .child(Headline::new("LLM Providers").size(HeadlineSize::Small))
                            .child(
                                Label::new("Add at least one provider to use AI-powered features.")
                                    .color(Color::Muted),
                            ),
                    )
                    .children(
                        providers
                            .into_iter()
                            .map(|provider| self.render_provider_configuration(&provider, cx)),
                    ),
            )
    }
}
