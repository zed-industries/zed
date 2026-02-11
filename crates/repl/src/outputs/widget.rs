use std::cell::RefCell;
use std::rc::Rc;

use collections::HashMap;
use editor::{Editor, EditorEvent};
use gpui::{AnyElement, App, Context, Corner, Entity, EventEmitter, Subscription, Window};
use html_to_markdown::{TagHandler, convert_html_to_markdown, markdown as html_md};
use markdown::{Markdown, MarkdownElement, MarkdownFont, MarkdownStyle};
use runtimelib::{CommId, CommMsg, JupyterMessage, MimeBundle, MimeType};
use theme::ActiveTheme;
use ui::prelude::*;
use ui::{Checkbox, ContextMenu, PopoverMenu, ProgressBar, ToggleState};

struct WidgetModel {
    comm_id: String,
    state: serde_json::Map<String, serde_json::Value>,
    model_name: String,
}

pub struct WidgetStore {
    models: HashMap<String, WidgetModel>,
    text_editors: HashMap<String, Entity<Editor>>,
    markdown_views: HashMap<String, Entity<Markdown>>,
    _editor_subscriptions: Vec<Subscription>,
}

pub(crate) struct WidgetCommMessage(pub JupyterMessage);

impl EventEmitter<WidgetCommMessage> for WidgetStore {}

