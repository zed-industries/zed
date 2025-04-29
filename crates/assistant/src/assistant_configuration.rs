use std::sync::Arc;

use collections::HashMap;
use gpui::{AnyView, App, EventEmitter, FocusHandle, Focusable, Subscription, canvas};
use language_model::{LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry};
use ui::{ElevationIndex, prelude::*};
use workspace::Item;

pub struct ConfigurationView {
    focus_handle: FocusHandle,
    configuration_views: HashMap<LanguageModelProviderId, AnyView>,
    _registry_subscription: Subscription,
}

impl ConfigurationView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let registry_subscription = cx.subscribe_in(
            &LanguageModelRegistry::global(cx),
            window,
            |this, _, event: &language_model::Event, window, cx| match event {
                language_model::Event::AddedProvider(provider_id) => {
                    let provider = LanguageModelRegistry::read_global(cx).provider(provider_id);
                    if let Some(provider) = provider {
                        this.add_configuration_view(&provider, window, cx);
                    }
                }
                language_model::Event::RemovedProvider(provider_id) => {
                    this.remove_configuration_view(provider_id);
                }
                _ => {}
            },
        );

        let mut this = Self {
            focus_handle,
            configuration_views: HashMap::default(),
            _registry_subscription: registry_subscription,
        };
        this.build_configuration_views(window, cx);
        this
    }

    fn build_configuration_views(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let providers = LanguageModelRegistry::read_global(cx).providers();
        for provider in providers {
            self.add_configuration_view(&provider, window, cx);
        }
    }

    fn remove_configuration_view(&mut self, provider_id: &LanguageModelProviderId) {
        self.configuration_views.remove(provider_id);
    }

    fn add_configuration_view(
        &mut self,
        provider: &Arc<dyn LanguageModelProvider>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let configuration_view = provider.configuration_view(window, cx);
        self.configuration_views
            .insert(provider.id(), configuration_view);
    }

    fn render_provider_view(
        &mut self,
        provider: &Arc<dyn LanguageModelProvider>,
        cx: &mut Context<Self>,
    ) -> Div {
        let provider_id = provider.id().0.clone();
        let provider_name = provider.name().0.clone();
        let configuration_view = self.configuration_views.get(&provider.id()).cloned();

        let open_new_context = cx.listener({
            let provider = provider.clone();
            move |_, _, _window, cx| {
                cx.emit(ConfigurationViewEvent::NewProviderContextEditor(
                    provider.clone(),
                ))
            }
        });

        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .justify_between()
                    .child(Headline::new(provider_name.clone()).size(HeadlineSize::Small))
                    .when(provider.is_authenticated(cx), move |this| {
                        this.child(
                            h_flex().justify_end().child(
                                Button::new(
                                    SharedString::from(format!("new-context-{provider_id}")),
                                    "Open New Chat",
                                )
                                .icon_position(IconPosition::Start)
                                .icon(IconName::Plus)
                                .style(ButtonStyle::Filled)
                                .layer(ElevationIndex::ModalSurface)
                                .on_click(open_new_context),
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
                    .rounded_sm()
                    .when(configuration_view.is_none(), |this| {
                        this.child(div().child(Label::new(format!(
                            "No configuration view for {}",
                            provider_name
                        ))))
                    })
                    .when_some(configuration_view, |this, configuration_view| {
                        this.child(configuration_view)
                    }),
            )
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let providers = LanguageModelRegistry::read_global(cx).providers();
        let provider_views = providers
            .into_iter()
            .map(|provider| self.render_provider_view(&provider, cx))
            .collect::<Vec<_>>();

        let mut element = v_flex()
            .id("assistant-configuration-view")
            .track_focus(&self.focus_handle(cx))
            .bg(cx.theme().colors().editor_background)
            .size_full()
            .overflow_y_scroll()
            .child(
                v_flex()
                    .p(DynamicSpacing::Base16.rems(cx))
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .gap_1()
                    .child(Headline::new("Configure your Assistant").size(HeadlineSize::Medium))
                    .child(
                        Label::new(
                            "At least one LLM provider must be configured to use the Assistant.",
                        )
                        .color(Color::Muted),
                    ),
            )
            .child(
                v_flex()
                    .p(DynamicSpacing::Base16.rems(cx))
                    .mt_1()
                    .gap_6()
                    .flex_1()
                    .children(provider_views),
            )
            .into_any();

        // We use a canvas here to get scrolling to work in the ConfigurationView. It's a workaround
        // because we couldn't the element to take up the size of the parent.
        canvas(
            move |bounds, window, cx| {
                element.prepaint_as_root(bounds.origin, bounds.size.into(), window, cx);
                element
            },
            |_, mut element, window, cx| {
                element.paint(window, cx);
            },
        )
        .flex_1()
        .w_full()
    }
}

pub enum ConfigurationViewEvent {
    NewProviderContextEditor(Arc<dyn LanguageModelProvider>),
}

impl EventEmitter<ConfigurationViewEvent> for ConfigurationView {}

impl Focusable for ConfigurationView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ConfigurationView {
    type Event = ConfigurationViewEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Configuration".into()
    }
}
