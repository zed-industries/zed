#[cfg(test)]
pub mod tests {
    use crate::editor_tests::update_test_language_settings;
    use crate::inlays::inlay_hints::InlayHintRefreshReason;
    use crate::scroll::Autoscroll;
    use crate::scroll::ScrollAmount;
    use crate::{Editor, SelectionEffects};
    use collections::HashSet;
    use futures::{StreamExt, future};
    use gpui::{AppContext as _, Context, TestAppContext, WindowHandle};
    use itertools::Itertools as _;
    use language::language_settings::InlayHintKind;
    use language::{Capability, FakeLspAdapter};
    use language::{Language, LanguageConfig, LanguageMatcher};
    use languages::rust_lang;
    use lsp::{DEFAULT_LSP_REQUEST_TIMEOUT, FakeLanguageServer};
    use multi_buffer::{MultiBuffer, MultiBufferOffset, PathKey};
    use parking_lot::Mutex;
    use pretty_assertions::assert_eq;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::{AllLanguageSettingsContent, InlayHintSettingsContent, SettingsStore};
    use std::ops::Range;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
    use std::time::Duration;
    use text::{OffsetRangeExt, Point};
    use ui::App;
    use util::path;
    use util::paths::natural_sort;

    #[gpui::test]
    async fn test_basic_cache_update_with_duplicate_hints(cx: &mut gpui::TestAppContext) {
        let allowed_hint_kinds = HashSet::from_iter([None, Some(InlayHintKind::Type)]);
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(allowed_hint_kinds.contains(&Some(InlayHintKind::Type))),
                show_parameter_hints: Some(
                    allowed_hint_kinds.contains(&Some(InlayHintKind::Parameter)),
                ),
                show_other_hints: Some(allowed_hint_kinds.contains(&None)),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });
        let (_, editor, fake_server) = prepare_test_objects(cx, |fake_server, file_with_hints| {
            let lsp_request_count = Arc::new(AtomicU32::new(0));
            fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                move |params, _| {
                    let task_lsp_request_count = Arc::clone(&lsp_request_count);
                    async move {
                        let i = task_lsp_request_count.fetch_add(1, Ordering::Release) + 1;
                        assert_eq!(
                            params.text_document.uri,
                            lsp::Uri::from_file_path(file_with_hints).unwrap(),
                        );
                        Ok(Some(vec![lsp::InlayHint {
                            position: lsp::Position::new(0, i),
                            label: lsp::InlayHintLabel::String(i.to_string()),
                            kind: None,
                            text_edits: None,
                            tooltip: None,
                            padding_left: None,
                            padding_right: None,
                            data: None,
                        }]))
                    }
                },
            );
        })
        .await;
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([MultiBufferOffset(13)..MultiBufferOffset(13)])
                });
                editor.handle_input("some change", window, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["2".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get new hints after an edit"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds"
                );
            })
            .unwrap();

        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>((), DEFAULT_LSP_REQUEST_TIMEOUT)
            .await
            .into_response()
            .expect("inlay refresh request failed");
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["3".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get new hints after hint refresh/ request"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_racy_cache_updates(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                enabled: Some(true),
                ..InlayHintSettingsContent::default()
            })
        });
        let (_, editor, fake_server) = prepare_test_objects(cx, |fake_server, file_with_hints| {
            let lsp_request_count = Arc::new(AtomicU32::new(0));
            fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                move |params, _| {
                    let task_lsp_request_count = Arc::clone(&lsp_request_count);
                    async move {
                        let i = task_lsp_request_count.fetch_add(1, Ordering::Release) + 1;
                        assert_eq!(
                            params.text_document.uri,
                            lsp::Uri::from_file_path(file_with_hints).unwrap(),
                        );
                        Ok(Some(vec![lsp::InlayHint {
                            position: lsp::Position::new(0, i),
                            label: lsp::InlayHintLabel::String(i.to_string()),
                            kind: Some(lsp::InlayHintKind::TYPE),
                            text_edits: None,
                            tooltip: None,
                            padding_left: None,
                            padding_right: None,
                            data: None,
                        }]))
                    }
                },
            );
        })
        .await;
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        // Emulate simultaneous events: both editing, refresh and, slightly after, scroll updates are triggered.
        editor
            .update(cx, |editor, window, cx| {
                editor.handle_input("foo", window, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(5));
        editor
            .update(cx, |editor, _window, cx| {
                editor.refresh_inlay_hints(
                    InlayHintRefreshReason::RefreshRequested {
                        server_id: fake_server.server.server_id(),
                        request_id: Some(1),
                    },
                    cx,
                );
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(5));
        editor
            .update(cx, |editor, _window, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["2".to_string()];
                assert_eq!(expected_hints, cached_hint_labels(editor, cx), "Despite multiple simultaneous refreshes, only one inlay hint query should be issued");
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_cache_update_on_lsp_completion_tasks(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let (_, editor, fake_server) = prepare_test_objects(cx, |fake_server, file_with_hints| {
            let lsp_request_count = Arc::new(AtomicU32::new(0));
            fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                move |params, _| {
                    let task_lsp_request_count = Arc::clone(&lsp_request_count);
                    async move {
                        assert_eq!(
                            params.text_document.uri,
                            lsp::Uri::from_file_path(file_with_hints).unwrap(),
                        );
                        let current_call_id =
                            Arc::clone(&task_lsp_request_count).fetch_add(1, Ordering::SeqCst);
                        Ok(Some(vec![lsp::InlayHint {
                            position: lsp::Position::new(0, current_call_id),
                            label: lsp::InlayHintLabel::String(current_call_id.to_string()),
                            kind: None,
                            text_edits: None,
                            tooltip: None,
                            padding_left: None,
                            padding_right: None,
                            data: None,
                        }]))
                    }
                },
            );
        })
        .await;
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _, cx| {
                let expected_hints = vec!["0".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        let progress_token = 42;
        fake_server
            .request::<lsp::request::WorkDoneProgressCreate>(
                lsp::WorkDoneProgressCreateParams {
                    token: lsp::ProgressToken::Number(progress_token),
                },
                DEFAULT_LSP_REQUEST_TIMEOUT,
            )
            .await
            .into_response()
            .expect("work done progress create request failed");
        cx.executor().run_until_parked();
        fake_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
            token: lsp::ProgressToken::Number(progress_token),
            value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Begin(
                lsp::WorkDoneProgressBegin::default(),
            )),
        });
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _, cx| {
                let expected_hints = vec!["0".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should not update hints while the work task is running"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        fake_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
            token: lsp::ProgressToken::Number(progress_token),
            value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::End(
                lsp::WorkDoneProgressEnd::default(),
            )),
        });
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "New hints should be queried after the work task is done"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_no_hint_updates_for_unrelated_language_files(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { a } // and some long comment to ensure inlays are not trimmed out",
                "other.md": "Test md file with some text",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let mut rs_fake_servers = None;
        let mut md_fake_servers = None;
        for (name, path_suffix) in [("Rust", "rs"), ("Markdown", "md")] {
            language_registry.add(Arc::new(Language::new(
                LanguageConfig {
                    name: name.into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec![path_suffix.to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )));
            let fake_servers = language_registry.register_fake_lsp(
                name,
                FakeLspAdapter {
                    name,
                    capabilities: lsp::ServerCapabilities {
                        inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                        ..Default::default()
                    },
                    initializer: Some(Box::new({
                        move |fake_server| {
                            let rs_lsp_request_count = Arc::new(AtomicU32::new(0));
                            let md_lsp_request_count = Arc::new(AtomicU32::new(0));
                            fake_server
                                .set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                                    move |params, _| {
                                        let i = match name {
                                            "Rust" => {
                                                assert_eq!(
                                                    params.text_document.uri,
                                                    lsp::Uri::from_file_path(path!("/a/main.rs"))
                                                        .unwrap(),
                                                );
                                                rs_lsp_request_count.fetch_add(1, Ordering::Release)
                                                    + 1
                                            }
                                            "Markdown" => {
                                                assert_eq!(
                                                    params.text_document.uri,
                                                    lsp::Uri::from_file_path(path!("/a/other.md"))
                                                        .unwrap(),
                                                );
                                                md_lsp_request_count.fetch_add(1, Ordering::Release)
                                                    + 1
                                            }
                                            unexpected => {
                                                panic!("Unexpected language: {unexpected}")
                                            }
                                        };

                                        async move {
                                            let query_start = params.range.start;
                                            Ok(Some(vec![lsp::InlayHint {
                                                position: query_start,
                                                label: lsp::InlayHintLabel::String(i.to_string()),
                                                kind: None,
                                                text_edits: None,
                                                tooltip: None,
                                                padding_left: None,
                                                padding_right: None,
                                                data: None,
                                            }]))
                                        }
                                    },
                                );
                        }
                    })),
                    ..Default::default()
                },
            );
            match name {
                "Rust" => rs_fake_servers = Some(fake_servers),
                "Markdown" => md_fake_servers = Some(fake_servers),
                _ => unreachable!(),
            }
        }

        let rs_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let rs_editor = cx.add_window(|window, cx| {
            Editor::for_buffer(rs_buffer, Some(project.clone()), window, cx)
        });
        cx.executor().run_until_parked();

        let _rs_fake_server = rs_fake_servers.unwrap().next().await.unwrap();
        cx.executor().run_until_parked();

        // Establish a viewport so the editor considers itself visible and the hint refresh
        // pipeline runs. Then explicitly trigger a refresh.
        rs_editor
            .update(cx, |editor, window, cx| {
                editor.set_visible_line_count(50.0, window, cx);
                editor.set_visible_column_count(120.0);
                editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        rs_editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        cx.executor().run_until_parked();
        let md_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/other.md"), cx)
            })
            .await
            .unwrap();
        let md_editor =
            cx.add_window(|window, cx| Editor::for_buffer(md_buffer, Some(project), window, cx));
        cx.executor().run_until_parked();

        let _md_fake_server = md_fake_servers.unwrap().next().await.unwrap();
        cx.executor().run_until_parked();

        // Establish a viewport so the editor considers itself visible and the hint refresh
        // pipeline runs. Then explicitly trigger a refresh.
        md_editor
            .update(cx, |editor, window, cx| {
                editor.set_visible_line_count(50.0, window, cx);
                editor.set_visible_column_count(120.0);
                editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        md_editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Markdown editor should have a separate version, repeating Rust editor rules"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        rs_editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([MultiBufferOffset(13)..MultiBufferOffset(13)])
                });
                editor.handle_input("some rs change", window, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        rs_editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["2".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Rust inlay cache should change after the edit"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
        md_editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Markdown editor should not be affected by Rust editor changes"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        md_editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([MultiBufferOffset(13)..MultiBufferOffset(13)])
                });
                editor.handle_input("some md change", window, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        md_editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["2".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Rust editor should not be affected by Markdown editor changes"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
        rs_editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["2".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Markdown editor should also change independently"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_hint_setting_changes(cx: &mut gpui::TestAppContext) {
        let allowed_hint_kinds = HashSet::from_iter([None, Some(InlayHintKind::Type)]);
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(allowed_hint_kinds.contains(&Some(InlayHintKind::Type))),
                show_parameter_hints: Some(
                    allowed_hint_kinds.contains(&Some(InlayHintKind::Parameter)),
                ),
                show_other_hints: Some(allowed_hint_kinds.contains(&None)),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let lsp_request_count = Arc::new(AtomicUsize::new(0));
        let (_, editor, fake_server) = prepare_test_objects(cx, {
            let lsp_request_count = lsp_request_count.clone();
            move |fake_server, file_with_hints| {
                let lsp_request_count = lsp_request_count.clone();
                fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                    move |params, _| {
                        lsp_request_count.fetch_add(1, Ordering::Release);
                        async move {
                            assert_eq!(
                                params.text_document.uri,
                                lsp::Uri::from_file_path(file_with_hints).unwrap(),
                            );
                            Ok(Some(vec![
                                lsp::InlayHint {
                                    position: lsp::Position::new(0, 1),
                                    label: lsp::InlayHintLabel::String("type hint".to_string()),
                                    kind: Some(lsp::InlayHintKind::TYPE),
                                    text_edits: None,
                                    tooltip: None,
                                    padding_left: None,
                                    padding_right: None,
                                    data: None,
                                },
                                lsp::InlayHint {
                                    position: lsp::Position::new(0, 2),
                                    label: lsp::InlayHintLabel::String(
                                        "parameter hint".to_string(),
                                    ),
                                    kind: Some(lsp::InlayHintKind::PARAMETER),
                                    text_edits: None,
                                    tooltip: None,
                                    padding_left: None,
                                    padding_right: None,
                                    data: None,
                                },
                                lsp::InlayHint {
                                    position: lsp::Position::new(0, 3),
                                    label: lsp::InlayHintLabel::String("other hint".to_string()),
                                    kind: None,
                                    text_edits: None,
                                    tooltip: None,
                                    padding_left: None,
                                    padding_right: None,
                                    data: None,
                                },
                            ]))
                        }
                    },
                );
            }
        })
        .await;
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    1,
                    "Should query new hints once"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(
                    vec!["type hint".to_string(), "other hint".to_string()],
                    visible_hint_labels(editor, cx)
                );
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds"
                );
            })
            .unwrap();

        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>((), DEFAULT_LSP_REQUEST_TIMEOUT)
            .await
            .into_response()
            .expect("inlay refresh request failed");
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should load new hints twice"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "Cached hints should not change due to allowed hint kinds settings update"
                );
                assert_eq!(
                    vec!["type hint".to_string(), "other hint".to_string()],
                    visible_hint_labels(editor, cx)
                );
            })
            .unwrap();

        for (new_allowed_hint_kinds, expected_visible_hints) in [
            (HashSet::from_iter([None]), vec!["other hint".to_string()]),
            (
                HashSet::from_iter([Some(InlayHintKind::Type)]),
                vec!["type hint".to_string()],
            ),
            (
                HashSet::from_iter([Some(InlayHintKind::Parameter)]),
                vec!["parameter hint".to_string()],
            ),
            (
                HashSet::from_iter([None, Some(InlayHintKind::Type)]),
                vec!["type hint".to_string(), "other hint".to_string()],
            ),
            (
                HashSet::from_iter([None, Some(InlayHintKind::Parameter)]),
                vec!["parameter hint".to_string(), "other hint".to_string()],
            ),
            (
                HashSet::from_iter([Some(InlayHintKind::Type), Some(InlayHintKind::Parameter)]),
                vec!["type hint".to_string(), "parameter hint".to_string()],
            ),
            (
                HashSet::from_iter([
                    None,
                    Some(InlayHintKind::Type),
                    Some(InlayHintKind::Parameter),
                ]),
                vec![
                    "type hint".to_string(),
                    "parameter hint".to_string(),
                    "other hint".to_string(),
                ],
            ),
        ] {
            update_test_language_settings(cx, &|settings| {
                settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                    show_value_hints: Some(true),
                    enabled: Some(true),
                    edit_debounce_ms: Some(0),
                    scroll_debounce_ms: Some(0),
                    show_type_hints: Some(
                        new_allowed_hint_kinds.contains(&Some(InlayHintKind::Type)),
                    ),
                    show_parameter_hints: Some(
                        new_allowed_hint_kinds.contains(&Some(InlayHintKind::Parameter)),
                    ),
                    show_other_hints: Some(new_allowed_hint_kinds.contains(&None)),
                    show_background: Some(false),
                    toggle_on_modifiers_press: None,
                })
            });
            cx.executor().run_until_parked();
            editor.update(cx, |editor, _, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should not load new hints on allowed hint kinds change for hint kinds {new_allowed_hint_kinds:?}"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "Should get its cached hints unchanged after the settings change for hint kinds {new_allowed_hint_kinds:?}"
                );
                assert_eq!(
                    expected_visible_hints,
                    visible_hint_labels(editor, cx),
                    "Should get its visible hints filtered after the settings change for hint kinds {new_allowed_hint_kinds:?}"
                );
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    new_allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds for hint kinds {new_allowed_hint_kinds:?}"
                );
            }).unwrap();
        }

        let another_allowed_hint_kinds = HashSet::from_iter([Some(InlayHintKind::Type)]);
        update_test_language_settings(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(false),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(
                    another_allowed_hint_kinds.contains(&Some(InlayHintKind::Type)),
                ),
                show_parameter_hints: Some(
                    another_allowed_hint_kinds.contains(&Some(InlayHintKind::Parameter)),
                ),
                show_other_hints: Some(another_allowed_hint_kinds.contains(&None)),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should not load new hints when hints got disabled"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "Should not clear the cache when hints got disabled"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "Should clear visible hints when hints got disabled"
                );
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    another_allowed_hint_kinds,
                    "Should update its allowed hint kinds even when hints got disabled"
                );
            })
            .unwrap();

        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>((), DEFAULT_LSP_REQUEST_TIMEOUT)
            .await
            .into_response()
            .expect("inlay refresh request failed");
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should not load new hints when they got disabled"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx)
                );
                assert_eq!(Vec::<String>::new(), visible_hint_labels(editor, cx));
            })
            .unwrap();

        let final_allowed_hint_kinds = HashSet::from_iter([Some(InlayHintKind::Parameter)]);
        update_test_language_settings(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(
                    final_allowed_hint_kinds.contains(&Some(InlayHintKind::Type)),
                ),
                show_parameter_hints: Some(
                    final_allowed_hint_kinds.contains(&Some(InlayHintKind::Parameter)),
                ),
                show_other_hints: Some(final_allowed_hint_kinds.contains(&None)),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should not query for new hints when they got re-enabled, as the file version did not change"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "Should get its cached hints fully repopulated after the hints got re-enabled"
                );
                assert_eq!(
                    vec!["parameter hint".to_string()],
                    visible_hint_labels(editor, cx),
                    "Should get its visible hints repopulated and filtered after the h"
                );
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    final_allowed_hint_kinds,
                    "Cache should update editor settings when hints got re-enabled"
                );
            })
            .unwrap();

        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>((), DEFAULT_LSP_REQUEST_TIMEOUT)
            .await
            .into_response()
            .expect("inlay refresh request failed");
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    3,
                    "Should query for new hints again"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                );
                assert_eq!(
                    vec!["parameter hint".to_string()],
                    visible_hint_labels(editor, cx),
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_hint_request_cancellation(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let lsp_request_count = Arc::new(AtomicU32::new(0));
        let (_, editor, _) = prepare_test_objects(cx, {
            let lsp_request_count = lsp_request_count.clone();
            move |fake_server, file_with_hints| {
                let lsp_request_count = lsp_request_count.clone();
                fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                    move |params, _| {
                        let lsp_request_count = lsp_request_count.clone();
                        async move {
                            let i = lsp_request_count.fetch_add(1, Ordering::SeqCst) + 1;
                            assert_eq!(
                                params.text_document.uri,
                                lsp::Uri::from_file_path(file_with_hints).unwrap(),
                            );
                            Ok(Some(vec![lsp::InlayHint {
                                position: lsp::Position::new(0, i),
                                label: lsp::InlayHintLabel::String(i.to_string()),
                                kind: None,
                                text_edits: None,
                                tooltip: None,
                                padding_left: None,
                                padding_right: None,
                                data: None,
                            }]))
                        }
                    },
                );
            }
        })
        .await;

        let mut expected_changes = Vec::new();
        for change_after_opening in [
            "initial change #1",
            "initial change #2",
            "initial change #3",
        ] {
            editor
                .update(cx, |editor, window, cx| {
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        s.select_ranges([MultiBufferOffset(13)..MultiBufferOffset(13)])
                    });
                    editor.handle_input(change_after_opening, window, cx);
                })
                .unwrap();
            expected_changes.push(change_after_opening);
        }

        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _window, cx| {
                let current_text = editor.text(cx);
                for change in &expected_changes {
                    assert!(
                        current_text.contains(change),
                        "Should apply all changes made"
                    );
                }
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should query new hints twice: for editor init and for the last edit that interrupted all others"
                );
                let expected_hints = vec!["2".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get hints from the last edit landed only"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        let mut edits = Vec::new();
        for async_later_change in [
            "another change #1",
            "another change #2",
            "another change #3",
        ] {
            expected_changes.push(async_later_change);
            let task_editor = editor;
            edits.push(cx.spawn(|mut cx| async move {
                task_editor
                    .update(&mut cx, |editor, window, cx| {
                        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                            s.select_ranges([MultiBufferOffset(13)..MultiBufferOffset(13)])
                        });
                        editor.handle_input(async_later_change, window, cx);
                    })
                    .unwrap();
            }));
        }
        let _ = future::join_all(edits).await;
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _, cx| {
                let current_text = editor.text(cx);
                for change in &expected_changes {
                    assert!(
                        current_text.contains(change),
                        "Should apply all changes made"
                    );
                }
                assert_eq!(
                    lsp_request_count.load(Ordering::SeqCst),
                    3,
                    "Should query new hints one more time, for the last edit only"
                );
                let expected_hints = vec!["3".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get hints from the last edit landed only"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
    }

    #[gpui::test(iterations = 4)]
    async fn test_large_buffer_inlay_requests_split(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                enabled: Some(true),
                ..InlayHintSettingsContent::default()
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": format!("fn main() {{\n{}\n}}", "let i = 5;\n".repeat(500)),
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());

        let lsp_request_ranges = Arc::new(Mutex::new(Vec::new()));
        let lsp_request_count = Arc::new(AtomicUsize::new(0));
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new({
                    let lsp_request_ranges = lsp_request_ranges.clone();
                    let lsp_request_count = lsp_request_count.clone();
                    move |fake_server| {
                        let closure_lsp_request_ranges = Arc::clone(&lsp_request_ranges);
                        let closure_lsp_request_count = Arc::clone(&lsp_request_count);
                        fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                            move |params, _| {
                                let task_lsp_request_ranges =
                                    Arc::clone(&closure_lsp_request_ranges);
                                let task_lsp_request_count = Arc::clone(&closure_lsp_request_count);
                                async move {
                                    assert_eq!(
                                        params.text_document.uri,
                                        lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                                    );

                                    task_lsp_request_ranges.lock().push(params.range);
                                    task_lsp_request_count.fetch_add(1, Ordering::Release);
                                    Ok(Some(vec![lsp::InlayHint {
                                        position: params.range.start,
                                        label: lsp::InlayHintLabel::String(
                                            params.range.end.line.to_string(),
                                        ),
                                        kind: None,
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    }]))
                                }
                            },
                        );
                    }
                })),
                ..FakeLspAdapter::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let editor =
            cx.add_window(|window, cx| Editor::for_buffer(buffer, Some(project), window, cx));
        cx.executor().run_until_parked();
        let _fake_server = fake_servers.next().await.unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        let ranges = lsp_request_ranges
            .lock()
            .drain(..)
            .sorted_by_key(|r| r.start)
            .collect::<Vec<_>>();
        assert_eq!(
            ranges.len(),
            1,
            "Should query 1 range initially, but got: {ranges:?}"
        );

        editor
            .update(cx, |editor, window, cx| {
                editor.scroll_screen(&ScrollAmount::Page(1.0), window, cx);
            })
            .unwrap();
        // Wait for the first hints request to fire off
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, window, cx| {
                editor.scroll_screen(&ScrollAmount::Page(1.0), window, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        let visible_range_after_scrolls = editor_visible_range(&editor, cx);
        let visible_line_count = editor
            .update(cx, |editor, _window, _| {
                editor.visible_line_count().unwrap()
            })
            .unwrap();
        let selection_in_cached_range = editor
            .update(cx, |editor, _window, cx| {
                let ranges = lsp_request_ranges
                    .lock()
                    .drain(..)
                    .sorted_by_key(|r| r.start)
                    .collect::<Vec<_>>();
                assert_eq!(
                    ranges.len(),
                    2,
                    "Should query 2 ranges after both scrolls, but got: {ranges:?}"
                );
                let first_scroll = &ranges[0];
                let second_scroll = &ranges[1];
                assert_eq!(
                    first_scroll.end.line, second_scroll.start.line,
                    "Should query 2 adjacent ranges after the scrolls, but got: {ranges:?}"
                );

                let lsp_requests = lsp_request_count.load(Ordering::Acquire);
                assert_eq!(
                    lsp_requests, 3,
                    "Should query hints initially, and after each scroll (2 times)"
                );
                assert_eq!(
                    vec!["50".to_string(), "100".to_string(), "150".to_string()],
                    cached_hint_labels(editor, cx),
                    "Chunks of 50 line width should have been queried each time"
                );
                assert_eq!(
                    vec!["50".to_string(), "100".to_string(), "150".to_string()],
                    visible_hint_labels(editor, cx),
                    "Editor should show only hints that it's scrolled to"
                );

                let mut selection_in_cached_range = visible_range_after_scrolls.end;
                selection_in_cached_range.row -= visible_line_count.ceil() as u32;
                selection_in_cached_range
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::center()),
                    window,
                    cx,
                    |s| s.select_ranges([selection_in_cached_range..selection_in_cached_range]),
                );
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        editor.update(cx, |_, _, _| {
            let ranges = lsp_request_ranges
                .lock()
                .drain(..)
                .sorted_by_key(|r| r.start)
                .collect::<Vec<_>>();
            assert!(ranges.is_empty(), "No new ranges or LSP queries should be made after returning to the selection with cached hints");
            assert_eq!(lsp_request_count.load(Ordering::Acquire), 3, "No new requests should be made when selecting within cached chunks");
        }).unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.handle_input("++++more text++++", window, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();
        editor.update(cx, |editor, _window, cx| {
            let mut ranges = lsp_request_ranges.lock().drain(..).collect::<Vec<_>>();
            ranges.sort_by_key(|r| r.start);

            assert_eq!(ranges.len(), 2,
                "On edit, should scroll to selection and query a range around it: that range should split into 2 50 rows wide chunks. Instead, got query ranges {ranges:?}");
            let first_chunk = &ranges[0];
            let second_chunk = &ranges[1];
            assert!(first_chunk.end.line == second_chunk.start.line,
                "First chunk {first_chunk:?} should be before second chunk {second_chunk:?}");
            assert!(first_chunk.start.line < selection_in_cached_range.row,
                "Hints should be queried with the selected range after the query range start");

            let lsp_requests = lsp_request_count.load(Ordering::Acquire);
            assert_eq!(lsp_requests, 5, "Two chunks should be re-queried");
            assert_eq!(vec!["100".to_string(), "150".to_string()], cached_hint_labels(editor, cx),
                "Should have (less) hints from the new LSP response after the edit");
            assert_eq!(vec!["100".to_string(), "150".to_string()], visible_hint_labels(editor, cx), "Should show only visible hints (in the center) from the new cached set");
        }).unwrap();
    }

    fn editor_visible_range(
        editor: &WindowHandle<Editor>,
        cx: &mut gpui::TestAppContext,
    ) -> Range<Point> {
        let ranges = editor
            .update(cx, |editor, _window, cx| editor.visible_buffer_ranges(cx))
            .unwrap();
        assert_eq!(
            ranges.len(),
            1,
            "Single buffer should produce a single excerpt with visible range"
        );
        let (buffer_snapshot, visible_range, _) = ranges.into_iter().next().unwrap();
        visible_range.to_point(&buffer_snapshot)
    }

    #[gpui::test]
    async fn test_multiple_excerpts_large_multibuffer(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
                path!("/a"),
                json!({
                    "main.rs": format!("fn main() {{\n{}\n}}", (0..501).map(|i| format!("let i = {i};\n")).collect::<String>()),
                    "other.rs": format!("fn main() {{\n{}\n}}", (0..501).map(|j| format!("let j = {j};\n")).collect::<String>()),
                }),
            )
            .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let language = rust_lang();
        language_registry.add(language);
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        );

        let (buffer_1, _handle1) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let (buffer_2, _handle2) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/other.rs"), cx)
            })
            .await
            .unwrap();
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_excerpts_for_path(
                PathKey::sorted(0),
                buffer_1.clone(),
                [
                    Point::new(0, 0)..Point::new(2, 0),
                    Point::new(4, 0)..Point::new(11, 0),
                    Point::new(22, 0)..Point::new(33, 0),
                    Point::new(44, 0)..Point::new(55, 0),
                    Point::new(56, 0)..Point::new(66, 0),
                    Point::new(67, 0)..Point::new(77, 0),
                ],
                0,
                cx,
            );
            multibuffer.set_excerpts_for_path(
                PathKey::sorted(1),
                buffer_2.clone(),
                [
                    Point::new(0, 1)..Point::new(2, 1),
                    Point::new(4, 1)..Point::new(11, 1),
                    Point::new(22, 1)..Point::new(33, 1),
                    Point::new(44, 1)..Point::new(55, 1),
                    Point::new(56, 1)..Point::new(66, 1),
                    Point::new(67, 1)..Point::new(77, 1),
                ],
                0,
                cx,
            );
            multibuffer
        });

        cx.executor().run_until_parked();
        let editor = cx.add_window(|window, cx| {
            Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx)
        });

        let editor_edited = Arc::new(AtomicBool::new(false));
        let fake_server = fake_servers.next().await.unwrap();
        let closure_editor_edited = Arc::clone(&editor_edited);
        fake_server
            .set_request_handler::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_editor_edited = Arc::clone(&closure_editor_edited);
                async move {
                    let hint_text = if params.text_document.uri
                        == lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap()
                    {
                        "main hint"
                    } else if params.text_document.uri
                        == lsp::Uri::from_file_path(path!("/a/other.rs")).unwrap()
                    {
                        "other hint"
                    } else {
                        panic!("unexpected uri: {:?}", params.text_document.uri);
                    };

                    // one hint per excerpt
                    let positions = [
                        lsp::Position::new(0, 2),
                        lsp::Position::new(4, 2),
                        lsp::Position::new(22, 2),
                        lsp::Position::new(44, 2),
                        lsp::Position::new(56, 2),
                        lsp::Position::new(67, 2),
                    ];
                    let out_of_range_hint = lsp::InlayHint {
                        position: lsp::Position::new(
                            params.range.start.line + 99,
                            params.range.start.character + 99,
                        ),
                        label: lsp::InlayHintLabel::String(
                            "out of excerpt range, should be ignored".to_string(),
                        ),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    };

                    let edited = task_editor_edited.load(Ordering::Acquire);
                    Ok(Some(
                        std::iter::once(out_of_range_hint)
                            .chain(positions.into_iter().enumerate().map(|(i, position)| {
                                lsp::InlayHint {
                                    position,
                                    label: lsp::InlayHintLabel::String(format!(
                                        "{hint_text}{E} #{i}",
                                        E = if edited { "(edited)" } else { "" },
                                    )),
                                    kind: None,
                                    text_edits: None,
                                    tooltip: None,
                                    padding_left: None,
                                    padding_right: None,
                                    data: None,
                                }
                            }))
                            .collect(),
                    ))
                }
            })
            .next()
            .await;
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec![
                    "main hint #0".to_string(),
                    "main hint #1".to_string(),
                    "main hint #2".to_string(),
                    "main hint #3".to_string(),
                    "main hint #4".to_string(),
                    "main hint #5".to_string(),
                ];
                assert_eq!(
                    expected_hints,
                    sorted_cached_hint_labels(editor, cx),
                    "When scroll is at the edge of a multibuffer, its visible excerpts only should be queried for inlay hints"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::Next),
                    window,
                    cx,
                    |s| s.select_ranges([Point::new(4, 0)..Point::new(4, 0)]),
                );
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::Next),
                    window,
                    cx,
                    |s| s.select_ranges([Point::new(22, 0)..Point::new(22, 0)]),
                );
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::Next),
                    window,
                    cx,
                    |s| s.select_ranges([Point::new(57, 0)..Point::new(57, 0)]),
                );
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec![
                    "main hint #0".to_string(),
                    "main hint #1".to_string(),
                    "main hint #2".to_string(),
                    "main hint #3".to_string(),
                    "main hint #4".to_string(),
                    "main hint #5".to_string(),
                ];
                assert_eq!(expected_hints, sorted_cached_hint_labels(editor, cx),
                    "New hints are not shown right after scrolling, we need to wait for the buffer to be registered");
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec![
                    "main hint #0".to_string(),
                    "main hint #1".to_string(),
                    "main hint #2".to_string(),
                    "main hint #3".to_string(),
                    "main hint #4".to_string(),
                    "main hint #5".to_string(),
                    "other hint #0".to_string(),
                    "other hint #1".to_string(),
                    "other hint #2".to_string(),
                    "other hint #3".to_string(),
                ];
                assert_eq!(
                    expected_hints,
                    sorted_cached_hint_labels(editor, cx),
                    "After scrolling to the new buffer and waiting for it to be registered, new hints should appear");
                assert_eq!(
                    expected_hints,
                    visible_hint_labels(editor, cx),
                    "Editor should show only visible hints",
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::Next),
                    window,
                    cx,
                    |s| s.select_ranges([Point::new(100, 0)..Point::new(100, 0)]),
                );
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec![
                    "main hint #0".to_string(),
                    "main hint #1".to_string(),
                    "main hint #2".to_string(),
                    "main hint #3".to_string(),
                    "main hint #4".to_string(),
                    "main hint #5".to_string(),
                    "other hint #0".to_string(),
                    "other hint #1".to_string(),
                    "other hint #2".to_string(),
                    "other hint #3".to_string(),
                    "other hint #4".to_string(),
                    "other hint #5".to_string(),
                ];
                assert_eq!(
                    expected_hints,
                    sorted_cached_hint_labels(editor, cx),
                    "After multibuffer was scrolled to the end, all hints for all excerpts should be fetched"
                );
                assert_eq!(
                    expected_hints,
                    visible_hint_labels(editor, cx),
                    "Editor shows only hints for excerpts that were visible when scrolling"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::Next),
                    window,
                    cx,
                    |s| s.select_ranges([Point::new(4, 0)..Point::new(4, 0)]),
                );
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec![
                    "main hint #0".to_string(),
                    "main hint #1".to_string(),
                    "main hint #2".to_string(),
                    "main hint #3".to_string(),
                    "main hint #4".to_string(),
                    "main hint #5".to_string(),
                    "other hint #0".to_string(),
                    "other hint #1".to_string(),
                    "other hint #2".to_string(),
                    "other hint #3".to_string(),
                    "other hint #4".to_string(),
                    "other hint #5".to_string(),
                ];
                assert_eq!(
                    expected_hints,
                    sorted_cached_hint_labels(editor, cx),
                    "After multibuffer was scrolled to the end, further scrolls up should not bring more hints"
                );
                assert_eq!(
                    expected_hints,
                    visible_hint_labels(editor, cx),
                );
            })
            .unwrap();

        // We prepare to change the scrolling on edit, but do not scroll yet
        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([Point::new(57, 0)..Point::new(57, 0)])
                });
            })
            .unwrap();
        cx.executor().run_until_parked();
        // Edit triggers the scrolling too
        editor_edited.store(true, Ordering::Release);
        editor
            .update(cx, |editor, window, cx| {
                editor.handle_input("++++more text++++", window, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        // Wait again to trigger the inlay hints fetch on scroll
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec![
                    "main hint(edited) #0".to_string(),
                    "main hint(edited) #1".to_string(),
                    "main hint(edited) #2".to_string(),
                    "main hint(edited) #3".to_string(),
                    "main hint(edited) #4".to_string(),
                    "main hint(edited) #5".to_string(),
                    "other hint(edited) #0".to_string(),
                    "other hint(edited) #1".to_string(),
                    "other hint(edited) #2".to_string(),
                    "other hint(edited) #3".to_string(),
                ];
                assert_eq!(
                    expected_hints,
                    sorted_cached_hint_labels(editor, cx),
                    "After multibuffer edit, editor gets scrolled back to the last selection; \
                all hints should be invalidated and required for all of its visible excerpts"
                );
                assert_eq!(
                    expected_hints,
                    visible_hint_labels(editor, cx),
                    "All excerpts should get their hints"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_editing_in_multi_buffer(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                enabled: Some(true),
                ..InlayHintSettingsContent::default()
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": format!("fn main() {{\n{}\n}}", (0..200).map(|i| format!("let i = {i};\n")).collect::<String>()),
                "lib.rs": r#"let a = 1;
let b = 2;
let c = 3;"#
            }),
        )
        .await;

        let lsp_request_ranges = Arc::new(Mutex::new(Vec::new()));

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let language = rust_lang();
        language_registry.add(language);

        let closure_ranges_fetched = lsp_request_ranges.clone();
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new(move |fake_server| {
                    let closure_ranges_fetched = closure_ranges_fetched.clone();
                    fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                        move |params, _| {
                            let closure_ranges_fetched = closure_ranges_fetched.clone();
                            async move {
                                let prefix = if params.text_document.uri
                                    == lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap()
                                {
                                    closure_ranges_fetched
                                        .lock()
                                        .push(("main.rs", params.range));
                                    "main.rs"
                                } else if params.text_document.uri
                                    == lsp::Uri::from_file_path(path!("/a/lib.rs")).unwrap()
                                {
                                    closure_ranges_fetched.lock().push(("lib.rs", params.range));
                                    "lib.rs"
                                } else {
                                    panic!("Unexpected file path {:?}", params.text_document.uri);
                                };
                                Ok(Some(
                                    (params.range.start.line..params.range.end.line)
                                        .map(|row| lsp::InlayHint {
                                            position: lsp::Position::new(row, 0),
                                            label: lsp::InlayHintLabel::String(format!(
                                                "{prefix} Inlay hint #{row}"
                                            )),
                                            kind: Some(lsp::InlayHintKind::TYPE),
                                            text_edits: None,
                                            tooltip: None,
                                            padding_left: None,
                                            padding_right: None,
                                            data: None,
                                        })
                                        .collect(),
                                ))
                            }
                        },
                    );
                })),
                ..FakeLspAdapter::default()
            },
        );

        let (buffer_1, _handle_1) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let (buffer_2, _handle_2) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/lib.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_excerpts_for_path(
                PathKey::sorted(0),
                buffer_1.clone(),
                [
                    Point::new(49, 0)..Point::new(53, 0),
                    Point::new(70, 0)..Point::new(73, 0),
                ],
                0,
                cx,
            );
            multibuffer.set_excerpts_for_path(
                PathKey::sorted(1),
                buffer_2.clone(),
                [Point::new(0, 0)..Point::new(4, 0)],
                0,
                cx,
            );
            multibuffer
        });

        let editor = cx.add_window(|window, cx| {
            let mut editor =
                Editor::for_multibuffer(multi_buffer, Some(project.clone()), window, cx);
            editor.change_selections(SelectionEffects::default(), window, cx, |s| {
                s.select_ranges([MultiBufferOffset(0)..MultiBufferOffset(0)])
            });
            editor
        });

        let _fake_server = fake_servers.next().await.unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        assert_eq!(
            vec![
                (
                    "lib.rs",
                    lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(2, 10))
                ),
                (
                    "main.rs",
                    lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(50, 0))
                ),
                (
                    "main.rs",
                    lsp::Range::new(lsp::Position::new(50, 0), lsp::Position::new(100, 0))
                ),
            ],
            lsp_request_ranges
                .lock()
                .drain(..)
                .sorted_by_key(|(prefix, r)| (prefix.to_owned(), r.start))
                .collect::<Vec<_>>(),
            "For large buffers, should query chunks that cover both visible excerpt"
        );
        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    (0..2)
                        .map(|i| format!("lib.rs Inlay hint #{i}"))
                        .chain((0..100).map(|i| format!("main.rs Inlay hint #{i}")))
                        .collect::<Vec<_>>(),
                    sorted_cached_hint_labels(editor, cx),
                    "Both chunks should provide their inlay hints"
                );
                assert_eq!(
                    vec![
                        "main.rs Inlay hint #49".to_owned(),
                        "main.rs Inlay hint #50".to_owned(),
                        "main.rs Inlay hint #51".to_owned(),
                        "main.rs Inlay hint #52".to_owned(),
                        "main.rs Inlay hint #53".to_owned(),
                        "main.rs Inlay hint #70".to_owned(),
                        "main.rs Inlay hint #71".to_owned(),
                        "main.rs Inlay hint #72".to_owned(),
                        "main.rs Inlay hint #73".to_owned(),
                        "lib.rs Inlay hint #0".to_owned(),
                        "lib.rs Inlay hint #1".to_owned(),
                    ],
                    visible_hint_labels(editor, cx),
                    "Only hints from visible excerpt should be added into the editor"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.handle_input("a", window, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(1000));
        cx.executor().run_until_parked();
        assert_eq!(
            vec![
                (
                    "lib.rs",
                    lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(2, 10))
                ),
                (
                    "main.rs",
                    lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(50, 0))
                ),
                (
                    "main.rs",
                    lsp::Range::new(lsp::Position::new(50, 0), lsp::Position::new(100, 0))
                ),
            ],
            lsp_request_ranges
                .lock()
                .drain(..)
                .sorted_by_key(|(prefix, r)| (prefix.to_owned(), r.start))
                .collect::<Vec<_>>(),
            "Same chunks should be re-queried on edit"
        );
        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    (0..2)
                        .map(|i| format!("lib.rs Inlay hint #{i}"))
                        .chain((0..100).map(|i| format!("main.rs Inlay hint #{i}")))
                        .collect::<Vec<_>>(),
                    sorted_cached_hint_labels(editor, cx),
                    "Same hints should be re-inserted after the edit"
                );
                assert_eq!(
                    vec![
                        "main.rs Inlay hint #49".to_owned(),
                        "main.rs Inlay hint #50".to_owned(),
                        "main.rs Inlay hint #51".to_owned(),
                        "main.rs Inlay hint #52".to_owned(),
                        "main.rs Inlay hint #53".to_owned(),
                        "main.rs Inlay hint #70".to_owned(),
                        "main.rs Inlay hint #71".to_owned(),
                        "main.rs Inlay hint #72".to_owned(),
                        "main.rs Inlay hint #73".to_owned(),
                        "lib.rs Inlay hint #0".to_owned(),
                        "lib.rs Inlay hint #1".to_owned(),
                    ],
                    visible_hint_labels(editor, cx),
                    "Same hints should be re-inserted into the editor after the edit"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_excerpts_removed(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(false),
                show_parameter_hints: Some(false),
                show_other_hints: Some(false),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": format!("fn main() {{\n{}\n}}", (0..501).map(|i| format!("let i = {i};\n")).collect::<String>()),
                "other.rs": format!("fn main() {{\n{}\n}}", (0..501).map(|j| format!("let j = {j};\n")).collect::<String>()),
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        );

        let (buffer_1, _handle) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let (buffer_2, _handle2) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/other.rs"), cx)
            })
            .await
            .unwrap();
        let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.set_excerpts_for_path(
                PathKey::sorted(0),
                buffer_1.clone(),
                [Point::new(0, 0)..Point::new(2, 0)],
                0,
                cx,
            );
            multibuffer.set_excerpts_for_path(
                PathKey::sorted(1),
                buffer_2.clone(),
                [Point::new(0, 1)..Point::new(2, 1)],
                0,
                cx,
            );
        });

        cx.executor().run_until_parked();
        let editor = cx.add_window(|window, cx| {
            Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx)
        });
        let editor_edited = Arc::new(AtomicBool::new(false));
        let fake_server = fake_servers.next().await.unwrap();
        let closure_editor_edited = Arc::clone(&editor_edited);
        fake_server
            .set_request_handler::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_editor_edited = Arc::clone(&closure_editor_edited);
                async move {
                    let hint_text = if params.text_document.uri
                        == lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap()
                    {
                        "main hint"
                    } else if params.text_document.uri
                        == lsp::Uri::from_file_path(path!("/a/other.rs")).unwrap()
                    {
                        "other hint"
                    } else {
                        panic!("unexpected uri: {:?}", params.text_document.uri);
                    };

                    let positions = [
                        lsp::Position::new(0, 2),
                        lsp::Position::new(4, 2),
                        lsp::Position::new(22, 2),
                        lsp::Position::new(44, 2),
                        lsp::Position::new(56, 2),
                        lsp::Position::new(67, 2),
                    ];
                    let out_of_range_hint = lsp::InlayHint {
                        position: lsp::Position::new(
                            params.range.start.line + 99,
                            params.range.start.character + 99,
                        ),
                        label: lsp::InlayHintLabel::String(
                            "out of excerpt range, should be ignored".to_string(),
                        ),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    };

                    let edited = task_editor_edited.load(Ordering::Acquire);
                    Ok(Some(
                        std::iter::once(out_of_range_hint)
                            .chain(positions.into_iter().enumerate().map(|(i, position)| {
                                lsp::InlayHint {
                                    position,
                                    label: lsp::InlayHintLabel::String(format!(
                                        "{hint_text}{} #{i}",
                                        if edited { "(edited)" } else { "" },
                                    )),
                                    kind: None,
                                    text_edits: None,
                                    tooltip: None,
                                    padding_left: None,
                                    padding_right: None,
                                    data: None,
                                }
                            }))
                            .collect(),
                    ))
                }
            })
            .next()
            .await;
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec![
                        "main hint #0".to_string(),
                        "main hint #1".to_string(),
                        "main hint #2".to_string(),
                        "main hint #3".to_string(),
                        "other hint #0".to_string(),
                        "other hint #1".to_string(),
                        "other hint #2".to_string(),
                        "other hint #3".to_string(),
                    ],
                    sorted_cached_hint_labels(editor, cx),
                    "Cache should update for both excerpts despite hints display was disabled; after selecting 2nd buffer, it's now registered with the langserever and should get its hints"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "All hints are disabled and should not be shown despite being present in the cache"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, _, cx| {
                editor.buffer().update(cx, |multibuffer, cx| {
                    multibuffer.remove_excerpts(PathKey::sorted(1), cx);
                })
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec![
                        "main hint #0".to_string(),
                        "main hint #1".to_string(),
                        "main hint #2".to_string(),
                        "main hint #3".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "For the removed excerpt, should clean corresponding cached hints as its buffer was dropped"
                );
                assert!(
                visible_hint_labels(editor, cx).is_empty(),
                "All hints are disabled and should not be shown despite being present in the cache"
            );
            })
            .unwrap();

        update_test_language_settings(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec![
                        "main hint #0".to_string(),
                        "main hint #1".to_string(),
                        "main hint #2".to_string(),
                        "main hint #3".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "Hint display settings change should not change the cache"
                );
                assert_eq!(
                    vec![
                        "main hint #0".to_string(),
                    ],
                    visible_hint_labels(editor, cx),
                    "Settings change should make cached hints visible, but only the visible ones, from the remaining excerpt"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_inside_char_boundary_range_hints(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": format!(r#"fn main() {{\n{}\n}}"#, format!("let i = {};\n", "√".repeat(10)).repeat(500)),
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());
        language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new(move |fake_server| {
                    let lsp_request_count = Arc::new(AtomicU32::new(0));
                    fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                        move |params, _| {
                            let i = lsp_request_count.fetch_add(1, Ordering::Release) + 1;
                            async move {
                                assert_eq!(
                                    params.text_document.uri,
                                    lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                                );
                                let query_start = params.range.start;
                                Ok(Some(vec![lsp::InlayHint {
                                    position: query_start,
                                    label: lsp::InlayHintLabel::String(i.to_string()),
                                    kind: None,
                                    text_edits: None,
                                    tooltip: None,
                                    padding_left: None,
                                    padding_right: None,
                                    data: None,
                                }]))
                            }
                        },
                    );
                })),
                ..FakeLspAdapter::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let editor =
            cx.add_window(|window, cx| Editor::for_buffer(buffer, Some(project), window, cx));

        // Allow LSP to initialize
        cx.executor().run_until_parked();

        // Establish a viewport and explicitly trigger hint refresh.
        // This ensures we control exactly when hints are requested.
        editor
            .update(cx, |editor, window, cx| {
                editor.set_visible_line_count(50.0, window, cx);
                editor.set_visible_column_count(120.0);
                editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            })
            .unwrap();

        // Allow LSP initialization and hint request/response to complete.
        // Use multiple advance_clock + run_until_parked cycles to ensure all async work completes.
        for _ in 0..5 {
            cx.executor().advance_clock(Duration::from_millis(100));
            cx.executor().run_until_parked();
        }

        // At this point we should have exactly one hint from our explicit refresh.
        // The test verifies that hints at character boundaries are handled correctly.
        editor
            .update(cx, |editor, _, cx| {
                assert!(
                    !cached_hint_labels(editor, cx).is_empty(),
                    "Should have at least one hint after refresh"
                );
                assert!(
                    !visible_hint_labels(editor, cx).is_empty(),
                    "Should have at least one visible hint"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_toggle_inlay_hints(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(false),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let (_, editor, _fake_server) = prepare_test_objects(cx, |fake_server, file_with_hints| {
            let lsp_request_count = Arc::new(AtomicU32::new(0));
            fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                move |params, _| {
                    let lsp_request_count = lsp_request_count.clone();
                    async move {
                        assert_eq!(
                            params.text_document.uri,
                            lsp::Uri::from_file_path(file_with_hints).unwrap(),
                        );

                        let i = lsp_request_count.fetch_add(1, Ordering::AcqRel) + 1;
                        Ok(Some(vec![lsp::InlayHint {
                            position: lsp::Position::new(0, i),
                            label: lsp::InlayHintLabel::String(i.to_string()),
                            kind: None,
                            text_edits: None,
                            tooltip: None,
                            padding_left: None,
                            padding_right: None,
                            data: None,
                        }]))
                    }
                },
            );
        })
        .await;

        editor
            .update(cx, |editor, window, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, window, cx)
            })
            .unwrap();

        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should display inlays after toggle despite them disabled in settings"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, window, cx)
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "Cache does not change because of toggles in the editor"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "Should clear hints after 2nd toggle"
                );
            })
            .unwrap();

        update_test_language_settings(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should not query LSP hints after enabling hints in settings, as file version is the same"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, window, cx)
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "Cache does not change because of toggles in the editor"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "Should clear hints after enabling in settings and a 3rd toggle"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, window, cx)
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor.update(cx, |editor, _, cx| {
            let expected_hints = vec!["1".to_string()];
            assert_eq!(
                expected_hints,
                cached_hint_labels(editor,cx),
                "Should not query LSP hints after enabling hints in settings and toggling them back on"
            );
            assert_eq!(expected_hints, visible_hint_labels(editor, cx));
        }).unwrap();
    }

    #[gpui::test]
    async fn test_modifiers_change(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let (_, editor, _fake_server) = prepare_test_objects(cx, |fake_server, file_with_hints| {
            let lsp_request_count = Arc::new(AtomicU32::new(0));
            fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                move |params, _| {
                    let lsp_request_count = lsp_request_count.clone();
                    async move {
                        assert_eq!(
                            params.text_document.uri,
                            lsp::Uri::from_file_path(file_with_hints).unwrap(),
                        );

                        let i = lsp_request_count.fetch_add(1, Ordering::AcqRel) + 1;
                        Ok(Some(vec![lsp::InlayHint {
                            position: lsp::Position::new(0, i),
                            label: lsp::InlayHintLabel::String(i.to_string()),
                            kind: None,
                            text_edits: None,
                            tooltip: None,
                            padding_left: None,
                            padding_right: None,
                            data: None,
                        }]))
                    }
                },
            );
        })
        .await;

        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "Should display inlays after toggle despite them disabled in settings"
                );
                assert_eq!(vec!["1".to_string()], visible_hint_labels(editor, cx));
            })
            .unwrap();

        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(true), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "Nothing happens with the cache on modifiers change"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "On modifiers change and hints toggled on, should hide editor inlays"
                );
            })
            .unwrap();
        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(true), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(vec!["1".to_string()], cached_hint_labels(editor, cx));
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "Nothing changes on consequent modifiers change of the same kind"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(false), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "When modifiers change is off, no extra requests are sent"
                );
                assert_eq!(
                    vec!["1".to_string()],
                    visible_hint_labels(editor, cx),
                    "When modifiers change is off, hints are back into the editor"
                );
            })
            .unwrap();
        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(false), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(vec!["1".to_string()], cached_hint_labels(editor, cx));
                assert_eq!(
                    vec!["1".to_string()],
                    visible_hint_labels(editor, cx),
                    "Nothing changes on consequent modifiers change of the same kind (2)"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, window, cx)
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "Nothing happens with the cache on modifiers change"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "When toggled off, should hide editor inlays"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(true), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "Nothing happens with the cache on modifiers change"
                );
                assert_eq!(
                    vec!["1".to_string()],
                    visible_hint_labels(editor, cx),
                    "On modifiers change & hints toggled off, should show editor inlays"
                );
            })
            .unwrap();
        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(true), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(vec!["1".to_string()], cached_hint_labels(editor, cx));
                assert_eq!(
                    vec!["1".to_string()],
                    visible_hint_labels(editor, cx),
                    "Nothing changes on consequent modifiers change of the same kind"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(false), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "When modifiers change is off, no extra requests are sent"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "When modifiers change is off, editor hints are back into their toggled off state"
                );
            })
            .unwrap();
        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(false), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(vec!["1".to_string()], cached_hint_labels(editor, cx));
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "Nothing changes on consequent modifiers change of the same kind (3)"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_inlays_at_the_same_place(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() {
                    let x = 42;
                    std::thread::scope(|s| {
                        s.spawn(|| {
                            let _x = x;
                        });
                    });
                }",
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());
        language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                initializer: Some(Box::new(move |fake_server| {
                    fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                        move |params, _| async move {
                            assert_eq!(
                                params.text_document.uri,
                                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                            );
                            Ok(Some(
                                serde_json::from_value(json!([
                                    {
                                        "position": {
                                            "line": 3,
                                            "character": 16
                                        },
                                        "label": "move",
                                        "paddingLeft": false,
                                        "paddingRight": false
                                    },
                                    {
                                        "position": {
                                            "line": 3,
                                            "character": 16
                                        },
                                        "label": "(",
                                        "paddingLeft": false,
                                        "paddingRight": false
                                    },
                                    {
                                        "position": {
                                            "line": 3,
                                            "character": 16
                                        },
                                        "label": [
                                            {
                                                "value": "&x"
                                            }
                                        ],
                                        "paddingLeft": false,
                                        "paddingRight": false,
                                        "data": {
                                            "file_id": 0
                                        }
                                    },
                                    {
                                        "position": {
                                            "line": 3,
                                            "character": 16
                                        },
                                        "label": ")",
                                        "paddingLeft": false,
                                        "paddingRight": true
                                    },
                                    // not a correct syntax, but checks that same symbols at the same place
                                    // are not deduplicated
                                    {
                                        "position": {
                                            "line": 3,
                                            "character": 16
                                        },
                                        "label": ")",
                                        "paddingLeft": false,
                                        "paddingRight": true
                                    },
                                ]))
                                .unwrap(),
                            ))
                        },
                    );
                })),
                ..FakeLspAdapter::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();

        // Use a VisualTestContext and explicitly establish a viewport on the editor (the production
        // trigger for `NewLinesShown` / inlay hint refresh) by setting visible line/column counts.
        let (editor_entity, cx) =
            cx.add_window_view(|window, cx| Editor::for_buffer(buffer, Some(project), window, cx));

        editor_entity.update_in(cx, |editor, window, cx| {
            // Establish a viewport. The exact values are not important for this test; we just need
            // the editor to consider itself visible so the refresh pipeline runs.
            editor.set_visible_line_count(50.0, window, cx);
            editor.set_visible_column_count(120.0);

            // Explicitly trigger a refresh now that the viewport exists.
            editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
        });
        cx.executor().run_until_parked();

        editor_entity.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges([Point::new(10, 0)..Point::new(10, 0)])
            });
        });
        cx.executor().run_until_parked();

        // Allow any async inlay hint request/response work to complete.
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        editor_entity.update(cx, |editor, cx| {
            let expected_hints = vec![
                "move".to_string(),
                "(".to_string(),
                "&x".to_string(),
                ") ".to_string(),
                ") ".to_string(),
            ];
            assert_eq!(
                expected_hints,
                cached_hint_labels(editor, cx),
                "Editor inlay hints should repeat server's order when placed at the same spot"
            );
            assert_eq!(expected_hints, visible_hint_labels(editor, cx));
        });
    }

    #[gpui::test]
    async fn test_invalidation_and_addition_race(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                enabled: Some(true),
                ..InlayHintSettingsContent::default()
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": r#"fn main() {
                    let x = 1;
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    let x = "2";
                }
"#,
                "lib.rs": r#"fn aaa() {
                    let aa = 22;
                }
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //

                fn bb() {
                    let bb = 33;
                }
