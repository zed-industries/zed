use std::{collections::hash_map, sync::Arc, time::Duration};

use collections::{HashMap, HashSet};
use futures::future::join_all;
use gpui::{
    App, Context, FontStyle, FontWeight, HighlightStyle, StrikethroughStyle, Task, UnderlineStyle,
};
use itertools::Itertools as _;
use language::language_settings::language_settings;
use project::{
    lsp_store::{BufferSemanticToken, RefreshForServer},
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
                    && settings.semantic_tokens;
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
                        Ok(tokens) => tokens,
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
                        for (&server_id, server_tokens) in &tokens.tokens {
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
                                server_tokens,
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
