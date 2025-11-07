use std::ops::Range;

use theme::ActiveTheme;

use clock::Global;
use collections::{HashMap, HashSet};
use gpui::{Context, HighlightStyle, Window};
use language::{Bias, BufferSnapshot};
use multi_buffer::{Anchor, ExcerptId};
use text::BufferId;

use crate::{DisplayPoint, DisplayRow, Editor};

const ROW_OVERSCAN: u32 = 64;

pub(super) struct RainbowHighlightState {
    cache: RainbowHighlightCache,
    active_color_keys: Vec<usize>,
}

impl Default for RainbowHighlightState {
    fn default() -> Self {
        Self {
            cache: RainbowHighlightCache::default(),
            active_color_keys: Vec::new(),
        }
    }
}

impl RainbowHighlightState {
    pub(super) fn invalidate_all(&mut self) {
        self.cache.clear();
        self.active_color_keys.clear();
    }

    pub(super) fn invalidate_buffer(&mut self, buffer_id: BufferId) {
        self.cache.invalidate_buffer(buffer_id);
    }

    pub(super) fn invalidate_excerpts(&mut self, excerpt_ids: &[ExcerptId]) {
        self.cache.invalidate_excerpts(excerpt_ids);
    }
}

#[derive(Default)]
struct RainbowHighlightCache {
    entries: HashMap<ExcerptId, RainbowCacheEntry>,
}

impl RainbowHighlightCache {
    fn get(
        &self,
        excerpt_id: &ExcerptId,
        buffer_id: BufferId,
        version: &Global,
        visible_ranges: &[Range<usize>],
        color_count: usize,
    ) -> Option<&RainbowCacheEntry> {
        let entry = self.entries.get(excerpt_id)?;
        (entry.buffer_id == buffer_id
            && entry.buffer_version == *version
            && entry.color_count == color_count
            && entry.visible_ranges == visible_ranges)
            .then_some(entry)
    }

    fn insert(&mut self, excerpt_id: ExcerptId, entry: RainbowCacheEntry) {
        self.entries.insert(excerpt_id, entry);
    }

    fn invalidate_buffer(&mut self, buffer_id: BufferId) {
        self.entries.retain(|_, entry| entry.buffer_id != buffer_id);
    }

    fn invalidate_excerpts(&mut self, excerpt_ids: &[ExcerptId]) {
        for excerpt_id in excerpt_ids {
            self.entries.remove(excerpt_id);
        }
    }

    fn clear(&mut self) {
        self.entries.clear();
    }
}

struct RainbowCacheEntry {
    buffer_id: BufferId,
    buffer_version: Global,
    visible_ranges: Vec<Range<usize>>,
    color_count: usize,
    ranges_by_color: Vec<Vec<Range<Anchor>>>,
}

impl RainbowCacheEntry {
    fn new(
        buffer_id: BufferId,
        buffer_version: Global,
        visible_ranges: Vec<Range<usize>>,
        color_count: usize,
        ranges_by_color: Vec<Vec<Range<Anchor>>>,
    ) -> Self {
        Self {
            buffer_id,
            buffer_version,
            visible_ranges,
            color_count,
            ranges_by_color,
        }
    }

    fn has_highlights(&self) -> bool {
        self.ranges_by_color.iter().any(|ranges| !ranges.is_empty())
    }
}

struct QueryExcerpt<'a> {
    buffer_snapshot: &'a BufferSnapshot,
    range: Range<usize>,
}

struct ActiveScope {
    end_byte: usize,
    level: usize,
    node_id: Option<usize>,
}

pub(super) struct RainbowBracketHighlight;

