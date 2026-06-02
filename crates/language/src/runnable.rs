use std::{cmp::Reverse, iter, ops::Range, sync::Arc};

use collections::HashMap;
use smallvec::SmallVec;
use text::BufferId;
use tree_sitter::QueryCapture;
use util::RangeExt;

use crate::{BufferSnapshot, Language, Runnable, RunnableCapture, RunnableConfig, RunnableTag};

pub struct RunnableRange {
    pub buffer_id: BufferId,
    pub run_range: Range<usize>,
    pub full_range: Range<usize>,
    pub runnable: Runnable,
    pub extra_captures: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct RunnableMatchCapture {
    range: Range<usize>,
    capture: ResolverCapture,
}

/// The subset of `RunnableCapture` that's meaningful to resolvers.
#[derive(Clone, Debug)]
enum ResolverCapture {
    Run,
    Named(String),
}

impl RunnableMatchCapture {
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    pub fn is_run(&self) -> bool {
        matches!(self.capture, ResolverCapture::Run)
    }

    pub fn name(&self) -> Option<&str> {
        match &self.capture {
            ResolverCapture::Named(name) => Some(name.as_str()),
            ResolverCapture::Run => None,
        }
    }
}

pub struct ResolvedRunnable {
    pub run_range: Range<usize>,
    pub extra_captures: SmallVec<[(String, String); 2]>,
}

pub trait RunnableResolver: Send + Sync {
    fn resolve(
        &self,
        local_captures: &[RunnableMatchCapture],
        shared_captures: &[RunnableMatchCapture],
        buffer: &BufferSnapshot,
    ) -> Option<ResolvedRunnable>;
}

pub(crate) fn runnable_ranges(
    buffer: &BufferSnapshot,
    offset_range: Range<usize>,
) -> impl Iterator<Item = RunnableRange> + '_ {
    let mut syntax_matches = buffer.matches(offset_range.clone(), |grammar| {
        grammar.runnable_config.as_ref().map(|config| &config.query)
    });

    let runnable_configs = syntax_matches
        .grammars()
        .iter()
        .map(|grammar| grammar.runnable_config.as_ref())
        .collect::<Vec<_>>();

    iter::from_fn(move || -> Option<SmallVec<[RunnableRange; 1]>> {
        let mat = syntax_matches.peek()?;

        let ranges = match runnable_configs[mat.grammar_index] {
            Some(runnable_config) => {
                let is_grouped = runnable_config.supports_grouped_runnables
                    && mat.captures.iter().any(|capture| {
                        matches!(
                            runnable_config.extra_captures.get(capture.index as usize),
                            Some(RunnableCapture::RunItem)
                        )
                    });
                if is_grouped {
                    runnable_ranges_from_grouped_matches(
                        buffer,
                        mat.captures,
                        runnable_config,
                        mat.pattern_index,
                        mat.language.clone(),
                        offset_range.clone(),
                    )
                } else {
                    runnable_range_from_captures(
                        buffer,
                        mat.captures,
                        runnable_config,
                        mat.pattern_index,
                        mat.language,
                    )
                    .into_iter()
                    .collect()
                }
            }
            None => SmallVec::new(),
        };

        syntax_matches.advance();
        Some(ranges)
    })
    .flatten()
}

type RunnableMatchCaptures = SmallVec<[RunnableMatchCapture; 4]>;

struct RunnableMatchGroup {
    range: Range<usize>,
    captures: RunnableMatchCaptures,
}

struct GroupedRunnableMatches {
    groups: SmallVec<[RunnableMatchGroup; 1]>,
    shared_captures: RunnableMatchCaptures,
}

fn runnable_tags_from_pattern(
    query: &tree_sitter::Query,
    pattern_index: usize,
) -> SmallVec<[RunnableTag; 1]> {
    query
        .property_settings(pattern_index)
        .iter()
        .filter_map(|property| {
            (*property.key == *"tag")
                .then(|| {
                    property
                        .value
                        .as_ref()
                        .map(|value| RunnableTag(value.to_string().into()))
                })
                .flatten()
        })
        .collect()
}

/// `overlaps` rejects empty ranges, so handle a zero-width `offset_range` (cursor) separately.
fn range_overlaps_or_contains(range: &Range<usize>, offset_range: &Range<usize>) -> bool {
    if offset_range.is_empty() {
        range.contains(&offset_range.start)
    } else {
        range.overlaps(offset_range)
    }
}

