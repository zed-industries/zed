use std::sync::Arc;

use collections::{HashMap, HashSet};
use futures::future::join_all;
use gpui::{MouseButton, SharedString, Task, WeakEntity};
use itertools::Itertools;
use language::{BufferId, ClientCommand};
use multi_buffer::{Anchor, MultiBufferRow, MultiBufferSnapshot, ToPoint as _};
use project::{CodeAction, TaskSourceKind};
use task::TaskContext;

use ui::{Context, Window, div, prelude::*};

use crate::{
    Editor, LSP_REQUEST_DEBOUNCE_TIMEOUT, SelectionEffects,
    actions::ToggleCodeLens,
    display_map::{BlockPlacement, BlockProperties, BlockStyle, CustomBlockId, RenderBlock},
    hover_links::HoverLink,
};

#[derive(Clone, Debug)]
struct CodeLensLine {
    position: Anchor,
    indent_column: u32,
    items: Vec<CodeLensItem>,
}

#[derive(Clone, Debug)]
struct CodeLensItem {
    title: SharedString,
    action: CodeAction,
}

pub(super) struct CodeLensBlock {
    block_id: CustomBlockId,
    anchor: Anchor,
    line: CodeLensLine,
}

pub(super) struct CodeLensState {
    pub(super) blocks: HashMap<BufferId, Vec<CodeLensBlock>>,
    actions: HashMap<BufferId, Vec<CodeAction>>,
    resolve_task: Task<()>,
}

impl Default for CodeLensState {
    fn default() -> Self {
        Self {
            blocks: HashMap::default(),
            actions: HashMap::default(),
            resolve_task: Task::ready(()),
        }
    }
}

pub(super) fn try_handle_client_command(
    action: &CodeAction,
    editor: &mut Editor,
    workspace: &gpui::Entity<workspace::Workspace>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    let Some(command) = action.lsp_action.command() else {
        return false;
    };

    let arguments = command.arguments.as_deref().unwrap_or_default();
    let project = workspace.read(cx).project().clone();
    let client_command = project
        .read(cx)
        .lsp_store()
        .read(cx)
        .language_server_adapter_for_id(action.server_id)
        .and_then(|adapter| adapter.adapter.client_command(&command.command, arguments))
        .or_else(|| match command.command.as_str() {
            "editor.action.showReferences"
            | "editor.action.goToLocations"
            | "editor.action.peekLocations" => Some(ClientCommand::ShowLocations),
            _ => None,
        });

    match client_command {
        Some(ClientCommand::ScheduleTask(task_template)) => {
            schedule_task(task_template, action, editor, workspace, window, cx)
        }
        Some(ClientCommand::ShowLocations) => {
            try_show_references(arguments, action, editor, window, cx)
        }
        None => false,
    }
}

fn schedule_task(
    task_template: task::TaskTemplate,
    action: &CodeAction,
    editor: &Editor,
    workspace: &gpui::Entity<workspace::Workspace>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    let task_context = TaskContext {
        cwd: task_template.cwd.as_ref().map(std::path::PathBuf::from),
        ..TaskContext::default()
    };
    let language_name = editor
        .buffer()
        .read(cx)
        .buffer(action.range.start.buffer_id)
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
    editor: &mut Editor,
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
    let nav_entry = editor.navigation_entry(editor.selections.newest_anchor().head(), cx);
    let links = locations
        .into_iter()
        .map(|location| HoverLink::InlayHint(location, server_id))
        .collect();
    editor
        .navigate_to_hover_links(None, links, nav_entry, false, window, cx)
        .detach_and_log_err(cx);

    true
}

