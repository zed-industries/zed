use crate::{BufferSnapshot, Point, ToPoint, ToTreeSitterPoint};
use fuzzy_nucleo::{Case, LengthPenalty, StringMatch, StringMatchCandidate};
use gpui::{BackgroundExecutor, HighlightStyle, SharedString};
use std::ops::Range;

/// An outline of all the symbols contained in a buffer.
#[derive(Debug)]
pub struct Outline<T> {
    pub items: Vec<OutlineItem<T>>,
    /// Candidates contain the full path of each item, used for matching.
    candidates: Vec<StringMatchCandidate>,
    /// leaf_offsets stores the byte offset within that full path where the
    /// item's own text starts. Anything before this offset is ancestor
    /// path text used purely as match context.
    leaf_offsets: Vec<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OutlineItem<T> {
    pub depth: usize,
    pub range: Range<T>,
    pub source_range_for_text: Range<T>,
    pub text: SharedString,
    pub highlight_ranges: Vec<(Range<usize>, HighlightStyle)>,
    pub name_ranges: Vec<Range<usize>>,
    pub body_range: Option<Range<T>>,
    pub annotation_range: Option<Range<T>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolPath(pub SharedString);

/// Result of [`Outline::search`]. Real fuzzy matches are `Match`; `Ancestor`
/// rows are synthetic entries pointing at parent items, included so callers can
/// show the full path of each match (Even when the ancestor has been filtered
/// out due to not matching) but treat those synthetic ancestors differently
/// from an entry that actually matched (e.g. they are not eligible for
/// auto-selection).
#[derive(Clone, Debug)]
pub enum OutlineSearchEntry {
    Match(StringMatch),
    Ancestor { candidate_id: usize },
}

impl OutlineSearchEntry {
    pub fn candidate_id(&self) -> usize {
        match self {
            Self::Match(m) => m.candidate_id,
            Self::Ancestor { candidate_id } => *candidate_id,
        }
    }

    pub fn as_match(&self) -> Option<&StringMatch> {
        match self {
            Self::Match(m) => Some(m),
            Self::Ancestor { .. } => None,
        }
    }

    pub fn into_match(self) -> Option<StringMatch> {
        match self {
            Self::Match(m) => Some(m),
            Self::Ancestor { .. } => None,
        }
    }
}

impl<T: ToPoint> OutlineItem<T> {
    /// Converts to an equivalent outline item, but with parameterized over Points.
    pub fn to_point(&self, buffer: &BufferSnapshot) -> OutlineItem<Point> {
        OutlineItem {
            depth: self.depth,
            range: self.range.start.to_point(buffer)..self.range.end.to_point(buffer),
            source_range_for_text: self.source_range_for_text.start.to_point(buffer)
                ..self.source_range_for_text.end.to_point(buffer),
            text: self.text.clone(),
            highlight_ranges: self.highlight_ranges.clone(),
            name_ranges: self.name_ranges.clone(),
            body_range: self
                .body_range
                .as_ref()
                .map(|r| r.start.to_point(buffer)..r.end.to_point(buffer)),
            annotation_range: self
                .annotation_range
                .as_ref()
                .map(|r| r.start.to_point(buffer)..r.end.to_point(buffer)),
        }
    }

    pub fn body_range(&self, buffer: &BufferSnapshot) -> Option<Range<Point>> {
        if let Some(range) = self.body_range.as_ref() {
            return Some(range.start.to_point(buffer)..range.end.to_point(buffer));
        }

        let range = self.range.start.to_point(buffer)..self.range.end.to_point(buffer);
        let start_indent = buffer.indent_size_for_line(range.start.row);
        let node = buffer.syntax_ancestor(range.clone())?;

        let mut cursor = node.walk();
        loop {
            let node = cursor.node();
            if node.start_position() >= range.start.to_ts_point()
                && node.end_position() <= range.end.to_ts_point()
            {
                break;
            }
            // If we can't descend further, the current node is the most specific
            // ancestor that contains `range.start`. Bail out rather than spinning
            // forever re-checking the same node.
            if cursor
                .goto_first_child_for_point(range.start.to_ts_point())
                .is_none()
            {
                return None;
            }
        }

        if !cursor.goto_last_child() {
            return None;
        }
        let body_node = loop {
            let node = cursor.node();
            if node.child_count() > 0 {
                break node;
            }
            if !cursor.goto_previous_sibling() {
                return None;
            }
        };

        let mut start_row = body_node.start_position().row as u32;
        let mut end_row = body_node.end_position().row as u32;

        while start_row < end_row && buffer.indent_size_for_line(start_row) == start_indent {
            start_row += 1;
        }
        while start_row < end_row && buffer.indent_size_for_line(end_row - 1) == start_indent {
            end_row -= 1;
        }
        if start_row < end_row {
            return Some(Point::new(start_row, 0)..Point::new(end_row, 0));
        }
        None
    }
}

impl<T> Outline<T> {
    pub fn new(items: Vec<OutlineItem<T>>) -> Self {
        let mut candidates = Vec::with_capacity(items.len());
        let mut leaf_offsets = Vec::with_capacity(items.len());
        let mut path_text = String::new();
        let mut path_stack = Vec::new();

        for (id, item) in items.iter().enumerate() {
            if item.depth < path_stack.len() {
                path_stack.truncate(item.depth);
                path_text.truncate(path_stack.last().copied().unwrap_or(0));
            }
            if !path_text.is_empty() {
                path_text.push(' ');
            }
            leaf_offsets.push(path_text.len());
            path_text.push_str(&item.text);
            path_stack.push(path_text.len());
            candidates.push(StringMatchCandidate::new(id, &path_text));
        }

        Self {
            items,
            candidates,
            leaf_offsets,
        }
    }

    /// Find the most similar symbol to the provided query using normalized Levenshtein distance.
    pub fn find_most_similar(&self, query: &str) -> Option<(SymbolPath, &OutlineItem<T>)> {
        const SIMILARITY_THRESHOLD: f64 = 0.6;

        let (position, similarity) = self
            .candidates
            .iter()
            .enumerate()
            .map(|(index, candidate)| {
                let similarity = strsim::normalized_levenshtein(&candidate.string, query);
                (index, similarity)
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())?;

        if similarity >= SIMILARITY_THRESHOLD {
            self.candidates
                .get(position)
                .map(|candidate| SymbolPath(candidate.string.clone()))
                .zip(self.items.get(position))
        } else {
            None
        }
    }

    /// Find all outline symbols that match with the nucleo fuzzy matcher, ordered by tree position.
    /// Each real match is preceded by [`OutlineSearchEntry::Ancestor`] rows carrying parent
    /// `candidate_id`s, so callers can render tree context above the match.
    pub async fn search(
        &self,
        query: &str,
        executor: BackgroundExecutor,
    ) -> Vec<OutlineSearchEntry> {
        let query = query.trim_start();
        if query.is_empty() {
            return Vec::new();
        }
        let mut matches = fuzzy_nucleo::match_strings_async(
            &self.candidates,
            query,
            Case::Smart,
            LengthPenalty::On,
            100,
            &Default::default(),
            executor,
        )
        .await;
        matches.sort_unstable_by_key(|m| m.candidate_id);

        // Single-atom queries (no whitespace) require *all* matched chars to
        // land in the leaf — typing "drop" should only surface leaves that
        // actually contain "drop", not items whose ancestor path happens to.
        // We can rely on that behavior because nucleo prefers matches at the
        // end of the haystack, so the leafiest part of the candidate.
        //
        // Multi-atom queries (whitespace-separated) use the ancestor path
        // for scoping. Rows whose entire match landed in an ancestor are
        // kept as context, with empty positions and zero score, so
        // descendants of a matched container surface alongside it. The
        // picker's score-based auto-select skips them so they never steal
        // focus from a row with real highlights.
        let single_atom = !query.contains(char::is_whitespace);
        matches.retain_mut(|string_match| {
            let leaf_offset = self.leaf_offsets[string_match.candidate_id];
            let total = string_match.positions.len();
            string_match
                .positions
                .retain(|position| *position >= leaf_offset);
            let kept = string_match.positions.len();
            if single_atom && kept != total {
                return false;
            }
            if kept == 0 {
                string_match.score = 0.0;
            }
            for position in &mut string_match.positions {
                *position -= leaf_offset;
            }
            string_match
                .string
                .clone_from(&self.items[string_match.candidate_id].text);
            true
        });

        expand_tree(|i| self.items[i].depth, matches)
    }
}

/// Interleaves synthetic [`OutlineSearchEntry::Ancestor`] rows before each match so callers
/// can render the parent chain as tree context above the match.
///
/// `matches` must be sorted ascending by `candidate_id` (which is what
/// [`Outline::search`] produces), this is so that we preserve the tree
/// structure of the outline. `depth_at` returns the tree depth for the item at
/// a given candidate index. Ancestors that already appear earlier in the output
/// either as their own match or as an ancestor of an earlier match, are not
/// duplicated.
fn expand_tree(
    depth_at: impl Fn(usize) -> usize,
    matches: Vec<StringMatch>,
) -> Vec<OutlineSearchEntry> {
    debug_assert!(matches.is_sorted_by_key(|m| m.candidate_id));
    let mut out = Vec::with_capacity(matches.len());
    let mut prev_item_ix = 0;
    for string_match in matches {
        let insertion_ix = out.len();
        let mut cur_depth = depth_at(string_match.candidate_id);
        for ix in (prev_item_ix..string_match.candidate_id).rev() {
            if cur_depth == 0 {
                break;
            }
            if depth_at(ix) == cur_depth - 1 {
                out.insert(
                    insertion_ix,
                    OutlineSearchEntry::Ancestor { candidate_id: ix },
                );
                cur_depth -= 1;
            }
        }
        prev_item_ix = string_match.candidate_id + 1;
        out.push(OutlineSearchEntry::Match(string_match));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Buffer, rust_lang};
    use gpui::{AppContext as _, TestAppContext};

    #[gpui::test]
    fn test_body_range_hangs_when_outline_range_is_inside_leaf_node(cx: &mut TestAppContext) {
        let text = "fn main() { let completion = 1; }";
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(rust_lang(), cx));
        let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());
        let identifier_start = text.find("completion").unwrap() + 1;
        let identifier_end = identifier_start + 1;
        let range =
            snapshot.offset_to_point(identifier_start)..snapshot.offset_to_point(identifier_end);

        let item = OutlineItem {
            depth: 0,
            range: range.clone(),
            source_range_for_text: range,
            text: "completion".into(),
            highlight_ranges: Vec::new(),
            name_ranges: Vec::new(),
            body_range: None,
            annotation_range: None,
        };

        assert_eq!(item.body_range(&snapshot), None);
    }

    #[gpui::test]
    async fn test_entries_with_no_names(cx: &mut TestAppContext) {
        let outline = Outline::new(vec![
            OutlineItem {
                depth: 0,
                range: Point::new(0, 0)..Point::new(5, 0),
                source_range_for_text: Point::new(0, 0)..Point::new(0, 9),
                text: "class Foo".into(),
                highlight_ranges: vec![],
                name_ranges: vec![6..9],
                body_range: None,
                annotation_range: None,
            },
            OutlineItem {
                depth: 0,
                range: Point::new(2, 0)..Point::new(2, 7),
                source_range_for_text: Point::new(0, 0)..Point::new(0, 7),
                text: "private".into(),
                highlight_ranges: vec![],
                name_ranges: vec![],
                body_range: None,
                annotation_range: None,
            },
        ]);
        assert!(
            outline.search("", cx.executor()).await.is_empty(),
            "empty queries return no matches; the picker handles 'show all' itself",
        );
        assert_eq!(
            outline
                .search("foo", cx.executor())
                .await
                .into_iter()
                .filter_map(OutlineSearchEntry::into_match)
                .map(|m| m.string)
                .collect::<Vec<SharedString>>(),
            vec![SharedString::from("class Foo")],
            "'private' (empty name_ranges) is correctly excluded; only the matching 'class Foo' is returned",
        );
    }

    #[test]
    fn test_find_most_similar_with_low_similarity() {
        let outline = Outline::new(vec![
            OutlineItem {
                depth: 0,
                range: Point::new(0, 0)..Point::new(5, 0),
                source_range_for_text: Point::new(0, 0)..Point::new(0, 10),
                text: "fn process".into(),
                highlight_ranges: vec![],
                name_ranges: vec![3..10],
                body_range: None,
                annotation_range: None,
            },
            OutlineItem {
                depth: 0,
                range: Point::new(7, 0)..Point::new(12, 0),
                source_range_for_text: Point::new(0, 0)..Point::new(0, 20),
                text: "struct DataProcessor".into(),
                highlight_ranges: vec![],
                name_ranges: vec![7..20],
                body_range: None,
                annotation_range: None,
            },
        ]);
        assert_eq!(
            outline.find_most_similar("pub fn process"),
            Some((SymbolPath("fn process".into()), &outline.items[0]))
        );
        assert_eq!(
            outline.find_most_similar("async fn process"),
            Some((SymbolPath("fn process".into()), &outline.items[0])),
        );
        assert_eq!(
            outline.find_most_similar("struct Processor"),
            Some((SymbolPath("struct DataProcessor".into()), &outline.items[1]))
        );
        assert_eq!(outline.find_most_similar("struct User"), None);
        assert_eq!(outline.find_most_similar("struct"), None);
    }
}
