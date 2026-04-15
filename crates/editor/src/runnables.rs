use std::{collections::BTreeMap, mem, ops::Range, sync::Arc};

use clock::Global;
use collections::{HashMap, HashSet};
use gpui::{
    App, AppContext as _, AsyncWindowContext, ClickEvent, Context, Entity, Focusable as _,
    MouseButton, Task, Window,
};
use language::{Buffer, BufferRow, Runnable};
use lsp::LanguageServerName;
use multi_buffer::{Anchor, BufferOffset, MultiBufferRow, MultiBufferSnapshot, ToPoint as _};
use project::{
    Location, Project, TaskSourceKind,
    debugger::breakpoint_store::{Breakpoint, BreakpointSessionState},
    project_settings::ProjectSettings,
};
use settings::Settings as _;
use smallvec::SmallVec;
use task::{ResolvedTask, RunnableTag, TaskContext, TaskTemplate, TaskVariables, VariableName};
use text::{BufferId, OffsetRangeExt as _, ToOffset as _, ToPoint as _};
use ui::{Clickable as _, Color, IconButton, IconSize, Toggleable as _};

use crate::{
    CodeActionSource, Editor, EditorSettings, EditorStyle, RangeToAnchorExt, SpawnNearestTask,
    ToggleCodeActions, UPDATE_DEBOUNCE, display_map::DisplayRow,
};

#[derive(Debug)]
pub(super) struct RunnableData {
    runnables: HashMap<BufferId, (Global, BTreeMap<BufferRow, RunnableTasks>)>,
    invalidate_buffer_data: HashSet<BufferId>,
    runnables_update_task: Task<()>,
}

impl RunnableData {
    pub fn new() -> Self {
        Self {
            runnables: HashMap::default(),
            invalidate_buffer_data: HashSet::default(),
            runnables_update_task: Task::ready(()),
        }
    }

    pub fn runnables(
        &self,
        (buffer_id, buffer_row): (BufferId, BufferRow),
    ) -> Option<&RunnableTasks> {
        self.runnables.get(&buffer_id)?.1.get(&buffer_row)
    }

    pub fn all_runnables(&self) -> impl Iterator<Item = &RunnableTasks> {
        self.runnables
            .values()
            .flat_map(|(_, tasks)| tasks.values())
    }

    pub fn has_cached(&self, buffer_id: BufferId, version: &Global) -> bool {
        self.runnables
            .get(&buffer_id)
            .is_some_and(|(cached_version, _)| !version.changed_since(cached_version))
    }

    #[cfg(test)]
    pub fn insert(
        &mut self,
        buffer_id: BufferId,
        buffer_row: BufferRow,
        version: Global,
        tasks: RunnableTasks,
    ) {
        self.runnables
            .entry(buffer_id)
            .or_insert_with(|| (version, BTreeMap::default()))
            .1
            .insert(buffer_row, tasks);
    }
}

#[derive(Clone, Debug)]
pub struct RunnableTasks {
    pub templates: Vec<(TaskSourceKind, TaskTemplate)>,
    pub offset: multi_buffer::Anchor,
    // We need the column at which the task context evaluation should take place (when we're spawning it via gutter).
    pub column: u32,
    // Values of all named captures, including those starting with '_'
    pub extra_variables: HashMap<String, String>,
    // Full range of the tagged region. We use it to determine which `extra_variables` to grab for context resolution in e.g. a modal.
    pub context_range: Range<BufferOffset>,
}

impl RunnableTasks {
    pub fn resolve<'a>(
        &'a self,
        cx: &'a task::TaskContext,
    ) -> impl Iterator<Item = (TaskSourceKind, ResolvedTask)> + 'a {
        self.templates.iter().filter_map(|(kind, template)| {
            template
                .resolve_task(&kind.to_id_base(), cx)
                .map(|task| (kind.clone(), task))
        })
    }
}

#[derive(Clone)]
pub struct ResolvedTasks {
    pub templates: SmallVec<[(TaskSourceKind, ResolvedTask); 1]>,
    pub position: Anchor,
}

