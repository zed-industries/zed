use super::{
    fold_map::{FoldPlaceholder, FoldSnapshot},
    inlay_map::{InlayEdit, InlaySnapshot},
};
use collections::{HashMap, HashSet};
use gpui::{AnyElement, App, Context, Entity};
use language::{Anchor, Buffer, BufferSnapshot, Edit, Point};
use multi_buffer::{ExcerptId, MultiBuffer, MultiBufferSnapshot, ToOffset, ToPoint};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{any::TypeId, ops::Range, sync::Arc};
use sum_tree::TreeMap;
use tree_sitter::{Query, QueryCapture, QueryCursor};
use ui::{Color, IntoElement, Label};
use util::ResultExt;

/// Defines how to find foldable regions using Tree-sitter queries
#[derive(Clone, Debug)]
pub struct FoldingQuery {
    pub query: Query,
    pub auto_fold: bool,                 // Should regions auto-fold on load?
    pub display_capture_ix: Option<u32>, // Which capture to show when folded
    pub action_capture_ix: Option<u32>,  // Which capture contains action data
    pub proximity_expand: bool,          // Expand when cursor is near?
}

/// Represents a foldable region found by queries
#[derive(Clone, Debug)]
pub struct SyntaxFold {
    pub id: SyntaxFoldId,
    pub range: Range<Anchor>,
    pub display_text: Option<String>,    // Text to show when folded
    pub action_data: Option<FoldAction>, // What happens on cmd+click
    pub proximity_expand: bool,
    pub auto_fold: bool,
    pub query_index: usize, // Which query created this fold
}

/// Unique identifier for a syntax fold
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SyntaxFoldId(usize);

/// Possible actions when clicking folded content
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FoldAction {
    OpenUrl(String),                   // Open URL in browser
    GoToFile(String),                  // Open file in editor
    RunCommand(String),                // Run system command
    Custom(String, serde_json::Value), // Extensibility point
}

/// Configuration for syntax folding
#[derive(Clone, Debug, Default)]
pub struct SyntaxFoldConfig {
    pub enabled: bool,
    pub auto_fold_on_open: bool,
    pub proximity_expand_distance: u32, // Characters away from fold to trigger expansion
}

/// Snapshot of the syntax fold state at a point in time
pub struct SyntaxFoldSnapshot {
    buffer_snapshot: MultiBufferSnapshot,
    folds: TreeMap<Range<Anchor>, SyntaxFold>,
    queries: Arc<Vec<FoldingQuery>>,
    config: SyntaxFoldConfig,
}

/// Map that tracks syntax-driven folds in the buffer
pub struct SyntaxFoldMap {
    buffer: Entity<MultiBuffer>,
    queries: Arc<Vec<FoldingQuery>>,
    config: SyntaxFoldConfig,
    // All detected syntax folds
    folds: TreeMap<Range<Anchor>, SyntaxFold>,
    next_fold_id: usize,
    // Track which folds are currently applied to the fold map
    applied_folds: HashMap<SyntaxFoldId, Range<Anchor>>,
    // Track which folds are temporarily expanded due to cursor proximity
    proximity_expanded: Arc<Mutex<HashSet<SyntaxFoldId>>>,
    // Track which folds have been manually toggled by the user
    user_toggled: Arc<Mutex<HashSet<SyntaxFoldId>>>,
}

impl SyntaxFoldMap {
    pub fn new(buffer: Entity<MultiBuffer>, cx: &mut App) -> Self {
        let mut this = Self {
            buffer: buffer.clone(),
            queries: Arc::new(Vec::new()),
            config: SyntaxFoldConfig::default(),
            folds: TreeMap::default(),
            next_fold_id: 0,
            applied_folds: HashMap::default(),
            proximity_expanded: Arc::new(Mutex::new(HashSet::default())),
            user_toggled: Arc::new(Mutex::new(HashSet::default())),
        };

        // Initialize queries from languages
        this.update_queries(cx);

        // Perform initial fold detection if auto-fold is enabled
        if this.config.enabled && this.config.auto_fold_on_open {
            this.detect_folds(cx);
        }

        this
    }

    pub fn snapshot(&self, buffer_snapshot: MultiBufferSnapshot) -> SyntaxFoldSnapshot {
        SyntaxFoldSnapshot {
            buffer_snapshot,
            folds: self.folds.clone(),
            queries: self.queries.clone(),
            config: self.config.clone(),
        }
    }