impl WidgetStore {
    pub fn new() -> Self {
        Self {
            models: HashMap::default(),
            text_editors: HashMap::default(),
            markdown_views: HashMap::default(),
            _editor_subscriptions: Vec::new(),
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
        self.text_editors.remove(comm_id);
        self.markdown_views.remove(comm_id);
    }

    pub fn create_missing_text_editors(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let needed: Vec<(String, String, bool)> = self
            .models
            .iter()
            .filter(|(_, m)| {
                matches!(
                    m.model_name.as_str(),
                    "TextModel" | "TextareaModel" | "PasswordModel"
                )
            })
            .filter(|(id, _)| !self.text_editors.contains_key(*id))
            .map(|(id, m)| {
                let value = m
                    .state
                    .get("value")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let is_textarea = m.model_name == "TextareaModel";
                (id.clone(), value, is_textarea)
            })
            .collect();

        for (model_id, value, is_textarea) in needed {
            let editor = cx.new(|cx| {
                let mut editor = if is_textarea {
                    Editor::auto_height(3, 10, window, cx)
                } else {
                    Editor::single_line(window, cx)
                };
                if !value.is_empty() {
                    editor.set_text(value, window, cx);
                }
                editor
            });

            let subscription = cx.subscribe(&editor, {
                let model_id = model_id.clone();
                move |this: &mut Self, editor, event: &EditorEvent, cx| {
                    if !matches!(event, EditorEvent::BufferEdited) {
                        return;
                    }
                    let new_text = editor.read(cx).text(cx);
                    let model_value = this
                        .models
                        .get(&model_id)
                        .and_then(|m| m.state.get("value"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if new_text == model_value {
                        return;
                    }
                    let mut state_patch = serde_json::Map::new();
                    state_patch.insert("value".into(), serde_json::Value::String(new_text));
                    this.send_update(&model_id, state_patch, cx);
                }
            });

            self.text_editors.insert(model_id, editor);
            self._editor_subscriptions.push(subscription);
        }
    }

    pub fn create_missing_markdown_views(&mut self, cx: &mut Context<Self>) {
        let needed: Vec<(String, String)> = self
            .models
            .iter()
            .filter(|(_, m)| m.model_name == "HTMLModel")
            .filter(|(id, _)| !self.markdown_views.contains_key(*id))
            .map(|(id, m)| {
                let html = m
                    .state
                    .get("value")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                (id.clone(), html)
            })
            .collect();

        for (model_id, html) in needed {
            let markdown_text = convert_html_to_markdown_string(&html);
            let markdown_entity =
                cx.new(|cx| Markdown::new(markdown_text.into(), None, None, cx));
            self.markdown_views.insert(model_id, markdown_entity);
        }
    }

    pub fn update_model(
        &mut self,
        comm_id: &str,
        data: &serde_json::Map<String, serde_json::Value>,
    ) {
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

    fn get_state(&self, model_id: &str) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.models.get(model_id).map(|m| &m.state)
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
            None => return div().into_any_element(),
        };

        let text_editor = store_ref.text_editors.get(model_id).cloned();
        let markdown_view = store_ref.markdown_views.get(model_id).cloned();

        match model.model_name.as_str() {
            "FloatProgressModel" | "IntProgressModel" => render_progress(model, window, cx),
            "LabelModel" => render_label(model),
            "HTMLModel" => render_html_as_markdown(model, markdown_view, window, cx),
            "HBoxModel" => render_hbox(store, model, window, cx),
            "VBoxModel" | "BoxModel" => render_vbox(store, model, window, cx),
            "ButtonModel" => render_button(store, model),
            "CheckboxModel" => render_checkbox(store, model),
            "DropdownModel" => render_dropdown(store, model),
            "IntSliderModel" | "FloatSliderModel" => render_slider(store, model, window, cx),
            "TextModel" | "TextareaModel" | "PasswordModel" => render_text(model, text_editor, cx),
            "OutputModel" => render_output_widget(store, model, window, cx),
            "LayoutModel"
            | "ProgressStyleModel"
            | "ButtonStyleModel"
            | "SliderStyleModel"
            | "DescriptionStyleModel"
            | "StyleModel" => div().into_any_element(),
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

fn render_html_as_markdown(
    model: &WidgetModel,
    markdown_entity: Option<Entity<Markdown>>,
    window: &Window,
    cx: &App,
) -> AnyElement {
    if let Some(markdown) = markdown_entity {
        let style = MarkdownStyle::themed(MarkdownFont::Editor, window, cx);
        div()
            .w_full()
            .child(MarkdownElement::new(markdown, style))
            .into_any_element()
    } else {
        let value = model
            .state
            .get("value")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if value.is_empty() {
            return div().into_any_element();
        }

        let markdown_text = convert_html_to_markdown_string(value);
        if markdown_text.trim().is_empty() {
            return div().into_any_element();
        }

        div()
            .child(
                Label::new(markdown_text)
                    .size(LabelSize::Small)
                    .color(Color::Default),
            )
            .into_any_element()
    }
}

fn convert_html_to_markdown_string(html: &str) -> String {
    let mut handlers: Vec<TagHandler> = vec![
        Rc::new(RefCell::new(html_md::WebpageChromeRemover)),
        Rc::new(RefCell::new(html_md::ParagraphHandler)),
        Rc::new(RefCell::new(html_md::HeadingHandler)),
        Rc::new(RefCell::new(html_md::ListHandler)),
        Rc::new(RefCell::new(html_md::TableHandler::new())),
        Rc::new(RefCell::new(html_md::StyledTextHandler)),
        Rc::new(RefCell::new(html_md::CodeHandler)),
    ];

    convert_html_to_markdown(html.as_bytes(), &mut handlers)
        .unwrap_or_else(|_| html.to_string())
}

const IPY_MODEL_PREFIX: &str = "IPY_MODEL_";

fn render_hbox(
    store: &Entity<WidgetStore>,
    model: &WidgetModel,
    window: &mut Window,
    cx: &App,
) -> AnyElement {
    let children = model.state.get("children").and_then(|v| v.as_array());

    let mut flex = h_flex().gap_1().items_center();

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

    h_flex()
        .child(
            Button::new(
                SharedString::from(format!("widget-btn-{}", model_id)),
                description,
            )
            .disabled(disabled)
            .on_click(move |_, _window, cx| {
                let model_id = model_id.clone();
                let mut content = serde_json::Map::new();
                content.insert("event".into(), "click".into());
                store.update(cx, |widget_store, cx| {
                    widget_store.send_custom(&model_id, content, cx);
                });
            }),
        )
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

    let options: Vec<(usize, String)> = model
        .state
        .get("_options_labels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .enumerate()
                .filter_map(|(i, v)| v.as_str().map(|s| (i, s.to_string())))
                .collect()
        })
        .unwrap_or_default();

    let current_index = model
        .state
        .get("index")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

    let current_label = options
        .iter()
        .find(|(i, _)| *i == current_index)
        .map(|(_, label)| label.clone())
        .unwrap_or_else(|| "—".to_string());

    let model_id = model.comm_id.clone();
    let store_for_menu = store.clone();
    let model_id_for_menu = model_id.clone();
    let has_options = !options.is_empty();

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
            PopoverMenu::new(SharedString::from(format!("widget-dd-pop-{}", model_id)))
                .menu(move |window, cx| {
                    let fresh_index = store_for_menu
                        .read(cx)
                        .get_state(&model_id_for_menu)
                        .and_then(|s| s.get("index"))
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as usize;

                    Some(ContextMenu::build(window, cx, {
                        let store = store_for_menu.clone();
                        let model_id = model_id_for_menu.clone();
                        let options = options.clone();
                        move |mut menu, _, _| {
                            for (index, label) in &options {
                                let is_selected = *index == fresh_index;
                                let store = store.clone();
                                let model_id = model_id.clone();
                                let new_index = *index;
                                menu = menu.toggleable_entry(
                                    label.clone(),
                                    is_selected,
                                    IconPosition::End,
                                    None,
                                    move |_window, cx| {
                                        let mut state_patch = serde_json::Map::new();
                                        state_patch.insert(
                                            "index".into(),
                                            serde_json::Value::Number(serde_json::Number::from(
                                                new_index as u64,
                                            )),
                                        );
                                        store.update(cx, |widget_store, cx| {
                                            widget_store.send_update(&model_id, state_patch, cx);
                                        });
                                    },
                                );
                            }
                            menu
                        }
                    }))
                })
                .trigger(
                    Button::new(
                        SharedString::from(format!("widget-dd-{}", model_id)),
                        current_label,
                    )
                    .icon(IconName::ChevronUpDown)
                    .icon_position(IconPosition::End)
                    .icon_size(IconSize::XSmall)
                    .icon_color(Color::Muted)
                    .disabled(disabled || !has_options),
                )
                .attach(Corner::BottomLeft),
        )
        .into_any_element()
}

fn render_slider(
    store: &Entity<WidgetStore>,
    model: &WidgetModel,
    _window: &mut Window,
    cx: &App,
) -> AnyElement {
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
    let step = model
        .state
        .get("step")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);
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

    let is_int = model.model_name == "IntSliderModel";
    let value_text = if is_int {
        format!("{}", value as i64)
    } else {
        format!("{:.2}", value)
    };

    let dec_value = (value - step).max(min);
    let inc_value = (value + step).min(max);
    let model_id = model.comm_id.clone();

    let store_dec = store.clone();
    let model_id_dec = model_id.clone();

    let store_inc = store.clone();
    let model_id_inc = model_id.clone();

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
            IconButton::new(
                SharedString::from(format!("slider-dec-{}", model_id)),
                IconName::Dash,
            )
            .size(ButtonSize::Compact)
            .disabled(disabled || value <= min)
            .on_click(move |_, _window, cx| {
                let mut state_patch = serde_json::Map::new();
                if is_int {
                    state_patch.insert("value".into(), serde_json::Value::from(dec_value as i64));
                } else {
                    state_patch.insert(
                        "value".into(),
                        serde_json::Number::from_f64(dec_value)
                            .map(serde_json::Value::Number)
                            .unwrap_or(serde_json::Value::from(dec_value as i64)),
                    );
                }
                store_dec.update(cx, |widget_store, cx| {
                    widget_store.send_update(&model_id_dec, state_patch, cx);
                });
            }),
        )
        .child(
            div()
                .flex_1()
                .child(ProgressBar::new(
                    format!("slider-bar-{}", model_id),
                    (value - min) as f32,
                    (max - min) as f32,
                    cx,
                ))
                .min_w(px(80.0)),
        )
        .child(
            IconButton::new(
                SharedString::from(format!("slider-inc-{}", model_id)),
                IconName::Plus,
            )
            .size(ButtonSize::Compact)
            .disabled(disabled || value >= max)
            .on_click(move |_, _window, cx| {
                let mut state_patch = serde_json::Map::new();
                if is_int {
                    state_patch.insert("value".into(), serde_json::Value::from(inc_value as i64));
                } else {
                    state_patch.insert(
                        "value".into(),
                        serde_json::Number::from_f64(inc_value)
                            .map(serde_json::Value::Number)
                            .unwrap_or(serde_json::Value::from(inc_value as i64)),
                    );
                }
                store_inc.update(cx, |widget_store, cx| {
                    widget_store.send_update(&model_id_inc, state_patch, cx);
                });
            }),
        )
        .child(
            Label::new(value_text)
                .size(LabelSize::Small)
                .color(Color::Default),
        )
        .into_any_element()
}

fn render_text(model: &WidgetModel, editor: Option<Entity<Editor>>, cx: &App) -> AnyElement {
    let description = model
        .state
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let is_textarea = model.model_name == "TextareaModel";
    let theme_colors = cx.theme().colors();

    let content: AnyElement = if let Some(editor) = editor {
        h_flex()
            .py_1()
            .px_2()
            .when(is_textarea, |el| el.min_h(px(80.0)))
            .when(!is_textarea, |el| el.h_8())
            .min_w_64()
            .rounded_md()
            .border_1()
            .border_color(theme_colors.border)
            .bg(theme_colors.editor_background)
            .child(editor)
            .into_any_element()
    } else {
        let value = model
            .state
            .get("value")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        div()
            .px_2()
            .py_1()
            .min_w(px(120.0))
            .rounded_md()
            .border_1()
            .border_color(theme_colors.border)
            .bg(theme_colors.editor_background)
            .when(is_textarea, |el| el.min_h(px(60.0)))
            .child(
                Label::new(value.to_string())
                    .size(LabelSize::Small)
                    .color(Color::Default),
            )
            .into_any_element()
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
        .child(content)
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
