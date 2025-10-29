use crate::{Editor, RangeToAnchorExt};
use gpui::{Context, HighlightStyle, Hsla, Window};
use itertools::Itertools;
use language::CursorShape;
use multi_buffer::ToPoint;
use text::{Bias, Point};

enum MatchingBracketHighlight {}

struct RainbowBracketHighlight;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum BracketRefreshReason {
    BufferEdited,
    ScrollPositionChanged,
    SelectionsChanged,
}

impl Editor {
    // todo! run with a debounce
    pub(crate) fn refresh_bracket_highlights(
        &mut self,
        refresh_reason: BracketRefreshReason,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        const COLORS: [Hsla; 4] = [gpui::red(), gpui::yellow(), gpui::green(), gpui::blue()];

        let snapshot = self.snapshot(window, cx);
        let multi_buffer_snapshot = &snapshot.buffer_snapshot();

        let multi_buffer_visible_start = snapshot
            .scroll_anchor
            .anchor
            .to_point(multi_buffer_snapshot);

        // todo! deduplicate?
        let multi_buffer_visible_end = multi_buffer_snapshot.clip_point(
            multi_buffer_visible_start
                + Point::new(self.visible_line_count().unwrap_or(40.).ceil() as u32, 0),
            Bias::Left,
        );

        let bracket_matches = multi_buffer_snapshot
            .range_to_buffer_ranges(multi_buffer_visible_start..multi_buffer_visible_end)
            .into_iter()
            .filter_map(|(buffer_snapshot, buffer_range, _)| {
                let buffer_brackets =
                    buffer_snapshot.bracket_ranges(buffer_range.start..buffer_range.end);

                // todo! is there a good way to use the excerpt_id instead?
                let mut excerpt = multi_buffer_snapshot.excerpt_containing(buffer_range.clone())?;

                Some(
                    buffer_brackets
                        .into_iter()
                        .filter_map(|pair| {
                            let buffer_range = pair.open_range.start..pair.close_range.end;
                            if excerpt.contains_buffer_range(buffer_range) {
                                Some((
                                    pair.depth,
                                    excerpt.map_range_from_buffer(pair.open_range),
                                    excerpt.map_range_from_buffer(pair.close_range),
                                ))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .flatten()
            .into_group_map_by(|&(depth, ..)| depth);

        for (depth, bracket_highlights) in bracket_matches {
            let style = HighlightStyle {
                color: Some({
                    // todo! these colors lack contrast for this/are not actually good for that?
                    // cx.theme().accents().color_for_index(depth as u32);
                    COLORS[depth as usize % COLORS.len()]
                }),
                ..HighlightStyle::default()
            };

            self.highlight_text_key::<RainbowBracketHighlight>(
                depth,
                bracket_highlights
                    .into_iter()
                    .flat_map(|(_, open, close)| {
                        [
                            open.to_anchors(&multi_buffer_snapshot),
                            close.to_anchors(&multi_buffer_snapshot),
                        ]
                    })
                    .collect(),
                style,
                cx,
            );
        }

        if refresh_reason == BracketRefreshReason::ScrollPositionChanged {
            return;
        }
        self.clear_background_highlights::<MatchingBracketHighlight>(cx);

        let newest_selection = self.selections.newest::<usize>(&snapshot);
        // Don't highlight brackets if the selection isn't empty
        if !newest_selection.is_empty() {
            return;
        }

        let head = newest_selection.head();
        if head > snapshot.buffer_snapshot().len() {
            log::error!("bug: cursor offset is out of range while refreshing bracket highlights");
            return;
        }

        let mut tail = head;
        if (self.cursor_shape == CursorShape::Block || self.cursor_shape == CursorShape::Hollow)
            && head < snapshot.buffer_snapshot().len()
        {
            if let Some(tail_ch) = snapshot.buffer_snapshot().chars_at(tail).next() {
                tail += tail_ch.len_utf8();
            }
        }

        if let Some((opening_range, closing_range)) = snapshot
            .buffer_snapshot()
            .innermost_enclosing_bracket_ranges(head..tail, None)
        {
            self.highlight_background::<MatchingBracketHighlight>(
                &[
                    opening_range.to_anchors(&snapshot.buffer_snapshot()),
                    closing_range.to_anchors(&snapshot.buffer_snapshot()),
                ],
                |theme| theme.colors().editor_document_highlight_bracket_background,
                cx,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor_tests::init_test, test::editor_lsp_test_context::EditorLspTestContext};
    use indoc::indoc;
    use language::{BracketPair, BracketPairConfig, Language, LanguageConfig, LanguageMatcher};
    use multi_buffer::AnchorRangeExt as _;

    #[gpui::test]
    async fn test_matching_bracket_highlights(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new(
            Language::new(
                LanguageConfig {
                    name: "Rust".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["rs".to_string()],
                        ..Default::default()
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
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_brackets_query(indoc! {r#"
                ("{" @open "}" @close)
                ("(" @open ")" @close)
                "#})
            .unwrap(),
            Default::default(),
            cx,
        )
        .await;

        // positioning cursor inside bracket highlights both
        cx.set_state(indoc! {r#"
            pub fn test("Test ˇargument") {
                another_test(1, 2, 3);
            }
        "#});
        cx.assert_editor_background_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test«(»"Test argument"«)» {
                another_test(1, 2, 3);
            }
        "#});

        cx.set_state(indoc! {r#"
            pub fn test("Test argument") {
                another_test(1, ˇ2, 3);
            }
        "#});
        cx.assert_editor_background_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test("Test argument") {
                another_test«(»1, 2, 3«)»;
            }
        "#});

        cx.set_state(indoc! {r#"
            pub fn test("Test argument") {
                anotherˇ_test(1, 2, 3);
            }
        "#});
        cx.assert_editor_background_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test("Test argument") «{»
                another_test(1, 2, 3);
            «}»
        "#});

        // positioning outside of brackets removes highlight
        cx.set_state(indoc! {r#"
            pub fˇn test("Test argument") {
                another_test(1, 2, 3);
            }
        "#});
        cx.assert_editor_background_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test("Test argument") {
                another_test(1, 2, 3);
            }
        "#});

        // non empty selection dismisses highlight
        cx.set_state(indoc! {r#"
            pub fn test("Te«st argˇ»ument") {
                another_test(1, 2, 3);
            }
        "#});
        cx.assert_editor_background_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test«("Test argument") {
                another_test(1, 2, 3);
            }
        "#});
    }

    #[gpui::test]
    async fn test_rainbow_bracket_highlights(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new(
            Language::new(
                LanguageConfig {
                    name: "Rust".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["rs".to_string()],
                        ..Default::default()
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
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_brackets_query(indoc! {r#"
                ("{" @open "}" @close)
                ("(" @open ")" @close)
                "#})
            .unwrap(),
            Default::default(),
            cx,
        )
        .await;

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

        let snapshot = cx.update_editor(|editor, window, cx| editor.snapshot(window, cx));
        let actual_ranges = snapshot
            .all_text_highlight_ranges::<RainbowBracketHighlight>()
            .iter()
            .flat_map(|ranges| {
                ranges
                    .1
                    .iter()
                    .map(|range| (ranges.0.color, range.to_point(&snapshot.buffer_snapshot())))
            })
            .collect::<Vec<_>>();
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
        cx.executor().run_until_parked();
        let snapshot = cx.update_editor(|editor, window, cx| editor.snapshot(window, cx));

        let actual_ranges = snapshot
            .all_text_highlight_ranges::<RainbowBracketHighlight>()
            .iter()
            .flat_map(|ranges| {
                ranges
                    .1
                    .iter()
                    .map(|range| (ranges.0.color, range.to_point(&snapshot.buffer_snapshot())))
            })
            .collect::<Vec<_>>();
        let new_last_bracket = actual_ranges
            .iter()
            .max_by_key(|(_, p)| p.end.row)
            .unwrap()
            .clone();
        // todo! we do scroll down and get new brackets matched by tree-sitter,
        // but something still prevents the `text_key_highlight_ranges` to return the right, newest visible bracket on the lower row we scrolled to
        assert_ne!(
            last_bracket, new_last_bracket,
            "After scrolling down, we should have highlighted more brackets"
        );
    }
}