fn group_runnable_matches(
    captures: &[QueryCapture<'_>],
    runnable_config: &RunnableConfig,
    offset_range: Range<usize>,
) -> GroupedRunnableMatches {
    let mut sorted: SmallVec<[&QueryCapture<'_>; 16]> = captures.iter().collect();
    sorted.sort_by_key(|capture| {
        let range = capture.node.byte_range();
        (range.start, Reverse(range.end))
    });

    let mut groups = SmallVec::new();
    let mut shared_captures = SmallVec::new();
    let mut current_group: Option<RunnableMatchGroup> = None;
    let mut current_in_offset = false;

    for capture in sorted {
        let range = capture.node.byte_range();
        let Some(kind) = runnable_config.extra_captures.get(capture.index as usize) else {
            continue;
        };

        let resolver_capture = match kind {
            RunnableCapture::RunItem => {
                if let Some(group) = current_group.take()
                    && current_in_offset
                {
                    groups.push(group);
                }
                current_in_offset = range_overlaps_or_contains(&range, &offset_range);
                current_group = Some(RunnableMatchGroup {
                    range,
                    captures: SmallVec::new(),
                });
                continue;
            }
            RunnableCapture::Run => ResolverCapture::Run,
            RunnableCapture::Named(name) => ResolverCapture::Named(name.to_string()),
        };

        let match_capture = RunnableMatchCapture {
            range: range.clone(),
            capture: resolver_capture,
        };

        match current_group.as_mut() {
            Some(group) if group.range.contains_inclusive(&range) => {
                if current_in_offset {
                    group.captures.push(match_capture);
                }
            }
            _ => {
                if let Some(group) = current_group.take()
                    && current_in_offset
                {
                    groups.push(group);
                }
                shared_captures.push(match_capture);
            }
        }
    }
    if let Some(group) = current_group.take()
        && current_in_offset
    {
        groups.push(group);
    }

    GroupedRunnableMatches {
        groups,
        shared_captures,
    }
}

fn runnable_ranges_from_grouped_matches(
    buffer: &BufferSnapshot,
    captures: &[QueryCapture<'_>],
    runnable_config: &RunnableConfig,
    pattern_index: usize,
    language: Arc<Language>,
    offset_range: Range<usize>,
) -> SmallVec<[RunnableRange; 1]> {
    let GroupedRunnableMatches {
        groups,
        shared_captures,
    } = group_runnable_matches(captures, runnable_config, offset_range);

    let shared_extras: SmallVec<[(String, String); 4]> = shared_captures
        .iter()
        .filter_map(|capture| {
            capture.name().map(|name| {
                (
                    name.to_string(),
                    buffer.text_for_range(capture.range()).collect::<String>(),
                )
            })
        })
        .collect();

    let Some(resolver) = language
        .context_provider()
        .and_then(|provider| provider.runnable_resolver())
    else {
        return SmallVec::new();
    };
    let mut runnable_ranges = SmallVec::with_capacity(groups.len());

    let tags = runnable_tags_from_pattern(&runnable_config.query, pattern_index);
    let buffer_id = buffer.remote_id();
    for group in groups {
        let Some(ResolvedRunnable {
            run_range,
            extra_captures: local_extras,
        }) = resolver.resolve(&group.captures, &shared_captures, buffer)
        else {
            continue;
        };

        let extra_captures = shared_extras.iter().cloned().chain(local_extras).collect();

        runnable_ranges.push(RunnableRange {
            run_range,
            full_range: group.range,
            runnable: Runnable {
                tags: tags.clone(),
                language: language.clone(),
                buffer: buffer_id,
            },
            extra_captures,
            buffer_id,
        });
    }

    runnable_ranges
}

fn runnable_range_from_captures(
    buffer: &BufferSnapshot,
    captures: &[QueryCapture<'_>],
    runnable_config: &RunnableConfig,
    pattern_index: usize,
    language: Arc<Language>,
) -> Option<RunnableRange> {
    let mut run_range = None;
    let first_capture = captures.first()?;
    let full_range =
        captures
            .iter()
            .skip(1)
            .fold(first_capture.node.byte_range(), |mut acc, next| {
                let byte_range = next.node.byte_range();
                if acc.start > byte_range.start {
                    acc.start = byte_range.start;
                }
                if acc.end < byte_range.end {
                    acc.end = byte_range.end;
                }
                acc
            });

    let extra_captures: SmallVec<[_; 1]> =
        SmallVec::from_iter(captures.iter().filter_map(|capture| {
            runnable_config
                .extra_captures
                .get(capture.index as usize)
                .cloned()
                .and_then(|tag_name| match tag_name {
                    RunnableCapture::Named(name) => Some((capture.node.byte_range(), name)),
                    RunnableCapture::Run => {
                        let _ = run_range.insert(capture.node.byte_range());
                        None
                    }
                    RunnableCapture::RunItem => None,
                })
        }));
    let run_range = run_range?;
    let tags = runnable_tags_from_pattern(&runnable_config.query, pattern_index);
    let extra_captures = extra_captures
        .into_iter()
        .map(|(range, name)| {
            (
                name.to_string(),
                buffer.text_for_range(range).collect::<String>(),
            )
        })
        .collect();

    // A runnable has one range, even when its query pattern contributes multiple tags.
    let buffer_id = buffer.remote_id();
    Some(RunnableRange {
        run_range,
        full_range,
        runnable: Runnable {
            tags,
            language,
            buffer: buffer_id,
        },
        extra_captures,
        buffer_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Buffer, ContextProvider, Language, LanguageConfig, LanguageMatcher, LanguageQueries,
    };
    use gpui::{AppContext as _, TestAppContext};
    use indoc::indoc;
    use std::{borrow::Cow, sync::Arc};

    struct TestContextProvider {
        resolver: Arc<dyn RunnableResolver>,
    }

    impl ContextProvider for TestContextProvider {
        fn runnable_resolver(&self) -> Option<Arc<dyn RunnableResolver>> {
            Some(self.resolver.clone())
        }
    }

    fn make_language(
        runnables_query: &'static str,
        resolver: Option<Arc<dyn RunnableResolver>>,
    ) -> Arc<Language> {
        let language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_queries(LanguageQueries {
            runnables: Some(Cow::Borrowed(runnables_query)),
            ..Default::default()
        })
        .expect("parse runnables query");
        let context_provider = resolver
            .map(|resolver| Arc::new(TestContextProvider { resolver }) as Arc<dyn ContextProvider>);
        Arc::new(language.with_context_provider(context_provider))
    }

    fn collect_runnables(
        cx: &mut TestAppContext,
        source: &str,
        runnables_query: &'static str,
        resolver: Option<Arc<dyn RunnableResolver>>,
    ) -> Vec<RunnableRange> {
        collect_runnables_in(cx, source, runnables_query, resolver, None)
    }

    fn collect_runnables_in(
        cx: &mut TestAppContext,
        source: &str,
        runnables_query: &'static str,
        resolver: Option<Arc<dyn RunnableResolver>>,
        offset_range: Option<Range<usize>>,
    ) -> Vec<RunnableRange> {
        let language = make_language(runnables_query, resolver);
        let source_owned = source.to_string();
        let buffer = cx
            .new(|cx| Buffer::local(source_owned.clone(), cx).with_language(language.clone(), cx));
        cx.executor().run_until_parked();
        let range = offset_range.unwrap_or(0..source_owned.len());
        buffer.update(cx, |buffer, _| {
            buffer.snapshot().runnable_ranges(range).collect()
        })
    }

    fn text_at(buffer: &BufferSnapshot, range: Range<usize>) -> String {
        buffer.text_for_range(range).collect()
    }

    /// Picks the first `@run` capture, attaches no extras.
    struct FirstRunResolver;

    impl RunnableResolver for FirstRunResolver {
        fn resolve(
            &self,
            local_captures: &[RunnableMatchCapture],
            _shared_captures: &[RunnableMatchCapture],
            _buffer: &BufferSnapshot,
        ) -> Option<ResolvedRunnable> {
            let run = local_captures.iter().find(|capture| capture.is_run())?;
            Some(ResolvedRunnable {
                run_range: run.range(),
                extra_captures: SmallVec::new(),
            })
        }
    }

    /// Picks the first `@run` and surfaces every local named capture as an extra.
    struct LocalExtrasResolver;

    impl RunnableResolver for LocalExtrasResolver {
        fn resolve(
            &self,
            local_captures: &[RunnableMatchCapture],
            _shared_captures: &[RunnableMatchCapture],
            buffer: &BufferSnapshot,
        ) -> Option<ResolvedRunnable> {
            let run = local_captures.iter().find(|capture| capture.is_run())?;
            let extras = local_captures
                .iter()
                .filter_map(|capture| {
                    capture
                        .name()
                        .map(|name| (name.to_string(), text_at(buffer, capture.range())))
                })
                .collect();
            Some(ResolvedRunnable {
                run_range: run.range(),
                extra_captures: extras,
            })
        }
    }

    /// Skips groups whose `@run` text equals `skip_text`; otherwise picks the first `@run`.
    struct SkipByTextResolver {
        skip_text: &'static str,
    }

    impl RunnableResolver for SkipByTextResolver {
        fn resolve(
            &self,
            local_captures: &[RunnableMatchCapture],
            _shared_captures: &[RunnableMatchCapture],
            buffer: &BufferSnapshot,
        ) -> Option<ResolvedRunnable> {
            let run = local_captures.iter().find(|capture| capture.is_run())?;
            if text_at(buffer, run.range()) == self.skip_text {
                return None;
            }
            Some(ResolvedRunnable {
                run_range: run.range(),
                extra_captures: SmallVec::new(),
            })
        }
    }

    /// Always emits `_outer = LOCAL` as a local extra (to exercise the shared/local merge).
    struct OverrideSharedResolver;

    impl RunnableResolver for OverrideSharedResolver {
        fn resolve(
            &self,
            local_captures: &[RunnableMatchCapture],
            _shared_captures: &[RunnableMatchCapture],
            _buffer: &BufferSnapshot,
        ) -> Option<ResolvedRunnable> {
            let run = local_captures.iter().find(|capture| capture.is_run())?;
            let mut extras: SmallVec<[(String, String); 2]> = SmallVec::new();
            extras.push(("_outer".to_string(), "LOCAL".to_string()));
            Some(ResolvedRunnable {
                run_range: run.range(),
                extra_captures: extras,
            })
        }
    }

    const GROUPED_QUERY: &str = indoc! {r#"
        (function_item
          name: (identifier) @_outer
          body: (block
            ((expression_statement
               (call_expression
                 function: (identifier) @run @_call)) @run_item)+))
    "#};

    const GROUPED_SOURCE: &str = indoc! {r#"
        fn outer() {
            alpha();
            beta();
            gamma();
        }
    "#};

    #[gpui::test]
    fn test_single_match_emits_one_runnable_per_match(cx: &mut TestAppContext) {
        let query = indoc! {r#"
            ((function_item
               name: (identifier) @run
               (#match? @run "^test_")) @_decl)
        "#};
        let source = indoc! {r#"
            fn test_alpha() {}
            fn helper() {}
            fn test_beta() {}
        "#};

        let runnables = collect_runnables(cx, source, query, None);
        let run_texts: Vec<String> = runnables
            .iter()
            .map(|range| source[range.run_range.clone()].to_string())
            .collect();
        assert_eq!(run_texts, vec!["test_alpha", "test_beta"]);

        let decls: Vec<&str> = runnables
            .iter()
            .filter_map(|range| range.extra_captures.get("_decl").map(String::as_str))
            .collect();
        assert_eq!(decls, vec!["fn test_alpha() {}", "fn test_beta() {}"]);
    }

    #[gpui::test]
    fn test_single_match_without_run_capture_skipped(cx: &mut TestAppContext) {
        // Pattern with only a named capture and no `@run`: should silently produce nothing.
        let query = indoc! {r#"
            (function_item) @_decl
        "#};
        let source = indoc! {r#"
            fn helper() {}
            fn another() {}
        "#};

        let runnables = collect_runnables(cx, source, query, None);
        assert!(
            runnables.is_empty(),
            "matches without @run should produce no runnables, got {}",
            runnables.len()
        );
    }

    #[gpui::test]
    fn test_match_with_no_runnable_does_not_terminate_iteration(cx: &mut TestAppContext) {
        // A syntax match yielding no runnable must not terminate the
        // outer iterator before later matches that DO have `@run` are visited.
        let query = indoc! {r#"
            ((function_item
               name: (identifier) @_helper
               (#match? @_helper "^helper")) @_decl_no_run)

            ((function_item
               name: (identifier) @run
               (#match? @run "^test_")) @_decl)
        "#};
        let source = indoc! {r#"
            fn helper() {}
            fn test_alpha() {}
        "#};

        let runnables = collect_runnables(cx, source, query, None);
        let run_texts: Vec<String> = runnables
            .iter()
            .map(|range| source[range.run_range.clone()].to_string())
            .collect();
        assert_eq!(
            run_texts,
            vec!["test_alpha"],
            "syntax matches that produce no runnable must not terminate iteration"
        );
    }

    #[gpui::test]
    fn test_grouped_match_without_resolver_emits_nothing(cx: &mut TestAppContext) {
        // `@run_item` is present but no resolver is registered on the language.
        let runnables = collect_runnables(cx, GROUPED_SOURCE, GROUPED_QUERY, None);
        assert!(
            runnables.is_empty(),
            "grouped path with no resolver should emit nothing, got {}",
            runnables.len()
        );
    }

    #[gpui::test]
    fn test_grouped_match_emits_one_runnable_per_run_item(cx: &mut TestAppContext) {
        let resolver: Arc<dyn RunnableResolver> = Arc::new(FirstRunResolver);
        let runnables = collect_runnables(cx, GROUPED_SOURCE, GROUPED_QUERY, Some(resolver));

        let run_texts: Vec<String> = runnables
            .iter()
            .map(|range| GROUPED_SOURCE[range.run_range.clone()].to_string())
            .collect();
        assert_eq!(run_texts, vec!["alpha", "beta", "gamma"]);
    }

    #[gpui::test]
    fn test_grouped_match_shared_captures_propagate(cx: &mut TestAppContext) {
        let resolver: Arc<dyn RunnableResolver> = Arc::new(FirstRunResolver);
        let runnables = collect_runnables(cx, GROUPED_SOURCE, GROUPED_QUERY, Some(resolver));

        for range in &runnables {
            assert_eq!(
                range.extra_captures.get("_outer").map(String::as_str),
                Some("outer"),
                "every grouped runnable should inherit the shared `_outer` capture"
            );
        }
        assert_eq!(runnables.len(), 3);
    }

    #[gpui::test]
    fn test_grouped_match_local_extras_are_per_group(cx: &mut TestAppContext) {
        let resolver: Arc<dyn RunnableResolver> = Arc::new(LocalExtrasResolver);
        let runnables = collect_runnables(cx, GROUPED_SOURCE, GROUPED_QUERY, Some(resolver));

        let calls: Vec<&str> = runnables
            .iter()
            .filter_map(|range| range.extra_captures.get("_call").map(String::as_str))
            .collect();
        assert_eq!(
            calls,
            vec!["alpha", "beta", "gamma"],
            "each group's local `_call` capture should come from that row only"
        );
    }

    #[gpui::test]
    fn test_grouped_match_resolver_returning_none_skips_group(cx: &mut TestAppContext) {
        let source = indoc! {r#"
            fn outer() {
                alpha();
                skip_me();
                gamma();
            }
        "#};
        let resolver: Arc<dyn RunnableResolver> = Arc::new(SkipByTextResolver {
            skip_text: "skip_me",
        });
        let runnables = collect_runnables(cx, source, GROUPED_QUERY, Some(resolver));

        let run_texts: Vec<String> = runnables
            .iter()
            .map(|range| source[range.run_range.clone()].to_string())
            .collect();
        assert_eq!(run_texts, vec!["alpha", "gamma"]);
    }

    #[gpui::test]
    fn test_grouped_match_offset_range_filters_groups(cx: &mut TestAppContext) {
        let resolver: Arc<dyn RunnableResolver> = Arc::new(FirstRunResolver);
        let beta_offset = GROUPED_SOURCE
            .find("beta()")
            .expect("source should contain `beta()`");
        let runnables = collect_runnables_in(
            cx,
            GROUPED_SOURCE,
            GROUPED_QUERY,
            Some(resolver),
            Some(beta_offset..beta_offset + "beta".len()),
        );

        let run_texts: Vec<String> = runnables
            .iter()
            .map(|range| GROUPED_SOURCE[range.run_range.clone()].to_string())
            .collect();
        assert_eq!(
            run_texts,
            vec!["beta"],
            "offset_range should restrict emitted groups to those overlapping it"
        );
    }

    #[gpui::test]
    fn test_grouped_match_zero_width_offset_at_group_start(cx: &mut TestAppContext) {
        let resolver: Arc<dyn RunnableResolver> = Arc::new(FirstRunResolver);
        let alpha_offset = GROUPED_SOURCE
            .find("alpha()")
            .expect("source should contain `alpha()`");
        let runnables = collect_runnables_in(
            cx,
            GROUPED_SOURCE,
            GROUPED_QUERY,
            Some(resolver),
            Some(alpha_offset..alpha_offset),
        );

        let run_texts: Vec<String> = runnables
            .iter()
            .map(|range| GROUPED_SOURCE[range.run_range.clone()].to_string())
            .collect();
        assert_eq!(
            run_texts,
            vec!["alpha"],
            "zero-width offset_range at the start of a group should include that group"
        );
    }

    #[gpui::test]
    fn test_local_extras_override_shared_extras_with_same_key(cx: &mut TestAppContext) {
        let resolver: Arc<dyn RunnableResolver> = Arc::new(OverrideSharedResolver);
        let runnables = collect_runnables(cx, GROUPED_SOURCE, GROUPED_QUERY, Some(resolver));

        for range in &runnables {
            assert_eq!(
                range.extra_captures.get("_outer").map(String::as_str),
                Some("LOCAL"),
                "local extras should override shared extras with the same key"
            );
        }
        assert_eq!(runnables.len(), 3);
    }
}
