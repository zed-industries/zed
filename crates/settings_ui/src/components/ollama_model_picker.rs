use std::sync::Arc;

use fuzzy::StringMatch;
use gpui::{AnyElement, App, Context, DismissEvent, ReadGlobal, SharedString, Task, Window, px};
use picker::{Picker, PickerDelegate};
use settings::SettingsStore;
use ui::{ListItem, ListItemSpacing, PopoverMenu, prelude::*};
use util::ResultExt;

use crate::{
    SettingField, SettingsFieldMetadata, SettingsUiFile, render_picker_trigger_button,
    update_settings_file,
};

type OllamaModelPicker = Picker<OllamaModelPickerDelegate>;

const DEFAULT_OLLAMA_API_URL: &str = "http://localhost:11434";

fn matches_for_query(models: &[SharedString], query: &str) -> Vec<StringMatch> {
    let query_lower = query.to_lowercase();
    models
        .iter()
        .enumerate()
        .filter(|(_, model)| query.is_empty() || model.to_lowercase().contains(&query_lower))
        .map(|(index, model)| StringMatch {
            candidate_id: index,
            string: model.to_string(),
            positions: Vec::new(),
            score: 0.0,
        })
        .collect()
}

struct OllamaModelPickerDelegate {
    models: Vec<SharedString>,
    filtered_models: Vec<StringMatch>,
    selected_index: usize,
    on_model_changed: Arc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>,
    loading: bool,
    _fetch_models_task: Option<Task<()>>,
}

impl OllamaModelPickerDelegate {
    fn new(
        current_model: SharedString,
        api_url: SharedString,
        on_model_changed: impl Fn(SharedString, &mut Window, &mut App) + 'static,
        cx: &mut Context<OllamaModelPicker>,
    ) -> Self {
        let mut models = Vec::new();
        if !current_model.is_empty() {
            models.push(current_model.clone());
        }
        let filtered_models = matches_for_query(&models, "");

        let loading = !api_url.is_empty();
        let fetch_models_task = loading.then(|| {
            let http_client = cx.http_client();
            cx.spawn(async move |this, cx| {
                let result = edit_prediction::ollama::fetch_models_from_server(
                    http_client,
                    api_url.as_ref(),
                )
                .await;
                this.update(cx, move |picker, cx| {
                    picker.delegate.loading = false;
                    match result {
                        Ok(mut fetched_models) => {
                            if !current_model.is_empty() && !fetched_models.contains(&current_model)
                            {
                                fetched_models.insert(0, current_model.clone());
                            }
                            picker.delegate.models = fetched_models;
                        }
                        Err(error) => {
                            log::warn!("Failed to fetch Ollama models from {api_url}: {error}");
                        }
                    }
                    let query = picker.query(cx);
                    picker.delegate.filtered_models =
                        matches_for_query(&picker.delegate.models, &query);
                    picker.delegate.selected_index = picker
                        .delegate
                        .models
                        .iter()
                        .position(|model| *model == current_model)
                        .unwrap_or(0);
                    cx.notify();
                })
                .ok();
            })
        });

        Self {
            models,
            filtered_models,
            selected_index: 0,
            on_model_changed: Arc::new(on_model_changed),
            loading,
            _fetch_models_task: fetch_models_task,
        }
    }
}

impl PickerDelegate for OllamaModelPickerDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "ollama model picker"
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
        cx: &mut Context<OllamaModelPicker>,
    ) {
        self.selected_index = ix.min(self.filtered_models.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search models…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some(if self.loading {
            "Loading models…".into()
        } else {
            "No models found. Check your Ollama server URL.".into()
        })
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<OllamaModelPicker>,
    ) -> Task<()> {
        self.filtered_models = matches_for_query(&self.models, &query);
        self.selected_index = 0;
        cx.notify();

        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<OllamaModelPicker>,
    ) {
        let Some(model_match) = self.filtered_models.get(self.selected_index) else {
            return;
        };

        (self.on_model_changed)(model_match.string.clone().into(), window, cx);
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<OllamaModelPicker>) {
        cx.defer_in(window, |picker, window, cx| {
            picker.set_query("", window, cx);
        });
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<OllamaModelPicker>,
    ) -> Option<Self::ListItem> {
        let model_match = self.filtered_models.get(ix)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new(model_match.string.clone()))
                .into_any_element(),
        )
    }
}

pub fn render_ollama_model_picker(
    field: SettingField<settings::OllamaModelName>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    title: &'static str,
    description: &'static str,
    _window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, value) = SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_value: SharedString = value
        .map(|m| m.0.clone().into())
        .unwrap_or_else(|| "".into());

    let (_, api_url_value) = SettingsStore::global(cx).get_value_from_file(
        file.to_settings(),
        |settings: &settings::SettingsContent| {
            settings
                .project
                .all_languages
                .edit_predictions
                .as_ref()?
                .ollama
                .as_ref()?
                .api_url
                .as_ref()
        },
    );
    let api_url: SharedString = api_url_value
        .filter(|api_url| !api_url.is_empty())
        .map(|api_url| SharedString::new(api_url.clone()))
        .unwrap_or_else(|| DEFAULT_OLLAMA_API_URL.into());

    let trigger_value: SharedString = if current_value.is_empty() {
        "Select a model…".into()
    } else {
        current_value.clone()
    };

    PopoverMenu::new("ollama-model-picker")
        .trigger(
            render_picker_trigger_button("ollama_model_picker_trigger".into(), trigger_value)
                .aria_label(title)
                .when(!description.is_empty(), |this| {
                    this.aria_description(description)
                }),
        )
        .menu(move |window, cx| {
            Some(cx.new(|cx| {
                let file = file.clone();
                let current_value = current_value.clone();
                let api_url = api_url.clone();
                let delegate = OllamaModelPickerDelegate::new(
                    current_value,
                    api_url,
                    move |model_name, window, cx| {
                        update_settings_file(
                            file.clone(),
                            field.json_path,
                            window,
                            cx,
                            move |settings, app| {
                                (field.write)(
                                    settings,
                                    Some(settings::OllamaModelName(model_name.to_string())),
                                    app,
                                );
                            },
                        )
                        .log_err();
                    },
                    cx,
                );

                Picker::uniform_list(delegate, window, cx)
                    .show_scrollbar(true)
                    .initial_width(rems_from_px(210_f32))
                    .max_height(rems(18.))
                    .popover()
            }))
        })
        .anchor(gpui::Anchor::TopLeft)
        .offset(gpui::Point {
            x: px(0.0),
            y: px(2.0),
        })
        .with_handle(ui::PopoverMenuHandle::default())
        .into_any_element()
}
