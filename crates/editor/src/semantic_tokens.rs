use std::{collections::hash_map, sync::Arc, time::Duration};

use collections::{HashMap, HashSet};
use futures::future::join_all;
use gpui::{
    App, Context, FontStyle, FontWeight, HighlightStyle, StrikethroughStyle, Task, UnderlineStyle,
};
use itertools::Itertools as _;
use language::language_settings::language_settings;
use project::{
    lsp_store::{BufferSemanticToken, BufferSemanticTokens, RefreshForServer},
    project_settings::ProjectSettings,
};
use settings::{
    SemanticTokenColorOverride, SemanticTokenFontStyle, SemanticTokenFontWeight,
    SemanticTokenRules, Settings as _,
};
use text::BufferId;
use theme::SyntaxTheme;
use ui::ActiveTheme as _;

use crate::{Editor, display_map::SemanticTokenHighlight};

impl Editor {
    pub(crate) fn update_semantic_tokens(
        &mut self,
        buffer_id: Option<BufferId>,
        for_server: Option<RefreshForServer>,
        cx: &mut Context<Self>,
    ) {
        if !self.mode().is_full() || !self.semantic_tokens_enabled {
            self.semantic_tokens_fetched_for_buffers.clear();
            self.display_map.update(cx, |display_map, _| {
                display_map.semantic_token_highlights.clear();
            });
            self.update_semantic_tokens_task = Task::ready(());
            return;
        }

        let mut invalidate_semantic_highlgights_for_buffers = HashSet::default();
        if for_server.is_some() {
            invalidate_semantic_highlgights_for_buffers.extend(
                self.semantic_tokens_fetched_for_buffers
                    .drain()
                    .map(|(buffer_id, _)| buffer_id),
            );
        }

        let Some((sema, project)) = self.semantics_provider.clone().zip(self.project.clone())
        else {
            return;
        };

        let buffers_to_query = self
            .visible_excerpts(true, cx)
            .into_values()
            .map(|(buffer, ..)| buffer)
            .chain(buffer_id.and_then(|buffer_id| self.buffer.read(cx).buffer(buffer_id)))
            .filter_map(|editor_buffer| {
                let editor_buffer_id = editor_buffer.read(cx).remote_id();
                let settings = language_settings(
                    editor_buffer.read(cx).language().map(|l| l.name()),
                    editor_buffer.read(cx).file(),
                    cx,
                );
                let retain = buffer_id.is_none_or(|buffer_id| buffer_id == editor_buffer_id)
                    && self.registered_buffers.contains_key(&editor_buffer_id)
                    && settings.semantic_tokens.enabled();
                if retain {
                    Some((editor_buffer_id, editor_buffer))
                } else {
                    self.display_map.update(cx, |display_map, _| {
                        display_map.invalidate_semantic_highlights(editor_buffer_id);
                    });
                    self.semantic_tokens_fetched_for_buffers
                        .remove(&editor_buffer_id);
                    None
                }
            })
            .unique_by(|(buffer_id, _)| *buffer_id)
            .collect::<Vec<_>>();

        self.update_semantic_tokens_task = cx.spawn(async move |editor, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(50))
                .await;
            let Some(all_semantic_tokens_task) = editor
                .update(cx, |editor, cx| {
                    buffers_to_query
                        .into_iter()
                        .filter_map(|(buffer_id, buffer)| {
                            let known_version =
                                editor.semantic_tokens_fetched_for_buffers.get(&buffer_id);
                            let query_version = buffer.read(cx).version();
                            if known_version.is_some_and(|known_version| {
                                !query_version.changed_since(known_version)
                            }) {
                                None
                            } else {
                                let task = sema.semantic_tokens(buffer, for_server, cx);
                                Some(async move { (buffer_id, query_version, task.await) })
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .ok()
            else {
                return;
            };

            let all_semantic_tokens = join_all(all_semantic_tokens_task).await;
            editor.update(cx, |editor, cx| {
                editor.display_map.update(cx, |display_map, _| {
                    for buffer_id in invalidate_semantic_highlgights_for_buffers {
                        display_map.invalidate_semantic_highlights(buffer_id);
                    }
                });


                if all_semantic_tokens.is_empty() {
                    return;
                }
                let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                let all_excerpts = editor.buffer().read(cx).excerpt_ids();

                for (buffer_id, query_version, tokens) in all_semantic_tokens {
                    let tokens = match tokens {
                        Ok(BufferSemanticTokens { tokens: Some(tokens) }) => {
                            tokens
                        },
                        Ok(BufferSemanticTokens { tokens: None }) => {
                            continue;
                        },
                        Err(e) => {
                            log::error!("Failed to fetch semantic tokens for buffer {buffer_id:?}: {e:#}");
                            continue;
                        },
                    };

                    match editor.semantic_tokens_fetched_for_buffers.entry(buffer_id) {
                        hash_map::Entry::Occupied(mut o) => {
                            if query_version.changed_since(o.get()) {
                                o.insert(query_version);
                            } else {
                                continue;
                            }
                        },
                        hash_map::Entry::Vacant(v) => {
                            v.insert(query_version);
                        },
                    }

                    editor.display_map.update(cx, |display_map, cx| {
                        let lsp_store = project.read(cx).lsp_store().read(cx);
                        let mut token_highlights = Vec::new();
                        for (server_id, server_tokens) in tokens {
                            let Some(legend) = lsp_store
                                .lsp_server_capabilities
                                .get(&server_id)
                                .and_then(|caps| caps.semantic_tokens_provider.as_ref())
                                .map(|provider| match provider {
                                    lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                                        opts,
                                    ) => &opts.legend,
                                    lsp::SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(opts) => {
                                        &opts.semantic_tokens_options.legend
                                    }
                                })
                            else {
                                continue;
                            };
                            let stylizer = SemanticTokenStylizer::new(legend, cx);
                            token_highlights.extend(buffer_into_editor_highlights(
                                &server_tokens,
                                &stylizer,
                                &all_excerpts,
                                &multi_buffer_snapshot,
                                cx,
                            ));
                        }

                        token_highlights.sort_by(|a, b| {
                            a.range.start.cmp(&b.range.start, &multi_buffer_snapshot)
                        });
                        display_map
                            .semantic_token_highlights
                            .insert(buffer_id, Arc::from(token_highlights));
                    });
                }

                cx.notify();
            }).ok();
        });
    }
}

fn buffer_into_editor_highlights<'a>(
    buffer_tokens: &'a [BufferSemanticToken],
    stylizer: &'a SemanticTokenStylizer<'a>,
    all_excerpts: &'a [multi_buffer::ExcerptId],
    multi_buffer_snapshot: &'a multi_buffer::MultiBufferSnapshot,
    cx: &'a gpui::App,
) -> impl Iterator<Item = SemanticTokenHighlight> + 'a {
    buffer_tokens.iter().filter_map(|token| {
        let multi_buffer_start = all_excerpts.iter().find_map(|&excerpt_id| {
            multi_buffer_snapshot.anchor_in_excerpt(excerpt_id, token.range.start)
        })?;
        let multi_buffer_end = all_excerpts.iter().find_map(|&excerpt_id| {
            multi_buffer_snapshot.anchor_in_excerpt(excerpt_id, token.range.end)
        })?;

        Some(SemanticTokenHighlight {
            range: multi_buffer_start..multi_buffer_end,
            style: stylizer.convert(
                cx.theme().syntax(),
                token.token_type,
                token.token_modifiers,
            )?,
        })
    })
}

