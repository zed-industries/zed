use crate::Editor;
use collections::HashMap;
use gpui::{App, Entity, WeakEntity};
use language::{Buffer, Point};
use std::ops::Range;
use std::sync::Arc;
use util::{paths::PathMatcher, rel_path::RelPath};

/// State for filtering locations in a multibuffer editor.
/// Stores the original unfiltered locations so that filters can be changed
/// without re-querying the LSP.
pub struct FilterableMultibufferState {
    /// Original unfiltered locations keyed by buffer
    original_locations: HashMap<Entity<Buffer>, Vec<Range<Point>>>,
    /// Cached paths for each buffer for efficient filtering
    buffer_paths: HashMap<Entity<Buffer>, Option<Arc<RelPath>>>,
    /// Title for the multibuffer
    title: String,
    /// Reference to the editor that owns this state
    editor: WeakEntity<Editor>,
    include_text: String,
    exclude_text: String,
    filters_enabled: bool,
}

impl FilterableMultibufferState {
    pub fn new(
        locations: HashMap<Entity<Buffer>, Vec<Range<Point>>>,
        title: String,
        editor: WeakEntity<Editor>,
        cx: &App,
    ) -> Self {
        let buffer_paths = locations
            .keys()
            .map(|buffer| {
                let path = buffer.read(cx).file().map(|f| f.path().clone());
                (buffer.clone(), path)
            })
            .collect();

        Self {
            original_locations: locations,
            buffer_paths,
            title,
            editor,
            include_text: String::new(),
            exclude_text: String::new(),
            filters_enabled: false,
        }
    }

    /// Returns the title of the multibuffer
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the original unfiltered locations
    pub fn original_locations(&self) -> &HashMap<Entity<Buffer>, Vec<Range<Point>>> {
        &self.original_locations
    }

    pub fn include_text(&self) -> &str {
        &self.include_text
    }

    pub fn exclude_text(&self) -> &str {
        &self.exclude_text
    }

    pub fn filters_enabled(&self) -> bool {
        self.filters_enabled
    }

    pub fn set_filters_enabled(&mut self, enabled: bool) {
        self.filters_enabled = enabled;
    }

    pub fn set_filter_texts(&mut self, include_text: String, exclude_text: String) {
        self.include_text = include_text;
        self.exclude_text = exclude_text;
    }

    /// Filters locations based on include/exclude patterns.
    /// Returns a new HashMap containing only the locations that match the filters.
    ///
    /// Filtering logic (matching project search):
    /// 1. If path matches exclude pattern → filter out
    /// 2. If no include patterns set → include all (non-excluded)
    /// 3. If path matches include pattern → include
    /// 4. Check path ancestors for partial matches
    pub fn filter_locations(
        &self,
        include: Option<&PathMatcher>,
        exclude: Option<&PathMatcher>,
    ) -> HashMap<Entity<Buffer>, Vec<Range<Point>>> {
        self.original_locations
            .iter()
            .filter_map(|(buffer, ranges)| {
                let path = self.buffer_paths.get(buffer)?.as_ref()?;

                if self.matches_filters(path, include, exclude) {
                    Some((buffer.clone(), ranges.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Check if a path matches the include/exclude filters.
    fn matches_filters(
        &self,
        path: &RelPath,
        include: Option<&PathMatcher>,
        exclude: Option<&PathMatcher>,
    ) -> bool {
        // Check exclude patterns first
        if let Some(exclude_matcher) = exclude {
            if exclude_matcher.is_match(path) {
                return false;
            }
        }

        // If no include patterns, include everything not excluded
        let Some(include_matcher) = include else {
            return true;
        };

        // Check include patterns - also check parent paths for partial matches
        let mut current_path = path.to_rel_path_buf();
        loop {
            if include_matcher.is_match(&current_path) {
                return true;
            }
            if !current_path.pop() {
                return false;
            }
        }
    }

    /// Returns the editor that owns this state
    pub fn editor(&self) -> &WeakEntity<Editor> {
        &self.editor
    }

    /// Returns the number of files in the original (unfiltered) locations
    pub fn original_file_count(&self) -> usize {
        self.original_locations.len()
    }

    /// Returns the total number of locations in the original (unfiltered) set
    pub fn original_location_count(&self) -> usize {
        self.original_locations.values().map(|v| v.len()).sum()
    }
}
