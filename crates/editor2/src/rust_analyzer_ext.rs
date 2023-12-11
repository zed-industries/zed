use std::{path::Path, sync::Arc};

use gpui::{AppContext, AsyncAppContext, Model, View, ViewContext, WindowContext};
use language::Buffer;
use lsp::{LanguageServer, LanguageServerId};
use project::{lsp_command::LspCommand, lsp_ext_command::ExpandMacro, Project};
use rpc::proto::{self, PeerId};
use serde::{Deserialize, Serialize};

use crate::{element::register_action, Editor, ExpandMacroRecursively};

pub fn apply_related_actions(editor: &View<Editor>, cx: &mut WindowContext) {
    let is_rust_related = editor.update(cx, |editor, cx| {
        editor
            .buffer()
            .read(cx)
            .all_buffers()
            .iter()
            .any(|b| b.read(cx).language().map(|l| l.name()).as_deref() == Some("Rust"))
    });

    if is_rust_related {
        register_action(editor, cx, expand_macro_recursively);
    }
}

pub fn expand_macro_recursively(
    editor: &mut Editor,
    _: &ExpandMacroRecursively,
    cx: &mut ViewContext<'_, Editor>,
) {
    if editor.selections.count() == 0 {
        return;
    }
    let Some(project) = &editor.project else {
        return;
    };

    let multibuffer = editor.buffer().read(cx);

    let Some((trigger_anchor, server_to_query, buffer)) = editor
        .selections
        .disjoint_anchors()
        .into_iter()
        .filter(|selection| selection.start == selection.end)
        .filter_map(|selection| Some((selection.start.buffer_id?, selection.start)))
        .find_map(|(buffer_id, trigger_anchor)| {
            let buffer = multibuffer.buffer(buffer_id)?;
            project
                .read(cx)
                .language_servers_for_buffer(buffer.read(cx), cx)
                .into_iter()
                .find_map(|(adapter, server)| {
                    if adapter.name.0.as_ref() == "rust-analyzer" {
                        Some((trigger_anchor, server.server_id(), buffer.clone()))
                    } else {
                        None
                    }
                })
        })
    else {
        return;
    };

    let z = project.update(cx, |project, cx| {
        project.request_lsp(
            buffer,
            project::LanguageServerToQuery::Other(server_to_query),
            ExpandMacro {},
            cx,
        )
    });

    // todo!("TODO kb")
}
