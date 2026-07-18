use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process;

use edit_prediction_metrics::{
    ClassificationMetrics, DeltaChrFMetrics, EditableContextCoverage, Excerpt, KeptRateResult,
    TokenAnnotation, annotate_kept_rate_tokens, braces_disbalance, compute_kept_rate,
    count_patch_token_changes, delta_chr_f, editable_context_coverage, exact_lines_match,
    extract_changed_lines_from_diff, has_isolated_whitespace_changes, is_editable_region_correct,
};
use serde::Deserialize;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        print_usage();
        return Err("missing arguments".to_string());
    }

    let input = CliInput::parse(&args)?;
    let report = match input {
        CliInput::Files {
            base_path,
            expected_patch_path,
            actual_patch_path,
        } => {
            let base = fs::read_to_string(&base_path)
                .map_err(|err| format!("failed to read {}: {err}", base_path.display()))?;
            let expected_patch = fs::read_to_string(&expected_patch_path).map_err(|err| {
                format!("failed to read {}: {err}", expected_patch_path.display())
            })?;
            let actual_patch = fs::read_to_string(&actual_patch_path)
                .map_err(|err| format!("failed to read {}: {err}", actual_patch_path.display()))?;

            let expected = apply_patch_to_excerpt(&base, &expected_patch, 0, None)?;
            let actual = apply_patch_to_excerpt(&base, &actual_patch, 0, None)?;
            let context = [];

            EvaluationReport::new(
                base,
                expected_patch,
                actual_patch,
                expected,
                actual,
                &context,
            )
        }
        CliInput::Json {
            json_path,
            prediction_index,
        } => {
            let json = fs::read_to_string(&json_path)
                .map_err(|err| format!("failed to read {}: {err}", json_path.display()))?;
            let example: JsonExample = serde_json::from_str(&json)
                .map_err(|err| format!("failed to parse {}: {err}", json_path.display()))?;

            report_from_json_example(example, prediction_index)?
        }
    };

    print_report(&report);
    Ok(())
}

fn get_context_excerpts(example: &JsonExample) -> Vec<Excerpt> {
    let mut context = vec![get_cursor_excerpt(example)];

    if let Some(related) = &example.prompt_inputs.related_files {
        context.extend(related.iter().flat_map(|file| {
            file.excerpts.iter().map(|excerpt| Excerpt {
                path: file.path.clone(),
                row_range: excerpt.row_range.clone(),
                content: excerpt.text.clone(),
            })
        }));
    }

    context
}

fn get_cursor_excerpt(example: &JsonExample) -> Excerpt {
    let content = example.prompt_inputs.cursor_excerpt.clone();
    let start_row = example.prompt_inputs.excerpt_start_row;
    let rows = content.lines().count() as u32;
    let row_range = start_row..start_row + rows;
    Excerpt {
        path: example.cursor_path.clone(),
        row_range,
        content,
    }
}

fn report_from_json_example(
    example: JsonExample,
    prediction_index: usize,
) -> Result<EvaluationReport, String> {
    let context = get_context_excerpts(&example);
    let excerpt_start_row = example.prompt_inputs.excerpt_start_row;
    let cursor_path = example.cursor_path;
    let base = example.prompt_inputs.cursor_excerpt;
    let expected_patch = example
        .expected_patches
        .into_iter()
        .next()
        .ok_or_else(|| "JSON input is missing expected_patches[0]".to_string())?;
    let actual_patch = if example.predictions.is_empty() {
        String::new()
    } else {
        example
            .predictions
            .into_iter()
            .nth(prediction_index)
            .ok_or_else(|| format!("JSON input does not contain predictions[{prediction_index}]"))?
            .actual_patch
    };

    let expected = apply_patch_to_excerpt(
        &base,
        &expected_patch,
        excerpt_start_row,
        Some(&cursor_path),
    )?;
    let actual =
        apply_patch_to_excerpt(&base, &actual_patch, excerpt_start_row, Some(&cursor_path))?;

    Ok(EvaluationReport::new(
        base,
        expected_patch,
        actual_patch,
        expected,
        actual,
        &context,
    ))
}

