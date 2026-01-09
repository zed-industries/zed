//! `ep split` implementation.
//!
//! This command splits a JSONL dataset into multiple files based on size specifications,
//! with stratification by repository URL (if the field is present).
//!
//! # Usage
//!
//! ```text
//! ep split [input.jsonl] <out1>=<size1> <out2>=<size2> ...
//! ```
//!
//! If `input.jsonl` is not provided or is `-`, reads from stdin.
//!
//! # Size specifications
//!
//! - `80%` - percentage of total (repositories if stratified, examples otherwise)
//! - `100` - absolute count of repositories (if stratified) or examples
//! - `rest` - all remaining items (only one split can use this)
//!
//! # Stratification
//!
//! When examples have a `repository_url` field, the split is stratified by repository.
//! This ensures each output file contains examples from non-overlapping repositories.
//! Size specifications apply to the number of repositories, not individual examples.
//!
//! Examples without `repository_url` are distributed proportionally across all outputs.

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
    about = "Split a JSONL dataset into multiple files (stratified by repository_url if present)",
    after_help = r#"SIZE SPECIFICATIONS:
  <percentage>%    Percentage of total (e.g., 80%)
  <count>          Absolute number (e.g., 100)
  rest             All remaining items (only one output can use this)

  When stratifying by repository_url, sizes apply to repositories, not examples.

EXAMPLES:
  # Split 80% train, 20% validation
  ep split input.jsonl train.jsonl=80% valid.jsonl=rest

  # Split into train/valid/test
  ep split input.jsonl train.jsonl=80% valid.jsonl=10% test.jsonl=rest

  # Use absolute counts (100 repos to train, rest to valid)
  ep split input.jsonl train.jsonl=100 valid.jsonl=rest

  # Read from stdin
  cat input.jsonl | ep split train.jsonl=80% valid.jsonl=rest

  # Reproducible split with seed
  ep split --seed 42 input.jsonl train.jsonl=80% valid.jsonl=rest

STRATIFICATION:
  When examples have a "repository_url" field, the split ensures each output
  file contains examples from non-overlapping repositories. This prevents
  data leakage between train/test splits.
"#
)]
pub struct SplitArgs {
    /// Random seed for reproducibility
    #[arg(long)]
    pub seed: Option<u64>,
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

fn get_repository_url(line: &str) -> Option<String> {
    let value: Value = serde_json::from_str(line).ok()?;
    value
        .get("repository_url")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn group_lines_by_repo(lines: Vec<String>) -> (HashMap<String, Vec<String>>, Vec<String>) {
    let mut by_repo: HashMap<String, Vec<String>> = HashMap::new();
    let mut without_repo: Vec<String> = Vec::new();

    for line in lines {
        if let Some(repo_url) = get_repository_url(&line) {
            by_repo.entry(repo_url).or_default().push(line);
        } else {
            without_repo.push(line);
        }
    }

    (by_repo, without_repo)
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

    let (by_repo, without_repo) = group_lines_by_repo(lines);
    let has_repos = !by_repo.is_empty();

    if has_repos {
        eprintln!(
            "Stratifying by repository_url ({} unique repositories, {} examples)",
            by_repo.len(),
            total_lines - without_repo.len()
        );
        if !without_repo.is_empty() {
            eprintln!(
                "  + {} examples without repository_url (distributed proportionally)",
                without_repo.len()
            );
        }
    }

    let mut rng = match args.seed {
        Some(seed) => rand::rngs::StdRng::seed_from_u64(seed),
        None => rand::rngs::StdRng::from_os_rng(),
    };

    let mut split_outputs: Vec<Vec<String>> = vec![Vec::new(); specs.len()];

    if has_repos {
        let mut repos: Vec<String> = by_repo.keys().cloned().collect();
        repos.shuffle(&mut rng);

        let repo_counts = compute_split_counts(&specs, repos.len())?;

        let mut repo_iter = repos.into_iter();
        for (split_idx, &count) in repo_counts.iter().enumerate() {
            for _ in 0..count {
                if let Some(repo) = repo_iter.next() {
                    if let Some(repo_lines) = by_repo.get(&repo) {
                        split_outputs[split_idx].extend(repo_lines.iter().cloned());
                    }
                }
            }
        }

        if !without_repo.is_empty() {
            let no_repo_counts = compute_split_counts(&specs, without_repo.len())?;
            let mut no_repo_shuffled = without_repo;
            no_repo_shuffled.shuffle(&mut rng);

            let mut line_iter = no_repo_shuffled.into_iter();
            for (split_idx, &count) in no_repo_counts.iter().enumerate() {
                for _ in 0..count {
                    if let Some(line) = line_iter.next() {
                        split_outputs[split_idx].push(line);
                    }
                }
            }
        }
    } else {
        let line_counts = compute_split_counts(&specs, total_lines)?;
        let mut shuffled_lines = without_repo;
        shuffled_lines.shuffle(&mut rng);

        let mut line_iter = shuffled_lines.into_iter();
        for (split_idx, &count) in line_counts.iter().enumerate() {
            for _ in 0..count {
                if let Some(line) = line_iter.next() {
                    split_outputs[split_idx].push(line);
                }
            }
        }
    }

    for (spec, output_lines) in specs.iter().zip(split_outputs.iter()) {
        write_lines_to_file(&spec.path, output_lines)?;
        eprintln!("{}: {} examples", spec.path.display(), output_lines.len());
    }

    Ok(())
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
    fn test_get_repository_url() {
        let line = r#"{"repository_url": "https://github.com/example/repo", "data": 123}"#;
        assert_eq!(
            get_repository_url(line),
            Some("https://github.com/example/repo".to_string())
        );

        let line_no_repo = r#"{"data": 123}"#;
        assert_eq!(get_repository_url(line_no_repo), None);
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

        let (by_repo, without_repo) = group_lines_by_repo(lines);

        assert_eq!(by_repo.len(), 2);
        assert_eq!(by_repo.get("repo1").unwrap().len(), 2);
        assert_eq!(by_repo.get("repo2").unwrap().len(), 1);
        assert_eq!(without_repo.len(), 1);
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

        let args = SplitArgs { seed: Some(42) };
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

        let train_repos: std::collections::HashSet<_> = train_lines
            .iter()
            .filter_map(|l| get_repository_url(l))
            .collect();
        let valid_repos: std::collections::HashSet<_> = valid_lines
            .iter()
            .filter_map(|l| get_repository_url(l))
            .collect();

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
}