impl Editor {
    pub fn refresh_rainbow_bracket_highlights(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let display_snapshot = self.display_snapshot(cx);
        if display_snapshot.is_empty() {
            self.clear_rainbow_bracket_highlights(cx);
            self.rainbow_highlight_state.invalidate_all();
            return;
        }

        let Some(visible_line_count) = self.visible_line_count() else {
            self.clear_rainbow_bracket_highlights(cx);
            self.rainbow_highlight_state.invalidate_all();
            return;
        };

        let buffer_snapshot = self.buffer().read(cx).snapshot(cx);
        let scroll_position = self
            .scroll_manager
            .anchor()
            .scroll_position(&display_snapshot);
        let max_display_row = display_snapshot.max_point().row().0;
        let start_row = if scroll_position.y.is_sign_negative() {
            0
        } else {
            scroll_position.y.floor() as u32
        };
        let mut end_row = (scroll_position.y + visible_line_count).ceil() as u32;
        if end_row < start_row {
            end_row = start_row;
        }
        end_row = end_row.min(max_display_row);

        let query_start_row = start_row.saturating_sub(ROW_OVERSCAN);
        let query_end_row = (end_row + ROW_OVERSCAN).min(max_display_row);

        let (visible_start_point, visible_end_point) = (
            display_snapshot
                .display_point_to_point(DisplayPoint::new(DisplayRow(start_row), 0), Bias::Left),
            display_snapshot
                .display_point_to_point(DisplayPoint::new(DisplayRow(end_row), 0), Bias::Right),
        );

        if visible_start_point == visible_end_point {
            self.clear_rainbow_bracket_highlights(cx);
            self.rainbow_highlight_state.invalidate_all();
            return;
        }

        let (query_start_point, query_end_point) = (
            display_snapshot.display_point_to_point(
                DisplayPoint::new(DisplayRow(query_start_row), 0),
                Bias::Left,
            ),
            display_snapshot.display_point_to_point(
                DisplayPoint::new(DisplayRow(query_end_row), 0),
                Bias::Right,
            ),
        );

        let mut visible_ranges_by_excerpt: HashMap<ExcerptId, Vec<Range<usize>>> =
            HashMap::default();
        for (_, range, excerpt_id) in
            buffer_snapshot.range_to_buffer_ranges(visible_start_point..visible_end_point)
        {
            if !range.is_empty() {
                visible_ranges_by_excerpt
                    .entry(excerpt_id)
                    .or_default()
                    .push(range);
            }
        }

        if visible_ranges_by_excerpt.is_empty() {
            self.clear_rainbow_bracket_highlights(cx);
            self.rainbow_highlight_state.active_color_keys.clear();
            return;
        }

        let mut query_excerpts: HashMap<ExcerptId, QueryExcerpt<'_>> = HashMap::default();
        for (buffer, range, excerpt_id) in
            buffer_snapshot.range_to_buffer_ranges(query_start_point..query_end_point)
        {
            if range.is_empty() {
                continue;
            }
            query_excerpts
                .entry(excerpt_id)
                .and_modify(|entry| {
                    entry.range.start = entry.range.start.min(range.start);
                    entry.range.end = entry.range.end.max(range.end);
                })
                .or_insert(QueryExcerpt {
                    buffer_snapshot: buffer,
                    range,
                });
        }

        if query_excerpts.is_empty() {
            self.clear_rainbow_bracket_highlights(cx);
            self.rainbow_highlight_state.active_color_keys.clear();
            return;
        }

        let accent_colors = cx.theme().accents().clone();
        let color_count = accent_colors.0.len();
        if color_count == 0 {
            self.clear_rainbow_bracket_highlights(cx);
            self.rainbow_highlight_state.invalidate_all();
            return;
        }

        let mut aggregated_ranges = vec![Vec::new(); color_count];
        let mut had_any_highlights = false;

        for (excerpt_id, query_entry) in query_excerpts {
            let Some(visible_slices) = visible_ranges_by_excerpt.get(&excerpt_id) else {
                continue;
            };
            if visible_slices.is_empty() {
                continue;
            }

            let buffer_id = query_entry.buffer_snapshot.remote_id();
            let buffer_version = query_entry.buffer_snapshot.version().clone();

            if let Some(entry) = self.rainbow_highlight_state.cache.get(
                &excerpt_id,
                buffer_id,
                &buffer_version,
                visible_slices,
                color_count,
            ) {
                extend_color_ranges(&mut aggregated_ranges, &entry.ranges_by_color);
                had_any_highlights |= entry.has_highlights();
                continue;
            }

            let Some(ranges_by_color) = compute_excerpt_highlights(
                query_entry.buffer_snapshot,
                query_entry.range.clone(),
                visible_slices,
                excerpt_id,
                color_count,
            ) else {
                self.rainbow_highlight_state
                    .cache
                    .invalidate_excerpts(&[excerpt_id]);
                continue;
            };

            let entry = RainbowCacheEntry::new(
                buffer_id,
                buffer_version,
                visible_slices.clone(),
                color_count,
                ranges_by_color,
            );
            had_any_highlights |= entry.has_highlights();
            extend_color_ranges(&mut aggregated_ranges, &entry.ranges_by_color);
            self.rainbow_highlight_state.cache.insert(excerpt_id, entry);
        }

        if !had_any_highlights {
            self.clear_rainbow_bracket_highlights(cx);
            self.rainbow_highlight_state.active_color_keys.clear();
            return;
        }

        let mut used_keys = Vec::new();
        for (color_idx, ranges) in aggregated_ranges.iter_mut().enumerate() {
            if ranges.is_empty() {
                continue;
            }
            used_keys.push(color_idx);
            let style = HighlightStyle {
                color: Some(accent_colors.color_for_index(color_idx as u32)),
                ..Default::default()
            };
            let highlight_ranges = std::mem::take(ranges);
            self.highlight_text_key::<RainbowBracketHighlight>(
                color_idx,
                highlight_ranges,
                style,
                cx,
            );
        }

        let previous_keys: HashSet<_> = self
            .rainbow_highlight_state
            .active_color_keys
            .iter()
            .copied()
            .collect();
        let current_keys: HashSet<_> = used_keys.iter().copied().collect();
        for key in previous_keys.difference(&current_keys) {
            self.clear_highlight_key::<RainbowBracketHighlight>(*key, cx);
        }
        self.rainbow_highlight_state.active_color_keys = used_keys;
    }

    pub fn clear_rainbow_bracket_highlights(&mut self, cx: &mut Context<Self>) {
        let keys = std::mem::take(&mut self.rainbow_highlight_state.active_color_keys);
        for key in keys {
            self.clear_highlight_key::<RainbowBracketHighlight>(key, cx);
        }
    }
}