impl Editor {
    pub fn refresh_runnables(
        &mut self,
        invalidate_buffer_data: Option<BufferId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.mode().is_full()
            || !EditorSettings::get_global(cx).gutter.runnables
            || !self.enable_runnables
        {
            self.clear_runnables(None);
            return;
        }
        if let Some(buffer) = self.buffer().read(cx).as_singleton() {
            let buffer_id = buffer.read(cx).remote_id();
            if invalidate_buffer_data != Some(buffer_id)
                && self
                    .runnables
                    .has_cached(buffer_id, &buffer.read(cx).version())
            {
                return;
            }
        }
        if let Some(buffer_id) = invalidate_buffer_data {
            self.runnables.invalidate_buffer_data.insert(buffer_id);
        }

        let project = self.project().map(Entity::downgrade);
        let lsp_task_sources = self.lsp_task_sources(true, true, cx);
        let multi_buffer = self.buffer.downgrade();
        self.runnables.runnables_update_task = cx.spawn_in(window, async move |editor, cx| {
            cx.background_executor().timer(UPDATE_DEBOUNCE).await;
            let Some(project) = project.and_then(|p| p.upgrade()) else {
                return;
            };

            let hide_runnables = project.update(cx, |project, _| project.is_via_collab());
            if hide_runnables {
                return;
            }
            let lsp_tasks = if lsp_task_sources.is_empty() {
                Vec::new()
            } else {
                let Ok(lsp_tasks) = cx
                    .update(|_, cx| crate::lsp_tasks(project.clone(), &lsp_task_sources, None, cx))
                else {
                    return;
                };
                lsp_tasks.await
            };
            let new_rows = {
                let Some((multi_buffer_snapshot, multi_buffer_query_range)) = editor
                    .update(cx, |editor, cx| {
                        let multi_buffer = editor.buffer().read(cx);
                        if multi_buffer.is_singleton() {
                            Some((multi_buffer.snapshot(cx), Anchor::Min..Anchor::Max))
                        } else {
                            let display_snapshot =
                                editor.display_map.update(cx, |map, cx| map.snapshot(cx));
                            let multi_buffer_query_range =
                                editor.multi_buffer_visible_range(&display_snapshot, cx);
                            let multi_buffer_snapshot = display_snapshot.buffer();
                            Some((
                                multi_buffer_snapshot.clone(),
                                multi_buffer_query_range.to_anchors(&multi_buffer_snapshot),
                            ))
                        }
                    })
                    .ok()
                    .flatten()
                else {
                    return;
                };
                cx.background_spawn({
                    async move {
                        multi_buffer_snapshot
                            .runnable_ranges(multi_buffer_query_range)
                            .collect()
                    }
                })
                .await
            };

            let Ok(multi_buffer_snapshot) =
                editor.update(cx, |editor, cx| editor.buffer().read(cx).snapshot(cx))
            else {
                return;
            };
            let Ok(mut lsp_tasks_by_rows) = cx.update(|_, cx| {
                lsp_tasks
                    .into_iter()
                    .flat_map(|(kind, tasks)| {
                        tasks.into_iter().filter_map(move |(location, task)| {
                            Some((kind.clone(), location?, task))
                        })
                    })
                    .fold(HashMap::default(), |mut acc, (kind, location, task)| {
                        let buffer = location.target.buffer;
                        let buffer_snapshot = buffer.read(cx).snapshot();
                        let offset =
                            multi_buffer_snapshot.anchor_in_excerpt(location.target.range.start);
                        if let Some(offset) = offset {
                            let task_buffer_range =
                                location.target.range.to_point(&buffer_snapshot);
                            let context_buffer_range =
                                task_buffer_range.to_offset(&buffer_snapshot);
                            let context_range = BufferOffset(context_buffer_range.start)
                                ..BufferOffset(context_buffer_range.end);

                            acc.entry((buffer_snapshot.remote_id(), task_buffer_range.start.row))
                                .or_insert_with(|| RunnableTasks {
                                    templates: Vec::new(),
                                    offset,
                                    column: task_buffer_range.start.column,
                                    extra_variables: HashMap::default(),
                                    context_range,
                                })
                                .templates
                                .push((kind, task.original_task().clone()));
                        }

                        acc
                    })
            }) else {
                return;
            };

            let Ok(prefer_lsp) = multi_buffer.update(cx, |buffer, cx| {
                buffer.language_settings(cx).tasks.prefer_lsp
            }) else {
                return;
            };

            let rows = Self::runnable_rows(
                project,
                multi_buffer_snapshot,
                prefer_lsp && !lsp_tasks_by_rows.is_empty(),
                new_rows,
                cx.clone(),
            )
            .await;
            editor
                .update(cx, |editor, cx| {
                    for buffer_id in std::mem::take(&mut editor.runnables.invalidate_buffer_data) {
                        editor.clear_runnables(Some(buffer_id));
                    }

                    for ((buffer_id, row), mut new_tasks) in rows {
                        let Some(buffer) = editor.buffer().read(cx).buffer(buffer_id) else {
                            continue;
                        };

                        if let Some(lsp_tasks) = lsp_tasks_by_rows.remove(&(buffer_id, row)) {
                            new_tasks.templates.extend(lsp_tasks.templates);
                        }
                        editor.insert_runnables(
                            buffer_id,
                            buffer.read(cx).version(),
                            row,
                            new_tasks,
                        );
                    }
                    for ((buffer_id, row), new_tasks) in lsp_tasks_by_rows {
                        let Some(buffer) = editor.buffer().read(cx).buffer(buffer_id) else {
                            continue;
                        };
                        editor.insert_runnables(
                            buffer_id,
                            buffer.read(cx).version(),
                            row,
                            new_tasks,
                        );
                    }
                })
                .ok();
        });
    }

