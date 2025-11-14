use std::ops::Range;

use collections::HashMap;
use gpui::{App, SharedString, Task};
use language::BufferId;
use multi_buffer::{Anchor, ToOffset as _};
use project::CodeAction;
use settings::Settings;
use text;
use ui::{Context, Window, div, prelude::*};

use crate::{
    Editor,
    display_map::{BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
};

#[derive(Clone, Debug)]
pub struct CodeLensData {
    pub position: Anchor,
    pub text: SharedString,
    pub action: Option<CodeAction>,
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
        let excerpt_buffer = buffer.as_singleton()?;
        let buffer_id = excerpt_buffer.read(cx).remote_id();

        let Some(project) = self.project.as_ref() else {
            return None;
        };

        let text_range = text::Anchor::MIN..text::Anchor::MAX;

        let project = project.clone();
        let excerpt_buffer = excerpt_buffer.clone();
        let multibuffer = self.buffer().clone();

        let task = cx.spawn_in(window, async move |editor, cx| {
            let actions_task = match project
                .update(cx, |project, cx| {
                    project.code_lens_actions::<text::Anchor>(&excerpt_buffer, text_range.clone(), cx)
                }) {
                Ok(task) => task,
                Err(_) => return,
            };

            let actions: anyhow::Result<Option<Vec<CodeAction>>> = actions_task.await;

            if let Ok(Some(actions)) = actions {
                let lenses: Vec<CodeLensData> = match multibuffer
                    .update(cx, |multibuffer, cx| -> Vec<CodeLensData> {
                        let snapshot = multibuffer.snapshot(cx);
                        actions
                            .into_iter()
                            .filter_map(|action| {
                                let position = snapshot
                                    .anchor_in_excerpt(snapshot.excerpts().next()?.0, action.range.start)?;

                                let text = match &action.lsp_action {
                                    project::LspAction::CodeLens(lens) => {
                                        if let Some(command) = &lens.command {
                                            Some(format!("↪ {}", command.title))
                                        } else {
                                            Some("↪ CodeLens".to_string())
                                        }
                                    }
                                    _ => None,
                                };

                                text.map(|text| CodeLensData {
                                    position,
                                    text: text.into(),
                                    action: Some(action),
                                })
                            })
                            .collect()
                    }) {
                    Ok(lenses) => lenses,
                    Err(_) => return,
                };

                editor
                    .update(cx, |editor, cx| {
                        if let Some(old_block_ids) = editor.code_lens_cache.get_block_ids(&buffer_id) {
                            editor.remove_blocks(old_block_ids.iter().copied().collect(), None, cx);
                        }

                        editor.code_lens_cache.set_lenses_for_buffer(buffer_id, lenses.clone());

                        let blocks = lenses
                            .into_iter()
                            .map(|lens| {
                                let text = lens.text.clone();
                                let position = lens.position;
                                BlockProperties {
                                    placement: BlockPlacement::Above(position),
                                    height: Some(1),
                                    style: BlockStyle::Sticky,
                                    render: std::sync::Arc::new(move |cx| {
                                        div()
                                            .text_ui_xs(cx.app)
                                            .text_color(cx.app.theme().colors().text_muted)
                                            .pl_8()
                                            .child(text.clone())
                                            .cursor_pointer()
                                            .hover(|style| {
                                                style.text_color(cx.app.theme().colors().text)
                                            })
                                            .into_any_element()
                                    }),
                                    priority: 0,
                                }
                            })
                            .collect::<Vec<_>>();

                        let block_ids = editor.insert_blocks(blocks, None, cx);
                        editor.code_lens_cache.set_block_ids(buffer_id, block_ids);
                        cx.notify();
                    })
                    .ok();
            }
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

        let start_offset = range.start.to_offset(&snapshot);
        let end_offset = range.end.to_offset(&snapshot);

        lenses
            .iter()
            .filter(|lens| {
                let offset = lens.position.to_offset(&snapshot);
                offset >= start_offset && offset <= end_offset
            })
            .cloned()
            .collect()
    }
}