struct SemanticTokenStylizer<'a> {
    rules: &'a SemanticTokenRules,
    token_types: Vec<&'a str>,
    modifier_mask: HashMap<&'a str, u32>,
}

impl<'a> SemanticTokenStylizer<'a> {
    pub fn new(legend: &'a lsp::SemanticTokensLegend, cx: &'a App) -> Self {
        let token_types = legend.token_types.iter().map(|s| s.as_str()).collect();
        let modifier_mask = legend
            .token_modifiers
            .iter()
            .enumerate()
            .map(|(i, modifier)| (modifier.as_str(), 1 << i))
            .collect();
        SemanticTokenStylizer {
            rules: &ProjectSettings::get_global(cx)
                .global_lsp_settings
                .semantic_token_rules,
            token_types,
            modifier_mask,
        }
    }

    pub fn token_type(&self, token_type: u32) -> Option<&'a str> {
        self.token_types.get(token_type as usize).copied()
    }

    pub fn has_modifier(&self, token_modifiers: u32, modifier: &str) -> bool {
        let Some(mask) = self.modifier_mask.get(modifier) else {
            return false;
        };
        (token_modifiers & mask) != 0
    }

    pub fn convert(
        &self,
        theme: &'a SyntaxTheme,
        token_type: u32,
        modifiers: u32,
    ) -> Option<HighlightStyle> {
        let name = self.token_type(token_type)?;

        let matching = self.rules.rules.iter().rev().filter(|rule| {
            rule.token_type.as_ref().is_none_or(|t| t == name)
                && rule
                    .token_modifiers
                    .iter()
                    .all(|m| self.has_modifier(modifiers, m))
        });

        let mut highlight = HighlightStyle::default();
        let mut empty = true;

        for rule in matching {
            empty = false;

            let style = rule.style.iter().find_map(|style| theme.get_opt(style));

            // Overwriting rules:
            // - Explicit fields have top priority.
            // - Then, styles from the theme (if found).
            // - Lastly, rules further down in the list are applied.
            macro_rules! overwrite {
                (
                    highlight.$highlight_field:ident,
                    SemanticTokenRule::$rule_field:ident,
                    $transform:expr $(,)?
                ) => {
                    highlight.$highlight_field = rule
                        .$rule_field
                        .map($transform)
                        .or_else(|| style.and_then(|s| s.$highlight_field))
                        .or(highlight.$highlight_field)
                };
            }

            overwrite!(
                highlight.color,
                SemanticTokenRule::foreground_color,
                Into::into,
            );

            overwrite!(
                highlight.background_color,
                SemanticTokenRule::background_color,
                Into::into,
            );

            overwrite!(
                highlight.font_weight,
                SemanticTokenRule::font_weight,
                |w| match w {
                    SemanticTokenFontWeight::Normal => FontWeight::NORMAL,
                    SemanticTokenFontWeight::Bold => FontWeight::BOLD,
                },
            );

            overwrite!(
                highlight.font_style,
                SemanticTokenRule::font_style,
                |s| match s {
                    SemanticTokenFontStyle::Normal => FontStyle::Normal,
                    SemanticTokenFontStyle::Italic => FontStyle::Italic,
                },
            );

            overwrite!(highlight.underline, SemanticTokenRule::underline, |u| {
                UnderlineStyle {
                    thickness: 1.0.into(),
                    color: match u {
                        SemanticTokenColorOverride::InheritForeground(true) => highlight.color,
                        SemanticTokenColorOverride::InheritForeground(false) => None,
                        SemanticTokenColorOverride::Replace(c) => Some(c.into()),
                    },
                    ..Default::default()
                }
            });

            overwrite!(
                highlight.strikethrough,
                SemanticTokenRule::strikethrough,
                |s| StrikethroughStyle {
                    thickness: 1.0.into(),
                    color: match s {
                        SemanticTokenColorOverride::InheritForeground(true) => highlight.color,
                        SemanticTokenColorOverride::InheritForeground(false) => None,
                        SemanticTokenColorOverride::Replace(c) => Some(c.into()),
                    },
                },
            );
        }

        if empty { None } else { Some(highlight) }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        ops::{Deref as _, Range},
        sync::atomic::{self, AtomicUsize},
    };

    use futures::StreamExt as _;
    use gpui::{AppContext as _, Entity, Focusable as _, TestAppContext, VisualTestContext};
    use language::{Language, LanguageConfig, LanguageMatcher};
    use languages::FakeLspAdapter;
    use multi_buffer::{AnchorRangeExt as _, ExcerptRange, MultiBuffer, MultiBufferOffset};
    use project::Project;
    use rope::Point;
    use serde_json::json;
    use settings::{LanguageSettingsContent, SemanticTokens, SettingsStore};
    use workspace::{Workspace, WorkspaceHandle as _};

    use crate::{
        Capability,
        editor_tests::{init_test, update_test_language_settings},
        test::{
            build_editor, build_editor_with_project, editor_lsp_test_context::EditorLspTestContext,
        },
    };

    use super::*;

    #[gpui::test]
    async fn lsp_semantic_tokens_full_capability(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |language_settings| {
            language_settings.languages.0.insert(
                "Rust".into(),
                LanguageSettingsContent {
                    semantic_tokens: Some(SemanticTokens::Full),
                    ..Default::default()
                },
            );
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                semantic_tokens_provider: Some(
                    lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                        lsp::SemanticTokensOptions {
                            legend: lsp::SemanticTokensLegend {
                                token_types: vec!["function".into()],
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

        let full_counter = Arc::new(AtomicUsize::new(0));
        let full_counter_clone = full_counter.clone();

        let mut full_request = cx
            .set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(
                move |_, _, _| {
                    full_counter_clone.fetch_add(1, atomic::Ordering::Release);
                    async move {
                        Ok(Some(lsp::SemanticTokensResult::Tokens(
                            lsp::SemanticTokens {
                                data: vec![
                                    0, // delta_line
                                    3, // delta_start
                                    4, // length
                                    0, // token_type
                                    0, // token_modifiers_bitset
                                ],
                                // The server isn't capable of deltas, so even though we sent back
                                // a result ID, the client shouldn't request a delta.
                                result_id: Some("a".into()),
                            },
                        )))
                    }
                },
            );

        cx.set_state("ˇfn main() {}");
        assert!(full_request.next().await.is_some());

        cx.run_until_parked();

        cx.set_state("ˇfn main() { a }");
        assert!(full_request.next().await.is_some());

        cx.run_until_parked();

        assert_eq!(
            extract_semantic_highlights(&cx.editor, &cx),
            vec![MultiBufferOffset(3)..MultiBufferOffset(7)]
        );

        assert_eq!(full_counter.load(atomic::Ordering::Acquire), 2);
    }

    #[gpui::test]
    async fn lsp_semantic_tokens_full_none_result_id(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |language_settings| {
            language_settings.languages.0.insert(
                "Rust".into(),
                LanguageSettingsContent {
                    semantic_tokens: Some(SemanticTokens::Full),
                    ..Default::default()
                },
            );
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                semantic_tokens_provider: Some(
                    lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                        lsp::SemanticTokensOptions {
                            legend: lsp::SemanticTokensLegend {
                                token_types: vec!["function".into()],
                                token_modifiers: vec![],
                            },
                            full: Some(lsp::SemanticTokensFullOptions::Delta { delta: Some(true) }),
                            ..Default::default()
                        },
                    ),
                ),
                ..Default::default()
            },
            cx,
        )
        .await;

        let full_counter = Arc::new(AtomicUsize::new(0));
        let full_counter_clone = full_counter.clone();

        let mut full_request = cx
            .set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(
                move |_, _, _| {
                    full_counter_clone.fetch_add(1, atomic::Ordering::Release);
                    async move {
                        Ok(Some(lsp::SemanticTokensResult::Tokens(
                            lsp::SemanticTokens {
                                data: vec![
                                    0, // delta_line
                                    3, // delta_start
                                    4, // length
                                    0, // token_type
                                    0, // token_modifiers_bitset
                                ],
                                result_id: None, // Sending back `None` forces the client to not use deltas.
                            },
                        )))
                    }
                },
            );

        cx.set_state("ˇfn main() {}");
        assert!(full_request.next().await.is_some());

        let task = cx.update_editor(|e, _, _| {
            std::mem::replace(&mut e.update_semantic_tokens_task, Task::ready(()))
        });
        task.await;

        cx.set_state("ˇfn main() { a }");
        assert!(full_request.next().await.is_some());

        let task = cx.update_editor(|e, _, _| {
            std::mem::replace(&mut e.update_semantic_tokens_task, Task::ready(()))
        });
        task.await;
        assert_eq!(
            extract_semantic_highlights(&cx.editor, &cx),
            vec![MultiBufferOffset(3)..MultiBufferOffset(7)]
        );
        assert_eq!(full_counter.load(atomic::Ordering::Acquire), 2);
    }

    #[gpui::test]
    async fn lsp_semantic_tokens_delta(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |language_settings| {
            language_settings.languages.0.insert(
                "Rust".into(),
                LanguageSettingsContent {
                    semantic_tokens: Some(SemanticTokens::Full),
                    ..Default::default()
                },
            );
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                semantic_tokens_provider: Some(
                    lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                        lsp::SemanticTokensOptions {
                            legend: lsp::SemanticTokensLegend {
                                token_types: vec!["function".into()],
                                token_modifiers: vec![],
                            },
                            full: Some(lsp::SemanticTokensFullOptions::Delta { delta: Some(true) }),
                            ..Default::default()
                        },
                    ),
                ),
                ..Default::default()
            },
            cx,
        )
        .await;

        let full_counter = Arc::new(AtomicUsize::new(0));
        let full_counter_clone = full_counter.clone();
        let delta_counter = Arc::new(AtomicUsize::new(0));
        let delta_counter_clone = delta_counter.clone();

        let mut full_request = cx
            .set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(
                move |_, _, _| {
                    full_counter_clone.fetch_add(1, atomic::Ordering::Release);
                    async move {
                        Ok(Some(lsp::SemanticTokensResult::Tokens(
                            lsp::SemanticTokens {
                                data: vec![
                                    0, // delta_line
                                    3, // delta_start
                                    4, // length
                                    0, // token_type
                                    0, // token_modifiers_bitset
                                ],
                                result_id: Some("a".into()),
                            },
                        )))
                    }
                },
            );

        let mut delta_request = cx
            .set_request_handler::<lsp::request::SemanticTokensFullDeltaRequest, _, _>(
                move |_, params, _| {
                    delta_counter_clone.fetch_add(1, atomic::Ordering::Release);
                    assert_eq!(params.previous_result_id, "a");
                    async move {
                        Ok(Some(lsp::SemanticTokensFullDeltaResult::TokensDelta(
                            lsp::SemanticTokensDelta {
                                edits: vec![],
                                result_id: Some("b".into()),
                            },
                        )))
                    }
                },
            );

        // Initial request, for the empty buffer.
        cx.set_state("ˇfn main() {}");
        assert!(full_request.next().await.is_some());
        let task = cx.update_editor(|e, _, _| {
            std::mem::replace(&mut e.update_semantic_tokens_task, Task::ready(()))
        });
        task.await;

        cx.set_state("ˇfn main() { a }");
        assert!(delta_request.next().await.is_some());
        let task = cx.update_editor(|e, _, _| {
            std::mem::replace(&mut e.update_semantic_tokens_task, Task::ready(()))
        });
        task.await;

        assert_eq!(
            extract_semantic_highlights(&cx.editor, &cx),
            vec![MultiBufferOffset(3)..MultiBufferOffset(7)]
        );

        assert_eq!(full_counter.load(atomic::Ordering::Acquire), 1);
        assert_eq!(delta_counter.load(atomic::Ordering::Acquire), 1);
    }

    #[gpui::test]
    async fn lsp_semantic_tokens_multiserver_full(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |language_settings| {
            language_settings.languages.0.insert(
                "TOML".into(),
                LanguageSettingsContent {
                    semantic_tokens: Some(SemanticTokens::Full),
                    ..Default::default()
                },
            );
        });

        let toml_language = Arc::new(Language::new(
            LanguageConfig {
                name: "TOML".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["toml".into()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        ));

        // We have 2 language servers for TOML in this test.
        let toml_legend_1 = lsp::SemanticTokensLegend {
            token_types: vec!["property".into()],
            token_modifiers: vec![],
        };
        let toml_legend_2 = lsp::SemanticTokensLegend {
            token_types: vec!["number".into()],
            token_modifiers: vec![],
        };

        let app_state = cx.update(workspace::AppState::test);

        cx.update(|cx| {
            assets::Assets.load_test_fonts(cx);
            crate::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let mut toml_server_1 = language_registry.register_fake_lsp(
            toml_language.name(),
            FakeLspAdapter {
                name: "toml1",
                capabilities: lsp::ServerCapabilities {
                    semantic_tokens_provider: Some(
                        lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                            lsp::SemanticTokensOptions {
                                legend: toml_legend_1,
                                full: Some(lsp::SemanticTokensFullOptions::Delta { delta: None }),
                                ..Default::default()
                            },
                        ),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        let mut toml_server_2 = language_registry.register_fake_lsp(
            toml_language.name(),
            FakeLspAdapter {
                name: "toml2",
                capabilities: lsp::ServerCapabilities {
                    semantic_tokens_provider: Some(
                        lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                            lsp::SemanticTokensOptions {
                                legend: toml_legend_2,
                                full: Some(lsp::SemanticTokensFullOptions::Delta { delta: None }),
                                ..Default::default()
                            },
                        ),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        language_registry.add(toml_language.clone());

        app_state
            .fs
            .as_fake()
            .insert_tree(
                EditorLspTestContext::root_path(),
                json!({
                    ".git": {},
                    "dir": {
                        "foo.toml": "a = 1\nb = 2\n",
                    }
                }),
            )
            .await;

        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();

        let mut cx = VisualTestContext::from_window(*window.deref(), cx);
        project
            .update(&mut cx, |project, cx| {
                project.find_or_create_worktree(EditorLspTestContext::root_path(), true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;

        let toml_file = cx.read(|cx| workspace.file_project_paths(cx)[0].clone());
        let toml_item = workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_path(toml_file, None, true, window, cx)
            })
            .await
            .expect("Could not open test file");

        let editor = cx.update(|_, cx| {
            toml_item
                .act_as::<Editor>(cx)
                .expect("Opened test file wasn't an editor")
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            let nav_history = workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .nav_history_for_item(&cx.entity());
            editor.set_nav_history(Some(nav_history));
            window.focus(&editor.focus_handle(cx), cx)
        });

        let toml_server_1 = toml_server_1.next().await.unwrap();
        let toml_server_2 = toml_server_2.next().await.unwrap();

        let full_counter_toml_1 = Arc::new(AtomicUsize::new(0));
        let full_counter_toml_1_clone = full_counter_toml_1.clone();
        let full_counter_toml_2 = Arc::new(AtomicUsize::new(0));
        let full_counter_toml_2_clone = full_counter_toml_2.clone();

        let mut toml_full_1_request = toml_server_1
            .set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(move |_, _| {
                full_counter_toml_1_clone.fetch_add(1, atomic::Ordering::Release);
                async move {
                    Ok(Some(lsp::SemanticTokensResult::Tokens(
                        lsp::SemanticTokens {
                            // highlight 'a' as a property
                            data: vec![
                                0, // delta_line
                                0, // delta_start
                                1, // length
                                0, // token_type
                                0, // token_modifiers_bitset
                            ],
                            result_id: Some("a".into()),
                        },
                    )))
                }
            });

        let mut toml_full_2_request = toml_server_2
            .set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(move |_, _| {
                full_counter_toml_2_clone.fetch_add(1, atomic::Ordering::Release);
                async move {
                    Ok(Some(lsp::SemanticTokensResult::Tokens(
                        lsp::SemanticTokens {
                            // highlight '3' as a literal
                            data: vec![
                                0, // delta_line
                                4, // delta_start
                                1, // length
                                0, // token_type
                                0, // token_modifiers_bitset
                            ],
                            result_id: Some("a".into()),
                        },
                    )))
                }
            });

        // Trigger semantic tokens.
        editor.update_in(&mut cx, |editor, _, cx| {
            editor.edit([(MultiBufferOffset(0)..MultiBufferOffset(1), "b")], cx);
        });
        let res = join_all([toml_full_1_request.next(), toml_full_2_request.next()]).await;
        assert!(res[0].is_some(), "server 1 did not get a request");
        assert!(res[1].is_some(), "server 2 did not get a request");
        let task = editor.update_in(&mut cx, |e, _, _| {
            std::mem::replace(&mut e.update_semantic_tokens_task, Task::ready(()))
        });
        task.await;

        assert_eq!(
            extract_semantic_highlights(&editor, &cx),
            vec![
                MultiBufferOffset(0)..MultiBufferOffset(1),
                MultiBufferOffset(4)..MultiBufferOffset(5),
            ]
        );

        assert_eq!(full_counter_toml_1.load(atomic::Ordering::Acquire), 1);
        assert_eq!(full_counter_toml_2.load(atomic::Ordering::Acquire), 1);
    }

    #[gpui::test]
    async fn lsp_semantic_tokens_multibuffer_part(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |language_settings| {
            language_settings.languages.0.insert(
                "TOML".into(),
                LanguageSettingsContent {
                    semantic_tokens: Some(SemanticTokens::Full),
                    ..Default::default()
                },
            );
            language_settings.languages.0.insert(
                "Rust".into(),
                LanguageSettingsContent {
                    semantic_tokens: Some(SemanticTokens::Full),
                    ..Default::default()
                },
            );
        });

        let toml_language = Arc::new(Language::new(
            LanguageConfig {
                name: "TOML".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["toml".into()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        ));
        let rust_language = Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".into()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        ));

        let toml_legend = lsp::SemanticTokensLegend {
            token_types: vec!["property".into()],
            token_modifiers: vec![],
        };
        let rust_legend = lsp::SemanticTokensLegend {
            token_types: vec!["constant".into()],
            token_modifiers: vec![],
        };

        let app_state = cx.update(workspace::AppState::test);

        cx.update(|cx| {
            assets::Assets.load_test_fonts(cx);
            crate::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let mut toml_server = language_registry.register_fake_lsp(
            toml_language.name(),
            FakeLspAdapter {
                name: "toml",
                capabilities: lsp::ServerCapabilities {
                    semantic_tokens_provider: Some(
                        lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                            lsp::SemanticTokensOptions {
                                legend: toml_legend,
                                full: Some(lsp::SemanticTokensFullOptions::Delta { delta: None }),
                                ..Default::default()
                            },
                        ),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        language_registry.add(toml_language.clone());
        let mut rust_server = language_registry.register_fake_lsp(
            rust_language.name(),
            FakeLspAdapter {
                name: "rust",
                capabilities: lsp::ServerCapabilities {
                    semantic_tokens_provider: Some(
                        lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                            lsp::SemanticTokensOptions {
                                legend: rust_legend,
                                full: Some(lsp::SemanticTokensFullOptions::Delta { delta: None }),
                                ..Default::default()
                            },
                        ),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        language_registry.add(rust_language.clone());

        app_state
            .fs
            .as_fake()
            .insert_tree(
                EditorLspTestContext::root_path(),
                json!({
                    ".git": {},
                    "dir": {
                        "foo.toml": "a = 1\nb = 2\n",
                        "bar.rs": "const c: usize = 3;\n",
                    }
                }),
            )
            .await;

        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();

        let mut cx = VisualTestContext::from_window(*window.deref(), cx);
        project
            .update(&mut cx, |project, cx| {
                project.find_or_create_worktree(EditorLspTestContext::root_path(), true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;

        let toml_file = cx.read(|cx| workspace.file_project_paths(cx)[1].clone());
        let rust_file = cx.read(|cx| workspace.file_project_paths(cx)[0].clone());
        let (toml_item, rust_item) = workspace.update_in(&mut cx, |workspace, window, cx| {
            (
                workspace.open_path(toml_file, None, true, window, cx),
                workspace.open_path(rust_file, None, true, window, cx),
            )
        });
        let toml_item = toml_item.await.expect("Could not open test file");
        let rust_item = rust_item.await.expect("Could not open test file");

        let (toml_editor, rust_editor) = cx.update(|_, cx| {
            (
                toml_item
                    .act_as::<Editor>(cx)
                    .expect("Opened test file wasn't an editor"),
                rust_item
                    .act_as::<Editor>(cx)
                    .expect("Opened test file wasn't an editor"),
            )
        });
        let toml_buffer = cx.read(|cx| {
            toml_editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap()
        });
        let rust_buffer = cx.read(|cx| {
            rust_editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap()
        });
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.push_excerpts(
                toml_buffer.clone(),
                [ExcerptRange::new(Point::new(0, 0)..Point::new(2, 0))],
                cx,
            );
            multibuffer.push_excerpts(
                rust_buffer.clone(),
                [ExcerptRange::new(Point::new(0, 0)..Point::new(1, 0))],
                cx,
            );
            multibuffer
        });

        let editor = workspace.update_in(&mut cx, |_, window, cx| {
            cx.new(|cx| build_editor(multibuffer, window, cx))
        });
        editor.update_in(&mut cx, |editor, window, cx| {
            let nav_history = workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .nav_history_for_item(&cx.entity());
            editor.set_nav_history(Some(nav_history));
            window.focus(&editor.focus_handle(cx), cx)
        });

        let toml_server = toml_server.next().await.unwrap();
        let _rust_server = rust_server.next().await.unwrap();

        let full_counter_toml = Arc::new(AtomicUsize::new(0));
        let full_counter_toml_clone = full_counter_toml.clone();

        let mut toml_full_request = toml_server
            .set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(move |_, _| {
                full_counter_toml_clone.fetch_add(1, atomic::Ordering::Release);
                async move {
                    Ok(Some(lsp::SemanticTokensResult::Tokens(
                        lsp::SemanticTokens {
                            // highlight 'a' as a property
                            data: vec![
                                0, // delta_line
                                0, // delta_start
                                1, // length
                                0, // token_type
                                0, // token_modifiers_bitset
                            ],
                            result_id: Some("a".into()),
                        },
                    )))
                }
            });

        // Initial request.
        toml_full_request.next().await.unwrap();
        let task = editor.update_in(&mut cx, |e, _, _| {
            std::mem::replace(&mut e.update_semantic_tokens_task, Task::ready(()))
        });
        task.await;
        assert_eq!(full_counter_toml.load(atomic::Ordering::Acquire), 1);

        // Only edit the first part of the buffer, which is the TOML bit.
        editor.update_in(&mut cx, |editor, _, cx| {
            editor.edit([(MultiBufferOffset(0)..MultiBufferOffset(1), "b")], cx);
        });
        toml_full_request.next().await.unwrap();
        let task = editor.update_in(&mut cx, |e, _, _| {
            std::mem::replace(&mut e.update_semantic_tokens_task, Task::ready(()))
        });
        // This task will only complete once all servers have responded.
        task.await;

        assert_eq!(full_counter_toml.load(atomic::Ordering::Acquire), 2);
    }

    #[gpui::test]
    async fn lsp_semantic_tokens_multibuffer_shared(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |language_settings| {
            language_settings.languages.0.insert(
                "TOML".into(),
                LanguageSettingsContent {
                    semantic_tokens: Some(SemanticTokens::Full),
                    ..Default::default()
                },
            );
        });

        let toml_language = Arc::new(Language::new(
            LanguageConfig {
                name: "TOML".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["toml".into()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        ));

        let toml_legend = lsp::SemanticTokensLegend {
            token_types: vec!["property".into()],
            token_modifiers: vec![],
        };

        let app_state = cx.update(workspace::AppState::test);

        cx.update(|cx| {
            assets::Assets.load_test_fonts(cx);
            crate::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let mut toml_server = language_registry.register_fake_lsp(
            toml_language.name(),
            FakeLspAdapter {
                name: "toml",
                capabilities: lsp::ServerCapabilities {
                    semantic_tokens_provider: Some(
                        lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                            lsp::SemanticTokensOptions {
                                legend: toml_legend,
                                full: Some(lsp::SemanticTokensFullOptions::Delta { delta: None }),
                                ..Default::default()
                            },
                        ),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        language_registry.add(toml_language.clone());

        app_state
            .fs
            .as_fake()
            .insert_tree(
                EditorLspTestContext::root_path(),
                json!({
                    ".git": {},
                    "dir": {
                        "foo.toml": "a = 1\nb = 2\n",
                    }
                }),
            )
            .await;

        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();

        let mut cx = VisualTestContext::from_window(*window.deref(), cx);
        project
            .update(&mut cx, |project, cx| {
                project.find_or_create_worktree(EditorLspTestContext::root_path(), true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;

        let toml_file = cx.read(|cx| workspace.file_project_paths(cx)[0].clone());
        let toml_item = workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_path(toml_file, None, true, window, cx)
            })
            .await
            .expect("Could not open test file");

        let toml_editor = cx.update(|_, cx| {
            toml_item
                .act_as::<Editor>(cx)
                .expect("Opened test file wasn't an editor")
        });
        let toml_buffer = cx.read(|cx| {
            toml_editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap()
        });
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.push_excerpts(
                toml_buffer.clone(),
                [ExcerptRange::new(Point::new(0, 0)..Point::new(2, 0))],
                cx,
            );
            multibuffer.push_excerpts(
                toml_buffer.clone(),
                [ExcerptRange::new(Point::new(0, 0)..Point::new(2, 0))],
                cx,
            );
            multibuffer
        });

        let editor = workspace.update_in(&mut cx, |_, window, cx| {
            cx.new(|cx| build_editor_with_project(project, multibuffer, window, cx))
        });
        editor.update_in(&mut cx, |editor, window, cx| {
            let nav_history = workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .nav_history_for_item(&cx.entity());
            editor.set_nav_history(Some(nav_history));
            window.focus(&editor.focus_handle(cx), cx)
        });

        let toml_server = toml_server.next().await.unwrap();

        let full_counter_toml = Arc::new(AtomicUsize::new(0));
        let full_counter_toml_clone = full_counter_toml.clone();

        let mut toml_full_request = toml_server
            .set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(move |_, _| {
                full_counter_toml_clone.fetch_add(1, atomic::Ordering::Release);
                async move {
                    Ok(Some(lsp::SemanticTokensResult::Tokens(
                        lsp::SemanticTokens {
                            // highlight 'a' as a property
                            data: vec![
                                0, // delta_line
                                0, // delta_start
                                1, // length
                                0, // token_type
                                0, // token_modifiers_bitset
                            ],
                            result_id: Some("a".into()),
                        },
                    )))
                }
            });

        // Initial request.
        toml_full_request.next().await.unwrap();
        let task = editor.update_in(&mut cx, |e, _, _| {
            std::mem::replace(&mut e.update_semantic_tokens_task, Task::ready(()))
        });
        task.await;
        assert_eq!(full_counter_toml.load(atomic::Ordering::Acquire), 1);

        // Edit two parts of the multibuffer, which both map to the same buffer.
        //
        // Without debouncing, this grabs semantic tokens 4 times (twice for the
        // toml editor, and twice for the multibuffer).
        editor.update_in(&mut cx, |editor, _, cx| {
            editor.edit([(MultiBufferOffset(0)..MultiBufferOffset(1), "b")], cx);
            editor.edit([(MultiBufferOffset(12)..MultiBufferOffset(13), "c")], cx);
        });
        toml_full_request.next().await.unwrap();
        let task = editor.update_in(&mut cx, |e, _, _| {
            std::mem::replace(&mut e.update_semantic_tokens_task, Task::ready(()))
        });
        task.await;
        assert_eq!(
            extract_semantic_highlights(&editor, &cx),
            vec![MultiBufferOffset(0)..MultiBufferOffset(1)]
        );

        assert_eq!(full_counter_toml.load(atomic::Ordering::Acquire), 2);
    }

    fn extract_semantic_highlights(
        editor: &Entity<Editor>,
        cx: &TestAppContext,
    ) -> Vec<Range<MultiBufferOffset>> {
        editor.read_with(cx, |editor, cx| {
            let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
            editor
                .display_map
                .read(cx)
                .semantic_token_highlights
                .iter()
                .flat_map(|(_, v)| v.iter())
                .map(|highlights| highlights.range.to_offset(&multi_buffer_snapshot))
                .collect()
        })
    }

    #[gpui::test]
    async fn test_semantic_tokens_rules_changes_restyle_tokens(cx: &mut TestAppContext) {
        use gpui::{Hsla, Rgba, UpdateGlobal as _};
        use settings::{GlobalLspSettingsContent, SemanticTokenRule};

        init_test(cx, |_| {});

        update_test_language_settings(cx, |language_settings| {
            language_settings.languages.0.insert(
                "Rust".into(),
                LanguageSettingsContent {
                    semantic_tokens: Some(SemanticTokens::Full),
                    ..LanguageSettingsContent::default()
                },
            );
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                semantic_tokens_provider: Some(
                    lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(
                        lsp::SemanticTokensOptions {
                            legend: lsp::SemanticTokensLegend {
                                token_types: Vec::from(["function".into()]),
                                token_modifiers: Vec::new(),
                            },
                            full: Some(lsp::SemanticTokensFullOptions::Delta { delta: None }),
                            ..lsp::SemanticTokensOptions::default()
                        },
                    ),
                ),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut full_request = cx
            .set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(
                move |_, _, _| {
                    async move {
                        Ok(Some(lsp::SemanticTokensResult::Tokens(
                            lsp::SemanticTokens {
                                data: vec![
                                    0, // delta_line
                                    3, // delta_start
                                    4, // length
                                    0, // token_type (function)
                                    0, // token_modifiers_bitset
                                ],
                                result_id: None,
                            },
                        )))
                    }
                },
            );

        // Trigger initial semantic tokens fetch
        cx.set_state("ˇfn main() {}");
        full_request.next().await;
        cx.run_until_parked();

        // Verify initial highlights exist (with no custom color yet)
        let initial_ranges = extract_semantic_highlights(&cx.editor, &cx);
        assert_eq!(
            initial_ranges,
            vec![MultiBufferOffset(3)..MultiBufferOffset(7)],
            "Should have initial semantic token highlights"
        );
        let initial_styles = extract_semantic_highlight_styles(&cx.editor, &cx);
        assert_eq!(initial_styles.len(), 1, "Should have one highlight style");
        // Initial color should be None or theme default (not red or blue)
        let initial_color = initial_styles[0].color;

        // Set a custom foreground color for function tokens via settings.json
        let red_color = Rgba {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.global_lsp_settings = Some(GlobalLspSettingsContent {
                        semantic_token_rules: Some(SemanticTokenRules {
                            rules: Vec::from([SemanticTokenRule {
                                token_type: Some("function".to_string()),
                                foreground_color: Some(red_color),
                                ..SemanticTokenRule::default()
                            }]),
                        }),
                        ..GlobalLspSettingsContent::default()
                    });
                });
            });
        });

        // Trigger a refetch by making an edit (which forces semantic tokens update)
        cx.set_state("ˇfn main() { }");
        full_request.next().await;
        cx.run_until_parked();

        // Verify the highlights now have the custom red color
        let styles_after_settings_change = extract_semantic_highlight_styles(&cx.editor, &cx);
        assert_eq!(
            styles_after_settings_change.len(),
            1,
            "Should still have one highlight"
        );
        assert_eq!(
            styles_after_settings_change[0].color,
            Some(Hsla::from(red_color)),
            "Highlight should have the custom red color from settings.json"
        );
        assert_ne!(
            styles_after_settings_change[0].color, initial_color,
            "Color should have changed from initial"
        );

        // Now update via the semantic_token_rules.json file simulation
        // by directly calling set_user_semantic_token_rules
        let blue_color = Rgba {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        };
        let rules_json = serde_json::to_string(&SemanticTokenRules {
            rules: Vec::from([SemanticTokenRule {
                token_type: Some("function".to_string()),
                foreground_color: Some(blue_color),
                ..SemanticTokenRule::default()
            }]),
        })
        .unwrap();

        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.set_user_semantic_token_rules(&rules_json, cx);
        });

        // Trigger a refetch by making an edit
        cx.set_state("ˇfn main() {  }");
        full_request.next().await;
        cx.run_until_parked();

        // Verify the highlights now have the blue color from the config file
        // (which takes priority over settings.json)
        let styles_after_file_change = extract_semantic_highlight_styles(&cx.editor, &cx);
        assert_eq!(
            styles_after_file_change.len(),
            1,
            "Should still have one highlight"
        );
        assert_eq!(
            styles_after_file_change[0].color,
            Some(Hsla::from(blue_color)),
            "Highlight should have the blue color from semantic_token_rules.json (higher priority)"
        );
    }

    fn extract_semantic_highlight_styles(
        editor: &Entity<Editor>,
        cx: &TestAppContext,
    ) -> Vec<HighlightStyle> {
        editor.read_with(cx, |editor, cx| {
            editor
                .display_map
                .read(cx)
                .semantic_token_highlights
                .iter()
                .flat_map(|(_, v)| v.iter())
                .map(|highlights| highlights.style)
                .collect()
        })
    }
}