fn print_usage() {
    eprintln!(
        "Usage:\n  edit_prediction_metrics --base <base.txt> --expected-patch <expected.diff> --actual-patch <actual.diff>\n  edit_prediction_metrics --json <example.json> [--prediction-index <n>]"
    );
}

enum CliInput {
    Files {
        base_path: std::path::PathBuf,
        expected_patch_path: std::path::PathBuf,
        actual_patch_path: std::path::PathBuf,
    },
    Json {
        json_path: std::path::PathBuf,
        prediction_index: usize,
    },
}

impl CliInput {
    fn parse(args: &[String]) -> Result<Self, String> {
        let mut base_path = None;
        let mut expected_patch_path = None;
        let mut actual_patch_path = None;
        let mut json_path = None;
        let mut prediction_index = 0usize;

        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "--base" => {
                    index += 1;
                    base_path = Some(path_arg(args, index, "--base")?);
                }
                "--expected-patch" => {
                    index += 1;
                    expected_patch_path = Some(path_arg(args, index, "--expected-patch")?);
                }
                "--actual-patch" => {
                    index += 1;
                    actual_patch_path = Some(path_arg(args, index, "--actual-patch")?);
                }
                "--json" => {
                    index += 1;
                    json_path = Some(path_arg(args, index, "--json")?);
                }
                "--prediction-index" => {
                    index += 1;
                    let raw = string_arg(args, index, "--prediction-index")?;
                    prediction_index = raw.parse::<usize>().map_err(|err| {
                        format!("invalid value for --prediction-index ({raw}): {err}")
                    })?;
                }
                "--help" | "-h" => {
                    print_usage();
                    process::exit(0);
                }
                unknown => {
                    return Err(format!("unrecognized argument: {unknown}"));
                }
            }
            index += 1;
        }

        if let Some(json_path) = json_path {
            if base_path.is_some() || expected_patch_path.is_some() || actual_patch_path.is_some() {
                return Err(
                    "--json cannot be combined with --base/--expected-patch/--actual-patch"
                        .to_string(),
                );
            }
            return Ok(CliInput::Json {
                json_path,
                prediction_index,
            });
        }

        match (base_path, expected_patch_path, actual_patch_path) {
            (Some(base_path), Some(expected_patch_path), Some(actual_patch_path)) => {
                Ok(CliInput::Files {
                    base_path,
                    expected_patch_path,
                    actual_patch_path,
                })
            }
            _ => Err(
                "expected either --json <file> or all of --base, --expected-patch, and --actual-patch"
                    .to_string(),
            ),
        }
    }
}

fn path_arg(args: &[String], index: usize, flag: &str) -> Result<std::path::PathBuf, String> {
    Ok(Path::new(string_arg(args, index, flag)?).to_path_buf())
}

fn string_arg<'a>(args: &'a [String], index: usize, flag: &str) -> Result<&'a str, String> {
    args.get(index)
        .map(|value| value.as_str())
        .ok_or_else(|| format!("missing value for {flag}"))
}

#[derive(Debug)]
struct EvaluationReport {
    base: String,
    expected: String,
    actual: String,
    kept_rate: KeptRateResult,
    exact_lines: ClassificationMetrics,
    delta_chr_f: DeltaChrFMetrics,
    expected_changed_lines: usize,
    actual_changed_lines: usize,
    token_changes: edit_prediction_metrics::TokenChangeCounts,
    isolated_whitespace_changes: bool,
    editable_region_correct: bool,
    expected_braces_disbalance: usize,
    actual_braces_disbalance: usize,
    editable_context_coverage: EditableContextCoverage,
}

impl EvaluationReport {
    fn new(
        base: String,
        expected_patch: String,
        actual_patch: String,
        expected: String,
        actual: String,
        context: &[Excerpt],
    ) -> Self {
        let kept_rate = compute_kept_rate(&base, &actual, &expected);
        let exact_lines = exact_lines_match(&expected_patch, &actual_patch);
        let delta_chr_f = delta_chr_f(&base, &expected, &actual);
        let expected_changed_lines = extract_changed_lines_from_diff(&expected_patch)
            .values()
            .sum();
        let actual_changed_lines = extract_changed_lines_from_diff(&actual_patch)
            .values()
            .sum();
        let token_changes = count_patch_token_changes(&actual_patch);
        let isolated_whitespace_changes = has_isolated_whitespace_changes(&actual_patch, None);
        let editable_region_correct = is_editable_region_correct(&actual_patch);
        let expected_braces_disbalance = braces_disbalance(&expected);
        let actual_braces_disbalance = braces_disbalance(&actual);
        let editable_context_coverage = editable_context_coverage(&expected_patch, context);

        Self {
            base,
            expected,
            actual,
            kept_rate,
            exact_lines,
            delta_chr_f,
            expected_changed_lines,
            actual_changed_lines,
            token_changes,
            isolated_whitespace_changes,
            editable_region_correct,
            expected_braces_disbalance,
            actual_braces_disbalance,
            editable_context_coverage,
        }
    }
}

