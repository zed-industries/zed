use anyhow::anyhow;
use anyhow::{Context, Result};
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::{fmt, ops::Range, path::Path, sync::Arc};
use sum_tree::SumTree;

pub use git2 as libgit;

pub fn run_git_blame(working_directory: &Path, path: &Path, contents: &str) -> Result<String> {
    let mut child = Command::new("git")
        .current_dir(working_directory)
        .arg("blame")
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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Oid(libgit::Oid);

impl Oid {
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl FromStr for Oid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        libgit::Oid::from_str(s)
            .map_err(|error| anyhow!("failed to parse git oid: {}", error))
            .map(|oid| Self(oid))
    }
}

impl fmt::Debug for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for Oid {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Oid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<Oid>().map_err(serde::de::Error::custom)
    }
}

impl Default for Oid {
    fn default() -> Self {
        Self(libgit::Oid::zero())
    }
}

impl From<Oid> for u32 {
    fn from(oid: Oid) -> Self {
        let bytes = oid.0.as_bytes();
        debug_assert!(bytes.len() > 4);

        let mut u32_bytes: [u8; 4] = [0; 4];

        for i in 0..4 {
            u32_bytes[i] = bytes[i];
        }

        u32::from_ne_bytes(u32_bytes)
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct BlameEntry {
    pub sha: Oid,

    pub range: Range<u32>,

    pub original_line_number: u32,

    pub author: Option<String>,
    pub author_mail: Option<String>,
    pub author_time: Option<i64>,
    pub author_tz: Option<String>,

    pub committer: Option<String>,
    pub committer_mail: Option<String>,
    pub committer_time: Option<i64>,
    pub committer_tz: Option<String>,

    pub summary: Option<String>,

    pub previous: Option<String>,
    pub filename: String,
}

impl BlameEntry {
    // Returns a BlameEntry by parsing the first line of a `git blame --incremental`
    // entry. The line MUST have this format:
    //
    //     <40-byte-hex-sha1> <sourceline> <resultline> <num-lines>
    fn new_from_blame_line(line: &str) -> Result<BlameEntry> {
        let mut parts = line.split_whitespace();

        let sha = parts
            .next()
            .and_then(|line| line.parse::<Oid>().ok())
            .ok_or_else(|| anyhow!("failed to parse sha"))?;

        let original_line_number = parts
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

        let start_line = final_line_number.saturating_sub(1);
        let end_line = start_line + line_count;
        let range = start_line..end_line;

        Ok(Self {
            sha,
            range,
            original_line_number,
            ..Default::default()
        })
    }

    pub fn committer_datetime(&self) -> Result<DateTime<chrono::Utc>> {
        if let (Some(committer_time), Some(committer_tz)) =
            (self.committer_time, &self.committer_tz)
        {
            let naive_datetime = NaiveDateTime::from_timestamp_opt(committer_time, 0)
                .ok_or_else(|| anyhow!("Failed to parse timestamp"))?;
            let timezone_offset_in_seconds = committer_tz
                .parse::<i32>()
                .map_err(|e| anyhow!("Failed to parse timezone offset: {}", e))?
                / 100
                * 36;
            let timezone = FixedOffset::east_opt(timezone_offset_in_seconds)
                .ok_or_else(|| anyhow!("Invalid timezone offset: {}", committer_tz))?;

            // Convert to DateTime<FixedOffset>, then to DateTime<Utc>
            let datetime_with_timezone =
                DateTime::<FixedOffset>::from_naive_utc_and_offset(naive_datetime, timezone);
            Ok(datetime_with_timezone.with_timezone(&chrono::Utc))
        } else {
            // Directly return current time in UTC if there's no committer time or timezone
            Ok(chrono::Utc::now())
        }
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
    let mut index: HashMap<Oid, usize> = HashMap::new();

    let mut current_entry: Option<BlameEntry> = None;

    for line in output.lines() {
        let mut done = false;

        match &mut current_entry {
            None => {
                let mut new_entry = BlameEntry::new_from_blame_line(line)?;

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
                let Some((key, value)) = line.split_once(' ') else {
                    continue;
                };
                let is_committed = !entry.sha.is_zero();
                match key {
                    "filename" => {
                        entry.filename = value.into();
                        done = true;
                    }
                    "previous" => entry.previous = Some(value.into()),

                    "summary" if is_committed => entry.summary = Some(value.into()),
                    "author" if is_committed => entry.author = Some(value.into()),
                    "author-mail" if is_committed => entry.author_mail = Some(value.into()),
                    "author-time" if is_committed => {
                        entry.author_time = Some(value.parse::<i64>()?)
                    }
                    "author-tz" if is_committed => entry.author_tz = Some(value.into()),

                    "committer" if is_committed => entry.committer = Some(value.into()),
                    "committer-mail" if is_committed => entry.committer_mail = Some(value.into()),
                    "committer-time" if is_committed => {
                        entry.committer_time = Some(value.parse::<i64>()?)
                    }
                    "committer-tz" if is_committed => entry.committer_tz = Some(value.into()),
                    _ => {}
                }
            }
        };

        if done {
            if let Some(entry) = current_entry.take() {
                index.insert(entry.sha, entries.len());

                // We only want annotations that have a commit.
                if !entry.sha.is_zero() {
                    entries.push(entry);
                }
            }
        }
    }

    Ok(entries)
}

impl sum_tree::Item for BlameEntry {
    type Summary = BlameEntrySummary;

    fn summary(&self) -> Self::Summary {
        BlameEntrySummary {
            buffer_range: self.range.clone(),
        }
    }
}

impl fmt::Display for BlameEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        if self.sha.is_zero() {
            write!(f, "Not committed")
        } else {
            let datetime = self.committer_datetime().map_err(|_| std::fmt::Error)?;

            let datetime = datetime.format("%Y-%m-%d %H:%M").to_string();

            let pretty_commit_id = format!("{}", self.sha);
            let short_commit_id = pretty_commit_id.chars().take(6).collect::<String>();

            write!(
                f,
                "{} - {} <{}> - ({})",
                short_commit_id,
                self.committer.as_deref().unwrap_or("<no name>"),
                self.committer_mail.as_deref().unwrap_or("no email"),
                datetime
            )
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct BlameEntrySummary {
    buffer_range: Range<u32>,
}

impl sum_tree::Summary for BlameEntrySummary {
    type Context = text::BufferSnapshot;

    fn add_summary(&mut self, other: &Self, _: &Self::Context) {
        self.buffer_range.start = self.buffer_range.start.min(other.buffer_range.start);
        self.buffer_range.end = self.buffer_range.end.max(other.buffer_range.end);
    }
}

#[derive(Clone)]
pub struct BufferBlame {
    tree: SumTree<BlameEntry>,
}

impl BufferBlame {
    pub fn new_with_cli(
        working_directory: &Path,
        path: &Arc<Path>,
        buffer: &text::BufferSnapshot,
    ) -> Result<BufferBlame> {
        let buffer_text = buffer.as_rope().to_string();

        let output = run_git_blame(working_directory, path, &buffer_text)
            .context("failed to run 'git blame'")?;

        let entries = parse_git_blame(&output)?;

        Ok(Self::new_with_entries(entries, buffer))
    }

    pub fn new_with_entries(mut entries: Vec<BlameEntry>, buffer: &text::BufferSnapshot) -> Self {
        entries.sort_by(|a, b| a.range.start.cmp(&b.range.start));

        let mut tree = SumTree::new();
        for entry in entries {
            tree.push(entry, buffer);
        }

        Self { tree }
    }

    // pub fn entries_for_rows(
    //     &self,
    //     rows: impl IntoIterator<Item = Option<u32>>,
    // ) -> impl Iterator<Item = Option<BlameEntry>> {
    //     todo!()
    // }

    pub fn entries_in_row_range<'a>(
        &'a self,
        range: Range<u32>,
        buffer: &'a text::BufferSnapshot,
    ) -> impl 'a + Iterator<Item = &BlameEntry> {
        // TODO: don't need filter, can use seek
        let mut cursor = self.tree.filter::<_, BlameEntrySummary>(move |summary| {
            let before_start = summary.buffer_range.end.cmp(&range.start).is_lt();
            let after_end = summary.buffer_range.start.cmp(&range.end).is_gt();
            !before_start && !after_end
        });

        std::iter::from_fn(move || {
            cursor.next(buffer);
            cursor.item()
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
    fn test_parse_git_blame_not_committed() {
        let output = read_test_data("blame_incremental_not_committed");
        let entries = parse_git_blame(&output).unwrap();
        assert_eq_golden(&entries, "blame_incremental_not_committed");
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