    /// Returns ranges that should be folded/unfolded based on syntax analysis
    pub fn sync(
        &mut self,
        buffer_snapshot: MultiBufferSnapshot,
        buffer_edits: Vec<Edit<Point>>,
        cx: &mut App,
    ) -> (Vec<Range<Anchor>>, Vec<Range<Anchor>>) {
        // Re-detect folds in edited regions
        for edit in &buffer_edits {
            self.detect_folds_in_range(edit.new.clone(), &buffer_snapshot, cx);
        }

        // Determine which folds to apply/remove
        let mut to_fold = Vec::new();
        let mut to_unfold = Vec::new();

        for (range, fold) in self.folds.iter() {
            let should_be_folded = fold.auto_fold
                && !self.user_toggled.lock().contains(&fold.id)
                && !self.proximity_expanded.lock().contains(&fold.id);

            let is_folded = self.applied_folds.contains_key(&fold.id);

            if should_be_folded && !is_folded {
                to_fold.push(range.clone());
                self.applied_folds.insert(fold.id, range.clone());
            } else if !should_be_folded && is_folded {
                to_unfold.push(range.clone());
                self.applied_folds.remove(&fold.id);
            }
        }

        (to_fold, to_unfold)
    }

    /// Update queries from the current languages in the buffer
    fn update_queries(&mut self, cx: &App) {
        let mut queries = Vec::new();
        let buffer = self.buffer.read(cx);

        for excerpt in buffer.excerpts() {
            if let Some(language) = excerpt.buffer.read(cx).language() {
                if let Some(folding_query_str) = language.folding_query() {
                    if let Ok(query) =
                        Query::new(&language.grammar().unwrap().ts_language, folding_query_str)
                    {
                        // Parse query metadata from captures
                        let mut auto_fold = false;
                        let mut display_capture_ix = None;
                        let mut action_capture_ix = None;
                        let mut proximity_expand = false;

                        for (ix, name) in query.capture_names().iter().enumerate() {
                            match name.as_str() {
                                "fold.auto" => auto_fold = true,
                                "fold.text" | "fold.display" => {
                                    display_capture_ix = Some(ix as u32)
                                }
                                "fold.action" | "fold.url" => action_capture_ix = Some(ix as u32),
                                "fold.proximity" => proximity_expand = true,
                                _ => {}
                            }
                        }

                        queries.push(FoldingQuery {
                            query,
                            auto_fold,
                            display_capture_ix,
                            action_capture_ix,
                            proximity_expand,
                        });
                    }
                }
            }
        }

        self.queries = Arc::new(queries);
        self.snapshot.queries = self.queries.clone();
    }

    /// Detect all folds in the buffer
    fn detect_folds(&mut self, cx: &mut App) {
        let buffer = self.buffer.read(cx);
        let snapshot = buffer.snapshot(cx);

        for (excerpt_id, excerpt) in snapshot.excerpts() {
            if let Some(tree) = excerpt.tree() {
                let excerpt_range = excerpt.range.to_point(&excerpt.buffer);
                self.detect_folds_in_excerpt(
                    excerpt_id,
                    &excerpt.buffer,
                    tree.root_node(),
                    excerpt_range,
                    cx,
                );
            }
        }
    }

    /// Detect folds in a specific range
    fn detect_folds_in_range(
        &mut self,
        range: Range<Point>,
        buffer_snapshot: &MultiBufferSnapshot,
        cx: &mut App,
    ) {
        // Remove existing folds in this range
        let range_anchors =
            buffer_snapshot.anchor_before(range.start)..buffer_snapshot.anchor_after(range.end);
        let mut folds_to_remove = Vec::new();

        for (fold_range, fold) in self.folds.iter() {
            if fold_range
                .start
                .cmp(&range_anchors.end, buffer_snapshot)
                .is_lt()
                && fold_range
                    .end
                    .cmp(&range_anchors.start, buffer_snapshot)
                    .is_gt()
            {
                folds_to_remove.push(fold.id);
            }
        }

        for fold_id in folds_to_remove {
            self.remove_fold(fold_id);
        }

        // Re-detect folds in affected excerpts
        for (excerpt_id, excerpt) in buffer_snapshot.excerpts() {
            let excerpt_range = excerpt.range.to_point(&excerpt.buffer);
            if excerpt_range.start < range.end && excerpt_range.end > range.start {
                if let Some(tree) = excerpt.tree() {
                    self.detect_folds_in_excerpt(
                        excerpt_id,
                        &excerpt.buffer,
                        tree.root_node(),
                        excerpt_range,
                        buffer_snapshot,
                        cx,
                    );
                }
            }
        }
    }

