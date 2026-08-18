use crate::Oid;
use crate::commit::{get_messages, get_tag_names};
use crate::repository::{GitBinary, RepoPath};
use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet};
use futures::{AsyncWriteExt, TryFutureExt, try_join};
use serde::{Deserialize, Serialize};
use smol::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use std::ops::Range;
use text::{LineEnding, Rope};
use time::OffsetDateTime;
use time::UtcOffset;
use time::macros::format_description;
use util::command::Stdio;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Blame {
    pub entries: Vec<BlameEntry>,
    pub messages: HashMap<Oid, String>,
    pub tag_names: HashMap<Oid, Vec<String>>,
}

impl Blame {
    pub(crate) async fn for_path(
        git: &GitBinary,
        path: &RepoPath,
        content: &Rope,
        line_ending: LineEnding,
    ) -> Result<Self> {
        let entries = run_git_blame(git, path, BlameSource::Contents(content, line_ending)).await?;
        Self::with_commit_details(git, entries).await
    }

    pub(crate) async fn for_path_at_revision(
        git: &GitBinary,
        path: &RepoPath,
        revision: Oid,
    ) -> Result<Self> {
        let entries = run_git_blame(git, path, BlameSource::Revision(revision)).await?;
        Self::with_commit_details(git, entries).await
    }

    async fn with_commit_details(git: &GitBinary, mut entries: Vec<BlameEntry>) -> Result<Self> {
        let mut unique_shas = HashSet::default();

        for entry in entries.iter_mut() {
            unique_shas.insert(entry.sha);
        }

        let shas = unique_shas.into_iter().collect::<Vec<_>>();
        let (messages, tag_names) = try_join!(
            get_messages(git, &shas)
                .map_err(|error| error.context("failed to get commit messages")),
            async {
                match get_tag_names(git, &shas).await {
                    Ok(tag_names) => Ok(tag_names),
                    Err(error) => {
                        log::warn!("failed to get commit tag names: {error:#}");
                        Ok(HashMap::default())
                    }
                }
            },
        )?;

        entries.sort_unstable_by_key(|entry| entry.range.start);
        Ok(Self {
            entries,
            messages,
            tag_names,
        })
    }
}

const GIT_BLAME_NO_COMMIT_ERROR: &str = "fatal: no such ref: HEAD";
const GIT_BLAME_NO_PATH: &str = "fatal: no such path";
const BLAME_PARSE_YIELD_INTERVAL: usize = 512;

#[derive(Clone, Copy)]
enum BlameSource<'a> {
    Contents(&'a Rope, LineEnding),
    Revision(Oid),
}

