use std::collections::hash_map::Entry;
use std::sync::Arc;

use crate::Editor;
use collections::HashMap;
use gpui::{Model, WindowContext};
use language::Buffer;
use language::Language;
use lsp::LanguageServerId;
use multi_buffer::Anchor;

pub(crate) fn find_specific_language_server_in_selection<F>(
    editor: &Editor,
    cx: &WindowContext,
    filter_language: F,
    language_server_name: &str,
) -> Option<(Anchor, Arc<Language>, LanguageServerId, Model<Buffer>)>
where
    F: Fn(&Language) -> bool,
{
    let Some(project) = &editor.project else {
        return None;
    };
    let multibuffer = editor.buffer().read(cx);
    let mut language_servers_for = HashMap::default();
    editor
        .selections
        .disjoint_anchors()
        .iter()
        .filter(|selection| selection.start == selection.end)
        .filter_map(|selection| Some((selection.start.buffer_id?, selection.start)))
        .filter_map(|(buffer_id, trigger_anchor)| {
            let buffer = multibuffer.buffer(buffer_id)?;
            let server_id = *match language_servers_for.entry(buffer_id) {
                Entry::Occupied(occupied_entry) => occupied_entry.into_mut(),
                Entry::Vacant(vacant_entry) => {
                    let language_server_id = project
                        .read(cx)
                        .language_servers_for_local_buffer(buffer.read(cx), cx)
                        .find_map(|(adapter, server)| {
                            if adapter.name.0.as_ref() == language_server_name {
                                Some(server.server_id())
                            } else {
                                None
                            }
                        });
                    vacant_entry.insert(language_server_id)
                }
            }
            .as_ref()?;

            Some((buffer, trigger_anchor, server_id))
        })
        .find_map(|(buffer, trigger_anchor, server_id)| {
            let language = buffer.read(cx).language_at(trigger_anchor.text_anchor)?;
            if !filter_language(&language) {
                return None;
            }
            Some((
                trigger_anchor,
                Arc::clone(&language),
                server_id,
                buffer.clone(),
            ))
        })
}
