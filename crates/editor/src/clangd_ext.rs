use anyhow::Context as _;
use gpui::{App, Context, Entity, Window};
use language::Language;
use project::lsp_store::lsp_ext_command::SwitchSourceHeaderResult;
use rpc::proto;
use url::Url;
use workspace::{OpenOptions, OpenVisible};

use crate::lsp_ext::find_specific_language_server_in_selection;

use crate::{Editor, SwitchSourceHeader, element::register_action};

use project::lsp_store::clangd_ext::CLANGD_SERVER_NAME;

fn is_c_language(language: &Language) -> bool {
    return language.name() == "C++".into() || language.name() == "C".into();
}

pub fn switch_source_header(
    editor: &mut Editor,
    _: &SwitchSourceHeader,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let Some(project) = &editor.project else {
        return;
    };
    let Some(workspace) = editor.workspace() else {
        return;
    };

    let server_lookup =
        find_specific_language_server_in_selection(editor, cx, is_c_language, CLANGD_SERVER_NAME);
    let project = project.clone();
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    cx.spawn_in(window, async move |_editor, cx| {
        let Some((_, _, server_to_query, buffer)) =
            server_lookup.await
        else {
            return Ok(());
        };
        let source_file = buffer.update(cx, |buffer, _| {
            buffer.file().map(|file| file.path()).map(|path| path.to_string_lossy().to_string()).unwrap_or_else(|| "Unknown".to_string())
        })?;

        let switch_source_header = if let Some((client, project_id)) = upstream_client {
            let buffer_id = buffer.update(cx, |buffer, _| buffer.remote_id())?;
            let request = proto::LspExtSwitchSourceHeader {
                project_id,
                buffer_id: buffer_id.to_proto(),
            };
            let response = client
                .request(request)
                .await
                .context("lsp ext switch source header proto request")?;
            SwitchSourceHeaderResult(response.target_file)
        } else {
            project.update(cx, |project, cx| {
                project.request_lsp(
                    buffer,
                    project::LanguageServerToQuery::Other(server_to_query),
                    project::lsp_store::lsp_ext_command::SwitchSourceHeader,
                    cx,
                )
            })?.await.with_context(|| format!("Switch source/header LSP request for path \"{source_file}\" failed"))?
        };

        if switch_source_header.0.is_empty() {
            log::info!("Clangd returned an empty string when requesting to switch source/header from \"{source_file}\"" );
            return Ok(());
        }

        let goto = Url::parse(&switch_source_header.0).with_context(|| {
            format!(
                "Parsing URL \"{}\" returned from switch source/header failed",
                switch_source_header.0
            )
        })?;

        let path = goto.to_file_path().map_err(|()| {
            anyhow::anyhow!("URL conversion to file path failed for \"{goto}\"")
        })?;

        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(path, OpenOptions { visible: Some(OpenVisible::None), ..Default::default() }, window, cx)
            })
            .with_context(|| {
                format!(
                    "Switch source/header could not open \"{goto}\" in workspace"
                )
            })?
            .await
            .map(|_| ())
    })
    .detach_and_log_err(cx);
}

pub fn apply_related_actions(editor: &Entity<Editor>, window: &mut Window, cx: &mut App) {
    if editor
        .read(cx)
        .buffer()
        .read(cx)
        .all_buffers()
        .into_iter()
        .filter_map(|buffer| buffer.read(cx).language())
        .any(|language| is_c_language(language))
    {
        register_action(&editor, window, switch_source_header);
    }
}