fn print_report(report: &EvaluationReport) {
    println!("Metrics");
    println!("=======");
    println!("kept_rate: {:.6}", report.kept_rate.kept_rate);
    println!("kept_rate_recall: {:.6}", report.kept_rate.recall_rate);
    println!("delta_chr_f: {:.6}", report.delta_chr_f.score);
    println!("delta_chr_f_precision: {:.6}", report.delta_chr_f.precision);
    println!("delta_chr_f_recall: {:.6}", report.delta_chr_f.recall);
    println!("delta_chr_f_beta: {:.6}", report.delta_chr_f.beta);
    println!();

    println!("Exact line match");
    println!("----------------");
    println!("true_positives: {}", report.exact_lines.true_positives);
    println!("false_positives: {}", report.exact_lines.false_positives);
    println!("false_negatives: {}", report.exact_lines.false_negatives);
    println!("precision: {:.6}", report.exact_lines.precision());
    println!("recall: {:.6}", report.exact_lines.recall());
    println!("f1: {:.6}", report.exact_lines.f1());
    println!("expected_changed_lines: {}", report.expected_changed_lines);
    println!("actual_changed_lines: {}", report.actual_changed_lines);
    println!();

    println!("Patch structure");
    println!("---------------");
    println!("inserted_tokens: {}", report.token_changes.inserted_tokens);
    println!("deleted_tokens: {}", report.token_changes.deleted_tokens);
    println!(
        "isolated_whitespace_changes: {}",
        report.isolated_whitespace_changes
    );
    println!(
        "editable_region_correct: {}",
        report.editable_region_correct
    );
    println!();

    println!("Final text checks");
    println!("-----------------");
    println!(
        "expected_braces_disbalance: {}",
        report.expected_braces_disbalance
    );
    println!(
        "actual_braces_disbalance: {}",
        report.actual_braces_disbalance
    );
    println!();

    println!("Kept-rate breakdown");
    println!("-------------------");
    println!(
        "candidate_new_chars: {}",
        report.kept_rate.candidate_new_chars
    );
    println!(
        "reference_new_chars: {}",
        report.kept_rate.reference_new_chars
    );
    println!(
        "candidate_deleted_chars: {}",
        report.kept_rate.candidate_deleted_chars
    );
    println!(
        "reference_deleted_chars: {}",
        report.kept_rate.reference_deleted_chars
    );
    println!("kept_chars: {}", report.kept_rate.kept_chars);
    println!(
        "correctly_deleted_chars: {}",
        report.kept_rate.correctly_deleted_chars
    );
    println!("discarded_chars: {}", report.kept_rate.discarded_chars);
    println!("context_chars: {}", report.kept_rate.context_chars);
    println!();

    print_kept_rate_explanation(&report.base, &report.actual, &report.expected);

    println!("Jumps metrics");
    println!("-------------");
    println!(
        "Editable context lines: P={}%, R={}%, F1={}% (tp: {}, fp: {}, fn: {})",
        (report.editable_context_coverage.lines_precision * 100.0).round(),
        (report.editable_context_coverage.lines_recall * 100.0).round(),
        (report.editable_context_coverage.lines_f1 * 100.0).round(),
        report.editable_context_coverage.lines_tp,
        report.editable_context_coverage.lines_fp,
        report.editable_context_coverage.lines_fn
    );
    println!(
        "Editable context files: P={}%, R={}%, F1={}% (tp: {}, fp: {}, fn: {})",
        (report.editable_context_coverage.files_precision * 100.0).round(),
        (report.editable_context_coverage.files_recall * 100.0).round(),
        (report.editable_context_coverage.files_f1 * 100.0).round(),
        report.editable_context_coverage.files_tp,
        report.editable_context_coverage.files_fp,
        report.editable_context_coverage.files_fn
    );
}

