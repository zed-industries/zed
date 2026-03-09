//! `ep split` implementation.
//!
//! This command splits a JSONL dataset into multiple files based on size specifications,
//! with optional stratification by a JSON field.
//!
//! # Usage
//!
//! ```text
//! ep split [--stratify=<field>] [input.jsonl] <out1>=<size1> <out2>=<size2> ...
//! ```
//!
//! If `input.jsonl` is not provided or is `-`, reads from stdin.
//!
//! # Size specifications
//!
//! - `80%` - percentage of total examples (lines)
//! - `100` - approximate absolute count of examples (lines)
//! - `rest` - all remaining items (only one split can use this)
//!
//! # Stratification
//!
//! The `--stratify` flag controls how examples are grouped before splitting:
//!
//! - `cursor-path` (default): group by the `cursor_path` JSON field
//! - `repo`: group by the `repository_url` JSON field
//! - `none`: no grouping, split individual examples
//!
//! When stratifying, the split ensures each output file contains examples from
//! non-overlapping groups. Size specifications always apply to the number of
//! examples (lines), with whole groups assigned greedily to meet the target.
//! Examples missing the stratification field are treated as individual groups.

use anyhow::{Context as _, Result, bail};
use clap::Args;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

/// `ep split` CLI args.
#[derive(Debug, Args, Clone)]
#[command(
    about = "Split a JSONL dataset into multiple files with optional stratification",
    after_help = r#"SIZE SPECIFICATIONS:
  <percentage>%    Percentage of total (e.g., 80%)
  <count>          Absolute number (e.g., 100)
  rest             All remaining items (only one output can use this)

  Sizes always apply to examples (lines). When stratifying, whole groups
  are assigned greedily to approximate the target count.

EXAMPLES:
  # Split 80% train, 20% validation (default: stratify by cursor_path)
  ep split input.jsonl train.jsonl=80% valid.jsonl=rest

  # Split into train/valid/test
  ep split input.jsonl train.jsonl=80% valid.jsonl=10% test.jsonl=rest

  # Stratify by repository_url instead of cursor_path
  ep split --stratify=repo input.jsonl train.jsonl=80% valid.jsonl=rest

  # No stratification (split by individual examples)
  ep split --stratify=none input.jsonl train.jsonl=80% valid.jsonl=rest

  # Read from stdin
  cat input.jsonl | ep split train.jsonl=80% valid.jsonl=rest

  # Reproducible split with seed
  ep split --seed 42 input.jsonl train.jsonl=80% valid.jsonl=rest

STRATIFICATION:
  Controls how examples are grouped before splitting:
    cursor-path  Group by "cursor_path" field (default)
    repo         Group by "repository_url" field
    none         No grouping, split individual examples

  When stratifying, the split ensures each output file contains examples
  from non-overlapping groups. This prevents data leakage between
  train/test splits.
"#
)]
pub struct SplitArgs {
    /// Random seed for reproducibility
    #[arg(long)]
    pub seed: Option<u64>,

    /// Stratification field for splitting the dataset
    #[arg(long, default_value = "cursor-path")]
    pub stratify: Stratify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, strum::Display)]
pub enum Stratify {
    #[strum(serialize = "cursor_path")]
    CursorPath,
    #[strum(serialize = "repo")]
    Repo,
    #[strum(serialize = "none")]
    None,
}

#[derive(Debug, Clone)]
pub enum SplitSize {
    Percentage(f64),
    Absolute(usize),
    Rest,
}

#[derive(Debug, Clone)]
pub struct SplitSpec {
    pub path: PathBuf,
    pub size: SplitSize,
}

fn parse_split_spec(spec: &str) -> Result<SplitSpec> {
    let (path, size_str) = spec
        .rsplit_once('=')
        .with_context(|| format!("invalid split spec '{}': expected <path>=<size>", spec))?;

    let size = if size_str == "rest" {
        SplitSize::Rest
    } else if size_str.ends_with('%') {
        let pct_str = size_str.trim_end_matches('%');
        let pct: f64 = pct_str
            .parse()
            .with_context(|| format!("invalid percentage '{}' in '{}'", pct_str, spec))?;
        if !(0.0..=100.0).contains(&pct) {
            bail!("percentage must be between 0 and 100, got {}", pct);
        }
        SplitSize::Percentage(pct / 100.0)
    } else {
        let count: usize = size_str
            .parse()
            .with_context(|| format!("invalid count '{}' in '{}'", size_str, spec))?;
        SplitSize::Absolute(count)
    };

    Ok(SplitSpec {
        path: PathBuf::from(path),
        size,
    })
}

