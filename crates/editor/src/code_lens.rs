use std::ops::Range;

use collections::HashMap;
use gpui::{App, SharedString, Task, WeakEntity};
use language::BufferId;
use multi_buffer::{Anchor, MultiBufferSnapshot, ToPoint as _};
use project::CodeAction;
use settings::Settings;
use ui::{Context, Window, div, prelude::*};

use crate::{
    Editor, FindAllReferences, GoToImplementation, SelectionEffects,
    display_map::{BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
};

#[derive(Clone, Debug)]
pub struct CodeLensItem {
    pub text: SharedString,
    pub action: Option<CodeAction>,
}

#[derive(Clone, Debug)]
pub struct CodeLensData {
    pub position: Anchor,
    pub items: Vec<CodeLensItem>,
}

#[derive(Default)]
pub struct CodeLensCache {
    enabled: bool,
    lenses: HashMap<BufferId, Vec<CodeLensData>>,
    pending_refresh: HashMap<BufferId, Task<()>>,
    block_ids: HashMap<BufferId, Vec<CustomBlockId>>,
}

impl CodeLensCache {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            lenses: HashMap::default(),
            pending_refresh: HashMap::default(),
            block_ids: HashMap::default(),
        }
    }

    pub fn toggle(&mut self, enabled: bool) -> bool {
        if self.enabled == enabled {
            return false;
        }
        self.enabled = enabled;
        if !enabled {
            self.clear();
        }
        true
    }

    pub fn clear(&mut self) {
        self.lenses.clear();
        self.pending_refresh.clear();
        self.block_ids.clear();
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_lenses_for_buffer(&self, buffer_id: BufferId) -> Option<&Vec<CodeLensData>> {
        self.lenses.get(&buffer_id)
    }

    pub fn set_lenses_for_buffer(&mut self, buffer_id: BufferId, lenses: Vec<CodeLensData>) {
        self.lenses.insert(buffer_id, lenses);
    }

    pub fn set_block_ids(&mut self, buffer_id: BufferId, block_ids: Vec<CustomBlockId>) {
        self.block_ids.insert(buffer_id, block_ids);
    }

    pub fn get_block_ids(&self, buffer_id: &BufferId) -> Option<&Vec<CustomBlockId>> {
        self.block_ids.get(buffer_id)
    }

    #[allow(dead_code)]
    pub fn remove_buffer(&mut self, buffer_id: &BufferId) {
        self.lenses.remove(buffer_id);
        self.pending_refresh.remove(buffer_id);
        self.block_ids.remove(buffer_id);
    }

    pub fn set_refresh_task(&mut self, buffer_id: BufferId, task: Task<()>) {
        self.pending_refresh.insert(buffer_id, task);
    }

    pub fn remove_refresh_task(&mut self, buffer_id: &BufferId) {
        self.pending_refresh.remove(buffer_id);
    }
}