    /// Detect folds in a specific excerpt using tree-sitter queries
    fn detect_folds_in_excerpt(
        &mut self,
        excerpt_id: ExcerptId,
        buffer: &BufferSnapshot,
        root: Node,
        range: Range<Point>,
        cx: &mut App,
    ) {
        let mut cursor = QueryCursor::new();
        cursor.set_point_range(range.clone());

        for (query_index, folding_query) in self.queries.iter().enumerate() {
            let matches = cursor.matches(
                &folding_query.query,
                root,
                buffer
                    .as_rope()
                    .chunks_in_range(range.clone())
                    .collect::<String>()
                    .as_bytes(),
            );

            for match_ in matches {
                if let Some(fold) = self.create_fold_from_match(
                    match_.captures,
                    query_index,
                    folding_query,
                    excerpt_id,
                    buffer,
                    cx,
                ) {
                    // Store the fold
                    self.folds.insert(fold.range.clone(), fold);
                }
            }
        }
    }

    /// Create a SyntaxFold from a tree-sitter match
    fn create_fold_from_match(
        &mut self,
        captures: &[QueryCapture],
        query_index: usize,
        query: &FoldingQuery,
        excerpt_id: ExcerptId,
        buffer: &BufferSnapshot,
        cx: &App,
    ) -> Option<SyntaxFold> {
        // Find the main fold capture (should be the one without a suffix)
        let fold_capture = captures.iter().find(|c| {
            let name = query.query.capture_names()[c.index as usize].as_str();
            name == "fold" || name == "fold.auto"
        })?;

        let fold_node = fold_capture.node;
        let start = Point::new(
            fold_node.start_position().row as u32,
            fold_node.start_position().column as u32,
        );
        let end = Point::new(
            fold_node.end_position().row as u32,
            fold_node.end_position().column as u32,
        );

        // Convert to anchors in the multi-buffer
        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let start_anchor = buffer_snapshot.anchor_in_excerpt(excerpt_id, start)?;
        let end_anchor = buffer_snapshot.anchor_in_excerpt(excerpt_id, end)?;

        // Extract display text if specified
        let display_text = if let Some(display_ix) = query.display_capture_ix {
            captures
                .iter()
                .find(|c| c.index == display_ix)
                .and_then(|c| {
                    let node = c.node;
                    let start = node.start_byte();
                    let end = node.end_byte();
                    buffer.text_for_range(start..end).collect::<String>().into()
                })
        } else {
            None
        };

        // Extract action data if specified
        let action_data = if let Some(action_ix) = query.action_capture_ix {
            captures
                .iter()
                .find(|c| c.index == action_ix)
                .and_then(|c| {
                    let node = c.node;
                    let start = node.start_byte();
                    let end = node.end_byte();
                    let text = buffer.text_for_range(start..end).collect::<String>();

                    // Parse action based on capture name
                    let capture_name = &query.query.capture_names()[c.index as usize];
                    match capture_name.as_str() {
                        "fold.url" => Some(FoldAction::OpenUrl(text)),
                        "fold.file" => Some(FoldAction::GoToFile(text)),
                        "fold.command" => Some(FoldAction::RunCommand(text)),
                        _ => None,
                    }
                })
        } else {
            None
        };

        let fold_id = SyntaxFoldId(self.next_fold_id);
        self.next_fold_id += 1;

        Some(SyntaxFold {
            id: fold_id,
            range: start_anchor..end_anchor,
            display_text,
            action_data,
            proximity_expand: query.proximity_expand,
            auto_fold: query.auto_fold,
            query_index,
        })
    }

    /// Create a fold placeholder for syntax folds
    pub fn create_fold_placeholder(fold: &SyntaxFold) -> FoldPlaceholder {
        let display_text = fold.display_text.clone();
        FoldPlaceholder {
            render: Arc::new(move |_id, _range, _cx| {
                if let Some(text) = &display_text {
                    // Render the display text as a clickable element
                    Label::new(text.clone())
                        .color(Color::Accent)
                        .into_any_element()
                } else {
                    // Default ellipsis
                    Label::new("…").color(Color::Muted).into_any_element()
                }
            }),
            constrain_width: true,
            merge_adjacent: false,
            type_tag: Some(TypeId::of::<SyntaxFold>()),
        }
    }

    /// Remove a syntax fold
    fn remove_fold(&mut self, fold_id: SyntaxFoldId) {
        let mut range_to_remove = None;
        for (range, fold) in self.folds.iter() {
            if fold.id == fold_id {
                range_to_remove = Some(range.clone());
                break;
            }
        }

        if let Some(range) = range_to_remove {
            self.folds.remove(&range);
            self.applied_folds.remove(&fold_id);
        }
    }

