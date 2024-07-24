//! REPL operations on an [`Editor`].

use std::ops::Range;
use std::sync::Arc;

use anyhow::{Context, Result};
use editor::{Anchor, Editor, MultiBuffer, RangeToAnchorExt};
use gpui::{prelude::*, AppContext, Entity, Model, View, WeakView, WindowContext};
use language::{Buffer, BufferSnapshot, Language, Point, ToOffset};
use multi_buffer::MultiBufferRow;
use runtimelib::dirs::ask_jupyter;
use util::ResultExt;

use crate::repl_store::ReplStore;
use crate::session::SessionEvent;
use crate::{KernelSpecification, Session};

pub fn run(editor: WeakView<Editor>, cx: &mut WindowContext) -> Result<()> {
    let store = ReplStore::global(cx);
    if !store.read(cx).is_enabled() {
        return Ok(());
    }

    let editor = editor.upgrade().context("editor was dropped")?;

    let multibuffer = editor.read(cx).buffer().clone();
    // todo!("don't return a reference for the excerpt id")

    let (excerpt_id, snapshot) =
        if let Some((excerpt_id, _, snapshot)) = multibuffer.read(cx).read(cx).as_singleton() {
            (*excerpt_id, snapshot.clone())
        } else {
            return Ok(());
        };

    let range = editor.read(cx).selections.newest::<Point>(cx).range();

    // todo!("FIX THIS!")
    // for (selected_text, language, anchor_range) in snippets(&snapshot, range) {
    //     let start = multibuffer
    //         .read(cx)
    //         .read(cx)
    //         .anchor_in_excerpt(excerpt_id, anchor_range.start)
    //         .unwrap();
    //     let end = multibuffer
    //         .read(cx)
    //         .read(cx)
    //         .anchor_in_excerpt(excerpt_id, anchor_range.end)
    //         .unwrap();
    //     let anchor_range = start..end;
    //     let entity_id = editor.entity_id();

    //     let kernel_specification = store.update(cx, |store, cx| {
    //         store
    //             .kernelspec(&language, cx)
    //             .with_context(|| format!("No kernel found for language: {}", language.name()))
    //     })?;

    //     let fs = store.read(cx).fs().clone();
    //     let session = if let Some(session) = store.read(cx).get_session(entity_id).cloned() {
    //         session
    //     } else {
    //         let weak_editor = editor.downgrade();
    //         let session = cx.new_view(|cx| Session::new(weak_editor, fs, kernel_specification, cx));

    //         editor.update(cx, |_editor, cx| {
    //             cx.notify();

    //             cx.subscribe(&session, {
    //                 let store = store.clone();
    //                 move |_this, _session, event, cx| match event {
    //                     SessionEvent::Shutdown(shutdown_event) => {
    //                         store.update(cx, |store, _cx| {
    //                             store.remove_session(shutdown_event.entity_id());
    //                         });
    //                     }
    //                 }
    //             })
    //             .detach();
    //         });

    //         store.update(cx, |store, _cx| {
    //             store.insert_session(entity_id, session.clone());
    //         });

    //         session
    //     };

    //     session.update(cx, |session, cx| {
    //         session.execute(&selected_text, anchor_range, cx);
    //     });
    // }

    anyhow::Ok(())
}

pub enum SessionSupport {
    ActiveSession(View<Session>),
    Inactive(Box<KernelSpecification>),
    RequiresSetup(Arc<str>),
    Unsupported,
}

pub fn session(editor: WeakView<Editor>, cx: &mut AppContext) -> SessionSupport {
    let store = ReplStore::global(cx);
    let entity_id = editor.entity_id();

    if let Some(session) = store.read(cx).get_session(entity_id).cloned() {
        return SessionSupport::ActiveSession(session);
    };

    let Some(language) = get_language(editor, cx) else {
        return SessionSupport::Unsupported;
    };
    let kernelspec = store.update(cx, |store, cx| store.kernelspec(&language, cx));

    match kernelspec {
        Some(kernelspec) => SessionSupport::Inactive(Box::new(kernelspec)),
        None => match language.name().as_ref() {
            "TypeScript" | "Python" => SessionSupport::RequiresSetup(language.name()),
            _ => SessionSupport::Unsupported,
        },
    }
}

pub fn clear_outputs(editor: WeakView<Editor>, cx: &mut WindowContext) {
    let store = ReplStore::global(cx);
    let entity_id = editor.entity_id();
    let Some(session) = store.read(cx).get_session(entity_id).cloned() else {
        return;
    };
    session.update(cx, |session, cx| {
        session.clear_outputs(cx);
        cx.notify();
    });
}

pub fn interrupt(editor: WeakView<Editor>, cx: &mut WindowContext) {
    let store = ReplStore::global(cx);
    let entity_id = editor.entity_id();
    let Some(session) = store.read(cx).get_session(entity_id).cloned() else {
        return;
    };

    session.update(cx, |session, cx| {
        session.interrupt(cx);
        cx.notify();
    });
}

pub fn shutdown(editor: WeakView<Editor>, cx: &mut WindowContext) {
    let store = ReplStore::global(cx);
    let entity_id = editor.entity_id();
    let Some(session) = store.read(cx).get_session(entity_id).cloned() else {
        return;
    };

    session.update(cx, |session, cx| {
        session.shutdown(cx);
        cx.notify();
    });
}

