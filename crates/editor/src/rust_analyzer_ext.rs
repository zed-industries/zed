use anyhow::Context as _;
use gpui::{Context, View, ViewContext, VisualContext, WindowContext};
use language::Language;
use multi_buffer::MultiBuffer;
use project::lsp_ext_command::ExpandMacro;
use text::ToPointUtf16;

use crate::{
    element::register_action, lsp_ext::find_specific_language_server_in_selection, Editor,
    ExpandMacroRecursively,
};

static RUST_ANALYZER_NAME: &str = "rust-analyzer";

fn is_rust_language(language: &Language) -> bool {
    language.name().as_ref() == "Rust"
}

pub fn apply_related_actions(editor: &View<Editor>, cx: &mut WindowContext) {
    if editor
        .update(cx, |e, cx| {
            find_specific_language_server_in_selection(e, cx, &is_rust_language, RUST_ANALYZER_NAME)
        })
        .is_some()
    {
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

    let Some((trigger_anchor, rust_language, server_to_query, buffer)) =
        find_specific_language_server_in_selection(
            &editor,
            cx,
            &is_rust_language,
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
                true,
                cx,
            );
        })
    })
    .detach_and_log_err(cx);
}
