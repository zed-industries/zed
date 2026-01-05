#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Reorder selected groups of edits (additions & deletions) into a new patch.
///
/// Intuition:
/// Think of the original patch as a timeline of atomic edit indices (0..N),
/// where one edit is one deleted or inserted line.
/// This function recombines these edits into a new patch which can be thought
/// of as a sequence of patches.
///
/// You provide `edits_order` describing logical chunks (e.g., "write a feature",
/// "refactor", "add tests"). For each group the function:
///  1. Extracts those edits
///  2. Appends them to the output patch
///  3. Removes them from an internal remainder so subsequent original indices
///     still point to the right (yet-to-be-extracted) edits.
///
/// The returned `Patch` contains only the edits you listed, emitted group by
/// group. The leftover remainder is discarded.
///
/// Parameters:
/// * `patch` - Source patch
/// * `edits_order` - Vector of sets of original (0-based) edit indexes
///
/// Returns:
/// * A new `Patch` containing the grouped edits in the requested order.
///
/// Example:
/// ```rust
/// use std::collections::BTreeSet;
/// use reorder_patch::{Patch, reorder_edits};
///
/// // Edits (indexes): 0:-old, 1:+new, 2:-old2, 3:+new2, 4:+added
/// let diff = "\
/// --- a/a.txt
/// +++ b/a.txt
/// @@ -1,3 +1,3 @@
///  one
/// -old
/// +new
///  end
/// @@ -5,3 +5,4 @@
///  tail
/// -old2
/// +new2
/// +added
///  fin
/// ";
/// let patch = Patch::parse_unified_diff(diff);
///
/// // First take the part of the second hunk's edits (2),
/// // then the first hunk (0,1), then the rest of the second hunk (3,4)
/// let order = vec![BTreeSet::from([2]), BTreeSet::from([0, 1]), BTreeSet::from([3, 4])];
/// let reordered = reorder_edits(&patch, order);
/// println!("{}", reordered.to_string());
/// ```
pub fn reorder_edits(patch: &Patch, edits_order: Vec<BTreeSet<usize>>) -> Patch {
    let mut result = Patch {
        header: patch.header.clone(),
        hunks: Vec::new(),
    };

    let mut remainder = patch.clone();

    // Indexes in `edits_order` will shift as we apply edits.
    // This structure maps the original index to the actual index.
    let stats = patch.stats();
    let total_edits = stats.added + stats.removed;
    let mut indexes_map = BTreeMap::from_iter((0..total_edits).map(|i| (i, Some(i))));

    for patch_edits_order in edits_order {
        // Skip duplicated indexes that were already processed
        let patch_edits_order = patch_edits_order
            .into_iter()
            .filter(|&i| indexes_map[&i].is_some()) // skip duplicated indexes
            .collect::<BTreeSet<_>>();

        if patch_edits_order.is_empty() {
            continue;
        }

        let order = patch_edits_order
            .iter()
            .map(|&i| {
                indexes_map[&i].unwrap_or_else(|| panic!("Edit index {i} has been already used. Perhaps your spec contains duplicates"))
            })
            .collect::<BTreeSet<_>>();

        let extracted;
        (extracted, remainder) = extract_edits(&remainder, &order);

        result.hunks.extend(extracted.hunks);

        // Update indexes_map to reflect applied edits. For example:
        //
        // Original_index | Removed?  | Mapped_value
        //       0        | false     | 0
        //       1        | true      | None
        //       2        | true      | None
        //       3        | false     | 1

        for index in patch_edits_order {
            indexes_map.insert(index, None);
            for j in (index + 1)..total_edits {
                if let Some(val) = indexes_map[&j] {
                    indexes_map.insert(j, Some(val - 1));
                }
            }
        }
    }

    result
}

/// Split a patch into (extracted, remainder) based on a set of edit indexes.
/// The first returned patch contains only the chosen edits; the second contains
/// everything else with those edits applied (converted into context).
pub fn extract_edits(patch: &Patch, edit_indexes: &BTreeSet<usize>) -> (Patch, Patch) {
    let mut extracted = patch.clone();
    let mut remainder = patch.clone();

    let stats = patch.stats();
    let num_edits = stats.added + stats.removed;
    let this_edits = edit_indexes.iter().cloned().collect::<Vec<_>>();
    let other_edits = (0..num_edits)
        .filter(|i| !edit_indexes.contains(i))
        .collect();

    remove_edits(&mut extracted, other_edits);
    apply_edits(&mut remainder, this_edits);

    (extracted, remainder)
}

#[derive(Debug, Default, Clone)]
pub struct Patch {
    pub header: String,
    pub hunks: Vec<Hunk>,
}

pub struct DiffStats {
    pub added: usize,
    pub removed: usize,
}

impl ToString for Patch {
    fn to_string(&self) -> String {
        let mut result = self.header.clone();
        for hunk in &self.hunks {
            let current_file = hunk.filename.clone();
            result.push_str(&format!("--- a/{}\n", current_file));
            result.push_str(&format!("+++ b/{}\n", current_file));
            result.push_str(&hunk.to_string());
        }

        result
    }
}

