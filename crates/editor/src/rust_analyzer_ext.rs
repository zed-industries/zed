use std::{fs, path::Path};

use anyhow::Context as _;
use gpui::{App, AppContext as _, Context, Entity, Window};
use language::{Capability, Language};
use multi_buffer::MultiBuffer;
use project::lsp_ext_command::ExpandMacro;
use text::ToPointUtf16;

use crate::{
    element::register_action, lsp_ext::find_specific_language_server_in_selection, Editor,
    ExpandMacroRecursively, OpenDocs,
};

const RUST_ANALYZER_NAME: &str = "rust-analyzer";

fn is_rust_language(language: &Language) -> bool {
    language.name() == "Rust".into()
}

pub fn apply_related_actions(editor: &Entity<Editor>, window: &mut Window, cx: &mut App) {
    if editor
        .update(cx, |e, cx| {
            find_specific_language_server_in_selection(e, cx, is_rust_language, RUST_ANALYZER_NAME)
        })
        .is_some()
    {
        register_action(editor, window, expand_macro_recursively);
        register_action(editor, window, open_docs);
    }
}

pub fn expand_macro_recursively(
    editor: &mut Editor,
    _: &ExpandMacroRecursively,
    window: &mut Window,
    cx: &mut Context<Editor>,
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

    let Some((trigger_anchor, rust_language, server_to_query, buffer)) =
        find_specific_language_server_in_selection(
            editor,
            cx,
            is_rust_language,
            RUST_ANALYZER_NAME,
        )
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
    cx.spawn_in(window, |_editor, mut cx| async move {
        let macro_expansion = expand_macro_task.await.context("expand macro")?;
        if macro_expansion.is_empty() {
            log::info!("Empty macro expansion for position {position:?}");
            return Ok(());
        }

        let buffer = project
            .update(&mut cx, |project, cx| project.create_buffer(cx))?
            .await?;
        workspace.update_in(&mut cx, |workspace, window, cx| {
            buffer.update(cx, |buffer, cx| {
                buffer.set_text(macro_expansion.expansion, cx);
                buffer.set_language(Some(rust_language), cx);
                buffer.set_capability(Capability::ReadOnly, cx);
            });
            let multibuffer =
                cx.new(|cx| MultiBuffer::singleton(buffer, cx).with_title(macro_expansion.name));
            workspace.add_item_to_active_pane(
                Box::new(cx.new(|cx| {
                    let mut editor = Editor::for_multibuffer(multibuffer, None, false, window, cx);
                    editor.set_read_only(true);
                    editor
                })),
                None,
                true,
                window,
                cx,
            );
        })
    })
    .detach_and_log_err(cx);
}

pub fn open_docs(editor: &mut Editor, _: &OpenDocs, window: &mut Window, cx: &mut Context<Editor>) {
    if editor.selections.count() == 0 {
        return;
    }
    let Some(project) = &editor.project else {
        return;
    };
    let Some(workspace) = editor.workspace() else {
        return;
    };

    let Some((trigger_anchor, _rust_language, server_to_query, buffer)) =
        find_specific_language_server_in_selection(
            editor,
            cx,
            is_rust_language,
            RUST_ANALYZER_NAME,
        )
    else {
        return;
    };

    let project = project.clone();
    let buffer_snapshot = buffer.read(cx).snapshot();
    let position = trigger_anchor.text_anchor.to_point_utf16(&buffer_snapshot);
    let open_docs_task = project.update(cx, |project, cx| {
        project.request_lsp(
            buffer,
            project::LanguageServerToQuery::Other(server_to_query),
            project::lsp_ext_command::OpenDocs { position },
            cx,
        )
    });

    cx.spawn_in(window, |_editor, mut cx| async move {
        let docs_urls = open_docs_task.await.context("open docs")?;
        if docs_urls.is_empty() {
            log::debug!("Empty docs urls for position {position:?}");
            return Ok(());
        } else {
            log::debug!("{:?}", docs_urls);
        }

        workspace.update(&mut cx, |_workspace, cx| {
            // Check if the local document exists, otherwise fallback to the online document.
            // Open with the default browser.
            if let Some(local_url) = docs_urls.local {
                if fs::metadata(Path::new(&local_url[8..])).is_ok() {
                    cx.open_url(&local_url);
                    return;
                }
            }

            if let Some(web_url) = docs_urls.web {
                cx.open_url(&web_url);
            }
        })
    })
    .detach_and_log_err(cx);
}