fn jupytext_snippets(buffer: &BufferSnapshot, range: Range<Point>) -> Vec<Range<Point>> {
    fn push_snippet(
        snippets: &mut Vec<Range<Point>>,
        buffer: &BufferSnapshot,
        start_row: u32,
        end_row: u32,
    ) {
        let mut snippet_end_row = end_row;
        while buffer.is_line_blank(snippet_end_row) && snippet_end_row > start_row {
            snippet_end_row -= 1;
        }
        snippets.push(
            Point::new(start_row, 0)..Point::new(snippet_end_row, buffer.line_len(snippet_end_row)),
        );
    }

    let mut current_row = range.start.row;

    let Some(language) = buffer.language() else {
        return Vec::new();
    };

    let default_scope = language.default_scope();
    let comment_prefixes = default_scope.line_comment_prefixes();
    if comment_prefixes.is_empty() {
        return Vec::new();
    }

    let jupytext_prefixes = comment_prefixes
        .iter()
        .map(|comment| format!("{comment} %%"))
        .collect::<Vec<_>>();

    let mut snippet_start_row = None;
    loop {
        if jupytext_prefixes
            .iter()
            .any(|prefix| buffer.contains_str_at(Point::new(current_row, 0), prefix))
        {
            snippet_start_row = Some(current_row);
            break;
        } else if current_row > 0 {
            current_row -= 1;
        } else {
            break;
        }
    }

    let mut snippets = Vec::new();
    if let Some(mut snippet_start_row) = snippet_start_row {
        for current_row in range.start.row + 1..=buffer.max_point().row {
            if jupytext_prefixes
                .iter()
                .any(|prefix| buffer.contains_str_at(Point::new(current_row, 0), prefix))
            {
                push_snippet(&mut snippets, buffer, snippet_start_row, current_row - 1);

                if current_row <= range.end.row {
                    snippet_start_row = current_row;
                } else {
                    return snippets;
                }
            }
        }

        push_snippet(
            &mut snippets,
            buffer,
            snippet_start_row,
            buffer.max_point().row,
        );
    }

    snippets
}

fn snippet_ranges(buffer: &BufferSnapshot, range: Range<Point>) -> Vec<Range<Point>> {
    // let buffer = buffer.read(cx);
    //
    let jupytext_snippets = jupytext_snippets(buffer, range);
    if !jupytext_snippets.is_empty() {
        return jupytext_snippets;
    }

    todo!()
    // let anchor_range = range.to_anchors(&multi_buffer_snapshot);

    // let selected_text = buffer
    //     .text_for_range(anchor_range.clone())
    //     .collect::<String>();

    // let start_language = buffer.language_at(anchor_range.start);
    // let end_language = buffer.language_at(anchor_range.end);
    // match start_language.zip(end_language) {
    //     Some((start, end)) if start == end => vec![(selected_text, start.clone(), anchor_range)],
    //     _ => Vec::new(),
    // }

    // vec![(selected_text, start_language.clone(), anchor_range)]
}

fn get_language(editor: WeakView<Editor>, cx: &mut AppContext) -> Option<Arc<Language>> {
    let editor = editor.upgrade()?;
    let selection = editor.read(cx).selections.newest::<usize>(cx);
    let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
    buffer.language_at(selection.head()).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::Context;
    use indoc::indoc;
    use language::{Buffer, Language, LanguageConfig};

    #[gpui::test]
    fn test_snippet_ranges(cx: &mut AppContext) {
        // Create a test language
        let test_language = Arc::new(Language::new(
            LanguageConfig {
                name: "TestLang".into(),
                line_comments: vec!["#".into()],
                ..Default::default()
            },
            None,
        ));

        let buffer = cx.new_model(|cx| {
            Buffer::local(
                indoc! { r#"
                    # Hello!
                    # %% [markdown]
                    # This is some arithmetic
                    print(1 + 1)
                    print(2 + 2)

                    # %%
                    print(3 + 3)
                    print(4 + 4)

                    print(5 + 5)



                "# },
                cx,
            )
            .with_language(test_language, cx)
        });
        let snapshot = buffer.read(cx).snapshot();

        // Jupytext snippet surrounding an empty selection
        let snippets = snippet_ranges(&snapshot, Point::new(2, 5)..Point::new(2, 5))
            .into_iter()
            .map(|range| snapshot.text_for_range(range).collect::<String>())
            .collect::<Vec<_>>();
        assert_eq!(
            snippets,
            vec![indoc! { r#"
                # %% [markdown]
                # This is some arithmetic
                print(1 + 1)
                print(2 + 2)"# }]
        );

        // Jupytext snippets intersecting a non-empty selection
        let snippets = snippet_ranges(&snapshot, Point::new(2, 5)..Point::new(6, 2))
            .into_iter()
            .map(|range| snapshot.text_for_range(range).collect::<String>())
            .collect::<Vec<_>>();
        assert_eq!(
            snippets,
            vec![
                indoc! { r#"
                    # %% [markdown]
                    # This is some arithmetic
                    print(1 + 1)
                    print(2 + 2)"#
                },
                indoc! { r#"
                    # %%
                    print(3 + 3)
                    print(4 + 4)

                    print(5 + 5)"#
                }
            ]
        );
    }
}
