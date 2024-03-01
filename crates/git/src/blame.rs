use anyhow::anyhow;
use anyhow::{Context, Result};
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::iter;
use std::process::{Command, Stdio};
use std::{fmt, ops::Range, path::Path, sync::Arc};
use sum_tree::SumTree;
use text::{Anchor, Point};

pub use git2 as libgit;

const UNCOMMITTED_SHA: [u8; 20] = [0; 20];

pub fn git_blame_incremental(
    working_directory: &Path,
    path: &Path,
    contents: &String,
) -> Result<String> {
    let mut child = Command::new("git")
        .current_dir(working_directory)
        .arg("blame")
        // TODO: turn off all the git configurations
        .arg("--incremental")
        .arg("--contents")
        .arg("-")
        .arg(path.as_os_str())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow!("Failed to start git blame process: {}", e))?;

    if let Some(ref mut stdin) = child.stdin {
        stdin
            .write_all(contents.as_bytes())
            .map_err(|e| anyhow!("Failed to write to git blame stdin: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| anyhow!("Failed to read git blame output: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("git blame process failed: {}", stderr));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BlameEntry {
    pub sha: [u8; 20],
    pub original_line_number: u32,
    pub final_line_number: u32,
    pub line_count: u32,

    pub author: String,
    pub author_mail: String,
    pub author_time: i64,
    pub author_tz: String,

    pub committer: String,
    pub committer_mail: String,
    pub committer_time: i64,
    pub committer_tz: String,

    pub summary: String,

    pub previous: Option<String>,
    pub filename: String,
}

impl BlameEntry {
    pub fn committer_datetime(&self) -> Result<DateTime<FixedOffset>> {
        let naive_datetime = NaiveDateTime::from_timestamp_opt(self.committer_time, 0)
            .expect("failed to parse timestamp");
        let timezone_offset_in_seconds = self
            .committer_tz
            .parse::<i32>()
            .map_err(|e| anyhow!("Failed to parse timezone offset: {}", e))?
            / 100
            * 36;
        let timezone = FixedOffset::east_opt(timezone_offset_in_seconds)
            .ok_or_else(|| anyhow!("Invalid timezone offset: {}", self.committer_tz))?;
        Ok(DateTime::<FixedOffset>::from_naive_utc_and_offset(
            naive_datetime,
            timezone,
        ))
    }
}

// parse_git_blame parses the output of `git blame --incremental`, which returns
// all the blame-entries for a given path incrementally, as it finds them.
//
// Each entry *always* starts with:
//
//     <40-byte-hex-sha1> <sourceline> <resultline> <num-lines>
//
// Each entry *always* ends with:
//
//     filename <whitespace-quoted-filename-goes-here>
//
// Line numbers are 1-indexed.
//
// A `git blame --incremental` entry looks like this:
//
//    6ad46b5257ba16d12c5ca9f0d4900320959df7f4 2 2 1
//    author Joe Schmoe
//    author-mail <joe.schmoe@example.com>
//    author-time 1709741400
//    author-tz +0100
//    committer Joe Schmoe
//    committer-mail <joe.schmoe@example.com>
//    committer-time 1709741400
//    committer-tz +0100
//    summary Joe's cool commit
//    previous 486c2409237a2c627230589e567024a96751d475 index.js
//    filename index.js
//
// If the entry has the same SHA as an entry that was already printed then no
// signature information is printed:
//
//    6ad46b5257ba16d12c5ca9f0d4900320959df7f4 3 4 1
//    previous 486c2409237a2c627230589e567024a96751d475 index.js
//    filename index.js
//
// More about `--incremental` output: https://mirrors.edge.kernel.org/pub/software/scm/git/docs/git-blame.html
pub fn parse_git_blame(output: &str) -> Result<Vec<BlameEntry>> {
    let mut entries: Vec<BlameEntry> = Vec::new();
    let mut index: HashMap<[u8; 20], usize> = HashMap::new();

    let mut current_entry: Option<BlameEntry> = None;

    for line in output.lines() {
        let mut parts = line.split_whitespace();
        let mut done = false;
        match &mut current_entry {
            None => {
                let sha = parts
                    .next()
                    .map(parse_commit_sha)
                    .ok_or_else(|| anyhow!("failed to parse sha"))??;

                let original_line_number =
                    parts
                        .next()
                        .and_then(|line| line.parse::<u32>().ok())
                        .ok_or_else(|| anyhow!("Failed to parse original line number"))?;
                let final_line_number = parts
                    .next()
                    .and_then(|line| line.parse::<u32>().ok())
                    .ok_or_else(|| anyhow!("Failed to parse final line number"))?;

                let line_count = parts
                    .next()
                    .and_then(|line| line.parse::<u32>().ok())
                    .ok_or_else(|| anyhow!("Failed to parse final line number"))?;

                let mut new_entry = BlameEntry {
                    sha,
                    original_line_number,
                    final_line_number,
                    line_count,
                    ..Default::default()
                };
                if let Some(existing_entry) = index
                    .get(&new_entry.sha)
                    .and_then(|slot| entries.get(*slot))
                {
                    new_entry.author = existing_entry.author.clone();
                    new_entry.author_mail = existing_entry.author_mail.clone();
                    new_entry.author_time = existing_entry.author_time;
                    new_entry.author_tz = existing_entry.author_tz.clone();
                    new_entry.committer = existing_entry.committer.clone();
                    new_entry.committer_mail = existing_entry.committer_mail.clone();
                    new_entry.committer_time = existing_entry.committer_time;
                    new_entry.committer_tz = existing_entry.committer_tz.clone();
                    new_entry.summary = existing_entry.summary.clone();
                }

                current_entry.replace(new_entry);
            }
            Some(entry) => {
                let Some(key) = parts.next() else {
                    continue;
                };
                let value = parts.collect::<String>();
                match key {
                    "filename" => {
                        entry.filename = value;
                        done = true;
                    }
                    "summary" => entry.summary = value,
                    "previous" => entry.previous = Some(value),

                    "author" => {
                        entry.author = if entry.sha == UNCOMMITTED_SHA {
                            "Not committed".to_string()
                        } else {
                            value
                        }
                    }
                    "author-mail" => entry.author_mail = value,
                    "author-time" => entry.author_time = value.parse::<i64>()?,
                    "author-tz" => entry.author_tz = value,

                    "committer" => {
                        entry.committer = if entry.sha == UNCOMMITTED_SHA {
                            "Not committed".to_string()
                        } else {
                            value
                        }
                    }
                    "committer-mail" => entry.committer_mail = value,
                    "committer-time" => entry.committer_time = value.parse::<i64>()?,
                    "committer-tz" => entry.committer_tz = value,
                    _ => {}
                }
            }
        };

        if done {
            if let Some(entry) = current_entry.take() {
                index.insert(entry.sha, entries.len());
                entries.push(entry);
            }
        }
    }

    Ok(entries)
}

fn parse_commit_sha(hex: &str) -> Result<[u8; 20]> {
    let mut bytes = [0u8; 20];
    for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
        bytes[i] = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16)?;
    }
    Ok(bytes)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlameHunk<T> {
    pub buffer_range: Range<T>,

    pub oid: libgit::Oid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub time: DateTime<FixedOffset>,
}

impl BlameHunk<u32> {}

impl sum_tree::Item for BlameHunk<Anchor> {
    type Summary = BlameHunkSummary;

    fn summary(&self) -> Self::Summary {
        BlameHunkSummary {
            buffer_range: self.buffer_range.clone(),
        }
    }
}

impl<T> fmt::Display for BlameHunk<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        let datetime = self.time.format("%Y-%m-%d %H:%M").to_string();

        let pretty_commit_id = format!("{}", self.oid);
        let short_commit_id = pretty_commit_id.chars().take(6).collect::<String>();

        write!(
            f,
            "{} - {} <{}> - ({})",
            short_commit_id,
            self.name.as_deref().unwrap_or("<no name>"),
            self.email.as_deref().unwrap_or("no email"),
            datetime
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct BlameHunkSummary {
    buffer_range: Range<Anchor>,
}

impl sum_tree::Summary for BlameHunkSummary {
    type Context = text::BufferSnapshot;

    fn add_summary(&mut self, other: &Self, buffer: &Self::Context) {
        self.buffer_range.start = self
            .buffer_range
            .start
            .min(&other.buffer_range.start, buffer);
        self.buffer_range.end = self.buffer_range.end.max(&other.buffer_range.end, buffer);
    }
}

#[derive(Clone)]
pub struct BufferBlame {
    tree: SumTree<BlameHunk<Anchor>>,
}

impl BufferBlame {
    pub fn new_with_cli(
        working_directory: &Path,
        path: &Arc<Path>,
        buffer: &text::BufferSnapshot,
    ) -> Result<BufferBlame> {
        let buffer_text = buffer.as_rope().to_string();
        let now = std::time::Instant::now();
        let output = git_blame_incremental(working_directory, path, &buffer_text)
            .context("failed to run 'git blame'")?;
        println!("running git blame took: {:?}", now.elapsed());

        let now = std::time::Instant::now();
        let mut entries = parse_git_blame(&output)?;
        entries.sort_by(|a, b| a.final_line_number.cmp(&b.final_line_number));

        println!(
            "parsing git blame output took: {:?}. entries: {}",
            now.elapsed(),
            entries.len()
        );

        let mut tree = SumTree::new();

        let now = std::time::Instant::now();
        for entry in entries {
            let start_line = entry.final_line_number - 1;
            let start = Point::new(start_line, 0);
            let end = Point::new(start_line + entry.line_count, 0);

            let buffer_range = buffer.anchor_before(start)..buffer.anchor_before(end);
            let oid = libgit::Oid::from_bytes(&entry.sha)?;

            let hunk = BlameHunk {
                buffer_range,
                oid,
                name: Some(entry.committer.clone()),
                email: Some(entry.committer_mail.clone()),
                time: entry.committer_datetime()?,
            };
            tree.push(hunk, buffer);
        }
        println!(
            "git blame incremental. pushing to tree took: {:?}",
            now.elapsed()
        );

        Ok(BufferBlame { tree })
    }

    pub fn hunks_in_row_range<'a>(
        &'a self,
        range: Range<u32>,
        buffer: &'a text::BufferSnapshot,
    ) -> impl 'a + Iterator<Item = BlameHunk<u32>> {
        let start = buffer.anchor_before(Point::new(range.start, 0));
        let end = buffer.anchor_after(Point::new(range.end, 0));

        self.hunks_intersecting_range(start..end, buffer)
    }

    pub fn hunks_intersecting_range<'a>(
        &'a self,
        range: Range<Anchor>,
        buffer: &'a text::BufferSnapshot,
    ) -> impl 'a + Iterator<Item = BlameHunk<u32>> {
        // TODO: This is just straight-up copy&pasted from git::diff::Diff.
        let mut cursor = self.tree.filter::<_, BlameHunkSummary>(move |summary| {
            let before_start = summary.buffer_range.end.cmp(&range.start, buffer).is_lt();
            let after_end = summary.buffer_range.start.cmp(&range.end, buffer).is_gt();
            !before_start && !after_end
        });

        let anchor_iter = std::iter::from_fn(move || {
            cursor.next(buffer);
            cursor.item()
        })
        .flat_map(move |hunk| {
            [
                (&hunk.buffer_range.start, hunk),
                (&hunk.buffer_range.end, hunk),
            ]
            .into_iter()
        });

        let mut summaries = buffer.summaries_for_anchors_with_payload::<Point, _, _>(anchor_iter);
        iter::from_fn(move || {
            let (start_point, hunk) = summaries.next()?;
            let (end_point, _) = summaries.next()?;

            let end_row = if end_point.column > 0 {
                end_point.row + 1
            } else {
                end_point.row
            };

            // TODO: Why do we have to clone here?
            Some(BlameHunk {
                buffer_range: start_point.row..end_row,
                oid: hunk.oid,
                name: hunk.name.clone(),
                email: hunk.email.clone(),
                time: hunk.time,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::parse_git_blame;
    use super::BlameEntry;

    fn read_test_data(filename: &str) -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_data");
        path.push(filename);

        std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Could not read test data at {:?}. Is it generated?", path))
    }

    fn assert_eq_golden(entries: &Vec<BlameEntry>, golden_filename: &str) {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_data");
        path.push("golden");
        path.push(format!("{}.json", golden_filename));

        let have_json =
            serde_json::to_string_pretty(&entries).expect("could not serialize entries to JSON");

        let update = std::env::var("UPDATE_GOLDEN")
            .map(|val| val.to_ascii_lowercase() == "true")
            .unwrap_or(false);

        if update {
            std::fs::create_dir_all(path.parent().unwrap())
                .expect("could not create golden test data directory");
            std::fs::write(&path, have_json).expect("could not write out golden data");
        } else {
            let want_json =
                std::fs::read_to_string(&path).unwrap_or_else(|_| {
                    panic!("could not read golden test data file at {:?}. Did you run the test with UPDATE_GOLDEN=true before?", path);
                });

            pretty_assertions::assert_eq!(have_json, want_json, "wrong blame entries");
        }
    }

    #[test]
    fn test_parse_git_blame_simple() {
        let output = read_test_data("blame_incremental_simple");
        let entries = parse_git_blame(&output).unwrap();
        assert_eq_golden(&entries, "blame_incremental_simple");
    }

    #[test]
    fn test_parse_git_blame_complex() {
        let output = read_test_data("blame_incremental_complex");
        let entries = parse_git_blame(&output).unwrap();
        assert_eq_golden(&entries, "blame_incremental_complex");
    }
}