    /// Handle cursor movement for proximity-based expansion
    pub fn handle_cursor_moved(
        &mut self,
        cursor_offset: usize,
        buffer_snapshot: &MultiBufferSnapshot,
    ) -> (Vec<Range<Anchor>>, Vec<Range<Anchor>>) {
        let cursor_point = cursor_offset.to_point(buffer_snapshot);
        let mut to_expand = Vec::new();
        let mut to_collapse = Vec::new();

        // Check each fold for proximity
        for (range, fold) in self.folds.iter() {
            if !fold.proximity_expand {
                continue;
            }

            let fold_start = range.start.to_point(buffer_snapshot);
            let fold_end = range.end.to_point(buffer_snapshot);

            // Check if cursor is near the fold
            let near_fold = Self::is_point_near_range(
                cursor_point,
                fold_start..fold_end,
                self.config.proximity_expand_distance,
            );

            let is_expanded = self.proximity_expanded.lock().contains(&fold.id);

            if near_fold && !is_expanded {
                // Mark for expansion
                self.proximity_expanded.lock().insert(fold.id);
                to_expand.push(range.clone());
            } else if !near_fold && is_expanded {
                // Mark for collapse
                self.proximity_expanded.lock().remove(&fold.id);
                to_collapse.push(range.clone());
            }
        }

        (to_expand, to_collapse)
    }

    /// Check if a point is near a range
    fn is_point_near_range(point: Point, range: Range<Point>, distance: u32) -> bool {
        // Check if point is within the range
        if point >= range.start && point <= range.end {
            return true;
        }

        // Check distance before range
        if point.row == range.start.row
            && range.start.column.saturating_sub(point.column) <= distance
        {
            return true;
        }

        // Check distance after range
        if point.row == range.end.row && point.column.saturating_sub(range.end.column) <= distance {
            return true;
        }

        false
    }

    /// Get fold at a specific position
    pub fn fold_at_position(
        &self,
        offset: usize,
        buffer_snapshot: &MultiBufferSnapshot,
    ) -> Option<&SyntaxFold> {
        let point = offset.to_point(buffer_snapshot);

        for (range, fold) in self.folds.iter() {
            let start = range.start.to_point(buffer_snapshot);
            let end = range.end.to_point(buffer_snapshot);

            if point >= start && point <= end {
                return Some(fold);
            }
        }

        None
    }

    /// Execute the action associated with a fold
    pub fn execute_fold_action(&self, fold: &SyntaxFold, cx: &mut App) {
        if let Some(action) = &fold.action_data {
            match action {
                FoldAction::OpenUrl(url) => {
                    // Use platform open to open URLs
                    cx.open_url(url.as_str());
                }
                FoldAction::GoToFile(path) => {
                    // This would be handled by the workspace
                    log::info!("Go to file: {}", path);
                }
                FoldAction::RunCommand(cmd) => {
                    log::info!("Run command: {}", cmd);
                }
                FoldAction::Custom(name, data) => {
                    log::info!("Custom action {}: {:?}", name, data);
                }
            }
        }
    }
}

impl Clone for SyntaxFoldSnapshot {
    fn clone(&self) -> Self {
        Self {
            buffer_snapshot: self.buffer_snapshot.clone(),
            folds: self.folds.clone(),
            queries: self.queries.clone(),
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Context as _, TestAppContext};
    use language::{Buffer, Language, LanguageConfig, LanguageMatcher};
    use multi_buffer::MultiBuffer;
    use std::sync::Arc;

    // Note: These tests are placeholders showing the intended API.
    // Real tests would require actual tree-sitter language support.

    #[gpui::test]
    async fn test_syntax_fold_api(cx: &mut TestAppContext) {
        cx.update(|cx| {
            // Create a buffer
            let buffer = cx.new_model(|cx| Buffer::local("[Example](https://example.com)", cx));

            let multibuffer = cx.new_model(|cx| {
                let mut multibuffer = MultiBuffer::new(0, language::Capability::ReadWrite);
                multibuffer.push_buffer(buffer.clone(), cx);
                multibuffer
            });

            // Create syntax fold map
            let mut syntax_fold_map = SyntaxFoldMap::new(multibuffer.clone(), cx);

            // Test basic API
            syntax_fold_map.config.enabled = true;
            syntax_fold_map.detect_folds(cx);

            // Verify the API works
            let buffer_snapshot = multibuffer.read(cx).snapshot(cx);
            let snapshot = syntax_fold_map.snapshot(buffer_snapshot);
            assert_eq!(snapshot.folds.len(), 0); // No folds without proper tree-sitter
        });
    }
}
