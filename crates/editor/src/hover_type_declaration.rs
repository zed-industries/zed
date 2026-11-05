//! Nested "type declaration" popover for the hover popover.
//!
//! When the mouse rests on an identifier inside a hover popover (e.g. the
//! `Foo` in `let x: Foo<Bar>`), we look the name up among the project's
//! workspace symbols (LSP). If it names a type, a second popover shows that
//! type's declaration source; clicking it jumps to the declaration.

use std::{ops::Range, time::Duration};

use gpui::{
    AnyElement, Context, Entity, IntoElement, Pixels, Render, SharedString, Task, WeakEntity,
    Window, anchored, deferred, point, px,
};
use language::{Bias, Buffer, LanguageName, OffsetRangeExt as _, Point, SymbolKind, ToPoint as _};
use markdown::{CopyButtonVisibility, HoveredWord, Markdown, MarkdownElement};
use project::{Project, Symbol, lsp_store::SymbolLocation};
use ui::prelude::*;
use util::ResultExt as _;

use crate::{
    Editor,
    hover_popover::{hide_hover, hover_markdown_style},
};

/// How long the mouse has to rest on a word before we look it up.
const RESOLVE_DELAY: Duration = Duration::from_millis(350);
/// Grace period for moving the mouse from the word into the nested popover.
const HIDE_DELAY: Duration = Duration::from_millis(300);
/// Longer declarations are cut off (the popover is a preview, not an editor).
const MAX_DECLARATION_LINES: u32 = 25;
/// At most this many doc-comment / attribute lines are pulled in above an item.
const MAX_LEADING_LINES: u32 = 30;
/// Lines shown when the language has no outline query to find the item's extent.
const FALLBACK_DECLARATION_LINES: u32 = 10;

pub struct TypeDeclarationPreview {
    editor: WeakEntity<Editor>,
    project: Entity<Project>,
    /// Word currently under the mouse in the parent hover popover.
    hovered_word: Option<SharedString>,
    declaration: Option<Declaration>,
    popover_hovered: bool,
    resolve_task: Option<Task<()>>,
    hide_task: Option<Task<()>>,
}

struct Declaration {
    word: SharedString,
    markdown: Entity<Markdown>,
    buffer: Entity<Buffer>,
    point: Point,
    location: SharedString,
    /// Where to show the popover, in window coordinates.
    position: gpui::Point<Pixels>,
}

/// What we learn about a type, before it's turned into UI.
struct ResolvedDeclaration {
    buffer: Entity<Buffer>,
    point: Point,
    code: String,
    language: Option<LanguageName>,
    location: String,
}

impl TypeDeclarationPreview {
    pub fn new(editor: WeakEntity<Editor>, project: Entity<Project>) -> Self {
        Self {
            editor,
            project,
            hovered_word: None,
            declaration: None,
            popover_hovered: false,
            resolve_task: None,
            hide_task: None,
        }
    }

    /// Called from the hover popover's markdown on every mouse move.
    pub fn hover_word(
        &mut self,
        word: Option<HoveredWord>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let word_text = word.as_ref().map(|word| word.text.clone());
        if word_text == self.hovered_word {
            return;
        }
        self.hovered_word = word_text;
        self.resolve_task = None;

        // Leaving the word (possibly towards the nested popover): hide after a
        // grace period unless the popover itself gets hovered.
        if self.declaration.is_some() {
            self.schedule_hide(cx);
        }

        let Some(word) = word else {
            return;
        };
        if self
            .declaration
            .as_ref()
            .is_some_and(|declaration| declaration.word == word.text)
        {
            self.hide_task = None;
            return;
        }

        let project = self.project.clone();
        self.resolve_task = Some(cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(RESOLVE_DELAY).await;
            let Some(resolved) = resolve_declaration(project.clone(), &word.text, cx)
                .await
                .log_err()
                .flatten()
            else {
                return;
            };
            this.update(cx, |this, cx| {
                if this.hovered_word.as_ref() != Some(&word.text) {
                    return;
                }
                let language_registry = project.read(cx).languages().clone();
                let markdown = cx.new(|cx| {
                    Markdown::new(
                        fenced_code_block(&resolved.code).into(),
                        Some(language_registry),
                        resolved.language,
                        cx,
                    )
                });
                this.declaration = Some(Declaration {
                    word: word.text.clone(),
                    markdown,
                    buffer: resolved.buffer,
                    point: resolved.point,
                    location: resolved.location.into(),
                    position: word.position,
                });
                this.hide_task = None;
                cx.notify();
            })
            .ok();
        }));
    }

    fn schedule_hide(&mut self, cx: &mut Context<Self>) {
        self.hide_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(HIDE_DELAY).await;
            this.update(cx, |this, cx| {
                let still_on_word = this.declaration.as_ref().is_some_and(|declaration| {
                    this.hovered_word.as_ref() == Some(&declaration.word)
                });
                if !this.popover_hovered && !still_on_word {
                    this.declaration = None;
                    cx.notify();
                }
            })
            .ok();
        }));
    }

    fn open_declaration(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(declaration) = self.declaration.take() else {
            return;
        };
        let Some(editor) = self.editor.upgrade() else {
            return;
        };
        let workspace = editor.update(cx, |editor, cx| {
            hide_hover(editor, cx);
            editor.workspace()
        });
        let Some(workspace) = workspace else {
            return;
        };
        workspace.update(cx, |workspace, cx| {
            let target = workspace.open_project_item::<Editor>(
                None,
                declaration.buffer,
                true,
                true,
                true,
                true,
                window,
                cx,
            );
            target.update(cx, |editor, cx| {
                editor.go_to_singleton_buffer_point(declaration.point, window, cx);
            });
        });
    }
}

