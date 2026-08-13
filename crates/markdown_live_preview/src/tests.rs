use super::*;
use editor::test::editor_test_context::EditorTestContext;
use gpui::TestAppContext;
use language::{Language, LanguageConfig};
use settings::SettingsStore;
use std::sync::Arc;

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        zlog::init_test();
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        crate::init(cx);
    });
}

fn markdown_inline_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "Markdown-Inline".into(),
            hidden: true,
            ..LanguageConfig::default()
        },
        Some(tree_sitter_md::INLINE_LANGUAGE.into()),
    ))
}

async fn markdown_test_context(cx: &mut TestAppContext) -> EditorTestContext {
    init_test(cx);
    let mut cx = EditorTestContext::new(cx).await;
    let registry = cx.language_registry();
    let markdown = language::markdown_lang();
    registry.add(markdown.clone());
    registry.add(markdown_inline_lang());
    cx.update_buffer(|buffer, cx| {
        buffer.set_language(Some(markdown), cx);
    });
    cx.executor().run_until_parked();
    cx
}

#[gpui::test]
async fn test_inline_markers_hidden_off_cursor_line(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;

    cx.set_state(indoc::indoc! {"
        ˇplain line
        some **bold** and *italic* and `code` here
        a [label](https://example.com) link
    "});
    cx.executor().run_until_parked();

    pretty_assertions::assert_eq!(
        cx.display_text(),
        indoc::indoc! {"
            plain line
            some bold and italic and code here
            a label link
        "}
    );
}

#[gpui::test]
async fn test_per_token_reveal(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;

    cx.set_state(indoc::indoc! {"
        ˇplain line
        some **bold** and *italic* text
    "});
    cx.executor().run_until_parked();
    pretty_assertions::assert_eq!(
        cx.display_text(),
        indoc::indoc! {"
            plain line
            some bold and italic text
        "}
    );

    // Cursor inside the bold span reveals only that span's markers; the
    // italic further along the same line stays rendered.
    cx.set_state(indoc::indoc! {"
        plain line
        some **boˇld** and *italic* text
    "});
    cx.executor().run_until_parked();
    pretty_assertions::assert_eq!(
        cx.display_text(),
        indoc::indoc! {"
            plain line
            some **bold** and italic text
        "}
    );

    // Cursor in plain text on the same line reveals nothing.
    cx.set_state(indoc::indoc! {"
        plain line
        some **bold** anˇd *italic* text
    "});
    cx.executor().run_until_parked();
    pretty_assertions::assert_eq!(
        cx.display_text(),
        indoc::indoc! {"
            plain line
            some bold and italic text
        "}
    );

    // A selection sweeping the line reveals everything it touches.
    cx.set_state(indoc::indoc! {"
        plain line
        «some **bold** and *italic* textˇ»
    "});
    cx.executor().run_until_parked();
    pretty_assertions::assert_eq!(
        cx.display_text(),
        indoc::indoc! {"
            plain line
            some **bold** and *italic* text
        "}
    );

    // A list marker stays rendered while editing elsewhere on its line,
    // and reveals only when the cursor touches it.
    cx.set_state(indoc::indoc! {"
        plain line
        - bullet with **bold** insideˇ
    "});
    cx.executor().run_until_parked();
    assert!(cx.display_text().contains("⋯ bullet with bold inside"));
    cx.set_state(indoc::indoc! {"
        plain line
        ˇ- bullet with **bold** inside
    "});
    cx.executor().run_until_parked();
    assert!(cx.display_text().contains("- bullet with bold inside"));

    // The boundary just past the marker's trailing space does not reveal it.
    cx.set_state(indoc::indoc! {"
        plain line
        - ˇbullet with **bold** inside
    "});
    cx.executor().run_until_parked();
    assert!(cx.display_text().contains("⋯ bullet with bold inside"));

    // A checkbox stays rendered while editing the task text.
    cx.set_state(indoc::indoc! {"
        plain line
        - [ ] task textˇ
    "});
    cx.executor().run_until_parked();
    assert!(cx.display_text().contains("⋯ task text"));

    // Moving to another line hides everything again.
    cx.set_state(indoc::indoc! {"
        plainˇ line
        some **bold** and *italic* text
    "});
    cx.executor().run_until_parked();
    pretty_assertions::assert_eq!(
        cx.display_text(),
        indoc::indoc! {"
            plain line
            some bold and italic text
        "}
    );
}

#[gpui::test]
async fn test_list_markers(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;

    cx.set_state(indoc::indoc! {"
        ˇplain line
        - bullet item
        - [ ] open task
        - [x] done task
    "});
    cx.executor().run_until_parked();

    // Bullet and checkbox folds keep the default `⋯` placeholder text; the
    // rendered element replaces it visually.
    pretty_assertions::assert_eq!(
        cx.display_text(),
        indoc::indoc! {"
            plain line
            ⋯ bullet item
            ⋯ open task
            ⋯ done task
        "}
    );

    let markers = cx.update_editor(|editor, _, cx| {
        let markers = extract_markers(editor, cx).expect("markdown buffer should produce markers");
        markers
            .inline
            .iter()
            .filter(|marker| matches!(marker.kind, InlineKind::Checkbox { .. }))
            .cloned()
            .collect::<Vec<_>>()
    });
    assert_eq!(markers.len(), 2);

    // Toggling the open task checks it.
    let InlineKind::Checkbox {
        checked,
        marker_range,
    } = markers[0].kind.clone()
    else {
        panic!("expected checkbox marker");
    };
    assert!(!checked);
    let editor = cx.editor.clone();
    cx.update(|_, cx| {
        toggle_task_marker(&editor.downgrade(), &marker_range, checked, cx);
    });
    cx.executor().run_until_parked();
    assert!(cx.buffer_text().contains("- [x] open task"));

    // Toggling the done task unchecks it.
    let InlineKind::Checkbox {
        checked,
        marker_range,
    } = markers[1].kind.clone()
    else {
        panic!("expected checkbox marker");
    };
    assert!(checked);
    let editor = cx.editor.clone();
    cx.update(|_, cx| {
        toggle_task_marker(&editor.downgrade(), &marker_range, checked, cx);
    });
    cx.executor().run_until_parked();
    assert!(cx.buffer_text().contains("- [ ] done task"));
}

#[gpui::test]
async fn test_block_widgets(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;

    cx.set_state(indoc::indoc! {"
        ˇplain first line
        extra spacing line

        # Heading

        | a | b |
        | - | - |
        | 1 | 2 |

        ---

        ```python
        import pandas as pd
        ```

        > a quote

        last line
    "});
    cx.executor().run_until_parked();

    // Heading, table, horizontal rule, code block, and blockquote.
    assert_eq!(applied_block_count(&mut cx), 5);

    // Moving the cursor onto the heading's row reveals it (removes its
    // block), while everything else stays rendered; an adjacent row does not
    // reveal it.
    cx.set_state(indoc::indoc! {"
        plain first line
        extra spacing line
        ˇ
        # Heading

        | a | b |
        | - | - |
        | 1 | 2 |

        ---

        ```python
        import pandas as pd
        ```

        > a quote

        last line
    "});
    cx.executor().run_until_parked();
    assert_eq!(applied_block_count(&mut cx), 5);

    cx.set_state(indoc::indoc! {"
        plain first line
        extra spacing line

        # Headingˇ

        | a | b |
        | - | - |
        | 1 | 2 |

        ---

        ```python
        import pandas as pd
        ```

        > a quote

        last line
    "});
    cx.executor().run_until_parked();
    assert_eq!(applied_block_count(&mut cx), 4);
}

#[gpui::test]
async fn test_frontmatter_renders_as_block(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;

    cx.set_state(indoc::indoc! {"
        ---
        title: Some Note
        parent: lectures
        ---

        body ˇtext
    "});
    cx.executor().run_until_parked();
    assert_eq!(applied_block_count(&mut cx), 1);

    // Cursor inside the frontmatter reveals the raw YAML.
    cx.set_state(indoc::indoc! {"
        ---
        title: Some Noteˇ
        parent: lectures
        ---

        body text
    "});
    cx.executor().run_until_parked();
    assert_eq!(applied_block_count(&mut cx), 0);
}

fn applied_block_count(cx: &mut EditorTestContext) -> usize {
    cx.update_editor(|editor, _, _| {
        editor
            .addon::<LivePreviewAddon>()
            .unwrap()
            .applied_blocks
            .len()
    })
}

#[gpui::test]
async fn test_keyboard_navigation_reaches_blocks(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;

    cx.set_state(indoc::indoc! {"
        ˇalpha

        # Heading

        omega
    "});
    cx.executor().run_until_parked();
    assert_eq!(applied_block_count(&mut cx), 1);

    // Arrow down until the cursor reaches the heading's buffer row; the
    // rendered block must dissolve so the heading is editable with the
    // keyboard alone.
    let mut reached = false;
    for _ in 0..4 {
        cx.update_editor(|editor, window, cx| {
            editor.move_down(&Default::default(), window, cx);
        });
        cx.executor().run_until_parked();
        let row = cx.update_editor(|editor, _, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            editor
                .selections
                .newest_anchor()
                .head()
                .to_point(&snapshot)
                .row
        });
        if row == 2 {
            reached = true;
            break;
        }
    }
    assert!(reached, "cursor never reached the heading row");
    assert_eq!(applied_block_count(&mut cx), 0);
    assert!(cx.display_text().contains("# Heading"));
}

#[gpui::test]
async fn test_heading_renders_after_typing(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;

    // Simulate typing a heading then pressing enter: the cursor ends up on
    // the line below, and the heading should render immediately.
    cx.set_state("# helloˇ");
    cx.executor().run_until_parked();
    assert_eq!(applied_block_count(&mut cx), 0);

    cx.update_editor(|editor, window, cx| {
        editor.newline(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();
    assert_eq!(applied_block_count(&mut cx), 1);
}

#[gpui::test]
async fn test_disabling_restores_raw_markdown(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;

    let source = indoc::indoc! {"
        ˇplain line
        some **bold** text

        # Heading
    "};
    cx.set_state(source);
    cx.executor().run_until_parked();
    assert_ne!(cx.display_text(), cx.buffer_text());

    cx.update_editor(|editor, _window, cx| {
        if let Some(addon) = editor.addon_mut::<LivePreviewAddon>() {
            addon.enabled_override = Some(false);
        }
        recompute(editor, cx);
    });
    cx.executor().run_until_parked();
    pretty_assertions::assert_eq!(cx.display_text(), cx.buffer_text());

    cx.update_editor(|editor, _window, cx| {
        if let Some(addon) = editor.addon_mut::<LivePreviewAddon>() {
            addon.enabled_override = Some(true);
        }
        recompute(editor, cx);
    });
    cx.executor().run_until_parked();
    assert_ne!(cx.display_text(), cx.buffer_text());
}

#[gpui::test]
async fn test_extended_markdown_coverage(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;

    cx.set_state(indoc::indoc! {"
        ˇplain line
        visit <https://example.com> or [text][ref] or [collapsed][]

        Setext Title
        ============

        <div>html block</div>
    "});
    cx.executor().run_until_parked();

    // Autolink angle brackets and reference-link syntax are concealed.
    assert!(
        cx.display_text()
            .contains("visit https://example.com or text or collapsed")
    );
    // Setext heading and HTML block render as widgets.
    assert_eq!(applied_block_count(&mut cx), 2);

    // Plain bracketed prose is NOT treated as a link: tree-sitter cannot
    // resolve reference definitions, so `[TODO]` must keep its brackets.
    cx.set_state("ˇplain line\nthis is [TODO] for later\n");
    cx.executor().run_until_parked();
    assert!(cx.display_text().contains("this is [TODO] for later"));

    // Mid-sentence images conceal their syntax, leaving the alt text.
    cx.set_state("ˇplain line\nbroken: ![missing image](nonexistent.png) here\n");
    cx.executor().run_until_parked();
    assert!(cx.display_text().contains("broken: missing image here"));
}

#[gpui::test]
async fn test_restored_generic_folds_are_removed(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state("ˇplain\nsome **bold** text\n");
    cx.executor().run_until_parked();

    // Simulate a fold restored from a session saved before concealment folds
    // were excluded from persistence: a default `⋯` placeholder sitting
    // exactly on the `**` marker before "bold".
    cx.update_editor(|editor, window, cx| {
        editor.fold_ranges(
            vec![MultiBufferOffset(11)..MultiBufferOffset(13)],
            false,
            window,
            cx,
        );
    });
    assert!(cx.display_text().contains('⋯'));

    // The next reparse heals it: the stale fold is removed and the marker is
    // concealed again.
    cx.set_state("editedˇ\nsome **bold** text\n");
    cx.executor().run_until_parked();
    pretty_assertions::assert_eq!(cx.display_text(), "edited\nsome bold text\n");
}

#[gpui::test]
async fn test_non_markdown_buffers_untouched(cx: &mut TestAppContext) {
    init_test(cx);
    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state("ˇsome **text** that is not markdown\n");
    cx.executor().run_until_parked();

    pretty_assertions::assert_eq!(cx.display_text(), cx.buffer_text());
    let has_decorations = cx.update_editor(|editor, _, _| {
        editor
            .addon::<LivePreviewAddon>()
            .is_some_and(|addon| !addon.applied_blocks.is_empty())
    });
    assert!(!has_decorations);
}

#[gpui::test]
async fn test_linked_image_renders_as_block(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(
        "ˇplain line\n\n[![Clickable image](https://example.com/a.png)](https://example.com)\n",
    );
    cx.executor().run_until_parked();
    let block_count = applied_block_count(&mut cx);
    if block_count != 1 {
        let debug: Vec<String> = cx.update_editor(|editor, _, cx| {
            let buffer = editor.buffer().read(cx).as_singleton().unwrap();
            let snapshot = buffer.read(cx).snapshot();
            snapshot
                .syntax_layers()
                .flat_map(|layer| {
                    let mut nodes = Vec::new();
                    let mut stack = vec![layer.node()];
                    while let Some(node) = stack.pop() {
                        nodes.push(format!("{} {:?}", node.kind(), node.byte_range()));
                        for index in (0..node.child_count() as u32).rev() {
                            if let Some(child) = node.child(index) {
                                stack.push(child);
                            }
                        }
                    }
                    nodes
                })
                .collect()
        });
        panic!(
            "expected 1 linked-image block, got {block_count}; tree was:\n{}",
            debug.join("\n")
        );
    }
}


#[gpui::test]
async fn test_images_section_context(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {r#"
        ˇ## 9. Images

        Inline image:

        ![Placeholder image](https://placehold.co/300x150.png "A placeholder")

        Reference-style image:

        ![Placeholder ref][img]

        Image as a link:

        [![Clickable image](https://placehold.co/150x60.png)](https://example.com)

        Broken image (alt text should show): ![missing image](nonexistent.png)

        [img]: https://placehold.co/200x100.png
    "#});
    cx.executor().run_until_parked();
    let (blocks, rows): (usize, Vec<(u32, u32)>) = cx.update_editor(|editor, _, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let addon = editor.addon::<LivePreviewAddon>().unwrap();
        let rows = addon
            .applied_blocks
            .iter()
            .map(|block| {
                (
                    block.range.start.to_point(&snapshot).row,
                    block.range.end.to_point(&snapshot).row,
                )
            })
            .collect();
        (addon.applied_blocks.len(), rows)
    });
    // Inline image, reference image, and linked image; the heading is
    // revealed because the cursor sits on it.
    assert_eq!(blocks, 3, "applied block rows: {rows:?}");
    assert!(rows.contains(&(12, 12)), "linked image row missing: {rows:?}");
}

#[gpui::test]
async fn test_concealments_invisible_to_fold_machinery(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state("ˇplain line\nsome **bold** text\n");
    cx.executor().run_until_parked();
    assert!(cx.display_text().contains("some bold text"));

    // "Unfold All" must not reveal concealments: they are not folds.
    cx.update_editor(|editor, window, cx| {
        editor.unfold_all(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();
    assert!(
        cx.display_text().contains("some bold text"),
        "unfold-all revealed concealments: {}",
        cx.display_text()
    );

    // Fold queries (what the gutter and fold persistence read) see nothing.
    let fold_count = cx.update_editor(|editor, window, cx| {
        let _ = &window;
        let snapshot = editor.snapshot(window, cx);
        let len = snapshot.buffer_snapshot().len();
        snapshot.folds_in_range(MultiBufferOffset(0)..len).count()
    });
    assert_eq!(fold_count, 0);

    // No row reads as folded (the gutter's chevron predicate), across
    // concealed inline markers, bullets, headings, and rules.
    cx.set_state(indoc::indoc! {"
        ˇplain line

        # Heading

        ---

        - bullet with **bold** text
        - [ ] a task

        [a link](https://example.com)
    "});
    cx.executor().run_until_parked();
    // The gutter's chevron predicate: fold-map folds only.
    let folded_rows: Vec<u32> = cx.update_editor(|editor, window, cx| {
        let _ = &window;
        let snapshot = editor.snapshot(window, cx);
        let max_row = snapshot.buffer_snapshot().max_point().row;
        (0..=max_row)
            .filter(|row| {
                snapshot
                    .fold_snapshot()
                    .is_line_folded(multi_buffer::MultiBufferRow(*row))
            })
            .collect()
    });
    assert_eq!(folded_rows, Vec::<u32>::new());
}

#[test]
fn test_image_source_resolution_decodes_percent_encoding() {
    let dir = std::env::temp_dir().join("mdlp-resolver-test");
    std::fs::create_dir_all(&dir).unwrap();
    // macOS screenshot names mix ASCII spaces (percent-encoded in links)
    // with a raw narrow no-break space before "AM".
    let file_name = "Screenshot 2026-08-09 at 11.38.37\u{202f}AM.png";
    std::fs::write(dir.join(file_name), b"png").unwrap();

    let destination = "Screenshot%202026-08-09%20at%2011.38.37\u{202f}AM.png";
    assert!(
        resolve_image_source(destination, Some(&dir)).is_some(),
        "percent-encoded path failed to resolve"
    );
    assert!(resolve_image_source("missing%20file.png", Some(&dir)).is_none());
    std::fs::remove_dir_all(&dir).ok();
}

#[gpui::test]
async fn test_image_size_syntax(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line

        ![sized|640](a.png)

        ![sized dims|320x200](b.png)

        ![unsized](c.png)
    "});
    cx.executor().run_until_parked();
    let widths: Vec<Option<f32>> = cx.update_editor(|editor, _, cx| {
        extract_markers(editor, cx)
            .unwrap()
            .blocks
            .iter()
            .filter_map(|block| match block.kind {
                BlockRenderKind::Image { display_width, .. } => Some(display_width),
                _ => None,
            })
            .collect()
    });
    assert_eq!(widths, vec![Some(640.), Some(320.), None]);
}

#[test]
fn test_with_image_width_rewrites_alt() {
    assert_eq!(
        with_image_width("![alt](a.png)", 500).as_deref(),
        Some("![alt|500](a.png)")
    );
    assert_eq!(
        with_image_width("![alt|300](a.png)", 500).as_deref(),
        Some("![alt|500](a.png)")
    );
    assert_eq!(
        with_image_width("![alt|320x200](a.png)", 500).as_deref(),
        Some("![alt|500](a.png)")
    );
    assert_eq!(
        with_image_width("![](a.png)", 500).as_deref(),
        Some("![|500](a.png)")
    );
    assert_eq!(
        with_image_width("![ref style|300][img]", 500).as_deref(),
        Some("![ref style|500][img]")
    );
    // A pipe that is not a size suffix stays intact.
    assert_eq!(
        with_image_width("![a|b](a.png)", 500).as_deref(),
        Some("![a|b|500](a.png)")
    );
}

#[gpui::test]
async fn test_wikilinks_conceal(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line
        see [[CLAUDE]] and [[notes/plan|the plan]] here
        embed stays raw: ![[image.png]]
        code stays raw: `[[not a link]]`
    "});
    cx.executor().run_until_parked();
    let display = cx.display_text();
    assert!(display.contains("see CLAUDE and the plan here"), "{display}");
    assert!(display.contains("embed stays raw: ![[image.png]]"), "{display}");
    assert!(display.contains("code stays raw: [[not a link]]"), "{display}");
}

#[gpui::test]
async fn test_table_structure_extraction(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line

        | Left | Center | Right |
        |:-----|:------:|------:|
        | a    |   b    |     c |
        | d    |   e    |     f |
    "});
    cx.executor().run_until_parked();
    let (structure, cell_texts) = cx.update_editor(|editor, _, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let markers = extract_markers(editor, cx).unwrap();
        let structure = markers
            .blocks
            .iter()
            .find_map(|block| match &block.kind {
                BlockRenderKind::Table(structure) => Some(structure.clone()),
                _ => None,
            })
            .expect("table structure");
        let texts: Vec<String> = structure
            .cells_in_order()
            .iter()
            .map(|range| {
                let start = range.start.to_offset(&snapshot);
                let end = range.end.to_offset(&snapshot);
                snapshot
                    .text_for_range(start..end)
                    .collect::<String>()
                    .trim()
                    .to_string()
            })
            .collect();
        (structure, texts)
    });
    assert_eq!(structure.header.len(), 3);
    assert_eq!(structure.rows.len(), 2);
    assert_eq!(
        structure.alignments,
        vec![
            CellAlignment::Left,
            CellAlignment::Center,
            CellAlignment::Right
        ]
    );
    assert_eq!(
        cell_texts,
        vec!["Left", "Center", "Right", "a", "b", "c", "d", "e", "f"]
    );

    // Empty cells omitted from the syntax tree are padded to a rectangle.
    cx.set_state(indoc::indoc! {"
        ˇplain line

        | A | B | C |
        |---|---|---|
        | 1 |   | 3 |
        |   | 2 |   |
    "});
    cx.executor().run_until_parked();
    let structure = cx.update_editor(|editor, _, cx| {
        extract_markers(editor, cx)
            .unwrap()
            .blocks
            .iter()
            .find_map(|block| match &block.kind {
                BlockRenderKind::Table(structure) => Some(structure.clone()),
                _ => None,
            })
            .expect("table structure")
    });
    assert_eq!(structure.header.len(), 3);
    assert!(structure.rows.iter().all(|row| row.len() == 3));
    // Empty cells sit between pipes, so they are real editable ranges with
    // correct column identity — never sentinels shifted to the row's end.
    let row_texts: Vec<Vec<String>> = cx.update_editor(|editor, _, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        structure
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| {
                        assert!(cell.start != Anchor::Min, "unexpected sentinel cell");
                        let start = cell.start.to_offset(&snapshot);
                        let end = cell.end.to_offset(&snapshot);
                        snapshot
                            .text_for_range(start..end)
                            .collect::<String>()
                            .trim()
                            .to_string()
                    })
                    .collect()
            })
            .collect()
    });
    assert_eq!(row_texts, vec![vec!["1", "", "3"], vec!["", "2", ""]]);
}

#[gpui::test]
async fn test_table_structural_changes(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line

        | A | B |
        | --- | --- |
        | 1 | 2 |
    "});
    cx.executor().run_until_parked();
    let range = cx.update_editor(|editor, _, cx| {
        extract_markers(editor, cx)
            .unwrap()
            .blocks
            .iter()
            .find_map(|block| match &block.kind {
                BlockRenderKind::Table(_) => Some(block.range.clone()),
                _ => None,
            })
            .expect("table")
    });
    cx.update_editor(|editor, _, cx| {
        apply_table_structural_change(editor, &range, TableStructuralChange::AddColumn, cx);
    });
    cx.executor().run_until_parked();
    assert!(cx.buffer_text().contains("| A | B |   |"), "{}", cx.buffer_text());

    // Reuse the now-stale range on purpose: structural ops must re-resolve the
    // table from the live tree, since widget closures outlive edits.
    cx.update_editor(|editor, _, cx| {
        apply_table_structural_change(editor, &range, TableStructuralChange::AddRow, cx);
    });
    cx.executor().run_until_parked();
    let text = cx.buffer_text();
    let table_lines: Vec<&str> = text.lines().filter(|line| line.starts_with('|')).collect();
    assert_eq!(table_lines.len(), 4, "{text}");
    assert!(
        text.lines().filter(|line| line.starts_with('|')).all(|line| line.matches('|').count() == 4),
        "every table line should have 3 columns after the changes: {text}"
    );
    assert_eq!(table_lines.len(), 4, "{text}");
}


#[gpui::test]
async fn test_add_row_no_outer_pipes(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line

        Name | Value
        --- | ---
        alpha | 1
        beta | 2

        after
    "});
    cx.executor().run_until_parked();
    let range = cx.update_editor(|editor, _, cx| {
        extract_markers(editor, cx)
            .unwrap()
            .blocks
            .iter()
            .find_map(|block| match &block.kind {
                BlockRenderKind::Table(_) => Some(block.range.clone()),
                _ => None,
            })
            .expect("table")
    });
    cx.update_editor(|editor, _, cx| {
        apply_table_structural_change(editor, &range, TableStructuralChange::AddRow, cx);
    });
    cx.executor().run_until_parked();
    let text = cx.buffer_text();
    let table_lines: Vec<&str> = text.lines().filter(|line| line.starts_with('|')).collect();
    assert_eq!(
        table_lines,
        vec![
            "| Name | Value |",
            "| --- | --- |",
            "| alpha | 1 |",
            "| beta | 2 |",
            "|   |   |",
        ],
        "full text: {text:?}"
    );
    assert!(text.contains("\n\nafter"), "must not eat the blank line: {text:?}");
}

#[gpui::test]
async fn test_rapid_structural_changes_between_reparses(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line

        | A | B |
        | --- | --- |
        | 1 | 2 |
    "});
    cx.executor().run_until_parked();
    let range = cx.update_editor(|editor, _, cx| {
        extract_markers(editor, cx)
            .unwrap()
            .blocks
            .iter()
            .find_map(|block| match &block.kind {
                BlockRenderKind::Table(_) => Some(block.range.clone()),
                _ => None,
            })
            .expect("table")
    });
    // Two clicks in quick succession: no run_until_parked between them, so
    // the second acts before any reparse — it must still resolve the table
    // from live text instead of the stale tree.
    cx.update_editor(|editor, _, cx| {
        apply_table_structural_change(editor, &range, TableStructuralChange::AddRow, cx);
        apply_table_structural_change(editor, &range, TableStructuralChange::AddRow, cx);
        apply_table_structural_change(editor, &range, TableStructuralChange::AddColumn, cx);
    });
    cx.executor().run_until_parked();
    let text = cx.buffer_text();
    let table_lines: Vec<&str> = text.lines().filter(|line| line.starts_with('|')).collect();
    assert_eq!(
        table_lines,
        vec![
            "| A | B |   |",
            "| --- | --- | --- |",
            "| 1 | 2 |   |",
            "|   |   |   |",
            "|   |   |   |",
        ],
        "full text: {text:?}"
    );
}

#[gpui::test]
async fn test_add_row_extends_widget_over_empty_row(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇMinimal table without outer pipes:

        Name | Value
        --- | ---
        alpha | 1
        beta | 2

        Table with empty cells:
    "});
    cx.executor().run_until_parked();
    let range = cx.update_editor(|editor, _, cx| {
        extract_markers(editor, cx)
            .unwrap()
            .blocks
            .iter()
            .find_map(|block| match &block.kind {
                BlockRenderKind::Table(_) => Some(block.range.clone()),
                _ => None,
            })
            .expect("table")
    });
    cx.update_editor(|editor, _, cx| {
        apply_table_structural_change(editor, &range, TableStructuralChange::AddRow, cx);
    });
    cx.executor().run_until_parked();
    // tree-sitter-md errors on the all-empty row and ends its table node
    // early; the widget must still span the full textual table.
    let tables: Vec<(u32, u32, usize)> = cx.update_editor(|editor, _, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        extract_markers(editor, cx)
            .unwrap()
            .blocks
            .iter()
            .filter_map(|block| match &block.kind {
                BlockRenderKind::Table(structure) => Some((
                    block.range.start.to_point(&snapshot).row,
                    block.range.end.to_point(&snapshot).row,
                    structure.rows.len(),
                )),
                _ => None,
            })
            .collect()
    });
    assert_eq!(tables, vec![(2, 6, 3)], "text: {:?}", cx.buffer_text());
}

#[gpui::test]
async fn test_table_reveals_only_via_button(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line

        | A | B |
        | --- | --- |
        | 1 | 2 |
    "});
    cx.executor().run_until_parked();
    assert_eq!(applied_block_count(&mut cx), 1);

    // Cursor inside the table does NOT reveal its source.
    cx.set_state(indoc::indoc! {"
        plain line

        | A | B |
        | --- | --- |
        | 1 |ˇ 2 |
    "});
    cx.executor().run_until_parked();
    assert_eq!(applied_block_count(&mut cx), 1);

    // Marking the block explicitly revealed (what the `</>` button does)
    // removes the widget while the cursor stays inside.
    cx.update_editor(|editor, _, cx| {
        let range = extract_markers(editor, cx)
            .unwrap()
            .blocks
            .iter()
            .find_map(|block| match &block.kind {
                BlockRenderKind::Table(_) => Some(block.range.clone()),
                _ => None,
            })
            .expect("table");
        if let Some(addon) = editor.addon_mut::<LivePreviewAddon>() {
            addon.source_revealed = Some(range);
        }
        apply_decorations(editor, cx);
    });
    cx.executor().run_until_parked();
    assert_eq!(applied_block_count(&mut cx), 0);

    // Moving the cursor out re-renders the widget and clears the reveal.
    cx.set_state(indoc::indoc! {"
        plain lineˇ

        | A | B |
        | --- | --- |
        | 1 | 2 |
    "});
    cx.executor().run_until_parked();
    assert_eq!(applied_block_count(&mut cx), 1);
    let cleared = cx.update_editor(|editor, _, _| {
        editor
            .addon::<LivePreviewAddon>()
            .and_then(|addon| addon.source_revealed.clone())
            .is_none()
    });
    assert!(cleared, "reveal must clear when the selection leaves the block");
}

#[gpui::test]
async fn test_move_and_delete_rows_columns(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line

        | A | B | C |
        | --- | --- | --- |
        | 1 | 2 | 3 |
        | x | y | z |
    "});
    cx.executor().run_until_parked();
    let range = cx.update_editor(|editor, _, cx| {
        extract_markers(editor, cx)
            .unwrap()
            .blocks
            .iter()
            .find_map(|block| match &block.kind {
                BlockRenderKind::Table(_) => Some(block.range.clone()),
                _ => None,
            })
            .expect("table")
    });
    cx.update_editor(|editor, _, cx| {
        apply_table_structural_change(
            editor,
            &range,
            TableStructuralChange::MoveRow { from: 0, to: 1 },
            cx,
        );
    });
    cx.executor().run_until_parked();
    assert!(
        cx.buffer_text().contains("| x | y | z |\n| 1 | 2 | 3 |"),
        "{}",
        cx.buffer_text()
    );

    cx.update_editor(|editor, _, cx| {
        apply_table_structural_change(
            editor,
            &range,
            TableStructuralChange::MoveColumn { from: 0, to: 2 },
            cx,
        );
    });
    cx.executor().run_until_parked();
    // Move semantics: remove+insert (drag across positions), not swap.
    assert!(
        cx.buffer_text().contains("| B | C | A |"),
        "{}",
        cx.buffer_text()
    );

    cx.update_editor(|editor, _, cx| {
        apply_table_structural_change(editor, &range, TableStructuralChange::DeleteRow(1), cx);
        apply_table_structural_change(editor, &range, TableStructuralChange::DeleteColumn(1), cx);
    });
    cx.executor().run_until_parked();
    let text = cx.buffer_text();
    assert!(text.contains("| B | A |"), "{text}");
    assert!(text.contains("| y | x |"), "{text}");
    assert!(!text.contains("| 1 |"), "deleted row should be gone: {text}");
}

#[gpui::test]
async fn test_drag_row_reorders_via_mouse(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line

        | A | B |
        | --- | --- |
        | one | 1 |
        | two | 2 |
    "});
    cx.executor().run_until_parked();

    let handle = cx
        .cx
        .debug_bounds("mdlp-row-handle-0")
        .expect("row handle rendered");
    let target = cx
        .cx
        .debug_bounds("mdlp-cell-1-0")
        .expect("target cell rendered");

    let start = handle.center();
    cx.cx
        .simulate_mouse_down(start, gpui::MouseButton::Left, gpui::Modifiers::none());
    // Cross the drag threshold, then hover the target row.
    cx.cx.simulate_mouse_move(
        start + gpui::point(gpui::px(0.), gpui::px(6.)),
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    let below_target = target.center() + gpui::point(gpui::px(0.), target.size.height * 0.3);
    cx.cx.simulate_mouse_move(
        below_target,
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    cx.executor().run_until_parked();

    let (drag_active, source_set, boundary_set) = cx.update_editor(|editor, _, cx| {
        let addon = editor.addon::<LivePreviewAddon>().expect("addon");
        (
            cx.has_active_drag(),
            addon.drag_source.as_ref().map(|s| s.unit),
            addon.drop_boundary.as_ref().map(|(_, boundary)| *boundary),
        )
    });
    assert!(drag_active, "drag should be active after moving past threshold");
    assert_eq!(source_set, Some(TableUnit::Row(0)), "source should be recorded");
    assert_eq!(
        boundary_set,
        Some(TableBoundary::Row(2)),
        "lower half of row 1 should target the boundary below it"
    );

    cx.cx.simulate_mouse_up(
        below_target,
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    cx.executor().run_until_parked();

    let text = cx.buffer_text();
    let rows: Vec<&str> = text.lines().filter(|line| line.starts_with('|')).collect();
    assert_eq!(
        rows,
        vec!["| A | B |", "| --- | --- |", "| two | 2 |", "| one | 1 |"],
        "rows should be reordered by the drop: {text:?}"
    );
}

#[gpui::test]
async fn test_drag_row_drop_anywhere_on_table(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line

        | A | B |
        | --- | --- |
        | one | 1 |
        | two | 2 |
    "});
    cx.executor().run_until_parked();

    let handle = cx
        .cx
        .debug_bounds("mdlp-row-handle-0")
        .expect("row handle rendered");
    let target_cell = cx
        .cx
        .debug_bounds("mdlp-cell-1-0")
        .expect("target cell rendered");
    let target_handle = cx
        .cx
        .debug_bounds("mdlp-row-handle-1")
        .expect("target row handle rendered");

    let start = handle.center();
    cx.cx
        .simulate_mouse_down(start, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.cx.simulate_mouse_move(
        start + gpui::point(gpui::px(0.), gpui::px(6.)),
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    // Hover the target row's lower half (tracks the below-boundary), then
    // drift onto the row HANDLE and release there — like a user following
    // the pill column.
    cx.cx.simulate_mouse_move(
        target_cell.center() + gpui::point(gpui::px(0.), target_cell.size.height * 0.3),
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    let handle_lower = target_handle.center()
        + gpui::point(gpui::px(0.), target_handle.size.height * 0.3);
    cx.cx.simulate_mouse_move(
        handle_lower,
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    cx.cx.simulate_mouse_up(
        handle_lower,
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    cx.executor().run_until_parked();

    let text = cx.buffer_text();
    let rows: Vec<&str> = text.lines().filter(|line| line.starts_with('|')).collect();
    assert_eq!(
        rows,
        vec!["| A | B |", "| --- | --- |", "| two | 2 |", "| one | 1 |"],
        "release anywhere over the table should still apply the tracked drop: {text:?}"
    );
}

#[gpui::test]
async fn test_drag_column_release_on_handle_strip(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line

        | A | B |
        | --- | --- |
        | one | 1 |
    "});
    cx.executor().run_until_parked();

    let source = cx
        .cx
        .debug_bounds("mdlp-column-handle-0")
        .expect("column handle rendered");
    let target = cx
        .cx
        .debug_bounds("mdlp-column-handle-1")
        .expect("second column handle rendered");

    let start = source.center();
    cx.cx
        .simulate_mouse_down(start, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.cx.simulate_mouse_move(
        start + gpui::point(gpui::px(6.), gpui::px(0.)),
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    let right_half = target.center() + gpui::point(target.size.width * 0.3, gpui::px(0.));
    cx.cx.simulate_mouse_move(
        right_half,
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    cx.cx.simulate_mouse_up(
        right_half,
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    cx.executor().run_until_parked();

    let text = cx.buffer_text();
    assert!(
        text.contains("| B | A |"),
        "dragging a column along the handle strip should reorder: {text:?}"
    );
}

#[gpui::test]
async fn test_drag_survives_mid_gesture_repaint(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line

        | A | B |
        | --- | --- |
        | one | 1 |
        | two | 2 |
    "});
    cx.executor().run_until_parked();

    let handle = cx
        .cx
        .debug_bounds("mdlp-row-handle-0")
        .expect("row handle rendered");
    let target = cx
        .cx
        .debug_bounds("mdlp-cell-1-0")
        .expect("target cell rendered");

    let start = handle.center();
    cx.cx
        .simulate_mouse_down(start, gpui::MouseButton::Left, gpui::Modifiers::none());
    // A repaint lands between press and first movement (cursor blink, agent
    // panel updates, etc.) — the armed gesture must survive it.
    cx.update_editor(|_, _, cx| cx.notify());
    cx.executor().run_until_parked();
    cx.cx.simulate_mouse_move(
        start + gpui::point(gpui::px(0.), gpui::px(6.)),
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    cx.update_editor(|_, _, cx| cx.notify());
    cx.executor().run_until_parked();
    let below_target = target.center() + gpui::point(gpui::px(0.), target.size.height * 0.3);
    cx.cx.simulate_mouse_move(
        below_target,
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    cx.cx.simulate_mouse_up(
        below_target,
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    cx.executor().run_until_parked();

    let text = cx.buffer_text();
    let rows: Vec<&str> = text.lines().filter(|line| line.starts_with('|')).collect();
    assert_eq!(
        rows,
        vec!["| A | B |", "| --- | --- |", "| two | 2 |", "| one | 1 |"],
        "drag should survive repaints mid-gesture: {text:?}"
    );
}

#[gpui::test]
async fn test_drag_column_between_others(cx: &mut TestAppContext) {
    let mut cx = markdown_test_context(cx).await;
    cx.set_state(indoc::indoc! {"
        ˇplain line

        | A | B | C |
        | --- | --- | --- |
        | 1 |   | 3 |
    "});
    cx.executor().run_until_parked();

    let source = cx
        .cx
        .debug_bounds("mdlp-column-handle-0")
        .expect("column handle rendered");
    let target = cx.cx.debug_bounds("mdlp-cell-h-2").expect("header C rendered");

    let start = source.center();
    cx.cx
        .simulate_mouse_down(start, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.cx.simulate_mouse_move(
        start + gpui::point(gpui::px(6.), gpui::px(0.)),
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    // Left half of column C targets the boundary between B and C.
    let left_half = target.center() - gpui::point(target.size.width * 0.3, gpui::px(0.));
    cx.cx.simulate_mouse_move(
        left_half,
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    let boundary = cx.update_editor(|editor, _, _| {
        editor
            .addon::<LivePreviewAddon>()
            .and_then(|addon| addon.drop_boundary.as_ref().map(|(_, boundary)| *boundary))
    });
    assert_eq!(
        boundary,
        Some(TableBoundary::Column(2)),
        "left half of C should target the B|C boundary"
    );
    cx.cx
        .simulate_mouse_up(left_half, gpui::MouseButton::Left, gpui::Modifiers::none());
    cx.executor().run_until_parked();

    let text = cx.buffer_text();
    assert!(
        text.contains("| B | A | C |"),
        "A dropped between B and C: {text:?}"
    );
}
