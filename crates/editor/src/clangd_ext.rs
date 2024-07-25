use std::path::PathBuf;

use anyhow::Context as _;
use gpui::{View, ViewContext, WindowContext};
use language::Language;
use url::Url;

use crate::{element::register_action, Editor, SwitchSourceHeader};

pub fn apply_related_actions(editor: &View<Editor>, cx: &mut WindowContext) {
    let is_c_related = editor.update(cx, |editor, cx| {
        editor
            .buffer()
            .read(cx)
            .all_buffers()
            .iter()
            .any(|b| match b.read(cx).language() {
                Some(l) => is_c_language(l),
                None => false,
            })
    });

    if is_c_related {
        register_action(editor, cx, switch_source_header);
    }
}

pub fn switch_source_header(
    editor: &mut Editor,
    _: &SwitchSourceHeader,
    cx: &mut ViewContext<'_, Editor>,
) {
    let Some(project) = &editor.project else {
        return;
    };
    let Some(workspace) = editor.workspace() else {
        return;
    };

    let multibuffer = editor.buffer().read(cx);

    let Some((server_to_query, buffer)) = editor
        .selections
        .disjoint_anchors()
        .into_iter()
        .filter(|selection| selection.start == selection.end)
        .filter_map(|selection| Some((selection.start.buffer_id?, selection.start)))
        .filter_map(|(buffer_id, trigger_anchor)| {
            let buffer = multibuffer.buffer(buffer_id)?;
            let c_language = buffer.read(cx).language_at(trigger_anchor.text_anchor)?;
            if !is_c_language(&c_language) {
                return None;
            }
            Some(buffer)
        })
        .find_map(|buffer| {
            project
                .read(cx)
                .language_servers_for_buffer(buffer.read(cx), cx)
                .find_map(|(adapter, server)| {
                    if adapter.name.0.as_ref() == "clangd" {
                        Some((server.server_id(), buffer.clone()))
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
    let source_file = buffer_snapshot
        .file()
        .unwrap()
        .file_name(cx)
        .to_str()
        .unwrap()
        .to_owned();

    let switch_source_header_task = project.update(cx, |project, cx| {
        project.request_lsp(
            buffer,
            project::LanguageServerToQuery::Other(server_to_query),
            project::lsp_ext_command::SwitchSourceHeader,
            cx,
        )
    });
    cx.spawn(|_editor, mut cx| async move {
        let switch_source_header = switch_source_header_task
            .await
            .with_context(|| format!("Switch source/header LSP request for path \"{}\" failed", source_file))?;
        if switch_source_header.0.is_empty() {
            log::info!("Clangd returned an empty string when requesting to switch source/header from \"{}\"", source_file);
            return Ok(());
        }

        let goto = Url::parse(&switch_source_header.0).with_context(|| {
            format!(
                "Parsing URL \"{}\" returned from switch source/header failed",
                switch_source_header.0
            )
        })?;

        workspace
            .update(&mut cx, |workspace, view_cx| {
                workspace.open_abs_path(PathBuf::from(goto.path()), false, view_cx)
            })
            .with_context(|| {
                format!(
                    "Switch source/header could not open \"{}\" in workspace",
                    goto.path()
                )
            })?
            .await
            .map(|_| ())
    })
    .detach_and_log_err(cx);
}

fn is_c_language(language: &Language) -> bool {
    return language.name().as_ref() == "C++" || language.name().as_ref() == "C";
}
