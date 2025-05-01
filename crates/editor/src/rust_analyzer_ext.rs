use std::{fs, path::Path};

use anyhow::Context as _;
use gpui::{App, AppContext as _, Context, Entity, Window};
use language::{Capability, Language, proto::serialize_anchor};
use multi_buffer::MultiBuffer;
use project::{
    lsp_command::location_link_from_proto,
    lsp_store::{
        lsp_ext_command::{DocsUrls, ExpandMacro, ExpandedMacro},
        rust_analyzer_ext::RUST_ANALYZER_NAME,
    },
};
use rpc::proto;
use text::ToPointUtf16;

use crate::{
    CancelFlycheck, ClearFlycheck, Editor, ExpandMacroRecursively, GoToParentModule,
    GotoDefinitionKind, OpenDocs, RunFlycheck, element::register_action, hover_links::HoverLink,
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
        register_action(&editor, window, go_to_parent_module);
        register_action(&editor, window, expand_macro_recursively);
        register_action(&editor, window, open_docs);
        register_action(&editor, window, cancel_flycheck);
        register_action(&editor, window, run_flycheck);
        register_action(&editor, window, clear_flycheck);
    }
}

pub fn go_to_parent_module(
    editor: &mut Editor,
    _: &GoToParentModule,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    if editor.selections.count() == 0 {
        return;
    }
    let Some(project) = &editor.project else {
        return;
    };

    let server_lookup = find_specific_language_server_in_selection(
        editor,
        cx,
        is_rust_language,
        RUST_ANALYZER_NAME,
    );

    let project = project.clone();
    let lsp_store = project.read(cx).lsp_store();
    let upstream_client = lsp_store.read(cx).upstream_client();
    cx.spawn_in(window, async move |editor, cx| {
        let Some((trigger_anchor, _, server_to_query, buffer)) = server_lookup.await else {
            return anyhow::Ok(());
        };

        let location_links = if let Some((client, project_id)) = upstream_client {
            let buffer_id = buffer.update(cx, |buffer, _| buffer.remote_id())?;

            let request = proto::LspExtGoToParentModule {
                project_id,
                buffer_id: buffer_id.to_proto(),
                position: Some(serialize_anchor(&trigger_anchor.text_anchor)),
            };
            let response = client
                .request(request)
                .await
                .context("lsp ext go to parent module proto request")?;
            futures::future::join_all(
                response
                    .links
                    .into_iter()
                    .map(|link| location_link_from_proto(link, lsp_store.clone(), cx)),
            )
            .await
            .into_iter()
            .collect::<anyhow::Result<_>>()
            .context("go to parent module via collab")?
        } else {
            let buffer_snapshot = buffer.update(cx, |buffer, _| buffer.snapshot())?;
            let position = trigger_anchor.text_anchor.to_point_utf16(&buffer_snapshot);
            project
                .update(cx, |project, cx| {
                    project.request_lsp(
                        buffer,
                        project::LanguageServerToQuery::Other(server_to_query),
                        project::lsp_store::lsp_ext_command::GoToParentModule { position },
                        cx,
                    )
                })?
                .await
                .context("go to parent module")?
        };

        editor
            .update_in(cx, |editor, window, cx| {
                editor.navigate_to_hover_links(
                    Some(GotoDefinitionKind::Declaration),
                    location_links.into_iter().map(HoverLink::Text).collect(),
                    false,
                    window,
                    cx,
                )
            })?
            .await?;
        Ok(())
    })
    .detach_and_log_err(cx);
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

fn cancel_flycheck(
    editor: &mut Editor,
    _: &CancelFlycheck,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let Some(project) = &editor.project else {
        return;
    };
    let Some(buffer_id) = editor
        .selections
        .disjoint_anchors()
        .iter()
        .find_map(|selection| selection.start.buffer_id.or(selection.end.buffer_id))
        .map(|buffer_id| buffer_id.to_proto())
    else {
        return;
    };

    let rust_analyzer_server = project
        .read(cx)
        .language_server_with_name(RUST_ANALYZER_NAME, cx);
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    let lsp_store = project.read(cx).lsp_store();

    cx.spawn_in(window, async move |_editor, cx| {
        let Some(rust_analyzer_server) = rust_analyzer_server.await else {
            return Ok(());
        };

        if let Some((client, project_id)) = upstream_client {
            let request = proto::LspExtCancelFlycheck {
                project_id,
                buffer_id,
                language_server_id: rust_analyzer_server.to_proto(),
            };
            client
                .request(request)
                .await
                .context("lsp ext cancel flycheck proto request")?;
        } else {
            lsp_store
                .update(cx, |lsp_store, _| {
                    if let Some(server) = lsp_store.language_server_for_id(rust_analyzer_server) {
                        server
                            .notify::<project::lsp_store::lsp_ext_command::LspExtCancelFlycheck>(
                                &(),
                            )?;
                    }
                    anyhow::Ok(())
                })?
                .context("lsp ext cancel flycheck")?;
        };
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn run_flycheck(
    editor: &mut Editor,
    _: &RunFlycheck,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let Some(project) = &editor.project else {
        return;
    };
    let Some(buffer_id) = editor
        .selections
        .disjoint_anchors()
        .iter()
        .find_map(|selection| selection.start.buffer_id.or(selection.end.buffer_id))
        .map(|buffer_id| buffer_id.to_proto())
    else {
        return;
    };

    let rust_analyzer_server = project
        .read(cx)
        .language_server_with_name(RUST_ANALYZER_NAME, cx);
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    let lsp_store = project.read(cx).lsp_store();

    cx.spawn_in(window, async move |_editor, cx| {
        let Some(rust_analyzer_server) = rust_analyzer_server.await else {
            return Ok(());
        };

        if let Some((client, project_id)) = upstream_client {
            let request = proto::LspExtRunFlycheck {
                project_id,
                buffer_id,
                language_server_id: rust_analyzer_server.to_proto(),
                current_file_only: false,
            };
            client
                .request(request)
                .await
                .context("lsp ext run flycheck proto request")?;
        } else {
            lsp_store
                .update(cx, |lsp_store, _| {
                    if let Some(server) = lsp_store.language_server_for_id(rust_analyzer_server) {
                        server.notify::<project::lsp_store::lsp_ext_command::LspExtRunFlycheck>(
                            &project::lsp_store::lsp_ext_command::RunFlycheckParams {
                                text_document: None,
                            },
                        )?;
                    }
                    anyhow::Ok(())
                })?
                .context("lsp ext run flycheck")?;
        };
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn clear_flycheck(
    editor: &mut Editor,
    _: &ClearFlycheck,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let Some(project) = &editor.project else {
        return;
    };
    let Some(buffer_id) = editor
        .selections
        .disjoint_anchors()
        .iter()
        .find_map(|selection| selection.start.buffer_id.or(selection.end.buffer_id))
        .map(|buffer_id| buffer_id.to_proto())
    else {
        return;
    };

    let rust_analyzer_server = project
        .read(cx)
        .language_server_with_name(RUST_ANALYZER_NAME, cx);
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    let lsp_store = project.read(cx).lsp_store();

    cx.spawn_in(window, async move |_editor, cx| {
        let Some(rust_analyzer_server) = rust_analyzer_server.await else {
            return Ok(());
        };

        if let Some((client, project_id)) = upstream_client {
            let request = proto::LspExtClearFlycheck {
                project_id,
                buffer_id,
                language_server_id: rust_analyzer_server.to_proto(),
            };
            client
                .request(request)
                .await
                .context("lsp ext clear flycheck proto request")?;
        } else {
            lsp_store
                .update(cx, |lsp_store, _| {
                    if let Some(server) = lsp_store.language_server_for_id(rust_analyzer_server) {
                        server.notify::<project::lsp_store::lsp_ext_command::LspExtClearFlycheck>(
                            &(),
                        )?;
                    }
                    anyhow::Ok(())
                })?
                .context("lsp ext clear flycheck")?;
        };
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