fn read_lines_from_input(input: Option<&Path>) -> Result<Vec<String>> {
    let reader: Box<dyn BufRead> = match input {
        Some(path) => {
            let file =
                File::open(path).with_context(|| format!("failed to open '{}'", path.display()))?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    let lines: Vec<String> = reader
        .lines()
        .collect::<io::Result<Vec<_>>>()
        .context("failed to read input lines")?;

    Ok(lines)
}

fn compute_split_counts(specs: &[SplitSpec], total: usize) -> Result<Vec<usize>> {
    let mut counts = vec![0usize; specs.len()];
    let mut remaining = total;
    let mut rest_index: Option<usize> = None;

    for (i, spec) in specs.iter().enumerate() {
        match &spec.size {
            SplitSize::Percentage(pct) => {
                let count = (total as f64 * pct).round() as usize;
                counts[i] = count.min(remaining);
                remaining = remaining.saturating_sub(counts[i]);
            }
            SplitSize::Absolute(count) => {
                counts[i] = (*count).min(remaining);
                remaining = remaining.saturating_sub(counts[i]);
            }
            SplitSize::Rest => {
                if rest_index.is_some() {
                    bail!("only one split can use 'rest'");
                }
                rest_index = Some(i);
            }
        }
    }

    if let Some(idx) = rest_index {
        counts[idx] = remaining;
    }

    Ok(counts)
}

fn write_lines_to_file(path: &Path, lines: &[String]) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create directory '{}'", parent.display()))?;
        }
    }

    let file =
        File::create(path).with_context(|| format!("failed to create '{}'", path.display()))?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)
            .with_context(|| format!("failed to write to '{}'", path.display()))?;
    }

    writer
        .flush()
        .with_context(|| format!("failed to flush '{}'", path.display()))?;

    Ok(())
}

pub fn run_split(args: &SplitArgs, inputs: &[PathBuf]) -> Result<()> {
    if inputs.is_empty() {
        bail!("usage: ep split [input.jsonl] train.jsonl=80% valid.jsonl=rest");
    }

    let (input_path, split_specs_raw): (Option<&Path>, &[PathBuf]) =
        if inputs.first().is_some_and(|p| {
            let s = p.to_string_lossy();
            !s.contains('=')
        }) {
            let first = inputs.first().map(|p| p.as_path());
            let first = if first == Some(Path::new("-")) {
                None
            } else {
                first
            };
            (first, &inputs[1..])
        } else {
            (None, inputs)
        };

    if split_specs_raw.is_empty() {
        bail!("no split specifications provided");
    }

    let specs: Vec<SplitSpec> = split_specs_raw
        .iter()
        .map(|p| parse_split_spec(&p.to_string_lossy()))
        .collect::<Result<Vec<_>>>()?;

    let lines = read_lines_from_input(input_path)?;
    let total_lines = lines.len();

    if total_lines == 0 {
        for spec in &specs {
            write_lines_to_file(&spec.path, &[])?;
        }
        return Ok(());
    }

    let mut grouped_lines = group_lines(&lines, args.stratify);

    if args.stratify != Stratify::None {
        eprintln!(
            "Stratifying by {} ({} unique groups, {} examples)",
            args.stratify,
            grouped_lines.len(),
            total_lines
        );
    } else {
        eprintln!(
            "No stratification, splitting {} examples by line",
            total_lines
        );
    }

    let mut rng = match args.seed {
        Some(seed) => rand::rngs::StdRng::seed_from_u64(seed),
        None => rand::rngs::StdRng::from_os_rng(),
    };

    grouped_lines.shuffle(&mut rng);

    let line_targets = compute_split_counts(&specs, total_lines)?;
    let rest_index = specs.iter().position(|s| matches!(s.size, SplitSize::Rest));
    let mut split_outputs: Vec<Vec<String>> = vec![Vec::new(); specs.len()];
    let mut group_iter = grouped_lines.into_iter();

    for (split_idx, &target) in line_targets.iter().enumerate() {
        if Some(split_idx) == rest_index {
            continue;
        }
        let mut accumulated = 0;
        while accumulated < target {
            if let Some(group) = group_iter.next() {
                accumulated += group.len();
                split_outputs[split_idx].extend(group);
            } else {
                break;
            }
        }
    }

    if let Some(idx) = rest_index {
        for group in group_iter {
            split_outputs[idx].extend(group);
        }
    }

    for (spec, output_lines) in specs.iter().zip(split_outputs.iter()) {
        write_lines_to_file(&spec.path, output_lines)?;
        eprintln!("{}: {} examples", spec.path.display(), output_lines.len());
    }

    Ok(())
}