impl Patch {
    /// Parse a unified diff (git style) string into a `Patch`.
    pub fn parse_unified_diff(unified_diff: &str) -> Patch {
        let mut current_file = String::new();
        let mut is_filename_inherited = false;
        let mut hunk = Hunk::default();
        let mut patch = Patch::default();
        let mut in_header = true;

        for line in unified_diff.lines() {
            if line.starts_with("--- ") || line.starts_with("+++ ") || line.starts_with("@@") {
                in_header = false;
            }

            if in_header {
                patch.header.push_str(format!("{}\n", &line).as_ref());
                continue;
            }

            if line.starts_with("@@") {
                if !hunk.lines.is_empty() {
                    patch.hunks.push(hunk);
                }
                hunk = Hunk::from_header(line, &current_file, is_filename_inherited);
                is_filename_inherited = true;
            } else if let Some(path) = line.strip_prefix("--- ") {
                is_filename_inherited = false;
                current_file = path.trim().strip_prefix("a/").unwrap_or(path).into();
            } else if let Some(path) = line.strip_prefix("+++ ") {
                is_filename_inherited = false;
                current_file = path.trim().strip_prefix("b/").unwrap_or(path).into();
            } else if let Some(line) = line.strip_prefix("+") {
                hunk.lines.push(PatchLine::Addition(line.to_string()));
            } else if let Some(line) = line.strip_prefix("-") {
                hunk.lines.push(PatchLine::Deletion(line.to_string()));
            } else if let Some(line) = line.strip_prefix(" ") {
                hunk.lines.push(PatchLine::Context(line.to_string()));
            } else {
                hunk.lines.push(PatchLine::Garbage(line.to_string()));
            }
        }

        if !hunk.lines.is_empty() {
            patch.hunks.push(hunk);
        }

        let header_lines = patch.header.lines().collect::<Vec<&str>>();
        let len = header_lines.len();
        if len >= 2 {
            if header_lines[len - 2].starts_with("diff --git")
                && header_lines[len - 1].starts_with("index ")
            {
                patch.header = header_lines[..len - 2].join("\n") + "\n";
            }
        }
        if patch.header.trim().is_empty() {
            patch.header = String::new();
        }

        patch
    }

    /// Drop hunks that contain no additions or deletions.
    pub fn remove_empty_hunks(&mut self) {
        self.hunks.retain(|hunk| {
            hunk.lines
                .iter()
                .any(|line| matches!(line, PatchLine::Addition(_) | PatchLine::Deletion(_)))
        });
    }

    /// Make sure there are no more than `context_lines` lines of context around each change.
    pub fn normalize_hunks(&mut self, context_lines: usize) {
        for hunk in &mut self.hunks {
            // Find indices of all changes (additions and deletions)
            let change_indices: Vec<usize> = hunk
                .lines
                .iter()
                .enumerate()
                .filter_map(|(i, line)| match line {
                    PatchLine::Addition(_) | PatchLine::Deletion(_) => Some(i),
                    _ => None,
                })
                .collect();

            // If there are no changes, clear the hunk (it's all context)
            if change_indices.is_empty() {
                hunk.lines.clear();
                hunk.old_count = 0;
                hunk.new_count = 0;
                continue;
            }

            // Determine the range to keep
            let first_change = change_indices[0];
            let last_change = change_indices[change_indices.len() - 1];

            let start = first_change.saturating_sub(context_lines);
            let end = (last_change + context_lines + 1).min(hunk.lines.len());

            // Count lines trimmed from the beginning
            let (old_lines_before, new_lines_before) = count_lines(&hunk.lines[0..start]);

            // Keep only the lines in range + garbage
            let garbage_before = hunk.lines[..start]
                .iter()
                .filter(|line| matches!(line, PatchLine::Garbage(_)));
            let garbage_after = hunk.lines[end..]
                .iter()
                .filter(|line| matches!(line, PatchLine::Garbage(_)));

            hunk.lines = garbage_before
                .chain(hunk.lines[start..end].iter())
                .chain(garbage_after)
                .cloned()
                .collect();

            // Update hunk header
            let (old_count, new_count) = count_lines(&hunk.lines);
            hunk.old_start += old_lines_before as isize;
            hunk.new_start += new_lines_before as isize;
            hunk.old_count = old_count as isize;
            hunk.new_count = new_count as isize;
        }
    }