async fn run_git_blame(
    git: &GitBinary,
    path: &RepoPath,
    source: BlameSource<'_>,
) -> Result<Vec<BlameEntry>> {
    let mut child = {
        let span = ztracing::debug_span!("spawning git-blame command", path = path.as_unix_str());
        let _enter = span.enter();
        let mut args = vec!["blame", "--incremental"];
        let revision_string;
        match source {
            BlameSource::Contents(..) => args.extend(["--contents", "-"]),
            BlameSource::Revision(revision) => {
                revision_string = revision.to_string();
                args.push(&revision_string);
            }
        }
        args.push("--");
        git.build_command(&args)
            .arg(path.as_unix_str())
            .stdin(if matches!(source, BlameSource::Contents(..)) {
                Stdio::piped()
            } else {
                Stdio::null()
            })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .context("starting git blame process")?
    };

    let stdin = child.stdin.take();
    let stdout = child
        .stdout
        .take()
        .context("failed to get stdout from git blame command")?;
    let stderr = child
        .stderr
        .take()
        .context("failed to get stderr from git blame command")?;

    let write_stdin = async move {
        if let BlameSource::Contents(contents, line_ending) = source {
            let mut stdin = stdin.context("failed to get pipe to stdin of git blame command")?;
            for chunk in text::chunks_with_line_ending(contents, line_ending) {
                stdin.write_all(chunk.as_bytes()).await?;
            }
            stdin.flush().await?;
        }
        anyhow::Ok(())
    };

    let read_stdout = async move {
        let mut parser = GitBlameParser::new();
        let mut reader = BufReader::new(stdout);
        let mut line_buffer = String::new();
        let mut lines_read = 0;

        loop {
            line_buffer.clear();
            let bytes_read = reader
                .read_line(&mut line_buffer)
                .await
                .context("reading git blame stdout")?;
            if bytes_read == 0 {
                break;
            }

            let line = line_buffer.trim_end_matches(&['\r', '\n'][..]);
            parser.push_line(line)?;
            lines_read += 1;

            if lines_read % BLAME_PARSE_YIELD_INTERVAL == 0 {
                smol::future::yield_now().await;
            }
        }

        Ok(parser.entries)
    };

    let read_stderr = async move {
        let mut stderr_output = String::new();
        BufReader::new(stderr)
            .read_to_string(&mut stderr_output)
            .await
            .context("reading git blame stderr")?;
        Result::<String>::Ok(stderr_output)
    };

    let wait_for_status = async {
        child
            .status()
            .await
            .context("waiting for git blame process")
    };

    let ((), entries, stderr, status) =
        try_join!(write_stdin, read_stdout, read_stderr, wait_for_status)?;

    if !status.success() {
        let trimmed = stderr.trim();
        if trimmed == GIT_BLAME_NO_COMMIT_ERROR || trimmed.contains(GIT_BLAME_NO_PATH) {
            return Ok(Vec::new());
        }
        anyhow::bail!("git blame process failed: {stderr}");
    }

    Ok(entries)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BlameEntry {
    pub sha: Oid,

    pub range: Range<u32>,

    pub original_line_number: u32,

    pub author: Option<String>,
    pub author_mail: Option<String>,
    pub author_time: Option<i64>,
    pub author_tz: Option<String>,

    pub committer_name: Option<String>,
    pub committer_email: Option<String>,
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
            .context("parsing sha")?;

        let original_line_number = parts
            .next()
            .and_then(|line| line.parse::<u32>().ok())
            .context("parsing original line number")?;
        let final_line_number = parts
            .next()
            .and_then(|line| line.parse::<u32>().ok())
            .context("parsing final line number")?;

        let line_count = parts
            .next()
            .and_then(|line| line.parse::<u32>().ok())
            .context("parsing line count")?;

        let start_line = final_line_number.saturating_sub(1);
        let end_line = start_line + line_count;
        let range = start_line..end_line;

        Ok(Self {
            sha,
            range,
            original_line_number,
            author: None,
            author_mail: None,
            author_time: None,
            author_tz: None,
            committer_name: None,
            committer_email: None,
            committer_time: None,
            committer_tz: None,
            summary: None,
            previous: None,
            filename: String::new(),
        })
    }

    pub fn previous_sha_and_filename(&self) -> Option<(Oid, &str)> {
        let (sha, filename) = self.previous.as_deref()?.split_once(' ')?;
        Some((sha.parse().ok()?, filename))
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

// GitBlameParser parses the output of `git blame --incremental`, which returns
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
struct GitBlameParser {
    entries: Vec<BlameEntry>,
    index: HashMap<Oid, usize>,
    current_entry: Option<BlameEntry>,
}

impl GitBlameParser {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            index: HashMap::default(),
            current_entry: None,
        }
    }

    fn push_line(&mut self, line: &str) -> Result<()> {
        let mut done = false;

        match &mut self.current_entry {
            None => {
                let mut new_entry = BlameEntry::new_from_blame_line(line)?;

                if let Some(existing_entry) = self
                    .index
                    .get(&new_entry.sha)
                    .and_then(|slot| self.entries.get(*slot))
                {
                    new_entry.author.clone_from(&existing_entry.author);
                    new_entry
                        .author_mail
                        .clone_from(&existing_entry.author_mail);
                    new_entry.author_time = existing_entry.author_time;
                    new_entry.author_tz.clone_from(&existing_entry.author_tz);
                    new_entry
                        .committer_name
                        .clone_from(&existing_entry.committer_name);
                    new_entry
                        .committer_email
                        .clone_from(&existing_entry.committer_email);
                    new_entry.committer_time = existing_entry.committer_time;
                    new_entry
                        .committer_tz
                        .clone_from(&existing_entry.committer_tz);
                    new_entry.summary.clone_from(&existing_entry.summary);
                }

                self.current_entry.replace(new_entry);
            }
            Some(entry) => {
                let Some((key, value)) = line.split_once(' ') else {
                    return Ok(());
                };
                let is_committed = !entry.sha.is_zero();
                match key {
                    "filename" => {
                        entry.filename = unquote_git_path(value);
                        done = true;
                    }
                    "previous" => {
                        entry.previous = Some(match value.split_once(' ') {
                            Some((sha, filename)) => {
                                format!("{sha} {}", unquote_git_path(filename))
                            }
                            None => value.into(),
                        })
                    }

                    "summary" if is_committed => entry.summary = Some(value.into()),
                    "author" if is_committed => entry.author = Some(value.into()),
                    "author-mail" if is_committed => entry.author_mail = Some(value.into()),
                    "author-time" if is_committed => {
                        entry.author_time = Some(value.parse::<i64>()?)
                    }
                    "author-tz" if is_committed => entry.author_tz = Some(value.into()),

                    "committer" if is_committed => entry.committer_name = Some(value.into()),
                    "committer-mail" if is_committed => entry.committer_email = Some(value.into()),
                    "committer-time" if is_committed => {
                        entry.committer_time = Some(value.parse::<i64>()?)
                    }
                    "committer-tz" if is_committed => entry.committer_tz = Some(value.into()),
                    _ => {}
                }
            }
        };

        if done {
            self.push_current_entry();
        }

        Ok(())
    }

    fn push_current_entry(&mut self) {
        let Some(entry) = self.current_entry.take() else {
            return;
        };

        self.index.insert(entry.sha, self.entries.len());

        // We only want annotations that have a commit.
        if !entry.sha.is_zero() {
            self.entries.push(entry);
        }
    }
}

