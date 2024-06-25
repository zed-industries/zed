use std::sync::Arc;

use anyhow::Context as _;
use gpui::{Context, View, ViewContext, VisualContext, WindowContext};
use language::Language;
use multi_buffer::MultiBuffer;
use project::lsp_ext_command::ExpandMacro;
use text::ToPointUtf16;

use crate::{element::register_action, Editor, ExpandMacroRecursively};

pub fn apply_related_actions(editor: &View<Editor>, cx: &mut WindowContext) {
    let is_rust_related = editor.update(cx, |editor, cx| {
        editor
            .buffer()
            .read(cx)
            .all_buffers()
            .iter()
            .any(|b| match b.read(cx).language() {
                Some(l) => is_rust_language(l),
                None => false,
            })
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
    let Some(workspace) = editor.workspace() else {
        return;
    };

    let multibuffer = editor.buffer().read(cx);

    let Some((trigger_anchor, rust_language, server_to_query, buffer)) = editor
        .selections
        .disjoint_anchors()
        .into_iter()
        .filter(|selection| selection.start == selection.end)
        .filter_map(|selection| Some((selection.start.buffer_id?, selection.start)))
        .filter_map(|(buffer_id, trigger_anchor)| {
            let buffer = multibuffer.buffer(buffer_id)?;
            let rust_language = buffer.read(cx).language_at(trigger_anchor.text_anchor)?;
            if !is_rust_language(&rust_language) {
                return None;
            }
            Some((trigger_anchor, rust_language, buffer))
        })
        .find_map(|(trigger_anchor, rust_language, buffer)| {
            project
                .read(cx)
                .language_servers_for_buffer(buffer.read(cx), cx)
                .find_map(|(adapter, server)| {
                    if adapter.name.0.as_ref() == "rust-analyzer" {
                        Some((
                            trigger_anchor,
                            Arc::clone(&rust_language),
                            server.server_id(),
                            buffer.clone(),
                        ))
                    } else {
                        None
                    }
                })
        })
    else {
        return;
    };

    let project = project.clone();
    let buffer_snapshot = buffer.read(cx).snapshot();
    let position = trigger_anchor.text_anchor.to_point_utf16(&buffer_snapshot);
    let expand_macro_task = project.update(cx, |project, cx| {
        project.request_lsp(
            buffer,
            project::LanguageServerToQuery::Other(server_to_query),
            ExpandMacro { position },
            cx,
        )
    });
    cx.spawn(|_editor, mut cx| async move {
        let macro_expansion = expand_macro_task.await.context("expand macro")?;
        if macro_expansion.is_empty() {
            log::info!("Empty macro expansion for position {position:?}");
            return Ok(());
        }

        let buffer = project
            .update(&mut cx, |project, cx| project.create_buffer(cx))?
            .await?;
        workspace.update(&mut cx, |workspace, cx| {
            buffer.update(cx, |buffer, cx| {
                buffer.edit([(0..0, macro_expansion.expansion)], None, cx);
                buffer.set_language(Some(rust_language), cx)
            });
            let multibuffer = cx.new_model(|cx| {
                MultiBuffer::singleton(buffer, cx).with_title(macro_expansion.name)
            });
            workspace.add_item_to_active_pane(
                Box::new(
                    cx.new_view(|cx| Editor::for_multibuffer(multibuffer, Some(project), true, cx)),
                ),
                None,
                cx,
            );
        })
    })
    .detach_and_log_err(cx);
}

fn is_rust_language(language: &Language) -> bool {
    language.name().as_ref() == "Rust"
}