/// Groups lines by the specified stratification field.
///
/// When `stratify` is `None`, each line becomes its own group.
/// When a line is missing the stratification field, it is also placed in its own group.
fn group_lines(lines: &[String], stratify: Stratify) -> Vec<Vec<String>> {
    if stratify == Stratify::None {
        return lines.iter().map(|line| vec![line.clone()]).collect();
    }

    let field = match stratify {
        Stratify::Repo => "repository_url",
        Stratify::CursorPath => "cursor_path",
        Stratify::None => unreachable!(),
    };

    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    let mut ungrouped: Vec<Vec<String>> = Vec::new();

    for line in lines {
        let key = serde_json::from_str::<Value>(line)
            .ok()
            .and_then(|v| v.get(field)?.as_str().map(|s| s.to_string()));
        match key {
            Some(key) => groups.entry(key).or_default().push(line.clone()),
            None => ungrouped.push(vec![line.clone()]),
        }
    }

    let mut result: Vec<Vec<String>> = groups.into_values().collect();
    result.extend(ungrouped);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_jsonl(lines: &[&str]) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        for line in lines {
            writeln!(file, "{}", line).unwrap();
        }
        file.flush().unwrap();
        file
    }

    #[test]
    fn test_parse_split_spec_percentage() {
        let spec = parse_split_spec("train.jsonl=80%").unwrap();
        assert_eq!(spec.path, PathBuf::from("train.jsonl"));
        match spec.size {
            SplitSize::Percentage(p) => assert!((p - 0.8).abs() < 0.001),
            _ => panic!("expected percentage"),
        }
    }

    #[test]
    fn test_parse_split_spec_absolute() {
        let spec = parse_split_spec("test.jsonl=100").unwrap();
        assert_eq!(spec.path, PathBuf::from("test.jsonl"));
        match spec.size {
            SplitSize::Absolute(n) => assert_eq!(n, 100),
            _ => panic!("expected absolute"),
        }
    }

    #[test]
    fn test_parse_split_spec_rest() {
        let spec = parse_split_spec("valid.jsonl=rest").unwrap();
        assert_eq!(spec.path, PathBuf::from("valid.jsonl"));
        assert!(matches!(spec.size, SplitSize::Rest));
    }

    #[test]
    fn test_group_lines_none() {
        let lines = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let groups = group_lines(&lines, Stratify::None);
        assert_eq!(groups.len(), 3);
        assert!(groups.iter().all(|g| g.len() == 1));
    }

    #[test]
    fn test_compute_split_counts_percentage() {
        let specs = vec![
            SplitSpec {
                path: PathBuf::from("a"),
                size: SplitSize::Percentage(0.8),
            },
            SplitSpec {
                path: PathBuf::from("b"),
                size: SplitSize::Percentage(0.2),
            },
        ];
        let counts = compute_split_counts(&specs, 100).unwrap();
        assert_eq!(counts, vec![80, 20]);
    }

    #[test]
    fn test_compute_split_counts_with_rest() {
        let specs = vec![
            SplitSpec {
                path: PathBuf::from("a"),
                size: SplitSize::Percentage(0.8),
            },
            SplitSpec {
                path: PathBuf::from("b"),
                size: SplitSize::Rest,
            },
        ];
        let counts = compute_split_counts(&specs, 100).unwrap();
        assert_eq!(counts, vec![80, 20]);
    }

    #[test]
    fn test_compute_split_counts_absolute() {
        let specs = vec![
            SplitSpec {
                path: PathBuf::from("a"),
                size: SplitSize::Absolute(50),
            },
            SplitSpec {
                path: PathBuf::from("b"),
                size: SplitSize::Rest,
            },
        ];
        let counts = compute_split_counts(&specs, 100).unwrap();
        assert_eq!(counts, vec![50, 50]);
    }

    #[test]
    fn test_group_lines_by_repo() {
        let lines = vec![
            r#"{"repository_url": "repo1", "id": 1}"#.to_string(),
            r#"{"repository_url": "repo1", "id": 2}"#.to_string(),
            r#"{"repository_url": "repo2", "id": 3}"#.to_string(),
            r#"{"id": 4}"#.to_string(),
        ];

        let groups = group_lines(&lines, Stratify::Repo);

        let grouped_count: usize = groups.iter().filter(|g| g.len() > 1).count();
        let ungrouped_count: usize = groups.iter().filter(|g| g.len() == 1).count();
        let total_lines: usize = groups.iter().map(|g| g.len()).sum();

        assert_eq!(grouped_count, 1); // repo1 has 2 lines
        assert_eq!(ungrouped_count, 2); // repo2 (1 line) + line without repo
        assert_eq!(total_lines, 4);
    }

    #[test]
    fn test_group_lines_by_cursor_path() {
        let lines = vec![
            r#"{"cursor_path": "src/main.rs", "id": 1}"#.to_string(),
            r#"{"cursor_path": "src/main.rs", "id": 2}"#.to_string(),
            r#"{"cursor_path": "src/lib.rs", "id": 3}"#.to_string(),
        ];

        let groups = group_lines(&lines, Stratify::CursorPath);

        let total_lines: usize = groups.iter().map(|g| g.len()).sum();
        assert_eq!(groups.len(), 2);
        assert_eq!(total_lines, 3);
    }

    #[test]
    fn test_run_split_basic() {
        let input = create_temp_jsonl(&[
            r#"{"repository_url": "repo1", "id": 1}"#,
            r#"{"repository_url": "repo1", "id": 2}"#,
            r#"{"repository_url": "repo2", "id": 3}"#,
            r#"{"repository_url": "repo2", "id": 4}"#,
            r#"{"repository_url": "repo3", "id": 5}"#,
            r#"{"repository_url": "repo3", "id": 6}"#,
            r#"{"repository_url": "repo4", "id": 7}"#,
            r#"{"repository_url": "repo4", "id": 8}"#,
        ]);

        let temp_dir = tempfile::tempdir().unwrap();
        let train_path = temp_dir.path().join("train.jsonl");
        let valid_path = temp_dir.path().join("valid.jsonl");

        let args = SplitArgs {
            seed: Some(42),
            stratify: Stratify::Repo,
        };
        let inputs = vec![
            input.path().to_path_buf(),
            PathBuf::from(format!("{}=50%", train_path.display())),
            PathBuf::from(format!("{}=rest", valid_path.display())),
        ];

        run_split(&args, &inputs).unwrap();

        let train_content = std::fs::read_to_string(&train_path).unwrap();
        let valid_content = std::fs::read_to_string(&valid_path).unwrap();

        let train_lines: Vec<&str> = train_content.lines().collect();
        let valid_lines: Vec<&str> = valid_content.lines().collect();

        assert_eq!(train_lines.len() + valid_lines.len(), 8);

        let get_repo = |line: &str| -> Option<String> {
            let value: Value = serde_json::from_str(line).ok()?;
            value
                .get("repository_url")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        };

        let train_repos: std::collections::HashSet<_> =
            train_lines.iter().filter_map(|l| get_repo(l)).collect();
        let valid_repos: std::collections::HashSet<_> =
            valid_lines.iter().filter_map(|l| get_repo(l)).collect();

        assert!(
            train_repos.is_disjoint(&valid_repos),
            "train and valid should have non-overlapping repos"
        );
    }

    #[test]
    fn test_multiple_rest_fails() {
        let specs = vec![
            SplitSpec {
                path: PathBuf::from("a"),
                size: SplitSize::Rest,
            },
            SplitSpec {
                path: PathBuf::from("b"),
                size: SplitSize::Rest,
            },
        ];
        assert!(compute_split_counts(&specs, 100).is_err());
    }

    #[test]
    fn test_absolute_targets_lines_not_groups() {
        // 5 repos × 3 lines each = 15 total lines.
        // `train=6` should target ~6 lines (2 groups), NOT 6 groups (all 15 lines).
        let input = create_temp_jsonl(&[
            r#"{"repository_url": "r1", "id": 1}"#,
            r#"{"repository_url": "r1", "id": 2}"#,
            r#"{"repository_url": "r1", "id": 3}"#,
            r#"{"repository_url": "r2", "id": 4}"#,
            r#"{"repository_url": "r2", "id": 5}"#,
            r#"{"repository_url": "r2", "id": 6}"#,
            r#"{"repository_url": "r3", "id": 7}"#,
            r#"{"repository_url": "r3", "id": 8}"#,
            r#"{"repository_url": "r3", "id": 9}"#,
            r#"{"repository_url": "r4", "id": 10}"#,
            r#"{"repository_url": "r4", "id": 11}"#,
            r#"{"repository_url": "r4", "id": 12}"#,
            r#"{"repository_url": "r5", "id": 13}"#,
            r#"{"repository_url": "r5", "id": 14}"#,
            r#"{"repository_url": "r5", "id": 15}"#,
        ]);

        let temp_dir = tempfile::tempdir().unwrap();
        let train_path = temp_dir.path().join("train.jsonl");
        let valid_path = temp_dir.path().join("valid.jsonl");

        let args = SplitArgs {
            seed: Some(42),
            stratify: Stratify::Repo,
        };
        let inputs = vec![
            input.path().to_path_buf(),
            PathBuf::from(format!("{}=6", train_path.display())),
            PathBuf::from(format!("{}=rest", valid_path.display())),
        ];

        run_split(&args, &inputs).unwrap();

        let train_content = std::fs::read_to_string(&train_path).unwrap();
        let valid_content = std::fs::read_to_string(&valid_path).unwrap();

        let train_lines: Vec<&str> = train_content.lines().collect();
        let valid_lines: Vec<&str> = valid_content.lines().collect();

        // With 3-line groups, train should get 2 groups (6 lines) to meet the
        // target of 6, NOT 6 groups (which don't even exist). Valid gets the rest.
        assert_eq!(train_lines.len(), 6);
        assert_eq!(valid_lines.len(), 9);
    }
}
