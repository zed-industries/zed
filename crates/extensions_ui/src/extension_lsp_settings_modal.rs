use std::sync::Arc;

use editor::{Editor, EditorElement, EditorStyle};
use extension::ExtensionManifest;
use fs::Fs;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ScrollHandle,
    TextStyle, WeakEntity, Window, prelude::*,
};
use language::LanguageServerName;
use project::{Project, lsp_store::LocalLspAdapterDelegate, project_settings::ProjectSettings};
use settings::{Settings as _, update_settings_file};
use theme_settings::ThemeSettings;
use ui::{
    ContextMenu, DropdownMenu, DropdownStyle, Modal, ModalFooter, ModalHeader, Section, Switch,
    ToggleState, WithScrollbar, prelude::*,
};
use workspace::{ModalView, Workspace};

/// A top-level JSON Schema property we can render in the Configure form.
#[derive(Clone, Debug)]
pub(crate) enum SchemaFieldKind {
    Boolean,
    String,
    Enum(Vec<SharedString>),
}

#[derive(Clone, Debug)]
pub(crate) struct SchemaField {
    pub key: SharedString,
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub kind: SchemaFieldKind,
    pub default: Option<serde_json::Value>,
}

/// Parse top-level `properties` from a JSON Schema object into flat form fields.
pub(crate) fn parse_schema_fields(schema: &serde_json::Value) -> Vec<SchemaField> {
    let Some(properties) = schema.get("properties").and_then(|v| v.as_object()) else {
        return Vec::new();
    };

    let mut fields = Vec::new();
    for (key, prop) in properties {
        let Some(prop_obj) = prop.as_object() else {
            continue;
        };

        let title = prop_obj
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or(key)
            .to_string();
        let description = prop_obj
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| SharedString::from(s.to_string()));
        let default = prop_obj.get("default").cloned();

        let kind = if let Some(enum_vals) = prop_obj.get("enum").and_then(|v| v.as_array()) {
            let options: Vec<SharedString> = enum_vals
                .iter()
                .filter_map(|v| v.as_str().map(|s| SharedString::from(s.to_string())))
                .collect();
            if options.is_empty() {
                continue;
            }
            SchemaFieldKind::Enum(options)
        } else {
            match prop_obj.get("type").and_then(|v| v.as_str()) {
                Some("boolean") => SchemaFieldKind::Boolean,
                Some("string") => SchemaFieldKind::String,
                _ => continue,
            }
        };

        fields.push(SchemaField {
            key: SharedString::from(key.clone()),
            title: SharedString::from(title),
            description,
            kind,
            default,
        });
    }

    fields
}

enum FieldControl {
    Boolean,
    String { editor: Entity<Editor> },
    Enum { options: Vec<SharedString> },
}

struct FormField {
    schema: SchemaField,
    control: FieldControl,
    value: serde_json::Value,
}

struct ServerForm {
    server_id: SharedString,
    fields: Vec<FormField>,
    json_editor: Option<Entity<Editor>>,
    status: SharedString,
}

pub struct ExtensionLspSettingsModal {
    extension_name: SharedString,
    servers: Vec<ServerForm>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    fs: Arc<dyn Fs>,
}

