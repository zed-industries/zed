//! REPL operations on an [`Editor`].

use std::ops::Range;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use editor::Editor;
use gpui::{App, Entity, WeakEntity, Window, prelude::*};
use language::{BufferSnapshot, Language, LanguageName, Point};
use project::{ProjectItem as _, WorktreeId};

use crate::repl_store::ReplStore;
use crate::session::SessionEvent;
use crate::{
    ClearOutputs, Interrupt, JupyterSettings, KernelSpecification, Restart, Session, Shutdown,
};

pub fn assign_kernelspec(
    kernel_specification: KernelSpecification,
    weak_editor: WeakEntity<Editor>,
    window: &mut Window,
    cx: &mut App,
) -> Result<()> {
    let store = ReplStore::global(cx);
    if !store.read(cx).is_enabled() {
        return Ok(());
    }

    let worktree_id = crate::repl_editor::worktree_id_for_editor(weak_editor.clone(), cx)
        .context("editor is not in a worktree")?;

    store.update(cx, |store, cx| {
        store.set_active_kernelspec(worktree_id, kernel_specification.clone(), cx);
    });

    let fs = store.read(cx).fs().clone();

    if let Some(session) = store.read(cx).get_session(weak_editor.entity_id()).cloned() {
        // Drop previous session, start new one
        session.update(cx, |session, cx| {
            session.clear_outputs(cx);
            session.shutdown(window, cx);
            cx.notify();
        });
    }

    let session =
        cx.new(|cx| Session::new(weak_editor.clone(), fs, kernel_specification, window, cx));

    weak_editor
        .update(cx, |_editor, cx| {
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
        })
        .ok();

    store.update(cx, |store, _cx| {
        store.insert_session(weak_editor.entity_id(), session.clone());
    });

    Ok(())
}

pub fn run(
    editor: WeakEntity<Editor>,
    move_down: bool,
    window: &mut Window,
    cx: &mut App,
) -> Result<()> {
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

    let Some(project_path) = buffer.read(cx).project_path(cx) else {
        return Ok(());
    };

    let (runnable_ranges, next_cell_point) =
        runnable_ranges(&buffer.read(cx).snapshot(), selected_range);

    for runnable_range in runnable_ranges {
        let Some(language) = multibuffer.read(cx).language_at(runnable_range.start, cx) else {
            continue;
        };

        let kernel_specification = store
            .read(cx)
            .active_kernelspec(project_path.worktree_id, Some(language.clone()), cx)
            .ok_or_else(|| anyhow::anyhow!("No kernel found for language: {}", language.name()))?;

        let fs = store.read(cx).fs().clone();

        let session = if let Some(session) = store.read(cx).get_session(editor.entity_id()).cloned()
        {
            session
        } else {
            let weak_editor = editor.downgrade();
            let session =
                cx.new(|cx| Session::new(weak_editor, fs, kernel_specification, window, cx));

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
            selected_text = snapshot
                .text_for_range(runnable_range.clone())
                .collect::<String>();
            anchor_range = snapshot.anchor_before(runnable_range.start)
                ..snapshot.anchor_after(runnable_range.end);
            next_cursor = next_cell_point.map(|point| snapshot.anchor_after(point));
        }

        session.update(cx, |session, cx| {
            session.execute(
                selected_text,
                anchor_range,
                next_cursor,
                move_down,
                window,
                cx,
            );
        });
    }

    anyhow::Ok(())
}

#[allow(clippy::large_enum_variant)]
pub enum SessionSupport {
    ActiveSession(Entity<Session>),
    Inactive(KernelSpecification),
    RequiresSetup(LanguageName),
    Unsupported,
}

pub fn worktree_id_for_editor(editor: WeakEntity<Editor>, cx: &mut App) -> Option<WorktreeId> {
    editor.upgrade().and_then(|editor| {
        editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()?
            .read(cx)
            .project_path(cx)
            .map(|path| path.worktree_id)
    })
}

pub fn session(editor: WeakEntity<Editor>, cx: &mut App) -> SessionSupport {
    let store = ReplStore::global(cx);
    let entity_id = editor.entity_id();

    if let Some(session) = store.read(cx).get_session(entity_id).cloned() {
        return SessionSupport::ActiveSession(session);
    };

    let Some(language) = get_language(editor.clone(), cx) else {
        return SessionSupport::Unsupported;
    };

    let worktree_id = worktree_id_for_editor(editor.clone(), cx);

    let Some(worktree_id) = worktree_id else {
        return SessionSupport::Unsupported;
    };

    let kernelspec = store
        .read(cx)
        .active_kernelspec(worktree_id, Some(language.clone()), cx);

    match kernelspec {
        Some(kernelspec) => SessionSupport::Inactive(kernelspec),
        None => {
            if language_supported(&language.clone()) {
                SessionSupport::RequiresSetup(language.name())
            } else {
                SessionSupport::Unsupported
            }
        }
    }
}