impl Editor {
    pub(super) fn refresh_code_lenses(
        &mut self,
        for_buffer: Option<BufferId>,
        _window: &Window,
        cx: &mut Context<Self>,
    ) {
        if !self.lsp_data_enabled() || self.code_lens.is_none() {
            return;
        }
        let Some(project) = self.project.clone() else {
            return;
        };

        let buffers_to_query = self
            .visible_buffers(cx)
            .into_iter()
            .filter(|buffer| self.is_lsp_relevant(buffer.read(cx).file(), cx))
            .chain(for_buffer.and_then(|buffer_id| self.buffer.read(cx).buffer(buffer_id)))
            .filter(|editor_buffer| {
                let editor_buffer_id = editor_buffer.read(cx).remote_id();
                for_buffer.is_none_or(|buffer_id| buffer_id == editor_buffer_id)
                    && self.registered_buffers.contains_key(&editor_buffer_id)
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

            editor
                .update(cx, |editor, cx| {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    for (buffer_id, result) in results {
                        let actions = match result {
                            Ok(Some(actions)) => actions,
                            Ok(None) => continue,
                            Err(e) => {
                                log::error!(
                                    "Failed to fetch code lenses for buffer {buffer_id:?}: {e:#}"
                                );
                                continue;
                            }
                        };
                        editor.apply_lens_actions_for_buffer(buffer_id, actions, &snapshot, cx);
                    }
                    editor.resolve_visible_code_lenses(cx);
                })
                .ok();
        });
    }

