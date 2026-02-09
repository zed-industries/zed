use collections::HashMap;
use gpui::{AnyElement, App, Window};
use runtimelib::{MimeBundle, MimeType};
use theme::ActiveTheme;
use ui::prelude::*;
use ui::ProgressBar;

struct WidgetModel {
    state: serde_json::Map<String, serde_json::Value>,
    model_name: String,
}

pub struct WidgetStore {
    models: HashMap<String, WidgetModel>,
}

impl WidgetStore {
    pub fn new() -> Self {
        Self {
            models: HashMap::default(),
        }
    }

    pub fn create_model(&mut self, comm_id: &str, data: &serde_json::Map<String, serde_json::Value>) {
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

        self.models.insert(
            comm_id.to_string(),
            WidgetModel { state, model_name },
        );
    }

    pub fn update_model(&mut self, comm_id: &str, data: &serde_json::Map<String, serde_json::Value>) {
        let method = data.get("method").and_then(|v| v.as_str());
        if method != Some("update") {
            return;
        }

        let state_patch = match data.get("state").and_then(|v| v.as_object()) {
            Some(patch) => patch,
            None => return,
        };

        if let Some(model) = self.models.get_mut(comm_id) {
            for (key, value) in state_patch {
                model.state.insert(key.clone(), value.clone());
            }
        }
    }

    pub fn render_widget(&self, model_id: &str, window: &mut Window, cx: &App) -> AnyElement {
        let model = match self.models.get(model_id) {
            Some(m) => m,
            None => return div().into_any_element(),
        };

        match model.model_name.as_str() {
            "FloatProgressModel" | "IntProgressModel" => render_progress(model, window, cx),
            "LabelModel" => render_label(model),
            "HTMLModel" => render_html_as_text(model),
            "HBoxModel" => render_hbox(self, model, window, cx),
            "VBoxModel" | "BoxModel" => render_vbox(self, model, window, cx),
            // Layout and style models are metadata-only, no visual rendering
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

fn render_hbox(store: &WidgetStore, model: &WidgetModel, window: &mut Window, cx: &App) -> AnyElement {
    let children = model.state.get("children").and_then(|v| v.as_array());

    let mut flex = h_flex().gap_1().items_center();

    if let Some(children) = children {
        for child_ref in children {
            if let Some(child_id) = child_ref
                .as_str()
                .and_then(|s| s.strip_prefix(IPY_MODEL_PREFIX))
            {
                flex = flex.child(store.render_widget(child_id, window, cx));
            }
        }
    }

    flex.into_any_element()
}

fn render_vbox(store: &WidgetStore, model: &WidgetModel, window: &mut Window, cx: &App) -> AnyElement {
    let children = model.state.get("children").and_then(|v| v.as_array());

    let mut flex = v_flex().gap_1();

    if let Some(children) = children {
        for child_ref in children {
            if let Some(child_id) = child_ref
                .as_str()
                .and_then(|s| s.strip_prefix(IPY_MODEL_PREFIX))
            {
                flex = flex.child(store.render_widget(child_id, window, cx));
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