pub fn clear_outputs(editor: WeakEntity<Editor>, cx: &mut App) {
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

pub fn interrupt(editor: WeakEntity<Editor>, cx: &mut App) {
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

pub fn shutdown(editor: WeakEntity<Editor>, window: &mut Window, cx: &mut App) {
    let store = ReplStore::global(cx);
    let entity_id = editor.entity_id();
    let Some(session) = store.read(cx).get_session(entity_id).cloned() else {
        return;
    };

    session.update(cx, |session, cx| {
        session.shutdown(window, cx);
        cx.notify();
    });
}

pub fn restart(editor: WeakEntity<Editor>, window: &mut Window, cx: &mut App) {
    let Some(editor) = editor.upgrade() else {
        return;
    };

    let entity_id = editor.entity_id();

    let Some(session) = ReplStore::global(cx)
        .read(cx)
        .get_session(entity_id)
        .cloned()
    else {
        return;
    };

    session.update(cx, |session, cx| {
        session.restart(window, cx);
        cx.notify();
    });
}

pub fn setup_editor_session_actions(editor: &mut Editor, editor_handle: WeakEntity<Editor>) {
    editor
        .register_action({
            let editor_handle = editor_handle.clone();
            move |_: &ClearOutputs, _, cx| {
                if !JupyterSettings::enabled(cx) {
                    return;
                }

                crate::clear_outputs(editor_handle.clone(), cx);
            }
        })
        .detach();

    editor
        .register_action({
            let editor_handle = editor_handle.clone();
            move |_: &Interrupt, _, cx| {
                if !JupyterSettings::enabled(cx) {
                    return;
                }

                crate::interrupt(editor_handle.clone(), cx);
            }
        })
        .detach();

    editor
        .register_action({
            let editor_handle = editor_handle.clone();
            move |_: &Shutdown, window, cx| {
                if !JupyterSettings::enabled(cx) {
                    return;
                }

                crate::shutdown(editor_handle.clone(), window, cx);
            }
        })
        .detach();

    editor
        .register_action({
            let editor_handle = editor_handle.clone();
            move |_: &Restart, window, cx| {
                if !JupyterSettings::enabled(cx) {
                    return;
                }

                crate::restart(editor_handle.clone(), window, cx);
            }
        })
        .detach();
}

fn cell_range(buffer: &BufferSnapshot, start_row: u32, end_row: u32) -> Range<Point> {
    let mut snippet_end_row = end_row;
    while buffer.is_line_blank(snippet_end_row) && snippet_end_row > start_row {
        snippet_end_row -= 1;
    }
    Point::new(start_row, 0)..Point::new(snippet_end_row, buffer.line_len(snippet_end_row))
}

// Returns the ranges of the snippets in the buffer and the next point for moving the cursor to
fn jupytext_cells(
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
                snippets.push(cell_range(buffer, snippet_start_row, current_row - 1));

                if current_row <= range.end.row {
                    snippet_start_row = current_row;
                } else {
                    // Return our snippets as well as the next point for moving the cursor to
                    return (snippets, Some(Point::new(current_row, 0)));
                }
            }
        }

        // Go to the end of the buffer (no more jupytext cells found)
        snippets.push(cell_range(
            buffer,
            snippet_start_row,
            buffer.max_point().row,
        ));
    }

    (snippets, None)
}

fn runnable_ranges(
    buffer: &BufferSnapshot,
    range: Range<Point>,
) -> (Vec<Range<Point>>, Option<Point>) {
    if let Some(language) = buffer.language() {
        if language.name() == "Markdown".into() {
            return (markdown_code_blocks(buffer, range.clone()), None);
        }
    }

    let (jupytext_snippets, next_cursor) = jupytext_cells(buffer, range.clone());
    if !jupytext_snippets.is_empty() {
        return (jupytext_snippets, next_cursor);
    }

    let snippet_range = cell_range(buffer, range.start.row, range.end.row);
    let start_language = buffer.language_at(snippet_range.start);
    let end_language = buffer.language_at(snippet_range.end);

    if start_language
        .zip(end_language)
        .map_or(false, |(start, end)| start == end)
    {
        (vec![snippet_range], None)
    } else {
        (Vec::new(), None)
    }
}

// We allow markdown code blocks to end in a trailing newline in order to render the output
// below the final code fence. This is different than our behavior for selections and Jupytext cells.
fn markdown_code_blocks(buffer: &BufferSnapshot, range: Range<Point>) -> Vec<Range<Point>> {
    buffer
        .injections_intersecting_range(range)
        .filter(|(_, language)| language_supported(language))
        .map(|(content_range, _)| {
            buffer.offset_to_point(content_range.start)..buffer.offset_to_point(content_range.end)
        })
        .collect()
}

fn language_supported(language: &Arc<Language>) -> bool {
    match language.name().as_ref() {
        "TypeScript" | "Python" => true,
        _ => false,
    }
}

