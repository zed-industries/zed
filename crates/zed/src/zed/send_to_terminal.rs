use crate::{
    App,
    zed::{TerminalPanel, with_active_or_new_workspace},
};
use editor::ToOffset;
use workspace::Panel;

pub(crate) fn send_to_terminal(cx: &mut App) {
    with_active_or_new_workspace(cx, |workspace, window, cx| {
        let Some(active_item) = workspace.active_item(cx) else {
            return;
        };

        let Some(editor) = active_item.act_as::<editor::Editor>(cx) else {
            return;
        };

        let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
        let selection = editor.read(cx).selections.newest_anchor();
        let selection_range = selection.start.to_offset(&buffer)..selection.end.to_offset(&buffer);

        let (expression_text, expression_range) = if selection_range.is_empty() {
            let cursor_offset = selection_range.start;
            let cursor_point = buffer.offset_to_point(cursor_offset);

            let line_range = buffer.point_to_offset(language::Point::new(cursor_point.row, 0))
                ..buffer.point_to_offset(language::Point::new(cursor_point.row + 1, 0));
            let line_text = buffer
                .text_for_range(line_range.clone())
                .collect::<String>();

            let search_offset = if line_text.trim().is_empty() {
                line_range.end
            } else {
                line_range.start
            };

            let mut range = search_offset..search_offset;
            let mut found_valid_expression = false;

            while let Some((node, new_range)) = buffer.syntax_ancestor(range.clone()) {
                range = new_range;
                if node.is_named() {
                    let kind = node.kind();
                    if kind.contains("call")
                        || kind.contains("statement")
                        || kind.contains("expression")
                        || kind.contains("assignment")
                        || kind == "binary_operator"
                    {
                        found_valid_expression = true;
                        break;
                    }
                }
            }

            if !found_valid_expression {
                range = search_offset..search_offset;
                while let Some((node, new_range)) = buffer.syntax_ancestor(range.clone()) {
                    range = new_range;
                    if node.is_named() {
                        break;
                    }
                }
            }

            if range.is_empty() {
                (line_text, line_range)
            } else {
                (
                    buffer.text_for_range(range.clone()).collect::<String>(),
                    range,
                )
            }
        } else {
            (
                buffer
                    .text_for_range(selection_range.clone())
                    .collect::<String>(),
                selection_range,
            )
        };

        let expression_text = expression_text.trim().to_string();

        if expression_text.is_empty() {
            return;
        }

        let Some(terminal_panel) = workspace.panel::<TerminalPanel>(cx) else {
            return;
        };

        let Some(terminal_view) = terminal_panel.read(cx).pane().and_then(|pane| {
            pane.read(cx)
                .active_item()
                .and_then(|item| item.downcast::<terminal_view::TerminalView>())
        }) else {
            return;
        };

        let command_with_newline = format!("{}\n", expression_text);
        terminal_view.update(cx, |view, cx| {
            view.terminal().update(cx, |terminal, _cx| {
                terminal.input(command_with_newline.into_bytes());
            });
        });

        let end_point = buffer.offset_to_point(expression_range.end);
        let next_line_offset = buffer.point_to_offset(language::Point::new(end_point.row + 1, 0));
        let anchor = buffer.anchor_after(next_line_offset);

        editor.update(cx, |editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_anchor_ranges([anchor..anchor]);
            });
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zed::tests::init_test;
    use editor::Editor;
    use gpui::TestAppContext;
    use language::tree_sitter_python;
    use language::{Language, LanguageConfig, LanguageMatcher};
    use project::Project;
    use serde_json::json;
    use std::{path::PathBuf, sync::Arc};
    use util::path;
    use workspace::{OpenOptions, OpenVisible, Workspace};

    fn python_language() -> Arc<Language> {
        Arc::new(Language::new(
            LanguageConfig {
                name: "Python".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["py".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_python::LANGUAGE.into()),
        ))
    }

    #[gpui::test]
    async fn test_send_to_terminal_finds_expression(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "test.py": ""
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        let python_lang = python_language();

        project.update(cx, |project, _cx| {
            project.languages().add(python_lang.clone());
        });

        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let _res = window
            .update(cx, |workspace, window, cx| {
                workspace.open_paths(
                    vec![PathBuf::from(path!("/root/test.py"))],
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    None,
                    window,
                    cx,
                )
            })
            .unwrap();

        cx.background_executor.run_until_parked();

        let editor = window
            .update(cx, |workspace, _, cx| {
                workspace.active_item_as::<Editor>(cx).unwrap()
            })
            .unwrap();

        let code = r#"
            import pandas as pd

            df = pd.read_csv("data.csv")
            print(df.head())

            df.describe()
        "#;

        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.set_text(code, window, cx);
                    editor.buffer().update(cx, |buffer, cx| {
                        buffer.as_singleton().unwrap().update(cx, |buffer, cx| {
                            buffer.set_language(Some(python_lang.clone()), cx);
                        });
                    });
                })
            })
            .unwrap();

        cx.background_executor.run_until_parked();

        // Test 1: Cursor at start should find "import pandas as pd"
        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.select_ranges([0..0]);
                    });
                })
            })
            .unwrap();

        let buffer_text = editor.update(cx, |editor, cx| {
            editor.buffer().read(cx).snapshot(cx).text()
        });

        assert!(buffer_text.contains("import pandas as pd"));
        assert!(buffer_text.contains("df.describe()"));

        // Test 2: Cursor on function call line
        let function_call_offset = buffer_text.find("print(df.head())").unwrap();
        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.select_ranges([function_call_offset..function_call_offset]);
                    });
                })
            })
            .unwrap();

        let cursor_line = editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let cursor_offset = editor.selections.newest_anchor().head().to_offset(&buffer);
            let cursor_point = buffer.offset_to_point(cursor_offset);
            let line_start = buffer.point_to_offset(language::Point::new(cursor_point.row, 0));
            let line_end = buffer.point_to_offset(language::Point::new(cursor_point.row + 1, 0));
            buffer
                .text_for_range(line_start..line_end)
                .collect::<String>()
        });

        assert!(cursor_line.contains("print(df.head())"));
    }

    #[gpui::test]
    async fn test_send_to_terminal_cursor_movement(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "test.py": ""
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        let python_lang = python_language();

        project.update(cx, |project, _cx| {
            project.languages().add(python_lang.clone());
        });

        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let task = window
            .update(cx, |workspace, window, cx| {
                workspace.open_paths(
                    vec![PathBuf::from(path!("/root/test.py"))],
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    None,
                    window,
                    cx,
                )
            })
            .unwrap();
        task.await;

        cx.background_executor.run_until_parked();

        let editor = window
            .update(cx, |workspace, _, cx| {
                workspace.active_item_as::<Editor>(cx).unwrap()
            })
            .unwrap();

        let code = r#"
            print("line 1")
            print("line 2")
            print("line 3")
        "#;

        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.set_text(code, window, cx);
                    editor.buffer().update(cx, |buffer, cx| {
                        buffer.as_singleton().unwrap().update(cx, |buffer, cx| {
                            buffer.set_language(Some(python_lang.clone()), cx);
                        });
                    });
                })
            })
            .unwrap();

        cx.background_executor.run_until_parked();

        // Place cursor on first line
        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.select_ranges([0..0]);
                    });
                })
            })
            .unwrap();

        let initial_line = editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let cursor_offset = editor.selections.newest_anchor().head().to_offset(&buffer);
            buffer.offset_to_point(cursor_offset).row
        });

        assert_eq!(initial_line, 0);

        // Verify the code is loaded correctly
        let buffer_text = editor.update(cx, |editor, cx| {
            editor.buffer().read(cx).snapshot(cx).text()
        });

        assert!(buffer_text.contains("line 1"));
        assert!(buffer_text.contains("line 2"));
        assert!(buffer_text.contains("line 3"));
    }
}
