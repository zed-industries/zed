//! `ep split-commit` implementation.
//!
//! This command generates a single evaluation example JSON object from a
//! chronologically-ordered unified diff (a "commit").
//!
//! TODO: Port Python code to generate chronologically-ordered commits
use crate::FailedHandling;
use crate::reorder_patch::{
    EditLocation, Patch, PatchLine, edit_locations, extract_edits, locate_edited_line,
};
use crate::word_diff::tokenize;

/// Find the largest valid UTF-8 char boundary at or before `index` in `s`.
fn floor_char_boundary(s: &str, index: usize) -> usize {
    if index >= s.len() {
        s.len()
    } else if s.is_char_boundary(index) {
        index
    } else {
        // Find the nearest valid character boundary at or before index
        (0..index)
            .rev()
            .find(|&i| s.is_char_boundary(i))
            .unwrap_or(0)
    }
}
use anyhow::{Context as _, Result};
use clap::Args;
use edit_prediction::example_spec::ExampleSpec;
use rand::Rng;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use serde::Deserialize;
use similar::{DiffTag, TextDiff};
use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;

const MAX_SPLIT_POINT_SAMPLING_ATTEMPTS: usize = 10;
const SAME_FILE_NEAR_LINE_THRESHOLD: usize = 30;

/// A commit has no split point matching the requested kind. This is an
/// expected outcome when filtering by kind, so such commits are skipped
/// rather than treated as failures.
#[derive(Debug)]
pub struct NoMatchingSplitPointError {
    kind: SplitPointKind,
}

impl std::fmt::Display for NoMatchingSplitPointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no split point found matching {}", self.kind)
    }
}

impl std::error::Error for NoMatchingSplitPointError {}

/// `ep split-commit` CLI args.
#[derive(Debug, Args, Clone)]
pub struct SplitCommitArgs {
    /// Split point (float 0.0-1.0 for fraction, integer for index, or one of: fim, same-file-near, same-file-far, cross-file; append :<index-or-fraction> to validate a specific split)
    #[arg(long, short = 's')]
    pub split_point: Option<String>,

    /// Random seed for reproducibility
    #[arg(long)]
    pub seed: Option<u64>,

    /// Pretty-print JSON output
    #[arg(long, short = 'p')]
    pub pretty: bool,

    /// Number of samples to generate per commit (samples random split points)
    #[arg(long, short = 'n')]
    pub num_samples: Option<usize>,
}

/// Input format for annotated commits (JSON Lines).
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct AnnotatedCommit {
    /// Repository path (e.g., "repos/zed")
    pub repo: String,
    /// Repository URL (e.g., "https://github.com/zed-industries/zed")
    pub repo_url: String,
    /// Commit SHA
    pub commit_sha: String,
    /// Chronologically reordered commit diff
    pub reordered_commit: String,
    /// Original commit diff
    pub original_commit: String,
    /// Whether diff stats match between original and reordered
    pub diff_stats_match: bool,
}

/// Cursor position in a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorPosition {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub line_length: usize,
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

/// Split point specification for evaluation generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitPointKind {
    Fim,
    SameFileNear,
    SameFileFar,
    CrossFile,
}

impl std::fmt::Display for SplitPointKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SplitPointKind::Fim => write!(f, "fim"),
            SplitPointKind::SameFileNear => write!(f, "same-file-near"),
            SplitPointKind::SameFileFar => write!(f, "same-file-far"),
            SplitPointKind::CrossFile => write!(f, "cross-file"),
        }
    }
}

