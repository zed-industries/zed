//! REPL operations on an [`Editor`].

use std::ops::Range;
use std::sync::Arc;

use anyhow::{Context, Result};
use editor::{Anchor, Editor, MultiBuffer, RangeToAnchorExt};
use gpui::{prelude::*, AppContext, Entity, Model, View, WeakView, WindowContext};
use language::{Buffer, BufferSnapshot, Language, Point};
use multi_buffer::MultiBufferRow;
use runtimelib::dirs::ask_jupyter;

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

    for (selected_text, language, anchor_range) in snippets(&snapshot, range) {
        let start = multibuffer
            .read(cx)
            .read(cx)
            .anchor_in_excerpt(excerpt_id, anchor_range.start)
            .unwrap();
        let end = multibuffer
            .read(cx)
            .read(cx)
            .anchor_in_excerpt(excerpt_id, anchor_range.end)
            .unwrap();
        let anchor_range = start..end;
        let entity_id = editor.entity_id();

        let kernel_specification = store.update(cx, |store, cx| {
            store
                .kernelspec(&language, cx)
                .with_context(|| format!("No kernel found for language: {}", language.name()))
        })?;

        let fs = store.read(cx).fs().clone();
        let session = if let Some(session) = store.read(cx).get_session(entity_id).cloned() {
            session
        } else {
            let weak_editor = editor.downgrade();
            let session = cx.new_view(|cx| Session::new(weak_editor, fs, kernel_specification, cx));

            editor.update(cx, |_editor, cx| {
                cx.notify();

                cx.subscribe(&session, {
                    let store = store.clone();
                    move |_this, _session, event, cx| match event {
                        SessionEvent::Shutdown(shutdown_event) => {
                            store.update(cx, |store, _cx| {
                                store.remove_session(shutdown_event.entity_id());
                            });
                        }
                    }
                })
                .detach();
            });

            store.update(cx, |store, _cx| {
                store.insert_session(entity_id, session.clone());
            });

            session
        };

        session.update(cx, |session, cx| {
            session.execute(&selected_text, anchor_range, cx);
        });
    }

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

fn snippets(
    buffer: &BufferSnapshot,
    range: Range<Point>,
) -> Vec<(String, Arc<Language>, Range<language::Anchor>)> {
    // let buffer = buffer.read(cx);

    let line_comment_prefixes = buffer.language().map_or([].as_slice(), |language| {
        language.default_scope().line_comment_prefixes()
    });

    if !line_comment_prefixes.is_empty() {
        // let mut snippets = Vec::new();

        let mut jupytext_start_row = None;
        let mut prev_lines = buffer
            .reversed_chunks_in_range(
                Point::zero()..Point::new(range.start.row, buffer.line_len(range.start.row)),
            )
            .lines();
        let mut current_row = range.start.row;
        while let Some(line) = prev_lines.next() {
            if let Some(prefix) = line_comment_prefixes
                .iter()
                .find(|prefix| line.starts_with(prefix.as_ref()))
            {
                if line[prefix.len()..].starts_with("%%") ||
                jupytext_start_row = Some(current_row));
                break;
            } else {
                current_row = current_row.saturating_sub(1);
            }
        }
    }

    // multi_buffer_snapshot.chunks(range, language_aware)

    // let range = if range.is_empty() {
    //     Point::new(range.start.row, 0)
    //         ..Point::new(
    //             range.start.row,
    //             multi_buffer_snapshot.line_len(MultiBufferRow(range.start.row)),
    //         )
    // } else {
    //     if range.end.column == 0 {
    //         range.end.row -= 1;
    //         range.end.column = multi_buffer_snapshot.line_len(MultiBufferRow(range.end.row));
    //     }
    //     range
    // };

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
    fn test_snippet(cx: &mut AppContext) {
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

        let snippets = snippets(&snapshot, Point::new(2, 5)..Point::new(2, 5))
            .into_iter()
            .map(|(selected_text, _, _)| selected_text)
            .collect::<Vec<_>>();
        assert_eq!(
            snippets,
            vec![indoc! { r#"
                # This is some arithmetic
                print(1 + 1)
                print(2 + 2)
            "# }]
        );
    }
}
