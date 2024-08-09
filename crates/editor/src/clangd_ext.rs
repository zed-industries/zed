use std::path::PathBuf;

use anyhow::Context as _;
use gpui::{View, ViewContext, WindowContext};
use language::Language;
use url::Url;

use crate::lsp_ext::find_specific_language_server_in_selection;

use crate::{element::register_action, Editor, SwitchSourceHeader};

static CLANGD_SERVER_NAME: &str = "clangd";

fn is_c_language(language: &Language) -> bool {
    return language.name().as_ref() == "C++" || language.name().as_ref() == "C";
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

    let Some((_, _, server_to_query, buffer)) =
        find_specific_language_server_in_selection(&editor, cx, &is_c_language, CLANGD_SERVER_NAME)
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

pub fn apply_related_actions(editor: &View<Editor>, cx: &mut WindowContext) {
    if editor.update(cx, |e, cx| {
        find_specific_language_server_in_selection(e, cx, &is_c_language, CLANGD_SERVER_NAME)
            .is_some()
    }) {
        register_action(editor, cx, switch_source_header);
    }
}