fn unquote_git_path(value: &str) -> String {
    let Some(quoted) = value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
    else {
        return value.to_owned();
    };
    let mut bytes = Vec::with_capacity(quoted.len());
    let mut input = quoted.bytes().peekable();
    while let Some(byte) = input.next() {
        if byte != b'\\' {
            bytes.push(byte);
            continue;
        }
        match input.next() {
            Some(b'a') => bytes.push(0x07),
            Some(b'b') => bytes.push(0x08),
            Some(b'f') => bytes.push(0x0c),
            Some(b'n') => bytes.push(b'\n'),
            Some(b'r') => bytes.push(b'\r'),
            Some(b't') => bytes.push(b'\t'),
            Some(b'v') => bytes.push(0x0b),
            Some(digit @ b'0'..=b'7') => {
                let mut octal = u32::from(digit - b'0');
                for _ in 0..2 {
                    match input.peek() {
                        Some(&next @ b'0'..=b'7') if octal * 8 + u32::from(next - b'0') <= 0xff => {
                            octal = octal * 8 + u32::from(next - b'0');
                            input.next();
                        }
                        _ => break,
                    }
                }
                bytes.push(octal as u8);
            }
            Some(other) => bytes.push(other),
            None => {}
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::blame::GitBlameParser;

    use super::{BlameEntry, unquote_git_path};

    fn parse_git_blame(output: &str) -> anyhow::Result<Vec<BlameEntry>> {
        let mut parser = GitBlameParser::new();

        for line in output.lines() {
            parser.push_line(line)?;
        }

        Ok(parser.entries)
    }

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

        let mut have_json =
            serde_json::to_string_pretty(&entries).expect("could not serialize entries to JSON");
        // We always want to save with a trailing newline.
        have_json.push('\n');

        let update = std::env::var("UPDATE_GOLDEN")
            .map(|val| val.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        if update {
            std::fs::create_dir_all(path.parent().unwrap())
                .expect("could not create golden test data directory");
            std::fs::write(&path, have_json).expect("could not write out golden data");
        } else {
            let want_json =
                std::fs::read_to_string(&path).unwrap_or_else(|_| {
                    panic!("could not read golden test data file at {:?}. Did you run the test with UPDATE_GOLDEN=true before?", path);
                }).replace("\r\n", "\n");

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

    #[test]
    fn test_parse_git_blame_quoted_filename() {
        let output = r#"6ad46b5257ba16d12c5ca9f0d4900320959df7f4 2 2 1
author Joe Schmoe
author-mail <joe.schmoe@example.com>
author-time 1709741400
author-tz +0100
committer Joe Schmoe
committer-mail <joe.schmoe@example.com>
committer-time 1709741400
committer-tz +0100
summary Joe's cool commit
previous 486c2409237a2c627230589e567024a96751d475 "\303\274rlich \"file\".txt"
filename "\303\274rlich \"file\".txt"
"#;
        let entries = parse_git_blame(output).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].filename, "ürlich \"file\".txt");
        assert_eq!(
            entries[0].previous.as_deref(),
            Some("486c2409237a2c627230589e567024a96751d475 ürlich \"file\".txt")
        );
        assert_eq!(
            entries[0].previous_sha_and_filename(),
            Some((
                "486c2409237a2c627230589e567024a96751d475".parse().unwrap(),
                "ürlich \"file\".txt"
            ))
        );
    }

    #[test]
    fn test_unquote_git_path() {
        assert_eq!(unquote_git_path("plain file.txt"), "plain file.txt");
        assert_eq!(unquote_git_path(r#""a\\b\"c\td""#), "a\\b\"c\td");
        assert_eq!(unquote_git_path(r#""\1\12\123""#), "\u{1}\nS");
        assert_eq!(unquote_git_path(r#""\777""#), "?7");
    }
}
