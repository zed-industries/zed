use std::sync::Arc;

use collections::HashMap;
use gpui::{AnyView, AppContext, EventEmitter, FocusHandle, FocusableView, Subscription};
use language_model::{LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry};
use ui::{prelude::*, ElevationIndex};

pub struct AssistantConfiguration {
    focus_handle: FocusHandle,
    configuration_views_by_provider: HashMap<LanguageModelProviderId, AnyView>,
    _registry_subscription: Subscription,
}

impl AssistantConfiguration {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let registry_subscription = cx.subscribe(
            &LanguageModelRegistry::global(cx),
            |this, _, event: &language_model::Event, cx| match event {
                language_model::Event::AddedProvider(provider_id) => {
                    let provider = LanguageModelRegistry::read_global(cx).provider(provider_id);
                    if let Some(provider) = provider {
                        this.add_provider_configuration_view(&provider, cx);
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
        this.build_provider_configuration_views(cx);
        this
    }

    fn build_provider_configuration_views(&mut self, cx: &mut ViewContext<Self>) {
        let providers = LanguageModelRegistry::read_global(cx).providers();
        for provider in providers {
            self.add_provider_configuration_view(&provider, cx);
        }
    }

    fn remove_provider_configuration_view(&mut self, provider_id: &LanguageModelProviderId) {
        self.configuration_views_by_provider.remove(provider_id);
    }

    fn add_provider_configuration_view(
        &mut self,
        provider: &Arc<dyn LanguageModelProvider>,
        cx: &mut ViewContext<Self>,
    ) {
        let configuration_view = provider.configuration_view(cx);
        self.configuration_views_by_provider
            .insert(provider.id(), configuration_view);
    }
}

impl FocusableView for AssistantConfiguration {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
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
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let provider_id = provider.id().0.clone();
        let provider_name = provider.name().0.clone();
        let configuration_view = self
            .configuration_views_by_provider
            .get(&provider.id())
            .cloned();

        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .justify_between()
                    .child(Headline::new(provider_name.clone()).size(HeadlineSize::Small))
                    .when(provider.is_authenticated(cx), |parent| {
                        parent.child(
                            h_flex().justify_end().child(
                                Button::new(
                                    SharedString::from(format!("new-thread-{provider_id}")),
                                    "Open New Thread",
                                )
                                .icon_position(IconPosition::Start)
                                .icon(IconName::Plus)
                                .style(ButtonStyle::Filled)
                                .layer(ElevationIndex::ModalSurface)
                                .on_click(cx.listener({
                                    let provider = provider.clone();
                                    move |_this, _event, cx| {
                                        cx.emit(AssistantConfigurationEvent::NewThread(
                                            provider.clone(),
                                        ))
                                    }
                                })),
                            ),
                        )
                    }),
            )
            .child(
                div()
                    .p(DynamicSpacing::Base08.rems(cx))
                    .bg(cx.theme().colors().surface_background)
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let providers = LanguageModelRegistry::read_global(cx).providers();

        v_flex()
            .id("assistant-configuration")
            .track_focus(&self.focus_handle(cx))
            .bg(cx.theme().colors().editor_background)
            .size_full()
            .overflow_y_scroll()
            .child(
                v_flex()
                    .p(DynamicSpacing::Base16.rems(cx))
                    .mt_1()
                    .gap_6()
                    .flex_1()
                    .children(
                        providers
                            .into_iter()
                            .map(|provider| self.render_provider_configuration(&provider, cx)),
                    ),
            )
    }
}
