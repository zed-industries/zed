use std::sync::Arc;

use crate::Editor;
use gpui::{App, AppContext as _, Entity, Task};
use itertools::Itertools;
use language::Buffer;
use language::Language;
use lsp::LanguageServerId;
use multi_buffer::Anchor;

pub(crate) fn find_specific_language_server_in_selection<F>(
    editor: &Editor,
    cx: &mut App,
    filter_language: F,
    language_server_name: &str,
) -> Task<Option<(Anchor, Arc<Language>, LanguageServerId, Entity<Buffer>)>>
where
    F: Fn(&Language) -> bool,
{
    let Some(project) = &editor.project else {
        return Task::ready(None);
    };

    let applicable_buffers = editor
        .selections
        .disjoint_anchors()
        .iter()
        .filter(|selection| selection.start == selection.end)
        .filter_map(|selection| Some((selection.start, selection.start.buffer_id?)))
        .filter_map(|(trigger_anchor, buffer_id)| {
            let buffer = editor.buffer().read(cx).buffer(buffer_id)?;
            let language = buffer.read(cx).language_at(trigger_anchor.text_anchor)?;
            if filter_language(&language) {
                Some((trigger_anchor, buffer, language))
            } else {
                None
            }
        })
        .unique_by(|(_, buffer, _)| buffer.read(cx).remote_id())
        .collect::<Vec<_>>();

    let applicable_buffer_tasks = applicable_buffers
        .into_iter()
        .map(|(trigger_anchor, buffer, language)| {
            let task = buffer.update(cx, |buffer, cx| {
                project.update(cx, |project, cx| {
                    project.language_server_id_for_name(buffer, language_server_name, cx)
                })
            });
            (trigger_anchor, buffer, language, task)
        })
        .collect::<Vec<_>>();
    cx.background_spawn(async move {
        for (trigger_anchor, buffer, language, task) in applicable_buffer_tasks {
            if let Some(server_id) = task.await {
                return Some((trigger_anchor, language, server_id, buffer));
            }
        }

        None
    })
}
