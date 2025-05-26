use gpui::{Context, Entity, Window};
use language_model::{LanguageModel, LanguageModelAvailability};
use language_model_repository::HuggingFaceModelRepository;
use std::sync::Arc;
use ui::{prelude::*, IconName, Label, LabelSize, Button, ButtonStyle, List, ListItem, Icon};

/// Manager for browsing and managing AI models
pub struct ModelManager {
    available_models: Vec<Arc<dyn LanguageModel>>,
    model_repository: Option<Entity<HuggingFaceModelRepository>>,
    #[allow(dead_code)]
    selected_model: Option<Arc<dyn LanguageModel>>,
    view_mode: ModelViewMode,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModelViewMode {
    Available,
    Repository,
    Favorites,
}

impl ModelManager {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            available_models: Vec::new(),
            model_repository: None,
            selected_model: None,
            view_mode: ModelViewMode::Available,
        }
    }

    pub fn set_view_mode(&mut self, mode: ModelViewMode, cx: &mut Context<Self>) {
        self.view_mode = mode;
        cx.notify();
    }

    pub fn open_repository(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.model_repository.is_none() {
            // Use a default models directory - this could be configurable
            let models_dir = std::path::PathBuf::from("~/.cache/zed/models");
            let http_client = Arc::new(reqwest_client::ReqwestClient::new());
            
            self.model_repository = Some(cx.new(|cx| {
                HuggingFaceModelRepository::new(http_client, models_dir, window, cx)
            }));
        }
        self.set_view_mode(ModelViewMode::Repository, cx);
    }

    pub fn close_repository(&mut self, cx: &mut Context<Self>) {
        self.model_repository = None;
        self.set_view_mode(ModelViewMode::Available, cx);
    }

    pub fn refresh_models(&mut self, cx: &mut Context<Self>) {
        // TODO: Refresh available models from providers
        cx.notify();
    }

    fn get_model_availability_info(&self, model: &Arc<dyn LanguageModel>) -> (IconName, Color, &'static str) {
        match model.availability() {
            LanguageModelAvailability::Public => (IconName::Globe, Color::Success, "Public"),
            LanguageModelAvailability::RequiresPlan(_) => (IconName::LockOutlined, Color::Warning, "Requires Plan"),
        }
    }

    fn render_view_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_2()
            .child(
                Button::new("available", "Available")
                    .style(if self.view_mode == ModelViewMode::Available {
                        ButtonStyle::Filled
                    } else {
                        ButtonStyle::Subtle
                    })
                    .icon(Some(IconName::FileCode))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.set_view_mode(ModelViewMode::Available, cx);
                    }))
            )
            .child(
                Button::new("repository", "Repository")
                    .style(if self.view_mode == ModelViewMode::Repository {
                        ButtonStyle::Filled
                    } else {
                        ButtonStyle::Subtle
                    })
                    .icon(Some(IconName::Download))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.open_repository(window, cx);
                    }))
            )
            .child(
                Button::new("favorites", "Favorites")
                    .style(if self.view_mode == ModelViewMode::Favorites {
                        ButtonStyle::Filled
                    } else {
                        ButtonStyle::Subtle
                    })
                    .icon(Some(IconName::Star))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.set_view_mode(ModelViewMode::Favorites, cx);
                    }))
            )
    }

    fn render_available_models(&self, cx: &mut Context<Self>) -> AnyElement {
        if self.available_models.is_empty() {
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
                            Icon::new(IconName::FileCode)
                                .size(ui::IconSize::XLarge)
                                .color(Color::Muted)
                        )
                        .child(
                            Label::new("No models available")
                                .size(LabelSize::Default)
                                .color(Color::Muted)
                        )
                        .child(
                            Label::new("Configure language model providers to see available models")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                        )
                        .child(
                            div()
                                .mt_4()
                                .child(
                                    Button::new("refresh", "Refresh")
                                        .style(ButtonStyle::Filled)
                                        .icon(Some(IconName::RotateCcw))
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.refresh_models(cx);
                                        }))
                                )
                        )
                )
        } else {
            div()
                .child(
                    List::new()
                        .children(
                            self.available_models.iter().map(|model| {
                                let (icon, color, status) = self.get_model_availability_info(model);
                                let model_id = model.id();
                                
                                ListItem::new(model_id.0.clone())
                                    .start_slot(Icon::new(icon).color(color))
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
                                                        Label::new(model.name().0.clone())
                                                            .size(LabelSize::Default)
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .gap_4()
                                                            .child(
                                                                Label::new(format!("Provider: {}", model.provider_name().0))
                                                                    .size(LabelSize::Small)
                                                                    .color(Color::Muted)
                                                            )
                                                            .child(
                                                                Label::new(format!("Max tokens: {}", model.max_token_count()))
                                                                    .size(LabelSize::Small)
                                                                    .color(Color::Muted)
                                                            )
                                                            .child(
                                                                Label::new(status)
                                                                    .size(LabelSize::Small)
                                                                    .color(color)
                                                            )
                                                    )
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .gap_2()
                                                    .child(
                                                        Button::new("test", "Test")
                                                            .style(ButtonStyle::Subtle)
                                                            .icon(Some(IconName::Play))
                                                            .on_click({
                                                                let model_name = model.name().0.clone();
                                                                cx.listener(move |_this, _, _window, _cx| {
                                                                    log::info!("Test model: {}", model_name);
                                                                })
                                                            })
                                                    )
                                                    .child(
                                                        Button::new("favorite", "")
                                                            .style(ButtonStyle::Subtle)
                                                            .icon(Some(IconName::Star))
                                                            .on_click({
                                                                let model_name = model.name().0.clone();
                                                                cx.listener(move |_this, _, _window, _cx| {
                                                                    log::info!("Favorite model: {}", model_name);
                                                                })
                                                            })
                                                    )
                                            )
                                    )
                            })
                        )
                )
        }.into_any_element()
    }

    fn render_repository(&self, _cx: &mut Context<Self>) -> AnyElement {
        if let Some(repository) = &self.model_repository {
            repository.clone().into_any_element()
        } else {
            div()
                .flex()
                .items_center()
                .justify_center()
                .h_64()
                .bg(_cx.theme().colors().surface_background)
                .rounded_lg()
                .child(
                    Label::new("Loading repository...")
                        .size(LabelSize::Default)
                        .color(Color::Muted)
                )
                .into_any_element()
        }
    }

    fn render_favorites(&self, cx: &mut Context<Self>) -> AnyElement {
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
                        Icon::new(IconName::Star)
                            .size(ui::IconSize::XLarge)
                            .color(Color::Muted)
                    )
                    .child(
                        Label::new("No favorite models")
                            .size(LabelSize::Default)
                            .color(Color::Muted)
                    )
                    .child(
                        Label::new("Mark models as favorites to see them here")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
            )
            .into_any_element()
    }
}

impl Render for ModelManager {
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
                        Label::new("AI Models")
                            .size(LabelSize::Large)
                    )
                    .child(self.render_view_selector(cx))
            )
            .child(
                match self.view_mode {
                    ModelViewMode::Available => self.render_available_models(cx),
                    ModelViewMode::Repository => self.render_repository(cx),
                    ModelViewMode::Favorites => self.render_favorites(cx),
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
                                Label::new("Model Information")
                                    .size(LabelSize::Default)
                            )
                    )
                    .child(
                        Label::new("Browse available AI models from configured providers, download new models from repositories, or manage your favorite models.")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
            )
    }
} 