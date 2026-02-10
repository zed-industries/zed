use std::{collections::hash_map, sync::Arc, time::Duration};

use collections::{HashMap, HashSet};
use futures::future::join_all;
use gpui::{
    App, Context, FontStyle, FontWeight, HighlightStyle, StrikethroughStyle, Task, UnderlineStyle,
};
use itertools::Itertools as _;
use language::language_settings::language_settings;
use project::{
    lsp_store::{
        BufferSemanticToken, BufferSemanticTokens, RefreshForServer, SemanticTokenStylizer,
        TokenType,
    },
    project_settings::ProjectSettings,
};
use settings::{
    SemanticTokenColorOverride, SemanticTokenFontStyle, SemanticTokenFontWeight,
    SemanticTokenRules, Settings as _,
};
use text::BufferId;
use theme::SyntaxTheme;
use ui::ActiveTheme as _;

use crate::{
    Editor,
    actions::ToggleSemanticHighlights,
    display_map::{HighlightStyleInterner, SemanticTokenHighlight},
};

pub(super) struct SemanticTokenState {
    rules: SemanticTokenRules,
    enabled: bool,
    update_task: Task<()>,
    fetched_for_buffers: HashMap<BufferId, clock::Global>,
}

impl SemanticTokenState {
    pub(super) fn new(cx: &App, enabled: bool) -> Self {
        Self {
            rules: ProjectSettings::get_global(cx)
                .global_lsp_settings
                .semantic_token_rules
                .clone(),
            enabled,
            update_task: Task::ready(()),
            fetched_for_buffers: HashMap::default(),
        }
    }

    pub(super) fn enabled(&self) -> bool {
        self.enabled
    }

    pub(super) fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }

    #[cfg(test)]
    pub(super) fn take_update_task(&mut self) -> Task<()> {
        std::mem::replace(&mut self.update_task, Task::ready(()))
    }

    pub(super) fn invalidate_buffer(&mut self, buffer_id: &BufferId) {
        self.fetched_for_buffers.remove(buffer_id);
    }

    pub(super) fn update_rules(&mut self, new_rules: SemanticTokenRules) -> bool {
        if new_rules != self.rules {
            self.rules = new_rules;
            true
        } else {
            false
        }
    }
}

impl Editor {
    pub fn supports_semantic_tokens(&self, cx: &mut App) -> bool {
        let Some(provider) = self.semantics_provider.as_ref() else {
            return false;
        };

        let mut supports = false;
        self.buffer().update(cx, |this, cx| {
            this.for_each_buffer(|buffer| {
                supports |= provider.supports_semantic_tokens(buffer, cx);
            });
        });

        supports
    }

    pub fn semantic_highlights_enabled(&self) -> bool {
        self.semantic_token_state.enabled()
    }

    pub fn toggle_semantic_highlights(
        &mut self,
        _: &ToggleSemanticHighlights,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        self.semantic_token_state.toggle_enabled();
        self.invalidate_semantic_tokens(None);
        self.refresh_semantic_tokens(None, None, cx);
    }

    pub(super) fn invalidate_semantic_tokens(&mut self, for_buffer: Option<BufferId>) {
        match for_buffer {
            Some(for_buffer) => self.semantic_token_state.invalidate_buffer(&for_buffer),
            None => self.semantic_token_state.fetched_for_buffers.clear(),
        }
    }

