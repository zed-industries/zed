use crate::commit::get_messages;
use crate::permalink::{build_commit_permalink, parse_git_remote_url, BuildCommitPermalinkParams};
use crate::Oid;
use anyhow::{anyhow, Context, Result};
use collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Command, Stdio};
use std::{ops::Range, path::Path};
use text::Rope;
use time;
use time::macros::format_description;
use time::OffsetDateTime;
use time::UtcOffset;
use url::Url;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

pub use git2 as libgit;

#[derive(Debug, Clone, Default)]
pub struct Blame {
    pub entries: Vec<BlameEntry>,
    pub messages: HashMap<Oid, String>,
    pub permalinks: HashMap<Oid, Url>,
    pub remote_url: Option<String>,
}

impl Blame {
    pub fn for_path(
        git_binary: &Path,
        working_directory: &Path,
        path: &Path,
        content: &Rope,
        remote_url: Option<String>,
    ) -> Result<Self> {
        let output = run_git_blame(git_binary, working_directory, path, &content)?;
        let mut entries = parse_git_blame(&output)?;
        entries.sort_unstable_by(|a, b| a.range.start.cmp(&b.range.start));

        let mut permalinks = HashMap::default();
        let mut unique_shas = HashSet::default();
        let parsed_remote_url = remote_url.as_deref().and_then(parse_git_remote_url);

        for entry in entries.iter_mut() {
            unique_shas.insert(entry.sha);
            // DEPRECATED (18 Apr 24): Sending permalinks over the wire is deprecated. Clients
            // now do the parsing.
            if let Some(remote) = parsed_remote_url.as_ref() {
                permalinks.entry(entry.sha).or_insert_with(|| {
                    build_commit_permalink(BuildCommitPermalinkParams {
                        remote,
                        sha: entry.sha.to_string().as_str(),
                    })
                });
            }
        }

        let shas = unique_shas.into_iter().collect::<Vec<_>>();
        let messages =
            get_messages(&working_directory, &shas).context("failed to get commit messages")?;

        Ok(Self {
            entries,
            permalinks,
            messages,
            remote_url,
        })
    }
}

const GIT_BLAME_NO_COMMIT_ERROR: &'static str = "fatal: no such ref: HEAD";
const GIT_BLAME_NO_PATH: &'static str = "fatal: no such path";

fn run_git_blame(
    git_binary: &Path,
    working_directory: &Path,
    path: &Path,
    contents: &Rope,
) -> Result<String> {
    let mut child = Command::new(git_binary);

    child
        .current_dir(working_directory)
        .arg("blame")
        .arg("--incremental")
        .arg("--contents")
        .arg("-")
        .arg(path.as_os_str())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    #[cfg(windows)]
    child.creation_flags(windows::Win32::System::Threading::CREATE_NO_WINDOW.0);

    let child = child
        .spawn()
        .map_err(|e| anyhow!("Failed to start git blame process: {}", e))?;

    let mut stdin = child
        .stdin
        .as_ref()
        .context("failed to get pipe to stdin of git blame command")?;

    for chunk in contents.chunks() {
        stdin.write_all(chunk.as_bytes())?;
    }
    stdin.flush()?;

    let output = child
        .wait_with_output()
        .map_err(|e| anyhow!("Failed to read git blame output: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let trimmed = stderr.trim();
        if trimmed == GIT_BLAME_NO_COMMIT_ERROR || trimmed.contains(GIT_BLAME_NO_PATH) {
            return Ok(String::new());
        }
        return Err(anyhow!("git blame process failed: {}", stderr));
    }

    Ok(String::from_utf8(output.stdout)?)
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

    pub fn author_offset_date_time(&self) -> Result<time::OffsetDateTime> {
        if let (Some(author_time), Some(author_tz)) = (self.author_time, &self.author_tz) {
            let format = format_description!("[offset_hour][offset_minute]");
            let offset = UtcOffset::parse(author_tz, &format)?;
            let date_time_utc = OffsetDateTime::from_unix_timestamp(author_time)?;

            Ok(date_time_utc.to_offset(offset))
        } else {
            // Directly return current time in UTC if there's no committer time or timezone
            Ok(time::OffsetDateTime::now_utc())
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
fn parse_git_blame(output: &str) -> Result<Vec<BlameEntry>> {
    let mut entries: Vec<BlameEntry> = Vec::new();
    let mut index: HashMap<Oid, usize> = HashMap::default();

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
