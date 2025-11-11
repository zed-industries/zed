use crate::Editor;
use collections::HashMap;
use gpui::{Context, HighlightStyle};
use language::language_settings;
use ui::{ActiveTheme, utils::ensure_minimum_contrast};

struct RainbowBracketHighlight;

impl Editor {
    pub(crate) fn colorize_brackets(&mut self, invalidate: bool, cx: &mut Context<Editor>) {
        if !self.mode.is_full() {
            return;
        }

        if invalidate {
            self.fetched_tree_sitter_chunks.clear();
        }

        let accents_count = cx.theme().accents().0.len();
        let multi_buffer_snapshot = self.buffer().read(cx).snapshot(cx);
        let bracket_matches_by_accent = self.visible_excerpts(cx).into_iter().fold(
            HashMap::default(),
            |mut acc, (excerpt_id, (buffer, buffer_version, buffer_range))| {
                let buffer_snapshot = buffer.read(cx).snapshot();
                if language_settings::language_settings(
                    buffer_snapshot.language().map(|language| language.name()),
                    buffer_snapshot.file(),
                    cx,
                )
                .colorize_brackets
                {
                    let fetched_chunks = self
                        .fetched_tree_sitter_chunks
                        .entry(excerpt_id)
                        .or_default();

                    let brackets_by_accent = buffer_snapshot
                        .fetch_bracket_ranges(
                            buffer_range.start..buffer_range.end,
                            Some((&buffer_version, fetched_chunks)),
                        )
                        .into_iter()
                        .flat_map(|(chunk_range, pairs)| {
                            if fetched_chunks.insert(chunk_range) {
                                pairs
                            } else {
                                Vec::new()
                            }
                        })
                        .filter_map(|pair| {
                            let color_index = pair.color_index?;
                            let buffer_open_range = buffer_snapshot
                                .anchor_before(pair.open_range.start)
                                ..buffer_snapshot.anchor_after(pair.open_range.end);
                            let multi_buffer_open_range = multi_buffer_snapshot
                                .anchor_in_excerpt(excerpt_id, buffer_open_range.start)?
                                ..multi_buffer_snapshot
                                    .anchor_in_excerpt(excerpt_id, buffer_open_range.end)?;
                            let buffer_close_range = buffer_snapshot
                                .anchor_before(pair.close_range.start)
                                ..buffer_snapshot.anchor_after(pair.close_range.end);
                            let multi_buffer_close_range = multi_buffer_snapshot
                                .anchor_in_excerpt(excerpt_id, buffer_close_range.start)?
                                ..multi_buffer_snapshot
                                    .anchor_in_excerpt(excerpt_id, buffer_close_range.end)?;
                            Some((
                                color_index % accents_count,
                                multi_buffer_open_range,
                                multi_buffer_close_range,
                            ))
                        });

                    for (accent_number, open_range, close_range) in brackets_by_accent {
                        let ranges = acc.entry(accent_number).or_insert_with(Vec::new);
                        ranges.push(open_range);
                        ranges.push(close_range);
                    }
                }

                acc
            },
        );

        if invalidate {
            self.clear_highlights::<RainbowBracketHighlight>(cx);
        }

        let editor_background = cx.theme().colors().editor_background;
        for (accent_number, bracket_highlights) in bracket_matches_by_accent {
            let bracket_color = cx.theme().accents().color_for_index(accent_number as u32);
            let adjusted_color = ensure_minimum_contrast(bracket_color, editor_background, 55.0);
            let style = HighlightStyle {
                color: Some(adjusted_color),
                ..HighlightStyle::default()
            };

            self.highlight_text_key::<RainbowBracketHighlight>(
                accent_number,
                bracket_highlights,
                style,
                true,
                cx,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, ops::Range, time::Duration};

    use super::*;
    use crate::{
        DisplayPoint,
        display_map::{DisplayRow, ToDisplayPoint},
        editor_tests::init_test,
        test::{
            editor_lsp_test_context::EditorLspTestContext, editor_test_context::EditorTestContext,
        },
    };
    use gpui::Hsla;
    use indoc::indoc;
    use language::{BracketPair, BracketPairConfig, Language, LanguageConfig, LanguageMatcher};
    use multi_buffer::AnchorRangeExt as _;
    use rope::Point;
    use text::OffsetRangeExt;

    #[gpui::test]
    async fn test_rainbow_bracket_highlights(cx: &mut gpui::TestAppContext) {
        fn collect_colored_brackets(
            cx: &mut EditorTestContext,
        ) -> Vec<(Option<Hsla>, Range<Point>)> {
            cx.update_editor(|editor, window, cx| {
                let snapshot = editor.snapshot(window, cx);
                snapshot
                    .all_text_highlight_ranges::<RainbowBracketHighlight>()
                    .iter()
                    .flat_map(|ranges| {
                        ranges.1.iter().map(|range| {
                            (ranges.0.color, range.to_point(&snapshot.buffer_snapshot()))
                        })
                    })
                    .collect::<Vec<_>>()
            })
        }

        init_test(cx, |language_settings| {
            language_settings.defaults.colorize_brackets = Some(true);
        });

        let mut cx = EditorLspTestContext::new(
            Language::new(
                LanguageConfig {
                    name: "Rust".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["rs".to_string()],
                        ..LanguageMatcher::default()
                    },
                    brackets: BracketPairConfig {
                        pairs: vec![
                            BracketPair {
                                start: "{".to_string(),
                                end: "}".to_string(),
                                close: false,
                                surround: false,
                                newline: true,
                            },
                            BracketPair {
                                start: "(".to_string(),
                                end: ")".to_string(),
                                close: false,
                                surround: false,
                                newline: true,
                            },
                        ],
                        ..BracketPairConfig::default()
                    },
                    ..LanguageConfig::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_brackets_query(indoc! {r#"
                ("{" @open "}" @close)
                ("(" @open ")" @close)
                "#})
            .unwrap(),
            lsp::ServerCapabilities::default(),
            cx,
        )
        .await;

        let mut highlighted_brackets = HashMap::default();

        // taken from r-a https://github.com/rust-lang/rust-analyzer/blob/d733c07552a2dc0ec0cc8f4df3f0ca969a93fd90/crates/ide/src/inlay_hints.rs#L81-L297
        cx.set_state(indoc! {r#"ˇ
            pub(crate) fn inlay_hints(
                db: &RootDatabase,
                file_id: FileId,
                range_limit: Option<TextRange>,
                config: &InlayHintsConfig,
            ) -> Vec<InlayHint> {
                let _p = tracing::info_span!("inlay_hints").entered();
                let sema = Semantics::new(db);
                let file_id = sema
                    .attach_first_edition(file_id)
                    .unwrap_or_else(|| EditionedFileId::current_edition(db, file_id));
                let file = sema.parse(file_id);
                let file = file.syntax();

                let mut acc = Vec::new();

                let Some(scope) = sema.scope(file) else {
                    return acc;
                };
                let famous_defs = FamousDefs(&sema, scope.krate());
                let display_target = famous_defs.1.to_display_target(sema.db);

                let ctx = &mut InlayHintCtx::default();
                let mut hints = |event| {
                    if let Some(node) = handle_event(ctx, event) {
                        hints(&mut acc, ctx, &famous_defs, config, file_id, display_target, node);
                    }
                };
                let mut preorder = file.preorder();
                salsa::attach(sema.db, || {
                    while let Some(event) = preorder.next() {
                        if matches!((&event, range_limit), (WalkEvent::Enter(node), Some(range)) if range.intersect(node.text_range()).is_none())
                        {
                            preorder.skip_subtree();
                            continue;
                        }
                        hints(event);
                    }
                });
                if let Some(range_limit) = range_limit {
                    acc.retain(|hint| range_limit.contains_range(hint.range));
                }
                acc
            }

            #[derive(Default)]
            struct InlayHintCtx {
                lifetime_stacks: Vec<Vec<SmolStr>>,
                extern_block_parent: Option<ast::ExternBlock>,
            }

            pub(crate) fn inlay_hints_resolve(
                db: &RootDatabase,
                file_id: FileId,
                resolve_range: TextRange,
                hash: u64,
                config: &InlayHintsConfig,
                hasher: impl Fn(&InlayHint) -> u64,
            ) -> Option<InlayHint> {
                let _p = tracing::info_span!("inlay_hints_resolve").entered();
                let sema = Semantics::new(db);
                let file_id = sema
                    .attach_first_edition(file_id)
                    .unwrap_or_else(|| EditionedFileId::current_edition(db, file_id));
                let file = sema.parse(file_id);
                let file = file.syntax();

                let scope = sema.scope(file)?;
                let famous_defs = FamousDefs(&sema, scope.krate());
                let mut acc = Vec::new();

                let display_target = famous_defs.1.to_display_target(sema.db);

                let ctx = &mut InlayHintCtx::default();
                let mut hints = |event| {
                    if let Some(node) = handle_event(ctx, event) {
                        hints(&mut acc, ctx, &famous_defs, config, file_id, display_target, node);
                    }
                };

                let mut preorder = file.preorder();
                while let Some(event) = preorder.next() {
                    // FIXME: This can miss some hints that require the parent of the range to calculate
                    if matches!(&event, WalkEvent::Enter(node) if resolve_range.intersect(node.text_range()).is_none())
                    {
                        preorder.skip_subtree();
                        continue;
                    }
                    hints(event);
                }
                acc.into_iter().find(|hint| hasher(hint) == hash)
            }

            fn handle_event(ctx: &mut InlayHintCtx, node: WalkEvent<SyntaxNode>) -> Option<SyntaxNode> {
                match node {
                    WalkEvent::Enter(node) => {
                        if let Some(node) = ast::AnyHasGenericParams::cast(node.clone()) {
                            let params = node
                                .generic_param_list()
                                .map(|it| {
                                    it.lifetime_params()
                                        .filter_map(|it| {
                                            it.lifetime().map(|it| format_smolstr!("{}", &it.text()[1..]))
                                        })
                                        .collect()
                                })
                                .unwrap_or_default();
                            ctx.lifetime_stacks.push(params);
                        }
                        if let Some(node) = ast::ExternBlock::cast(node.clone()) {
                            ctx.extern_block_parent = Some(node);
                        }
                        Some(node)
                    }
                    WalkEvent::Leave(n) => {
                        if ast::AnyHasGenericParams::can_cast(n.kind()) {
                            ctx.lifetime_stacks.pop();
                        }
                        if ast::ExternBlock::can_cast(n.kind()) {
                            ctx.extern_block_parent = None;
                        }
                        None
                    }
                }
            }

            // FIXME: At some point when our hir infra is fleshed out enough we should flip this and traverse the
            // HIR instead of the syntax tree.
            fn hints(
                hints: &mut Vec<InlayHint>,
                ctx: &mut InlayHintCtx,
                famous_defs @ FamousDefs(sema, _krate): &FamousDefs<'_, '_>,
                config: &InlayHintsConfig,
                file_id: EditionedFileId,
                display_target: DisplayTarget,
                node: SyntaxNode,
            ) {
                closing_brace::hints(
                    hints,
                    sema,
                    config,
                    display_target,
                    InRealFile { file_id, value: node.clone() },
                );
                if let Some(any_has_generic_args) = ast::AnyHasGenericArgs::cast(node.clone()) {
                    generic_param::hints(hints, famous_defs, config, any_has_generic_args);
                }

                match_ast! {
                    match node {
                        ast::Expr(expr) => {
                            chaining::hints(hints, famous_defs, config, display_target, &expr);
                            adjustment::hints(hints, famous_defs, config, display_target, &expr);
                            match expr {
                                ast::Expr::CallExpr(it) => param_name::hints(hints, famous_defs, config, file_id, ast::Expr::from(it)),
                                ast::Expr::MethodCallExpr(it) => {
                                    param_name::hints(hints, famous_defs, config, file_id, ast::Expr::from(it))
                                }
                                ast::Expr::ClosureExpr(it) => {
                                    closure_captures::hints(hints, famous_defs, config, it.clone());
                                    closure_ret::hints(hints, famous_defs, config, display_target, it)
                                },
                                ast::Expr::RangeExpr(it) => range_exclusive::hints(hints, famous_defs, config, it),
                                _ => Some(()),
                            }
                        },
                        ast::Pat(it) => {
                            binding_mode::hints(hints, famous_defs, config, &it);
                            match it {
                                ast::Pat::IdentPat(it) => {
                                    bind_pat::hints(hints, famous_defs, config, display_target, &it);
                                }
                                ast::Pat::RangePat(it) => {
                                    range_exclusive::hints(hints, famous_defs, config, it);
                                }
                                _ => {}
                            }
                            Some(())
                        },
                        ast::Item(it) => match it {
                            ast::Item::Fn(it) => {
                                implicit_drop::hints(hints, famous_defs, config, display_target, &it);
                                if let Some(extern_block) = &ctx.extern_block_parent {
                                    extern_block::fn_hints(hints, famous_defs, config, &it, extern_block);
                                }
                                lifetime::fn_hints(hints, ctx, famous_defs, config,  it)
                            },
                            ast::Item::Static(it) => {
                                if let Some(extern_block) = &ctx.extern_block_parent {
                                    extern_block::static_hints(hints, famous_defs, config, &it, extern_block);
                                }
                                implicit_static::hints(hints, famous_defs, config,  Either::Left(it))
                            },
                            ast::Item::Const(it) => implicit_static::hints(hints, famous_defs, config, Either::Right(it)),
                            ast::Item::Enum(it) => discriminant::enum_hints(hints, famous_defs, config, it),
                            ast::Item::ExternBlock(it) => extern_block::extern_block_hints(hints, famous_defs, config, it),
                            _ => None,
                        },
                        // FIXME: trait object type elisions
                        ast::Type(ty) => match ty {
                            ast::Type::FnPtrType(ptr) => lifetime::fn_ptr_hints(hints, ctx, famous_defs, config,  ptr),
                            ast::Type::PathType(path) => {
                                lifetime::fn_path_hints(hints, ctx, famous_defs, config, &path);
                                implied_dyn_trait::hints(hints, famous_defs, config, Either::Left(path));
                                Some(())
                            },
                            ast::Type::DynTraitType(dyn_) => {
                                implied_dyn_trait::hints(hints, famous_defs, config, Either::Right(dyn_));
                                Some(())
                            },
                            _ => Some(()),
                        },
                        ast::GenericParamList(it) => bounds::hints(hints, famous_defs, config,  it),
                        _ => Some(()),
                    }
                };
            }
        "#});
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        let actual_ranges = collect_colored_brackets(&mut cx);

        for (color, range) in actual_ranges.iter().cloned() {
            highlighted_brackets.insert(range, color);
        }

        let last_bracket = actual_ranges
            .iter()
            .max_by_key(|(_, p)| p.end.row)
            .unwrap()
            .clone();

        cx.update_editor(|editor, window, cx| {
            let was_scrolled = editor.set_scroll_position(
                gpui::Point::new(0.0, last_bracket.1.end.row as f64 * 2.0),
                window,
                cx,
            );
            assert!(was_scrolled.0);
        });
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        let ranges_after_scrolling = collect_colored_brackets(&mut cx);
        let new_last_bracket = ranges_after_scrolling
            .iter()
            .max_by_key(|(_, p)| p.end.row)
            .unwrap()
            .clone();

        assert_ne!(
            last_bracket, new_last_bracket,
            "After scrolling down, we should have highlighted more brackets"
        );

        cx.update_editor(|editor, window, cx| {
            let was_scrolled = editor.set_scroll_position(gpui::Point::default(), window, cx);
            assert!(was_scrolled.0);
        });

        for _ in 0..200 {
            cx.update_editor(|editor, window, cx| {
                editor.apply_scroll_delta(gpui::Point::new(0.0, 0.25), window, cx);
            });
            cx.executor().run_until_parked();

            let colored_brackets = collect_colored_brackets(&mut cx);
            for (color, range) in colored_brackets.clone() {
                assert!(
                    highlighted_brackets.entry(range).or_insert(color) == &color,
                    "Colors should stay consistent while scrolling!"
                );
            }

            let snapshot = cx.update_editor(|editor, window, cx| editor.snapshot(window, cx));
            let scroll_position = snapshot.scroll_position();
            let visible_lines =
                cx.update_editor(|editor, _, _| editor.visible_line_count().unwrap());
            let visible_range = DisplayRow(scroll_position.y as u32)
                ..DisplayRow((scroll_position.y + visible_lines) as u32);

            let current_highlighted_bracket_set: HashSet<Point> = HashSet::from_iter(
                colored_brackets
                    .iter()
                    .flat_map(|(_, range)| [range.start, range.end]),
            );

            for highlight_range in highlighted_brackets.keys().filter(|bracket_range| {
                visible_range.contains(&bracket_range.start.to_display_point(&snapshot).row())
                    || visible_range.contains(&bracket_range.end.to_display_point(&snapshot).row())
            }) {
                assert!(
                    current_highlighted_bracket_set.contains(&highlight_range.start)
                        || current_highlighted_bracket_set.contains(&highlight_range.end),
                    "Should not lose highlights while scrolling in the visible range!"
                );
            }

            let buffer_snapshot = snapshot.buffer().as_singleton().unwrap().2;
            for (start, end) in snapshot
                .bracket_ranges(
                    DisplayPoint::new(visible_range.start, Default::default()).to_point(&snapshot)
                        ..DisplayPoint::new(
                            visible_range.end,
                            snapshot.line_len(visible_range.end),
                        )
                        .to_point(&snapshot),
                )
                .into_iter()
                .flatten()
            {
                let start_bracket = colored_brackets
                    .iter()
                    .find(|(_, range)| range.to_offset(buffer_snapshot) == start);
                assert!(
                    start_bracket.is_some(),
                    "Existing bracket start in the visible range should be highlighted"
                );

                let end_bracket = colored_brackets
                    .iter()
                    .find(|(_, range)| range.to_offset(buffer_snapshot) == end);
                assert!(
                    end_bracket.is_some(),
                    "Existing bracket end in the visible range should be highlighted"
                );

                assert_eq!(
                    start_bracket.unwrap().0,
                    end_bracket.unwrap().0,
                    "Bracket pair should be highlighted the same color!"
                )
            }
        }

        // todo! more tests, check no brackets missing in range, settings toggle
    }
}
