use std::{collections::HashMap as StdHashMap, ops::Range, sync::Arc};

use collections::{HashMap, HashSet};
use futures::future::join_all;
use gpui::{MouseButton, SharedString, Task, WeakEntity};
use itertools::Itertools;
use language::BufferId;
use multi_buffer::{Anchor, MultiBufferSnapshot, ToPoint as _};
use project::{CodeAction, LspAction, TaskSourceKind, lsp_store::lsp_ext_command};
use task::TaskContext;
use text::Point;

use ui::{Context, Window, div, prelude::*};

use crate::{
    Editor, LSP_REQUEST_DEBOUNCE_TIMEOUT, MultibufferSelectionMode, SelectionEffects,
    actions::ToggleCodeLens,
    display_map::{BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
};

#[derive(Clone, Debug)]
struct CodeLensLine {
    position: Anchor,
    items: Vec<CodeLensItem>,
}

#[derive(Clone, Debug)]
struct CodeLensItem {
    title: SharedString,
    action: CodeAction,
}

#[derive(Default)]
pub(super) struct CodeLensState {
    pub(super) block_ids: HashMap<BufferId, Vec<CustomBlockId>>,
}

impl CodeLensState {
    fn all_block_ids(&self) -> HashSet<CustomBlockId> {
        self.block_ids.values().flatten().copied().collect()
    }
}

fn group_lenses_by_row(
    lenses: Vec<(Anchor, CodeLensItem)>,
    snapshot: &MultiBufferSnapshot,
) -> Vec<CodeLensLine> {
    let mut grouped: HashMap<u32, (Anchor, Vec<CodeLensItem>)> = HashMap::default();

    for (position, item) in lenses {
        let row = position.to_point(snapshot).row;
        grouped
            .entry(row)
            .or_insert_with(|| (position, Vec::new()))
            .1
            .push(item);
    }

    let mut result: Vec<CodeLensLine> = grouped
        .into_iter()
        .map(|(_, (position, items))| CodeLensLine { position, items })
        .collect();

    result.sort_by_key(|lens| lens.position.to_point(snapshot).row);
    result
}

fn render_code_lens_line(
    lens: CodeLensLine,
    editor: WeakEntity<Editor>,
) -> impl Fn(&mut crate::display_map::BlockContext) -> gpui::AnyElement {
    move |cx| {
        let mut children: Vec<gpui::AnyElement> = Vec::new();
        let text_style = &cx.editor_style.text;
        let font = text_style.font();
        let font_size = text_style.font_size.to_pixels(cx.window.rem_size()) * 0.9;

        for (i, item) in lens.items.iter().enumerate() {
            if i > 0 {
                children.push(
                    div()
                        .font(font.clone())
                        .text_size(font_size)
                        .text_color(cx.app.theme().colors().text_muted)
                        .child(" | ")
                        .into_any_element(),
                );
            }

            let title = item.title.clone();
            let action = item.action.clone();
            let editor_handle = editor.clone();
            let position = lens.position;

            children.push(
                div()
                    .id(SharedString::from(format!(
                        "code-lens-{}-{}-{}",
                        position.text_anchor.offset, i, title
                    )))
                    .font(font.clone())
                    .text_size(font_size)
                    .text_color(cx.app.theme().colors().text_muted)
                    .cursor_pointer()
                    .hover(|style| style.text_color(cx.app.theme().colors().text))
                    .child(title.clone())
                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    .on_mouse_down(MouseButton::Right, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    .on_click({
                        move |_event, window, cx| {
                            if let Some(editor) = editor_handle.upgrade() {
                                editor.update(cx, |editor, cx| {
                                    editor.change_selections(
                                        SelectionEffects::default(),
                                        window,
                                        cx,
                                        |s| {
                                            s.select_anchor_ranges([position..position]);
                                        },
                                    );

                                    let action = action.clone();
                                    if let Some(workspace) = editor.workspace() {
                                        if try_handle_client_command(
                                            &action, editor, &workspace, window, cx,
                                        ) {
                                            return;
                                        }

                                        let project = workspace.read(cx).project().clone();
                                        let buffer = editor.buffer().clone();
                                        if let Some(excerpt_buffer) = buffer.read(cx).as_singleton()
                                        {
                                            project
                                                .update(cx, |project, cx| {
                                                    project.apply_code_action(
                                                        excerpt_buffer.clone(),
                                                        action,
                                                        true,
                                                        cx,
                                                    )
                                                })
                                                .detach_and_log_err(cx);
                                        }
                                    }
                                });
                            }
                        }
                    })
                    .into_any_element(),
            );
        }

        div()
            .pl(cx.margins.gutter.full_width())
            .h_full()
            .flex()
            .flex_row()
            .items_end()
            .children(children)
            .into_any_element()
    }
}

fn try_handle_client_command(
    action: &CodeAction,
    editor: &mut Editor,
    workspace: &gpui::Entity<workspace::Workspace>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    let command = match &action.lsp_action {
        LspAction::CodeLens(lens) => lens.command.as_ref(),
        _ => None,
    };
    let Some(command) = command else {
        return false;
    };
    let arguments = command.arguments.as_deref().unwrap_or_default();

    match command.command.as_str() {
        "rust-analyzer.runSingle" | "rust-analyzer.debugSingle" => {
            try_schedule_runnable(arguments, action, editor, workspace, window, cx)
        }
        "rust-analyzer.showReferences" => {
            try_show_references(arguments, action, editor, workspace, window, cx)
        }
        _ => false,
    }
}

fn try_schedule_runnable(
    arguments: &[serde_json::Value],
    action: &CodeAction,
    editor: &Editor,
    workspace: &gpui::Entity<workspace::Workspace>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    let Some(first_arg) = arguments.first() else {
        return false;
    };
    let Ok(runnable) = serde_json::from_value::<lsp_ext_command::Runnable>(first_arg.clone())
    else {
        return false;
    };

    let task_template = lsp_ext_command::runnable_to_task_template(runnable.label, runnable.args);
    let task_context = TaskContext {
        cwd: task_template.cwd.as_ref().map(std::path::PathBuf::from),
        ..TaskContext::default()
    };
    let language_name = editor
        .buffer()
        .read(cx)
        .as_singleton()
        .and_then(|buffer| buffer.read(cx).language())
        .map(|language| language.name());
    let task_source_kind = match language_name {
        Some(language_name) => TaskSourceKind::Lsp {
            server: action.server_id,
            language_name: SharedString::from(language_name),
        },
        None => TaskSourceKind::AbsPath {
            id_base: "code-lens".into(),
            abs_path: task_template
                .cwd
                .as_ref()
                .map(std::path::PathBuf::from)
                .unwrap_or_default(),
        },
    };

    workspace.update(cx, |workspace, cx| {
        workspace.schedule_task(
            task_source_kind,
            &task_template,
            &task_context,
            false,
            window,
            cx,
        );
    });
    true
}

fn try_show_references(
    arguments: &[serde_json::Value],
    action: &CodeAction,
    _editor: &mut Editor,
    workspace: &gpui::Entity<workspace::Workspace>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    if arguments.len() < 3 {
        return false;
    }
    let Ok(locations) = serde_json::from_value::<Vec<lsp::Location>>(arguments[2].clone()) else {
        return false;
    };
    if locations.is_empty() {
        return false;
    }

    let server_id = action.server_id;
    let project = workspace.read(cx).project().clone();
    let workspace = workspace.clone();

    cx.spawn_in(window, async move |_editor, cx| {
        let mut buffer_locations: StdHashMap<gpui::Entity<language::Buffer>, Vec<Range<Point>>> =
            StdHashMap::default();

        for location in &locations {
            let open_task = cx.update(|_, cx| {
                project.update(cx, |project, cx| {
                    let uri: lsp::Uri = location.uri.clone();
                    project.open_local_buffer_via_lsp(uri, server_id, cx)
                })
            })?;
            let buffer = open_task.await?;

            let range = range_from_lsp(location.range);
            buffer_locations.entry(buffer).or_default().push(range);
        }

        workspace.update_in(cx, |workspace, window, cx| {
            Editor::open_locations_in_multibuffer(
                workspace,
                buffer_locations,
                "References".to_owned(),
                false,
                true,
                MultibufferSelectionMode::First,
                window,
                cx,
            );
        })?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);

    true
}

fn range_from_lsp(range: lsp::Range) -> Range<Point> {
    let start = Point::new(range.start.line, range.start.character);
    let end = Point::new(range.end.line, range.end.character);
    start..end
}

impl Editor {
    pub(super) fn refresh_code_lenses(
        &mut self,
        for_buffer: Option<BufferId>,
        _window: &Window,
        cx: &mut Context<Self>,
    ) {
        if !self.mode().is_full() {
            return;
        }
        if self.code_lens.is_none() {
            return;
        }
        let Some(project) = self.project.clone() else {
            return;
        };

        let buffers_to_query = self
            .visible_excerpts(true, cx)
            .into_values()
            .map(|(buffer, ..)| buffer)
            .chain(for_buffer.and_then(|id| self.buffer.read(cx).buffer(id)))
            .filter(|buffer| {
                let id = buffer.read(cx).remote_id();
                for_buffer.is_none_or(|target| target == id)
                    && self.registered_buffers.contains_key(&id)
            })
            .unique_by(|buffer| buffer.read(cx).remote_id())
            .collect::<Vec<_>>();

        if buffers_to_query.is_empty() {
            return;
        }

        let project = project.downgrade();
        self.refresh_code_lens_task = cx.spawn(async move |editor, cx| {
            cx.background_executor()
                .timer(LSP_REQUEST_DEBOUNCE_TIMEOUT)
                .await;

            let Some(tasks) = project
                .update(cx, |project, cx| {
                    project.lsp_store().update(cx, |lsp_store, cx| {
                        buffers_to_query
                            .into_iter()
                            .map(|buffer| {
                                let buffer_id = buffer.read(cx).remote_id();
                                let task = lsp_store.code_lens_actions(&buffer, cx);
                                async move { (buffer_id, task.await) }
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .ok()
            else {
                return;
            };

            let results = join_all(tasks).await;
            if results.is_empty() {
                return;
            }

            let Ok(multi_buffer_snapshot) =
                editor.update(cx, |editor, cx| editor.buffer().read(cx).snapshot(cx))
            else {
                return;
            };

            let mut new_lenses_per_buffer: HashMap<BufferId, Vec<CodeLensLine>> =
                HashMap::default();

            for (buffer_id, result) in results {
                match result {
                    Ok(Some(actions)) => {
                        let individual_lenses: Vec<(Anchor, CodeLensItem)> = actions
                            .into_iter()
                            .filter_map(|action| {
                                let title = match &action.lsp_action {
                                    project::LspAction::CodeLens(lens) => {
                                        lens.command.as_ref().map(|cmd| cmd.title.clone())
                                    }
                                    _ => None,
                                }?;

                                let position = multi_buffer_snapshot.anchor_in_excerpt(
                                    multi_buffer_snapshot.excerpts().next()?.0,
                                    action.range.start,
                                )?;

                                Some((
                                    position,
                                    CodeLensItem {
                                        title: title.into(),
                                        action,
                                    },
                                ))
                            })
                            .collect();

                        let grouped =
                            group_lenses_by_row(individual_lenses, &multi_buffer_snapshot);
                        new_lenses_per_buffer.insert(buffer_id, grouped);
                    }
                    Ok(None) => {}
                    Err(e) => {
                        log::error!("Failed to fetch code lenses for buffer {buffer_id:?}: {e:#}");
                    }
                }
            }

            editor
                .update(cx, |editor, cx| {
                    let code_lens = editor.code_lens.get_or_insert_with(CodeLensState::default);

                    let mut blocks_to_remove: HashSet<CustomBlockId> = HashSet::default();
                    for (buffer_id, _) in &new_lenses_per_buffer {
                        if let Some(old_ids) = code_lens.block_ids.remove(buffer_id) {
                            blocks_to_remove.extend(old_ids);
                        }
                    }

                    if !blocks_to_remove.is_empty() {
                        editor.remove_blocks(blocks_to_remove, None, cx);
                    }

                    let editor_handle = cx.entity().downgrade();

                    let mut all_new_blocks: Vec<(BufferId, Vec<BlockProperties<Anchor>>)> =
                        Vec::new();
                    for (buffer_id, lenses) in new_lenses_per_buffer {
                        if lenses.is_empty() {
                            continue;
                        }
                        let blocks: Vec<BlockProperties<Anchor>> = lenses
                            .into_iter()
                            .map(|lens| {
                                let position = lens.position;
                                let render_fn = render_code_lens_line(lens, editor_handle.clone());
                                BlockProperties {
                                    placement: BlockPlacement::Above(position),
                                    height: Some(1),
                                    style: BlockStyle::Flex,
                                    render: Arc::new(render_fn),
                                    priority: 0,
                                }
                            })
                            .collect();
                        all_new_blocks.push((buffer_id, blocks));
                    }

                    for (buffer_id, blocks) in all_new_blocks {
                        let block_ids = editor.insert_blocks(blocks, None, cx);
                        editor
                            .code_lens
                            .get_or_insert_with(CodeLensState::default)
                            .block_ids
                            .insert(buffer_id, block_ids);
                    }

                    cx.notify();
                })
                .ok();
        });
    }

    pub fn supports_code_lens(&self, cx: &ui::App) -> bool {
        let Some(project) = self.project.as_ref() else {
            return false;
        };
        let lsp_store = project.read(cx).lsp_store().read(cx);
        lsp_store
            .lsp_server_capabilities
            .values()
            .any(|caps| caps.code_lens_provider.is_some())
    }

    pub fn code_lens_enabled(&self) -> bool {
        self.code_lens.is_some()
    }

    pub fn toggle_code_lens_action(
        &mut self,
        _: &ToggleCodeLens,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let currently_enabled = self.code_lens.is_some();
        self.toggle_code_lens(!currently_enabled, window, cx);
    }

    pub(super) fn toggle_code_lens(
        &mut self,
        enabled: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if enabled {
            self.code_lens.get_or_insert_with(CodeLensState::default);
            self.refresh_code_lenses(None, window, cx);
        } else {
            self.clear_code_lenses(cx);
        }
    }

    pub(super) fn clear_code_lenses(&mut self, cx: &mut Context<Self>) {
        if let Some(code_lens) = self.code_lens.take() {
            let all_blocks = code_lens.all_block_ids();
            if !all_blocks.is_empty() {
                self.remove_blocks(all_blocks, None, cx);
            }
            cx.notify();
        }
        self.refresh_code_lens_task = Task::ready(());
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use gpui::TestAppContext;

    use settings::CodeLens;

    use crate::{
        editor_tests::{init_test, update_test_editor_settings},
        test::editor_lsp_test_context::EditorLspTestContext,
    };

    #[gpui::test]
    async fn test_code_lens_blocks(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: None,
                }),
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["lens_cmd".to_string()],
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                Ok(Some(vec![
                    lsp::CodeLens {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                        command: Some(lsp::Command {
                            title: "2 references".to_owned(),
                            command: "lens_cmd".to_owned(),
                            arguments: None,
                        }),
                        data: None,
                    },
                    lsp::CodeLens {
                        range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 19)),
                        command: Some(lsp::Command {
                            title: "0 references".to_owned(),
                            command: "lens_cmd".to_owned(),
                            arguments: None,
                        }),
                        data: None,
                    },
                ]))
            });

        cx.set_state("ˇfunction hello() {}\nfunction world() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received a code lens request"
        );
        cx.run_until_parked();

        cx.editor.read_with(&cx.cx.cx, |editor, _cx| {
            assert_eq!(
                editor.code_lens_enabled(),
                true,
                "code lens should be enabled"
            );
            let total_blocks: usize = editor
                .code_lens
                .as_ref()
                .map(|s| s.block_ids.values().map(|v| v.len()).sum())
                .unwrap_or(0);
            assert_eq!(total_blocks, 2, "Should have inserted two code lens blocks");
        });
    }

    #[gpui::test]
    async fn test_code_lens_disabled_by_default(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: None,
                }),
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["lens_cmd".to_string()],
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        cx.lsp
            .set_request_handler::<lsp::request::CodeLensRequest, _, _>(|_, _| async move {
                panic!("Should not request code lenses when disabled");
            });

        cx.set_state("ˇfunction hello() {}");
        cx.run_until_parked();

        cx.editor.read_with(&cx.cx.cx, |editor, _cx| {
            assert_eq!(
                editor.code_lens_enabled(),
                false,
                "code lens should not be enabled when setting is off"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_toggling(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: None,
                }),
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["lens_cmd".to_string()],
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                Ok(Some(vec![lsp::CodeLens {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                    command: Some(lsp::Command {
                        title: "1 reference".to_owned(),
                        command: "lens_cmd".to_owned(),
                        arguments: None,
                    }),
                    data: None,
                }]))
            });

        cx.set_state("ˇfunction hello() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received a code lens request"
        );
        cx.run_until_parked();

        cx.editor.read_with(&cx.cx.cx, |editor, _cx| {
            assert_eq!(
                editor.code_lens_enabled(),
                true,
                "code lens should be enabled"
            );
            let total_blocks: usize = editor
                .code_lens
                .as_ref()
                .map(|s| s.block_ids.values().map(|v| v.len()).sum())
                .unwrap_or(0);
            assert_eq!(total_blocks, 1, "Should have one code lens block");
        });

        cx.update_editor(|editor, _window, cx| {
            editor.clear_code_lenses(cx);
        });

        cx.editor.read_with(&cx.cx.cx, |editor, _cx| {
            assert_eq!(
                editor.code_lens_enabled(),
                false,
                "code lens should be disabled after clearing"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_resolve(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let mut cx = EditorLspTestContext::new_typescript(
            lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                Ok(Some(vec![
                    lsp::CodeLens {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                        command: None,
                        data: Some(serde_json::json!({"id": "lens_1"})),
                    },
                    lsp::CodeLens {
                        range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 19)),
                        command: None,
                        data: Some(serde_json::json!({"id": "lens_2"})),
                    },
                ]))
            });

        cx.lsp
            .set_request_handler::<lsp::request::CodeLensResolve, _, _>(|lens, _| async move {
                let id = lens
                    .data
                    .as_ref()
                    .and_then(|d| d.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let title = match id {
                    "lens_1" => "3 references",
                    "lens_2" => "1 implementation",
                    _ => "unknown",
                };
                Ok(lsp::CodeLens {
                    command: Some(lsp::Command {
                        title: title.to_owned(),
                        command: format!("resolved_{id}"),
                        arguments: None,
                    }),
                    ..lens
                })
            });

        cx.set_state("ˇfunction hello() {}\nfunction world() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received a code lens request"
        );
        cx.run_until_parked();

        cx.editor.read_with(&cx.cx.cx, |editor, _cx| {
            let total_blocks: usize = editor
                .code_lens
                .as_ref()
                .map(|s| s.block_ids.values().map(|v| v.len()).sum())
                .unwrap_or(0);
            assert_eq!(
                total_blocks, 2,
                "Unresolved lenses should have been resolved and displayed"
            );
        });
    }
}
