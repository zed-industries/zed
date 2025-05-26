use gpui::{Context, Window};
use language_model::{LanguageModelProvider, LanguageModelProviderId};
use std::collections::HashMap;
use std::sync::Arc;
use ui::{prelude::*, IconName, Label, LabelSize, Button, ButtonStyle, List, ListItem, Icon};

/// Registry for managing language model providers
pub struct ProviderRegistry {
    providers: HashMap<LanguageModelProviderId, Arc<dyn LanguageModelProvider>>,
}

impl ProviderRegistry {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn add_provider(&mut self, provider: Arc<dyn LanguageModelProvider>, cx: &mut Context<Self>) {
        let id = provider.id();
        self.providers.insert(id, provider);
        cx.notify();
    }

    pub fn remove_provider(&mut self, provider_id: &LanguageModelProviderId, cx: &mut Context<Self>) {
        self.providers.remove(provider_id);
        cx.notify();
    }

    pub fn get_provider(&self, provider_id: &LanguageModelProviderId) -> Option<Arc<dyn LanguageModelProvider>> {
        self.providers.get(provider_id).cloned()
    }

    pub fn list_providers(&self) -> Vec<Arc<dyn LanguageModelProvider>> {
        self.providers.values().cloned().collect()
    }

    fn get_provider_status(&self, _provider: &Arc<dyn LanguageModelProvider>) -> ProviderStatus {
        // This would need access to the app context to check authentication
        // For now, we'll return a placeholder
        ProviderStatus::Unknown
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum ProviderStatus {
    Connected,
    Disconnected,
    Error(String),
    Unknown,
}

impl ProviderStatus {
    #[allow(dead_code)]
    fn icon(&self) -> IconName {
        match self {
            ProviderStatus::Connected => IconName::Check,
            ProviderStatus::Disconnected => IconName::X,
            ProviderStatus::Error(_) => IconName::Warning,
            ProviderStatus::Unknown => IconName::Circle,
        }
    }

    #[allow(dead_code)]
    fn color(&self) -> Color {
        match self {
            ProviderStatus::Connected => Color::Success,
            ProviderStatus::Disconnected => Color::Muted,
            ProviderStatus::Error(_) => Color::Error,
            ProviderStatus::Unknown => Color::Muted,
        }
    }

    #[allow(dead_code)]
    fn label(&self) -> &str {
        match self {
            ProviderStatus::Connected => "Connected",
            ProviderStatus::Disconnected => "Disconnected",
            ProviderStatus::Error(_) => "Error",
            ProviderStatus::Unknown => "Unknown",
        }
    }
}

impl Render for ProviderRegistry {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_6()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        Label::new("Language Model Providers")
                            .size(LabelSize::Large)
                    )
                    .child(
                        Button::new("add_provider", "Add Provider")
                            .style(ButtonStyle::Filled)
                            .icon(Some(IconName::Plus))
                            .on_click(cx.listener(|_this, _, _window, _cx| {
                                // TODO: Implement add provider dialog
                                log::info!("Add provider clicked");
                            }))
                    )
            )
            .child(
                if self.providers.is_empty() {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h_64()
                        .bg(cx.theme().colors().surface_background)
                        .rounded_lg()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_2()
                                .child(
                                    Icon::new(IconName::Server)
                                        .size(ui::IconSize::XLarge)
                                        .color(Color::Muted)
                                )
                                .child(
                                    Label::new("No providers configured")
                                        .size(LabelSize::Default)
                                        .color(Color::Muted)
                                )
                                .child(
                                    Label::new("Add a language model provider to get started")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                )
                        )
                } else {
                    div()
                        .child(
                            List::new()
                                .children(
                                    self.providers.values().map(|provider| {
                                        let provider_id = provider.id();
                                        let provider_name = provider.name().0.clone();
                                        let provider_icon = provider.icon();
                                        
                                        // Calculate status outside to avoid lifetime issues
                                        let (status_icon, status_color, status_label) = match self.get_provider_status(provider) {
                                            ProviderStatus::Connected => (IconName::Check, Color::Success, "Connected"),
                                            ProviderStatus::Disconnected => (IconName::X, Color::Muted, "Disconnected"),
                                            ProviderStatus::Error(_) => (IconName::Warning, Color::Error, "Error"),
                                            ProviderStatus::Unknown => (IconName::Circle, Color::Muted, "Unknown"),
                                        };
                                        
                                        ListItem::new(provider_id.0.clone())
                                            .start_slot(Icon::new(provider_icon))
                                            .child(
                                                div()
                                                    .flex()
                                                    .justify_between()
                                                    .items_center()
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .gap_1()
                                                            .child(
                                                                Label::new(provider_name)
                                                                    .size(LabelSize::Default)
                                                            )
                                                            .child(
                                                                Label::new(provider_id.0.clone())
                                                                    .size(LabelSize::Small)
                                                                    .color(Color::Muted)
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap_2()
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .items_center()
                                                                    .gap_1()
                                                                    .child(
                                                                        Icon::new(status_icon)
                                                                            .size(ui::IconSize::Small)
                                                                            .color(status_color)
                                                                    )
                                                                    .child(
                                                                        Label::new(status_label)
                                                                            .size(LabelSize::Small)
                                                                            .color(status_color)
                                                                    )
                                                            )
                                                            .child(
                                                                Button::new("configure", "Configure")
                                                                    .style(ButtonStyle::Subtle)
                                                                    .icon(Some(IconName::Settings))
                                                                    .on_click({
                                                                        let provider_id = provider_id.clone();
                                                                        cx.listener(move |_this, _, _window, _cx| {
                                                                            log::info!("Configure provider: {}", provider_id.0);
                                                                        })
                                                                    })
                                                            )
                                                    )
                                            )
                                    })
                                )
                        )
                }
            )
            .child(
                div()
                    .mt_6()
                    .p_4()
                    .bg(cx.theme().colors().surface_background)
                    .rounded_lg()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .mb_2()
                            .child(Icon::new(IconName::Info))
                            .child(
                                Label::new("Provider Information")
                                    .size(LabelSize::Default)
                            )
                    )
                    .child(
                        Label::new("Language model providers give you access to different AI models. Each provider may require different authentication methods and have different capabilities.")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
            )
    }
} 