    pub fn spawn_nearest_task(
        &mut self,
        action: &SpawnNearestTask,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((workspace, _)) = self.workspace.clone() else {
            return;
        };
        let Some(project) = self.project.clone() else {
            return;
        };

        // Try to find a closest, enclosing node using tree-sitter that has a task
        let Some((buffer, buffer_row, tasks)) = self
            .find_enclosing_node_task(cx)
            // Or find the task that's closest in row-distance.
            .or_else(|| self.find_closest_task(cx))
        else {
            return;
        };

        let reveal_strategy = action.reveal;
        let task_context = Self::build_tasks_context(&project, &buffer, buffer_row, &tasks, cx);
        cx.spawn_in(window, async move |_, cx| {
            let context = task_context.await?;
            let (task_source_kind, mut resolved_task) = tasks.resolve(&context).next()?;

            let resolved = &mut resolved_task.resolved;
            resolved.reveal = reveal_strategy;

            workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.schedule_resolved_task(
                        task_source_kind,
                        resolved_task,
                        false,
                        window,
                        cx,
                    );
                })
                .ok()
        })
        .detach();
    }

    pub fn clear_runnables(&mut self, for_buffer: Option<BufferId>) {
        if let Some(buffer_id) = for_buffer {
            self.runnables.runnables.remove(&buffer_id);
        } else {
            self.runnables.runnables.clear();
        }
        self.runnables.invalidate_buffer_data.clear();
        self.runnables.runnables_update_task = Task::ready(());
    }

    pub fn task_context(&self, window: &mut Window, cx: &mut App) -> Task<Option<TaskContext>> {
        let Some(project) = self.project.clone() else {
            return Task::ready(None);
        };
        let (selection, buffer, editor_snapshot) = {
            let selection = self.selections.newest_adjusted(&self.display_snapshot(cx));
            let Some((buffer, _)) = self
                .buffer()
                .read(cx)
                .point_to_buffer_offset(selection.start, cx)
            else {
                return Task::ready(None);
            };
            let snapshot = self.snapshot(window, cx);
            (selection, buffer, snapshot)
        };
        let selection_range = selection.range();
        let Some((_, range)) = editor_snapshot
            .display_snapshot
            .buffer_snapshot()
            .anchor_range_to_buffer_anchor_range(
                editor_snapshot
                    .display_snapshot
                    .buffer_snapshot()
                    .anchor_after(selection_range.start)
                    ..editor_snapshot
                        .display_snapshot
                        .buffer_snapshot()
                        .anchor_before(selection_range.end),
            )
        else {
            return Task::ready(None);
        };
        let location = Location { buffer, range };
        let captured_variables = {
            let mut variables = TaskVariables::default();
            let buffer = location.buffer.read(cx);
            let buffer_id = buffer.remote_id();
            let snapshot = buffer.snapshot();
            let starting_point = location.range.start.to_point(&snapshot);
            let starting_offset = starting_point.to_offset(&snapshot);
            for (_, tasks) in self
                .runnables
                .runnables
                .get(&buffer_id)
                .into_iter()
                .flat_map(|(_, tasks)| tasks.range(0..starting_point.row + 1))
            {
                if !tasks
                    .context_range
                    .contains(&crate::BufferOffset(starting_offset))
                {
                    continue;
                }
                for (capture_name, value) in tasks.extra_variables.iter() {
                    variables.insert(
                        VariableName::Custom(capture_name.to_owned().into()),
                        value.clone(),
                    );
                }
            }
            variables
        };

        project.update(cx, |project, cx| {
            project.task_store().update(cx, |task_store, cx| {
                task_store.task_context_for_location(captured_variables, location, cx)
            })
        })
    }

    pub fn lsp_task_sources(
        &self,
        visible_only: bool,
        skip_cached: bool,
        cx: &mut Context<Self>,
    ) -> HashMap<LanguageServerName, Vec<BufferId>> {
        if !self.lsp_data_enabled() {
            return HashMap::default();
        }
        let buffers = if visible_only {
            self.visible_buffers(cx)
                .into_iter()
                .filter(|buffer| self.is_lsp_relevant(buffer.read(cx).file(), cx))
                .collect()
        } else {
            self.buffer().read(cx).all_buffers()
        };

        let lsp_settings = &ProjectSettings::get_global(cx).lsp;

        buffers
            .into_iter()
            .filter_map(|buffer| {
                let lsp_tasks_source = buffer
                    .read(cx)
                    .language()?
                    .context_provider()?
                    .lsp_task_source()?;
                if lsp_settings
                    .get(&lsp_tasks_source)
                    .is_none_or(|s| s.enable_lsp_tasks)
                {
                    let buffer_id = buffer.read(cx).remote_id();
                    if skip_cached
                        && self
                            .runnables
                            .has_cached(buffer_id, &buffer.read(cx).version())
                    {
                        None
                    } else {
                        Some((lsp_tasks_source, buffer_id))
                    }
                } else {
                    None
                }
            })
            .fold(
                HashMap::default(),
                |mut acc, (lsp_task_source, buffer_id)| {
                    acc.entry(lsp_task_source)
                        .or_insert_with(Vec::new)
                        .push(buffer_id);
                    acc
                },
            )
    }

    pub fn find_enclosing_node_task(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Option<(Entity<Buffer>, u32, Arc<RunnableTasks>)> {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let anchor = self.selections.newest_anchor().head();
        let (anchor, buffer_snapshot) = snapshot.anchor_to_buffer_anchor(anchor)?;
        let offset = anchor.to_offset(buffer_snapshot);

        let layer = buffer_snapshot.syntax_layer_at(offset)?;
        let mut cursor = layer.node().walk();

        while cursor.goto_first_child_for_byte(offset).is_some() {
            if cursor.node().end_byte() == offset {
                cursor.goto_next_sibling();
            }
        }

        // Ascend to the smallest ancestor that contains the range and has a task.
        loop {
            let node = cursor.node();
            let node_range = node.byte_range();
            let symbol_start_row = buffer_snapshot.offset_to_point(node.start_byte()).row;

            // Check if this node contains our offset
            if node_range.start <= offset && node_range.end >= offset {
                // If it contains offset, check for task
                if let Some(tasks) = self
                    .runnables
                    .runnables
                    .get(&buffer_snapshot.remote_id())
                    .and_then(|(_, tasks)| tasks.get(&symbol_start_row))
                {
                    let buffer = self.buffer.read(cx).buffer(buffer_snapshot.remote_id())?;
                    return Some((buffer, symbol_start_row, Arc::new(tasks.to_owned())));
                }
            }

            if !cursor.goto_parent() {
                break;
            }
        }
        None
    }

    pub fn render_run_indicator(
        &self,
        _style: &EditorStyle,
        is_active: bool,
        row: DisplayRow,
        breakpoint: Option<(Anchor, Breakpoint, Option<BreakpointSessionState>)>,
        cx: &mut Context<Self>,
    ) -> IconButton {
        let color = Color::Muted;
        let position = breakpoint.as_ref().map(|(anchor, _, _)| *anchor);

        IconButton::new(
            ("run_indicator", row.0 as usize),
            ui::IconName::PlayOutlined,
        )
        .shape(ui::IconButtonShape::Square)
        .icon_size(IconSize::XSmall)
        .icon_color(color)
        .toggle_state(is_active)
        .on_click(cx.listener(move |editor, e: &ClickEvent, window, cx| {
            let quick_launch = match e {
                ClickEvent::Keyboard(_) => true,
                ClickEvent::Mouse(e) => e.down.button == MouseButton::Left,
            };

            window.focus(&editor.focus_handle(cx), cx);
            editor.toggle_code_actions(
                &ToggleCodeActions {
                    deployed_from: Some(CodeActionSource::RunMenu(row)),
                    quick_launch,
                },
                window,
                cx,
            );
        }))
        .on_right_click(cx.listener(move |editor, event: &ClickEvent, window, cx| {
            editor.set_breakpoint_context_menu(row, position, event.position(), window, cx);
        }))
    }

    fn insert_runnables(
        &mut self,
        buffer: BufferId,
        version: Global,
        row: BufferRow,
        new_tasks: RunnableTasks,
    ) {
        let (old_version, tasks) = self.runnables.runnables.entry(buffer).or_default();
        if !old_version.changed_since(&version) {
            *old_version = version;
            tasks.insert(row, new_tasks);
        }
    }

    fn runnable_rows(
        project: Entity<Project>,
        snapshot: MultiBufferSnapshot,
        prefer_lsp: bool,
        runnable_ranges: Vec<(Range<Anchor>, language::RunnableRange)>,
        cx: AsyncWindowContext,
    ) -> Task<Vec<((BufferId, BufferRow), RunnableTasks)>> {
        cx.spawn(async move |cx| {
            let mut runnable_rows = Vec::with_capacity(runnable_ranges.len());
            for (run_range, mut runnable) in runnable_ranges {
                let Some(tasks) = cx
                    .update(|_, cx| Self::templates_with_tags(&project, &mut runnable.runnable, cx))
                    .ok()
                else {
                    continue;
                };
                let mut tasks = tasks.await;

                if prefer_lsp {
                    tasks.retain(|(task_kind, _)| {
                        !matches!(task_kind, TaskSourceKind::Language { .. })
                    });
                }
                if tasks.is_empty() {
                    continue;
                }

                let point = run_range.start.to_point(&snapshot);
                let Some(row) = snapshot
                    .buffer_line_for_row(MultiBufferRow(point.row))
                    .map(|(_, range)| range.start.row)
                else {
                    continue;
                };

                let context_range =
                    BufferOffset(runnable.full_range.start)..BufferOffset(runnable.full_range.end);
                runnable_rows.push((
                    (runnable.buffer_id, row),
                    RunnableTasks {
                        templates: tasks,
                        offset: run_range.start,
                        context_range,
                        column: point.column,
                        extra_variables: runnable.extra_captures,
                    },
                ));
            }
            runnable_rows
        })
    }

    fn templates_with_tags(
        project: &Entity<Project>,
        runnable: &mut Runnable,
        cx: &mut App,
    ) -> Task<Vec<(TaskSourceKind, TaskTemplate)>> {
        let (inventory, worktree_id, buffer) = project.read_with(cx, |project, cx| {
            let buffer = project.buffer_for_id(runnable.buffer, cx);
            let worktree_id = buffer
                .as_ref()
                .and_then(|buffer| buffer.read(cx).file())
                .map(|file| file.worktree_id(cx));

            (
                project.task_store().read(cx).task_inventory().cloned(),
                worktree_id,
                buffer,
            )
        });

        let tags = mem::take(&mut runnable.tags);
        let language = runnable.language.clone();
        cx.spawn(async move |cx| {
            let mut templates_with_tags = Vec::new();
            if let Some(inventory) = inventory {
                for RunnableTag(tag) in tags {
                    let new_tasks = inventory.update(cx, |inventory, cx| {
                        inventory.list_tasks(
                            buffer.clone(),
                            Some(language.clone()),
                            worktree_id,
                            cx,
                        )
                    });
                    templates_with_tags.extend(new_tasks.await.into_iter().filter(
                        move |(_, template)| {
                            template.tags.iter().any(|source_tag| source_tag == &tag)
                        },
                    ));
                }
            }
            templates_with_tags.sort_by_key(|(kind, _)| kind.to_owned());

            if let Some((leading_tag_source, _)) = templates_with_tags.first() {
                // Strongest source wins; if we have worktree tag binding, prefer that to
                // global and language bindings;
                // if we have a global binding, prefer that to language binding.
                let first_mismatch = templates_with_tags
                    .iter()
                    .position(|(tag_source, _)| tag_source != leading_tag_source);
                if let Some(index) = first_mismatch {
                    templates_with_tags.truncate(index);
                }
            }

            templates_with_tags
        })
    }

    fn find_closest_task(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Option<(Entity<Buffer>, u32, Arc<RunnableTasks>)> {
        let cursor_row = self
            .selections
            .newest_adjusted(&self.display_snapshot(cx))
            .head()
            .row;

        let ((buffer_id, row), tasks) = self
            .runnables
            .runnables
            .iter()
            .flat_map(|(buffer_id, (_, tasks))| {
                tasks.iter().map(|(row, tasks)| ((*buffer_id, *row), tasks))
            })
            .min_by_key(|((_, row), _)| cursor_row.abs_diff(*row))?;

        let buffer = self.buffer.read(cx).buffer(buffer_id)?;
        let tasks = Arc::new(tasks.to_owned());
        Some((buffer, row, tasks))
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use futures::StreamExt as _;
    use gpui::{AppContext as _, Entity, Task, TestAppContext};
    use indoc::indoc;
    use language::{ContextProvider, FakeLspAdapter};
    use languages::rust_lang;
    use lsp::LanguageServerName;
    use multi_buffer::{MultiBuffer, PathKey};
    use project::{
        FakeFs, Project,
        lsp_store::lsp_ext_command::{CargoRunnableArgs, Runnable, RunnableArgs, RunnableKind},
    };
    use serde_json::json;
    use task::{TaskTemplate, TaskTemplates};
    use text::Point;
    use util::path;

    use crate::{
        Editor, UPDATE_DEBOUNCE, editor_tests::init_test, scroll::scroll_amount::ScrollAmount,
        test::build_editor_with_project,
    };

    const FAKE_LSP_NAME: &str = "the-fake-language-server";

    struct TestRustContextProvider;

    impl ContextProvider for TestRustContextProvider {
        fn associated_tasks(
            &self,
            _: Option<Entity<language::Buffer>>,
            _: &gpui::App,
        ) -> Task<Option<TaskTemplates>> {
            Task::ready(Some(TaskTemplates(vec![
                TaskTemplate {
                    label: "Run main".into(),
                    command: "cargo".into(),
                    args: vec!["run".into()],
                    tags: vec!["rust-main".into()],
                    ..TaskTemplate::default()
                },
                TaskTemplate {
                    label: "Run test".into(),
                    command: "cargo".into(),
                    args: vec!["test".into()],
                    tags: vec!["rust-test".into()],
                    ..TaskTemplate::default()
                },
            ])))
        }
    }

    struct TestRustContextProviderWithLsp;

    impl ContextProvider for TestRustContextProviderWithLsp {
        fn associated_tasks(
            &self,
            _: Option<Entity<language::Buffer>>,
            _: &gpui::App,
        ) -> Task<Option<TaskTemplates>> {
            Task::ready(Some(TaskTemplates(vec![TaskTemplate {
                label: "Run test".into(),
                command: "cargo".into(),
                args: vec!["test".into()],
                tags: vec!["rust-test".into()],
                ..TaskTemplate::default()
            }])))
        }

        fn lsp_task_source(&self) -> Option<LanguageServerName> {
            Some(LanguageServerName::new_static(FAKE_LSP_NAME))
        }
    }

    fn rust_lang_with_task_context() -> Arc<language::Language> {
        Arc::new(
            Arc::try_unwrap(rust_lang())
                .unwrap()
                .with_context_provider(Some(Arc::new(TestRustContextProvider))),
        )
    }

    fn rust_lang_with_lsp_task_context() -> Arc<language::Language> {
        Arc::new(
            Arc::try_unwrap(rust_lang())
                .unwrap()
                .with_context_provider(Some(Arc::new(TestRustContextProviderWithLsp))),
        )
    }

    fn collect_runnable_labels(
        editor: &Editor,
    ) -> Vec<(text::BufferId, language::BufferRow, Vec<String>)> {
        let mut result = editor
            .runnables
            .runnables
            .iter()
            .flat_map(|(buffer_id, (_, tasks))| {
                tasks.iter().map(move |(row, runnable_tasks)| {
                    let mut labels: Vec<String> = runnable_tasks
                        .templates
                        .iter()
                        .map(|(_, template)| template.label.clone())
                        .collect();
                    labels.sort();
                    (*buffer_id, *row, labels)
                })
            })
            .collect::<Vec<_>>();
        result.sort_by_key(|(id, row, _)| (*id, *row));
        result
    }

    #[gpui::test]
    async fn test_multi_buffer_runnables_on_scroll(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let padding_lines = 50;
        let mut first_rs = String::from("fn main() {\n    println!(\"hello\");\n}\n");
        for _ in 0..padding_lines {
            first_rs.push_str("//\n");
        }
        let test_one_row = 3 + padding_lines as u32 + 1;
        first_rs.push_str("#[test]\nfn test_one() {\n    assert!(true);\n}\n");

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "first.rs": first_rs,
                "second.rs": indoc! {"
                    #[test]
                    fn test_two() {
                        assert!(true);
                    }

                    #[test]
                    fn test_three() {
                        assert!(true);
                    }
                "},
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang_with_task_context());

        let buffer_1 = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/first.rs"), cx)
            })
            .await
            .unwrap();
        let buffer_2 = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/second.rs"), cx)
            })
            .await
            .unwrap();

        let buffer_1_id = buffer_1.read_with(cx, |buffer, _| buffer.remote_id());
        let buffer_2_id = buffer_2.read_with(cx, |buffer, _| buffer.remote_id());

        let multi_buffer = cx.new(|cx| {
            let mut multi_buffer = MultiBuffer::new(language::Capability::ReadWrite);
            let end = buffer_1.read(cx).max_point();
            multi_buffer.set_excerpts_for_path(
                PathKey::sorted(0),
                buffer_1.clone(),
                [Point::new(0, 0)..end],
                0,
                cx,
            );
            multi_buffer.set_excerpts_for_path(
                PathKey::sorted(1),
                buffer_2.clone(),
                [Point::new(0, 0)..Point::new(8, 1)],
                0,
                cx,
            );
            multi_buffer
        });

        let editor = cx.add_window(|window, cx| {
            Editor::for_multibuffer(multi_buffer, Some(project.clone()), window, cx)
        });
        cx.executor().advance_clock(Duration::from_millis(500));
        cx.executor().run_until_parked();

        // Clear stale data from startup events, then refresh.
        // first.rs is long enough that second.rs is below the ~47-line viewport.
        editor
            .update(cx, |editor, window, cx| {
                editor.clear_runnables(None);
                editor.refresh_runnables(None, window, cx);
            })
            .unwrap();
        cx.executor().advance_clock(UPDATE_DEBOUNCE);
        cx.executor().run_until_parked();
        assert_eq!(
            editor
                .update(cx, |editor, _, _| collect_runnable_labels(editor))
                .unwrap(),
            vec![(buffer_1_id, 0, vec!["Run main".to_string()])],
            "Only fn main from first.rs should be visible before scrolling"
        );

        // Scroll down to bring second.rs excerpts into view.
        editor
            .update(cx, |editor, window, cx| {
                editor.scroll_screen(&ScrollAmount::Page(1.0), window, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(200));
        cx.executor().run_until_parked();

        let after_scroll = editor
            .update(cx, |editor, _, _| collect_runnable_labels(editor))
            .unwrap();
        assert_eq!(
            after_scroll,
            vec![
                (buffer_1_id, 0, vec!["Run main".to_string()]),
                (buffer_1_id, test_one_row, vec!["Run test".to_string()]),
                (buffer_2_id, 1, vec!["Run test".to_string()]),
                (buffer_2_id, 6, vec!["Run test".to_string()]),
            ],
            "Tree-sitter should detect both #[test] fns in second.rs after scroll"
        );

        // Edit second.rs to invalidate its cache; first.rs data should persist.
        buffer_2.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "// added comment\n")], None, cx);
        });
        editor
            .update(cx, |editor, window, cx| {
                editor.scroll_screen(&ScrollAmount::Page(-1.0), window, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(200));
        cx.executor().run_until_parked();

        assert_eq!(
            editor
                .update(cx, |editor, _, _| collect_runnable_labels(editor))
                .unwrap(),
            vec![
                (buffer_1_id, 0, vec!["Run main".to_string()]),
                (buffer_1_id, test_one_row, vec!["Run test".to_string()]),
            ],
            "first.rs runnables should survive an edit to second.rs"
        );
    }

    #[gpui::test]
    async fn test_lsp_runnables_removed_after_edit(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "main.rs": indoc! {"
                    #[test]
                    fn test_one() {
                        assert!(true);
                    }

                    fn helper() {}
                "},
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang_with_lsp_task_context());

        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: FAKE_LSP_NAME,
                ..FakeLspAdapter::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/main.rs"), cx)
            })
            .await
            .unwrap();

        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());

        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));
        let editor = cx.add_window(|window, cx| {
            build_editor_with_project(project.clone(), multi_buffer, window, cx)
        });

        let fake_server = fake_servers.next().await.expect("fake LSP server");

        use project::lsp_store::lsp_ext_command::Runnables;
        fake_server.set_request_handler::<Runnables, _, _>(move |params, _| async move {
            let text = params.text_document.uri.path().to_string();
            if text.contains("main.rs") {
                let uri = lsp::Uri::from_file_path(path!("/project/main.rs")).expect("valid uri");
                Ok(vec![Runnable {
                    label: "LSP test_one".into(),
                    location: Some(lsp::LocationLink {
                        origin_selection_range: None,
                        target_uri: uri,
                        target_range: lsp::Range::new(
                            lsp::Position::new(0, 0),
                            lsp::Position::new(3, 1),
                        ),
                        target_selection_range: lsp::Range::new(
                            lsp::Position::new(0, 0),
                            lsp::Position::new(3, 1),
                        ),
                    }),
                    kind: RunnableKind::Cargo,
                    args: RunnableArgs::Cargo(CargoRunnableArgs {
                        environment: Default::default(),
                        cwd: path!("/project").into(),
                        override_cargo: None,
                        workspace_root: None,
                        cargo_args: vec!["test".into(), "test_one".into()],
                        executable_args: Vec::new(),
                    }),
                }])
            } else {
                Ok(Vec::new())
            }
        });

        // Trigger a refresh to pick up both tree-sitter and LSP runnables.
        editor
            .update(cx, |editor, window, cx| {
                editor.refresh_runnables(None, window, cx);
            })
            .expect("editor update");
        cx.executor().advance_clock(UPDATE_DEBOUNCE);
        cx.executor().run_until_parked();

        let labels = editor
            .update(cx, |editor, _, _| collect_runnable_labels(editor))
            .expect("editor update");
        assert_eq!(
            labels,
            vec![(buffer_id, 0, vec!["LSP test_one".to_string()]),],
            "LSP runnables should appear for #[test] fn"
        );

        // Remove `#[test]` attribute so the function is no longer a test.
        buffer.update(cx, |buffer, cx| {
            let test_attr_end = buffer.text().find("\nfn test_one").expect("find fn");
            buffer.edit([(0..test_attr_end, "")], None, cx);
        });

        // Also update the LSP handler to return no runnables.
        fake_server
            .set_request_handler::<Runnables, _, _>(move |_, _| async move { Ok(Vec::new()) });

        cx.executor().advance_clock(UPDATE_DEBOUNCE);
        cx.executor().run_until_parked();

        let labels = editor
            .update(cx, |editor, _, _| collect_runnable_labels(editor))
            .expect("editor update");
        assert_eq!(
            labels,
            Vec::<(text::BufferId, language::BufferRow, Vec<String>)>::new(),
            "Runnables should be removed after #[test] is deleted and LSP returns empty"
        );
    }
}
