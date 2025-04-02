use std::{fs, path::Path};

use anyhow::Context as _;
use gpui::{App, AppContext as _, Context, Entity, Window};
use language::{Capability, Language, proto::serialize_anchor};
use multi_buffer::MultiBuffer;
use project::lsp_store::{
    lsp_ext_command::{DocsUrls, ExpandMacro, ExpandedMacro},
    rust_analyzer_ext::RUST_ANALYZER_NAME,
};
use rpc::proto;
use text::ToPointUtf16;

use crate::{
    Editor, ExpandMacroRecursively, OpenDocs, element::register_action,
    lsp_ext::find_specific_language_server_in_selection,
};

fn is_rust_language(language: &Language) -> bool {
    language.name() == "Rust".into()
}

pub fn apply_related_actions(editor: &Entity<Editor>, window: &mut Window, cx: &mut App) {
    if editor
        .read(cx)
        .buffer()
        .read(cx)
        .all_buffers()
        .into_iter()
        .filter_map(|buffer| buffer.read(cx).language())
        .any(|language| is_rust_language(language))
    {
        register_action(&editor, window, expand_macro_recursively);
        register_action(&editor, window, open_docs);
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

    let server_lookup = find_specific_language_server_in_selection(
        editor,
        cx,
        is_rust_language,
        RUST_ANALYZER_NAME,
    );

    let project = project.clone();
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    cx.spawn_in(window, async move |_editor, cx| {
        let Some((trigger_anchor, rust_language, server_to_query, buffer)) = server_lookup.await
        else {
            return Ok(());
        };

        let macro_expansion = if let Some((client, project_id)) = upstream_client {
            let buffer_id = buffer.update(cx, |buffer, _| buffer.remote_id())?;
            let request = proto::LspExtExpandMacro {
                project_id,
                buffer_id: buffer_id.to_proto(),
                position: Some(serialize_anchor(&trigger_anchor.text_anchor)),
            };
            let response = client
                .request(request)
                .await
                .context("lsp ext expand macro proto request")?;
            ExpandedMacro {
                name: response.name,
                expansion: response.expansion,
            }
        } else {
            let buffer_snapshot = buffer.update(cx, |buffer, _| buffer.snapshot())?;
            let position = trigger_anchor.text_anchor.to_point_utf16(&buffer_snapshot);
            project
                .update(cx, |project, cx| {
                    project.request_lsp(
                        buffer,
                        project::LanguageServerToQuery::Other(server_to_query),
                        ExpandMacro { position },
                        cx,
                    )
                })?
                .await
                .context("expand macro")?
        };

        if macro_expansion.is_empty() {
            log::info!(
                "Empty macro expansion for position {:?}",
                trigger_anchor.text_anchor
            );
            return Ok(());
        }

        let buffer = project
            .update(cx, |project, cx| project.create_buffer(cx))?
            .await?;
        workspace.update_in(cx, |workspace, window, cx| {
            buffer.update(cx, |buffer, cx| {
                buffer.set_text(macro_expansion.expansion, cx);
                buffer.set_language(Some(rust_language), cx);
                buffer.set_capability(Capability::ReadOnly, cx);
            });
            let multibuffer =
                cx.new(|cx| MultiBuffer::singleton(buffer, cx).with_title(macro_expansion.name));
            workspace.add_item_to_active_pane(
                Box::new(cx.new(|cx| {
                    let mut editor = Editor::for_multibuffer(multibuffer, None, window, cx);
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

    let server_lookup = find_specific_language_server_in_selection(
        editor,
        cx,
        is_rust_language,
        RUST_ANALYZER_NAME,
    );

    let project = project.clone();
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    cx.spawn_in(window, async move |_editor, cx| {
        let Some((trigger_anchor, _, server_to_query, buffer)) = server_lookup.await else {
            return Ok(());
        };

        let docs_urls = if let Some((client, project_id)) = upstream_client {
            let buffer_id = buffer.update(cx, |buffer, _| buffer.remote_id())?;
            let request = proto::LspExtOpenDocs {
                project_id,
                buffer_id: buffer_id.to_proto(),
                position: Some(serialize_anchor(&trigger_anchor.text_anchor)),
            };
            let response = client
                .request(request)
                .await
                .context("lsp ext open docs proto request")?;
            DocsUrls {
                web: response.web,
                local: response.local,
            }
        } else {
            let buffer_snapshot = buffer.update(cx, |buffer, _| buffer.snapshot())?;
            let position = trigger_anchor.text_anchor.to_point_utf16(&buffer_snapshot);
            project
                .update(cx, |project, cx| {
                    project.request_lsp(
                        buffer,
                        project::LanguageServerToQuery::Other(server_to_query),
                        project::lsp_store::lsp_ext_command::OpenDocs { position },
                        cx,
                    )
                })?
                .await
                .context("open docs")?
        };

        if docs_urls.is_empty() {
            log::debug!(
                "Empty docs urls for position {:?}",
                trigger_anchor.text_anchor
            );
            return Ok(());
        }

        workspace.update(cx, |_workspace, cx| {
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