impl ExtensionLspSettingsModal {
    pub fn show(
        manifest: Arc<ExtensionManifest>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if !window.is_window_active() || manifest.language_servers.is_empty() {
            return;
        }

        let Ok(fs) = workspace.update(cx, |workspace, _| workspace.app_state().fs.clone()) else {
            return;
        };
        let Ok(project) = workspace.update(cx, |workspace, _| workspace.project().clone()) else {
            return;
        };

        let server_ids: Vec<SharedString> = manifest
            .language_servers
            .keys()
            .map(|name| SharedString::from(name.0.to_string()))
            .collect();
        let extension_name = SharedString::from(manifest.name.clone());

        let _ = workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                Self::new(extension_name, server_ids, project, fs, window, cx)
            });
        });
    }

    fn new(
        extension_name: SharedString,
        server_ids: Vec<SharedString>,
        project: Entity<Project>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut servers = Vec::new();

        for server_id in &server_ids {
            let current_settings = current_lsp_settings(server_id, cx);
            let pretty =
                serde_json::to_string_pretty(&current_settings).unwrap_or_else(|_| "{}".into());
            let json_editor = cx.new(|cx| {
                let mut editor = Editor::auto_height(3, 12, window, cx);
                editor.set_text(pretty, window, cx);
                editor.set_show_gutter(false, cx);
                editor
            });

            servers.push(ServerForm {
                server_id: server_id.clone(),
                fields: Vec::new(),
                json_editor: Some(json_editor),
                status: SharedString::from("Loading settings schema…"),
            });
        }

        let this = Self {
            extension_name,
            servers,
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
            fs,
        };
        this.spawn_schema_load(project, server_ids, window, cx);
        this
    }

    fn spawn_schema_load(
        &self,
        project: Entity<Project>,
        server_ids: Vec<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.spawn_in(window, async move |this, cx| {
            for server_id in server_ids {
                let schema = fetch_settings_schema(&project, &server_id, cx).await;

                let _ = this.update_in(cx, |this, window, cx| {
                    let Some(server) = this
                        .servers
                        .iter_mut()
                        .find(|s| s.server_id == server_id)
                    else {
                        return;
                    };

                    let current_settings = current_lsp_settings(&server_id, cx);

                    match schema {
                        Some(schema) => {
                            let parsed = parse_schema_fields(&schema);
                            if parsed.is_empty() {
                                server.status = SharedString::from(
                                    "Schema has no simple fields — edit JSON below",
                                );
                            } else {
                                server.fields = parsed
                                    .into_iter()
                                    .map(|field| build_form_field(field, &current_settings, window, cx))
                                    .collect();
                                server.json_editor = None;
                                server.status = SharedString::from("Schema-backed settings");
                            }
                        }
                        None => {
                            server.status = SharedString::from(
                                "No settings schema from language server — edit JSON below",
                            );
                        }
                    }
                    cx.notify();
                });
            }
        })
        .detach();
    }

    fn set_bool(
        &mut self,
        server_id: SharedString,
        key: SharedString,
        value: bool,
        cx: &mut Context<Self>,
    ) {
        if let Some(field) = self.find_field_mut(&server_id, &key) {
            field.value = serde_json::Value::Bool(value);
        }
        self.write_typed_settings(&server_id, cx);
        cx.notify();
    }

    fn set_enum(
        &mut self,
        server_id: SharedString,
        key: SharedString,
        value: SharedString,
        cx: &mut Context<Self>,
    ) {
        if let Some(field) = self.find_field_mut(&server_id, &key) {
            field.value = serde_json::Value::String(value.to_string());
        }
        self.write_typed_settings(&server_id, cx);
        cx.notify();
    }

    fn find_field_mut(&mut self, server_id: &str, key: &str) -> Option<&mut FormField> {
        self.servers
            .iter_mut()
            .find(|s| s.server_id.as_ref() == server_id)?
            .fields
            .iter_mut()
            .find(|f| f.schema.key.as_ref() == key)
    }

    fn write_typed_settings(&self, server_id: &SharedString, cx: &mut Context<Self>) {
        let Some(server) = self.servers.iter().find(|s| &s.server_id == server_id) else {
            return;
        };
        if server.fields.is_empty() {
            return;
        }

        let mut map = serde_json::Map::new();
        for field in &server.fields {
            match &field.control {
                FieldControl::String { editor } => {
                    let text = editor.read(cx).text(cx);
                    map.insert(
                        field.schema.key.to_string(),
                        serde_json::Value::String(text),
                    );
                }
                FieldControl::Boolean | FieldControl::Enum { .. } => {
                    if !field.value.is_null() {
                        map.insert(field.schema.key.to_string(), field.value.clone());
                    }
                }
            }
        }

        let server_key: Arc<str> = server_id.to_string().into();
        let settings_value = serde_json::Value::Object(map);
        update_settings_file(self.fs.clone(), cx, move |content, _| {
            content
                .project
                .lsp
                .0
                .entry(server_key)
                .or_default()
                .settings = Some(settings_value);
        });
    }

    fn save_json_editor(&self, server_id: &SharedString, cx: &mut Context<Self>) {
        let Some(server) = self.servers.iter().find(|s| &s.server_id == server_id) else {
            return;
        };
        let Some(editor) = &server.json_editor else {
            return;
        };
        let text = editor.read(cx).text(cx);
        let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) else {
            log::error!("Invalid LSP settings JSON for {server_id}");
            return;
        };
        let server_key: Arc<str> = server_id.to_string().into();
        update_settings_file(self.fs.clone(), cx, move |content, _| {
            content
                .project
                .lsp
                .0
                .entry(server_key)
                .or_default()
                .settings = Some(parsed);
        });
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        let typed: Vec<_> = self
            .servers
            .iter()
            .filter(|s| !s.fields.is_empty())
            .map(|s| s.server_id.clone())
            .collect();
        for id in typed {
            self.write_typed_settings(&id, cx);
        }
        let json_ids: Vec<_> = self
            .servers
            .iter()
            .filter(|s| s.json_editor.is_some())
            .map(|s| s.server_id.clone())
            .collect();
        for id in json_ids {
            self.save_json_editor(&id, cx);
        }
        cx.emit(DismissEvent);
    }
}

