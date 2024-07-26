//! REPL operations on an [`Editor`].

use std::ops::Range;
use std::sync::Arc;

use anyhow::{Context, Result};
use editor::Editor;
use gpui::{prelude::*, AppContext, Entity, View, WeakView, WindowContext};
use language::{BufferSnapshot, Language, Point};

use crate::repl_store::ReplStore;
use crate::session::SessionEvent;
use crate::{KernelSpecification, Session};

pub fn run(editor: WeakView<Editor>, cx: &mut WindowContext) -> Result<()> {
    let store = ReplStore::global(cx);
    if !store.read(cx).is_enabled() {
        return Ok(());
    }

    let editor = editor.upgrade().context("editor was dropped")?;
    let selected_range = editor
        .update(cx, |editor, cx| editor.selections.newest_adjusted(cx))
        .range();
    let multibuffer = editor.read(cx).buffer().clone();
    let Some(buffer) = multibuffer.read(cx).as_singleton() else {
        return Ok(());
    };

    let (ranges, next_cell_point) = snippet_ranges(&buffer.read(cx).snapshot(), selected_range);

    for range in ranges {
        let Some(language) = multibuffer.read(cx).language_at(range.start, cx) else {
            continue;
        };

        let kernel_specification = store.update(cx, |store, cx| {
            store
                .kernelspec(&language, cx)
                .with_context(|| format!("No kernel found for language: {}", language.name()))
        })?;

        let fs = store.read(cx).fs().clone();
        let telemetry = store.read(cx).telemetry().clone();

        let session = if let Some(session) = store.read(cx).get_session(editor.entity_id()).cloned()
        {
            session
        } else {
            let weak_editor = editor.downgrade();
            let session = cx
                .new_view(|cx| Session::new(weak_editor, fs, telemetry, kernel_specification, cx));

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
                store.insert_session(editor.entity_id(), session.clone());
            });

            session
        };

        let selected_text;
        let anchor_range;
        let next_cursor;
        {
            let snapshot = multibuffer.read(cx).read(cx);
            selected_text = snapshot.text_for_range(range.clone()).collect::<String>();
            anchor_range = snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end);
            next_cursor = next_cell_point.map(|point| snapshot.anchor_after(point));
        }

        session.update(cx, |session, cx| {
            session.execute(selected_text, anchor_range, next_cursor, cx);
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

fn snippet_range(buffer: &BufferSnapshot, start_row: u32, end_row: u32) -> Range<Point> {
    let mut snippet_end_row = end_row;
    while buffer.is_line_blank(snippet_end_row) && snippet_end_row > start_row {
        snippet_end_row -= 1;
    }
    Point::new(start_row, 0)..Point::new(snippet_end_row, buffer.line_len(snippet_end_row))
}

// Returns the ranges of the snippets in the buffer and the next range for moving the cursor to
fn jupytext_snippets(
    buffer: &BufferSnapshot,
    range: Range<Point>,
) -> (Vec<Range<Point>>, Option<Point>) {
    let mut current_row = range.start.row;

    let Some(language) = buffer.language() else {
        return (Vec::new(), None);
    };

    let default_scope = language.default_scope();
    let comment_prefixes = default_scope.line_comment_prefixes();
    if comment_prefixes.is_empty() {
        return (Vec::new(), None);
    }

    let jupytext_prefixes = comment_prefixes
        .iter()
        .map(|comment_prefix| format!("{comment_prefix}%%"))
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
                snippets.push(snippet_range(buffer, snippet_start_row, current_row - 1));

                if current_row <= range.end.row {
                    snippet_start_row = current_row;
                } else {
                    // Return our snippets as well as the next range for moving the cursor to
                    return (snippets, Some(Point::new(current_row, 0)));
                }
            }
        }

        // Go to the end of the buffer (no more jupytext cells found)
        snippets.push(snippet_range(
            buffer,
            snippet_start_row,
            buffer.max_point().row,
        ));
    }

    (snippets, None)
}

fn snippet_ranges(
    buffer: &BufferSnapshot,
    range: Range<Point>,
) -> (Vec<Range<Point>>, Option<Point>) {
    let (jupytext_snippets, next_cursor) = jupytext_snippets(buffer, range.clone());
    if !jupytext_snippets.is_empty() {
        return (jupytext_snippets, next_cursor);
    }

    let snippet_range = snippet_range(buffer, range.start.row, range.end.row);
    let start_language = buffer.language_at(snippet_range.start);
    let end_language = buffer.language_at(snippet_range.end);

    if let Some((start, end)) = start_language.zip(end_language) {
        if start == end {
            return (vec![snippet_range], None);
        }
    }

    (Vec::new(), None)
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
                line_comments: vec!["# ".into()],
                ..Default::default()
            },
            None,
        ));

        let buffer = cx.new_model(|cx| {
            Buffer::local(
                indoc! { r#"
                    print(1 + 1)
                    print(2 + 2)

                    print(4 + 4)


                "# },
                cx,
            )
            .with_language(test_language, cx)
        });
        let snapshot = buffer.read(cx).snapshot();

        // Single-point selection
        let (snippets, _) = snippet_ranges(&snapshot, Point::new(0, 4)..Point::new(0, 4));
        let snippets = snippets
            .into_iter()
            .map(|range| snapshot.text_for_range(range).collect::<String>())
            .collect::<Vec<_>>();
        assert_eq!(snippets, vec!["print(1 + 1)"]);

        // Multi-line selection
        let (snippets, _) = snippet_ranges(&snapshot, Point::new(0, 5)..Point::new(2, 0));
        let snippets = snippets
            .into_iter()
            .map(|range| snapshot.text_for_range(range).collect::<String>())
            .collect::<Vec<_>>();
        assert_eq!(
            snippets,
            vec![indoc! { r#"
                print(1 + 1)
                print(2 + 2)"# }]
        );

        // Trimming multiple trailing blank lines
        let (snippets, _) = snippet_ranges(&snapshot, Point::new(0, 5)..Point::new(5, 0));

        let snippets = snippets
            .into_iter()
            .map(|range| snapshot.text_for_range(range).collect::<String>())
            .collect::<Vec<_>>();
        assert_eq!(
            snippets,
            vec![indoc! { r#"
                print(1 + 1)
                print(2 + 2)

                print(4 + 4)"# }]
        );
    }

    #[gpui::test]
    fn test_jupytext_snippet_ranges(cx: &mut AppContext) {
        // Create a test language
        let test_language = Arc::new(Language::new(
            LanguageConfig {
                name: "TestLang".into(),
                line_comments: vec!["# ".into()],
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
        let (snippets, _) = snippet_ranges(&snapshot, Point::new(2, 5)..Point::new(2, 5));

        let snippets = snippets
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
        let (snippets, _) = snippet_ranges(&snapshot, Point::new(2, 5)..Point::new(6, 2));
        let snippets = snippets
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