impl Render for TypeDeclarationPreview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(declaration) = self.declaration.as_ref() else {
            return div().into_any_element();
        };
        let editor = self.editor.clone();
        let popover: AnyElement = div()
            .id("type-declaration-popover")
            .occlude()
            .elevation_2(cx)
            .max_w(rems(40.))
            .max_h(rems(24.))
            .overflow_y_scroll()
            .p_2()
            .flex()
            .flex_col()
            .gap_1()
            .cursor_pointer()
            .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                this.popover_hovered = *hovered;
                if !*hovered {
                    this.schedule_hide(cx);
                }
            }))
            // Keep the parent hover popover alive while the mouse is over us,
            // and don't let the move reach the editor (which would dismiss it).
            .on_mouse_move(move |_, _, cx| {
                editor
                    .update(cx, |editor, _| {
                        editor.hover_state.closest_mouse_distance = Some(px(0.));
                        editor.hover_state.hiding_delay_task = None;
                    })
                    .ok();
                cx.stop_propagation();
            })
            .on_click(cx.listener(|this, _, window, cx| this.open_declaration(window, cx)))
            .child(
                Label::new(declaration.location.clone())
                    .size(LabelSize::Small)
                    .color(Color::Accent),
            )
            .child(
                MarkdownElement::new(
                    declaration.markdown.clone(),
                    hover_markdown_style(window, cx),
                )
                .code_block_renderer(markdown::CodeBlockRenderer::Default {
                    copy_button_visibility: CopyButtonVisibility::Hidden,
                    wrap_button_visibility: markdown::WrapButtonVisibility::Hidden,
                    border: false,
                }),
            )
            .into_any_element();

        deferred(
            anchored()
                .position(declaration.position + point(px(0.), px(16.)))
                .snap_to_window_with_margin(px(8.))
                .child(popover),
        )
        .with_priority(2)
        .into_any_element()
    }
}

fn is_type_kind(kind: SymbolKind) -> bool {
    matches!(
        kind,
        SymbolKind::Class
            | SymbolKind::Struct
            | SymbolKind::Interface
            | SymbolKind::Enum
            | SymbolKind::TypeParameter
    )
}

/// Picks the declaration of the type called exactly `name`, preferring
/// declarations inside the project over dependencies.
fn pick_type_symbol(symbols: Vec<Symbol>, name: &str) -> Option<Symbol> {
    symbols
        .into_iter()
        .filter(|symbol| symbol.name == name && is_type_kind(symbol.kind))
        .min_by_key(|symbol| !matches!(symbol.path, SymbolLocation::InProject(_)))
}

