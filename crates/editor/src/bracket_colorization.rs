//! Bracket highlights, also known as "rainbow brackets".
//! Uses tree-sitter queries from brackets.scm to capture bracket pairs,
//! and theme accents to colorize those.

use std::ops::Range;

use crate::Editor;
use collections::HashMap;
use gpui::{Context, HighlightStyle};
use language::language_settings;
use multi_buffer::{Anchor, ExcerptId};
use ui::{ActiveTheme, utils::ensure_minimum_contrast};

struct ColorizedBracketsHighlight;

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
        let all_excerpts = self.buffer().read(cx).excerpt_ids();
        let anchor_in_multi_buffer = |current_excerpt: ExcerptId, text_anchor: text::Anchor| {
            multi_buffer_snapshot
                .anchor_in_excerpt(current_excerpt, text_anchor)
                .or_else(|| {
                    all_excerpts
                        .iter()
                        .filter(|&&excerpt_id| excerpt_id != current_excerpt)
                        .find_map(|&excerpt_id| {
                            multi_buffer_snapshot.anchor_in_excerpt(excerpt_id, text_anchor)
                        })
                })
        };

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
                            let buffer_close_range = buffer_snapshot
                                .anchor_before(pair.close_range.start)
                                ..buffer_snapshot.anchor_after(pair.close_range.end);
                            let multi_buffer_open_range =
                                anchor_in_multi_buffer(excerpt_id, buffer_open_range.start)
                                    .zip(anchor_in_multi_buffer(excerpt_id, buffer_open_range.end));
                            let multi_buffer_close_range =
                                anchor_in_multi_buffer(excerpt_id, buffer_close_range.start).zip(
                                    anchor_in_multi_buffer(excerpt_id, buffer_close_range.end),
                                );

                            let mut ranges = Vec::with_capacity(2);
                            if let Some((open_start, open_end)) = multi_buffer_open_range {
                                ranges.push(open_start..open_end);
                            }
                            if let Some((close_start, close_end)) = multi_buffer_close_range {
                                ranges.push(close_start..close_end);
                            }
                            if ranges.is_empty() {
                                None
                            } else {
                                Some((color_index % accents_count, ranges))
                            }
                        });

                    for (accent_number, new_ranges) in brackets_by_accent {
                        let ranges = acc
                            .entry(accent_number)
                            .or_insert_with(Vec::<Range<Anchor>>::new);

                        for new_range in new_ranges {
                            let i = ranges
                                .binary_search_by(|probe| {
                                    probe.start.cmp(&new_range.start, &multi_buffer_snapshot)
                                })
                                .unwrap_or_else(|i| i);
                            ranges.insert(i, new_range);
                        }
                    }
                }

                acc
            },
        );

        if invalidate {
            self.clear_highlights::<ColorizedBracketsHighlight>(cx);
        }

        let editor_background = cx.theme().colors().editor_background;
        for (accent_number, bracket_highlights) in bracket_matches_by_accent {
            let bracket_color = cx.theme().accents().color_for_index(accent_number as u32);
            let adjusted_color = ensure_minimum_contrast(bracket_color, editor_background, 55.0);
            let style = HighlightStyle {
                color: Some(adjusted_color),
                ..HighlightStyle::default()
            };

            self.highlight_text_key::<ColorizedBracketsHighlight>(
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
    use std::{cmp, sync::Arc, time::Duration};

    use super::*;
    use crate::{
        DisplayPoint, EditorSnapshot, MoveToBeginning, MoveToEnd, MoveUp,
        display_map::{DisplayRow, ToDisplayPoint},
        editor_tests::init_test,
        test::{
            editor_lsp_test_context::EditorLspTestContext, editor_test_context::EditorTestContext,
        },
    };
    use collections::HashSet;
    use fs::FakeFs;
    use gpui::{AppContext as _, UpdateGlobal as _};
    use indoc::indoc;
    use itertools::Itertools;
    use language::Capability;
    use languages::rust_lang;
    use multi_buffer::{ExcerptRange, MultiBuffer};
    use pretty_assertions::assert_eq;
    use project::Project;
    use rope::Point;
    use serde_json::json;
    use settings::{AccentContent, SettingsStore};
    use text::{Bias, OffsetRangeExt, ToOffset};
    use theme::ThemeStyleContent;
    use ui::SharedString;
    use util::{path, post_inc};

    #[gpui::test]
    async fn test_basic_bracket_colorization(cx: &mut gpui::TestAppContext) {
        init_test(cx, |language_settings| {
            language_settings.defaults.colorize_brackets = Some(true);
        });
        let mut cx = EditorLspTestContext::new(
            Arc::into_inner(rust_lang()).unwrap(),
            lsp::ServerCapabilities::default(),
            cx,
        )
        .await;

        cx.set_state(indoc! {r#"ˇuse std::{collections::HashMap, future::Future};

fn main() {
    let a = one((), { () }, ());
    println!("{a}");
    println!("{a}");
    for i in 0..a {
        println!("{i}");
    }

    let b = {
        {
            {
                [([([([([([([([([([((), ())])])])])])])])])])]
            }
        }
    };
}

#[rustfmt::skip]
fn one(a: (), (): (), c: ()) -> usize { 1 }

fn two<T>(a: HashMap<String, Vec<Option<T>>>) -> usize
where
    T: Future<Output = HashMap<String, Vec<Option<Box<()>>>>>,
{
    2
}
"#});
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        assert_eq!(
            r#"use std::«1{collections::HashMap, future::Future}1»;

fn main«1()1» «1{
    let a = one«2(«3()3», «3{ «4()4» }3», «3()3»)2»;
    println!«2("{a}")2»;
    println!«2("{a}")2»;
    for i in 0..a «2{
        println!«3("{i}")3»;
    }2»

    let b = «2{
        «3{
            «4{
                «5[«6(«7[«1(«2[«3(«4[«5(«6[«7(«1[«2(«3[«4(«5[«6(«7[«1(«2[«3(«4()4», «4()4»)3»]2»)1»]7»)6»]5»)4»]3»)2»]1»)7»]6»)5»]4»)3»]2»)1»]7»)6»]5»
            }4»
        }3»
    }2»;
}1»

#«1[rustfmt::skip]1»
fn one«1(a: «2()2», «2()2»: «2()2», c: «2()2»)1» -> usize «1{ 1 }1»

fn two«1<T>1»«1(a: HashMap«2<String, Vec«3<Option«4<T>4»>3»>2»)1» -> usize
where
    T: Future«1<Output = HashMap«2<String, Vec«3<Option«4<Box«5<«6()6»>5»>4»>3»>2»>1»,
«1{
    2
}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
4 hsla(187.00, 47.00%, 59.22%, 1.00)
5 hsla(355.00, 65.00%, 75.94%, 1.00)
6 hsla(95.00, 38.00%, 62.00%, 1.00)
7 hsla(39.00, 67.00%, 69.00%, 1.00)
"#,
            &bracket_colors_markup(&mut cx),
            "All brackets should be colored based on their depth"
        );
    }

    #[gpui::test]
    async fn test_bracket_colorization_when_editing(cx: &mut gpui::TestAppContext) {
        init_test(cx, |language_settings| {
            language_settings.defaults.colorize_brackets = Some(true);
        });
        let mut cx = EditorLspTestContext::new(
            Arc::into_inner(rust_lang()).unwrap(),
            lsp::ServerCapabilities::default(),
            cx,
        )
        .await;

        cx.set_state(indoc! {r#"
struct Foo<'a, T> {
    data: Vec<Option<&'a T>>,
}

fn process_data() {
    let map:ˇ
}
"#});

        cx.update_editor(|editor, window, cx| {
            editor.handle_input(" Result<", window, cx);
        });
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        assert_eq!(
            indoc! {r#"
struct Foo«1<'a, T>1» «1{
    data: Vec«2<Option«3<&'a T>3»>2»,
}1»

fn process_data«1()1» «1{
    let map: Result<
}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
"#},
            &bracket_colors_markup(&mut cx),
            "Brackets without pairs should be ignored and not colored"
        );

        cx.update_editor(|editor, window, cx| {
            editor.handle_input("Option<Foo<'_, ()", window, cx);
        });
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        assert_eq!(
            indoc! {r#"
struct Foo«1<'a, T>1» «1{
    data: Vec«2<Option«3<&'a T>3»>2»,
}1»

fn process_data«1()1» «1{
    let map: Result<Option<Foo<'_, «2()2»
}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
"#},
            &bracket_colors_markup(&mut cx),
        );

        cx.update_editor(|editor, window, cx| {
            editor.handle_input(">", window, cx);
        });
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        assert_eq!(
            indoc! {r#"
struct Foo«1<'a, T>1» «1{
    data: Vec«2<Option«3<&'a T>3»>2»,
}1»

fn process_data«1()1» «1{
    let map: Result<Option<Foo«2<'_, «3()3»>2»
}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
"#},
            &bracket_colors_markup(&mut cx),
            "When brackets start to get closed, inner brackets are re-colored based on their depth"
        );

        cx.update_editor(|editor, window, cx| {
            editor.handle_input(">", window, cx);
        });
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        assert_eq!(
            indoc! {r#"
struct Foo«1<'a, T>1» «1{
    data: Vec«2<Option«3<&'a T>3»>2»,
}1»

fn process_data«1()1» «1{
    let map: Result<Option«2<Foo«3<'_, «4()4»>3»>2»
}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
4 hsla(187.00, 47.00%, 59.22%, 1.00)
"#},
            &bracket_colors_markup(&mut cx),
        );

        cx.update_editor(|editor, window, cx| {
            editor.handle_input(", ()> = unimplemented!();", window, cx);
        });
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        assert_eq!(
            indoc! {r#"
struct Foo«1<'a, T>1» «1{
    data: Vec«2<Option«3<&'a T>3»>2»,
}1»

fn process_data«1()1» «1{
    let map: Result«2<Option«3<Foo«4<'_, «5()5»>4»>3», «3()3»>2» = unimplemented!«2()2»;
}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
4 hsla(187.00, 47.00%, 59.22%, 1.00)
5 hsla(355.00, 65.00%, 75.94%, 1.00)
"#},
            &bracket_colors_markup(&mut cx),
        );
    }

    #[gpui::test]
    async fn test_bracket_colorization_chunks(cx: &mut gpui::TestAppContext) {
        let comment_lines = 100;

        init_test(cx, |language_settings| {
            language_settings.defaults.colorize_brackets = Some(true);
        });
        let mut cx = EditorLspTestContext::new(
            Arc::into_inner(rust_lang()).unwrap(),
            lsp::ServerCapabilities::default(),
            cx,
        )
        .await;

        cx.set_state(&separate_with_comment_lines(
            indoc! {r#"
mod foo {
    ˇfn process_data_1() {
        let map: Option<Vec<()>> = None;
    }
"#},
            indoc! {r#"
    fn process_data_2() {
        let map: Option<Vec<()>> = None;
    }
}
"#},
            comment_lines,
        ));

        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        assert_eq!(
            &separate_with_comment_lines(
                indoc! {r#"
mod foo «1{
    fn process_data_1«2()2» «2{
        let map: Option«3<Vec«4<«5()5»>4»>3» = None;
    }2»
"#},
                indoc! {r#"
    fn process_data_2() {
        let map: Option<Vec<()>> = None;
    }
}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
4 hsla(187.00, 47.00%, 59.22%, 1.00)
5 hsla(355.00, 65.00%, 75.94%, 1.00)
"#},
                comment_lines,
            ),
            &bracket_colors_markup(&mut cx),
            "First, the only visible chunk is getting the bracket highlights"
        );

        cx.update_editor(|editor, window, cx| {
            editor.move_to_end(&MoveToEnd, window, cx);
            editor.move_up(&MoveUp, window, cx);
        });
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        assert_eq!(
            &separate_with_comment_lines(
                indoc! {r#"
mod foo «1{
    fn process_data_1«2()2» «2{
        let map: Option«3<Vec«4<«5()5»>4»>3» = None;
    }2»
"#},
                indoc! {r#"
    fn process_data_2«2()2» «2{
        let map: Option«3<Vec«4<«5()5»>4»>3» = None;
    }2»
}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
4 hsla(187.00, 47.00%, 59.22%, 1.00)
5 hsla(355.00, 65.00%, 75.94%, 1.00)
"#},
                comment_lines,
            ),
            &bracket_colors_markup(&mut cx),
            "After scrolling to the bottom, both chunks should have the highlights"
        );

        cx.update_editor(|editor, window, cx| {
            editor.handle_input("{{}}}", window, cx);
        });
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        assert_eq!(
            &separate_with_comment_lines(
                indoc! {r#"
mod foo «1{
    fn process_data_1() {
        let map: Option<Vec<()>> = None;
    }
"#},
                indoc! {r#"
    fn process_data_2«2()2» «2{
        let map: Option«3<Vec«4<«5()5»>4»>3» = None;
    }
    «3{«4{}4»}3»}2»}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
4 hsla(187.00, 47.00%, 59.22%, 1.00)
5 hsla(355.00, 65.00%, 75.94%, 1.00)
"#},
                comment_lines,
            ),
            &bracket_colors_markup(&mut cx),
            "First chunk's brackets are invalidated after an edit, and only 2nd (visible) chunk is re-colorized"
        );

        cx.update_editor(|editor, window, cx| {
            editor.move_to_beginning(&MoveToBeginning, window, cx);
        });
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        assert_eq!(
            &separate_with_comment_lines(
                indoc! {r#"
mod foo «1{
    fn process_data_1«2()2» «2{
        let map: Option«3<Vec«4<«5()5»>4»>3» = None;
    }2»
"#},
                indoc! {r#"
    fn process_data_2«2()2» «2{
        let map: Option«3<Vec«4<«5()5»>4»>3» = None;
    }
    «3{«4{}4»}3»}2»}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
4 hsla(187.00, 47.00%, 59.22%, 1.00)
5 hsla(355.00, 65.00%, 75.94%, 1.00)
"#},
                comment_lines,
            ),
            &bracket_colors_markup(&mut cx),
            "Scrolling back to top should re-colorize all chunks' brackets"
        );

        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.colorize_brackets = Some(false);
                });
            });
        });
        assert_eq!(
            &separate_with_comment_lines(
                indoc! {r#"
mod foo {
    fn process_data_1() {
        let map: Option<Vec<()>> = None;
    }
"#},
                r#"    fn process_data_2() {
        let map: Option<Vec<()>> = None;
    }
    {{}}}}

"#,
                comment_lines,
            ),
            &bracket_colors_markup(&mut cx),
            "Turning bracket colorization off should remove all bracket colors"
        );

        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.colorize_brackets = Some(true);
                });
            });
        });
        assert_eq!(
            &separate_with_comment_lines(
                indoc! {r#"
mod foo «1{
    fn process_data_1«2()2» «2{
        let map: Option«3<Vec«4<«5()5»>4»>3» = None;
    }2»
"#},
                r#"    fn process_data_2() {
        let map: Option<Vec<()>> = None;
    }
    {{}}}}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
4 hsla(187.00, 47.00%, 59.22%, 1.00)
5 hsla(355.00, 65.00%, 75.94%, 1.00)
"#,
                comment_lines,
            ),
            &bracket_colors_markup(&mut cx),
            "Turning bracket colorization back on refreshes the visible excerpts' bracket colors"
        );
    }

    #[gpui::test]
    async fn test_rainbow_bracket_highlights(cx: &mut gpui::TestAppContext) {
        init_test(cx, |language_settings| {
            language_settings.defaults.colorize_brackets = Some(true);
        });
        let mut cx = EditorLspTestContext::new(
            Arc::into_inner(rust_lang()).unwrap(),
            lsp::ServerCapabilities::default(),
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
                    // This can miss some hints that require the parent of the range to calculate
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

            // At some point when our hir infra is fleshed out enough we should flip this and traverse the
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
                        // trait object type elisions
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

        let actual_ranges = cx.update_editor(|editor, window, cx| {
            editor
                .snapshot(window, cx)
                .all_text_highlight_ranges::<ColorizedBracketsHighlight>()
        });

        let mut highlighted_brackets = HashMap::default();
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

        let ranges_after_scrolling = cx.update_editor(|editor, window, cx| {
            editor
                .snapshot(window, cx)
                .all_text_highlight_ranges::<ColorizedBracketsHighlight>()
        });
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
            cx.executor().advance_clock(Duration::from_millis(100));
            cx.executor().run_until_parked();

            let colored_brackets = cx.update_editor(|editor, window, cx| {
                editor
                    .snapshot(window, cx)
                    .all_text_highlight_ranges::<ColorizedBracketsHighlight>()
            });
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
            for bracket_match in buffer_snapshot
                .fetch_bracket_ranges(
                    snapshot
                        .display_point_to_point(
                            DisplayPoint::new(visible_range.start, 0),
                            Bias::Left,
                        )
                        .to_offset(&buffer_snapshot)
                        ..snapshot
                            .display_point_to_point(
                                DisplayPoint::new(
                                    visible_range.end,
                                    snapshot.line_len(visible_range.end),
                                ),
                                Bias::Right,
                            )
                            .to_offset(&buffer_snapshot),
                    None,
                )
                .iter()
                .flat_map(|entry| entry.1)
                .filter(|bracket_match| bracket_match.color_index.is_some())
            {
                let start = bracket_match.open_range.to_point(buffer_snapshot);
                let end = bracket_match.close_range.to_point(buffer_snapshot);
                let start_bracket = colored_brackets.iter().find(|(_, range)| *range == start);
                assert!(
                    start_bracket.is_some(),
                    "Existing bracket start in the visible range should be highlighted. Missing color for match: \"{}\" at position {:?}",
                    buffer_snapshot
                        .text_for_range(start.start..end.end)
                        .collect::<String>(),
                    start
                );

                let end_bracket = colored_brackets.iter().find(|(_, range)| *range == end);
                assert!(
                    end_bracket.is_some(),
                    "Existing bracket end in the visible range should be highlighted. Missing color for match: \"{}\" at position {:?}",
                    buffer_snapshot
                        .text_for_range(start.start..end.end)
                        .collect::<String>(),
                    start
                );

                assert_eq!(
                    start_bracket.unwrap().0,
                    end_bracket.unwrap().0,
                    "Bracket pair should be highlighted the same color!"
                )
            }
        }
    }

    #[gpui::test]
    async fn test_multi_buffer(cx: &mut gpui::TestAppContext) {
        let comment_lines = 100;

        init_test(cx, |language_settings| {
            language_settings.defaults.colorize_brackets = Some(true);
        });
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() {{()}}",
                "lib.rs": separate_with_comment_lines(
                    indoc! {r#"
    mod foo {
        fn process_data_1() {
            let map: Option<Vec<()>> = None;
            // a
            // b
            // c
        }
    "#},
                    indoc! {r#"
        fn process_data_2() {
            let other_map: Option<Vec<()>> = None;
        }
    }
    "#},
                    comment_lines,
                )
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());

        let buffer_1 = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/lib.rs"), cx)
            })
            .await
            .unwrap();
        let buffer_2 = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();

        let multi_buffer = cx.new(|cx| {
            let mut multi_buffer = MultiBuffer::new(Capability::ReadWrite);
            multi_buffer.push_excerpts(
                buffer_2.clone(),
                [ExcerptRange::new(Point::new(0, 0)..Point::new(1, 0))],
                cx,
            );

            let excerpt_rows = 5;
            let rest_of_first_except_rows = 3;
            multi_buffer.push_excerpts(
                buffer_1.clone(),
                [
                    ExcerptRange::new(Point::new(0, 0)..Point::new(excerpt_rows, 0)),
                    ExcerptRange::new(
                        Point::new(
                            comment_lines as u32 + excerpt_rows + rest_of_first_except_rows,
                            0,
                        )
                            ..Point::new(
                                comment_lines as u32
                                    + excerpt_rows
                                    + rest_of_first_except_rows
                                    + excerpt_rows,
                                0,
                            ),
                    ),
                ],
                cx,
            );
            multi_buffer
        });

        let editor = cx.add_window(|window, cx| {
            Editor::for_multibuffer(multi_buffer, Some(project.clone()), window, cx)
        });
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        let editor_snapshot = editor
            .update(cx, |editor, window, cx| editor.snapshot(window, cx))
            .unwrap();
        assert_eq!(
            indoc! {r#"


fn main«1()1» «1{«2{«3()3»}2»}1»


mod foo «1{
    fn process_data_1«2()2» «2{
        let map: Option«3<Vec«4<«5()5»>4»>3» = None;
        // a
        // b


    fn process_data_2«2()2» «2{
        let other_map: Option«3<Vec«4<«5()5»>4»>3» = None;
    }2»
}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
4 hsla(187.00, 47.00%, 59.22%, 1.00)
5 hsla(355.00, 65.00%, 75.94%, 1.00)
"#,},
            &editor_bracket_colors_markup(&editor_snapshot),
            "Multi buffers should have their brackets colored even if no excerpts contain the bracket counterpart (after fn `process_data_2()`) \
or if the buffer pair spans across multiple excerpts (the one after `mod foo`)"
        );

        editor
            .update(cx, |editor, window, cx| {
                editor.handle_input("{[]", window, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        let editor_snapshot = editor
            .update(cx, |editor, window, cx| editor.snapshot(window, cx))
            .unwrap();
        assert_eq!(
            indoc! {r#"


{«1[]1»fn main«1()1» «1{«2{«3()3»}2»}1»


mod foo «1{
    fn process_data_1«2()2» «2{
        let map: Option«3<Vec«4<«5()5»>4»>3» = None;
        // a
        // b


    fn process_data_2«2()2» «2{
        let other_map: Option«3<Vec«4<«5()5»>4»>3» = None;
    }2»
}1»

1 hsla(207.80, 16.20%, 69.19%, 1.00)
2 hsla(29.00, 54.00%, 65.88%, 1.00)
3 hsla(286.00, 51.00%, 75.25%, 1.00)
4 hsla(187.00, 47.00%, 59.22%, 1.00)
5 hsla(355.00, 65.00%, 75.94%, 1.00)
"#,},
            &editor_bracket_colors_markup(&editor_snapshot),
        );

        cx.update(|cx| {
            let theme = cx.theme().name.clone();
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.theme.theme_overrides = HashMap::from_iter([(
                        theme.to_string(),
                        ThemeStyleContent {
                            accents: vec![
                                AccentContent(Some(SharedString::new("#ff0000"))),
                                AccentContent(Some(SharedString::new("#0000ff"))),
                            ],
                            ..ThemeStyleContent::default()
                        },
                    )]);
                });
            });
        });
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        let editor_snapshot = editor
            .update(cx, |editor, window, cx| editor.snapshot(window, cx))
            .unwrap();
        assert_eq!(
            indoc! {r#"


{«1[]1»fn main«1()1» «1{«2{«1()1»}2»}1»


mod foo «1{
    fn process_data_1«2()2» «2{
        let map: Option«1<Vec«2<«1()1»>2»>1» = None;
        // a
        // b


    fn process_data_2«2()2» «2{
        let other_map: Option«1<Vec«2<«1()1»>2»>1» = None;
    }2»
}1»

1 hsla(0.00, 100.00%, 78.12%, 1.00)
2 hsla(240.00, 100.00%, 82.81%, 1.00)
"#,},
            &editor_bracket_colors_markup(&editor_snapshot),
            "After updating theme accents, the editor should update the bracket coloring"
        );
    }

    fn separate_with_comment_lines(head: &str, tail: &str, comment_lines: usize) -> String {
        let mut result = head.to_string();
        result.push_str("\n");
        result.push_str(&"//\n".repeat(comment_lines));
        result.push_str(tail);
        result
    }

    fn bracket_colors_markup(cx: &mut EditorTestContext) -> String {
        cx.update_editor(|editor, window, cx| {
            editor_bracket_colors_markup(&editor.snapshot(window, cx))
        })
    }

    fn editor_bracket_colors_markup(snapshot: &EditorSnapshot) -> String {
        fn display_point_to_offset(text: &str, point: DisplayPoint) -> usize {
            let mut offset = 0;
            for (row_idx, line) in text.lines().enumerate() {
                if row_idx < point.row().0 as usize {
                    offset += line.len() + 1; // +1 for newline
                } else {
                    offset += point.column() as usize;
                    break;
                }
            }
            offset
        }

        let actual_ranges = snapshot.all_text_highlight_ranges::<ColorizedBracketsHighlight>();
        let editor_text = snapshot.text();

        let mut next_index = 1;
        let mut color_to_index = HashMap::default();
        let mut annotations = Vec::new();
        for (color, range) in &actual_ranges {
            let color_index = *color_to_index
                .entry(*color)
                .or_insert_with(|| post_inc(&mut next_index));
            let start = snapshot.point_to_display_point(range.start, Bias::Left);
            let end = snapshot.point_to_display_point(range.end, Bias::Right);
            let start_offset = display_point_to_offset(&editor_text, start);
            let end_offset = display_point_to_offset(&editor_text, end);
            let bracket_text = &editor_text[start_offset..end_offset];
            let bracket_char = bracket_text.chars().next().unwrap();

            if matches!(bracket_char, '{' | '[' | '(' | '<') {
                annotations.push((start_offset, format!("«{color_index}")));
            } else {
                annotations.push((end_offset, format!("{color_index}»")));
            }
        }

        annotations.sort_by(|(pos_a, text_a), (pos_b, text_b)| {
            pos_a.cmp(pos_b).reverse().then_with(|| {
                let a_is_opening = text_a.starts_with('«');
                let b_is_opening = text_b.starts_with('«');
                match (a_is_opening, b_is_opening) {
                    (true, false) => cmp::Ordering::Less,
                    (false, true) => cmp::Ordering::Greater,
                    _ => cmp::Ordering::Equal,
                }
            })
        });
        annotations.dedup();

        let mut markup = editor_text;
        for (offset, text) in annotations {
            markup.insert_str(offset, &text);
        }

        markup.push_str("\n");
        for (index, color) in color_to_index
            .iter()
            .map(|(color, index)| (*index, *color))
            .sorted_by_key(|(index, _)| *index)
        {
            markup.push_str(&format!("{index} {color}\n"));
        }

        markup
    }
}