"#
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let language = rust_lang();
        language_registry.add(language);

        let requests_count = Arc::new(AtomicUsize::new(0));
        let closure_requests_count = requests_count.clone();
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: "rust-analyzer",
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new(move |fake_server| {
                    let requests_count = closure_requests_count.clone();
                    fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                        move |params, _| {
                            let requests_count = requests_count.clone();
                            async move {
                                requests_count.fetch_add(1, Ordering::Release);
                                if params.text_document.uri
                                    == lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap()
                                {
                                    Ok(Some(vec![
                                        lsp::InlayHint {
                                            position: lsp::Position::new(1, 9),
                                            label: lsp::InlayHintLabel::String(": i32".to_owned()),
                                            kind: Some(lsp::InlayHintKind::TYPE),
                                            text_edits: None,
                                            tooltip: None,
                                            padding_left: None,
                                            padding_right: None,
                                            data: None,
                                        },
                                        lsp::InlayHint {
                                            position: lsp::Position::new(19, 9),
                                            label: lsp::InlayHintLabel::String(": i33".to_owned()),
                                            kind: Some(lsp::InlayHintKind::TYPE),
                                            text_edits: None,
                                            tooltip: None,
                                            padding_left: None,
                                            padding_right: None,
                                            data: None,
                                        },
                                    ]))
                                } else if params.text_document.uri
                                    == lsp::Uri::from_file_path(path!("/a/lib.rs")).unwrap()
                                {
                                    Ok(Some(vec![
                                        lsp::InlayHint {
                                            position: lsp::Position::new(1, 10),
                                            label: lsp::InlayHintLabel::String(": i34".to_owned()),
                                            kind: Some(lsp::InlayHintKind::TYPE),
                                            text_edits: None,
                                            tooltip: None,
                                            padding_left: None,
                                            padding_right: None,
                                            data: None,
                                        },
                                        lsp::InlayHint {
                                            position: lsp::Position::new(29, 10),
                                            label: lsp::InlayHintLabel::String(": i35".to_owned()),
                                            kind: Some(lsp::InlayHintKind::TYPE),
                                            text_edits: None,
                                            tooltip: None,
                                            padding_left: None,
                                            padding_right: None,
                                            data: None,
                                        },
                                    ]))
                                } else {
                                    panic!("Unexpected file path {:?}", params.text_document.uri);
                                }
                            }
                        },
                    );
                })),
                ..FakeLspAdapter::default()
            },
        );

        // Add another server that does send the same, duplicate hints back
        let mut fake_servers_2 = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: "CrabLang-ls",
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new(move |fake_server| {
                    fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                        move |params, _| async move {
                            if params.text_document.uri
                                == lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap()
                            {
                                Ok(Some(vec![
                                    lsp::InlayHint {
                                        position: lsp::Position::new(1, 9),
                                        label: lsp::InlayHintLabel::String(": i32".to_owned()),
                                        kind: Some(lsp::InlayHintKind::TYPE),
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    },
                                    lsp::InlayHint {
                                        position: lsp::Position::new(19, 9),
                                        label: lsp::InlayHintLabel::String(": i33".to_owned()),
                                        kind: Some(lsp::InlayHintKind::TYPE),
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    },
                                ]))
                            } else if params.text_document.uri
                                == lsp::Uri::from_file_path(path!("/a/lib.rs")).unwrap()
                            {
                                Ok(Some(vec![
                                    lsp::InlayHint {
                                        position: lsp::Position::new(1, 10),
                                        label: lsp::InlayHintLabel::String(": i34".to_owned()),
                                        kind: Some(lsp::InlayHintKind::TYPE),
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    },
                                    lsp::InlayHint {
                                        position: lsp::Position::new(29, 10),
                                        label: lsp::InlayHintLabel::String(": i35".to_owned()),
                                        kind: Some(lsp::InlayHintKind::TYPE),
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    },
                                ]))
                            } else {
                                panic!("Unexpected file path {:?}", params.text_document.uri);
                            }
                        },
                    );
                })),
                ..FakeLspAdapter::default()
            },
        );

        let (buffer_1, _handle_1) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let (buffer_2, _handle_2) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/lib.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_excerpts_for_path(
                PathKey::sorted(0),
                buffer_2.clone(),
                [
                    Point::new(0, 0)..Point::new(10, 0),
                    Point::new(23, 0)..Point::new(34, 0),
                ],
                0,
                cx,
            );
            multibuffer.set_excerpts_for_path(
                PathKey::sorted(1),
                buffer_1.clone(),
                [
                    Point::new(0, 0)..Point::new(10, 0),
                    Point::new(13, 0)..Point::new(23, 0),
                ],
                0,
                cx,
            );
            multibuffer
        });

        let editor = cx.add_window(|window, cx| {
            let mut editor =
                Editor::for_multibuffer(multi_buffer, Some(project.clone()), window, cx);
            editor.change_selections(SelectionEffects::default(), window, cx, |s| {
                s.select_ranges([Point::new(3, 3)..Point::new(3, 3)])
            });
            editor
        });

        let fake_server = fake_servers.next().await.unwrap();
        let _fake_server_2 = fake_servers_2.next().await.unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    vec![
                        ": i32".to_string(),
                        ": i32".to_string(),
                        ": i33".to_string(),
                        ": i33".to_string(),
                        ": i34".to_string(),
                        ": i34".to_string(),
                        ": i35".to_string(),
                        ": i35".to_string(),
                    ],
                    sorted_cached_hint_labels(editor, cx),
                    "We receive duplicate hints from 2 servers and cache them all"
                );
                assert_eq!(
                    vec![
                        ": i34".to_string(),
                        ": i35".to_string(),
                        ": i32".to_string(),
                        ": i33".to_string(),
                    ],
                    visible_hint_labels(editor, cx),
                    "lib.rs is added before main.rs , so its excerpts should be visible first; hints should be deduplicated per label"
                );
            })
            .unwrap();
        assert_eq!(
            requests_count.load(Ordering::Acquire),
            2,
            "Should have queried hints once per each file"
        );

        // Scroll all the way down so the 1st buffer is out of sight.
        // The selection is on the 1st buffer still.
        editor
            .update(cx, |editor, window, cx| {
                editor.scroll_screen(&ScrollAmount::Line(88.0), window, cx);
            })
            .unwrap();
        // Emulate a language server refresh request, coming in the background..
        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(
                    InlayHintRefreshReason::RefreshRequested {
                        server_id: fake_server.server.server_id(),
                        request_id: Some(1),
                    },
                    cx,
                );
            })
            .unwrap();
        // Edit the 1st buffer while scrolled down and not seeing that.
        // The edit will auto scroll to the edit (1st buffer).
        editor
            .update(cx, |editor, window, cx| {
                editor.handle_input("a", window, cx);
            })
            .unwrap();
        // Add more racy additive hint tasks.
        editor
            .update(cx, |editor, window, cx| {
                editor.scroll_screen(&ScrollAmount::Line(0.2), window, cx);
            })
            .unwrap();

        cx.executor().advance_clock(Duration::from_millis(1000));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    vec![
                        ": i32".to_string(),
                        ": i32".to_string(),
                        ": i33".to_string(),
                        ": i33".to_string(),
                        ": i34".to_string(),
                        ": i34".to_string(),
                        ": i35".to_string(),
                        ": i35".to_string(),
                    ],
                    sorted_cached_hint_labels(editor, cx),
                    "No hint changes/duplicates should occur in the cache",
                );
                assert_eq!(
                    vec![
                        ": i34".to_string(),
                        ": i35".to_string(),
                        ": i32".to_string(),
                        ": i33".to_string(),
                    ],
                    visible_hint_labels(editor, cx),
                    "No hint changes/duplicates should occur in the editor excerpts",
                );
            })
            .unwrap();
        assert_eq!(
            requests_count.load(Ordering::Acquire),
            4,
            "Should have queried hints once more per each file, after editing the file once"
        );
    }

    #[gpui::test]
    async fn test_edit_then_scroll_race(cx: &mut gpui::TestAppContext) {
        // Bug 1: An edit fires with a long debounce, and a scroll brings new lines
        // before that debounce elapses. The edit task's apply_fetched_hints removes
        // ALL visible hints (including the scroll-added ones) but only adds back
        // hints for its own chunks. The scroll chunk remains in hint_chunk_fetching,
        // so it is never re-queried, leaving it permanently empty.
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                enabled: Some(true),
                edit_debounce_ms: Some(700),
                scroll_debounce_ms: Some(50),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                ..InlayHintSettingsContent::default()
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        let mut file_content = String::from("fn main() {\n");
        for i in 0..150 {
            file_content.push_str(&format!("    let v{i} = {i};\n"));
        }
        file_content.push_str("}\n");
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": file_content,
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());

        let lsp_request_ranges = Arc::new(Mutex::new(Vec::new()));
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new({
                    let lsp_request_ranges = lsp_request_ranges.clone();
                    move |fake_server| {
                        let lsp_request_ranges = lsp_request_ranges.clone();
                        fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                            move |params, _| {
                                let lsp_request_ranges = lsp_request_ranges.clone();
                                async move {
                                    lsp_request_ranges.lock().push(params.range);
                                    let start_line = params.range.start.line;
                                    Ok(Some(vec![lsp::InlayHint {
                                        position: lsp::Position::new(start_line + 1, 9),
                                        label: lsp::InlayHintLabel::String(format!(
                                            "chunk_{start_line}"
                                        )),
                                        kind: Some(lsp::InlayHintKind::TYPE),
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    }]))
                                }
                            },
                        );
                    }
                })),
                ..FakeLspAdapter::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let editor =
            cx.add_window(|window, cx| Editor::for_buffer(buffer, Some(project), window, cx));
        cx.executor().run_until_parked();
        let _fake_server = fake_servers.next().await.unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.set_visible_line_count(50.0, window, cx);
                editor.set_visible_column_count(120.0);
                editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _window, cx| {
                let visible = visible_hint_labels(editor, cx);
                assert!(
                    visible.iter().any(|h| h.starts_with("chunk_0")),
                    "Should have chunk_0 hints initially, got: {visible:?}"
                );
            })
            .unwrap();

        lsp_request_ranges.lock().clear();

        // Step 1: Make an edit → triggers BufferEdited with 700ms debounce.
        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([MultiBufferOffset(13)..MultiBufferOffset(13)])
                });
                editor.handle_input("x", window, cx);
            })
            .unwrap();
        // Let the BufferEdited event propagate and the edit task get spawned.
        cx.executor().run_until_parked();

        // Step 2: Scroll down to reveal a new chunk, then trigger NewLinesShown.
        // This spawns a scroll task with the shorter 50ms debounce.
        editor
            .update(cx, |editor, window, cx| {
                editor.scroll_screen(&ScrollAmount::Page(1.0), window, cx);
            })
            .unwrap();
        // Explicitly trigger NewLinesShown for the new visible range.
        editor
            .update(cx, |editor, _window, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            })
            .unwrap();

        // Step 3: Advance clock past scroll debounce (50ms) but NOT past edit
        // debounce (700ms). The scroll task completes and adds hints for the
        // new chunk.
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        // The scroll task's apply_fetched_hints also processes
        // invalidate_hints_for_buffers (set by the earlier BufferEdited), which
        // removes the old chunk_0 hint. Only the scroll chunk's hint remains.
        editor
            .update(cx, |editor, _window, cx| {
                let visible = visible_hint_labels(editor, cx);
                assert!(
                    visible.iter().any(|h| h.starts_with("chunk_50")),
                    "After scroll task completes, the scroll chunk's hints should be \
                     present, got: {visible:?}"
                );
            })
            .unwrap();

        // Step 4: Advance clock past the edit debounce (700ms). The edit task
        // completes, calling apply_fetched_hints with should_invalidate()=true,
        // which removes ALL visible hints (including the scroll chunk's) but only
        // adds back hints for its own chunks (chunk_0).
        cx.executor().advance_clock(Duration::from_millis(700));
        cx.executor().run_until_parked();

        // At this point the edit task has:
        //   - removed chunk_50's hint (via should_invalidate removing all visible)
        //   - added chunk_0's hint (from its own fetch)
        //   - (with fix) cleared chunk_50 from hint_chunk_fetching
        // Without the fix, chunk_50 is stuck in hint_chunk_fetching and will
        // never be re-queried by NewLinesShown.

        // Step 5: Trigger NewLinesShown to give the system a chance to re-fetch
        // any chunks whose hints were lost.
        editor
            .update(cx, |editor, _window, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _window, cx| {
                let visible = visible_hint_labels(editor, cx);
                assert!(
                    visible.iter().any(|h| h.starts_with("chunk_0")),
                    "chunk_0 hints (from edit task) should be present. Got: {visible:?}"
                );
                assert!(
                    visible.iter().any(|h| h.starts_with("chunk_50")),
                    "chunk_50 hints should have been re-fetched after NewLinesShown. \
                     Bug 1: the scroll chunk's hints were removed by the edit task \
                     and the chunk was stuck in hint_chunk_fetching, preventing \
                     re-fetch. Got: {visible:?}"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_refresh_requested_multi_server(cx: &mut gpui::TestAppContext) {
        // Bug 2: When one LSP server sends workspace/inlayHint/refresh, the editor
        // wipes all tracking state via clear(), then spawns tasks that call
        // LspStore::inlay_hints with for_server=Some(requesting_server). The LspStore
        // filters out other servers' cached hints via the for_server guard, so only
        // the requesting server's hints are returned. apply_fetched_hints removes ALL
        // visible hints (should_invalidate()=true) but only adds back the requesting
        // server's hints. Other servers' hints disappear permanently.
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                ..InlayHintSettingsContent::default()
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { let x = 1; } // padding to keep hints from being trimmed",
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());

        // Server A returns a hint labeled "server_a".
        let server_a_request_count = Arc::new(AtomicU32::new(0));
        let mut fake_servers_a = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: "rust-analyzer",
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new({
                    let server_a_request_count = server_a_request_count.clone();
                    move |fake_server| {
                        let server_a_request_count = server_a_request_count.clone();
                        fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                            move |_params, _| {
                                let count =
                                    server_a_request_count.fetch_add(1, Ordering::Release) + 1;
                                async move {
                                    Ok(Some(vec![lsp::InlayHint {
                                        position: lsp::Position::new(0, 9),
                                        label: lsp::InlayHintLabel::String(format!(
                                            "server_a_{count}"
                                        )),
                                        kind: Some(lsp::InlayHintKind::TYPE),
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    }]))
                                }
                            },
                        );
                    }
                })),
                ..FakeLspAdapter::default()
            },
        );

        // Server B returns a hint labeled "server_b" at a different position.
        let server_b_request_count = Arc::new(AtomicU32::new(0));
        let mut fake_servers_b = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: "secondary-ls",
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new({
                    let server_b_request_count = server_b_request_count.clone();
                    move |fake_server| {
                        let server_b_request_count = server_b_request_count.clone();
                        fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                            move |_params, _| {
                                let count =
                                    server_b_request_count.fetch_add(1, Ordering::Release) + 1;
                                async move {
                                    Ok(Some(vec![lsp::InlayHint {
                                        position: lsp::Position::new(0, 22),
                                        label: lsp::InlayHintLabel::String(format!(
                                            "server_b_{count}"
                                        )),
                                        kind: Some(lsp::InlayHintKind::TYPE),
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    }]))
                                }
                            },
                        );
                    }
                })),
                ..FakeLspAdapter::default()
            },
        );

        let (buffer, _buffer_handle) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let editor =
            cx.add_window(|window, cx| Editor::for_buffer(buffer, Some(project), window, cx));
        cx.executor().run_until_parked();

        let fake_server_a = fake_servers_a.next().await.unwrap();
        let _fake_server_b = fake_servers_b.next().await.unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.set_visible_line_count(50.0, window, cx);
                editor.set_visible_column_count(120.0);
                editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        // Verify both servers' hints are present initially.
        editor
            .update(cx, |editor, _window, cx| {
                let visible = visible_hint_labels(editor, cx);
                let has_a = visible.iter().any(|h| h.starts_with("server_a"));
                let has_b = visible.iter().any(|h| h.starts_with("server_b"));
                assert!(
                    has_a && has_b,
                    "Both servers should have hints initially. Got: {visible:?}"
                );
            })
            .unwrap();

        // Trigger RefreshRequested from server A. This should re-fetch server A's
        // hints while keeping server B's hints intact.
        editor
            .update(cx, |editor, _window, cx| {
                editor.refresh_inlay_hints(
                    InlayHintRefreshReason::RefreshRequested {
                        server_id: fake_server_a.server.server_id(),
                        request_id: Some(1),
                    },
                    cx,
                );
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        // Also trigger NewLinesShown to give the system a chance to recover
        // any chunks that might have been cleared.
        editor
            .update(cx, |editor, _window, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _window, cx| {
                let visible = visible_hint_labels(editor, cx);
                let has_a = visible.iter().any(|h| h.starts_with("server_a"));
                let has_b = visible.iter().any(|h| h.starts_with("server_b"));
                assert!(
                    has_a,
                    "Server A hints should be present after its own refresh. Got: {visible:?}"
                );
                assert!(
                    has_b,
                    "Server B hints should NOT be lost when server A triggers \
                     RefreshRequested. Bug 2: clear() wipes all tracking, then \
                     LspStore filters out server B's cached hints via the for_server \
                     guard, and apply_fetched_hints removes all visible hints but only \
                     adds back server A's. Got: {visible:?}"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_multi_language_multibuffer_no_duplicate_hints(cx: &mut gpui::TestAppContext) {
        init_test(cx, &|settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { let x = 1; } // padding to keep hints from being trimmed",
                "index.ts": "const y = 2; // padding to keep hints from being trimmed in typescript",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());

        let mut rs_fake_servers = None;
        let mut ts_fake_servers = None;
        for (name, path_suffix) in [("Rust", "rs"), ("TypeScript", "ts")] {
            language_registry.add(Arc::new(Language::new(
                LanguageConfig {
                    name: name.into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec![path_suffix.to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )));
            let fake_servers = language_registry.register_fake_lsp(
                name,
                FakeLspAdapter {
                    name,
                    capabilities: lsp::ServerCapabilities {
                        inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                        ..Default::default()
                    },
                    initializer: Some(Box::new({
                        move |fake_server| {
                            let request_count = Arc::new(AtomicU32::new(0));
                            fake_server
                                .set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                                    move |params, _| {
                                        let count =
                                            request_count.fetch_add(1, Ordering::Release) + 1;
                                        let prefix = match name {
                                            "Rust" => "rs_hint",
                                            "TypeScript" => "ts_hint",
                                            other => panic!("Unexpected language: {other}"),
                                        };
                                        async move {
                                            Ok(Some(vec![lsp::InlayHint {
                                                position: params.range.start,
                                                label: lsp::InlayHintLabel::String(format!(
                                                    "{prefix}_{count}"
                                                )),
                                                kind: None,
                                                text_edits: None,
                                                tooltip: None,
                                                padding_left: None,
                                                padding_right: None,
                                                data: None,
                                            }]))
                                        }
                                    },
                                );
                        }
                    })),
                    ..Default::default()
                },
            );
            match name {
                "Rust" => rs_fake_servers = Some(fake_servers),
                "TypeScript" => ts_fake_servers = Some(fake_servers),
                _ => unreachable!(),
            }
        }

        let (rs_buffer, _rs_handle) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let (ts_buffer, _ts_handle) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/index.ts"), cx)
            })
            .await
            .unwrap();

        let multi_buffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_excerpts_for_path(
                PathKey::sorted(0),
                rs_buffer.clone(),
                [Point::new(0, 0)..Point::new(1, 0)],
                0,
                cx,
            );
            multibuffer.set_excerpts_for_path(
                PathKey::sorted(1),
                ts_buffer.clone(),
                [Point::new(0, 0)..Point::new(1, 0)],
                0,
                cx,
            );
            multibuffer
        });

        cx.executor().run_until_parked();
        let editor = cx.add_window(|window, cx| {
            Editor::for_multibuffer(multi_buffer, Some(project.clone()), window, cx)
        });

        let _rs_fake_server = rs_fake_servers.unwrap().next().await.unwrap();
        let _ts_fake_server = ts_fake_servers.unwrap().next().await.unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        // Verify initial state: both languages have exactly one hint each
        editor
            .update(cx, |editor, _window, cx| {
                let visible = visible_hint_labels(editor, cx);
                let rs_hints: Vec<_> = visible
                    .iter()
                    .filter(|h| h.starts_with("rs_hint"))
                    .collect();
                let ts_hints: Vec<_> = visible
                    .iter()
                    .filter(|h| h.starts_with("ts_hint"))
                    .collect();
                assert_eq!(
                    rs_hints.len(),
                    1,
                    "Should have exactly 1 Rust hint initially, got: {rs_hints:?}"
                );
                assert_eq!(
                    ts_hints.len(),
                    1,
                    "Should have exactly 1 TypeScript hint initially, got: {ts_hints:?}"
                );
            })
            .unwrap();

        // Edit the Rust buffer — triggers BufferEdited(rust_buffer_id).
        // The language filter in refresh_inlay_hints excludes TypeScript excerpts
        // from processing, but the global clear() wipes added_hints for ALL buffers.
        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([MultiBufferOffset(0)..MultiBufferOffset(0)])
                });
                editor.handle_input("x", window, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();

        // Trigger NewLinesShown — this causes TypeScript chunks to be re-fetched
        // because hint_chunk_fetching was wiped by clear(). The cached hints pass
        // the added_hints.insert(...).is_none() filter (also wiped) and get inserted
        // alongside the still-displayed copies, causing duplicates.
        editor
            .update(cx, |editor, _window, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();

        // Assert: TypeScript hints must NOT be duplicated
        editor
            .update(cx, |editor, _window, cx| {
                let visible = visible_hint_labels(editor, cx);
                let ts_hints: Vec<_> = visible
                    .iter()
                    .filter(|h| h.starts_with("ts_hint"))
                    .collect();
                assert_eq!(
                    ts_hints.len(),
                    1,
                    "TypeScript hints should NOT be duplicated after editing Rust buffer \
                     and triggering NewLinesShown. Got: {ts_hints:?}"
                );

                let rs_hints: Vec<_> = visible
                    .iter()
                    .filter(|h| h.starts_with("rs_hint"))
                    .collect();
                assert_eq!(
                    rs_hints.len(),
                    1,
                    "Rust hints should still be present after editing. Got: {rs_hints:?}"
                );
            })
            .unwrap();
    }

    pub(crate) fn init_test(cx: &mut TestAppContext, f: &dyn Fn(&mut AllLanguageSettingsContent)) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            release_channel::init(semver::Version::new(0, 0, 0), cx);
            crate::init(cx);
        });

        update_test_language_settings(cx, f);
    }

    async fn prepare_test_objects(
        cx: &mut TestAppContext,
        initialize: impl 'static + Send + Fn(&mut FakeLanguageServer, &'static str) + Send + Sync,
    ) -> (&'static str, WindowHandle<Editor>, FakeLanguageServer) {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { a } // and some long comment to ensure inlays are not trimmed out",
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let file_path = path!("/a/main.rs");

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new(move |server| initialize(server, file_path))),
                ..FakeLspAdapter::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let editor =
            cx.add_window(|window, cx| Editor::for_buffer(buffer, Some(project), window, cx));

        editor
            .update(cx, |editor, _, cx| {
                assert!(cached_hint_labels(editor, cx).is_empty());
                assert!(visible_hint_labels(editor, cx).is_empty());
            })
            .unwrap();

        cx.executor().run_until_parked();
        let fake_server = fake_servers.next().await.unwrap();

        // Establish a viewport so the editor considers itself visible and the hint refresh
        // pipeline runs. Then explicitly trigger a refresh.
        editor
            .update(cx, |editor, window, cx| {
                editor.set_visible_line_count(50.0, window, cx);
                editor.set_visible_column_count(120.0);
                editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        (file_path, editor, fake_server)
    }

    // Inlay hints in the cache are stored per excerpt as a key, and those keys are guaranteed to be ordered same as in the multi buffer.
    // Ensure a stable order for testing.
    fn sorted_cached_hint_labels(editor: &Editor, cx: &mut App) -> Vec<String> {
        let mut labels = cached_hint_labels(editor, cx);
        labels.sort_by(|a, b| natural_sort(a, b));
        labels
    }

    pub fn cached_hint_labels(editor: &Editor, cx: &mut App) -> Vec<String> {
        let lsp_store = editor.project().unwrap().read(cx).lsp_store();

        let mut all_cached_labels = Vec::new();
        let mut all_fetched_hints = Vec::new();
        for buffer in editor.buffer.read(cx).all_buffers() {
            lsp_store.update(cx, |lsp_store, cx| {
                let hints = lsp_store.latest_lsp_data(&buffer, cx).inlay_hints();
                all_cached_labels.extend(hints.all_cached_hints().into_iter().map(|hint| {
                    let mut label = hint.text().to_string();
                    if hint.padding_left {
                        label.insert(0, ' ');
                    }
                    if hint.padding_right {
                        label.push_str(" ");
                    }
                    label
                }));
                all_fetched_hints.extend(hints.all_fetched_hints());
            });
        }

        all_cached_labels
    }

    pub fn visible_hint_labels(editor: &Editor, cx: &Context<Editor>) -> Vec<String> {
        Editor::visible_inlay_hints(editor.display_map.read(cx))
            .map(|hint| hint.text().to_string())
            .collect()
    }

    fn allowed_hint_kinds_for_editor(editor: &Editor) -> HashSet<Option<InlayHintKind>> {
        Editor::allowed_hint_kinds_for_editor(editor)
    }
}