fn group_lenses_by_row(
    lenses: Vec<(Anchor, CodeLensItem)>,
    snapshot: &MultiBufferSnapshot,
) -> Vec<CodeLensData> {
    let mut grouped: HashMap<u32, (Anchor, Vec<CodeLensItem>)> = HashMap::default();

    for (position, item) in lenses {
        let row = position.to_point(snapshot).row;
        grouped
            .entry(row)
            .or_insert_with(|| (position, Vec::new()))
            .1
            .push(item);
    }

    let mut result: Vec<CodeLensData> = grouped
        .into_iter()
        .map(|(_, (position, items))| CodeLensData { position, items })
        .collect();

    result.sort_by_key(|lens| lens.position.to_point(snapshot).row);
    result
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CodeLensKind {
    References,
    Implementations,
    Other,
}

fn detect_lens_kind(title: &str) -> CodeLensKind {
    let title_lower = title.to_lowercase();
    if title_lower.contains("reference") {
        CodeLensKind::References
    } else if title_lower.contains("implementation") {
        CodeLensKind::Implementations
    } else {
        CodeLensKind::Other
    }
}

fn render_code_lens_line(
    lens: CodeLensData,
    editor: WeakEntity<Editor>,
) -> impl Fn(&mut crate::display_map::BlockContext) -> gpui::AnyElement {
    move |cx| {
        let mut children: Vec<gpui::AnyElement> = Vec::new();

        for (i, item) in lens.items.iter().enumerate() {
            if i > 0 {
                children.push(
                    div()
                        .text_ui_xs(cx.app)
                        .text_color(cx.app.theme().colors().text_muted)
                        .child(" | ")
                        .into_any_element(),
                );
            }

            let text = item.text.clone();
            let action = item.action.clone();
            let editor_clone = editor.clone();
            let position = lens.position;

            children.push(
                div()
                    .id(SharedString::from(format!("code-lens-{}-{}", i, text)))
                    .text_ui_xs(cx.app)
                    .text_color(cx.app.theme().colors().text_muted)
                    .cursor_pointer()
                    .hover(|style| style.text_color(cx.app.theme().colors().text))
                    .child(text.clone())
                    .on_click({
                        let text = text.clone();
                        move |_event, window, cx| {
                            let kind = detect_lens_kind(&text);
                            if let Some(editor) = editor_clone.upgrade() {
                                editor.update(cx, |editor, cx| {
                                    editor.change_selections(
                                        SelectionEffects::default(),
                                        window,
                                        cx,
                                        |s| {
                                            s.select_anchor_ranges([position..position]);
                                        },
                                    );

                                    match kind {
                                        CodeLensKind::References => {
                                            let _ = editor.find_all_references(
                                                &FindAllReferences::default(),
                                                window,
                                                cx,
                                            );
                                        }
                                        CodeLensKind::Implementations => {
                                            let _ = editor.go_to_implementation(
                                                &GoToImplementation,
                                                window,
                                                cx,
                                            );
                                        }
                                        CodeLensKind::Other => {
                                            if let Some(action) = &action {
                                                if let Some(workspace) = editor.workspace() {
                                                    let project =
                                                        workspace.read(cx).project().clone();
                                                    let action = action.clone();
                                                    let buffer = editor.buffer().clone();
                                                    if let Some(excerpt_buffer) =
                                                        buffer.read(cx).as_singleton()
                                                    {
                                                        let _ =
                                                            project.update(cx, |project, cx| {
                                                                project.apply_code_action(
                                                                    excerpt_buffer.clone(),
                                                                    action,
                                                                    true,
                                                                    cx,
                                                                )
                                                            });
                                                    }
                                                }
                                            }
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
            .pl(cx.anchor_x)
            .flex()
            .flex_row()
            .children(children)
            .into_any_element()
    }
}

impl Editor {
    pub fn code_lens_enabled(&self, cx: &App) -> bool {
        crate::EditorSettings::get_global(cx).code_lens.enabled
    }

    pub fn refresh_code_lenses(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<()>> {
        if !self.code_lens_enabled(cx) {
            return None;
        }

        let buffer = self.buffer().read(cx);
        let excerpt_buffer = match buffer.as_singleton() {
            Some(b) => b,
            None => return None,
        };
        let buffer_id = excerpt_buffer.read(cx).remote_id();
        let excerpt_buffer = excerpt_buffer.clone();

        let Some(project) = self.project.clone() else {
            return None;
        };

        let text_range = text::Anchor::MIN..text::Anchor::MAX;
        let multibuffer = self.buffer().clone();

        let task = cx.spawn_in(window, async move |editor, cx| {
            let actions_task = project.update(cx, |project, cx| {
                project.code_lens_actions::<text::Anchor>(&excerpt_buffer, text_range.clone(), cx)
            });

            let actions: anyhow::Result<Option<Vec<CodeAction>>> = actions_task.await;

            if let Ok(Some(actions)) = actions {
                let lenses = multibuffer.update(cx, |multibuffer, cx| {
                    let snapshot = multibuffer.snapshot(cx);

                    let individual_lenses: Vec<(Anchor, CodeLensItem)> = actions
                        .into_iter()
                        .filter_map(|action| {
                            let position = snapshot.anchor_in_excerpt(
                                snapshot.excerpts().next()?.0,
                                action.range.start,
                            )?;

                            let text = match &action.lsp_action {
                                project::LspAction::CodeLens(lens) => {
                                    lens.command.as_ref().map(|cmd| cmd.title.clone())
                                }
                                _ => None,
                            };

                            text.map(|text| {
                                (
                                    position,
                                    CodeLensItem {
                                        text: text.into(),
                                        action: Some(action),
                                    },
                                )
                            })
                        })
                        .collect();

                    group_lenses_by_row(individual_lenses, &snapshot)
                });

                if let Err(_) = editor.update(cx, |editor, cx| {
                    if let Some(old_block_ids) = editor.code_lens_cache.get_block_ids(&buffer_id) {
                        editor.remove_blocks(old_block_ids.iter().copied().collect(), None, cx);
                    }

                    editor
                        .code_lens_cache
                        .set_lenses_for_buffer(buffer_id, lenses.clone());

                    let editor_handle = cx.entity().downgrade();

                    let blocks = lenses
                        .into_iter()
                        .map(|lens| {
                            let position = lens.position;
                            let render_fn = render_code_lens_line(lens, editor_handle.clone());
                            BlockProperties {
                                placement: BlockPlacement::Above(position),
                                height: Some(1),
                                style: BlockStyle::Sticky,
                                render: std::sync::Arc::new(render_fn),
                                priority: 0,
                            }
                        })
                        .collect::<Vec<_>>();

                    let block_ids = editor.insert_blocks(blocks, None, cx);
                    editor.code_lens_cache.set_block_ids(buffer_id, block_ids);
                    cx.notify();
                }) {
                    editor
                        .update(cx, |editor, _cx| {
                            editor.code_lens_cache.remove_refresh_task(&buffer_id);
                        })
                        .ok();
                    return;
                }
            }

            editor
                .update(cx, |editor, _cx| {
                    editor.code_lens_cache.remove_refresh_task(&buffer_id);
                })
                .ok();
        });

        self.code_lens_cache.set_refresh_task(buffer_id, task);
        None
    }

    pub fn toggle_code_lenses(
        &mut self,
        _: &crate::actions::ToggleCodeLens,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let enabled = !self.code_lens_cache.enabled();
        if self.code_lens_cache.toggle(enabled) {
            if enabled {
                self.refresh_code_lenses(window, cx);
            } else {
                let all_block_ids: Vec<CustomBlockId> = self
                    .code_lens_cache
                    .block_ids
                    .values()
                    .flat_map(|ids| ids.iter().copied())
                    .collect();
                if !all_block_ids.is_empty() {
                    self.remove_blocks(all_block_ids.into_iter().collect(), None, cx);
                }
            }
            cx.notify();
        }
    }

    pub fn get_code_lenses_for_visible_range(
        &self,
        range: Range<Anchor>,
        cx: &App,
    ) -> Vec<CodeLensData> {
        if !self.code_lens_enabled(cx) {
            return Vec::new();
        }

        let buffer = self.buffer().read(cx);
        let Some(excerpt_buffer) = buffer.as_singleton() else {
            return Vec::new();
        };

        let buffer_id = excerpt_buffer.read(cx).remote_id();
        let snapshot = buffer.snapshot(cx);

        let Some(lenses) = self.code_lens_cache.get_lenses_for_buffer(buffer_id) else {
            return Vec::new();
        };

        let start_point = range.start.to_point(&snapshot);
        let end_point = range.end.to_point(&snapshot);

        lenses
            .iter()
            .filter(|lens| {
                let point = lens.position.to_point(&snapshot);
                point.row >= start_point.row && point.row <= end_point.row
            })
            .cloned()
            .collect()
    }
}