    /// Count total added and removed lines
    pub fn stats(&self) -> DiffStats {
        let mut added = 0;
        let mut removed = 0;

        for hunk in &self.hunks {
            for line in &hunk.lines {
                match line {
                    PatchLine::Addition(_) => added += 1,
                    PatchLine::Deletion(_) => removed += 1,
                    _ => {}
                }
            }
        }

        DiffStats { added, removed }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Hunk {
    pub old_start: isize,
    pub old_count: isize,
    pub new_start: isize,
    pub new_count: isize,
    pub comment: String,
    pub filename: String,
    pub is_filename_inherited: bool,
    pub lines: Vec<PatchLine>,
}

impl ToString for Hunk {
    fn to_string(&self) -> String {
        let header = self.header_string();
        let lines = self
            .lines
            .iter()
            .map(|line| line.to_string() + "\n")
            .collect::<Vec<String>>()
            .join("");
        format!("{header}\n{lines}")
    }
}

impl Hunk {
    /// Render the hunk header
    pub fn header_string(&self) -> String {
        format!(
            "@@ -{},{} +{},{} @@ {}",
            self.old_start,
            self.old_count,
            self.new_start,
            self.new_count,
            self.comment.clone()
        )
        .trim_end()
        .into()
    }

    /// Create a `Hunk` from a raw header line and associated filename.
    pub fn from_header(header: &str, filename: &str, is_filename_inherited: bool) -> Self {
        let (old_start, old_count, new_start, new_count, comment) = Self::parse_hunk_header(header);
        Self {
            old_start,
            old_count,
            new_start,
            new_count,
            comment,
            filename: filename.to_string(),
            is_filename_inherited,
            lines: Vec::new(),
        }
    }

    /// Parse hunk headers like `@@ -3,2 +3,2 @@ some garbage"
    fn parse_hunk_header(line: &str) -> (isize, isize, isize, isize, String) {
        let header_part = line.trim_start_matches("@@").trim();
        let parts: Vec<&str> = header_part.split_whitespace().collect();

        if parts.len() < 2 {
            return (0, 0, 0, 0, String::new());
        }

        let old_part = parts[0].trim_start_matches('-');
        let new_part = parts[1].trim_start_matches('+');

        let (old_start, old_count) = Hunk::parse_hunk_header_range(old_part);
        let (new_start, new_count) = Hunk::parse_hunk_header_range(new_part);

        let comment = if parts.len() > 2 {
            parts[2..]
                .join(" ")
                .trim_start_matches("@@")
                .trim()
                .to_string()
        } else {
            String::new()
        };

        (
            old_start as isize,
            old_count as isize,
            new_start as isize,
            new_count as isize,
            comment,
        )
    }

    fn parse_hunk_header_range(part: &str) -> (usize, usize) {
        let (old_start, old_count) = if part.contains(',') {
            let old_parts: Vec<&str> = part.split(',').collect();
            (
                old_parts[0].parse().unwrap_or(0),
                old_parts[1].parse().unwrap_or(0),
            )
        } else {
            (part.parse().unwrap_or(0), 1)
        };
        (old_start, old_count)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PatchLine {
    Context(String),
    Addition(String),
    Deletion(String),
    HunkHeader(usize, usize, usize, usize, String),
    FileStartMinus(String),
    FileStartPlus(String),
    Garbage(String),
}

impl PatchLine {
    pub fn parse(line: &str) -> Self {
        if let Some(line) = line.strip_prefix("+") {
            Self::Addition(line.to_string())
        } else if let Some(line) = line.strip_prefix("-") {
            Self::Deletion(line.to_string())
        } else if let Some(line) = line.strip_prefix(" ") {
            Self::Context(line.to_string())
        } else {
            Self::Garbage(line.to_string())
        }
    }
}

impl ToString for PatchLine {
    fn to_string(&self) -> String {
        match self {
            PatchLine::Context(line) => format!(" {}", line),
            PatchLine::Addition(line) => format!("+{}", line),
            PatchLine::Deletion(line) => format!("-{}", line),
            PatchLine::HunkHeader(old_start, old_end, new_start, new_end, comment) => format!(
                "@@ -{},{} +{},{} @@ {}",
                old_start, old_end, new_start, new_end, comment
            )
            .trim_end()
            .into(),
            PatchLine::FileStartMinus(filename) => format!("--- {}", filename),
            PatchLine::FileStartPlus(filename) => format!("+++ {}", filename),
            PatchLine::Garbage(line) => line.to_string(),
        }
    }
}

///
/// Removes specified edits from a patch by their indexes and adjusts line numbers accordingly.
///
/// This function removes edits (additions and deletions) from the patch as they never were made.
/// The resulting patch is adjusted to maintain correctness.
///
/// # Arguments
///
/// * `patch` - A patch to modify
/// * `edit_indexes` - A vector of edit indexes to remove (0-based, counting only additions and deletions)
/// ```
pub fn remove_edits(patch: &mut Patch, edit_indexes: Vec<usize>) {
    let mut current_edit_index: isize = -1;
    let mut new_start_delta_by_file: HashMap<String, isize> = HashMap::new();

    for hunk in &mut patch.hunks {
        if !hunk.is_filename_inherited {
            new_start_delta_by_file.insert(hunk.filename.clone(), 0);
        }
        let delta = new_start_delta_by_file
            .entry(hunk.filename.clone())
            .or_insert(0);
        hunk.new_start += *delta;

        hunk.lines = hunk
            .lines
            .drain(..)
            .filter_map(|line| {
                let is_edit = matches!(line, PatchLine::Addition(_) | PatchLine::Deletion(_));
                if is_edit {
                    current_edit_index += 1;
                    if !edit_indexes.contains(&(current_edit_index as usize)) {
                        return Some(line);
                    }
                }
                match line {
                    PatchLine::Addition(_) => {
                        hunk.new_count -= 1;
                        *delta -= 1;
                        None
                    }
                    PatchLine::Deletion(content) => {
                        hunk.new_count += 1;
                        *delta += 1;
                        Some(PatchLine::Context(content))
                    }
                    _ => Some(line),
                }
            })
            .collect();
    }

    patch.normalize_hunks(3);
    patch.remove_empty_hunks();
}

///
/// Apply specified edits in the patch.
///
/// This generates another patch that looks like selected edits are already made
/// and became part of the context
///
/// See also: `remove_edits()`
///
pub fn apply_edits(patch: &mut Patch, edit_indexes: Vec<usize>) {
    let mut current_edit_index: isize = -1;
    let mut delta_by_file: HashMap<String, isize> = HashMap::new();

    for hunk in &mut patch.hunks {
        if !hunk.is_filename_inherited {
            delta_by_file.insert(hunk.filename.clone(), 0);
        }
        let delta = delta_by_file.entry(hunk.filename.clone()).or_insert(0);
        hunk.old_start += *delta;

        hunk.lines = hunk
            .lines
            .drain(..)
            .filter_map(|line| {
                let is_edit = matches!(line, PatchLine::Addition(_) | PatchLine::Deletion(_));
                if is_edit {
                    current_edit_index += 1;
                    if !edit_indexes.contains(&(current_edit_index as usize)) {
                        return Some(line);
                    }
                }
                match line {
                    PatchLine::Addition(content) => {
                        hunk.old_count += 1;
                        *delta += 1;
                        Some(PatchLine::Context(content))
                    }
                    PatchLine::Deletion(_) => {
                        hunk.old_count -= 1;
                        *delta -= 1;
                        None
                    }
                    _ => Some(line),
                }
            })
            .collect();
    }

    patch.normalize_hunks(3);
    patch.remove_empty_hunks();
}

/// Parse an order specification text into groups of edit indexes.
/// Supports numbers, ranges (a-b), commas, comments starting with `//`, and blank lines.
///
/// # Example spec
///
/// // Add new dependency
/// 1, 49
///
/// // Add new imports and types
/// 8-9, 51
///
/// // Add new struct and methods
/// 10-47
///
/// // Update tests
/// 48, 50
///
pub fn parse_order_spec(spec: &str) -> Vec<BTreeSet<usize>> {
    let mut order = Vec::new();

    for line in spec.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        // Parse the line into a BTreeSet
        let mut set = BTreeSet::new();

        for part in line.split(',') {
            let part = part.trim();

            if part.contains('-') {
                // Handle ranges like "8-9" or "10-47"
                let range_parts: Vec<&str> = part.split('-').collect();
                if range_parts.len() == 2 {
                    if let (Ok(start), Ok(end)) = (
                        range_parts[0].parse::<usize>(),
                        range_parts[1].parse::<usize>(),
                    ) {
                        for i in start..=end {
                            set.insert(i);
                        }
                    } else {
                        eprintln!("Warning: Invalid range format '{}'", part);
                    }
                } else {
                    eprintln!("Warning: Invalid range format '{}'", part);
                }
            } else {
                // Handle single numbers
                if let Ok(num) = part.parse::<usize>() {
                    set.insert(num);
                } else {
                    eprintln!("Warning: Invalid number format '{}'", part);
                }
            }
        }

        if !set.is_empty() {
            order.push(set);
        }
    }

    order
}

#[derive(Debug, Eq, PartialEq)]
pub struct EditLocation {
    pub filename: String,
    pub source_line_number: usize,
    pub target_line_number: usize,
    pub patch_line: PatchLine,
    pub hunk_index: usize,
    pub line_index_within_hunk: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum EditType {
    Deletion,
    Insertion,
}

pub fn locate_edited_line(patch: &Patch, mut edit_index: isize) -> Option<EditLocation> {
    let mut edit_locations = vec![];

    for (hunk_index, hunk) in patch.hunks.iter().enumerate() {
        let mut old_line_number = hunk.old_start;
        let mut new_line_number = hunk.new_start;
        for (line_index, line) in hunk.lines.iter().enumerate() {
            if matches!(line, PatchLine::Context(_)) {
                old_line_number += 1;
                new_line_number += 1;
                continue;
            }

            if !matches!(line, PatchLine::Addition(_) | PatchLine::Deletion(_)) {
                continue;
            }

            // old  new
            //  1    1       context
            //  2    2       context
            //  3    3      -deleted
            //  4    3      +insert
            //  4    4       more context
            //
            // old   new
            //  1     1      context
            //  2     2      context
            //  3     3     +inserted
            //  3     4      more context
            //
            // old  new
            //  1    1      -deleted
            //
            // old  new
            //  1    1       context
            //  2    2       context
            //  3    3      -deleted
            //  4    3       more context

            edit_locations.push(EditLocation {
                filename: hunk.filename.clone(),
                source_line_number: old_line_number as usize,
                target_line_number: new_line_number as usize,
                patch_line: line.clone(),
                hunk_index,
                line_index_within_hunk: line_index,
            });

            match line {
                PatchLine::Addition(_) => new_line_number += 1,
                PatchLine::Deletion(_) => old_line_number += 1,
                PatchLine::Context(_) => (),
                _ => (),
            };
        }
    }

    if edit_index < 0 {
        edit_index += edit_locations.len() as isize; // take from end
    }
    (0..edit_locations.len())
        .contains(&(edit_index as usize))
        .then(|| edit_locations.swap_remove(edit_index as usize)) // remove to take ownership
}
//
// Helper function to count old and new lines
fn count_lines(lines: &[PatchLine]) -> (usize, usize) {
    lines.iter().fold((0, 0), |(old, new), line| match line {
        PatchLine::Context(_) => (old + 1, new + 1),
        PatchLine::Deletion(_) => (old + 1, new),
        PatchLine::Addition(_) => (old, new + 1),
        _ => (old, new),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_unified_diff() {
        let patch_str = indoc! {"
            Patch header
            ============

            diff --git a/text.txt b/text.txt
            index 86c770d..a1fd855 100644
            --- a/text.txt
            +++ b/text.txt
            @@ -1,7 +1,7 @@
             azuere
             beige
             black
            -blue
            +dark blue
             brown
             cyan
             gold

            Some garbage

            diff --git a/second.txt b/second.txt
            index 86c770d..a1fd855 100644
            --- a/second.txt
            +++ b/second.txt
            @@ -9,6 +9,7 @@ gray
             green
             indigo
             magenta
            +silver
             orange
             pink
             purple
            diff --git a/text.txt b/text.txt
            index 86c770d..a1fd855 100644
            --- a/text.txt
            +++ b/text.txt
            @@ -16,4 +17,3 @@ red
             violet
             white
             yellow
            -zinc
        "};
        let patch = Patch::parse_unified_diff(patch_str);

        assert_eq!(patch.header, "Patch header\n============\n\n");
        assert_eq!(patch.hunks.len(), 3);
        assert_eq!(patch.hunks[0].header_string(), "@@ -1,7 +1,7 @@");
        assert_eq!(patch.hunks[1].header_string(), "@@ -9,6 +9,7 @@ gray");
        assert_eq!(patch.hunks[2].header_string(), "@@ -16,4 +17,3 @@ red");
        assert_eq!(patch.hunks[0].is_filename_inherited, false);
        assert_eq!(patch.hunks[1].is_filename_inherited, false);
        assert_eq!(patch.hunks[2].is_filename_inherited, false);
    }

    #[test]
    fn test_locate_edited_line() {
        let patch_str = indoc! {"
            Patch header
            ============

            diff --git a/text.txt b/text.txt
            index 86c770d..a1fd855 100644
            --- a/text.txt
            +++ b/text.txt
            @@ -1,7 +1,7 @@
             azuere
             beige
             black
            -blue
            +dark blue
             brown
             cyan
             gold
            diff --git a/second.txt b/second.txt
            index 86c770d..a1fd855 100644
            --- a/second.txt
            +++ b/second.txt
            @@ -9,6 +9,7 @@ gray
             green
             indigo
             magenta
            +silver
             orange
             pink
             purple
            diff --git a/text.txt b/text.txt
            index 86c770d..a1fd855 100644
            --- a/text.txt
            +++ b/text.txt
            @@ -16,4 +17,3 @@ red
             violet
             white
             yellow
            -zinc
        "};
        let patch = Patch::parse_unified_diff(patch_str);

        assert_eq!(
            locate_edited_line(&patch, 0), // -blue
            Some(EditLocation {
                filename: "text.txt".to_string(),
                source_line_number: 4,
                target_line_number: 4,
                patch_line: PatchLine::Deletion("blue".to_string()),
                hunk_index: 0,
                line_index_within_hunk: 3
            })
        );
        assert_eq!(
            locate_edited_line(&patch, 1), // +dark blue
            Some(EditLocation {
                filename: "text.txt".to_string(),
                source_line_number: 5,
                target_line_number: 4,
                patch_line: PatchLine::Addition("dark blue".to_string()),
                hunk_index: 0,
                line_index_within_hunk: 4
            })
        );
        assert_eq!(
            locate_edited_line(&patch, 2), // +silver
            Some(EditLocation {
                filename: "second.txt".to_string(),
                source_line_number: 12,
                target_line_number: 12,
                patch_line: PatchLine::Addition("silver".to_string()),
                hunk_index: 1,
                line_index_within_hunk: 3
            })
        );
    }

    mod remove_edits {
        use super::*;
        use indoc::indoc;
        use pretty_assertions::assert_eq;

        static PATCH: &'static str = indoc! {"
            diff --git a/text.txt b/text.txt
            index 86c770d..a1fd855 100644
            --- a/text.txt
            +++ b/text.txt
            @@ -1,7 +1,7 @@
             azuere
             beige
             black
            -blue
            +dark blue
             brown
             cyan
             gold
            @@ -9,6 +9,7 @@ gray
             green
             indigo
             magenta
            +silver
             orange
             pink
             purple
            @@ -16,4 +17,3 @@ red
             violet
             white
             yellow
            -zinc
        "};

        #[test]
        fn test_removes_hunks_without_edits() {
            // Remove the first two edits:
            // -blue
            // +dark blue
            let mut patch = Patch::parse_unified_diff(PATCH);
            remove_edits(&mut patch, vec![0, 1]);

            // The whole hunk should be removed since there are no other edits in it
            let actual = patch.to_string();
            let expected = indoc! {"
                --- a/text.txt
                +++ b/text.txt
                @@ -9,6 +9,7 @@ gray
                 green
                 indigo
                 magenta
                +silver
                 orange
                 pink
                 purple
                --- a/text.txt
                +++ b/text.txt
                @@ -16,4 +17,3 @@ red
                 violet
                 white
                 yellow
                -zinc
            "};
            assert_eq!(actual, String::from(expected));
        }

        #[test]
        fn test_adjust_line_numbers_after_deletion() {
            // Remove the first deletion (`-blue`)
            let mut patch = Patch::parse_unified_diff(PATCH);
            remove_edits(&mut patch, vec![0]);

            // The line numbers should be adjusted in the subsequent hunks
            println!("{}", &patch.to_string());
            assert_eq!(patch.hunks[0].header_string(), "@@ -2,6 +2,7 @@");
            assert_eq!(patch.hunks[1].header_string(), "@@ -9,6 +10,7 @@ gray");
            assert_eq!(patch.hunks[2].header_string(), "@@ -16,4 +18,3 @@ red");
        }
        #[test]
        fn test_adjust_line_numbers_after_insertion() {
            // Remove the first insertion (`+dark blue`)
            let mut patch = Patch::parse_unified_diff(PATCH);
            remove_edits(&mut patch, vec![1]);

            // The line numbers should be adjusted in the subsequent hunks
            assert_eq!(patch.hunks[0].header_string(), "@@ -1,7 +1,6 @@");
            assert_eq!(patch.hunks[1].header_string(), "@@ -9,6 +8,7 @@ gray");
            assert_eq!(patch.hunks[2].header_string(), "@@ -16,4 +16,3 @@ red");
        }
        #[test]
        fn test_adjust_line_numbers_multifile_case() {
            // Given a patch that spans multiple files
            let patch_str = indoc! {"
                --- a/first.txt
                +++ b/first.txt
                @@ -1,7 +1,7 @@
                 azuere
                 beige
                 black
                -blue
                +dark blue
                 brown
                 cyan
                 gold
                @@ -16,4 +17,3 @@ red
                 violet
                 white
                 yellow
                -zinc
                --- a/second.txt
                +++ b/second.txt
                @@ -9,6 +9,7 @@ gray
                 green
                 indigo
                 magenta
                +silver
                 orange
                 pink
                 purple
            "};

            // When removing edit from one of the files (`+dark blue`)
            let mut patch = Patch::parse_unified_diff(patch_str);
            remove_edits(&mut patch, vec![1]);

            // Then the line numbers should only be adjusted in subsequent hunks from that file
            assert_eq!(patch.hunks[0].header_string(), "@@ -1,7 +1,6 @@"); // edited hunk
            assert_eq!(patch.hunks[1].header_string(), "@@ -16,4 +16,3 @@ red"); // hunk from edited file again
            assert_eq!(patch.hunks[2].header_string(), "@@ -9,6 +9,7 @@ gray"); // hunk from another file

            // When removing hunk from `second.txt`
            let mut patch = Patch::parse_unified_diff(patch_str);
            remove_edits(&mut patch, vec![3]);

            // Then patch serialization should list `first.txt` only once
            // (because hunks from that file become adjacent)
            let expected = indoc! {"
                --- a/first.txt
                +++ b/first.txt
                @@ -1,7 +1,7 @@
                 azuere
                 beige
                 black
                -blue
                +dark blue
                 brown
                 cyan
                 gold
                --- a/first.txt
                +++ b/first.txt
                @@ -16,4 +17,3 @@ red
                 violet
                 white
                 yellow
                -zinc
            "};
            assert_eq!(patch.to_string(), expected);
        }

        #[test]
        fn test_dont_adjust_line_numbers_samefile_case() {
            // Given a patch that has hunks in the same file, but with a file header
            // (which makes `git apply` flush edits so far and start counting lines numbers afresh)
            let patch_str = indoc! {"
                diff --git a/text.txt b/text.txt
                index 86c770d..a1fd855 100644
                --- a/text.txt
                +++ b/text.txt
                @@ -1,7 +1,7 @@
                 azuere
                 beige
                 black
                -blue
                +dark blue
                 brown
                 cyan
                 gold
                --- a/text.txt
                +++ b/text.txt
                @@ -16,4 +16,3 @@ red
                 violet
                 white
                 yellow
                -zinc
        "};

            // When removing edit from one of the files (`+dark blue`)
            let mut patch = Patch::parse_unified_diff(patch_str);
            remove_edits(&mut patch, vec![1]);

            // Then the line numbers should **not** be adjusted in a subsequent hunk,
            // because it starts with a file header
            assert_eq!(patch.hunks[0].header_string(), "@@ -1,7 +1,6 @@"); // edited hunk
            assert_eq!(patch.hunks[1].header_string(), "@@ -16,4 +16,3 @@ red"); // subsequent hunk
        }
    }

    mod apply_edits {
        use super::*;
        use indoc::indoc;
        use pretty_assertions::assert_eq;

        static PATCH: &'static str = indoc! {"
            diff --git a/text.txt b/text.txt
            index 86c770d..a1fd855 100644
            --- a/text.txt
            +++ b/text.txt
            @@ -1,7 +1,7 @@
             azuere
             beige
             black
            -blue
            +dark blue
             brown
             cyan
             gold
             --- a/text.txt
             +++ b/text.txt
            @@ -9,6 +9,7 @@ gray
             green
             indigo
             magenta
            +silver
             orange
             pink
             purple
             --- a/text.txt
             +++ b/text.txt
            @@ -16,4 +17,3 @@ red
             violet
             white
             yellow
            -zinc
        "};

        #[test]
        fn test_removes_hunks_without_edits() {
            // When applying the first two edits (`-blue`, `+dark blue`)
            let mut patch = Patch::parse_unified_diff(PATCH);
            apply_edits(&mut patch, vec![0, 1]);

            // Then the whole hunk should be removed since there are no other edits in it,
            // and the line numbers should be adjusted in the subsequent hunks
            assert_eq!(patch.hunks[0].header_string(), "@@ -9,6 +9,7 @@ gray");
            assert_eq!(patch.hunks[1].header_string(), "@@ -16,4 +17,3 @@ red");
            assert_eq!(patch.hunks.len(), 2);
        }

        #[test]
        fn test_adjust_line_numbers_after_applying_deletion() {
            // Apply the first deletion (`-blue`)
            let mut patch = Patch::parse_unified_diff(PATCH);
            apply_edits(&mut patch, vec![0]);

            // The line numbers should be adjusted
            assert_eq!(patch.hunks[0].header_string(), "@@ -1,6 +1,7 @@");
            assert_eq!(patch.hunks[1].header_string(), "@@ -8,6 +9,7 @@ gray");
            assert_eq!(patch.hunks[2].header_string(), "@@ -15,4 +17,3 @@ red");
        }
        #[test]
        fn test_adjust_line_numbers_after_applying_insertion() {
            // Apply the first insertion (`+dark blue`)
            let mut patch = Patch::parse_unified_diff(PATCH);
            apply_edits(&mut patch, vec![1]);

            // The line numbers should be adjusted in the subsequent hunks
            println!("{}", &patch.to_string());
            assert_eq!(patch.hunks[0].header_string(), "@@ -1,7 +1,6 @@");
            assert_eq!(patch.hunks[1].header_string(), "@@ -10,6 +9,7 @@ gray");
            assert_eq!(patch.hunks[2].header_string(), "@@ -17,4 +17,3 @@ red");
        }
    }

    mod reorder_edits {
        use super::*;
        use indoc::indoc;
        use pretty_assertions::assert_eq;

        static PATCH: &'static str = indoc! {"
            Some header.

            diff --git a/first.txt b/first.txt
            index 86c770d..a1fd855 100644
            --- a/first.txt
            +++ b/first.txt
            @@ -1,7 +1,7 @@
             azuere
             beige
             black
            -blue
            +dark blue
             brown
             cyan
             gold
            --- a/second.txt
            +++ b/second.txt
            @@ -9,6 +9,7 @@ gray
             green
             indigo
             magenta
            +silver
             orange
             pink
             purple
            --- a/first.txt
            +++ b/first.txt
            @@ -16,4 +17,3 @@ red
             violet
             white
             yellow
            -zinc
        "};

        #[test]
        fn test_reorder_1() {
            let edits_order = vec![
                BTreeSet::from([2]),    // +silver
                BTreeSet::from([3]),    // -zinc
                BTreeSet::from([0, 1]), // -blue +dark blue
            ];

            let patch = Patch::parse_unified_diff(PATCH);
            let reordered_patch = reorder_edits(&patch, edits_order);

            // The whole hunk should be removed since there are no other edits in it
            let actual = reordered_patch.to_string();

            println!("{}", actual);

            let expected = indoc! {"
               Some header.

               --- a/second.txt
               +++ b/second.txt
               @@ -9,6 +9,7 @@ gray
                green
                indigo
                magenta
               +silver
                orange
                pink
                purple
               --- a/first.txt
               +++ b/first.txt
               @@ -16,4 +17,3 @@ red
                violet
                white
                yellow
               -zinc
               --- a/first.txt
               +++ b/first.txt
               @@ -1,7 +1,7 @@
                azuere
                beige
                black
               -blue
               +dark blue
                brown
                cyan
                gold
            "};
            assert_eq!(actual, String::from(expected));
        }

        #[test]
        fn test_reorder_duplicates() {
            let edits_order = vec![
                BTreeSet::from([2]), // +silver
                BTreeSet::from([2]), // +silver again
                BTreeSet::from([3]), // -zinc
            ];

            let patch = Patch::parse_unified_diff(PATCH);
            let reordered_patch = reorder_edits(&patch, edits_order);

            // The whole hunk should be removed since there are no other edits in it
            let actual = reordered_patch.to_string();

            println!("{}", actual);

            let expected = indoc! {"
                       Some header.

                       --- a/second.txt
                       +++ b/second.txt
                       @@ -9,6 +9,7 @@ gray
                        green
                        indigo
                        magenta
                       +silver
                        orange
                        pink
                        purple
                       --- a/first.txt
                       +++ b/first.txt
                       @@ -16,4 +17,3 @@ red
                        violet
                        white
                        yellow
                       -zinc
                    "};
            assert_eq!(actual, String::from(expected));
        }
    }

    mod extract_edits {

        use super::*;
        use indoc::indoc;
        use pretty_assertions::assert_eq;

        static PATCH: &'static str = indoc! {"
            Some header.

            diff --git a/first.txt b/first.txt
            index 86c770d..a1fd855 100644
            --- a/first.txt
            +++ b/first.txt
            @@ -1,7 +1,7 @@
             azuere
             beige
             black
            -blue
            +dark blue
             brown
             cyan
             gold
            @@ -16,4 +17,3 @@ red
             violet
             white
             yellow
            -zinc
            --- a/second.txt
            +++ b/second.txt
            @@ -9,6 +9,7 @@ gray
             green
             indigo
             magenta
            +silver
             orange
             pink
             purple
        "};

        #[test]
        fn test_extract_edits() {
            let to_extract = BTreeSet::from([
                3, // +silver
                0, // -blue
            ]);

            let mut patch = Patch::parse_unified_diff(PATCH);
            let (extracted, remainder) = extract_edits(&mut patch, &to_extract);

            // Edits will be extracted in the sorted order, so [0, 3]
            let expected_extracted = indoc! {"
               Some header.

               --- a/first.txt
               +++ b/first.txt
               @@ -1,7 +1,6 @@
                azuere
                beige
                black
               -blue
                brown
                cyan
                gold
               --- a/second.txt
               +++ b/second.txt
               @@ -9,6 +9,7 @@ gray
                green
                indigo
                magenta
               +silver
                orange
                pink
                purple
            "};

            let expected_remainder = indoc! {"
                Some header.

                --- a/first.txt
                +++ b/first.txt
                @@ -1,6 +1,7 @@
                 azuere
                 beige
                 black
                +dark blue
                 brown
                 cyan
                 gold
                --- a/first.txt
                +++ b/first.txt
                @@ -15,4 +17,3 @@ red
                 violet
                 white
                 yellow
                -zinc
            "};
            assert_eq!(extracted.to_string(), String::from(expected_extracted));
            assert_eq!(remainder.to_string(), String::from(expected_remainder));
        }
    }

    #[test]
    fn test_parse_order_file() {
        let content = r#"
// Add new dependency
1, 49

// Add new imports and types
8-9, 51

// Add new struct and login command method
10-47

// Modify AgentServerDelegate to make status_tx optional
2-3

// Update status_tx usage to handle optional value
4
5-7

// Update all existing callers to use None for status_tx
48, 50

// Update the main login implementation to use custom command
52-55
56-95
"#;

        let order = parse_order_spec(content);

        assert_eq!(order.len(), 9);

        // First group: 1, 49
        assert_eq!(order[0], BTreeSet::from([1, 49]));

        // Second group: 8-9, 51
        assert_eq!(order[1], BTreeSet::from([8, 9, 51]));

        // Third group: 10-47
        let expected_range: BTreeSet<usize> = (10..=47).collect();
        assert_eq!(order[2], expected_range);

        // Fourth group: 2-3
        assert_eq!(order[3], BTreeSet::from([2, 3]));

        // Fifth group: 4
        assert_eq!(order[4], BTreeSet::from([4]));

        // Sixth group: 5-7
        assert_eq!(order[5], BTreeSet::from([5, 6, 7]));

        // Seventh group: 48, 50
        assert_eq!(order[6], BTreeSet::from([48, 50]));

        // Eighth group: 52-55
        assert_eq!(order[7], BTreeSet::from([52, 53, 54, 55]));

        // Ninth group: 56-95
        let expected_range_2: BTreeSet<usize> = (56..=95).collect();
        assert_eq!(order[8], expected_range_2);
    }

    #[test]
    fn test_normalize_hunk() {
        let mut patch = Patch::parse_unified_diff(indoc! {"
            This patch has too many lines of context.

            --- a/first.txt
            +++ b/first.txt
            @@ -1,7 +1,6 @@
             azuere
             beige
             black
            -blue
             brown
             cyan
             gold
            // Some garbage
        "});

        patch.normalize_hunks(1);
        let actual = patch.to_string();
        assert_eq!(
            actual,
            indoc! {"
            This patch has too many lines of context.

            --- a/first.txt
            +++ b/first.txt
            @@ -3,3 +3,2 @@
             black
            -blue
             brown
            // Some garbage
        "}
        );
    }
}
