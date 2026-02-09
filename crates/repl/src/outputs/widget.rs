use collections::HashMap;
use gpui::{AnyElement, App, Context, Entity, EventEmitter, Window};
use runtimelib::{CommId, CommMsg, JupyterMessage, MimeBundle, MimeType};
use theme::ActiveTheme;
use ui::prelude::*;
use ui::{Checkbox, ProgressBar, ToggleState};

struct WidgetModel {
    comm_id: String,
    state: serde_json::Map<String, serde_json::Value>,
    model_name: String,
}

pub struct WidgetStore {
    models: HashMap<String, WidgetModel>,
}

pub(crate) struct WidgetCommMessage(pub JupyterMessage);

impl EventEmitter<WidgetCommMessage> for WidgetStore {}

impl WidgetStore {
    pub fn new() -> Self {
        Self {
            models: HashMap::default(),
        }
    }

    pub fn create_model(
        &mut self,
        comm_id: &str,
        data: &serde_json::Map<String, serde_json::Value>,
    ) {
        let state = data
            .get("state")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        let model_name = state
            .get("_model_name")
            .and_then(|v| v.as_str())
            .unwrap_or("UnknownModel")
            .to_string();

        let state_keys: Vec<&String> = state.keys().collect();
        log::info!(
            "widget create_model: comm_id={}, model_name={}, state_keys={:?}, data_keys={:?}",
            comm_id,
            model_name,
            state_keys,
            data.keys().collect::<Vec<_>>()
        );

        self.models.insert(
            comm_id.to_string(),
            WidgetModel {
                comm_id: comm_id.to_string(),
                state,
                model_name,
            },
        );
    }

    pub fn close_model(&mut self, comm_id: &str) {
        self.models.remove(comm_id);
    }

    pub fn update_model(
        &mut self,
        comm_id: &str,
        data: &serde_json::Map<String, serde_json::Value>,
    ) {
        let method = data.get("method").and_then(|v| v.as_str());
        log::info!(
            "widget update_model: comm_id={}, method={:?}, data_keys={:?}",
            comm_id,
            method,
            data.keys().collect::<Vec<_>>()
        );

        if method != Some("update") {
            return;
        }

        let state_patch = match data.get("state").and_then(|v| v.as_object()) {
            Some(patch) => patch,
            None => return,
        };

        let patch_keys: Vec<&String> = state_patch.keys().collect();
        let found = self.models.contains_key(comm_id);
        log::info!(
            "widget update_model: comm_id={}, found_in_store={}, patch_keys={:?}",
            comm_id,
            found,
            patch_keys
        );

        if let Some(model) = self.models.get_mut(comm_id) {
            for (key, value) in state_patch {
                model.state.insert(key.clone(), value.clone());
            }
        }
    }

    pub fn send_update(
        &mut self,
        model_id: &str,
        state_patch: serde_json::Map<String, serde_json::Value>,
        cx: &mut Context<Self>,
    ) {
        if let Some(model) = self.models.get_mut(model_id) {
            for (key, value) in &state_patch {
                model.state.insert(key.clone(), value.clone());
            }

            let mut data = serde_json::Map::new();
            data.insert("method".into(), "update".into());
            data.insert("state".into(), serde_json::Value::Object(state_patch));

            let message: JupyterMessage = CommMsg {
                comm_id: CommId(model.comm_id.clone()),
                data,
            }
            .into();
            cx.emit(WidgetCommMessage(message));
        }
        cx.notify();
    }

    pub fn send_custom(
        &mut self,
        model_id: &str,
        content: serde_json::Map<String, serde_json::Value>,
        cx: &mut Context<Self>,
    ) {
        if let Some(model) = self.models.get(model_id) {
            let mut data = serde_json::Map::new();
            data.insert("method".into(), "custom".into());
            data.insert("content".into(), serde_json::Value::Object(content));

            let message: JupyterMessage = CommMsg {
                comm_id: CommId(model.comm_id.clone()),
                data,
            }
            .into();
            cx.emit(WidgetCommMessage(message));
        }
    }