    /// Reconciles the set of blocks for `buffer_id` with `actions`. For each
    /// existing block at row `R`:
    /// - if the new fetch has no lens at `R` → remove the block (the lens is
    ///   gone, e.g. the function was deleted);
    /// - if the new fetch has a titled lens at `R` whose rendered text
    ///   differs from the block's current line → swap the renderer in place
    ///   via [`Editor::replace_blocks`];
    /// - if the new fetch has a titled lens at `R` with the same rendered
    ///   text → keep the block as-is;
    /// - if the new fetch has a lens at `R` but no `command` yet (the server
    ///   sent a shallow response that needs a separate `resolve`) → keep the
    ///   block as-is. The previously rendered (resolved) content stays on
    ///   screen until the next viewport-driven `resolve` produces a new
    ///   title; only then does the comparison-and-replace happen. This is
    ///   what keeps the post-edit screen from flickering for shallow servers
    ///   like `rust-analyzer`.
    ///
    /// Rows present in the new fetch with a title but no existing block get
    /// a fresh block inserted.
    fn apply_lens_actions_for_buffer(
        &mut self,
        buffer_id: BufferId,
        actions: Vec<CodeAction>,
        snapshot: &MultiBufferSnapshot,
        cx: &mut Context<Self>,
    ) {
        let mut rows_with_any_lens = HashSet::default();
        let mut titled_lenses = Vec::new();
        for action in &actions {
            let Some(position) = snapshot.anchor_in_excerpt(action.range.start) else {
                continue;
            };

            rows_with_any_lens.insert(MultiBufferRow(position.to_point(snapshot).row));
            if let project::LspAction::CodeLens(lens) = &action.lsp_action {
                if let Some(title) = lens
                    .command
                    .as_ref()
                    .map(|cmd| SharedString::from(&cmd.title))
                {
                    titled_lenses.push((
                        position,
                        CodeLensItem {
                            title,
                            action: action.clone(),
                        },
                    ));
                }
            }
        }

        let mut new_lines_by_row = group_lenses_by_row(titled_lenses, snapshot)
            .map(|line| (MultiBufferRow(line.position.to_point(snapshot).row), line))
            .collect::<HashMap<_, _>>();

        let editor_handle = cx.entity().downgrade();
        let code_lens = self.code_lens.get_or_insert_with(CodeLensState::default);
        let old_blocks = code_lens.blocks.remove(&buffer_id).unwrap_or_default();

        let mut kept_blocks = Vec::new();
        let mut renderers_to_replace = HashMap::default();
        let mut blocks_to_remove = HashSet::default();
        let mut covered_rows = HashSet::default();

        for old in old_blocks {
            let row = MultiBufferRow(old.anchor.to_point(snapshot).row);
            if !rows_with_any_lens.contains(&row) {
                blocks_to_remove.insert(old.block_id);
                continue;
            }
            covered_rows.insert(row);
            let Some(new_line) = new_lines_by_row.remove(&row) else {
                kept_blocks.push(old);
                continue;
            };
            if rendered_text_matches(&old.line, &new_line) {
                kept_blocks.push(old);
            } else {
                let mut updated = old;
                updated.line = new_line.clone();
                renderers_to_replace.insert(
                    updated.block_id,
                    build_code_lens_renderer(new_line, editor_handle.clone()),
                );
                kept_blocks.push(updated);
            }
        }

        let mut to_insert = Vec::new();
        for (row, new_line) in new_lines_by_row {
            if covered_rows.contains(&row) {
                continue;
            }
            let anchor = new_line.position;
            let props = BlockProperties {
                placement: BlockPlacement::Above(anchor),
                height: Some(1),
                style: BlockStyle::Flex,
                render: build_code_lens_renderer(new_line.clone(), editor_handle.clone()),
                priority: 0,
            };
            to_insert.push((props, anchor, new_line));
        }

        if !blocks_to_remove.is_empty() {
            self.remove_blocks(blocks_to_remove, None, cx);
        }
        if !renderers_to_replace.is_empty() {
            self.replace_blocks(renderers_to_replace, None, cx);
        }
        if !to_insert.is_empty() {
            let mut props = Vec::with_capacity(to_insert.len());
            let mut metadata = Vec::with_capacity(to_insert.len());
            for (p, anchor, line) in to_insert {
                props.push(p);
                metadata.push((anchor, line));
            }
            let block_ids = self.insert_blocks(props, None, cx);
            for (block_id, (anchor, line)) in block_ids.into_iter().zip(metadata) {
                kept_blocks.push(CodeLensBlock {
                    block_id,
                    anchor,
                    line,
                });
            }
        }

        let code_lens = self.code_lens.get_or_insert_with(CodeLensState::default);
        if actions.is_empty() {
            code_lens.actions.remove(&buffer_id);
        } else {
            code_lens.actions.insert(buffer_id, actions);
        }
        if kept_blocks.is_empty() {
            code_lens.blocks.remove(&buffer_id);
        } else {
            code_lens.blocks.insert(buffer_id, kept_blocks);
        }
        cx.notify();
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

    pub(super) fn resolve_visible_code_lenses(&mut self, cx: &mut Context<Self>) {
        if !self.lsp_data_enabled() || self.code_lens.is_none() {
            return;
        }
        let Some(project) = self.project.clone() else {
            return;
        };

        let resolve_tasks = self
            .visible_buffer_ranges(cx)
            .into_iter()
            .filter_map(|(snapshot, visible_range, _)| {
                let buffer_id = snapshot.remote_id();
                let buffer = self.buffer.read(cx).buffer(buffer_id)?;
                let visible_anchor_range = snapshot.anchor_before(visible_range.start)
                    ..snapshot.anchor_after(visible_range.end);
                let task = project.update(cx, |project, cx| {
                    project.lsp_store().update(cx, |lsp_store, cx| {
                        lsp_store.resolve_visible_code_lenses(&buffer, visible_anchor_range, cx)
                    })
                });
                Some((buffer_id, task))
            })
            .collect::<Vec<_>>();
        if resolve_tasks.is_empty() {
            return;
        }

        let code_lens = self.code_lens.get_or_insert_with(CodeLensState::default);
        code_lens.resolve_task = cx.spawn(async move |editor, cx| {
            let resolved_per_buffer = join_all(
                resolve_tasks
                    .into_iter()
                    .map(|(buffer_id, task)| async move { (buffer_id, task.await) }),
            )
            .await;
            editor
                .update(cx, |editor, cx| {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    for (buffer_id, newly_resolved) in resolved_per_buffer {
                        if newly_resolved.is_empty() {
                            continue;
                        }
                        let Some(mut actions) = editor
                            .code_lens
                            .as_ref()
                            .and_then(|state| state.actions.get(&buffer_id))
                            .cloned()
                        else {
                            continue;
                        };
                        for resolved in newly_resolved {
                            if let Some(unresolved) = actions.iter_mut().find(|action| {
                                action.server_id == resolved.server_id
                                    && action.range == resolved.range
                            }) {
                                *unresolved = resolved;
                            }
                        }
                        editor.apply_lens_actions_for_buffer(buffer_id, actions, &snapshot, cx);
                    }
                })
                .ok();
        });
    }

    pub(super) fn clear_code_lenses(&mut self, cx: &mut Context<Self>) {
        if let Some(code_lens) = self.code_lens.take() {
            let all_blocks = code_lens
                .blocks
                .into_values()
                .flatten()
                .map(|block| block.block_id)
                .collect::<HashSet<_>>();
            if !all_blocks.is_empty() {
                self.remove_blocks(all_blocks, None, cx);
            }
            cx.notify();
        }
        self.refresh_code_lens_task = Task::ready(());
    }
}

/// Whether two lens lines would render the same on screen — same indent
/// and same titles in the same order. Used to skip recreating a renderer
/// (and thus a click handler) when nothing about the displayed line
/// changed; the captured [`CodeAction`] inside the existing renderer keeps
/// pointing at the right spot because its anchors track buffer edits.
fn rendered_text_matches(a: &CodeLensLine, b: &CodeLensLine) -> bool {
    a.indent_column == b.indent_column
        && a.items.len() == b.items.len()
        && a.items
            .iter()
            .zip(&b.items)
            .all(|(x, y)| x.title == y.title)
}

fn group_lenses_by_row(
    lenses: Vec<(Anchor, CodeLensItem)>,
    snapshot: &MultiBufferSnapshot,
) -> impl Iterator<Item = CodeLensLine> {
    lenses
        .into_iter()
        .into_group_map_by(|(position, _)| {
            let row = position.to_point(snapshot).row;
            MultiBufferRow(row)
        })
        .into_iter()
        .sorted_by_key(|(row, _)| *row)
        .filter_map(|(row, entries)| {
            let position = entries.first()?.0;
            let items = entries.into_iter().map(|(_, item)| item).collect();
            let indent_column = snapshot.indent_size_for_line(row).len;
            Some(CodeLensLine {
                position,
                indent_column,
                items,
            })
        })
}

fn build_code_lens_renderer(line: CodeLensLine, editor: WeakEntity<Editor>) -> RenderBlock {
    Arc::new(move |cx| {
        let mut children = Vec::with_capacity((2 * line.items.len()).saturating_sub(1));
        let text_style = &cx.editor_style.text;
        let font = text_style.font();
        let font_size = text_style.font_size.to_pixels(cx.window.rem_size()) * 0.9;

        for (i, item) in line.items.iter().enumerate() {
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
            let position = line.position;
            let editor_handle = editor.clone();

            children.push(
                div()
                    .id(ElementId::from(i))
                    .font(font.clone())
                    .text_size(font_size)
                    .text_color(cx.app.theme().colors().text_muted)
                    .cursor_pointer()
                    .hover(|style| style.text_color(cx.app.theme().colors().text))
                    .child(title)
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
                                        if let Some(buffer) = editor
                                            .buffer()
                                            .read(cx)
                                            .buffer(action.range.start.buffer_id)
                                        {
                                            project
                                                .update(cx, |project, cx| {
                                                    project
                                                        .apply_code_action(buffer, action, true, cx)
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
            .id(cx.block_id)
            .pl(cx.margins.gutter.full_width() + cx.em_width * (line.indent_column as f32 + 0.5))
            .h_full()
            .flex()
            .flex_row()
            .items_end()
            .children(children)
            .into_any_element()
    })
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };

    use collections::HashSet;
    use futures::StreamExt;
    use gpui::TestAppContext;
    use settings::CodeLens;
    use util::path;

    use crate::{
        Editor, LSP_REQUEST_DEBOUNCE_TIMEOUT,
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
                .map(|s| s.blocks.values().map(|v| v.len()).sum())
                .unwrap_or(0);
            assert_eq!(total_blocks, 2, "Should have inserted two code lens blocks");
        });
    }

    #[gpui::test]
    async fn test_code_lens_blocks_kept_across_refresh(cx: &mut TestAppContext) {
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

        cx.set_state("ˇfunction hello() {}\nfunction world() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received the initial code lens request"
        );
        cx.run_until_parked();

        let initial_block_ids = cx.editor.read_with(&cx.cx.cx, |editor, _| {
            editor
                .code_lens
                .as_ref()
                .map(|s| {
                    s.blocks
                        .values()
                        .flatten()
                        .map(|b| b.block_id)
                        .collect::<HashSet<_>>()
                })
                .unwrap_or_default()
        });
        assert_eq!(
            initial_block_ids.len(),
            1,
            "Should have one initial code lens block"
        );

        cx.update_editor(|editor, window, cx| {
            editor.move_to_end(&crate::actions::MoveToEnd, window, cx);
            editor.handle_input("\n// trailing comment", window, cx);
        });
        cx.executor()
            .advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT + Duration::from_millis(50));
        assert!(
            code_lens_request.next().await.is_some(),
            "should have received another code lens request after edit"
        );
        cx.run_until_parked();

        let refreshed_block_ids = cx.editor.read_with(&cx.cx.cx, |editor, _| {
            editor
                .code_lens
                .as_ref()
                .map(|s| {
                    s.blocks
                        .values()
                        .flatten()
                        .map(|b| b.block_id)
                        .collect::<HashSet<_>>()
                })
                .unwrap_or_default()
        });
        assert_eq!(
            refreshed_block_ids, initial_block_ids,
            "Code lens blocks should be preserved across refreshes when their content is unchanged"
        );
    }

    #[gpui::test]
    async fn test_code_lens_blocks_kept_when_only_resolve_fills_titles(cx: &mut TestAppContext) {
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

        // The LSP returns shallow code lenses on every fetch; only `resolve`
        // populates the command/title. This is the realistic flow with
        // servers like rust-analyzer and exercises the path where each
        // post-edit refresh comes back unresolved before the resolve catches
        // up.
        let mut code_lens_request =
            cx.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _, _| async {
                Ok(Some(vec![lsp::CodeLens {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 19)),
                    command: None,
                    data: Some(serde_json::json!({"id": "lens_1"})),
                }]))
            });

