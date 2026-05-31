use super::*;
use gpui::{Context, Global, WeakEntity, Window};
use std::time::{Duration, Instant};

use crate::actions::{GoToNextGlobalChange, GoToPreviousGlobalChange};
use workspace::Workspace;

/// An entry in the global change list, tracking changes across all editors.
#[derive(Clone)]
pub(crate) struct GlobalChangeEntry {
    pub(crate) editor: WeakEntity<Editor>,
    pub(crate) project_path: Option<ProjectPath>,
    pub(crate) anchors: Vec<Anchor>,
    pub(crate) points: Vec<Point>,
    pub(crate) timestamp: Instant,
}

const GLOBAL_CHANGE_GROUPING_THRESHOLD: Duration = Duration::from_millis(300);

/// A global list of changes across all editors in the workspace.
#[derive(Default)]
pub(crate) struct GlobalChangeList {
    pub(crate) changes: Vec<GlobalChangeEntry>,
    pub(crate) position: Option<usize>,
}

impl Global for GlobalChangeList {}

/// Pushes the current change to the global change list.
pub fn push_to_global_change_list(
    editor: &Editor,
    group: bool,
    anchors: Vec<Anchor>,
    cx: &mut Context<Editor>,
) {
    let project_path = editor.project_path(cx);
    let buffer = editor.buffer.read(cx).snapshot(cx);
    let points = anchors.iter().map(|a| a.to_point(&buffer)).collect();
    let now = Instant::now();

    let entry = GlobalChangeEntry {
        editor: cx.entity().downgrade(),
        project_path,
        anchors,
        points,
        timestamp: now,
    };

    cx.update_global::<GlobalChangeList, _>(|list, _| {
        list.position = None;

        let should_group = group
            || list
                .changes
                .last()
                .is_some_and(|last| {
                    last.editor == entry.editor
                        && now.duration_since(last.timestamp) < GLOBAL_CHANGE_GROUPING_THRESHOLD
                });

        if should_group {
            if let Some(last) = list.changes.last_mut() {
                if last.editor == entry.editor {
                    *last = entry;
                    return;
                }
            }
        }

        list.changes.retain(|existing| {
            if existing.editor == entry.editor {
                if existing.anchors.len() == entry.anchors.len() {
                    return !existing.anchors.iter().zip(entry.anchors.iter()).all(|(a1, a2)| {
                        a1.to_point(&buffer).row == a2.to_point(&buffer).row
                    });
                }
            } else if existing.project_path == entry.project_path
                && entry.project_path.is_some()
            {
                if existing.points.len() == entry.points.len() {
                    return !existing
                        .points
                        .iter()
                        .zip(entry.points.iter())
                        .all(|(p1, p2)| p1.row == p2.row);
                }
            }
            true
        });

        list.changes.push(entry);
    });
}

pub fn go_to_next_global_change(
    workspace: &mut Workspace,
    _: &GoToNextGlobalChange,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    navigate_global_change_list(workspace, workspace::searchable::Direction::Next, window, cx);
}

pub fn go_to_previous_global_change(
    workspace: &mut Workspace,
    _: &GoToPreviousGlobalChange,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    navigate_global_change_list(workspace, workspace::searchable::Direction::Prev, window, cx);
}

fn navigate_global_change_list(
    workspace: &mut Workspace,
    direction: workspace::searchable::Direction,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let current_cursor_info = workspace
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx))
        .and_then(|editor_handle| {
            editor_handle.update(cx, |editor, cx| {
                let point = editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx))
                    .head();
                let project_path = editor.project_path(cx);
                Some((editor_handle.clone(), project_path, point))
            })
        });

    loop {
        let entry_opt = cx.update_global::<GlobalChangeList, _>(|list, _| {
            if list.changes.is_empty() {
                return None;
            }

            let current_position = list.position.unwrap_or(list.changes.len());
            let new_position = match direction {
                workspace::searchable::Direction::Next => {
                    (current_position + 1).min(list.changes.len() - 1)
                }
                workspace::searchable::Direction::Prev => current_position.saturating_sub(1),
            };

            if list.position == Some(new_position) {
                return None;
            }

            list.position = Some(new_position);
            Some(list.changes[new_position].clone())
        });

        let Some(entry) = entry_opt else {
            return;
        };

        let cursor_already_at_entry = current_cursor_info.as_ref().is_some_and(
            |(active_editor, active_path, cursor_point)| {
                if let Some(entry_editor) = entry.editor.upgrade() {
                    if entry_editor == *active_editor {
                        return entry.points.contains(cursor_point);
                    }
                }

                if let (Some(p1), Some(p2)) = (active_path, &entry.project_path) {
                    if p1 == p2 {
                        return entry.points.contains(cursor_point);
                    }
                }

                false
            },
        );

        if cursor_already_at_entry {
            continue;
        }

        if let Some(editor) = entry.editor.upgrade() {
            workspace.activate_item(&editor, true, true, window, cx);
            editor.update(cx, |editor, cx| {
                editor.change_selections(Default::default(), window, cx, |s| {
                    let map = s.display_snapshot();
                    s.select_display_ranges(
                        entry
                            .anchors
                            .iter()
                            .map(|a| a.to_display_point(&map)..a.to_display_point(&map)),
                    )
                });
            });
        } else if let Some(project_path) = entry.project_path {
            let points = entry.points.clone();
            let open_task = workspace.open_path(project_path, None, true, window, cx);
            cx.spawn_in(window, async move |_workspace, cx| {
                let item = open_task.await?;
                if let Some(editor) = item.downcast::<Editor>() {
                    editor.update_in(cx, |editor, window, cx| {
                        editor.change_selections(Default::default(), window, cx, |s| {
                            s.select_ranges(points.iter().map(|&point| point..point))
                        });
                    })?;
                }
                anyhow::Ok(())
            })
            .detach();
        } else {
            continue;
        }

        break;
    }
}