    pub fn render_widget(
        store: &Entity<Self>,
        model_id: &str,
        window: &mut Window,
        cx: &App,
    ) -> AnyElement {
        let store_ref = store.read(cx);
        let model = match store_ref.models.get(model_id) {
            Some(m) => m,
            None => {
                log::warn!("widget render_widget: model_id={} NOT FOUND in store (store has {} models: {:?})",
                    model_id,
                    store_ref.models.len(),
                    store_ref.models.keys().collect::<Vec<_>>()
                );
                return div().into_any_element();
            }
        };

        log::info!(
            "widget render_widget: model_id={}, model_name={}, state_keys={:?}",
            model_id,
            model.model_name,
            model.state.keys().collect::<Vec<_>>()
        );

        match model.model_name.as_str() {
            "FloatProgressModel" | "IntProgressModel" => render_progress(model, window, cx),
            "LabelModel" => render_label(model),
            "HTMLModel" => render_html_as_text(model),
            "HBoxModel" => render_hbox(store, model, window, cx),
            "VBoxModel" | "BoxModel" => render_vbox(store, model, window, cx),
            "ButtonModel" => render_button(store, model),
            "CheckboxModel" => render_checkbox(store, model),
            "DropdownModel" => render_dropdown(store, model),
            "IntSliderModel" | "FloatSliderModel" => render_slider(model),
            "TextModel" | "TextareaModel" => render_text(model),
            "OutputModel" => render_output_widget(store, model, window, cx),
            "LayoutModel" | "ProgressStyleModel" | "ButtonStyleModel"
            | "SliderStyleModel" | "DescriptionStyleModel" | "StyleModel" => {
                div().into_any_element()
            }
            _ => render_unsupported(model),
        }
    }
}

fn render_unsupported(model: &WidgetModel) -> AnyElement {
    Label::new(format!("Unsupported widget: {}", model.model_name))
        .size(LabelSize::Small)
        .color(Color::Muted)
        .into_any_element()
}

fn render_progress(model: &WidgetModel, _window: &mut Window, cx: &App) -> AnyElement {
    let value = model
        .state
        .get("value")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as f32;
    let min = model
        .state
        .get("min")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as f32;
    let max = model
        .state
        .get("max")
        .and_then(|v| v.as_f64())
        .unwrap_or(100.0) as f32;
    let description = model
        .state
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let bar_style = model
        .state
        .get("bar_style")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let mut progress_bar = ProgressBar::new("widget-progress", value - min, max - min, cx);

    match bar_style {
        "success" => {
            progress_bar = progress_bar.fg_color(cx.theme().status().success);
        }
        "info" => {
            progress_bar = progress_bar.fg_color(cx.theme().status().info);
        }
        "warning" => {
            progress_bar = progress_bar.fg_color(cx.theme().status().warning);
        }
        "danger" => {
            progress_bar = progress_bar.fg_color(cx.theme().status().error);
        }
        _ => {}
    }

    h_flex()
        .gap_2()
        .items_center()
        .flex_1()
        .when(!description.is_empty(), |el| {
            el.child(
                Label::new(description.to_string())
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
        })
        .child(div().flex_1().child(progress_bar))
        .into_any_element()
}

fn render_label(model: &WidgetModel) -> AnyElement {
    let value = model
        .state
        .get("value")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if value.is_empty() {
        return div().into_any_element();
    }

    Label::new(value.to_string())
        .size(LabelSize::Small)
        .color(Color::Muted)
        .into_any_element()
}

fn render_html_as_text(model: &WidgetModel) -> AnyElement {
    let value = model
        .state
        .get("value")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if value.is_empty() {
        return div().into_any_element();
    }

    let text = strip_html_tags(value);
    if text.is_empty() {
        return div().into_any_element();
    }

    Label::new(text)
        .size(LabelSize::Small)
        .color(Color::Muted)
        .into_any_element()
}

fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut inside_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            _ if !inside_tag => result.push(ch),
            _ => {}
        }
    }

    decode_html_entities(&result)
}

