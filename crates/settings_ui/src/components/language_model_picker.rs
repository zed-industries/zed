use std::sync::Arc;

use fuzzy::StringMatch;
use gpui::{AnyElement, App, Context, DismissEvent, SharedString, Task, Window, px};
use language_model::{LanguageModel, LanguageModelRegistry};
use picker::{Picker, PickerDelegate};
use ui::{ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle, prelude::*};

use crate::{render_picker_trigger_button, wire_picker_trigger_a11y};

type LanguageModelPicker = Picker<LanguageModelPickerDelegate>;

/// The same model source as the agent panel's `LanguageModelSelector`: every
/// visible provider that is currently authenticated contributes its recommended
/// models first, then its remaining models.
fn all_models(cx: &App) -> Vec<LanguageModel> {
    let providers: Vec<_> = LanguageModelRegistry::global(cx)
        .read(cx)
        .visible_providers()
        .into_iter()
        .filter(|provider| provider.is_authenticated(cx))
        .collect();

    let mut models = Vec::new();
    for provider in &providers {
        models.extend(provider.recommended_models(cx));
    }
    for provider in &providers {
        for model in provider.provided_models(cx) {
            let already_recommended = models.iter().any(|existing| {
                existing.provider_id() == model.provider_id() && existing.id() == model.id()
            });
            if !already_recommended {
                models.push(model);
            }
        }
    }
    models
}

fn matches_for_query(models: &[LanguageModel], query: &str) -> Vec<StringMatch> {
    let query_lower = query.to_lowercase();
    models
        .iter()
        .enumerate()
        .filter(|(_, model)| {
            query.is_empty()
                || format!("{}/{}", model.provider_name().0, model.name().0)
                    .to_lowercase()
                    .contains(&query_lower)
        })
        .map(|(index, model)| StringMatch {
            candidate_id: index,
            string: format!("{}/{}", model.provider_name().0, model.name().0),
            positions: Vec::new(),
            score: 0.0,
        })
        .collect()
}

struct LanguageModelPickerDelegate {
    models: Vec<LanguageModel>,
    filtered_models: Vec<StringMatch>,
    selected_index: usize,
    on_select: Arc<dyn Fn(LanguageModel, &mut App) + 'static>,
}

impl LanguageModelPickerDelegate {
    fn new(
        current_model: Option<LanguageModel>,
        on_select: Arc<dyn Fn(LanguageModel, &mut App) + 'static>,
        cx: &mut Context<LanguageModelPicker>,
    ) -> Self {
        let models = all_models(cx);
        let filtered_models = matches_for_query(&models, "");
        let selected_index = current_model
            .and_then(|current| {
                models.iter().position(|model| {
                    model.provider_id() == current.provider_id() && model.id() == current.id()
                })
            })
            .unwrap_or(0);

        Self {
            models,
            filtered_models,
            selected_index,
            on_select,
        }
    }
}

impl PickerDelegate for LanguageModelPickerDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "language model picker"
    }

    fn match_count(&self) -> usize {
        self.filtered_models.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _: &mut Window,
        cx: &mut Context<LanguageModelPicker>,
    ) {
        self.selected_index = ix.min(self.filtered_models.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a model…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No models available. Configure a provider in LLM Providers.".into())
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<LanguageModelPicker>,
    ) -> Task<()> {
        self.filtered_models = matches_for_query(&self.models, &query);
        self.selected_index = 0;
        cx.notify();

        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        _window: &mut Window,
        cx: &mut Context<LanguageModelPicker>,
    ) {
        let Some(model_match) = self.filtered_models.get(self.selected_index) else {
            return;
        };
        let Some(model) = self.models.get(model_match.candidate_id).cloned() else {
            return;
        };

        (self.on_select)(model, cx);
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<LanguageModelPicker>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<LanguageModelPicker>,
    ) -> Option<Self::ListItem> {
        let model_match = self.filtered_models.get(ix)?;
        let model = self.models.get(model_match.candidate_id)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex().gap_2().child(Label::new(model.name().0)).child(
                        Label::new(model.provider_name().0)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
                )
                .into_any_element(),
        )
    }
}

pub fn render_language_model_picker(
    trigger_label: SharedString,
    current_model: Option<LanguageModel>,
    disabled: bool,
    on_select: impl Fn(LanguageModel, &mut App) + 'static,
    _window: &mut Window,
    _cx: &mut App,
) -> AnyElement {
    let on_select = Arc::new(on_select);
    let handle = PopoverMenuHandle::default();

    PopoverMenu::new("language-model-picker")
        .trigger(wire_picker_trigger_a11y(
            render_picker_trigger_button("language_model_picker_trigger".into(), trigger_label)
                .disabled(disabled)
                .aria_label("Default Model"),
            handle.clone(),
        ))
        .menu(move |window, cx| {
            let on_select = on_select.clone();
            let current_model = current_model.clone();
            Some(cx.new(|cx| {
                Picker::uniform_list(
                    LanguageModelPickerDelegate::new(current_model, on_select, cx),
                    window,
                    cx,
                )
                .show_scrollbar(true)
                .initial_width(rems_from_px(280_f32))
                .max_height(rems(18.))
                .popover()
            }))
        })
        .anchor(gpui::Anchor::TopLeft)
        .offset(gpui::Point {
            x: px(0.0),
            y: px(2.0),
        })
        .with_handle(handle)
        .into_any_element()
}
