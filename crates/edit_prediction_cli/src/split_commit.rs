//! `ep split-commit` implementation.
//!
//! This command generates a single evaluation example JSON object from a
//! chronologically-ordered unified diff (a "commit").
//!
//! TODO: Port Python code to generate chronologically-ordered commits
use crate::reorder_patch::{Patch, PatchLine, extract_edits, locate_edited_line};
use anyhow::{Context as _, Result};
use clap::Args;
use rand::Rng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use similar::{DiffTag, TextDiff};
use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Read};

/// `ep split-commit` CLI args.
#[derive(Debug, Args)]
pub struct SplitCommitArgs {
    /// Path to the commit file (use "-" for stdin)
    #[arg(long, short = 'c')]
    pub commit: String,

    /// Repository URL
    #[arg(long, short = 'r', default_value_t = String::new())]
    pub repository_url: String,

    /// Commit hash
    #[arg(long, default_value_t = String::new())]
    pub commit_hash: String,

    /// Split point (float 0.0-1.0 for fraction, or integer for index)
    #[arg(long, short = 's')]
    pub split_point: Option<String>,

    /// Random seed for reproducibility
    #[arg(long)]
    pub seed: Option<u64>,

    /// Pretty-print JSON output
    #[arg(long, short = 'p')]
    pub pretty: bool,
}

/// Cursor position in a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorPosition {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for CursorPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// Represents a split commit with source and target patches.
#[derive(Debug, Clone)]
pub struct SplitCommit {
    pub source_patch: String,
    pub target_patch: String,
}

/// The evaluation case structure that will be serialized to JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCase {
    pub repository_url: String,
    pub commit: String,
    pub edit_history: Vec<String>,
    pub cursor_position: String,
    pub cursor_excerpt: String,
    pub expected_hunks: Vec<String>,
    pub expected_patch: String,
    pub allowed_patch: String,
    pub expected_context_excerpts: Vec<String>,
    pub extra: serde_json::Value,
}

/// Split point specification for evaluation generation.
#[derive(Debug, Clone)]
pub enum SplitPoint {
    /// Fraction of total edits (0.0 to 1.0)
    Fraction(f64),
    /// Absolute index
    Index(usize),
}

fn parse_split_point(value: &str) -> Option<SplitPoint> {
    if value.contains('.') {
        value.parse::<f64>().ok().map(SplitPoint::Fraction)
    } else {
        value.parse::<usize>().ok().map(SplitPoint::Index)
    }
}

/// Entry point for the `ep split-commit` subcommand.
///
/// This runs synchronously and prints a single JSON object to stdout.
pub fn run_split_commit(args: &SplitCommitArgs) -> Result<()> {
    let commit = if args.commit == "-" {
        let mut content = String::new();
        io::stdin()
            .read_to_string(&mut content)
            .context("failed to read commit diff from stdin")?;
        content
    } else {
        fs::read_to_string(&args.commit)
            .with_context(|| format!("failed to read commit diff from {}", args.commit))?
    };

    let split_point = args.split_point.as_deref().and_then(parse_split_point);

    let case = generate_evaluation_example_from_ordered_commit(
        &commit,
        &args.repository_url,
        &args.commit_hash,
        split_point,
        args.seed,
    )
    .context("failed to generate evaluation example")?;

    let json = if args.pretty {
        serde_json::to_string_pretty(&case)
    } else {
        serde_json::to_string(&case)
    }
    .context("failed to serialize evaluation case as JSON")?;

    println!("{json}");
    Ok(())
}

/// Main function to generate an evaluation example from an ordered commit.
///
/// # Arguments
/// * `commit` - Chronologically ordered unified diff of the commit
/// * `repository_url` - URL of the repository
/// * `commit_hash` - Hash of the commit
/// * `split_point` - Point at which the commit will be split (None for random)
/// * `seed` - Optional seed for randomness
pub fn generate_evaluation_example_from_ordered_commit(
    commit: &str,
    repository_url: &str,
    commit_hash: &str,
    split_point: Option<SplitPoint>,
    seed: Option<u64>,
) -> Result<EvaluationCase> {
    let mut rng: Box<dyn rand::RngCore> = match seed {
        Some(seed) => Box::new(rand::rngs::StdRng::seed_from_u64(seed)),
        None => Box::new(rand::rngs::ThreadRng::default()),
    };

    // Parse and normalize the commit
    let mut patch = Patch::parse_unified_diff(commit);

    // Filter header to only keep lines starting with "//"
    let header_lines: Vec<&str> = patch
        .header
        .lines()
        .filter(|line| line.starts_with("//"))
        .collect();
    patch.header = if header_lines.is_empty() {
        String::new()
    } else {
        header_lines.join("\n") + "\n"
    };
    let commit_normalized = patch.to_string();

    // Compute the split point
    let stats = patch.stats();
    let num_edits = stats.added + stats.removed;

    anyhow::ensure!(num_edits != 0, "no edits found in commit");

    let split = match split_point {
        None => rng.random_range(1..=num_edits),
        Some(SplitPoint::Fraction(f)) => {
            let v = (f * num_edits as f64).floor() as usize;
            v.min(num_edits)
        }
        Some(SplitPoint::Index(i)) => i.min(num_edits),
    };

    // Split the commit into source and target patches
    let (prefix, suffix) = split_ordered_commit(&commit_normalized, split);

    let mut split_commit = SplitCommit {
        source_patch: prefix,
        target_patch: suffix,
    };

    // Imitate human edits
    let human_edit_seed = rng.random_range(1..=10000u64);
    let (src_patch, tgt_patch, cursor_opt) = imitate_human_edits(
        &split_commit.source_patch,
        &split_commit.target_patch,
        human_edit_seed,
    );
    split_commit.source_patch = src_patch;
    split_commit.target_patch = tgt_patch;

    // Sample cursor position
    let cursor = match cursor_opt {
        Some(c) => c,
        None => sample_cursor_position(&patch, &split_commit)
            .context("failed to sample cursor position")?,
    };

    // Get cursor excerpt
    let cursor_excerpt = get_cursor_excerpt(
        &cursor,
        &split_commit.source_patch,
        &split_commit.target_patch,
    )
    .context("failed to generate cursor excerpt")?;

    // Handle edge case where split_point == 0
    if split == 0 {
        split_commit.target_patch = String::new();
    }

    Ok(EvaluationCase {
        repository_url: repository_url.to_string(),
        commit: format!("{}~1", commit_hash),
        edit_history: vec![split_commit.source_patch.clone()],
        cursor_position: cursor.to_string(),
        cursor_excerpt,
        expected_hunks: vec![split_commit.target_patch.clone()],
        expected_patch: split_commit.target_patch.clone(),
        allowed_patch: split_commit.target_patch,
        expected_context_excerpts: vec![],
        extra: serde_json::json!({}),
    })
}