    pub(super) fn refresh_semantic_tokens(
        &mut self,
        buffer_id: Option<BufferId>,
        for_server: Option<RefreshForServer>,
        cx: &mut Context<Self>,
    ) {
        if !self.mode().is_full() || !self.semantic_token_state.enabled() {
            self.invalidate_semantic_tokens(None);
            self.display_map.update(cx, |display_map, _| {
                display_map.semantic_token_highlights.clear();
            });
            self.semantic_token_state.update_task = Task::ready(());
            cx.notify();
            return;
        }

        let mut invalidate_semantic_highlights_for_buffers = HashSet::default();
        if for_server.is_some() {
            invalidate_semantic_highlights_for_buffers.extend(
                self.semantic_token_state
                    .fetched_for_buffers
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
                if self.registered_buffers.contains_key(&editor_buffer_id)
                    && language_settings(
                        editor_buffer.read(cx).language().map(|l| l.name()),
                        editor_buffer.read(cx).file(),
                        cx,
                    )
                    .semantic_tokens
                    .enabled()
                {
                    Some((editor_buffer_id, editor_buffer))
                } else {
                    None
                }
            })
            .unique_by(|(buffer_id, _)| *buffer_id)
            .collect::<Vec<_>>();

        self.semantic_token_state.update_task = cx.spawn(async move |editor, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(50))
                .await;
            let Some(all_semantic_tokens_task) = editor
                .update(cx, |editor, cx| {
                    buffers_to_query
                        .into_iter()
                        .filter_map(|(buffer_id, buffer)| {
                            let known_version =
                                editor.semantic_token_state.fetched_for_buffers.get(&buffer_id);
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
                    for buffer_id in invalidate_semantic_highlights_for_buffers {
                        display_map.invalidate_semantic_highlights(buffer_id);
                        editor.semantic_token_state.invalidate_buffer(&buffer_id);
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
                            editor.display_map.update(cx, |display_map, _| {
                                display_map.invalidate_semantic_highlights(buffer_id);
                            });
                            continue;
                        },
                        Err(e) => {
                            log::error!("Failed to fetch semantic tokens for buffer {buffer_id:?}: {e:#}");
                            continue;
                        },
                    };

                    match editor.semantic_token_state.fetched_for_buffers.entry(buffer_id) {
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

                    let language_name = editor
                        .buffer()
                        .read(cx)
                        .buffer(buffer_id)
                        .and_then(|buf| buf.read(cx).language().map(|l| l.name()));

                    editor.display_map.update(cx, |display_map, cx| {
                        project.read(cx).lsp_store().update(cx, |lsp_store, cx| {
                            let mut token_highlights = Vec::new();
                            let mut interner = HighlightStyleInterner::default();
                            for (server_id, server_tokens) in tokens {
                                let Some(stylizer) = lsp_store.get_or_create_token_stylizer(
                                    server_id,
                                    language_name.as_ref(),
                                    cx,
                                )
                                else {
                                    continue;
                                };
                                token_highlights.extend(buffer_into_editor_highlights(
                                    &server_tokens,
                                    stylizer,
                                    &all_excerpts,
                                    &multi_buffer_snapshot,
                                    &mut interner,
                                    cx,
                                ));
                            }

                            token_highlights.sort_by(|a, b| {
                                a.range.start.cmp(&b.range.start, &multi_buffer_snapshot)
                            });
                            display_map
                                .semantic_token_highlights
                                .insert(buffer_id, (Arc::from(token_highlights), Arc::new(interner)));
                        });
                    });
                }

                cx.notify();
            }).ok();
        });
    }
}

fn buffer_into_editor_highlights<'a, 'b>(
    buffer_tokens: &'a [BufferSemanticToken],
    stylizer: &'a SemanticTokenStylizer,
    all_excerpts: &'a [multi_buffer::ExcerptId],
    multi_buffer_snapshot: &'a multi_buffer::MultiBufferSnapshot,
    interner: &'b mut HighlightStyleInterner,
    cx: &'a App,
) -> impl Iterator<Item = SemanticTokenHighlight> + use<'a, 'b> {
    buffer_tokens.iter().filter_map(|token| {
        let multi_buffer_start = all_excerpts.iter().find_map(|&excerpt_id| {
            multi_buffer_snapshot.anchor_in_excerpt(excerpt_id, token.range.start)
        })?;
        let multi_buffer_end = all_excerpts.iter().find_map(|&excerpt_id| {
            multi_buffer_snapshot.anchor_in_excerpt(excerpt_id, token.range.end)
        })?;

        let style = convert_token(
            stylizer,
            cx.theme().syntax(),
            token.token_type,
            token.token_modifiers,
        )?;
        let style = interner.intern(style);
        Some(SemanticTokenHighlight {
            range: multi_buffer_start..multi_buffer_end,
            style,
            token_type: token.token_type,
            token_modifiers: token.token_modifiers,
            server_id: stylizer.server_id(),
        })
    })
}

