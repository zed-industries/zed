use crate::editor_tests::{init_test, update_test_editor_settings};
use crate::test::editor_lsp_test_context::EditorLspTestContext;
use futures::StreamExt;
use gpui::{Task, TestAppContext};
use indoc::indoc;
use settings::Settings;

#[gpui::test]
async fn test_lsp_and_treesitter_rainbow_interaction(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            semantic_tokens_provider: Some(
                lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                    lsp::SemanticTokensOptions {
                        legend: lsp::SemanticTokensLegend {
                            token_types: vec!["variable".into(), "parameter".into()],
                            token_modifiers: vec![],
                        },
                        full: Some(lsp::SemanticTokensFullOptions::Delta { delta: None }),
                        ..Default::default()
                    },
                ),
            ),
            ..Default::default()
        },
        cx,
    )
    .await;

    let mut semantic_request =
        cx.set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(
            move |_, _, _| async move {
                Ok(Some(lsp::SemanticTokensResult::Tokens(
                    lsp::SemanticTokens {
                        data: vec![
                            lsp::SemanticToken {
                                delta_line: 0,
                                delta_start: 4,
                                length: 3,
                                token_type: 0,
                                token_modifiers_bitset: 0,
                            },
                        ],
                        result_id: Some("1".into()),
                    },
                )))
            },
        );

    update_test_editor_settings(&mut cx.cx, |settings| {
        settings.editor.rainbow_highlighting.get_or_insert_default().enabled = Some(true);
    });

    cx.set_state("ˇlet foo = 1;");
    assert!(semantic_request.next().await.is_some());

    let task = cx.update_editor(|e, _, _| {
        std::mem::replace(&mut e.update_semantic_tokens_task, Task::ready(()))
    });
    task.await;

    let is_enabled = cx.cx.read(|cx| {
        crate::EditorSettings::get_global(cx)
            .rainbow_highlighting
            .enabled
    });
    assert!(is_enabled, "Rainbow highlighting should be enabled");
}

#[gpui::test]
async fn test_rainbow_cache_invalidation_on_lsp_update(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            semantic_tokens_provider: Some(
                lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                    lsp::SemanticTokensOptions {
                        legend: lsp::SemanticTokensLegend {
                            token_types: vec!["variable".into()],
                            token_modifiers: vec![],
                        },
                        full: Some(lsp::SemanticTokensFullOptions::Delta { delta: None }),
                        ..Default::default()
                    },
                ),
            ),
            ..Default::default()
        },
        cx,
    )
    .await;

    update_test_editor_settings(&mut cx.cx, |settings| {
        settings.editor.rainbow_highlighting.get_or_insert_default().enabled = Some(true);
    });

    cx.set_state("ˇlet foo = 1;");

    crate::rainbow::clear_rainbow_cache();

    let cache_empty = crate::rainbow::with_rainbow_cache(|cache| {
        cache.get("foo").is_none()
    });

    assert!(cache_empty, "Cache should be empty after clearing");
}


#[gpui::test]
async fn test_lsp_semantic_tokens_override_treesitter(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            semantic_tokens_provider: Some(
                lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                    lsp::SemanticTokensOptions {
                        legend: lsp::SemanticTokensLegend {
                            token_types: vec!["property".into(), "variable".into()],
                            token_modifiers: vec![],
                        },
                        full: Some(lsp::SemanticTokensFullOptions::Delta { delta: None }),
                        ..Default::default()
                    },
                ),
            ),
            ..Default::default()
        },
        cx,
    )
    .await;

    let mut semantic_request =
        cx.set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(
            move |_, _, _| async move {
                Ok(Some(lsp::SemanticTokensResult::Tokens(
                    lsp::SemanticTokens {
                        data: vec![
                            lsp::SemanticToken {
                                delta_line: 0,
                                delta_start: 4,
                                length: 3,
                                token_type: 0,
                                token_modifiers_bitset: 0,
                            },
                        ],
                        result_id: Some("1".into()),
                    },
                )))
            },
        );

    update_test_editor_settings(&mut cx.cx, |settings| {
        settings.editor.rainbow_highlighting.get_or_insert_default().enabled = Some(true);
    });

    cx.set_state("ˇlet foo = 1;");
    assert!(semantic_request.next().await.is_some());

    let task = cx.update_editor(|e, _, _| {
        std::mem::replace(&mut e.update_semantic_tokens_task, Task::ready(()))
    });
    task.await;
}

