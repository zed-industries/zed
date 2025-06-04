use super::*;
use crate::{
    display_map::{DisplayMap, FoldMap, InlayMap},
    test::editor_test_context::EditorTestContext,
};
use gpui::{Context, TestAppContext};
use language::{Language, LanguageConfig, LanguageMatcher, LanguageRegistry};
use multi_buffer::MultiBuffer;
use project::Project;
use std::sync::Arc;

#[gpui::test]
async fn test_markdown_link_folding(cx: &mut TestAppContext) {
    init_test(cx);

    let mut cx = EditorTestContext::new(cx).await;

    // Set up markdown language with folding query
    cx.update_buffer(|buffer, cx| {
        let registry = LanguageRegistry::test(cx.background_executor().clone());
        let markdown = Arc::new(Language::new(
            LanguageConfig {
                name: "Markdown".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["md".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        ));

        registry.add(markdown.clone());
        buffer.set_language_registry(registry);
        buffer.set_language(Some(markdown), cx);
    });

    // Add markdown content with links
    cx.set_state(indoc! {"
        # Channel Notes

        Check out [our website](https://example.com) for more info.

        Here's a [link to docs](https://docs.example.com/guide) that you should read.

        And another [reference](https://reference.example.com).
    "});

    cx.update_editor(|editor, cx| {
        // Enable syntax folding
        let display_map = editor.display_map.update(cx, |map, cx| {
            map.syntax_fold_config = SyntaxFoldConfig {
                enabled: true,
                auto_fold_on_open: true,
                proximity_expand_distance: 2,
            };
            map.detect_syntax_folds(cx);
        });

        // Verify folds were created for each link
        let snapshot = editor.snapshot(cx);
        let fold_count = snapshot.fold_count();
        assert_eq!(fold_count, 3, "Should have 3 folds for 3 links");
    });
}

#[gpui::test]
async fn test_link_proximity_expansion(cx: &mut TestAppContext) {
    init_test(cx);

    let mut cx = EditorTestContext::new(cx).await;

    // Set up markdown with a single link
    cx.set_state(indoc! {"
        Check out [this link](https://example.com) for details.
    "});

    cx.update_editor(|editor, cx| {
        // Enable syntax folding with proximity expansion
        editor.display_map.update(cx, |map, cx| {
            map.syntax_fold_config = SyntaxFoldConfig {
                enabled: true,
                auto_fold_on_open: true,
                proximity_expand_distance: 3,
            };
            map.detect_syntax_folds(cx);
        });
    });

    // Move cursor near the link
    cx.set_selections_state(indoc! {"
        Check out |[this link](https://example.com) for details.
    "});

    cx.update_editor(|editor, cx| {
        let display_map = editor.display_map.update(cx, |map, cx| {
            // Simulate cursor movement handling
            let cursor_offset = editor.selections.newest::<usize>(cx).head();
            map.handle_cursor_movement(cursor_offset, cx);
        });

        // Verify link is expanded due to proximity
        let snapshot = editor.snapshot(cx);
        let expanded_count = snapshot.expanded_fold_count();
        assert_eq!(
            expanded_count, 1,
            "Link should be expanded when cursor is near"
        );
    });

    // Move cursor away from the link
    cx.set_selections_state(indoc! {"
        Check out [this link](https://example.com) for details.|
    "});

    cx.update_editor(|editor, cx| {
        let display_map = editor.display_map.update(cx, |map, cx| {
            let cursor_offset = editor.selections.newest::<usize>(cx).head();
            map.handle_cursor_movement(cursor_offset, cx);
        });

        // Verify link is folded again
        let snapshot = editor.snapshot(cx);
        let expanded_count = snapshot.expanded_fold_count();
        assert_eq!(
            expanded_count, 0,
            "Link should be folded when cursor moves away"
        );
    });
}

#[gpui::test]
async fn test_link_click_action(cx: &mut TestAppContext) {
    init_test(cx);

    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state(indoc! {"
        Visit [GitHub](https://github.com) for code.
    "});

    cx.update_editor(|editor, cx| {
        editor.display_map.update(cx, |map, cx| {
            map.syntax_fold_config = SyntaxFoldConfig {
                enabled: true,
                auto_fold_on_open: true,
                proximity_expand_distance: 2,
            };
            map.detect_syntax_folds(cx);
        });
    });

    // Simulate clicking on the folded link
    cx.update_editor(|editor, cx| {
        let point = editor.pixel_position_of_cursor(cx);

        // Find fold at click position
        let display_map = editor.display_map.read(cx);
        let snapshot = display_map.snapshot(cx);
        let click_offset = snapshot.display_point_to_offset(point, cx);

        if let Some(fold) = display_map.syntax_fold_at_offset(click_offset) {
            // Verify the fold has the correct URL action
            assert!(matches!(
                fold.action_data,
                Some(FoldAction::OpenUrl(ref url)) if url == "https://github.com"
            ));

            // Execute the action (in tests, this would be mocked)
            display_map.execute_fold_action(&fold, cx);
        } else {
            panic!("No fold found at click position");
        }
    });
}

#[gpui::test]
async fn test_nested_markdown_structures(cx: &mut TestAppContext) {
    init_test(cx);

    let mut cx = EditorTestContext::new(cx).await;

    // Test with nested brackets and complex markdown
    cx.set_state(indoc! {"
        # Documentation

        See [the [nested] guide](https://example.com/guide) for details.

        ![Image description](https://example.com/image.png)

        Code: `[not a link](just code)`

        > Quote with [link in quote](https://quoted.com)
    "});

    cx.update_editor(|editor, cx| {
        editor.display_map.update(cx, |map, cx| {
            map.syntax_fold_config = SyntaxFoldConfig {
                enabled: true,
                auto_fold_on_open: true,
                proximity_expand_distance: 2,
            };
            map.detect_syntax_folds(cx);
        });

        // Verify correct number of folds (should not fold code block content)
        let snapshot = editor.snapshot(cx);
        let fold_count = snapshot.fold_count();
        assert_eq!(
            fold_count, 3,
            "Should have 3 folds: nested link, image, and quote link"
        );
    });
}

#[gpui::test]
async fn test_fold_persistence_across_edits(cx: &mut TestAppContext) {
    init_test(cx);

    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state(indoc! {"
        First [link one](https://one.com) here.
        Second [link two](https://two.com) here.
    "});

    cx.update_editor(|editor, cx| {
        editor.display_map.update(cx, |map, cx| {
            map.syntax_fold_config = SyntaxFoldConfig {
                enabled: true,
                auto_fold_on_open: true,
                proximity_expand_distance: 2,
            };
            map.detect_syntax_folds(cx);
        });
    });

    // Edit text between links
    cx.set_selections_state(indoc! {"
        First [link one](https://one.com) here|.
        Second [link two](https://two.com) here.
    "});

    cx.simulate_keystroke(" and more text");

    cx.update_editor(|editor, cx| {
        // Verify folds are maintained after edit
        let snapshot = editor.snapshot(cx);
        let fold_count = snapshot.fold_count();
        assert_eq!(fold_count, 2, "Both folds should persist after edit");
    });

    // Add a new link
    cx.simulate_keystroke("\nThird [link three](https://three.com) added.");

    cx.update_editor(|editor, cx| {
        // Verify new fold is detected
        let snapshot = editor.snapshot(cx);
        let fold_count = snapshot.fold_count();
        assert_eq!(fold_count, 3, "New fold should be detected for added link");
    });
}

#[gpui::test]
async fn test_channel_notes_integration(cx: &mut TestAppContext) {
    init_test(cx);

    let mut cx = EditorTestContext::new(cx).await;

    // Simulate channel notes content with many links
    cx.set_state(indoc! {"
        # Project Resources

        ## Documentation
        - [API Reference](https://api.example.com/docs)
        - [User Guide](https://guide.example.com)
        - [Developer Handbook](https://dev.example.com/handbook)

        ## Tools
        - [Build Status](https://ci.example.com/status)
        - [Issue Tracker](https://issues.example.com)
        - [Code Review](https://review.example.com)

        ## External Links
        - [Blog Post](https://blog.example.com/announcement)
        - [Video Tutorial](https://youtube.com/watch?v=demo)
        - [Community Forum](https://forum.example.com)
    "});

    cx.update_editor(|editor, cx| {
        editor.display_map.update(cx, |map, cx| {
            map.syntax_fold_config = SyntaxFoldConfig {
                enabled: true,
                auto_fold_on_open: true,
                proximity_expand_distance: 2,
            };
            map.detect_syntax_folds(cx);
        });

        // Verify all links are folded
        let snapshot = editor.snapshot(cx);
        let fold_count = snapshot.fold_count();
        assert_eq!(fold_count, 9, "All 9 links should be folded");

        // Verify visual appearance (folded links show only text)
        let display_text = snapshot.display_text();
        assert!(display_text.contains("API Reference"));
        assert!(!display_text.contains("https://api.example.com/docs"));
    });
}

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings = crate::test::test_settings(cx);
        cx.set_global(settings);
        theme::init(theme::LoadThemes::JustBase, cx);
        language::init(cx);
        crate::init(cx);
    });
}