async fn resolve_declaration(
    project: Entity<Project>,
    name: &str,
    cx: &mut gpui::AsyncApp,
) -> anyhow::Result<Option<ResolvedDeclaration>> {
    let symbols = project
        .update(cx, |project, cx| project.symbols(name, cx))
        .await?;
    let Some(symbol) = pick_type_symbol(symbols, name) else {
        return Ok(None);
    };
    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer_for_symbol(&symbol, cx)
        })
        .await?;
    let path_style = project.read_with(cx, |project, cx| project.path_style(cx));
    Ok(Some(buffer.read_with(cx, |buffer_ref, _| {
        let snapshot = buffer_ref.snapshot();
        let position = snapshot.clip_point_utf16(symbol.range.start, Bias::Left);
        let point = snapshot.point_utf16_to_point(position);

        // Use the language's outline to find the full extent of the item
        // (including doc comments / attributes), else show a few lines.
        let items = snapshot.symbols_containing(point, None);
        let item = items
            .iter()
            .rev()
            .find(|item| {
                let selection = item.selection_range.to_point(&snapshot);
                selection.start.row <= point.row && point.row <= selection.end.row
            })
            .or(items.last());
        let item_rows = item.map(|item| {
            let range = item.range.to_point(&snapshot);
            let start = item
                .annotation_range
                .as_ref()
                .map_or(range.start.row, |annotation| {
                    annotation
                        .start
                        .to_point(&snapshot)
                        .row
                        .min(range.start.row)
                });
            start..range.end.row
        });
        let item_rows = item_rows.map(|rows| {
            let start = leading_annotation_start(rows.start, |row| {
                snapshot
                    .text_for_range(Point::new(row, 0)..Point::new(row, snapshot.line_len(row)))
                    .collect()
            });
            start..rows.end
        });
        let (rows, truncated) = snippet_rows(item_rows, point.row, snapshot.max_point().row);
        let end = Point::new(rows.end, snapshot.line_len(rows.end));
        let mut code = dedent(
            &snapshot
                .text_for_range(Point::new(rows.start, 0)..end)
                .collect::<String>(),
        );
        if truncated {
            code.push_str("\n…");
        }

        let location = match &symbol.path {
            SymbolLocation::InProject(path) => path.path.display(path_style).into_owned(),
            SymbolLocation::OutsideProject { abs_path, .. } => {
                abs_path.to_string_lossy().into_owned()
            }
        };
        ResolvedDeclaration {
            buffer: buffer.clone(),
            point,
            code,
            language: buffer_ref.language().map(|language| language.name()),
            location: format!("{location}:{}", point.row + 1),
        }
    })))
}

/// Walks up from `item_start` over the doc comments, attributes and
/// decorators directly attached to the item (no blank line in between).
fn leading_annotation_start(item_start: u32, line: impl Fn(u32) -> String) -> u32 {
    let is_annotation = |text: &str| {
        let text = text.trim_start();
        ["//", "/*", "*", "#[", "#!", "@", "#"]
            .iter()
            .any(|prefix| text.starts_with(prefix))
    };
    let mut start = item_start;
    while start > 0 && item_start - start < MAX_LEADING_LINES && is_annotation(&line(start - 1)) {
        start -= 1;
    }
    start
}

/// Inclusive row range to show, and whether it was cut short.
fn snippet_rows(
    item_rows: Option<Range<u32>>,
    symbol_row: u32,
    max_row: u32,
) -> (Range<u32>, bool) {
    let rows = item_rows.unwrap_or(symbol_row..symbol_row + FALLBACK_DECLARATION_LINES - 1);
    let end = rows.end.min(max_row);
    let last_allowed = rows.start + MAX_DECLARATION_LINES - 1;
    if end > last_allowed {
        (rows.start..last_allowed, true)
    } else {
        (rows.start..end, false)
    }
}