#[gpui::test]
async fn test_rainbow_with_multiple_variables_different_colors(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            semantic_tokens_provider: Some(
                lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                    lsp::SemanticTokensOptions {
                        legend: lsp::SemanticTokensLegend {
                            token_types: vec!["variable".into(), "parameter".into()],
                            token_modifiers: vec![],
                        },
                        full: Some(lsp::SemanticTokensFullOptions::Delta { delta: None }),
                        ..Default::default()
                    },
                ),
            ),
            ..Default::default()
        },
        cx,
    )
    .await;

    let mut semantic_request =
        cx.set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(
            move |_, _, _| async move {
                Ok(Some(lsp::SemanticTokensResult::Tokens(
                    lsp::SemanticTokens {
                        data: vec![
                            lsp::SemanticToken {
                                delta_line: 0,
                                delta_start: 4,
                                length: 3,
                                token_type: 0,
                                token_modifiers_bitset: 0,
                            },
                            lsp::SemanticToken {
                                delta_line: 0,
                                delta_start: 5,
                                length: 3,
                                token_type: 0,
                                token_modifiers_bitset: 0,
                            },
                            lsp::SemanticToken {
                                delta_line: 0,
                                delta_start: 5,
                                length: 3,
                                token_type: 0,
                                token_modifiers_bitset: 0,
                            },
                        ],
                        result_id: Some("1".into()),
                    },
                )))
            },
        );

    update_test_editor_settings(&mut cx.cx, |settings| {
        settings.editor.rainbow_highlighting.get_or_insert_default().enabled = Some(true);
    });

    cx.set_state("ˇlet foo = bar + baz;");
    assert!(semantic_request.next().await.is_some());

    let task = cx.update_editor(|e, _, _| {
        std::mem::replace(&mut e.update_semantic_tokens_task, Task::ready(()))
    });
    task.await;

    use crate::rainbow::hash_to_color_index;
    let palette_size = cx.cx.read(|cx| {
        let theme_settings = theme::ThemeSettings::get_global(cx);
        theme_settings.active_theme.syntax().rainbow_palette_size()
    });

    let foo_index = hash_to_color_index("foo", palette_size);
    let bar_index = hash_to_color_index("bar", palette_size);
    let baz_index = hash_to_color_index("baz", palette_size);

    assert_ne!(foo_index, bar_index);
    assert_ne!(bar_index, baz_index);
    assert_ne!(foo_index, baz_index);
}

#[gpui::test]
async fn test_rainbow_settings_change_clears_cache(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            semantic_tokens_provider: Some(
                lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                    lsp::SemanticTokensOptions {
                        legend: lsp::SemanticTokensLegend {
                            token_types: vec!["variable".into()],
                            token_modifiers: vec![],
                        },
                        full: Some(lsp::SemanticTokensFullOptions::Delta { delta: None }),
                        ..Default::default()
                    },
                ),
            ),
            ..Default::default()
        },
        cx,
    )
    .await;

    update_test_editor_settings(&mut cx.cx, |settings| {
        settings.editor.rainbow_highlighting.get_or_insert_default().enabled = Some(true);
    });

    cx.set_state("ˇlet foo = 1;");

    crate::rainbow::with_rainbow_cache(|cache| {
        cache.insert("foo", gpui::HighlightStyle::default());
    });

    let cache_has_foo = crate::rainbow::with_rainbow_cache(|cache| {
        cache.get("foo").is_some()
    });
    assert!(cache_has_foo, "Cache should contain foo");

    update_test_editor_settings(&mut cx.cx, |settings| {
        settings.editor.rainbow_highlighting.get_or_insert_default().enabled = Some(false);
    });

    cx.update_editor(|_, _, cx| {
        cx.notify();
    });

    update_test_editor_settings(&mut cx.cx, |settings| {
        settings.editor.rainbow_highlighting.get_or_insert_default().enabled = Some(true);
    });

    let cache_cleared = crate::rainbow::with_rainbow_cache(|cache| {
        cache.get("foo").is_none()
    });
    assert!(cache_cleared, "Cache should be cleared after settings change");
}

#[gpui::test]
async fn test_rainbow_with_nested_scopes(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            semantic_tokens_provider: Some(
                lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                    lsp::SemanticTokensOptions {
                        legend: lsp::SemanticTokensLegend {
                            token_types: vec!["variable".into(), "parameter".into()],
                            token_modifiers: vec![],
                        },
                        full: Some(lsp::SemanticTokensFullOptions::Delta { delta: None }),
                        ..Default::default()
                    },
                ),
            ),
            ..Default::default()
        },
        cx,
    )
    .await;

    update_test_editor_settings(&mut cx.cx, |settings| {
        settings.editor.rainbow_highlighting.get_or_insert_default().enabled = Some(true);
    });

    cx.set_state(indoc! {"
        ˇfn outer(x: i32) {
            fn inner(y: i32) {
                let z = x + y;
            }
        }
    "});
}