/// Split an ordered commit into source and target commits.
///
/// # Arguments
/// * `commit` - Ordered commit string
/// * `split_pos` - Position to split the commit (number of edited lines)
///
/// # Returns
/// A tuple of (source_diff, target_diff)
pub fn split_ordered_commit(commit: &str, split_pos: usize) -> (String, String) {
    let patch = Patch::parse_unified_diff(commit);
    let source_edits: BTreeSet<usize> = (0..split_pos).collect();
    let (source, target) = extract_edits(&patch, &source_edits);

    let mut source_str = source.to_string();
    let target_str = target.to_string();

    // Strip last group header from the source (lines starting with "//" at the end)
    let source_lines: Vec<&str> = source_str.lines().collect();
    let mut end_idx = source_lines.len();
    for i in (0..source_lines.len()).rev() {
        if source_lines[i].starts_with("//") {
            end_idx = i;
        } else {
            break;
        }
    }
    if end_idx < source_lines.len() {
        source_str = source_lines[..end_idx].join("\n");
        if !source_str.is_empty() {
            source_str.push('\n');
        }
    }

    (source_str, target_str)
}

/// Tokenize text into words and non-word characters.
fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for ch in text.chars() {
        if ch.is_alphanumeric() {
            current.push(ch);
        } else if ch == '_' {
            // Include underscore with the current word, then flush
            current.push(ch);
            if !current.is_empty() {
                tokens.push(std::mem::take(&mut current));
            }
        } else {
            // Punctuation or whitespace - flush current word first
            if !current.is_empty() {
                tokens.push(std::mem::take(&mut current));
            }
            // Each punctuation/whitespace is its own token
            tokens.push(ch.to_string());
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

/// Calculate the weight for a split position based on the character at that position.
///
/// Higher weights indicate more natural pause points (e.g., after punctuation,
/// at identifier boundaries). Lower weights indicate less natural points
/// (e.g., mid-identifier).
fn position_weight(text: &str, pos: usize) -> u32 {
    if pos == 0 || pos > text.len() {
        return 1;
    }

    let chars: Vec<char> = text.chars().collect();
    if pos > chars.len() {
        return 1;
    }

    // Get the character just before this position (what we just "typed")
    let prev_char = chars[pos - 1];

    // High weight: natural pause points (end of statement/argument, opening brackets)
    if matches!(prev_char, ',' | ';' | ':' | '(' | '[' | '{') {
        return 10;
    }

    // High weight: closing brackets (finished a group)
    if matches!(prev_char, ')' | ']' | '}') {
        return 8;
    }

    // Medium weight: operators and method chains
    if matches!(
        prev_char,
        '.' | '+' | '-' | '*' | '/' | '=' | '<' | '>' | '&' | '|' | '!'
    ) {
        return 5;
    }

    // Check if we're at the end of an identifier (word char followed by non-word char)
    let is_prev_word_char = prev_char.is_alphanumeric() || prev_char == '_';
    let is_next_word_char =
        pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_');

    if is_prev_word_char && !is_next_word_char {
        // End of identifier - high weight
        return 8;
    }

    // Whitespace is a natural pause
    if prev_char.is_whitespace() {
        return 6;
    }

    // Mid-identifier: low weight (rare autocomplete scenarios)
    if is_prev_word_char && is_next_word_char {
        return 1;
    }

    // Default medium-low weight
    3
}

/// Select a weighted random index from a list of weights.
///
/// Returns an index based on the weights, using the provided seed for
/// deterministic selection.
fn weighted_select(weights: &[u32], seed: u64) -> usize {
    if weights.is_empty() {
        return 0;
    }

    let total_weight: u64 = weights.iter().map(|&w| w as u64).sum();
    if total_weight == 0 {
        // Fallback to uniform selection if all weights are zero
        return seed as usize % weights.len();
    }

    // Use seed to select a value in [0, total_weight)
    let target = seed % total_weight;
    let mut cumulative: u64 = 0;

    for (idx, &weight) in weights.iter().enumerate() {
        cumulative += weight as u64;
        if target < cumulative {
            return idx;
        }
    }

    // Fallback to last index
    weights.len() - 1
}

/// Calculate similarity ratio between two strings (0-100).
fn fuzzy_ratio(s1: &str, s2: &str) -> u32 {
    if s1.is_empty() && s2.is_empty() {
        return 100;
    }
    if s1.is_empty() || s2.is_empty() {
        return 0;
    }

    let diff = TextDiff::from_chars(s1, s2);
    let matching: usize = diff
        .ops()
        .iter()
        .filter_map(|op| {
            if matches!(op.tag(), DiffTag::Equal) {
                Some(op.new_range().len())
            } else {
                None
            }
        })
        .sum();

    let total = s1.len() + s2.len();
    ((2 * matching * 100) / total) as u32
}

/// Imitate human edits by introducing partial line edits.
///
/// This function simulates how a human might incrementally type code,
/// rather than making complete line replacements.
pub fn imitate_human_edits(
    source_patch: &str,
    target_patch: &str,
    seed: u64,
) -> (String, String, Option<CursorPosition>) {
    let no_change = (source_patch.to_string(), target_patch.to_string(), None);

    let src_patch = Patch::parse_unified_diff(source_patch);
    let tgt_patch = Patch::parse_unified_diff(target_patch);

    if tgt_patch.hunks.is_empty() {
        return no_change;
    }

    // Try to locate the first edit in target
    let tgt_edit_loc = match locate_edited_line(&tgt_patch, 0) {
        Some(loc) => loc,
        None => return no_change,
    };

    let tgt_is_addition = matches!(tgt_edit_loc.patch_line, PatchLine::Addition(_));
    if !tgt_is_addition {
        return no_change;
    }

    let tgt_line = match &tgt_edit_loc.patch_line {
        PatchLine::Addition(s) => s.clone(),
        _ => return no_change,
    };

    // Try to locate the last edit in source
    let src_edit_loc = locate_edited_line(&src_patch, -1);

    // Check if source has ANY edit at the same line as target's first edit
    // We need to iterate through all edits to check this
    let src_has_edit_at_target_line = {
        let mut found = false;
        let mut idx = 0isize;
        while let Some(loc) = locate_edited_line(&src_patch, idx) {
            if loc.filename == tgt_edit_loc.filename
                && loc.target_line_number == tgt_edit_loc.target_line_number
            {
                found = true;
                break;
            }
            idx += 1;
        }
        found
    };

    // Check if this is a replacement (deletion followed by insertion on the same line)
    // or a pure insertion (no corresponding deletion in source)
    let is_replacement = src_edit_loc.as_ref().map_or(false, |loc| {
        matches!(loc.patch_line, PatchLine::Deletion(_))
            && loc.filename == tgt_edit_loc.filename
            && loc.target_line_number == tgt_edit_loc.target_line_number
    });

    // If source has an edit at the same line but it's not a replacement (i.e., it's an addition),
    // we shouldn't process this as a pure insertion either
    if !is_replacement && src_has_edit_at_target_line {
        return no_change;
    }

    let src_line = if is_replacement {
        match &src_edit_loc.as_ref().unwrap().patch_line {
            PatchLine::Deletion(s) => s.clone(),
            _ => return no_change,
        }
    } else {
        // Pure insertion: source line is empty
        String::new()
    };

    // Don't process if source and target are the same
    if src_line == tgt_line {
        return no_change;
    }

    // Tokenize both lines
    let src_tokens = tokenize(&src_line);
    let tgt_tokens = tokenize(&tgt_line);

    // Convert to slices for similar
    let src_refs: Vec<&str> = src_tokens.iter().map(|s| s.as_str()).collect();
    let tgt_refs: Vec<&str> = tgt_tokens.iter().map(|s| s.as_str()).collect();

    // Use similar to get diff operations
    let diff = TextDiff::from_slices(&src_refs, &tgt_refs);

    // Build weights for each possible split position
    let mut position_weights: Vec<u32> = Vec::new();

    // Simulate the edit process to collect weights for all possible split positions
    {
        let mut current_text = String::new();

        for op in diff.ops() {
            match op.tag() {
                DiffTag::Equal => {
                    for i in op.old_range() {
                        current_text.push_str(&src_tokens[i]);
                    }
                }
                DiffTag::Replace => {
                    let ins: String = op.new_range().map(|i| tgt_tokens[i].as_str()).collect();
                    let del: String = op.old_range().map(|i| src_tokens[i].as_str()).collect();

                    // For insertion part
                    for ch in ins.chars() {
                        current_text.push(ch);
                        let weight = position_weight(&current_text, current_text.len());
                        position_weights.push(weight);
                    }

                    // For deletion part (we're "untyping" from source)
                    for _ in del.chars() {
                        // Weight deletions lower as they represent removing text
                        position_weights.push(2);
                    }
                }
                DiffTag::Insert => {
                    let ins: String = op.new_range().map(|i| tgt_tokens[i].as_str()).collect();
                    for ch in ins.chars() {
                        current_text.push(ch);
                        let weight = position_weight(&current_text, current_text.len());
                        position_weights.push(weight);
                    }
                }
                DiffTag::Delete => {
                    let del: String = op.old_range().map(|i| src_tokens[i].as_str()).collect();
                    for _ in del.chars() {
                        // Weight deletions lower
                        position_weights.push(2);
                    }
                }
            }
        }
    }

    // Use weighted selection to choose split index
    if position_weights.is_empty() {
        return no_change;
    }
    let split_index = weighted_select(&position_weights, seed);

    let mut edit_index = 0usize;
    let mut new_src = String::new();
    let mut split_found = false;
    let mut last_old_end = 0usize;

    for op in diff.ops() {
        match op.tag() {
            DiffTag::Equal => {
                for i in op.old_range() {
                    new_src.push_str(&src_tokens[i]);
                }
                last_old_end = op.old_range().end;
            }
            DiffTag::Replace => {
                // Handle replace as delete + insert
                let del: String = op.old_range().map(|i| src_tokens[i].as_str()).collect();
                let ins: String = op.new_range().map(|i| tgt_tokens[i].as_str()).collect();
                let repl_len = del.len() + ins.len();
                if edit_index + repl_len >= split_index {
                    // Split within this replace operation
                    let offset = split_index - edit_index;
                    if offset < ins.len() {
                        new_src.push_str(&ins[..offset]);
                    } else {
                        new_src.push_str(&ins);
                        let del_offset = offset - ins.len();
                        new_src.push_str(&del[..del_offset.min(del.len())]);
                    }
                    split_found = true;
                    last_old_end = op.old_range().end;
                    break;
                } else {
                    edit_index += repl_len;
                    new_src.push_str(&ins);
                    last_old_end = op.old_range().end;
                }
            }
            DiffTag::Insert => {
                let repl: String = op.new_range().map(|i| tgt_tokens[i].as_str()).collect();
                if edit_index + repl.len() >= split_index {
                    let offset = split_index - edit_index;
                    new_src.push_str(&repl[..offset]);
                    split_found = true;
                    break;
                } else {
                    edit_index += repl.len();
                    new_src.push_str(&repl);
                }
            }
            DiffTag::Delete => {
                let repl: String = op.old_range().map(|i| src_tokens[i].as_str()).collect();
                if edit_index + repl.len() >= split_index {
                    let offset = split_index - edit_index;
                    new_src.push_str(&repl[..offset]);
                    split_found = true;
                    last_old_end = op.old_range().start + offset.min(op.old_range().len());
                    break;
                } else {
                    edit_index += repl.len();
                    new_src.push_str(&repl);
                    last_old_end = op.old_range().end;
                }
            }
        }
    }

    if !split_found {
        return no_change;
    }

    // Calculate cursor position
    let cursor = CursorPosition {
        file: tgt_edit_loc.filename.clone(),
        line: if is_replacement {
            src_edit_loc.as_ref().unwrap().source_line_number
        } else {
            tgt_edit_loc.target_line_number
        },
        column: new_src.len() + 1,
    };

    // Add remainder of source if similar enough to target remainder
    let remainder_src: String = (last_old_end..src_tokens.len())
        .map(|i| src_tokens[i].as_str())
        .collect();
    let remainder_tgt: String = (last_old_end..tgt_tokens.len())
        .filter_map(|i| tgt_tokens.get(i).map(|s| s.as_str()))
        .collect();

    let ratio = fuzzy_ratio(&remainder_src, &remainder_tgt);
    if ratio > 35 {
        new_src.push_str(&remainder_src);
    }

    if new_src.trim().is_empty() {
        return no_change;
    }

    if new_src == src_line {
        return no_change;
    }

    // Build new source patch with the intermediate line
    let mut new_src_patch = src_patch;
    if is_replacement {
        // For replacements, insert after the deletion line
        let src_loc = src_edit_loc.as_ref().unwrap();
        if let Some(hunk) = new_src_patch.hunks.get_mut(src_loc.hunk_index) {
            hunk.lines.insert(
                src_loc.line_index_within_hunk + 1,
                PatchLine::Addition(new_src.clone()),
            );
            hunk.new_count += 1;
        }
    } else {
        // For pure insertions, we need to add or modify a hunk
        if let Some(hunk) = new_src_patch.hunks.get_mut(tgt_edit_loc.hunk_index) {
            // Insert the partial line at the same position as target
            hunk.lines.insert(
                tgt_edit_loc.line_index_within_hunk,
                PatchLine::Addition(new_src.clone()),
            );
            hunk.new_count += 1;
        } else if new_src_patch.hunks.is_empty() {
            // Source patch is empty, create a new hunk based on target
            if let Some(tgt_hunk) = tgt_patch.hunks.get(tgt_edit_loc.hunk_index) {
                let mut new_hunk = tgt_hunk.clone();
                // Replace the full addition with the partial one
                new_hunk.lines.clear();
                for (i, line) in tgt_hunk.lines.iter().enumerate() {
                    if i == tgt_edit_loc.line_index_within_hunk {
                        new_hunk.lines.push(PatchLine::Addition(new_src.clone()));
                    } else {
                        match line {
                            PatchLine::Addition(_) => {
                                // Skip other additions from target
                            }
                            _ => new_hunk.lines.push(line.clone()),
                        }
                    }
                }
                new_hunk.new_count = new_hunk.old_count + 1;
                new_src_patch.hunks.push(new_hunk);
                // Copy header from target if source doesn't have one
                if new_src_patch.header.is_empty() {
                    new_src_patch.header = tgt_patch.header.clone();
                }
            }
        }
    }

    // Build new target patch with the intermediate line as deletion
    let mut new_tgt_patch = tgt_patch;
    if let Some(hunk) = new_tgt_patch.hunks.get_mut(tgt_edit_loc.hunk_index) {
        hunk.lines.insert(
            tgt_edit_loc.line_index_within_hunk,
            PatchLine::Deletion(new_src),
        );
        hunk.old_count += 1;
    }

    (
        new_src_patch.to_string(),
        new_tgt_patch.to_string(),
        Some(cursor),
    )
}

/// Locate the end of the last edit in a patch.
fn locate_end_of_last_edit(patch: &Patch) -> Option<CursorPosition> {
    let loc = locate_edited_line(patch, -1)?;

    let (line, col) = match &loc.patch_line {
        PatchLine::Addition(content) => (loc.target_line_number, content.len()),
        PatchLine::Deletion(_) => (loc.target_line_number, 1),
        _ => return None,
    };

    Some(CursorPosition {
        file: loc.filename,
        line,
        column: col,
    })
}

/// Locate the beginning of the first edit in a patch.
fn locate_beginning_of_first_edit(patch: &Patch) -> Option<CursorPosition> {
    let loc = locate_edited_line(patch, 0)?;

    let hunk = patch.hunks.get(loc.hunk_index)?;
    let column = if loc.line_index_within_hunk > 0 {
        if let Some(prev_line) = hunk.lines.get(loc.line_index_within_hunk - 1) {
            let content = match prev_line {
                PatchLine::Context(s) | PatchLine::Addition(s) | PatchLine::Deletion(s) => s,
                _ => return None,
            };
            content.len().max(1) - 1
        } else {
            0
        }
    } else {
        0
    };

    let line = loc.target_line_number.saturating_sub(1).max(1);

    Some(CursorPosition {
        file: loc.filename,
        line,
        column,
    })
}

/// Sample cursor position according to the following rules:
/// 1. 50% chance of cursor being at the end of the source patch
/// 2. 50% chance of cursor being at the beginning of the target patch
pub fn sample_cursor_position(patch: &Patch, split_commit: &SplitCommit) -> Option<CursorPosition> {
    // Try end of history first
    let src_patch = Patch::parse_unified_diff(&split_commit.source_patch);
    if let Some(cursor) = locate_end_of_last_edit(&src_patch) {
        return Some(cursor);
    }

    // Try beginning of target
    let tgt_patch = Patch::parse_unified_diff(&split_commit.target_patch);
    if let Some(cursor) = locate_beginning_of_first_edit(&tgt_patch) {
        return Some(cursor);
    }

    // Fallback: use the original patch
    locate_end_of_last_edit(patch)
}

/// Get cursor excerpt from the patches.
///
/// This extracts the lines around the cursor position with a cursor marker.
pub fn get_cursor_excerpt(
    cursor: &CursorPosition,
    source_patch: &str,
    target_patch: &str,
) -> Option<String> {
    let mut excerpt_lines: Vec<String> = Vec::new();
    let mut excerpt_first_line: usize = 0;

    // Search in the last hunk of source patch
    let src = Patch::parse_unified_diff(source_patch);
    if let Some(loc) = locate_edited_line(&src, -1) {
        if loc.filename == cursor.file && loc.target_line_number == cursor.line {
            if let Some(hunk) = src.hunks.get(loc.hunk_index) {
                excerpt_first_line = hunk.new_start as usize;
                for line in &hunk.lines {
                    match line {
                        PatchLine::Addition(s) | PatchLine::Context(s) => {
                            excerpt_lines.push(s.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Search in target patch if not found
    if excerpt_lines.is_empty() {
        let tgt = Patch::parse_unified_diff(target_patch);
        if let Some(loc) = locate_edited_line(&tgt, 0) {
            if loc.filename == cursor.file {
                if let Some(hunk) = tgt.hunks.get(loc.hunk_index) {
                    excerpt_first_line = hunk.new_start as usize;
                    for line in &hunk.lines {
                        match line {
                            PatchLine::Deletion(s) | PatchLine::Context(s) => {
                                excerpt_lines.push(s.clone());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    if excerpt_lines.is_empty() {
        return None;
    }

    // Add cursor marker
    for (i, line) in excerpt_lines.iter_mut().enumerate() {
        let line_num = excerpt_first_line + i;
        if line_num == cursor.line {
            let col = cursor.column.min(line.len());
            let (before, after) = line.split_at(col);
            *line = format!("{}<|user_cursor|>{}", before, after);
            break;
        }
    }

    Some(excerpt_lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("hello world");
        assert_eq!(tokens, vec!["hello", " ", "world"]);

        let tokens = tokenize("foo_bar123 + baz");
        assert_eq!(tokens, vec!["foo_", "bar123", " ", "+", " ", "baz"]);

        let tokens = tokenize("print(\"hello\")");
        assert_eq!(tokens, vec!["print", "(", "\"", "hello", "\"", ")"]);

        let tokens = tokenize("hello_world");
        assert_eq!(tokens, vec!["hello_", "world"]);

        let tokens = tokenize("fn();");
        assert_eq!(tokens, vec!["fn", "(", ")", ";"]);
    }

    #[test]
    fn test_fuzzy_ratio() {
        assert_eq!(fuzzy_ratio("hello", "hello"), 100);
        assert_eq!(fuzzy_ratio("", ""), 100);
        assert!(fuzzy_ratio("hello", "world") < 50);
        assert!(fuzzy_ratio("hello world", "hello worl") > 80);
    }

    #[test]
    fn test_split_ordered_commit() {
        let commit = r#"// First change
--- a/test.rs
+++ b/test.rs
@@ -1,3 +1,4 @@
 fn main() {
+    println!("hello");
+    println!("world");
 }
"#;
        let patch = Patch::parse_unified_diff(commit);
        let stats = patch.stats();
        assert_eq!(stats.added, 2);

        let (source, target) = split_ordered_commit(commit, 1);

        // Source should have 1 addition
        let src_patch = Patch::parse_unified_diff(&source);
        assert_eq!(src_patch.stats().added, 1);

        // Target should have 1 addition
        let tgt_patch = Patch::parse_unified_diff(&target);
        assert_eq!(tgt_patch.stats().added, 1);
    }

    #[test]
    fn test_split_ordered_commit_with_deletions() {
        let commit = r#"// Change
--- a/test.rs
+++ b/test.rs
@@ -1,3 +1,3 @@
 fn main() {
-    println!("old");
+    println!("new");
 }
"#;
        let patch = Patch::parse_unified_diff(commit);
        let stats = patch.stats();
        assert_eq!(stats.added, 1);
        assert_eq!(stats.removed, 1);

        // Split at position 1 (after the deletion)
        let (source, target) = split_ordered_commit(commit, 1);

        let src_patch = Patch::parse_unified_diff(&source);
        let tgt_patch = Patch::parse_unified_diff(&target);

        // Source should have the deletion
        assert_eq!(src_patch.stats().removed, 1);
        // Target should have the addition
        assert_eq!(tgt_patch.stats().added, 1);
    }

    #[test]
    fn test_generate_evaluation_example() {
        let commit = r#"commit abc123
Author: Test <test@example.com>
Date: Mon Jan 1 00:00:00 2024

    Test commit

////////////////////////////////////////////////////////////////////////////////
// Add greeting
////////////////////////////////////////////////////////////////////////////////
--- a/test.rs
+++ b/test.rs
@@ -1,3 +1,5 @@
 fn main() {
+    println!("hello");
+    println!("world");
 }
"#;

        let result = generate_evaluation_example_from_ordered_commit(
            commit,
            "https://github.com/test/repo",
            "abc123",
            Some(SplitPoint::Fraction(0.5)),
            Some(42),
        );

        assert!(result.is_ok());
        let case = result.unwrap();
        assert_eq!(case.repository_url, "https://github.com/test/repo");
        assert_eq!(case.commit, "abc123~1");
        assert!(!case.edit_history.is_empty());
    }

    #[test]
    fn test_generate_evaluation_example_reproducible() {
        let commit = r#"////////////////////////////////////////////////////////////////////////////////
// Add greeting
////////////////////////////////////////////////////////////////////////////////
--- a/test.rs
+++ b/test.rs
@@ -1,3 +1,5 @@
 fn main() {
+    println!("hello");
+    println!("world");
 }
"#;

        // Run twice with the same seed
        let result1 = generate_evaluation_example_from_ordered_commit(
            commit,
            "https://github.com/test/repo",
            "abc123",
            Some(SplitPoint::Fraction(0.5)),
            Some(12345),
        )
        .unwrap();

        let result2 = generate_evaluation_example_from_ordered_commit(
            commit,
            "https://github.com/test/repo",
            "abc123",
            Some(SplitPoint::Fraction(0.5)),
            Some(12345),
        )
        .unwrap();

        // Results should be identical
        assert_eq!(result1.edit_history, result2.edit_history);
        assert_eq!(result1.expected_patch, result2.expected_patch);
        assert_eq!(result1.cursor_position, result2.cursor_position);
    }

    #[test]
    fn test_cursor_position_display() {
        let cursor = CursorPosition {
            file: "src/main.rs".to_string(),
            line: 42,
            column: 10,
        };
        assert_eq!(cursor.to_string(), "src/main.rs:42:10");
    }

    #[test]
    fn test_imitate_human_edits_no_change_when_no_replacement() {
        // Source and target patches that don't form a replacement pattern
        let source = r#"--- a/test.rs
+++ b/test.rs
@@ -1,3 +1,4 @@
 fn main() {
+    println!("hello");
 }
"#;
        let target = r#"--- a/test.rs
+++ b/test.rs
@@ -1,3 +1,4 @@
 fn main() {
+    println!("world");
 }
"#;

        let (new_src, new_tgt, cursor) = imitate_human_edits(source, target, 42);

        // Should return unchanged when not a replacement pattern
        assert_eq!(new_src, source);
        assert_eq!(new_tgt, target);
        assert!(cursor.is_none());
    }

    #[test]
    fn test_split_point_fraction() {
        let commit = r#"// Change
--- a/test.rs
+++ b/test.rs
@@ -1,5 +1,10 @@
 fn main() {
+    line1();
+    line2();
+    line3();
+    line4();
+    line5();
 }
"#;

        // Split at 20% should give first edit in source
        let result = generate_evaluation_example_from_ordered_commit(
            commit,
            "",
            "hash",
            Some(SplitPoint::Fraction(0.2)),
            Some(1),
        );

        assert!(result.is_ok());
        let case = result.unwrap();

        // Source should have some edits
        let src_patch = Patch::parse_unified_diff(&case.edit_history[0]);
        assert!(src_patch.stats().added > 0);
    }

    #[test]
    fn test_split_point_index() {
        let commit = r#"// Change
--- a/test.rs
+++ b/test.rs
@@ -1,5 +1,10 @@
 fn main() {
+    line1();
+    line2();
+    line3();
+    line4();
+    line5();
 }
"#;

        // Split at index 2 should give first 2 edits in source
        // With pure insertion handling, source gets 2 original + 1 partial = 3 additions
        let result = generate_evaluation_example_from_ordered_commit(
            commit,
            "",
            "hash",
            Some(SplitPoint::Index(2)),
            Some(1),
        );

        assert!(result.is_ok());
        let case = result.unwrap();

        let src_patch = Patch::parse_unified_diff(&case.edit_history[0]);
        // Pure insertion adds a partial line, so we expect 3 (2 original + 1 partial)
        assert_eq!(src_patch.stats().added, 3);
    }

    #[test]
    fn test_cursor_excerpt_contains_marker() {
        let commit = r#"////////////////////////////////////////////////////////////////////////////////
// Add code
////////////////////////////////////////////////////////////////////////////////
--- a/test.rs
+++ b/test.rs
@@ -1,3 +1,5 @@
 fn main() {
+    println!("hello");
+    println!("world");
 }
"#;

        let result = generate_evaluation_example_from_ordered_commit(
            commit,
            "",
            "hash",
            Some(SplitPoint::Fraction(0.5)),
            Some(42),
        )
        .unwrap();

        // Cursor excerpt should contain the cursor marker
        assert!(
            result.cursor_excerpt.contains("<|user_cursor|>"),
            "Cursor excerpt should contain marker: {}",
            result.cursor_excerpt
        );
    }

    #[test]
    fn test_evaluation_case_json_serialization() {
        let case = EvaluationCase {
            repository_url: "https://github.com/test/repo".to_string(),
            commit: "abc123~1".to_string(),
            edit_history: vec!["patch1".to_string()],
            cursor_position: "file.rs:10:5".to_string(),
            cursor_excerpt: "some code<|user_cursor|>".to_string(),
            expected_hunks: vec!["hunk1".to_string()],
            expected_patch: "patch".to_string(),
            allowed_patch: "patch".to_string(),
            expected_context_excerpts: vec![],
            extra: serde_json::json!({}),
        };

        let json = serde_json::to_string(&case).unwrap();
        let deserialized: EvaluationCase = serde_json::from_str(&json).unwrap();

        assert_eq!(case.repository_url, deserialized.repository_url);
        assert_eq!(case.commit, deserialized.commit);
        assert_eq!(case.cursor_position, deserialized.cursor_position);
    }

    #[test]
    fn test_empty_commit_returns_error() {
        let commit = "";

        let result = generate_evaluation_example_from_ordered_commit(
            commit,
            "",
            "hash",
            Some(SplitPoint::Fraction(0.5)),
            Some(1),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_header_filtering() {
        let commit = r#"commit abc123
Author: Test
Date: Today

    Message

diff --git a/test.rs b/test.rs
index 123..456 789
////////////////////////////////////////////////////////////////////////////////
// First group
////////////////////////////////////////////////////////////////////////////////
--- a/test.rs
+++ b/test.rs
@@ -1,3 +1,4 @@
 fn main() {
+    code();
 }
"#;

        let result = generate_evaluation_example_from_ordered_commit(
            commit,
            "",
            "hash",
            Some(SplitPoint::Index(1)),
            Some(1),
        );

        assert!(result.is_ok());
        let case = result.unwrap();

        // The edit history should contain the group header (// lines)
        // but not the commit metadata
        assert!(!case.edit_history[0].contains("Author:"));
        assert!(!case.edit_history[0].contains("Date:"));
    }

    #[test]
    fn test_position_weight() {
        // High weight positions (natural pause points)
        assert_eq!(position_weight("foo(", 4), 10); // After '('
        assert_eq!(position_weight("a, b", 2), 10); // After ','
        assert_eq!(position_weight("x;", 2), 10); // After ';'
        assert_eq!(position_weight("a: b", 2), 10); // After ':'
        assert_eq!(position_weight("[", 1), 10); // After '['
        assert_eq!(position_weight("{", 1), 10); // After '{'

        // High weight for closing brackets
        assert_eq!(position_weight("foo)", 4), 8); // After ')'
        assert_eq!(position_weight("]", 1), 8); // After ']'
        assert_eq!(position_weight("}", 1), 8); // After '}'

        // High weight at end of identifier
        assert_eq!(position_weight("foo ", 3), 8); // End of 'foo' before space
        assert_eq!(position_weight("bar(", 3), 8); // End of 'bar' before '('

        // Medium weight for operators
        assert_eq!(position_weight("a + b", 3), 5); // After '+'
        assert_eq!(position_weight("x.", 2), 5); // After '.'
        assert_eq!(position_weight("a=b", 2), 5); // After '='

        // Medium weight for whitespace
        assert_eq!(position_weight("a ", 2), 6); // After space

        // Low weight mid-identifier
        assert_eq!(position_weight("foobar", 3), 1); // Mid-identifier 'foo|bar'

        // Edge cases
        assert_eq!(position_weight("", 0), 1); // Empty string
        assert_eq!(position_weight("a", 0), 1); // Position 0
    }

    #[test]
    fn test_weighted_select() {
        // Test that weighted selection returns correct indices
        let weights = vec![1, 10, 1];

        // With total weight 12, seed 0 should select index 0
        // seed 0 % 12 = 0, cumulative: 1 at idx 0, so returns 0
        assert_eq!(weighted_select(&weights, 0), 0);

        // seed 1 % 12 = 1, cumulative: 1 at idx 0 (1 < 1 is false), 11 at idx 1 (1 < 11 is true)
        assert_eq!(weighted_select(&weights, 1), 1);

        // seed 10 % 12 = 10, cumulative: 1, 11 at idx 1 (10 < 11 is true)
        assert_eq!(weighted_select(&weights, 10), 1);

        // seed 11 % 12 = 11, cumulative: 1, 11 at idx 1 (11 < 11 is false), 12 at idx 2 (11 < 12 is true)
        assert_eq!(weighted_select(&weights, 11), 2);

        // Empty weights should return 0
        let empty: Vec<u32> = vec![];
        assert_eq!(weighted_select(&empty, 42), 0);

        // Single weight should always return index 0
        let single = vec![10];
        assert_eq!(weighted_select(&single, 0), 0);
        assert_eq!(weighted_select(&single, 100), 0);
    }

    #[test]
    fn test_weighted_split_prefers_natural_boundaries() {
        // Test that with different seeds, weighted selection tends to prefer
        // positions after punctuation over mid-identifier positions
        let text_with_punctuation = "foo(bar, baz)";
        let text_mid_identifier = "foobar";

        // Position after '(' should have high weight
        let weight_after_paren = position_weight(text_with_punctuation, 4);
        // Position after ',' should have high weight
        let weight_after_comma = position_weight(text_with_punctuation, 8);
        // Position mid-identifier should have low weight
        let weight_mid_ident = position_weight(text_mid_identifier, 3);

        assert!(
            weight_after_paren > weight_mid_ident,
            "After '(' ({}) should be weighted higher than mid-identifier ({})",
            weight_after_paren,
            weight_mid_ident
        );
        assert!(
            weight_after_comma > weight_mid_ident,
            "After ',' ({}) should be weighted higher than mid-identifier ({})",
            weight_after_comma,
            weight_mid_ident
        );
    }

    #[test]
    fn test_imitate_human_edits_pure_insertion() {
        // Source patch is empty (no edits yet)
        // Target patch has a pure insertion (adding a new line)
        let source = r#"--- a/test.rs
+++ b/test.rs
@@ -1,2 +1,2 @@
 fn main() {
 }
"#;
        let target = r#"--- a/test.rs
+++ b/test.rs
@@ -1,2 +1,3 @@
 fn main() {
+    println!("debug");
 }
"#;

        let (new_src, new_tgt, cursor) = imitate_human_edits(source, target, 42);

        // Should have transformed the patches
        assert_ne!(
            new_src, source,
            "Source should be modified for pure insertion"
        );
        assert_ne!(
            new_tgt, target,
            "Target should be modified for pure insertion"
        );
        assert!(cursor.is_some(), "Cursor should be set");

        // Source should now have a partial addition
        let src_patch = Patch::parse_unified_diff(&new_src);
        assert!(
            src_patch.stats().added > 0,
            "Source should have added lines"
        );

        // Target should have both a deletion (of partial) and addition (of full)
        let tgt_patch = Patch::parse_unified_diff(&new_tgt);
        assert!(
            tgt_patch.stats().removed > 0,
            "Target should have removed lines (partial)"
        );
        assert!(
            tgt_patch.stats().added > 0,
            "Target should have added lines (full)"
        );

        // The cursor should be in test.rs
        let cursor = cursor.unwrap();
        assert_eq!(cursor.file, "test.rs");
    }

    #[test]
    fn test_imitate_human_edits_pure_insertion_empty_source() {
        // Source patch has no hunks at all
        let source = "";
        let target = r#"--- a/test.rs
+++ b/test.rs
@@ -1,2 +1,3 @@
 fn main() {
+    println!("hello");
 }
"#;

        let (new_src, _new_tgt, cursor) = imitate_human_edits(source, target, 123);

        // Should have created a source patch with partial insertion
        assert!(!new_src.is_empty(), "Source should not be empty");
        assert!(cursor.is_some(), "Cursor should be set");

        let src_patch = Patch::parse_unified_diff(&new_src);
        assert!(
            src_patch.stats().added > 0,
            "Source should have added lines"
        );
    }

    #[test]
    fn test_imitate_human_edits_pure_insertion_intermediate_content() {
        // Verify the actual intermediate content is a realistic partial typing state
        let source = "";
        let target = r#"--- a/test.rs
+++ b/test.rs
@@ -1,2 +1,3 @@
 fn main() {
+    println!("hello world");
 }
"#;

        // Test with multiple seeds to see different split points
        let mut found_partial = false;
        for seed in 1..=50 {
            let (new_src, new_tgt, cursor) = imitate_human_edits(source, target, seed);

            if cursor.is_some() {
                let src_patch = Patch::parse_unified_diff(&new_src);
                let tgt_patch = Patch::parse_unified_diff(&new_tgt);

                // Find the added line in source
                for hunk in &src_patch.hunks {
                    for line in &hunk.lines {
                        if let PatchLine::Addition(content) = line {
                            // The partial line should be a prefix of the full line
                            let full_line = "    println!(\"hello world\");";
                            if content != full_line && full_line.starts_with(content) {
                                found_partial = true;

                                // Verify target has the partial as deletion
                                let mut has_deletion = false;
                                for tgt_hunk in &tgt_patch.hunks {
                                    for tgt_line in &tgt_hunk.lines {
                                        if let PatchLine::Deletion(del_content) = tgt_line {
                                            if del_content == content {
                                                has_deletion = true;
                                            }
                                        }
                                    }
                                }
                                assert!(
                                    has_deletion,
                                    "Target should have deletion of partial line"
                                );
                            }
                        }
                    }
                }
            }
        }

        assert!(
            found_partial,
            "At least one seed should produce a partial intermediate state"
        );
    }
}