impl std::str::FromStr for SplitPointKind {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "fim" => Ok(Self::Fim),
            "same-file-near" => Ok(Self::SameFileNear),
            "same-file-far" => Ok(Self::SameFileFar),
            "cross-file" => Ok(Self::CrossFile),
            _ => anyhow::bail!(
                "invalid split point kind '{value}' (expected fim, same-file-near, same-file-far, or cross-file)"
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SplitPoint {
    /// Fraction of total edits (0.0 to 1.0)
    Fraction(f64),
    /// Absolute index
    Index(usize),
    /// Random split point matching the requested kind.
    Kind(SplitPointKind),
    /// Explicit split point that must match the requested kind.
    KindWithSplit {
        kind: SplitPointKind,
        split_point: SplitPointValue,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SplitPointValue {
    Fraction(f64),
    Index(usize),
}

fn parse_split_point_value(value: &str) -> Result<SplitPointValue> {
    if value.contains('.') {
        value
            .parse::<f64>()
            .map(SplitPointValue::Fraction)
            .with_context(|| format!("invalid split point fraction '{value}'"))
    } else {
        value
            .parse::<usize>()
            .map(SplitPointValue::Index)
            .with_context(|| format!("invalid split point index '{value}'"))
    }
}

fn parse_split_point(value: &str) -> Result<SplitPoint> {
    if let Some((kind, split_point)) = value.split_once(':') {
        let kind = kind.parse::<SplitPointKind>()?;
        anyhow::ensure!(
            !split_point.is_empty(),
            "missing split point after kind '{kind}:'"
        );
        return Ok(SplitPoint::KindWithSplit {
            kind,
            split_point: parse_split_point_value(split_point)?,
        });
    }

    if let Ok(kind) = value.parse::<SplitPointKind>() {
        return Ok(SplitPoint::Kind(kind));
    }

    match parse_split_point_value(value)? {
        SplitPointValue::Fraction(value) => Ok(SplitPoint::Fraction(value)),
        SplitPointValue::Index(value) => Ok(SplitPoint::Index(value)),
    }
}

fn is_service_file(path: &str) -> bool {
    let path = path.trim();
    let path = path
        .strip_prefix("a/")
        .or_else(|| path.strip_prefix("b/"))
        .unwrap_or(path)
        .trim_start_matches("./");

    if path.is_empty() || path == "/dev/null" {
        return true;
    }

    let file_name = path.rsplit('/').next().unwrap_or(path);
    if matches!(
        file_name,
        "package.json"
            | "package-lock.json"
            | "pnpm-lock.yaml"
            | "Cargo.lock"
            | "yarn.lock"
            | "bun.lock"
            | "bun.lockb"
            | "go.sum"
            | "composer.lock"
            | "Gemfile.lock"
            | "Pipfile.lock"
            | "poetry.lock"
            | "uv.lock"
            | ".gitlab-ci.yml"
            | ".travis.yml"
            | "azure-pipelines.yml"
            | "Jenkinsfile"
    ) {
        return true;
    }

    if file_name.ends_with(".min.js")
        || file_name.ends_with(".bundle.js")
        || file_name.contains(".generated.")
        || file_name.ends_with(".pb.go")
    {
        return true;
    }

    if path == ".github/workflows"
        || path.starts_with(".github/workflows/")
        || path == ".circleci"
        || path.starts_with(".circleci/")
    {
        return true;
    }

    path.split('/').any(|component| {
        matches!(
            component,
            "dist" | "build" | "coverage" | "node_modules" | "vendor"
        )
    })
}

fn edit_starts_on_service_file(patch: &Patch, split_pos: usize) -> bool {
    locate_edited_line(patch, split_pos as isize)
        .is_some_and(|edit_location| is_service_file(&edit_location.filename))
}

fn has_submodule_gitlink_hunk(commit: &str) -> bool {
    commit.lines().any(line_indicates_submodule_gitlink)
}

fn line_indicates_submodule_gitlink(line: &str) -> bool {
    let line = line.trim();

    matches!(
        line,
        "new file mode 160000" | "deleted file mode 160000" | "old mode 160000" | "new mode 160000"
    ) || line
        .strip_prefix("index ")
        .and_then(|line| line.split_whitespace().last())
        .is_some_and(|mode| mode == "160000")
        || line
            .strip_prefix('+')
            .or_else(|| line.strip_prefix('-'))
            .is_some_and(|line| line.starts_with("Subproject commit "))
}

fn sample_split_point(patch: &Patch, rng: &mut dyn rand::RngCore) -> usize {
    let stats = patch.stats();
    let num_edits = stats.added + stats.removed;
    if num_edits == 0 {
        return 0;
    }

    let mut split = rng.random_range(1..=num_edits);
    for _ in 1..MAX_SPLIT_POINT_SAMPLING_ATTEMPTS {
        if !edit_starts_on_service_file(patch, split) {
            break;
        }
        split = rng.random_range(1..=num_edits);
    }

    split
}

fn resolve_split_point_value(split_point: SplitPointValue, num_edits: usize) -> usize {
    match split_point {
        SplitPointValue::Fraction(fraction) => {
            let split = (fraction * num_edits as f64).floor() as usize;
            split.min(num_edits)
        }
        SplitPointValue::Index(index) => index.min(num_edits),
    }
}

#[derive(Debug, Clone)]
struct GeneratedSplitCommit {
    split: usize,
    split_commit: SplitCommit,
    cursor: CursorPosition,
    cursor_from_human_edit: bool,
}

fn generate_split_commit_at_split(
    patch: &Patch,
    split: usize,
    rng: &mut dyn rand::RngCore,
) -> Result<GeneratedSplitCommit> {
    let (prefix, suffix) = split_ordered_patch(patch, split);

    let mut split_commit = SplitCommit {
        source_patch: prefix,
        target_patch: suffix,
    };

    let human_edit_seed = rng.random_range(1..=10000u64);
    let (src_patch, tgt_patch, cursor_opt) = imitate_human_edits(
        &split_commit.source_patch,
        &split_commit.target_patch,
        human_edit_seed,
    );
    split_commit.source_patch = src_patch;
    split_commit.target_patch = tgt_patch;

    let cursor_from_human_edit = cursor_opt.is_some();
    let cursor = match cursor_opt {
        Some(cursor) => cursor,
        None => sample_cursor_position(&split_commit, rng)
            .context("failed to sample cursor position")?,
    };

    Ok(GeneratedSplitCommit {
        split,
        split_commit,
        cursor,
        cursor_from_human_edit,
    })
}

fn classify_generated_split_commit(
    generated_split_commit: &GeneratedSplitCommit,
) -> Option<SplitPointKind> {
    let target_patch = Patch::parse_unified_diff(&generated_split_commit.split_commit.target_patch);
    let next_edit = locate_edited_line(&target_patch, 0)?;

    if next_edit.filename != generated_split_commit.cursor.file {
        return Some(SplitPointKind::CrossFile);
    }

    if generated_split_commit.cursor_from_human_edit
        && next_edit.target_line_number == generated_split_commit.cursor.line
    {
        return Some(SplitPointKind::Fim);
    }

    let line_distance = next_edit
        .target_line_number
        .abs_diff(generated_split_commit.cursor.line);
    if line_distance <= SAME_FILE_NEAR_LINE_THRESHOLD {
        Some(SplitPointKind::SameFileNear)
    } else {
        Some(SplitPointKind::SameFileFar)
    }
}

/// Cheap necessary condition for a split to be classifiable as `kind`,
/// computed from the full patch without generating the split.
///
/// The cursor ends up either at the first target edit (or, via
/// `imitate_human_edits`, on its line), or at the last source edit. So the
/// edits adjacent to the split bound what classifications are reachable.
/// Line numbers here are in full-patch coordinates, which can drift slightly
/// from split-patch coordinates, so this is a heuristic pre-filter; the final
/// classification is always verified on the generated split.
fn split_can_match_kind(
    edit_locations: &[EditLocation],
    split: usize,
    kind: SplitPointKind,
) -> bool {
    let (Some(previous_edit), Some(next_edit)) = (
        split.checked_sub(1).and_then(|i| edit_locations.get(i)),
        edit_locations.get(split),
    ) else {
        return false;
    };

    match kind {
        SplitPointKind::Fim => matches!(next_edit.patch_line, PatchLine::Addition(_)),
        SplitPointKind::SameFileNear => true,
        SplitPointKind::SameFileFar => {
            previous_edit.filename == next_edit.filename
                && previous_edit
                    .target_line_number
                    .abs_diff(next_edit.target_line_number)
                    > SAME_FILE_NEAR_LINE_THRESHOLD
        }
        SplitPointKind::CrossFile => previous_edit.filename != next_edit.filename,
    }
}

fn sample_split_commit_of_kind(
    patch: &Patch,
    kind: SplitPointKind,
    rng: &mut dyn rand::RngCore,
) -> Result<GeneratedSplitCommit> {
    let edit_locations = edit_locations(patch);
    let num_edits = edit_locations.len();

    let mut candidate_splits: Vec<usize> = (1..num_edits)
        .filter(|&split| {
            !edit_locations
                .get(split)
                .is_some_and(|next_edit| is_service_file(&next_edit.filename))
                && split_can_match_kind(&edit_locations, split, kind)
        })
        .collect();
    candidate_splits.shuffle(rng);

    for split in candidate_splits {
        for _ in 0..MAX_SPLIT_POINT_SAMPLING_ATTEMPTS {
            let Ok(generated_split_commit) = generate_split_commit_at_split(patch, split, rng)
            else {
                continue;
            };

            if classify_generated_split_commit(&generated_split_commit) == Some(kind) {
                return Ok(generated_split_commit);
            }
        }
    }

    Err(NoMatchingSplitPointError { kind }.into())
}

/// Entry point for the `ep split-commit` subcommand.
///
/// This runs synchronously and outputs JSON Lines (one output per input line).
pub fn run_split_commit(
    args: &SplitCommitArgs,
    inputs: &[PathBuf],
    output_path: Option<&PathBuf>,
    failed: FailedHandling,
) -> Result<()> {
    use std::collections::HashSet;
    use std::io::BufRead;

    let stdin_path = PathBuf::from("-");
    let inputs = if inputs.is_empty() {
        std::slice::from_ref(&stdin_path)
    } else {
        inputs
    };

    let split_point = args
        .split_point
        .as_deref()
        .map(parse_split_point)
        .transpose()?;
    let mut output_lines = Vec::new();
    let mut processed_commits = 0usize;

    for input_path in inputs {
        let input: Box<dyn BufRead> = if input_path.as_os_str() == "-" {
            Box::new(io::BufReader::new(io::stdin()))
        } else {
            let file = fs::File::open(input_path)
                .with_context(|| format!("failed to open input file {}", input_path.display()))?;
            Box::new(io::BufReader::new(file))
        };

        for (line_num, line_result) in input.lines().enumerate() {
            let line =
                line_result.with_context(|| format!("failed to read line {}", line_num + 1))?;

            if line.trim().is_empty() {
                continue;
            }

            let annotated: AnnotatedCommit = serde_json::from_str(&line)
                .with_context(|| format!("failed to parse JSON at line {}", line_num + 1))?;

            // Generate multiple samples if num_samples is set
            if let Some(num_samples) = args.num_samples {
                let mut seen_samples: HashSet<String> = HashSet::new();
                let base_seed = args.seed.unwrap_or_else(|| rand::random());

                for sample_idx in 0..num_samples {
                    let sample_seed = base_seed.wrapping_add(sample_idx as u64);

                    let case = match generate_evaluation_example_from_ordered_commit(
                        &annotated.reordered_commit,
                        &annotated.repo_url,
                        &annotated.commit_sha,
                        split_point.clone(),
                        Some(sample_seed),
                        Some(sample_idx),
                    ) {
                        Ok(case) => case,
                        Err(e) => {
                            let err_msg = format!(
                                "failed to generate evaluation example for commit {} at line {} (sample {}): {}",
                                annotated.commit_sha,
                                line_num + 1,
                                sample_idx,
                                e
                            );
                            if e.is::<NoMatchingSplitPointError>() {
                                eprintln!("skipping: {}", err_msg);
                                continue;
                            }
                            match failed {
                                FailedHandling::Skip | FailedHandling::SkipNoFiles => {
                                    eprintln!("{}", err_msg);
                                    continue;
                                }
                                FailedHandling::Keep => {
                                    anyhow::bail!(err_msg);
                                }
                            }
                        }
                    };

                    let json = if args.pretty {
                        serde_json::to_string_pretty(&case)
                    } else {
                        serde_json::to_string(&case)
                    }
                    .context("failed to serialize evaluation case as JSON")?;

                    // Only add unique samples (different split points may produce same result)
                    if seen_samples.insert(json.clone()) {
                        output_lines.push(json);
                    }
                }
            } else {
                let case = match generate_evaluation_example_from_ordered_commit(
                    &annotated.reordered_commit,
                    &annotated.repo_url,
                    &annotated.commit_sha,
                    split_point.clone(),
                    args.seed,
                    None,
                ) {
                    Ok(case) => case,
                    Err(e) => {
                        let err_msg = format!(
                            "failed to generate evaluation example for commit {} at line {}: {}",
                            annotated.commit_sha,
                            line_num + 1,
                            e
                        );
                        if e.is::<NoMatchingSplitPointError>() {
                            eprintln!("skipping: {}", err_msg);
                            continue;
                        }
                        match failed {
                            FailedHandling::Skip | FailedHandling::SkipNoFiles => {
                                eprintln!("{}", err_msg);
                                continue;
                            }
                            FailedHandling::Keep => {
                                anyhow::bail!(err_msg);
                            }
                        }
                    }
                };

                let json = if args.pretty {
                    serde_json::to_string_pretty(&case)
                } else {
                    serde_json::to_string(&case)
                }
                .context("failed to serialize evaluation case as JSON")?;

                output_lines.push(json);
            }

            processed_commits += 1;
            eprint!(
                "\rsplit-commit: processed {} commits, generated {} examples",
                processed_commits,
                output_lines.len()
            );
            io::stderr()
                .flush()
                .context("failed to flush progress to stderr")?;
        }
    }

    if processed_commits > 0 {
        eprintln!();
    }

    let output_content = output_lines.join("\n") + if output_lines.is_empty() { "" } else { "\n" };

    if let Some(path) = output_path {
        fs::write(path, &output_content)
            .with_context(|| format!("failed to write output to {}", path.display()))?;
    } else {
        io::stdout()
            .write_all(output_content.as_bytes())
            .context("failed to write to stdout")?;
    }

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
/// * `sample_num` - Optional sample number for generating unique names
pub fn generate_evaluation_example_from_ordered_commit(
    commit: &str,
    repository_url: &str,
    commit_hash: &str,
    split_point: Option<SplitPoint>,
    seed: Option<u64>,
    sample_num: Option<usize>,
) -> Result<ExampleSpec> {
    anyhow::ensure!(
        !has_submodule_gitlink_hunk(commit),
        "commit contains submodule/gitlink hunk"
    );

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

    // Compute the split point
    let stats = patch.stats();
    let num_edits = stats.added + stats.removed;

    anyhow::ensure!(num_edits != 0, "no edits found in commit");

    let generated_split_commit = match split_point {
        None => {
            let split = sample_split_point(&patch, rng.as_mut());
            generate_split_commit_at_split(&patch, split, rng.as_mut())?
        }
        Some(SplitPoint::Fraction(fraction)) => {
            let split = resolve_split_point_value(SplitPointValue::Fraction(fraction), num_edits);
            generate_split_commit_at_split(&patch, split, rng.as_mut())?
        }
        Some(SplitPoint::Index(index)) => {
            let split = resolve_split_point_value(SplitPointValue::Index(index), num_edits);
            generate_split_commit_at_split(&patch, split, rng.as_mut())?
        }
        Some(SplitPoint::Kind(kind)) => sample_split_commit_of_kind(&patch, kind, rng.as_mut())?,
        Some(SplitPoint::KindWithSplit { kind, split_point }) => {
            let split = resolve_split_point_value(split_point, num_edits);
            let generated_split_commit =
                generate_split_commit_at_split(&patch, split, rng.as_mut())?;
            let actual_kind = classify_generated_split_commit(&generated_split_commit);
            anyhow::ensure!(
                actual_kind == Some(kind),
                "split point {split} classified as {}, expected {kind}",
                actual_kind
                    .map(|kind| kind.to_string())
                    .unwrap_or_else(|| "empty-target".to_string())
            );
            generated_split_commit
        }
    };

    let split = generated_split_commit.split;
    let cursor = generated_split_commit.cursor;
    let mut split_commit = generated_split_commit.split_commit;

    // Get cursor excerpt
    let cursor_excerpt = get_cursor_excerpt(
        &cursor,
        &split_commit.source_patch,
        &split_commit.target_patch,
    )
    .context("failed to generate cursor excerpt")?;

    // Where the source patch is empty, there's not enough info to make a
    // meaningful prediction
    if split == 0 {
        split_commit.target_patch = String::new();
    }

    let repo_name = repository_url
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("unknown");
    let short_sha = &commit_hash[..commit_hash.len().min(8)];
    let name = match sample_num {
        Some(n) => format!("{}-{}-{}", repo_name, short_sha, n),
        None => format!("{}-{}", repo_name, short_sha),
    };

    Ok(ExampleSpec {
        name,
        repository_url: repository_url.to_string(),
        revision: format!("{}~1", commit_hash),
        edit_history: split_commit.source_patch.clone(),
        cursor_path: Path::new(&cursor.file).into(),
        cursor_position: cursor_excerpt,
        expected_patches: vec![split_commit.target_patch],
        tags: vec![],
        reasoning: None,
        uncommitted_diff: String::new(),
        recently_opened_files: Vec::new(),
        recently_viewed_files: Vec::new(),
        uncommitted_diff_contains_edit_history: false,
        rejected_patch: None,

        telemetry: None,
        human_feedback: Vec::new(),
        rating: None,
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
pub fn split_ordered_patch(patch: &Patch, split_pos: usize) -> (String, String) {
    let source_edits: BTreeSet<usize> = (0..split_pos).collect();
    let (source, mut target) = extract_edits(patch, &source_edits);
    if !target.hunks.is_empty() {
        if let Some(header) = header_for_edit(patch, split_pos) {
            target.header = header;
        }
    }

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

fn header_for_edit(patch: &Patch, edit_index: usize) -> Option<String> {
    let edit_index = edit_index.try_into().ok()?;
    let edit_location = locate_edited_line(patch, edit_index)?;
    header_for_hunk(patch, edit_location.hunk_index)
}

fn header_for_hunk(patch: &Patch, hunk_index: usize) -> Option<String> {
    for hunk in patch.hunks.get(..hunk_index)?.iter().rev() {
        let mut header_lines = Vec::new();
        for line in hunk.lines.iter().rev() {
            let PatchLine::Garbage(line) = line else {
                break;
            };
            if line.trim().is_empty() && header_lines.is_empty() {
                continue;
            }
            if !line.starts_with("//") {
                break;
            }
            header_lines.push(line.as_str());
        }
        if !header_lines.is_empty() {
            return Some(render_reversed_header_lines(header_lines));
        }
    }

    let header_lines = patch
        .header
        .lines()
        .rev()
        .skip_while(|line| line.trim().is_empty())
        .take_while(|line| line.starts_with("//"))
        .collect::<Vec<_>>();
    (!header_lines.is_empty()).then(|| render_reversed_header_lines(header_lines))
}

fn render_reversed_header_lines(mut lines: Vec<&str>) -> String {
    lines.reverse();
    lines.join("\n") + "\n"
}

/// Calculate the weight for a split byte offset in `text`.
///
/// Higher weights indicate more natural pause points (e.g., after punctuation,
/// at identifier boundaries). Lower weights indicate less natural points
/// (e.g., mid-identifier).
fn position_weight(text: &str, byte_offset: usize) -> u32 {
    if byte_offset == 0 || byte_offset > text.len() || !text.is_char_boundary(byte_offset) {
        return 1;
    }

    let Some(prev_char) = text[..byte_offset].chars().next_back() else {
        return 1;
    };
    let next_char = text[byte_offset..].chars().next();

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
    let is_next_word_char = next_char.is_some_and(|ch| ch.is_alphanumeric() || ch == '_');

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
#[cfg(test)]
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

#[derive(Clone, Copy)]
struct CandidateSplit {
    edit_byte_offset: usize,
    weight: u32,
}

fn push_typed_text_candidates(
    candidates: &mut Vec<CandidateSplit>,
    edit_start_byte_offset: usize,
    final_line: &str,
    final_line_start_byte_offset: usize,
    typed_text: &str,
) {
    for (byte_offset, character) in typed_text.char_indices() {
        let next_byte_offset = byte_offset + character.len_utf8();
        let final_line_candidate_byte_offset = final_line_start_byte_offset + next_byte_offset;
        if final_line[..final_line_candidate_byte_offset]
            .trim()
            .is_empty()
        {
            continue;
        }
        candidates.push(CandidateSplit {
            edit_byte_offset: edit_start_byte_offset + next_byte_offset,
            weight: position_weight(final_line, final_line_candidate_byte_offset),
        });
    }
}

fn push_deleted_text_candidates(
    candidates: &mut Vec<CandidateSplit>,
    edit_start_byte_offset: usize,
    deleted_text: &str,
) {
    for (byte_offset, character) in deleted_text.char_indices() {
        candidates.push(CandidateSplit {
            edit_byte_offset: edit_start_byte_offset + byte_offset + character.len_utf8(),
            weight: 2,
        });
    }
}

fn weighted_select_candidate(candidates: &[CandidateSplit], seed: u64) -> Option<CandidateSplit> {
    if candidates.is_empty() {
        return None;
    }

    let total_weight: u64 = candidates
        .iter()
        .map(|candidate| candidate.weight as u64)
        .sum();
    if total_weight == 0 {
        return Some(candidates[seed as usize % candidates.len()]);
    }

    let target = seed % total_weight;
    let mut cumulative: u64 = 0;

    for candidate in candidates {
        cumulative += candidate.weight as u64;
        if target < cumulative {
            return Some(*candidate);
        }
    }

    candidates.last().copied()
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

    let source_edit_locations = edit_locations(&src_patch);
    let src_edit_loc = source_edit_locations.last().cloned();

    let src_has_edit_at_target_line = source_edit_locations.iter().any(|loc| {
        loc.filename == tgt_edit_loc.filename
            && loc.target_line_number == tgt_edit_loc.target_line_number
    });

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

    // Use similar to get diff operations
    let diff = TextDiff::from_slices(&src_tokens, &tgt_tokens);

    let mut candidate_splits = Vec::new();
    let mut edit_byte_offset = 0usize;
    let mut final_line_byte_offset = 0usize;

    for op in diff.ops() {
        match op.tag() {
            DiffTag::Equal => {
                let equal_text: String = op.old_range().map(|i| src_tokens[i]).collect();
                final_line_byte_offset += equal_text.len();
            }
            DiffTag::Replace => {
                let inserted_text: String = op.new_range().map(|i| tgt_tokens[i]).collect();
                let deleted_text: String = op.old_range().map(|i| src_tokens[i]).collect();
                push_typed_text_candidates(
                    &mut candidate_splits,
                    edit_byte_offset,
                    &tgt_line,
                    final_line_byte_offset,
                    &inserted_text,
                );
                push_deleted_text_candidates(
                    &mut candidate_splits,
                    edit_byte_offset + inserted_text.len(),
                    &deleted_text,
                );
                edit_byte_offset += inserted_text.len() + deleted_text.len();
                final_line_byte_offset += inserted_text.len();
            }
            DiffTag::Insert => {
                let inserted_text: String = op.new_range().map(|i| tgt_tokens[i]).collect();
                push_typed_text_candidates(
                    &mut candidate_splits,
                    edit_byte_offset,
                    &tgt_line,
                    final_line_byte_offset,
                    &inserted_text,
                );
                edit_byte_offset += inserted_text.len();
                final_line_byte_offset += inserted_text.len();
            }
            DiffTag::Delete => {
                let deleted_text: String = op.old_range().map(|i| src_tokens[i]).collect();
                push_deleted_text_candidates(
                    &mut candidate_splits,
                    edit_byte_offset,
                    &deleted_text,
                );
                edit_byte_offset += deleted_text.len();
            }
        }
    }

    let Some(selected_split) = weighted_select_candidate(&candidate_splits, seed) else {
        return no_change;
    };
    let split_byte_offset = selected_split.edit_byte_offset;

    let mut edit_index = 0usize;
    let mut new_src = String::new();
    let mut split_found = false;
    let mut last_old_end = 0usize;

    for op in diff.ops() {
        match op.tag() {
            DiffTag::Equal => {
                for i in op.old_range() {
                    new_src.push_str(src_tokens[i]);
                }
                last_old_end = op.old_range().end;
            }
            DiffTag::Replace => {
                // Handle replace as delete + insert
                let del: String = op.old_range().map(|i| src_tokens[i]).collect();
                let ins: String = op.new_range().map(|i| tgt_tokens[i]).collect();
                let repl_len = del.len() + ins.len();
                if edit_index + repl_len >= split_byte_offset {
                    // Split within this replace operation
                    let offset = split_byte_offset - edit_index;
                    if offset < ins.len() {
                        let safe_offset = floor_char_boundary(&ins, offset);
                        new_src.push_str(&ins[..safe_offset]);
                    } else {
                        new_src.push_str(&ins);
                        let del_offset = offset - ins.len();
                        let safe_del_offset = floor_char_boundary(&del, del_offset.min(del.len()));
                        new_src.push_str(&del[..safe_del_offset]);
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
                let repl: String = op.new_range().map(|i| tgt_tokens[i]).collect();
                if edit_index + repl.len() >= split_byte_offset {
                    let offset = split_byte_offset - edit_index;
                    let safe_offset = floor_char_boundary(&repl, offset);
                    new_src.push_str(&repl[..safe_offset]);
                    split_found = true;
                    break;
                } else {
                    edit_index += repl.len();
                    new_src.push_str(&repl);
                }
            }
            DiffTag::Delete => {
                let repl: String = op.old_range().map(|i| src_tokens[i]).collect();
                if edit_index + repl.len() >= split_byte_offset {
                    let offset = split_byte_offset - edit_index;
                    let safe_offset = floor_char_boundary(&repl, offset);
                    new_src.push_str(&repl[..safe_offset]);
                    split_found = true;
                    last_old_end = op.old_range().start + safe_offset.min(op.old_range().len());
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
    let line = if is_replacement {
        src_edit_loc.as_ref().unwrap().source_line_number
    } else {
        tgt_edit_loc.target_line_number
    };
    let column = new_src.len() + 1;

    // Add remainder of source if similar enough to target remainder
    let remainder_src: String = (last_old_end..src_tokens.len())
        .map(|i| src_tokens[i])
        .collect();
    let remainder_tgt: String = (last_old_end..tgt_tokens.len())
        .filter_map(|i| tgt_tokens.get(i).copied())
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

    let cursor = CursorPosition {
        file: tgt_edit_loc.filename.clone(),
        line,
        column: column.min(new_src.len()),
        line_length: new_src.len(),
    };

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
        // For pure insertions, insert after the last edit in source patch
        // This imitates human typing - the intermediate content is what the user is currently typing
        let last_src_edit = locate_edited_line(&new_src_patch, -1);

        if let Some(src_loc) = last_src_edit {
            // Insert after the last edit in source
            if let Some(hunk) = new_src_patch.hunks.get_mut(src_loc.hunk_index) {
                hunk.lines.insert(
                    src_loc.line_index_within_hunk + 1,
                    PatchLine::Addition(new_src.clone()),
                );
                hunk.new_count += 1;
            }
        } else {
            // Source patch is empty or has incompatible hunk structure, create a new hunk based on target
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

    let (line, column, line_length) = match &loc.patch_line {
        PatchLine::Addition(content) => (loc.target_line_number, content.len(), content.len()),
        PatchLine::Deletion(_) => (loc.target_line_number, 1, 1),
        _ => return None,
    };

    Some(CursorPosition {
        file: loc.filename,
        line,
        column,
        line_length,
    })
}

/// Locate the beginning of the first edit in a patch.
fn locate_beginning_of_first_edit(patch: &Patch) -> Option<CursorPosition> {
    let loc = locate_edited_line(patch, 0)?;

    let hunk = patch.hunks.get(loc.hunk_index)?;
    let line_length = if loc.line_index_within_hunk > 0 {
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
    let column = line_length.saturating_sub(1);

    Some(CursorPosition {
        file: loc.filename,
        line,
        column,
        line_length,
    })
}

/// Sample cursor position according to the following rules:
/// 1. 80% chance of cursor being at the end of the source patch
/// 2. 20% chance of cursor being at the beginning of the target patch
/// 3. 20% chance of adding a jitter offset
pub fn sample_cursor_position(
    split_commit: &SplitCommit,
    rng: &mut dyn rand::RngCore,
) -> Option<CursorPosition> {
    // End of history
    let src_patch = Patch::parse_unified_diff(&split_commit.source_patch);
    let src_cursor = locate_end_of_last_edit(&src_patch);

    // Beginning of target
    let tgt_patch = Patch::parse_unified_diff(&split_commit.target_patch);
    let tgt_cursor = locate_beginning_of_first_edit(&tgt_patch);

    // Randomly pick a cursor position
    let prefer_source = rng.random_bool(0.8);
    let mut cursor = if prefer_source {
        src_cursor.or(tgt_cursor)
    } else {
        tgt_cursor.or(src_cursor)
    };

    // Possible add jitter
    let should_jitter = rng.random_bool(0.2);
    if should_jitter {
        if let Some(cursor) = cursor.as_mut() {
            let col_offset = rng.random_range(1..=5);
            if rng.random_bool(0.5) {
                cursor.column = cursor
                    .column
                    .saturating_add(col_offset)
                    .min(cursor.line_length);
            } else {
                cursor.column = cursor.column.saturating_sub(col_offset);
            }
        }
    }

    cursor
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
                // If hunk only has deletions (file deletion), include deletion lines
                if excerpt_lines.is_empty() {
                    excerpt_first_line = hunk.old_start as usize;
                    for line in &hunk.lines {
                        match line {
                            PatchLine::Deletion(s) => {
                                excerpt_lines.push(s.clone());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    // Search in target patch if not found
    if excerpt_lines.is_empty() {
        let tgt = Patch::parse_unified_diff(target_patch);
        // Search all hunks for the cursor file, not just the first edit's hunk
        for hunk in &tgt.hunks {
            if hunk.filename == cursor.file {
                excerpt_first_line = hunk.new_start as usize;
                // First try to collect deletions and context (what exists before edits)
                for line in &hunk.lines {
                    match line {
                        PatchLine::Deletion(s) | PatchLine::Context(s) => {
                            excerpt_lines.push(s.clone());
                        }
                        _ => {}
                    }
                }
                // If hunk only has additions (no deletions/context), include all lines
                // This handles cases like adding to an empty file or section
                if excerpt_lines.is_empty() {
                    for line in &hunk.lines {
                        match line {
                            PatchLine::Addition(s)
                            | PatchLine::Deletion(s)
                            | PatchLine::Context(s) => {
                                excerpt_lines.push(s.clone());
                            }
                            _ => {}
                        }
                    }
                }
                if !excerpt_lines.is_empty() {
                    break;
                }
            }
        }
    }

    // Also search source patch hunks if still not found (for fallback cursor case)
    if excerpt_lines.is_empty() {
        for hunk in &src.hunks {
            if hunk.filename == cursor.file {
                excerpt_first_line = hunk.new_start as usize;
                for line in &hunk.lines {
                    match line {
                        PatchLine::Addition(s) | PatchLine::Context(s) => {
                            excerpt_lines.push(s.clone());
                        }
                        _ => {}
                    }
                }
                // If hunk only has deletions, include deletion lines
                if excerpt_lines.is_empty() {
                    excerpt_first_line = hunk.old_start as usize;
                    for line in &hunk.lines {
                        match line {
                            PatchLine::Deletion(s) => {
                                excerpt_lines.push(s.clone());
                            }
                            _ => {}
                        }
                    }
                }
                if !excerpt_lines.is_empty() {
                    break;
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
            // Ensure we split at a valid UTF-8 character boundary
            let col = if line.is_char_boundary(col) {
                col
            } else {
                // Find the nearest valid character boundary
                (0..=col)
                    .rev()
                    .find(|&i| line.is_char_boundary(i))
                    .unwrap_or(0)
            };
            let (before, after) = line.split_at(col);
            *line = format!("{}<|user_cursor|>{}", before, after);
            break;
        }
    }

    Some(excerpt_lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use edit_prediction::example_spec::ExampleSpec;

    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("hello world");
        assert_eq!(tokens, vec!["hello", " ", "world"]);

        let tokens = tokenize("foo_bar123 + baz");
        assert_eq!(tokens, vec!["foo_bar123", " ", "+", " ", "baz"]);

        let tokens = tokenize("print(\"hello\")");
        assert_eq!(tokens, vec!["print", "(", "\"", "hello", "\"", ")"]);

        let tokens = tokenize("hello_world");
        assert_eq!(tokens, vec!["hello_world"]);

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

        let (source, target) = split_ordered_patch(&patch, 1);

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
        let (source, target) = split_ordered_patch(&patch, 1);

        let src_patch = Patch::parse_unified_diff(&source);
        let tgt_patch = Patch::parse_unified_diff(&target);

        // Source should have the deletion
        assert_eq!(src_patch.stats().removed, 1);
        // Target should have the addition
        assert_eq!(tgt_patch.stats().added, 1);
    }

    #[test]
    fn test_split_ordered_commit_target_header_continues_current_group() {
        let commit = r#"////////////////////////////////////////////////////////////////////////////////
// Update dependency version
////////////////////////////////////////////////////////////////////////////////
--- a/go.mod
+++ b/go.mod
@@ -1,3 +1,3 @@
 require (
-	gopkg.in/yaml.v3 v3.0.0 // indirect
+	gopkg.in/yaml.v3 v3.0.1 // indirect
 )
diff --git a/go.sum b/go.sum
index f71a068..b8cc3c2 100644
////////////////////////////////////////////////////////////////////////////////
// Update go.sum checksums
////////////////////////////////////////////////////////////////////////////////
--- a/go.sum
+++ b/go.sum
@@ -1,3 +1,5 @@
 gopkg.in/yaml.v3 v3.0.0 h1:old
 gopkg.in/yaml.v3 v3.0.0/go.mod h1:oldmod
+gopkg.in/yaml.v3 v3.0.1 h1:new
+gopkg.in/yaml.v3 v3.0.1/go.mod h1:newmod
diff --git a/lib/handler.go b/lib/handler.go
index 1827a70..d9b3ed1 100644
////////////////////////////////////////////////////////////////////////////////
// Fix error wrapping
////////////////////////////////////////////////////////////////////////////////
--- a/lib/handler.go
+++ b/lib/handler.go
@@ -1,3 +1,3 @@
-	return fmt.Errorf("failed: %s", err)
+	return fmt.Errorf("failed: %w", err)
"#;

        let (_source, target) = split_ordered_patch(&Patch::parse_unified_diff(commit), 3);

        assert!(
            target.starts_with(
                "////////////////////////////////////////////////////////////////////////////////\n// Update go.sum checksums\n////////////////////////////////////////////////////////////////////////////////\n"
            ),
            "target patch should continue with the active group header:\n{target}"
        );
        assert!(!target.starts_with(
            "////////////////////////////////////////////////////////////////////////////////\n// Update dependency version\n////////////////////////////////////////////////////////////////////////////////\n"
        ));
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
            None,
        );

        assert!(result.is_ok());
        let case = result.unwrap();
        assert_eq!(case.repository_url, "https://github.com/test/repo");
        assert_eq!(case.revision, "abc123~1");
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
            None,
        )
        .unwrap();

        let result2 = generate_evaluation_example_from_ordered_commit(
            commit,
            "https://github.com/test/repo",
            "abc123",
            Some(SplitPoint::Fraction(0.5)),
            Some(12345),
            None,
        )
        .unwrap();

        // Results should be identical
        assert_eq!(result1.edit_history, result2.edit_history);
        assert_eq!(result1.expected_patches, result2.expected_patches);
        assert_eq!(result1.cursor_position, result2.cursor_position);
    }

    #[test]
    fn test_cursor_position_display() {
        let cursor = CursorPosition {
            file: "src/main.rs".to_string(),
            line: 42,
            column: 10,
            line_length: 80,
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
    fn test_parse_typed_split_points() {
        assert_eq!(
            parse_split_point("fim").unwrap(),
            SplitPoint::Kind(SplitPointKind::Fim)
        );
        assert_eq!(
            parse_split_point("same-file-near").unwrap(),
            SplitPoint::Kind(SplitPointKind::SameFileNear)
        );
        assert_eq!(
            parse_split_point("same-file-far:2").unwrap(),
            SplitPoint::KindWithSplit {
                kind: SplitPointKind::SameFileFar,
                split_point: SplitPointValue::Index(2),
            }
        );
        assert_eq!(
            parse_split_point("cross-file:0.5").unwrap(),
            SplitPoint::KindWithSplit {
                kind: SplitPointKind::CrossFile,
                split_point: SplitPointValue::Fraction(0.5),
            }
        );
        assert!(parse_split_point("local").is_err());
    }

    fn assert_generated_split_kind(
        commit: &str,
        kind: SplitPointKind,
        seed: u64,
    ) -> GeneratedSplitCommit {
        let patch = Patch::parse_unified_diff(commit);
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let generated_split_commit = sample_split_commit_of_kind(&patch, kind, &mut rng).unwrap();
        assert_eq!(
            classify_generated_split_commit(&generated_split_commit),
            Some(kind)
        );
        generated_split_commit
    }

    #[test]
    fn test_classify_generated_split_commit() {
        let target_patch = r#"--- a/src/main.rs
+++ b/src/main.rs
@@ -10,3 +10,3 @@
 fn main() {
-old();
+new();
 }
"#;
        let mut generated_split_commit = GeneratedSplitCommit {
            split: 1,
            split_commit: SplitCommit {
                source_patch: String::new(),
                target_patch: target_patch.to_string(),
            },
            cursor: CursorPosition {
                file: "src/main.rs".to_string(),
                line: 11,
                column: 5,
                line_length: 10,
            },
            cursor_from_human_edit: true,
        };
        assert_eq!(
            classify_generated_split_commit(&generated_split_commit),
            Some(SplitPointKind::Fim)
        );

        generated_split_commit.cursor_from_human_edit = false;
        assert_eq!(
            classify_generated_split_commit(&generated_split_commit),
            Some(SplitPointKind::SameFileNear)
        );

        generated_split_commit.cursor.line = 100;
        assert_eq!(
            classify_generated_split_commit(&generated_split_commit),
            Some(SplitPointKind::SameFileFar)
        );

        generated_split_commit.cursor.file = "src/other.rs".to_string();
        assert_eq!(
            classify_generated_split_commit(&generated_split_commit),
            Some(SplitPointKind::CrossFile)
        );
    }

    #[test]
    fn test_sample_fim_split_point() {
        let commit = r#"--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,5 @@
 fn main() {
+    let first = 1;
+    let second = 2;
 }
"#;

        assert_generated_split_kind(commit, SplitPointKind::Fim, 1);
    }

    #[test]
    fn test_sample_same_file_near_split_point() {
        let commit = r#"--- a/src/main.rs
+++ b/src/main.rs
@@ -1,4 +1,5 @@
 fn main() {
+    let inserted = 0;
-    old();
+    new();
 }
"#;

        assert_generated_split_kind(commit, SplitPointKind::SameFileNear, 1);
    }

    #[test]
    fn test_sample_same_file_far_split_point() {
        let commit = r#"--- a/src/main.rs
+++ b/src/main.rs
@@ -1,2 +1,3 @@
 start
+source_edit();
 context
@@ -100,2 +101,2 @@
-far_old();
+far_new();
 end
"#;

        assert_generated_split_kind(commit, SplitPointKind::SameFileFar, 1);
    }

    #[test]
    fn test_sample_cross_file_split_point() {
        let commit = r#"--- a/src/main.rs
+++ b/src/main.rs
@@ -1,2 +1,3 @@
 fn main() {
+    source_edit();
 }
--- a/src/other.rs
+++ b/src/other.rs
@@ -1,3 +1,3 @@
 fn other() {
-    old();
+    new();
 }
"#;

        assert_generated_split_kind(commit, SplitPointKind::CrossFile, 1);
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
            None,
        );

        assert!(result.is_ok());
        let case = result.unwrap();

        // Source should have some edits
        let src_patch = Patch::parse_unified_diff(&case.edit_history);
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
            None,
        );

        assert!(result.is_ok());
        let case = result.unwrap();

        let src_patch = Patch::parse_unified_diff(&case.edit_history);
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
            None,
        )
        .unwrap();

        // Cursor excerpt should contain the cursor marker
        assert!(
            result.cursor_position.contains("<|user_cursor|>"),
            "Cursor excerpt should contain marker: {}",
            result.cursor_position
        );
    }

    #[test]
    fn test_evaluation_case_json_serialization() {
        let case = ExampleSpec {
            name: "test-abc123".to_string(),
            repository_url: "https://github.com/test/repo".to_string(),
            revision: "abc123~1".to_string(),
            edit_history: "patch1".to_string(),
            cursor_path: Path::new("file.rs").into(),
            cursor_position: "some code<|user_cursor|>".to_string(),
            expected_patches: vec!["patch".to_string()],
            tags: vec![],
            reasoning: None,
            uncommitted_diff: String::new(),
            recently_opened_files: Vec::new(),
            recently_viewed_files: Vec::new(),
            uncommitted_diff_contains_edit_history: false,
            rejected_patch: None,

            telemetry: None,
            human_feedback: Vec::new(),
            rating: None,
        };

        let json = serde_json::to_string(&case).unwrap();
        let deserialized: ExampleSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(case.repository_url, deserialized.repository_url);
        assert_eq!(case.revision, deserialized.revision);
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
            None,
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
            None,
        );

        assert!(result.is_ok());
        let case = result.unwrap();

        // The edit history should contain the group header (// lines)
        // but not the commit metadata
        assert!(!case.edit_history.contains("Author:"));
        assert!(!case.edit_history.contains("Date:"));
    }

    #[test]
    fn test_service_file_detection() {
        assert!(is_service_file("package.json"));
        assert!(is_service_file("frontend/yarn.lock"));
        assert!(is_service_file("a/src/generated/types.pb.go"));
        assert!(is_service_file("b/.github/workflows/ci.yml"));
        assert!(is_service_file("web/node_modules/pkg/index.js"));
        assert!(is_service_file("dist/app.bundle.js"));

        assert!(!is_service_file("src/main.rs"));
        assert!(!is_service_file("src/build.rs"));
        assert!(!is_service_file("Cargo.toml"));
    }

    #[test]
    fn test_edit_starts_on_service_file() {
        let commit = r#"--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,1 +1,2 @@
 fn lib() {}
+pub fn added() {}
--- a/package-lock.json
+++ b/package-lock.json
@@ -1,1 +1,2 @@
 {}
+{"lockfileVersion": 3}
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,1 +1,2 @@
 fn main() {}
+println!("hello");
"#;
        let patch = Patch::parse_unified_diff(commit);

        assert!(edit_starts_on_service_file(&patch, 1));
        assert!(!edit_starts_on_service_file(&patch, 2));
    }

    #[test]
    fn test_submodule_gitlink_hunk_detection() {
        assert!(has_submodule_gitlink_hunk(
            r#"diff --git a/controllers/llguidance b/controllers/llguidance
index 21e68b9..cadabda 160000
--- a/controllers/llguidance
+++ b/controllers/llguidance
@@ -1 +1 @@
-Subproject commit 21e68b916d4705107e1c45ea7bc927e829136258
+Subproject commit cadabdad21f3b81ff58b1918f8c23116b4ff7af3
"#
        ));
        assert!(has_submodule_gitlink_hunk(
            r#"--- a/controllers/derivre
+++ b/controllers/derivre
@@ -1 +1 @@
-Subproject commit e83d8fb3cd92d2c6dd0437e98bfa9b64d8d8284b
+Subproject commit fb0ba7b6307782e0d43a0ca598b237836cb6d304
"#
        ));
        assert!(has_submodule_gitlink_hunk(
            r#"diff --git a/vendor/dependency b/vendor/dependency
new file mode 160000
index 0000000..1234567
--- /dev/null
+++ b/vendor/dependency
"#
        ));
        assert!(!has_submodule_gitlink_hunk(
            r#"diff --git a/src/lib.rs b/src/lib.rs
index 1234567..89abcde 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1 +1,2 @@
 fn lib() {}
+fn helper() {}
"#
        ));
    }

    #[test]
    fn test_generate_evaluation_example_rejects_submodule_gitlink_hunk() {
        let commit = r#"diff --git a/controllers/llguidance b/controllers/llguidance
index 21e68b9..cadabda 160000
--- a/controllers/llguidance
+++ b/controllers/llguidance
@@ -1 +1 @@
-Subproject commit 21e68b916d4705107e1c45ea7bc927e829136258
+Subproject commit cadabdad21f3b81ff58b1918f8c23116b4ff7af3
"#;

        let result = generate_evaluation_example_from_ordered_commit(
            commit,
            "https://github.com/microsoft/aici",
            "cadabdad21f3b81ff58b1918f8c23116b4ff7af3",
            None,
            Some(0),
            None,
        );

        let Err(error) = result else {
            panic!("expected submodule/gitlink commit to be rejected");
        };
        assert!(error.to_string().contains("submodule/gitlink"));
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

    #[test]
    fn test_imitate_human_edits_inserts_after_last_source_edit() {
        // Regression test: intermediate content should appear after the last edit
        // in the source patch, not at the position of the first target edit.
        // This ensures the diff output correctly imitates human typing order.
        //
        // The bug was: when source has edits and target has a pure insertion,
        // the intermediate content was inserted at tgt_edit_loc.line_index_within_hunk
        // (position of first target edit) instead of after the last source edit.
        //
        // Source patch has edits at lines 1-4, target has a new edit at line 10
        // (different location to avoid the "same line" early return)
        let source = r#"--- a/test.py
+++ b/test.py
@@ -1,4 +1,5 @@
+import foo
 import bar
-import old
 import baz
+import qux
"#;
        // Target has a pure insertion at a different line (line 10, not overlapping with source)
        let target = r#"--- a/test.py
+++ b/test.py
@@ -10,3 +10,4 @@
 def main():
+    print("hello world")
     pass
"#;

        // Use a seed that produces a partial result
        let (new_src, _new_tgt, cursor) = imitate_human_edits(source, target, 42);

        // The function should produce a modified patch
        assert!(cursor.is_some(), "Should produce intermediate state");

        let src_patch = Patch::parse_unified_diff(&new_src);
        let all_additions: Vec<_> = src_patch
            .hunks
            .iter()
            .flat_map(|h| h.lines.iter())
            .filter_map(|l| match l {
                PatchLine::Addition(s) => Some(s.as_str()),
                _ => None,
            })
            .collect();

        // The intermediate content (partial 'print("hello world")') should be
        // the LAST addition, appearing after "+import qux" (the last source edit)
        let last_addition = all_additions.last().expect("Should have additions");
        assert!(
            last_addition.trim_start().starts_with("pr"),
            "Intermediate content should be the last addition (partial 'print'), but last was: {:?}",
            last_addition
        );

        // Verify the original source edits are still in order before the intermediate
        let foo_pos = all_additions.iter().position(|s| *s == "import foo");
        let qux_pos = all_additions.iter().position(|s| *s == "import qux");
        let intermediate_pos = all_additions
            .iter()
            .position(|s| s.trim_start().starts_with("pr"));

        assert!(foo_pos.is_some(), "Should have 'import foo'");
        assert!(qux_pos.is_some(), "Should have 'import qux'");
        assert!(
            intermediate_pos.is_some(),
            "Should have intermediate content"
        );

        assert!(
            foo_pos < qux_pos && qux_pos < intermediate_pos,
            "Order should be: foo < qux < intermediate. Got foo={:?}, qux={:?}, intermediate={:?}",
            foo_pos,
            qux_pos,
            intermediate_pos
        );
    }

    #[test]
    fn test_cursor_excerpt_with_multibyte_utf8() {
        // Test that cursor excerpt handles multi-byte UTF-8 characters correctly
        // The Chinese character '第' is 3 bytes (0..3)
        let cursor = CursorPosition {
            file: "test.md".to_string(),
            line: 1,
            column: 1, // Byte index 1 is inside '第' (bytes 0..3)
            line_length: 80,
        };

        let source_patch = r#"--- a/test.md
+++ b/test.md
@@ -1,1 +1,1 @@
+第 14 章 Flask 工作原理与机制解析**
"#;

        let target_patch = "";

        // This should not panic even though column=1 is not a char boundary
        let result = get_cursor_excerpt(&cursor, source_patch, target_patch);

        // The function should handle the invalid byte index gracefully
        if let Some(excerpt) = result {
            assert!(
                excerpt.contains("<|user_cursor|>"),
                "Cursor excerpt should contain marker"
            );
            // The marker should be placed at a valid character boundary
            // (either at the start or after '第')
        }
    }
}