        cx.lsp
            .set_request_handler::<lsp::request::CodeLensResolve, _, _>(|lens, _| async move {
                Ok(lsp::CodeLens {
                    command: Some(lsp::Command {
                        title: "1 reference".to_owned(),
                        command: "resolved_cmd".to_owned(),
                        arguments: None,
                    }),
                    ..lens
                })
            });

        cx.set_state("ˇfunction hello() {}\nfunction world() {}");

        assert!(
            code_lens_request.next().await.is_some(),
            "should have received the initial code lens request"
        );
        cx.run_until_parked();

        let initial = cx.editor.read_with(&cx.cx.cx, |editor, _| {
            editor
                .code_lens
                .as_ref()
                .map(|s| {
                    s.blocks
                        .values()
                        .flatten()
                        .map(|b| b.block_id)
                        .collect::<HashSet<_>>()
                })
                .unwrap_or_default()
        });
        assert_eq!(
            initial.len(),
            1,
            "resolve should have inserted exactly one block from the shallow lens"
        );

        for keystroke in [" ", "x", "y"] {
            cx.update_editor(|editor, window, cx| {
                editor.move_to_end(&crate::actions::MoveToEnd, window, cx);
                editor.handle_input(keystroke, window, cx);
            });
            cx.executor()
                .advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT + Duration::from_millis(50));
            assert!(
                code_lens_request.next().await.is_some(),
                "should have received another (shallow) code lens request after edit"
            );
            cx.run_until_parked();