fn print_kept_rate_explanation(base: &str, actual: &str, expected: &str) {
    println!("Kept-rate explanation");
    println!("---------------------");
    println!("Legend: context = default, kept = green background, discarded = red background");
    println!();

    let annotated = annotate_kept_rate_tokens(base, actual, expected);
    println!("Actual final text with token annotations:");
    println!("{}", render_annotated_tokens(&annotated));
    println!();
}

fn render_annotated_tokens(tokens: &[edit_prediction_metrics::AnnotatedToken]) -> String {
    const RESET: &str = "\x1b[0m";
    const KEPT_STYLE: &str = "\x1b[30;42m";
    const DISCARDED_STYLE: &str = "\x1b[30;41m";

    let mut rendered = String::new();
    for token in tokens {
        let style = match token.annotation {
            TokenAnnotation::Context => "",
            TokenAnnotation::Kept => KEPT_STYLE,
            TokenAnnotation::Discarded => DISCARDED_STYLE,
        };

        if style.is_empty() {
            rendered.push_str(&visualize_whitespace(&token.token));
        } else {
            rendered.push_str(style);
            rendered.push_str(&visualize_whitespace(&token.token));
            rendered.push_str(RESET);
        }
    }
    rendered
}

fn visualize_whitespace(token: &str) -> String {
    let mut rendered = String::new();
    for ch in token.chars() {
        match ch {
            ' ' => rendered.push('·'),
            '\t' => rendered.push('⇥'),
            '\n' => rendered.push_str("↵\n"),
            _ => rendered.push(ch),
        }
    }
    rendered
}

#[derive(Debug, Deserialize)]
struct JsonExample {
    prompt_inputs: PromptInputs,
    cursor_path: String,
    expected_patches: Vec<String>,
    #[serde(default)]
    predictions: Vec<Prediction>,
}

#[derive(Debug, Deserialize)]
struct PromptInputs {
    cursor_excerpt: String,
    excerpt_start_row: u32,
    pub related_files: Option<Vec<RelatedFile>>,
}

#[derive(Clone, Debug, PartialEq, Hash, Deserialize)]
pub struct RelatedFile {
    pub path: String,
    pub max_row: u32,
    pub excerpts: Vec<RelatedExcerpt>,
}

