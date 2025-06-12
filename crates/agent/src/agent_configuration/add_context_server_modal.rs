use std::sync::Arc;

use collections::HashMap;
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, TextStyle, WeakEntity, prelude::*,
};
use language::{Language, LanguageRegistry};
use project::project_settings::ContextServerConfiguration;
use settings::Settings as _;
use theme::ThemeSettings;
use ui::{KeyBinding, Modal, ModalFooter, ModalHeader, Section, prelude::*};
use workspace::{ModalView, Workspace};

use crate::AddContextServer;

pub struct AddContextServerModal {
    workspace: WeakEntity<Workspace>,
    editor: Entity<Editor>,
}

impl AddContextServerModal {
    pub fn register(
        workspace: &mut Workspace,
        language_registry: Arc<LanguageRegistry>,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action({
            let language_registry = language_registry.clone();
            move |_workspace, _: &AddContextServer, window, cx| {
                cx.spawn_in(window, {
                    let language_registry = language_registry.clone();
                    async move |this, cx| {
                        let jsonc_language =
                            language_registry.language_for_name("jsonc").await.ok();
                        let workspace_handle = this.clone();
                        this.update_in(cx, |workspace, window, cx| {
                            workspace.toggle_modal(window, cx, |window, cx| {
                                Self::new(workspace_handle, jsonc_language, window, cx)
                            })
                        })
                    }
                })
                .detach()
            }
        });
    }

    pub fn new(
        workspace: WeakEntity<Workspace>,
        jsonc_language: Option<Arc<Language>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let json = serde_json::json!({
            "your-mcp-server": {
                "command": {
                    "path": "",
                    "args": [],
                    "env": {}
                },
                "settings": {}
            }
        });

        let editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(16, window, cx);
            editor.set_text(serde_json::to_string_pretty(&json).unwrap(), window, cx);
            editor.set_show_gutter(false, cx);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
            if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                buffer.update(cx, |buffer, cx| buffer.set_language(jsonc_language, cx))
            }
            editor
        });

        Self { editor, workspace }
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut Context<Self>) {
        // let mut command_parts = command.split(' ').map(|part| part.trim().to_string());
        // let Some(path) = command_parts.next() else {
        //     return;
        // };
        // let args = command_parts.collect::<Vec<_>>();

        // if let Some(workspace) = self.workspace.upgrade() {
        //     workspace.update(cx, |workspace, cx| {
        //         let fs = workspace.app_state().fs.clone();
        //         update_settings_file::<ProjectSettings>(fs.clone(), cx, |settings, _| {
        //             settings.context_servers.insert(
        //                 name.into(),
        //                 ContextServerConfiguration {
        //                     command: Some(ContextServerCommand {
        //                         path,
        //                         args,
        //                         env: None,
        //                     }),
        //                     settings: Some(json!({})),
        //                 },
        //             );
        //         });
        //     });
        // }

        cx.emit(DismissEvent);
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl ModalView for AddContextServerModal {}

impl Focusable for AddContextServerModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx).clone()
    }
}

impl EventEmitter<DismissEvent> for AddContextServerModal {}

impl Render for AddContextServerModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const MODAL_DESCRIPTION: &'static str = "Visit the MCP server configuration docs to find all necessary arguments and environment variables.";

        let focus_handle = self.focus_handle(cx);

        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("AddContextServerModal")
            .on_action(
                cx.listener(|this, _: &menu::Cancel, _window, cx| this.cancel(&menu::Cancel, cx)),
            )
            .on_action(
                cx.listener(|this, _: &menu::Confirm, _window, cx| {
                    this.confirm(&menu::Confirm, cx)
                }),
            )
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window);
            }))
            .child(
                Modal::new("add-context-server", None)
                    .header(ModalHeader::new().headline("Add MCP Server"))
                    .section(
                        Section::new()
                            .child(Label::new(MODAL_DESCRIPTION).color(Color::Muted))
                            .child(
                                div()
                                    .p_2()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(cx.theme().colors().border_variant)
                                    .bg(cx.theme().colors().editor_background)
                                    .child({
                                        let settings = ThemeSettings::get_global(cx);
                                        let text_style = TextStyle {
                                            color: cx.theme().colors().text,
                                            font_family: settings.buffer_font.family.clone(),
                                            font_fallbacks: settings.buffer_font.fallbacks.clone(),
                                            font_size: settings.buffer_font_size(cx).into(),
                                            font_weight: settings.buffer_font.weight,
                                            line_height: relative(
                                                settings.buffer_line_height.value(),
                                            ),
                                            ..Default::default()
                                        };
                                        EditorElement::new(
                                            &self.editor,
                                            EditorStyle {
                                                background: cx.theme().colors().editor_background,
                                                local_player: cx.theme().players().local(),
                                                text: text_style,
                                                syntax: cx.theme().syntax().clone(),
                                                ..Default::default()
                                            },
                                        )
                                    }),
                            ),
                    )
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_2()
                                .child(
                                    Button::new("cancel", "Cancel")
                                        .key_binding(
                                            KeyBinding::for_action_in(
                                                &menu::Cancel,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                            .map(|kb| kb.size(rems_from_px(12.))),
                                        )
                                        .on_click(cx.listener(|this, _event, _window, cx| {
                                            this.cancel(&menu::Cancel, cx)
                                        })),
                                )
                                .child(
                                    Button::new("add-server", "Add Server")
                                        .key_binding(
                                            KeyBinding::for_action_in(
                                                &menu::Confirm,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                            .map(|kb| kb.size(rems_from_px(12.))),
                                        )
                                        .on_click(cx.listener(|this, _event, _window, cx| {
                                            this.confirm(&menu::Confirm, cx)
                                        })),
                                ),
                        ),
                    ),
            )
    }
}