            let after = cx.editor.read_with(&cx.cx.cx, |editor, _| {
                editor
                    .code_lens
                    .as_ref()
                    .map(|s| {
                        s.blocks
                            .values()
                            .flatten()
                            .map(|b| b.block_id)
                            .collect::<HashSet<_>>()
                    })
                    .unwrap_or_default()
            });
            assert_eq!(
                after, initial,
                "Block IDs must survive the unresolved-fetch → resolve cycle without churn"
            );
        }
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
                .map(|s| s.blocks.values().map(|v| v.len()).sum())
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
                .map(|s| s.blocks.values().map(|v| v.len()).sum())
                .unwrap_or(0);
            assert_eq!(
                total_blocks, 2,
                "Unresolved lenses should have been resolved and displayed"
            );
        });
    }

    #[gpui::test]
    async fn test_code_lens_resolve_only_visible(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        update_test_editor_settings(cx, &|settings| {
            settings.code_lens = Some(CodeLens::On);
        });

        let line_count: u32 = 100;
        let lens_every: u32 = 10;
        let lines = (0..line_count)
            .map(|i| format!("function func_{i}() {{}}"))
            .collect::<Vec<_>>()
            .join("\n");

        let lens_lines = (0..line_count)
            .filter(|i| i % lens_every == 0)
            .collect::<Vec<_>>();

        let resolved_lines = Arc::new(Mutex::new(Vec::<u32>::new()));

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), serde_json::json!({ "main.ts": lines }))
            .await;

        let project = project::Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            workspace::MultiWorkspace::test_new(project.clone(), window, cx)
        });
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(language::Language::new(
            language::LanguageConfig {
                name: "TypeScript".into(),
                matcher: language::LanguageMatcher {
                    path_suffixes: vec!["ts".to_string()],
                    ..language::LanguageMatcher::default()
                },
                ..language::LanguageConfig::default()
            },
            Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
        )));

        let mut fake_servers = language_registry.register_fake_lsp(
            "TypeScript",
            language::FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    code_lens_provider: Some(lsp::CodeLensOptions {
                        resolve_provider: Some(true),
                    }),
                    ..lsp::ServerCapabilities::default()
                },
                ..language::FakeLspAdapter::default()
            },
        );

        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    std::path::PathBuf::from(path!("/dir/main.ts")),
                    workspace::OpenOptions::default(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let fake_server = fake_servers.next().await.unwrap();

        let lens_lines_for_handler = lens_lines.clone();
        fake_server.set_request_handler::<lsp::request::CodeLensRequest, _, _>(move |_, _| {
            let lens_lines = lens_lines_for_handler.clone();
            async move {
                Ok(Some(
                    lens_lines
                        .iter()
                        .map(|&line| lsp::CodeLens {
                            range: lsp::Range::new(
                                lsp::Position::new(line, 0),
                                lsp::Position::new(line, 10),
                            ),
                            command: None,
                            data: Some(serde_json::json!({ "line": line })),
                        })
                        .collect(),
                ))
            }
        });

        {
            let resolved_lines = resolved_lines.clone();
            fake_server.set_request_handler::<lsp::request::CodeLensResolve, _, _>(
                move |lens, _| {
                    let resolved_lines = resolved_lines.clone();
                    async move {
                        let line = lens
                            .data
                            .as_ref()
                            .and_then(|d| d.get("line"))
                            .and_then(|v| v.as_u64())
                            .unwrap() as u32;
                        resolved_lines.lock().unwrap().push(line);
                        Ok(lsp::CodeLens {
                            command: Some(lsp::Command {
                                title: format!("{line} references"),
                                command: format!("show_refs_{line}"),
                                arguments: None,
                            }),
                            ..lens
                        })
                    }
                },
            );
        }

        cx.executor().advance_clock(Duration::from_millis(500));
        cx.run_until_parked();

        let initial_resolved = resolved_lines
            .lock()
            .unwrap()
            .drain(..)
            .collect::<HashSet<_>>();
        assert_eq!(
            initial_resolved,
            HashSet::from_iter([0, 10, 20, 30, 40]),
            "Only lenses visible at the top should be resolved"
        );

        editor.update_in(cx, |editor, window, cx| {
            editor.move_to_end(&crate::actions::MoveToEnd, window, cx);
        });
        cx.executor().advance_clock(Duration::from_millis(500));
        cx.run_until_parked();

        let after_scroll_resolved = resolved_lines
            .lock()
            .unwrap()
            .drain(..)
            .collect::<HashSet<_>>();
        assert_eq!(
            after_scroll_resolved,
            HashSet::from_iter([60, 70, 80, 90]),
            "Only newly visible lenses at the bottom should be resolved, not middle ones"
        );
    }
}