fn get_language(editor: WeakEntity<Editor>, cx: &mut App) -> Option<Arc<Language>> {
    editor
        .update(cx, |editor, cx| {
            let selection = editor
                .selections
                .newest::<usize>(&editor.selections.display_map(cx));
            let buffer = editor.buffer().read(cx).snapshot(cx);
            buffer.language_at(selection.head()).cloned()
        })
        .ok()
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::App;
    use indoc::indoc;
    use language::{Buffer, Language, LanguageConfig, LanguageRegistry};

    #[gpui::test]
    fn test_snippet_ranges(cx: &mut App) {
        // Create a test language
        let test_language = Arc::new(Language::new(
            LanguageConfig {
                name: "TestLang".into(),
                line_comments: vec!["# ".into()],
                ..Default::default()
            },
            None,
        ));

        let buffer = cx.new(|cx| {
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
        let (snippets, _) = runnable_ranges(&snapshot, Point::new(0, 4)..Point::new(0, 4));
        let snippets = snippets
            .into_iter()
            .map(|range| snapshot.text_for_range(range).collect::<String>())
            .collect::<Vec<_>>();
        assert_eq!(snippets, vec!["print(1 + 1)"]);

        // Multi-line selection
        let (snippets, _) = runnable_ranges(&snapshot, Point::new(0, 5)..Point::new(2, 0));
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
        let (snippets, _) = runnable_ranges(&snapshot, Point::new(0, 5)..Point::new(5, 0));

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
    fn test_jupytext_snippet_ranges(cx: &mut App) {
        // Create a test language
        let test_language = Arc::new(Language::new(
            LanguageConfig {
                name: "TestLang".into(),
                line_comments: vec!["# ".into()],
                ..Default::default()
            },
            None,
        ));

        let buffer = cx.new(|cx| {
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
        let (snippets, _) = runnable_ranges(&snapshot, Point::new(2, 5)..Point::new(2, 5));

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
        let (snippets, _) = runnable_ranges(&snapshot, Point::new(2, 5)..Point::new(6, 2));
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

    #[gpui::test]
    fn test_markdown_code_blocks(cx: &mut App) {
        let markdown = languages::language("markdown", tree_sitter_md::LANGUAGE.into());
        let typescript = languages::language(
            "typescript",
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        );
        let python = languages::language("python", tree_sitter_python::LANGUAGE.into());
        let language_registry = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));
        language_registry.add(markdown.clone());
        language_registry.add(typescript.clone());
        language_registry.add(python.clone());

        // Two code blocks intersecting with selection
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(
                indoc! { r#"
                    Hey this is Markdown!

                    ```typescript
                    let foo = 999;
                    console.log(foo + 1999);
                    ```

                    ```typescript
                    console.log("foo")
                    ```
                    "#
                },
                cx,
            );
            buffer.set_language_registry(language_registry.clone());
            buffer.set_language(Some(markdown.clone()), cx);
            buffer
        });
        let snapshot = buffer.read(cx).snapshot();

        let (snippets, _) = runnable_ranges(&snapshot, Point::new(3, 5)..Point::new(8, 5));
        let snippets = snippets
            .into_iter()
            .map(|range| snapshot.text_for_range(range).collect::<String>())
            .collect::<Vec<_>>();

        assert_eq!(
            snippets,
            vec![
                indoc! { r#"
                    let foo = 999;
                    console.log(foo + 1999);
                    "#
                },
                "console.log(\"foo\")\n"
            ]
        );

        // Three code blocks intersecting with selection
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(
                indoc! { r#"
                    Hey this is Markdown!

                    ```typescript
                    let foo = 999;
                    console.log(foo + 1999);
                    ```

                    ```ts
                    console.log("foo")
                    ```

                    ```typescript
                    console.log("another code block")
                    ```
                "# },
                cx,
            );
            buffer.set_language_registry(language_registry.clone());
            buffer.set_language(Some(markdown.clone()), cx);
            buffer
        });
        let snapshot = buffer.read(cx).snapshot();

        let (snippets, _) = runnable_ranges(&snapshot, Point::new(3, 5)..Point::new(12, 5));
        let snippets = snippets
            .into_iter()
            .map(|range| snapshot.text_for_range(range).collect::<String>())
            .collect::<Vec<_>>();

        assert_eq!(
            snippets,
            vec![
                indoc! { r#"
                    let foo = 999;
                    console.log(foo + 1999);
                    "#
                },
                "console.log(\"foo\")\n",
                "console.log(\"another code block\")\n",
            ]
        );

        // Python code block
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(
                indoc! { r#"
                    Hey this is Markdown!

                    ```python
                    print("hello there")
                    print("hello there")
                    print("hello there")
                    ```
                "# },
                cx,
            );
            buffer.set_language_registry(language_registry.clone());
            buffer.set_language(Some(markdown.clone()), cx);
            buffer
        });
        let snapshot = buffer.read(cx).snapshot();

        let (snippets, _) = runnable_ranges(&snapshot, Point::new(4, 5)..Point::new(5, 5));
        let snippets = snippets
            .into_iter()
            .map(|range| snapshot.text_for_range(range).collect::<String>())
            .collect::<Vec<_>>();

        assert_eq!(
            snippets,
            vec![indoc! { r#"
                print("hello there")
                print("hello there")
                print("hello there")
                "#
            },]
        );
    }
}