#[derive(Clone, Debug, PartialEq, Hash, Deserialize)]
pub struct RelatedExcerpt {
    pub row_range: std::ops::Range<u32>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
struct Prediction {
    actual_patch: String,
}

#[derive(Debug, Clone)]
struct ParsedHunk {
    old_start: u32,
    filename: Option<String>,
    lines: Vec<HunkLine>,
}

#[derive(Debug, Clone)]
enum HunkLine {
    Context(String),
    Addition(String),
    Deletion(String),
}

fn apply_patch_to_excerpt(
    base: &str,
    patch: &str,
    excerpt_start_row: u32,
    target_path: Option<&str>,
) -> Result<String, String> {
    let hunks = parse_diff_hunks(patch);
    let hunks = if let Some(target_path) = target_path {
        hunks
            .into_iter()
            .filter(|hunk| match hunk.filename.as_deref() {
                Some(filename) => filename == target_path,
                None => true,
            })
            .collect::<Vec<_>>()
    } else {
        hunks
    };

    let result = try_apply_hunks(base, &hunks, excerpt_start_row);

    // Predicted patches may use excerpt-relative line numbers instead of
    // file-global ones. When all hunks fall outside the excerpt window the
    // result is identical to the base text. Retry with a zero offset so the
    // line numbers are interpreted relative to the excerpt.
    if excerpt_start_row > 0 && !hunks.is_empty() {
        let should_retry = match &result {
            Ok(text) => text == base,
            Err(_) => true,
        };

        if should_retry {
            let fallback = try_apply_hunks(base, &hunks, 0);
            if matches!(&fallback, Ok(text) if text != base) {
                return fallback;
            }
        }
    }

    result
}

fn try_apply_hunks(
    base: &str,
    hunks: &[ParsedHunk],
    excerpt_start_row: u32,
) -> Result<String, String> {
    let base_has_trailing_newline = base.ends_with('\n');
    let mut lines = split_preserving_final_empty_line(base);
    let original_line_count = lines.len() as u32;

    let excerpt_end_row = excerpt_start_row + original_line_count;
    let mut line_delta: i64 = 0;

    for hunk in hunks {
        let filtered = match filter_hunk_to_excerpt(hunk, excerpt_start_row, excerpt_end_row) {
            Some(filtered) => filtered,
            None => continue,
        };

        let local_start = filtered.old_start.saturating_sub(excerpt_start_row) as i64 + line_delta;
        if local_start < 0 {
            return Err(format!(
                "patch application moved before excerpt start at source row {}",
                filtered.old_start
            ));
        }
        let local_start = local_start as usize;

        if local_start > lines.len() {
            return Err(format!(
                "patch application starts past excerpt end at local line {}",
                local_start + 1
            ));
        }

        let old_len = filtered
            .lines
            .iter()
            .filter(|line| !matches!(line, HunkLine::Addition(_)))
            .count();
        let new_len = filtered
            .lines
            .iter()
            .filter(|line| !matches!(line, HunkLine::Deletion(_)))
            .count();

        let old_segment: Vec<&str> = filtered
            .lines
            .iter()
            .filter_map(|line| match line {
                HunkLine::Context(text) | HunkLine::Deletion(text) => Some(text.as_str()),
                HunkLine::Addition(_) => None,
            })
            .collect();

        let new_segment: Vec<String> = filtered
            .lines
            .iter()
            .filter_map(|line| match line {
                HunkLine::Context(text) | HunkLine::Addition(text) => Some(text.clone()),
                HunkLine::Deletion(_) => None,
            })
            .collect();

        if local_start + old_len > lines.len() {
            return Err(format!(
                "patch application exceeds excerpt bounds near source row {}",
                filtered.old_start
            ));
        }

        let current_segment: Vec<&str> = lines[local_start..local_start + old_len]
            .iter()
            .map(String::as_str)
            .collect();

        if current_segment != old_segment {
            let mut details = String::new();
            let _ = write!(
                details,
                "patch context mismatch near source row {}: expected {:?}, found {:?}",
                filtered.old_start, old_segment, current_segment
            );
            return Err(details);
        }

        lines.splice(local_start..local_start + old_len, new_segment);
        line_delta += new_len as i64 - old_len as i64;
    }

    Ok(join_lines(&lines, base_has_trailing_newline))
}

fn split_preserving_final_empty_line(text: &str) -> Vec<String> {
    let mut lines: Vec<String> = text.lines().map(ToString::to_string).collect();
    if text.ends_with('\n') {
        if lines.last().is_some_and(|line| !line.is_empty()) || lines.is_empty() {
            lines.push(String::new());
        }
    }
    lines
}

fn join_lines(lines: &[String], had_trailing_newline: bool) -> String {
    if lines.is_empty() {
        return String::new();
    }

    let mut joined = lines.join("\n");
    if had_trailing_newline && !joined.ends_with('\n') {
        joined.push('\n');
    }
    if !had_trailing_newline && joined.ends_with('\n') {
        joined.pop();
    }
    joined
}

fn filter_hunk_to_excerpt(
    hunk: &ParsedHunk,
    excerpt_start_row: u32,
    excerpt_end_row: u32,
) -> Option<ParsedHunk> {
    let mut filtered_lines = Vec::new();
    let mut current_old_row = hunk.old_start.saturating_sub(1);
    let mut filtered_old_start = None;
    let mut has_overlap = false;

    for line in &hunk.lines {
        match line {
            HunkLine::Context(text) => {
                let in_excerpt =
                    current_old_row >= excerpt_start_row && current_old_row < excerpt_end_row;
                if in_excerpt {
                    filtered_old_start.get_or_insert(current_old_row);
                    filtered_lines.push(HunkLine::Context(text.clone()));
                    has_overlap = true;
                }
                current_old_row += 1;
            }
            HunkLine::Deletion(text) => {
                let in_excerpt =
                    current_old_row >= excerpt_start_row && current_old_row < excerpt_end_row;
                if in_excerpt {
                    filtered_old_start.get_or_insert(current_old_row);
                    filtered_lines.push(HunkLine::Deletion(text.clone()));
                    has_overlap = true;
                }
                current_old_row += 1;
            }
            HunkLine::Addition(text) => {
                let insertion_in_excerpt =
                    current_old_row >= excerpt_start_row && current_old_row <= excerpt_end_row;
                if insertion_in_excerpt {
                    filtered_old_start.get_or_insert(current_old_row);
                    filtered_lines.push(HunkLine::Addition(text.clone()));
                    has_overlap = true;
                }
            }
        }
    }

    if !has_overlap {
        return None;
    }

    Some(ParsedHunk {
        old_start: filtered_old_start.unwrap_or(excerpt_start_row),
        filename: hunk.filename.clone(),
        lines: filtered_lines,
    })
}

fn parse_diff_hunks(diff: &str) -> Vec<ParsedHunk> {
    let mut hunks = Vec::new();
    let mut current_hunk: Option<ParsedHunk> = None;
    let mut current_filename = None;

    for line in diff.lines() {
        if let Some(filename) = parse_diff_filename(line) {
            current_filename = Some(filename);
            continue;
        }

        if let Some((old_start, old_count, _new_start, _new_count)) = parse_hunk_header(line) {
            if let Some(hunk) = current_hunk.take() {
                hunks.push(hunk);
            }
            let _ = old_count;
            current_hunk = Some(ParsedHunk {
                old_start,
                filename: current_filename.clone(),
                lines: Vec::new(),
            });
            continue;
        }

        let Some(hunk) = current_hunk.as_mut() else {
            continue;
        };

        if let Some(text) = line.strip_prefix('+') {
            if !line.starts_with("+++") {
                hunk.lines.push(HunkLine::Addition(text.to_string()));
            }
        } else if let Some(text) = line.strip_prefix('-') {
            if !line.starts_with("---") {
                hunk.lines.push(HunkLine::Deletion(text.to_string()));
            }
        } else if let Some(text) = line.strip_prefix(' ') {
            hunk.lines.push(HunkLine::Context(text.to_string()));
        } else if line.is_empty() {
            hunk.lines.push(HunkLine::Context(String::new()));
        }
    }

    if let Some(hunk) = current_hunk {
        hunks.push(hunk);
    }

    hunks
}

fn parse_hunk_header(line: &str) -> Option<(u32, u32, u32, u32)> {
    let line = line.strip_prefix("@@ -")?;
    let (old_part, rest) = line.split_once(' ')?;
    let rest = rest.strip_prefix('+')?;
    let (new_part, _) = rest.split_once(" @@")?;

    let (old_start, old_count) = parse_hunk_range(old_part)?;
    let (new_start, new_count) = parse_hunk_range(new_part)?;
    Some((old_start, old_count, new_start, new_count))
}

fn parse_hunk_range(part: &str) -> Option<(u32, u32)> {
    if let Some((start, count)) = part.split_once(',') {
        Some((start.parse().ok()?, count.parse().ok()?))
    } else {
        Some((part.parse().ok()?, 1))
    }
}

fn parse_diff_filename(line: &str) -> Option<String> {
    let path = line
        .strip_prefix("--- ")
        .or_else(|| line.strip_prefix("+++ "))?;
    normalize_diff_path(path)
}

fn normalize_diff_path(path: &str) -> Option<String> {
    let path = path.trim();
    let path = path
        .strip_prefix("a/")
        .or_else(|| path.strip_prefix("b/"))
        .unwrap_or(path);

    if path == "/dev/null" {
        None
    } else {
        Some(path.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_patch_in_file_mode() {
        let base = "fn main() {\n    println!(\"hello\");\n}\n";
        let patch = "@@ -1,3 +1,3 @@\n fn main() {\n-    println!(\"hello\");\n+    println!(\"world\");\n }\n";

        let actual = apply_patch_to_excerpt(base, patch, 0, None).unwrap();
        assert_eq!(actual, "fn main() {\n    println!(\"world\");\n}\n");
    }

    #[test]
    fn applies_patch_in_json_excerpt_mode() {
        let base = "b\nc\nd\n";
        let patch = "@@ -2,2 +2,2 @@\n-b\n-c\n+x\n+y\n";

        let actual = apply_patch_to_excerpt(base, patch, 1, None).unwrap();
        assert_eq!(actual, "x\ny\nd\n");
    }

    #[test]
    fn applies_patch_with_excerpt_relative_line_numbers() {
        let base = "a\nb\nc\nd\n";
        // Patch uses excerpt-relative line numbers (line 2 of excerpt)
        // even though the excerpt starts at file row 100.
        let patch = "@@ -2,2 +2,2 @@\n-b\n-c\n+x\n+y\n";

        let actual = apply_patch_to_excerpt(base, patch, 100, None).unwrap();
        assert_eq!(actual, "a\nx\ny\nd\n");
    }

    #[test]
    fn prefers_file_global_line_numbers_over_excerpt_relative() {
        let base = "a\nb\nc\n";
        // Patch uses file-global line numbers: excerpt starts at row 5,
        // hunk targets line 6 (1-based) = row 5 (0-based) = first line.
        let patch = "@@ -6,2 +6,2 @@\n-a\n-b\n+x\n+y\n";

        let actual = apply_patch_to_excerpt(base, patch, 5, None).unwrap();
        assert_eq!(actual, "x\ny\nc\n");
    }

    #[test]
    fn json_patch_application_ignores_unrelated_file_hunks() {
        let base = "first\nsecond\nthird\n";
        let patch = "--- a/src/other.rs\n+++ b/src/other.rs\n@@ -2,1 +2,1 @@\n-second\n+changed\n";

        let actual = apply_patch_to_excerpt(base, patch, 0, Some("src/main.rs")).unwrap();
        assert_eq!(actual, base);
    }

    #[test]
    fn json_patch_application_applies_matching_file_hunks() {
        let base = "first\nsecond\nthird\n";
        let patch = "--- a/src/main.rs\n+++ b/src/main.rs\n@@ -2,1 +2,1 @@\n-second\n+changed\n";

        let actual = apply_patch_to_excerpt(base, patch, 0, Some("src/main.rs")).unwrap();
        assert_eq!(actual, "first\nchanged\nthird\n");
    }

    #[test]
    fn json_patch_application_applies_headerless_hunks() {
        let base = "first\nsecond\nthird\n";
        let patch = "@@ -2,1 +2,1 @@\n-second\n+changed\n";

        let actual = apply_patch_to_excerpt(base, patch, 0, Some("src/main.rs")).unwrap();
        assert_eq!(actual, "first\nchanged\nthird\n");
    }

    fn json_example(predictions: Option<&str>) -> String {
        let predictions = predictions
            .map(|predictions| {
                format!(
                    r#",
    "predictions": {predictions}"#
                )
            })
            .unwrap_or_default();

        format!(
            r#"{{
    "prompt_inputs": {{
        "cursor_excerpt": "first\nsecond\nthird\n",
        "excerpt_start_row": 0
    }},
    "cursor_path": "src/main.rs",
    "expected_patches": [
        "--- a/src/main.rs\n+++ b/src/main.rs\n@@ -2,1 +2,1 @@\n-second\n+changed\n"
    ]{predictions}
}}"#
        )
    }

    fn report_from_json(predictions: Option<&str>) -> EvaluationReport {
        let example = serde_json::from_str(&json_example(predictions)).unwrap();
        report_from_json_example(example, 0).unwrap()
    }

    #[test]
    fn json_report_with_missing_predictions_uses_expected_patch_for_context_coverage() {
        let report = report_from_json(None);

        assert_eq!(report.actual, "first\nsecond\nthird\n");
        assert_eq!(report.actual_changed_lines, 0);
        assert_eq!(
            report.editable_context_coverage,
            EditableContextCoverage::new(3, 0, 0, 1, 0, 0)
        );
    }

    #[test]
    fn json_report_with_empty_predictions_uses_expected_patch_for_context_coverage() {
        let report = report_from_json(Some("[]"));

        assert_eq!(report.actual, "first\nsecond\nthird\n");
        assert_eq!(report.actual_changed_lines, 0);
        assert_eq!(
            report.editable_context_coverage,
            EditableContextCoverage::new(3, 0, 0, 1, 0, 0)
        );
    }
}