fn current_lsp_settings(server_id: &str, cx: &App) -> serde_json::Value {
    ProjectSettings::get_global(cx)
        .lsp
        .get(&LanguageServerName(server_id.to_string().into()))
        .and_then(|s| s.settings.clone())
        .unwrap_or_else(|| serde_json::json!({}))
}

fn build_form_field(
    field: SchemaField,
    current_settings: &serde_json::Value,
    window: &mut Window,
    cx: &mut App,
) -> FormField {
    let value = current_settings
        .get(field.key.as_ref())
        .cloned()
        .or_else(|| field.default.clone())
        .unwrap_or(serde_json::Value::Null);

    let control = match &field.kind {
        SchemaFieldKind::Boolean => FieldControl::Boolean,
        SchemaFieldKind::Enum(options) => FieldControl::Enum {
            options: options.clone(),
        },
        SchemaFieldKind::String => {
            let text = value.as_str().unwrap_or("").to_string();
            let editor = cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_text(text, window, cx);
                editor.set_show_gutter(false, cx);
                editor
            });
            FieldControl::String { editor }
        }
    };

    FormField {
        schema: field,
        control,
        value,
    }
}

async fn fetch_settings_schema(
    project: &Entity<Project>,
    server_id: &str,
    cx: &mut gpui::AsyncWindowContext,
) -> Option<serde_json::Value> {
    let (adapter, delegate) = cx
        .update(|_, cx| {
            let languages = project.read(cx).languages().clone();
            let name = LanguageServerName(server_id.to_string().into());
            let adapter = languages
                .adapter_for_name(&name)
                .or_else(|| languages.load_available_lsp_adapter(&name))?;

            let lsp_store = project.read(cx).lsp_store();
            let delegate = lsp_store.update(cx, |lsp_store, cx| {
                let local = lsp_store.as_local()?;
                let worktree = local.worktree_store.read(cx).worktrees().next()?;
                Some(
                    LocalLspAdapterDelegate::from_local_lsp(local, &worktree, cx)
                        as Arc<dyn language::LspAdapterDelegate>,
                )
            })?;
            Some((adapter, delegate))
        })
        .ok()
        .flatten()?;

    adapter.settings_schema(&delegate, cx).await
}

impl Focusable for ExtensionLspSettingsModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ExtensionLspSettingsModal {}
impl ModalView for ExtensionLspSettingsModal {}