fn extend_color_ranges(target: &mut [Vec<Range<Anchor>>], source: &[Vec<Range<Anchor>>]) {
    for (idx, ranges) in source.iter().enumerate() {
        if ranges.is_empty() {
            continue;
        }
        if let Some(destination) = target.get_mut(idx) {
            destination.extend(ranges.iter().cloned());
        }
    }
}

fn compute_excerpt_highlights(
    buffer_snapshot: &BufferSnapshot,
    query_range: Range<usize>,
    visible_ranges: &[Range<usize>],
    excerpt_id: ExcerptId,
    color_count: usize,
) -> Option<Vec<Vec<Range<Anchor>>>> {
    if color_count == 0 || visible_ranges.is_empty() || query_range.is_empty() {
        return Some(vec![Vec::new(); color_count]);
    }

    let mut matches = buffer_snapshot.matches(query_range.clone(), |grammar| {
        grammar.rainbow_config().map(|config| &config.query)
    });
    let configs = matches
        .grammars()
        .iter()
        .map(|grammar| grammar.rainbow_config())
        .collect::<Vec<_>>();
    if configs.iter().all(|config| config.is_none()) {
        return None;
    }

    let mut level_ranges = vec![Vec::new(); color_count];
    let mut scope_stack = Vec::<ActiveScope>::new();

    while let Some(mat) = matches.peek() {
        let config = match configs.get(mat.grammar_index).and_then(|c| c.as_ref()) {
            Some(config) => config,
            None => {
                matches.advance();
                continue;
            }
        };

        let capture_start = mat
            .captures
            .iter()
            .map(|capture| capture.node.byte_range().start)
            .min()
            .unwrap_or(query_range.end);
        while let Some(scope) = scope_stack.last() {
            if capture_start < scope.end_byte {
                break;
            }
            scope_stack.pop();
        }

        if let Some(scope_capture_ix) = config.scope_capture_ix {
            for capture in mat
                .captures
                .iter()
                .filter(|capture| capture.index == scope_capture_ix)
            {
                let node = capture.node;
                let level = scope_stack.last().map_or(0, |scope| scope.level + 1);
                scope_stack.push(ActiveScope {
                    end_byte: node.end_byte(),
                    level,
                    node_id: config.patterns[mat.pattern_index]
                        .include_children
                        .then(|| node.id()),
                });
            }
        }

        for capture in mat
            .captures
            .iter()
            .filter(|capture| capture.index == config.bracket_capture_ix)
        {
            let Some(scope) = scope_stack.last() else {
                continue;
            };
            let node = capture.node;
            if let Some(scope_node_id) = scope.node_id {
                let Some(parent) = node.parent() else {
                    continue;
                };
                if parent.id() != scope_node_id {
                    continue;
                }
            }

            let byte_range = node.byte_range();
            if !is_range_visible(&byte_range, visible_ranges) {
                continue;
            }
            if byte_range.is_empty() {
                continue;
            }

            let color_index = scope.level % color_count;
            let anchor_range = Anchor::range_in_buffer(
                excerpt_id,
                buffer_snapshot.remote_id(),
                buffer_snapshot.anchor_after(byte_range.start)
                    ..buffer_snapshot.anchor_before(byte_range.end),
            );
            level_ranges[color_index].push(anchor_range);
        }

        matches.advance();
    }

    Some(level_ranges)
}

fn is_range_visible(range: &Range<usize>, visible_ranges: &[Range<usize>]) -> bool {
    visible_ranges.iter().any(|visible| {
        range.start >= visible.start && range.end <= visible.end && range.start < range.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use indoc::indoc;
    use language::{Buffer, Language, LanguageConfig, LanguageMatcher, LanguageQueries};
    use std::{borrow::Cow, sync::Arc};
    use text::Rope;

    fn javascript_test_language() -> Language {
        Language::new(
            LanguageConfig {
                name: "JavaScript".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["js".into()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
        )
        .with_queries(LanguageQueries {
            rainbow: Some(Cow::from(include_str!(
                "../../languages/src/javascript/rainbow.scm"
            ))),
            ..LanguageQueries::default()
        })
        .expect("failed to load rainbow query")
    }

    #[gpui::test]
    async fn computes_bracket_levels(cx: &mut TestAppContext) {
        let language = Arc::new(javascript_test_language());
        let rope = Rope::from(indoc! {
            r#"
            function demo() {
                const data = [{ value: (items[0]) }];
            }
            "#
        });

        let snapshot = Buffer::build_snapshot_sync(rope, Some(language), None, cx);
        let excerpt_id = ExcerptId::min();
        let visible = vec![0..snapshot.len()];
        let ranges =
            compute_excerpt_highlights(&snapshot, 0..snapshot.len(), &visible, excerpt_id, 4)
                .expect("missing rainbow config");

        let total_highlights: usize = ranges.iter().map(|r| r.len()).sum();
        assert!(
            total_highlights >= 8,
            "expected multiple highlighted brackets"
        );
        assert!(ranges[0].len() >= ranges[1].len());
    }
}