/// Removes the indentation shared by all non-blank lines.
fn dedent(code: &str) -> String {
    let indent = code
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    code.lines()
        .map(|line| line.get(indent..).unwrap_or(line.trim_start()))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Wraps code in a fence that can't be closed by backticks inside the code.
fn fenced_code_block(code: &str) -> String {
    let longest_run = code.split(|c| c != '`').map(str::len).max().unwrap_or(0);
    let fence = "`".repeat((longest_run + 1).max(3));
    format!("{fence}\n{code}\n{fence}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor_tests::init_test, test::editor_lsp_test_context::EditorLspTestContext};
    use gpui::TestAppContext;
    use indoc::indoc;

    fn word(text: &str) -> Option<HoveredWord> {
        Some(HoveredWord {
            text: text.to_string().into(),
            source_range: 0..text.len(),
            position: point(px(10.), px(10.)),
        })
    }

    #[allow(deprecated)]
    fn lsp_symbol(
        name: &str,
        kind: lsp::SymbolKind,
        uri: &lsp::Uri,
        range: lsp::Range,
    ) -> lsp::SymbolInformation {
        lsp::SymbolInformation {
            name: name.to_string(),
            kind,
            tags: None,
            deprecated: None,
            container_name: None,
            location: lsp::Location::new(uri.clone(), range),
        }
    }

    #[gpui::test]
    async fn test_type_declaration_preview(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                workspace_symbol_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.set_state(indoc! {"
            fn unrelated() {}

            /// A foo.
            #[derive(Debug)]
            pub struct Foo {
                a: u32,
            }

            fn bar() {}ˇ
        "});

        let uri = cx.buffer_lsp_url.clone();
        let foo_range = cx.lsp_range(indoc! {"
            fn unrelated() {}

            /// A foo.
            #[derive(Debug)]
            pub struct «Foo» {
                a: u32,
            }

            fn bar() {}
        "});
        let bar_range = cx.lsp_range(indoc! {"
            fn unrelated() {}

            /// A foo.
            #[derive(Debug)]
            pub struct Foo {
                a: u32,
            }

            fn «bar»() {}
        "});
        let symbols = vec![
            lsp_symbol("Foo", lsp::SymbolKind::STRUCT, &uri, foo_range),
            // A fuzzy match the server might return, which must be ignored.
            lsp_symbol("FooBar", lsp::SymbolKind::STRUCT, &uri, foo_range),
            lsp_symbol("bar", lsp::SymbolKind::FUNCTION, &uri, bar_range),
        ];
        cx.set_request_handler::<lsp::WorkspaceSymbolRequest, _, _>(move |_, _, _| {
            let symbols = symbols.clone();
            async move { Ok(Some(lsp::WorkspaceSymbolResponse::Flat(symbols))) }
        });

        let project = cx.update_editor(|editor, _, _| editor.project.clone().unwrap());
        let editor = cx.editor.downgrade();
        let preview = (**cx).new(|_| TypeDeclarationPreview::new(editor, project));

        // Resting on a type shows its declaration, with docs and attributes.
        preview.update_in(&mut **cx, |preview, window, cx| {
            preview.hover_word(word("Foo"), window, cx)
        });
        cx.executor().advance_clock(RESOLVE_DELAY * 2);
        cx.run_until_parked();
        preview.read_with(&**cx, |preview, cx| {
            let declaration = preview.declaration.as_ref().expect("declaration shown");
            assert_eq!(declaration.location.as_ref(), "dir/file.rs:5");
            assert_eq!(declaration.point, Point::new(4, 11));
            assert_eq!(
                declaration.markdown.read(cx).source().as_ref(),
                "```\n/// A foo.\n#[derive(Debug)]\npub struct Foo {\n    a: u32,\n}\n```"
            );
        });

        // A function is not a type: the preview goes away after the grace period.
        preview.update_in(&mut **cx, |preview, window, cx| {
            preview.hover_word(word("bar"), window, cx)
        });
        cx.executor().advance_clock(RESOLVE_DELAY * 2);
        cx.run_until_parked();
        preview.read_with(&**cx, |preview, _| assert!(preview.declaration.is_none()));

        // Leaving the word briefly (e.g. towards the popover) keeps it visible
        // as long as the mouse comes back within the grace period.
        preview.update_in(&mut **cx, |preview, window, cx| {
            preview.hover_word(word("Foo"), window, cx)
        });
        cx.executor().advance_clock(RESOLVE_DELAY * 2);
        cx.run_until_parked();
        preview.update_in(&mut **cx, |preview, window, cx| {
            preview.hover_word(None, window, cx);
            preview.popover_hovered = true;
        });
        cx.executor().advance_clock(HIDE_DELAY * 2);
        cx.run_until_parked();
        preview.read_with(&**cx, |preview, _| assert!(preview.declaration.is_some()));
    }

    #[test]
    fn snippet_rows_uses_item_extent_and_caps_it() {
        assert_eq!(snippet_rows(Some(3..7), 3, 100), (3..7, false));
        assert_eq!(
            snippet_rows(Some(0..200), 0, 300),
            (0..MAX_DECLARATION_LINES - 1, true)
        );
        // No outline: a few lines from the symbol, clamped to the buffer.
        assert_eq!(
            snippet_rows(None, 5, 100),
            (5..5 + FALLBACK_DECLARATION_LINES - 1, false)
        );
        assert_eq!(snippet_rows(None, 5, 7), (5..7, false));
    }

    #[test]
    fn leading_annotations_are_included() {
        let lines = [
            "fn other() {}",
            "",
            "/// Docs.",
            "#[derive(Debug)]",
            "pub struct Foo {",
        ];
        let line = |row: u32| lines[row as usize].to_string();
        assert_eq!(leading_annotation_start(4, line), 2);
        // Stops at a blank line / code; nothing above row 0.
        assert_eq!(leading_annotation_start(0, line), 0);
        let java = [
            "}",
            "  /**",
            "   * Docs.",
            "   */",
            "  @Entity",
            "  class Foo {",
        ];
        assert_eq!(
            leading_annotation_start(5, |row| java[row as usize].to_string()),
            1
        );
    }

    #[test]
    fn dedent_removes_common_indentation() {
        assert_eq!(
            dedent("    struct Foo {\n        a: u32,\n\n    }"),
            "struct Foo {\n    a: u32,\n\n}"
        );
    }

    #[test]
    fn fence_survives_backticks_in_code() {
        assert_eq!(fenced_code_block("a"), "```\na\n```");
        assert_eq!(
            fenced_code_block("/// ```\n/// x\n/// ```"),
            "````\n/// ```\n/// x\n/// ```\n````"
        );
    }
}