impl Render for ExtensionLspSettingsModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let modal_entity = cx.entity().downgrade();
        let mut body = v_flex().gap_4().w_full();

        for server in &self.servers {
            let server_id = server.server_id.clone();
            let mut section = v_flex()
                .gap_2()
                .child(
                    Label::new(format!("Language server: {}", server.server_id))
                        .size(LabelSize::Small)
                        .color(Color::Accent),
                )
                .child(
                    Label::new(server.status.clone())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                );

            for field in &server.fields {
                let key = field.schema.key.clone();
                let row_label = v_flex().min_w_0().child(Label::new(field.schema.title.clone()));
                let row_label = if let Some(desc) = field.schema.description.clone() {
                    row_label.child(Label::new(desc).size(LabelSize::Small).color(Color::Muted))
                } else {
                    row_label
                };

                let control: AnyElement = match &field.control {
                    FieldControl::Boolean => {
                        let checked = field.value.as_bool().unwrap_or(false);
                        Switch::new(
                            SharedString::from(format!("bool-{server_id}-{key}")),
                            if checked {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .on_click(cx.listener({
                            let server_id = server_id.clone();
                            let key = key.clone();
                            move |this, state, _window, cx| {
                                this.set_bool(
                                    server_id.clone(),
                                    key.clone(),
                                    matches!(state, ToggleState::Selected),
                                    cx,
                                );
                            }
                        }))
                        .into_any_element()
                    }
                    FieldControl::Enum { options } => {
                        let current = field
                            .value
                            .as_str()
                            .map(SharedString::from)
                            .unwrap_or_else(|| {
                                options.first().cloned().unwrap_or_else(|| "—".into())
                            });
                        let options = options.clone();
                        let menu = ContextMenu::build(window, cx, {
                            let server_id = server_id.clone();
                            let key = key.clone();
                            let current = current.clone();
                            let modal_entity = modal_entity.clone();
                            move |mut menu, _, _cx| {
                                for opt in &options {
                                    let selected = opt == &current;
                                    let opt = opt.clone();
                                    let server_id = server_id.clone();
                                    let key = key.clone();
                                    let modal_entity = modal_entity.clone();
                                    menu = menu.toggleable_entry(
                                        opt.clone(),
                                        selected,
                                        IconPosition::Start,
                                        None,
                                        move |_window, cx| {
                                            let _ = modal_entity.update(cx, |this, cx| {
                                                this.set_enum(
                                                    server_id.clone(),
                                                    key.clone(),
                                                    opt.clone(),
                                                    cx,
                                                );
                                            });
                                        },
                                    );
                                }
                                menu
                            }
                        });
                        DropdownMenu::new(
                            SharedString::from(format!("enum-{server_id}-{key}")),
                            current,
                            menu,
                        )
                        .style(DropdownStyle::Outlined)
                        .into_any_element()
                    }
                    FieldControl::String { editor } => {
                        let settings = ThemeSettings::get_global(cx);
                        let text_style = TextStyle {
                            color: cx.theme().colors().text,
                            font_family: settings.ui_font.family.clone(),
                            font_fallbacks: settings.ui_font.fallbacks.clone(),
                            font_size: settings.ui_font_size(cx).into(),
                            font_weight: settings.ui_font.weight,
                            ..Default::default()
                        };
                        div()
                            .w(rems(14.))
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .border_1()
                            .border_color(cx.theme().colors().border_variant)
                            .bg(cx.theme().colors().editor_background)
                            .child(EditorElement::new(
                                editor,
                                EditorStyle {
                                    background: cx.theme().colors().editor_background,
                                    local_player: cx.theme().players().local(),
                                    text: text_style,
                                    ..Default::default()
                                },
                            ))
                            .into_any_element()
                    }
                };

                section = section.child(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .gap_3()
                        .child(row_label)
                        .child(control),
                );
            }

            if let Some(editor) = &server.json_editor {
                let settings = ThemeSettings::get_global(cx);
                let text_style = TextStyle {
                    color: cx.theme().colors().text,
                    font_family: settings.buffer_font.family.clone(),
                    font_fallbacks: settings.buffer_font.fallbacks.clone(),
                    font_size: settings.buffer_font_size(cx).into(),
                    font_weight: settings.buffer_font.weight,
                    line_height: relative(settings.buffer_line_height.value()),
                    ..Default::default()
                };
                section = section.child(
                    div()
                        .p_2()
                        .rounded_md()
                        .border_1()
                        .border_color(cx.theme().colors().border_variant)
                        .bg(cx.theme().colors().editor_background)
                        .child(EditorElement::new(
                            editor,
                            EditorStyle {
                                background: cx.theme().colors().editor_background,
                                local_player: cx.theme().players().local(),
                                text: text_style,
                                syntax: cx.theme().syntax().clone(),
                                ..Default::default()
                            },
                        )),
                );
            }

            body = body.child(section);
        }

        div()
            .elevation_3(cx)
            .w(rems(42.))
            .key_context("ExtensionLspSettingsModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &menu::Cancel, _window, cx| {
                this.dismiss(cx);
            }))
            .child(
                Modal::new("configure-extension-lsp-settings", None)
                    .header(
                        ModalHeader::new()
                            .headline(format!("Configure {}", self.extension_name)),
                    )
                    .section(Section::new().child(
                        div()
                            .id("extension-lsp-settings-body")
                            .max_h(vh(0.7, window))
                            .overflow_y_scroll()
                            .track_scroll(&self.scroll_handle)
                            .child(
                                Label::new(
                                    "Values are written to lsp.<server>.settings in settings.json.",
                                )
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                            )
                            .child(body)
                            .vertical_scrollbar_for(&self.scroll_handle, window, cx),
                    ))
                    .footer(
                        ModalFooter::new()
                            .start_slot(
                                Button::new("open-settings-json", "Open settings.json")
                                    .style(ButtonStyle::OutlinedGhost)
                                    .on_click(cx.listener(|_this, _, window, cx| {
                                        window.dispatch_action(
                                            Box::new(zed_actions::OpenSettingsFile),
                                            cx,
                                        );
                                    })),
                            )
                            .end_slot(
                                Button::new("done", "Done")
                                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.dismiss(cx);
                                    })),
                            ),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_bool_string_and_enum_fields() {
        let schema = json!({
            "type": "object",
            "properties": {
                "engine": {
                    "type": "string",
                    "enum": ["tree-sitter", "asciidoctor-js", "asciidoctor-d"],
                    "description": "AsciiDoc engine"
                },
                "enabled": { "type": "boolean", "default": true },
                "path": { "type": "string", "title": "Binary Path" },
                "nested": { "type": "object" }
            }
        });
        let fields = parse_schema_fields(&schema);
        assert_eq!(fields.len(), 3);
        assert!(
            fields
                .iter()
                .any(|f| matches!(f.kind, SchemaFieldKind::Enum(ref o) if o.len() == 3))
        );
        assert!(
            fields
                .iter()
                .any(|f| matches!(f.kind, SchemaFieldKind::Boolean))
        );
        assert!(fields.iter().any(
            |f| matches!(f.kind, SchemaFieldKind::String) && f.key.as_ref() == "path"
        ));
        assert!(!fields.iter().any(|f| f.key.as_ref() == "nested"));
    }
}