fn decode_html_entities(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

const IPY_MODEL_PREFIX: &str = "IPY_MODEL_";

fn render_hbox(
    store: &Entity<WidgetStore>,
    model: &WidgetModel,
    window: &mut Window,
    cx: &App,
) -> AnyElement {
    let children = model.state.get("children").and_then(|v| v.as_array());

    log::info!(
        "widget render_hbox: comm_id={}, has_children={}, children_raw={:?}",
        model.comm_id,
        children.is_some(),
        model.state.get("children")
    );

    let mut flex = h_flex().gap_1().items_center();

    if let Some(children) = children {
        for child_ref in children {
            if let Some(child_id) = child_ref
                .as_str()
                .and_then(|s| s.strip_prefix(IPY_MODEL_PREFIX))
            {
                log::info!("widget render_hbox: rendering child_id={}", child_id);
                flex = flex.child(WidgetStore::render_widget(store, child_id, window, cx));
            } else {
                log::warn!("widget render_hbox: child_ref not IPY_MODEL_: {:?}", child_ref);
            }
        }
    }

    flex.into_any_element()
}

fn render_vbox(
    store: &Entity<WidgetStore>,
    model: &WidgetModel,
    window: &mut Window,
    cx: &App,
) -> AnyElement {
    let children = model.state.get("children").and_then(|v| v.as_array());

    let mut flex = v_flex().gap_1();

    if let Some(children) = children {
        for child_ref in children {
            if let Some(child_id) = child_ref
                .as_str()
                .and_then(|s| s.strip_prefix(IPY_MODEL_PREFIX))
            {
                flex = flex.child(WidgetStore::render_widget(store, child_id, window, cx));
            }
        }
    }

    flex.into_any_element()
}

fn render_button(store: &Entity<WidgetStore>, model: &WidgetModel) -> AnyElement {
    log::info!(
        "widget render_button: comm_id={}, state={:?}",
        model.comm_id,
        model.state
    );
    let description = model
        .state
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("Button")
        .to_string();
    let disabled = model
        .state
        .get("disabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let model_id = model.comm_id.clone();
    let store = store.clone();

    Button::new(
        SharedString::from(format!("widget-btn-{}", model_id)),
        description,
    )
    .disabled(disabled)
    .on_click(move |_, _window, cx| {
        let model_id = model_id.clone();
        store.update(cx, |widget_store, cx| {
            widget_store.send_custom(&model_id, serde_json::Map::new(), cx);
        });
    })
    .into_any_element()
}

fn render_checkbox(store: &Entity<WidgetStore>, model: &WidgetModel) -> AnyElement {
    let value = model
        .state
        .get("value")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let description = model
        .state
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let disabled = model
        .state
        .get("disabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let toggle_state = if value {
        ToggleState::Selected
    } else {
        ToggleState::Unselected
    };

    let model_id = model.comm_id.clone();
    let store = store.clone();

    h_flex()
        .gap_2()
        .items_center()
        .child(
            Checkbox::new(
                SharedString::from(format!("widget-cb-{}", model_id)),
                toggle_state,
            )
            .disabled(disabled)
            .on_click(move |new_state, _window, cx| {
                let new_value = matches!(new_state, ToggleState::Selected);
                let mut state_patch = serde_json::Map::new();
                state_patch.insert("value".into(), serde_json::Value::Bool(new_value));
                let model_id = model_id.clone();
                store.update(cx, |widget_store, cx| {
                    widget_store.send_update(&model_id, state_patch, cx);
                });
            }),
        )
        .when(!description.is_empty(), |el| {
            el.child(
                Label::new(description.to_string())
                    .size(LabelSize::Small)
                    .color(Color::Default),
            )
        })
        .into_any_element()
}

fn render_dropdown(store: &Entity<WidgetStore>, model: &WidgetModel) -> AnyElement {
    let description = model
        .state
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let disabled = model
        .state
        .get("disabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let options: Vec<String> = model
        .state
        .get("_options_labels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let current_index = model
        .state
        .get("index")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

    let current_label = options
        .get(current_index)
        .cloned()
        .unwrap_or_else(|| "—".to_string());

    let next_index = if options.is_empty() {
        0
    } else {
        (current_index + 1) % options.len()
    };

    let model_id = model.comm_id.clone();
    let store = store.clone();

    h_flex()
        .gap_2()
        .items_center()
        .when(!description.is_empty(), |el| {
            el.child(
                Label::new(description.to_string())
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
        })
        .child(
            Button::new(
                SharedString::from(format!("widget-dd-{}", model_id)),
                format!("{} ▾", current_label),
            )
            .disabled(disabled || options.is_empty())
            .on_click(move |_, _window, cx| {
                let mut state_patch = serde_json::Map::new();
                state_patch.insert(
                    "index".into(),
                    serde_json::Value::Number(serde_json::Number::from(next_index as u64)),
                );
                let model_id = model_id.clone();
                store.update(cx, |widget_store, cx| {
                    widget_store.send_update(&model_id, state_patch, cx);
                });
            }),
        )
        .into_any_element()
}

fn render_slider(model: &WidgetModel) -> AnyElement {
    let value = model
        .state
        .get("value")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let min = model
        .state
        .get("min")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let max = model
        .state
        .get("max")
        .and_then(|v| v.as_f64())
        .unwrap_or(100.0);
    let description = model
        .state
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let is_int = model.model_name == "IntSliderModel";
    let value_text = if is_int {
        format!("{}", value as i64)
    } else {
        format!("{:.2}", value)
    };

    h_flex()
        .gap_2()
        .items_center()
        .when(!description.is_empty(), |el| {
            el.child(
                Label::new(description.to_string())
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
        })
        .child(
            Label::new(format!("{} [{}, {}]", value_text, min, max))
                .size(LabelSize::Small)
                .color(Color::Default),
        )
        .into_any_element()
}

fn render_text(model: &WidgetModel) -> AnyElement {
    let value = model
        .state
        .get("value")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let description = model
        .state
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    h_flex()
        .gap_2()
        .items_center()
        .when(!description.is_empty(), |el| {
            el.child(
                Label::new(description.to_string())
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
        })
        .child(
            Label::new(value.to_string())
                .size(LabelSize::Small)
                .color(Color::Default),
        )
        .into_any_element()
}

fn render_output_widget(
    store: &Entity<WidgetStore>,
    model: &WidgetModel,
    window: &mut Window,
    cx: &App,
) -> AnyElement {
    let children = model.state.get("children").and_then(|v| v.as_array());
    let outputs = model.state.get("outputs").and_then(|v| v.as_array());

    let mut flex = v_flex().gap_1();

    if let Some(children) = children {
        for child_ref in children {
            if let Some(child_id) = child_ref
                .as_str()
                .and_then(|s| s.strip_prefix(IPY_MODEL_PREFIX))
            {
                flex = flex.child(WidgetStore::render_widget(store, child_id, window, cx));
            }
        }
    }

    if let Some(outputs) = outputs {
        for output in outputs {
            if let Some(text) = output.get("text").and_then(|v| v.as_str()) {
                if !text.is_empty() {
                    flex = flex.child(
                        Label::new(text.to_string())
                            .size(LabelSize::Small)
                            .color(Color::Default),
                    );
                }
            }
        }
    }

    flex.into_any_element()
}

pub(crate) fn extract_widget_model_id(data: &MimeBundle) -> Option<String> {
    for mime in &data.content {
        if let MimeType::WidgetView(json) = mime {
            return json
                .get("model_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
        }
    }
    None
}
