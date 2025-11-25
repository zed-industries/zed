use std::{collections::hash_map, time::Duration};

use collections::HashMap;
use futures::future::join_all;
use gpui::{
    App, Context, FontStyle, FontWeight, HighlightStyle, StrikethroughStyle, Task, UnderlineStyle,
};
use itertools::Itertools as _;
use language::language_settings::language_settings;
use project::project_settings::ProjectSettings;
use settings::{
    SemanticTokenColorOverride, SemanticTokenFontStyle, SemanticTokenFontWeight,
    SemanticTokenRules, Settings as _,
};
use text::{Bias, BufferId};
use theme::SyntaxTheme;
use ui::ActiveTheme as _;

use crate::{Editor, display_map::SemanticTokenHighlight};

impl Editor {
    pub fn toggle_semantic_tokens(&mut self, cx: &mut Context<Self>) {
        self.semantic_tokens_enabled = !self.semantic_tokens_enabled;
        self.update_semantic_tokens(None, false, cx);
    }

    pub fn semantic_tokens_available(&self, cx: &App) -> bool {
        self.buffer.read(cx).all_buffers().iter().any(|buffer| {
            let buffer = buffer.read(cx);
            let file = buffer.file();
            language_settings(buffer.language().map(|l| l.name()), file, cx).semantic_tokens
        })
    }

    pub fn semantic_tokens_enabled(&self) -> bool {
        self.semantic_tokens_enabled
    }

    pub(crate) fn update_semantic_tokens(
        &mut self,
        buffer_id: Option<BufferId>,
        refresh: bool,
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

        if refresh {
            self.semantic_tokens_fetched_for_buffers.clear();
        }

        let Some((sema, project)) = self.semantics_provider.clone().zip(self.project.clone())
        else {
            return;
        };

        let buffers_to_query = self
            .visible_excerpts(cx)
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
                                let task = sema.semantic_tokens(buffer, refresh, cx);
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
            if all_semantic_tokens.is_empty() {
                return;
            }

            editor.update(cx, |editor, cx| {
                let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                let all_excerpts = editor.buffer().read(cx).excerpt_ids();

                for (buffer_id, query_version, tokens) in all_semantic_tokens {
                    let Some(buffer) = editor.buffer().read(cx).buffer(buffer_id) else {
                        continue;
                    };

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
                        let stylizers = tokens.servers.keys().filter_map(|&server_id| {
                            let legend = match lsp_store.lsp_server_capabilities.get(&server_id)?.semantic_tokens_provider.as_ref()? {
                                lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(opts) => &opts.legend,
                                lsp::SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(opts) => &opts.semantic_tokens_options.legend,
                            };
                            Some((server_id, legend))
                        }).map(|(server_id, legend)| (server_id, SemanticTokenStylizer::new(legend, cx)))
                        .collect::<HashMap<_, _>>();

                        let buffer_snapshot = buffer.read(cx).snapshot();
                        let token_highlights = tokens
                            .all_tokens()
                            .filter_map(|(server_id, token)| {
                                let stylizer = stylizers.get(&server_id)?;
                                let start = text::Unclipped(text::PointUtf16::new(token.line, token.start));
                                let (start_offset, end_offset) = point_offset_to_offsets(
                                    buffer.read(cx).clip_point_utf16(start, Bias::Left),
                                    text::OffsetUtf16(token.length as usize),
                                    &buffer.read(cx),
                                );
                                let buffer_range = buffer_snapshot.anchor_before(start_offset)..buffer_snapshot.anchor_after(end_offset);
                                let multi_buffer_start = all_excerpts.iter().find_map(|&excerpt_id| multi_buffer_snapshot.anchor_in_excerpt(excerpt_id, buffer_range.start))?;
                                let multi_buffer_end = all_excerpts.iter().find_map(|&excerpt_id| multi_buffer_snapshot.anchor_in_excerpt(excerpt_id, buffer_range.end))?;

                                Some(SemanticTokenHighlight {
                                    range: multi_buffer_start..multi_buffer_end,
                                    style: stylizer.convert(
                                        cx.theme().syntax(),
                                        token.token_type,
                                        token.token_modifiers,
                                    )?,
                                })
                            });

                        display_map.invalidate_semantic_highlights(buffer_id);

                        for highlight in token_highlights {
                            let i = display_map
                                .semantic_token_highlights.binary_search_by(|probe| probe.range.start.cmp(&&highlight.range.start, &multi_buffer_snapshot)).unwrap_or_else(|i| i);
                            display_map
                                .semantic_token_highlights
                                .insert(i, highlight);
                        }
                    });
                }

                cx.notify();
            }).ok();
        });
    }
}

// TODO kb this looks odd
fn point_offset_to_offsets(
    point: text::PointUtf16,
    length: text::OffsetUtf16,
    buffer: &text::Buffer,
) -> (usize, usize) {
    let start = buffer.as_rope().point_utf16_to_offset(point);
    let start_offset = buffer.as_rope().offset_to_offset_utf16(start);
    let end_offset = start_offset + length;
    let end = buffer.as_rope().offset_utf16_to_offset(end_offset);

    (start, end)
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