fn convert_token(
    stylizer: &SemanticTokenStylizer,
    theme: &SyntaxTheme,
    token_type: TokenType,
    modifiers: u32,
) -> Option<HighlightStyle> {
    let rules = stylizer.rules_for_token(token_type)?;
    let matching = rules.iter().filter(|rule| {
        rule.token_modifiers
            .iter()
            .all(|m| stylizer.has_modifier(modifiers, m))
    });

    let mut highlight = HighlightStyle::default();
    let mut empty = true;

    for rule in matching {
        empty = false;

        let style = rule.style.iter().find_map(|style| theme.get_opt(style));

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

#[cfg(test)]
mod tests {
    use std::{
        ops::{Deref as _, Range},
        sync::atomic::{self, AtomicUsize},
    };

    use futures::StreamExt as _;
    use gpui::{
        AppContext as _, Entity, Focusable as _, HighlightStyle, TestAppContext, VisualTestContext,
    };
    use language::{Language, LanguageConfig, LanguageMatcher};
    use languages::FakeLspAdapter;
    use multi_buffer::{
        AnchorRangeExt, ExcerptRange, ExpandExcerptDirection, MultiBuffer, MultiBufferOffset,
    };
    use project::Project;
    use rope::Point;
    use serde_json::json;
    use settings::{LanguageSettingsContent, SemanticTokenRules, SemanticTokens, SettingsStore};
    use workspace::{Workspace, WorkspaceHandle as _};

    use crate::{
        Capability,
        editor_tests::{init_test, update_test_language_settings},
        test::{build_editor_with_project, editor_lsp_test_context::EditorLspTestContext},
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
                                token_modifiers: Vec::new(),
                            },
                            full: Some(lsp::SemanticTokensFullOptions::Delta { delta: Some(true) }),
                            ..lsp::SemanticTokensOptions::default()
                        },
                    ),
                ),
                ..lsp::ServerCapabilities::default()
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

        let task = cx.update_editor(|e, _, _| e.semantic_token_state.take_update_task());
        task.await;

        cx.set_state("ˇfn main() { a }");
        assert!(full_request.next().await.is_some());

        let task = cx.update_editor(|e, _, _| e.semantic_token_state.take_update_task());
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
                                token_modifiers: Vec::new(),
                            },
                            full: Some(lsp::SemanticTokensFullOptions::Delta { delta: Some(true) }),
                            ..lsp::SemanticTokensOptions::default()
                        },
                    ),
                ),
                ..lsp::ServerCapabilities::default()
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
                                edits: Vec::new(),
                                result_id: Some("b".into()),
                            },
                        )))
                    }
                },
            );

        // Initial request, for the empty buffer.
        cx.set_state("ˇfn main() {}");
        assert!(full_request.next().await.is_some());
        let task = cx.update_editor(|e, _, _| e.semantic_token_state.take_update_task());
        task.await;

        cx.set_state("ˇfn main() { a }");
        assert!(delta_request.next().await.is_some());
        let task = cx.update_editor(|e, _, _| e.semantic_token_state.take_update_task());
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
            token_modifiers: Vec::new(),
        };
        let toml_legend_2 = lsp::SemanticTokensLegend {
            token_types: vec!["number".into()],
            token_modifiers: Vec::new(),
        };

        let app_state = cx.update(workspace::AppState::test);

        cx.update(|cx| {
            assets::Assets.load_test_fonts(cx);
            crate::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());

        let full_counter_toml_1 = Arc::new(AtomicUsize::new(0));
        let full_counter_toml_1_clone = full_counter_toml_1.clone();
        let full_counter_toml_2 = Arc::new(AtomicUsize::new(0));
        let full_counter_toml_2_clone = full_counter_toml_2.clone();

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
                                ..lsp::SemanticTokensOptions::default()
                            },
                        ),
                    ),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new({
                    let full_counter_toml_1_clone = full_counter_toml_1_clone.clone();
                    move |fake_server| {
                        let full_counter = full_counter_toml_1_clone.clone();
                        fake_server
                            .set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(
                                move |_, _| {
                                    full_counter.fetch_add(1, atomic::Ordering::Release);
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
                                },
                            );
                    }
                })),
                ..FakeLspAdapter::default()
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
                                ..lsp::SemanticTokensOptions::default()
                            },
                        ),
                    ),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new({
                    let full_counter_toml_2_clone = full_counter_toml_2_clone.clone();
                    move |fake_server| {
                        let full_counter = full_counter_toml_2_clone.clone();
                        fake_server
                            .set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(
                                move |_, _| {
                                    full_counter.fetch_add(1, atomic::Ordering::Release);
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
                                },
                            );
                    }
                })),
                ..FakeLspAdapter::default()
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

        let _toml_server_1 = toml_server_1.next().await.unwrap();
        let _toml_server_2 = toml_server_2.next().await.unwrap();

        // Trigger semantic tokens.
        editor.update_in(&mut cx, |editor, _, cx| {
            editor.edit([(MultiBufferOffset(0)..MultiBufferOffset(1), "b")], cx);
        });
        cx.executor().advance_clock(Duration::from_millis(200));
        let task = editor.update_in(&mut cx, |e, _, _| e.semantic_token_state.take_update_task());
        cx.run_until_parked();
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
            token_modifiers: Vec::new(),
        };
        let rust_legend = lsp::SemanticTokensLegend {
            token_types: vec!["constant".into()],
            token_modifiers: Vec::new(),
        };

        let app_state = cx.update(workspace::AppState::test);

        cx.update(|cx| {
            assets::Assets.load_test_fonts(cx);
            crate::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let full_counter_toml = Arc::new(AtomicUsize::new(0));
        let full_counter_toml_clone = full_counter_toml.clone();

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
                                ..lsp::SemanticTokensOptions::default()
                            },
                        ),
                    ),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new({
                    let full_counter_toml_clone = full_counter_toml_clone.clone();
                    move |fake_server| {
                        let full_counter = full_counter_toml_clone.clone();
                        fake_server
                            .set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(
                                move |_, _| {
                                    full_counter.fetch_add(1, atomic::Ordering::Release);
                                    async move {
                                        Ok(Some(lsp::SemanticTokensResult::Tokens(
                                            lsp::SemanticTokens {
                                                // highlight 'a', 'b', 'c' as properties on lines 0, 1, 2
                                                data: vec![
                                                    0, // delta_line (line 0)
                                                    0, // delta_start
                                                    1, // length
                                                    0, // token_type
                                                    0, // token_modifiers_bitset
                                                    1, // delta_line (line 1)
                                                    0, // delta_start
                                                    1, // length
                                                    0, // token_type
                                                    0, // token_modifiers_bitset
                                                    1, // delta_line (line 2)
                                                    0, // delta_start
                                                    1, // length
                                                    0, // token_type
                                                    0, // token_modifiers_bitset
                                                ],
                                                result_id: Some("a".into()),
                                            },
                                        )))
                                    }
                                },
                            );
                    }
                })),
                ..FakeLspAdapter::default()
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
                                ..lsp::SemanticTokensOptions::default()
                            },
                        ),
                    ),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
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
                        "foo.toml": "a = 1\nb = 2\nc = 3\n",
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
                [ExcerptRange::new(Point::new(0, 0)..Point::new(1, 0))],
                cx,
            );
            multibuffer.push_excerpts(
                rust_buffer.clone(),
                [ExcerptRange::new(Point::new(0, 0)..Point::new(1, 0))],
                cx,
            );
            multibuffer
        });

        let editor = workspace.update_in(&mut cx, |workspace, window, cx| {
            let editor = cx.new(|cx| build_editor_with_project(project, multibuffer, window, cx));
            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
            editor
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

        let _toml_server = toml_server.next().await.unwrap();
        let _rust_server = rust_server.next().await.unwrap();

        // Initial request.
        cx.executor().advance_clock(Duration::from_millis(200));
        let task = editor.update_in(&mut cx, |e, _, _| e.semantic_token_state.take_update_task());
        cx.run_until_parked();
        task.await;
        assert_eq!(full_counter_toml.load(atomic::Ordering::Acquire), 1);
        cx.run_until_parked();

        // Initially, excerpt only covers line 0, so only the 'a' token should be highlighted.
        // The excerpt content is "a = 1\n" (6 chars), so 'a' is at offset 0.
        assert_eq!(
            extract_semantic_highlights(&editor, &cx),
            vec![MultiBufferOffset(0)..MultiBufferOffset(1)]
        );

        // Get the excerpt id for the TOML excerpt and expand it down by 2 lines.
        let toml_excerpt_id =
            editor.read_with(&cx, |editor, cx| editor.buffer().read(cx).excerpt_ids()[0]);
        editor.update_in(&mut cx, |editor, _, cx| {
            editor.buffer().update(cx, |buffer, cx| {
                buffer.expand_excerpts([toml_excerpt_id], 2, ExpandExcerptDirection::Down, cx);
            });
        });

        // Wait for semantic tokens to be re-fetched after expansion.
        cx.executor().advance_clock(Duration::from_millis(200));
        let task = editor.update_in(&mut cx, |e, _, _| e.semantic_token_state.take_update_task());
        cx.run_until_parked();
        task.await;

        // After expansion, the excerpt covers lines 0-2, so 'a', 'b', 'c' should all be highlighted.
        // Content is now "a = 1\nb = 2\nc = 3\n" (18 chars).
        // 'a' at offset 0, 'b' at offset 6, 'c' at offset 12.
        assert_eq!(
            extract_semantic_highlights(&editor, &cx),
            vec![
                MultiBufferOffset(0)..MultiBufferOffset(1),
                MultiBufferOffset(6)..MultiBufferOffset(7),
                MultiBufferOffset(12)..MultiBufferOffset(13),
            ]
        );
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
            token_modifiers: Vec::new(),
        };

        let app_state = cx.update(workspace::AppState::test);

        cx.update(|cx| {
            assets::Assets.load_test_fonts(cx);
            crate::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let full_counter_toml = Arc::new(AtomicUsize::new(0));
        let full_counter_toml_clone = full_counter_toml.clone();

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
                                ..lsp::SemanticTokensOptions::default()
                            },
                        ),
                    ),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new({
                    let full_counter_toml_clone = full_counter_toml_clone.clone();
                    move |fake_server| {
                        let full_counter = full_counter_toml_clone.clone();
                        fake_server
                            .set_request_handler::<lsp::request::SemanticTokensFullRequest, _, _>(
                                move |_, _| {
                                    full_counter.fetch_add(1, atomic::Ordering::Release);
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
                                },
                            );
                    }
                })),
                ..FakeLspAdapter::default()
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

        let _toml_server = toml_server.next().await.unwrap();

        // Initial request.
        cx.executor().advance_clock(Duration::from_millis(200));
        let task = editor.update_in(&mut cx, |e, _, _| e.semantic_token_state.take_update_task());
        cx.run_until_parked();
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
        cx.executor().advance_clock(Duration::from_millis(200));
        let task = editor.update_in(&mut cx, |e, _, _| e.semantic_token_state.take_update_task());
        cx.run_until_parked();
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
                .flat_map(|(_, (v, _))| v.iter())
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
    }

    #[gpui::test]
    async fn test_theme_override_changes_restyle_semantic_tokens(cx: &mut TestAppContext) {
        use collections::IndexMap;
        use gpui::{Hsla, Rgba, UpdateGlobal as _};
        use theme::{HighlightStyleContent, ThemeStyleContent};

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
                move |_, _, _| async move {
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
                },
            );

        cx.set_state("ˇfn main() {}");
        full_request.next().await;
        cx.run_until_parked();

        let initial_styles = extract_semantic_highlight_styles(&cx.editor, &cx);
        assert_eq!(initial_styles.len(), 1, "Should have one highlight style");
        let initial_color = initial_styles[0].color;

        // Changing experimental_theme_overrides triggers GlobalTheme reload,
        // which fires theme_changed → refresh_semantic_token_highlights.
        let red_color: Hsla = Rgba {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
        .into();
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.theme.experimental_theme_overrides = Some(ThemeStyleContent {
                        syntax: IndexMap::from_iter([(
                            "function".to_string(),
                            HighlightStyleContent {
                                color: Some("#ff0000".to_string()),
                                background_color: None,
                                font_style: None,
                                font_weight: None,
                            },
                        )]),
                        ..ThemeStyleContent::default()
                    });
                });
            });
        });

        cx.executor().advance_clock(Duration::from_millis(200));
        cx.run_until_parked();

        let styles_after_override = extract_semantic_highlight_styles(&cx.editor, &cx);
        assert_eq!(styles_after_override.len(), 1);
        assert_eq!(
            styles_after_override[0].color,
            Some(red_color),
            "Highlight should have red color from theme override"
        );
        assert_ne!(
            styles_after_override[0].color, initial_color,
            "Color should have changed from initial"
        );

        // Changing the override to a different color also restyles.
        let blue_color: Hsla = Rgba {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        }
        .into();
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.theme.experimental_theme_overrides = Some(ThemeStyleContent {
                        syntax: IndexMap::from_iter([(
                            "function".to_string(),
                            HighlightStyleContent {
                                color: Some("#0000ff".to_string()),
                                background_color: None,
                                font_style: None,
                                font_weight: None,
                            },
                        )]),
                        ..ThemeStyleContent::default()
                    });
                });
            });
        });

        cx.executor().advance_clock(Duration::from_millis(200));
        cx.run_until_parked();

        let styles_after_second_override = extract_semantic_highlight_styles(&cx.editor, &cx);
        assert_eq!(styles_after_second_override.len(), 1);
        assert_eq!(
            styles_after_second_override[0].color,
            Some(blue_color),
            "Highlight should have blue color from updated theme override"
        );

        // Removing overrides reverts to the original theme color.
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.theme.experimental_theme_overrides = None;
                });
            });
        });

        cx.executor().advance_clock(Duration::from_millis(200));
        cx.run_until_parked();

        let styles_after_clear = extract_semantic_highlight_styles(&cx.editor, &cx);
        assert_eq!(styles_after_clear.len(), 1);
        assert_eq!(
            styles_after_clear[0].color, initial_color,
            "Highlight should revert to initial color after clearing overrides"
        );
    }

    #[gpui::test]
    async fn test_per_theme_overrides_restyle_semantic_tokens(cx: &mut TestAppContext) {
        use collections::IndexMap;
        use gpui::{Hsla, Rgba, UpdateGlobal as _};
        use theme::{HighlightStyleContent, ThemeStyleContent};
        use ui::ActiveTheme as _;

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
                move |_, _, _| async move {
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
                },
            );

        cx.set_state("ˇfn main() {}");
        full_request.next().await;
        cx.run_until_parked();

        let initial_styles = extract_semantic_highlight_styles(&cx.editor, &cx);
        assert_eq!(initial_styles.len(), 1, "Should have one highlight style");
        let initial_color = initial_styles[0].color;

        // Per-theme overrides (theme_overrides keyed by theme name) also go through
        // GlobalTheme reload → theme_changed → refresh_semantic_token_highlights.
        let theme_name = cx.update(|_, cx| cx.theme().name.to_string());
        let green_color: Hsla = Rgba {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        }
        .into();
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.theme.theme_overrides = collections::HashMap::from_iter([(
                        theme_name.clone(),
                        ThemeStyleContent {
                            syntax: IndexMap::from_iter([(
                                "function".to_string(),
                                HighlightStyleContent {
                                    color: Some("#00ff00".to_string()),
                                    background_color: None,
                                    font_style: None,
                                    font_weight: None,
                                },
                            )]),
                            ..ThemeStyleContent::default()
                        },
                    )]);
                });
            });
        });

        cx.executor().advance_clock(Duration::from_millis(200));
        cx.run_until_parked();

        let styles_after_override = extract_semantic_highlight_styles(&cx.editor, &cx);
        assert_eq!(styles_after_override.len(), 1);
        assert_eq!(
            styles_after_override[0].color,
            Some(green_color),
            "Highlight should have green color from per-theme override"
        );
        assert_ne!(
            styles_after_override[0].color, initial_color,
            "Color should have changed from initial"
        );
    }

    #[gpui::test]
    async fn test_stopping_language_server_clears_semantic_tokens(cx: &mut TestAppContext) {
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
                                token_types: vec!["function".into()],
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
                move |_, _, _| async move {
                    Ok(Some(lsp::SemanticTokensResult::Tokens(
                        lsp::SemanticTokens {
                            data: vec![
                                0, // delta_line
                                3, // delta_start
                                4, // length
                                0, // token_type
                                0, // token_modifiers_bitset
                            ],
                            result_id: None,
                        },
                    )))
                },
            );

        cx.set_state("ˇfn main() {}");
        assert!(full_request.next().await.is_some());
        cx.run_until_parked();

        assert_eq!(
            extract_semantic_highlights(&cx.editor, &cx),
            vec![MultiBufferOffset(3)..MultiBufferOffset(7)],
            "Semantic tokens should be present before stopping the server"
        );

        cx.update_editor(|editor, _, cx| {
            let buffers = editor.buffer.read(cx).all_buffers().into_iter().collect();
            editor.project.as_ref().unwrap().update(cx, |project, cx| {
                project.stop_language_servers_for_buffers(buffers, HashSet::default(), cx);
            })
        });
        cx.executor().advance_clock(Duration::from_millis(200));
        cx.run_until_parked();

        assert_eq!(
            extract_semantic_highlights(&cx.editor, &cx),
            Vec::new(),
            "Semantic tokens should be cleared after stopping the server"
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
                .flat_map(|(_, (v, interner))| {
                    v.iter().map(|highlights| interner[highlights.style])
                })
                .collect()
        })
    }
}
