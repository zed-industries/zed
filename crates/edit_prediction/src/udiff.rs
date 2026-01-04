use std::{
    borrow::Cow,
    fmt::{Debug, Display, Write},
    mem,
    ops::Range,
    path::Path,
    sync::Arc,
};

use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use gpui::{AsyncApp, Entity};
use language::{Anchor, Buffer, OffsetRangeExt as _, TextBufferSnapshot, text_diff};
use postage::stream::Stream as _;
use project::Project;
use util::{paths::PathStyle, rel_path::RelPath};

#[derive(Clone, Debug)]
pub struct OpenedBuffers(#[allow(unused)] HashMap<String, Entity<Buffer>>);

#[must_use]
pub async fn apply_diff(
    diff_str: &str,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<OpenedBuffers> {
    let worktree = project
        .read_with(cx, |project, cx| project.visible_worktrees(cx).next())?
        .context("project has no worktree")?;

    // Ensure the files in the diff are loaded, since worktree scanning is disabled in
    // edit prediction CLI.
    let mut paths = Vec::new();
    for line in diff_str.lines() {
        if let DiffLine::OldPath { path } = DiffLine::parse(line) {
            paths.push(RelPath::new(Path::new(path.as_ref()), PathStyle::Posix)?.into_arc());
        }
    }
    worktree
        .update(cx, |worktree, _| {
            worktree
                .as_local()
                .unwrap()
                .refresh_entries_for_paths(paths)
        })?
        .recv()
        .await;

    let mut included_files = HashMap::default();

    let ranges = [Anchor::MIN..Anchor::MAX];
    let mut diff = DiffParser::new(diff_str);
    let mut current_file = None;
    let mut edits = vec![];

    while let Some(event) = diff.next()? {
        match event {
            DiffEvent::Hunk { path, hunk } => {
                let buffer = match current_file {
                    None => {
                        let buffer = project
                            .update(cx, |project, cx| {
                                let project_path = project
                                    .find_project_path(path.as_ref(), cx)
                                    .context("no such path")?;
                                anyhow::Ok(project.open_buffer(project_path, cx))
                            })??
                            .await?;
                        included_files.insert(path.to_string(), buffer.clone());
                        current_file = Some(buffer);
                        current_file.as_ref().unwrap()
                    }
                    Some(ref current) => current,
                };

                buffer.read_with(cx, |buffer, _| {
                    edits.extend(
                        resolve_hunk_edits_in_buffer(hunk, buffer, ranges.as_slice())
                            .with_context(|| format!("Diff:\n{diff_str}"))?,
                    );
                    anyhow::Ok(())
                })??;
            }
            DiffEvent::FileEnd { renamed_to } => {
                let buffer = current_file
                    .take()
                    .context("Got a FileEnd event before an Hunk event")?;

                if let Some(renamed_to) = renamed_to {
                    project
                        .update(cx, |project, cx| {
                            let new_project_path = project
                                .find_project_path(Path::new(renamed_to.as_ref()), cx)
                                .with_context(|| {
                                    format!("Failed to find worktree for new path: {}", renamed_to)
                                })?;

                            let project_file = project::File::from_dyn(buffer.read(cx).file())
                                .expect("Wrong file type");

                            anyhow::Ok(project.rename_entry(
                                project_file.entry_id.unwrap(),
                                new_project_path,
                                cx,
                            ))
                        })??
                        .await?;
                }

                let edits = mem::take(&mut edits);
                buffer.update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                })?;
            }
        }
    }

    Ok(OpenedBuffers(included_files))
}

/// Extract the diff for a specific file from a multi-file diff.
/// Returns an error if the file is not found in the diff.
pub fn extract_file_diff(full_diff: &str, file_path: &str) -> Result<String> {
    let mut result = String::new();
    let mut in_target_file = false;
    let mut found_file = false;

    for line in full_diff.lines() {
        if line.starts_with("diff --git") {
            if in_target_file {
                break;
            }
            in_target_file = line.contains(&format!("a/{}", file_path))
                || line.contains(&format!("b/{}", file_path));
            if in_target_file {
                found_file = true;
            }
        }

        if in_target_file {
            result.push_str(line);
            result.push('\n');
        }
    }

    if !found_file {
        anyhow::bail!("File '{}' not found in diff", file_path);
    }

    Ok(result)
}

/// Strip unnecessary git metadata lines from a diff, keeping only the lines
/// needed for patch application: path headers (--- and +++), hunk headers (@@),
/// and content lines (+, -, space).
pub fn strip_diff_metadata(diff: &str) -> String {
    let mut result = String::new();

    for line in diff.lines() {
        let dominated = DiffLine::parse(line);
        match dominated {
            // Keep path headers, hunk headers, and content lines
            DiffLine::OldPath { .. }
            | DiffLine::NewPath { .. }
            | DiffLine::HunkHeader(_)
            | DiffLine::Context(_)
            | DiffLine::Deletion(_)
            | DiffLine::Addition(_) => {
                result.push_str(line);
                result.push('\n');
            }
            // Skip garbage lines (diff --git, index, etc.)
            DiffLine::Garbage(_) => {}
        }
    }

    result
}

pub fn apply_diff_to_string(diff_str: &str, text: &str) -> Result<String> {
    let mut diff = DiffParser::new(diff_str);

    let mut text = text.to_string();

    while let Some(event) = diff.next()? {
        match event {
            DiffEvent::Hunk { hunk, .. } => {
                let hunk_offset = text
                    .find(&hunk.context)
                    .ok_or_else(|| anyhow!("couldn't resolve hunk {:?}", hunk.context))?;
                for edit in hunk.edits.iter().rev() {
                    let range = (hunk_offset + edit.range.start)..(hunk_offset + edit.range.end);
                    text.replace_range(range, &edit.text);
                }
            }
            DiffEvent::FileEnd { .. } => {}
        }
    }

    Ok(text)
}

/// Returns the individual edits that would be applied by a diff to the given content.
/// Each edit is a tuple of (byte_range_in_content, replacement_text).
/// Uses sub-line diffing to find the precise character positions of changes.
/// Returns an empty vec if the hunk context is not found or is ambiguous.
pub fn edits_for_diff(content: &str, diff_str: &str) -> Result<Vec<(Range<usize>, String)>> {
    let mut diff = DiffParser::new(diff_str);
    let mut result = Vec::new();

    while let Some(event) = diff.next()? {
        match event {
            DiffEvent::Hunk { hunk, .. } => {
                if hunk.context.is_empty() {
                    return Ok(Vec::new());
                }

                // Find the context in the content
                let first_match = content.find(&hunk.context);
                let Some(context_offset) = first_match else {
                    return Ok(Vec::new());
                };

                // Check for ambiguity - if context appears more than once, reject
                if content[context_offset + 1..].contains(&hunk.context) {
                    return Ok(Vec::new());
                }

                // Use sub-line diffing to find precise edit positions
                for edit in &hunk.edits {
                    let old_text = &content
                        [context_offset + edit.range.start..context_offset + edit.range.end];
                    let edits_within_hunk = text_diff(old_text, &edit.text);
                    for (inner_range, inner_text) in edits_within_hunk {
                        let absolute_start = context_offset + edit.range.start + inner_range.start;
                        let absolute_end = context_offset + edit.range.start + inner_range.end;
                        result.push((absolute_start..absolute_end, inner_text.to_string()));
                    }
                }
            }
            DiffEvent::FileEnd { .. } => {}
        }
    }

    Ok(result)
}

struct PatchFile<'a> {
    old_path: Cow<'a, str>,
    new_path: Cow<'a, str>,
}

struct DiffParser<'a> {
    current_file: Option<PatchFile<'a>>,
    current_line: Option<(&'a str, DiffLine<'a>)>,
    hunk: Hunk,
    diff: std::str::Lines<'a>,
}

#[derive(Debug, PartialEq)]
enum DiffEvent<'a> {
    Hunk { path: Cow<'a, str>, hunk: Hunk },
    FileEnd { renamed_to: Option<Cow<'a, str>> },
}

#[derive(Debug, Default, PartialEq)]
struct Hunk {
    context: String,
    edits: Vec<Edit>,
}

impl Hunk {
    fn is_empty(&self) -> bool {
        self.context.is_empty() && self.edits.is_empty()
    }
}

#[derive(Debug, PartialEq)]
struct Edit {
    range: Range<usize>,
    text: String,
}

impl<'a> DiffParser<'a> {
    fn new(diff: &'a str) -> Self {
        let mut diff = diff.lines();
        let current_line = diff.next().map(|line| (line, DiffLine::parse(line)));
        DiffParser {
            current_file: None,
            hunk: Hunk::default(),
            current_line,
            diff,
        }
    }

    fn next(&mut self) -> Result<Option<DiffEvent<'a>>> {
        loop {
            let (hunk_done, file_done) = match self.current_line.as_ref().map(|e| &e.1) {
                Some(DiffLine::OldPath { .. }) | Some(DiffLine::Garbage(_)) | None => (true, true),
                Some(DiffLine::HunkHeader(_)) => (true, false),
                _ => (false, false),
            };

            if hunk_done {
                if let Some(file) = &self.current_file
                    && !self.hunk.is_empty()
                {
                    return Ok(Some(DiffEvent::Hunk {
                        path: file.old_path.clone(),
                        hunk: mem::take(&mut self.hunk),
                    }));
                }
            }

            if file_done {
                if let Some(PatchFile { old_path, new_path }) = self.current_file.take() {
                    return Ok(Some(DiffEvent::FileEnd {
                        renamed_to: if old_path != new_path {
                            Some(new_path)
                        } else {
                            None
                        },
                    }));
                }
            }

            let Some((line, parsed_line)) = self.current_line.take() else {
                break;
            };

            util::maybe!({
                match parsed_line {
                    DiffLine::OldPath { path } => {
                        self.current_file = Some(PatchFile {
                            old_path: path,
                            new_path: "".into(),
                        });
                    }
                    DiffLine::NewPath { path } => {
                        if let Some(current_file) = &mut self.current_file {
                            current_file.new_path = path
                        }
                    }
                    DiffLine::HunkHeader(_) => {}
                    DiffLine::Context(ctx) => {
                        if self.current_file.is_some() {
                            writeln!(&mut self.hunk.context, "{ctx}")?;
                        }
                    }
                    DiffLine::Deletion(del) => {
                        if self.current_file.is_some() {
                            let range = self.hunk.context.len()
                                ..self.hunk.context.len() + del.len() + '\n'.len_utf8();
                            if let Some(last_edit) = self.hunk.edits.last_mut()
                                && last_edit.range.end == range.start
                            {
                                last_edit.range.end = range.end;
                            } else {
                                self.hunk.edits.push(Edit {
                                    range,
                                    text: String::new(),
                                });
                            }
                            writeln!(&mut self.hunk.context, "{del}")?;
                        }
                    }
                    DiffLine::Addition(add) => {
                        if self.current_file.is_some() {
                            let range = self.hunk.context.len()..self.hunk.context.len();
                            if let Some(last_edit) = self.hunk.edits.last_mut()
                                && last_edit.range.end == range.start
                            {
                                writeln!(&mut last_edit.text, "{add}").unwrap();
                            } else {
                                self.hunk.edits.push(Edit {
                                    range,
                                    text: format!("{add}\n"),
                                });
                            }
                        }
                    }
                    DiffLine::Garbage(_) => {}
                }

                anyhow::Ok(())
            })
            .with_context(|| format!("on line:\n\n```\n{}```", line))?;

            self.current_line = self.diff.next().map(|line| (line, DiffLine::parse(line)));
        }

        anyhow::Ok(None)
    }
}

fn resolve_hunk_edits_in_buffer(
    hunk: Hunk,
    buffer: &TextBufferSnapshot,
    ranges: &[Range<Anchor>],
) -> Result<impl Iterator<Item = (Range<Anchor>, Arc<str>)>, anyhow::Error> {
    let context_offset = if hunk.context.is_empty() {
        Ok(0)
    } else {
        let mut offset = None;
        for range in ranges {
            let range = range.to_offset(buffer);
            let text = buffer.text_for_range(range.clone()).collect::<String>();
            for (ix, _) in text.match_indices(&hunk.context) {
                if offset.is_some() {
                    anyhow::bail!("Context is not unique enough:\n{}", hunk.context);
                }
                offset = Some(range.start + ix);
            }
        }
        offset.ok_or_else(|| anyhow!("Failed to match context:\n{}", hunk.context))
    }?;
    let iter = hunk.edits.into_iter().flat_map(move |edit| {
        let old_text = buffer
            .text_for_range(context_offset + edit.range.start..context_offset + edit.range.end)
            .collect::<String>();
        let edits_within_hunk = language::text_diff(&old_text, &edit.text);
        edits_within_hunk
            .into_iter()
            .map(move |(inner_range, inner_text)| {
                (
                    buffer.anchor_after(context_offset + edit.range.start + inner_range.start)
                        ..buffer.anchor_before(context_offset + edit.range.start + inner_range.end),
                    inner_text,
                )
            })
    });
    Ok(iter)
}

#[derive(Debug, PartialEq)]
pub enum DiffLine<'a> {
    OldPath { path: Cow<'a, str> },
    NewPath { path: Cow<'a, str> },
    HunkHeader(Option<HunkLocation>),
    Context(&'a str),
    Deletion(&'a str),
    Addition(&'a str),
    Garbage(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct HunkLocation {
    start_line_old: u32,
    count_old: u32,
    start_line_new: u32,
    count_new: u32,
}

impl<'a> DiffLine<'a> {
    pub fn parse(line: &'a str) -> Self {
        Self::try_parse(line).unwrap_or(Self::Garbage(line))
    }

    fn try_parse(line: &'a str) -> Option<Self> {
        if let Some(header) = line.strip_prefix("---").and_then(eat_required_whitespace) {
            let path = parse_header_path("a/", header);
            Some(Self::OldPath { path })
        } else if let Some(header) = line.strip_prefix("+++").and_then(eat_required_whitespace) {
            Some(Self::NewPath {
                path: parse_header_path("b/", header),
            })
        } else if let Some(header) = line.strip_prefix("@@").and_then(eat_required_whitespace) {
            if header.starts_with("...") {
                return Some(Self::HunkHeader(None));
            }

            let mut tokens = header.split_whitespace();
            let old_range = tokens.next()?.strip_prefix('-')?;
            let new_range = tokens.next()?.strip_prefix('+')?;

            let (start_line_old, count_old) = old_range.split_once(',').unwrap_or((old_range, "1"));
            let (start_line_new, count_new) = new_range.split_once(',').unwrap_or((new_range, "1"));

            Some(Self::HunkHeader(Some(HunkLocation {
                start_line_old: start_line_old.parse::<u32>().ok()?.saturating_sub(1),
                count_old: count_old.parse().ok()?,
                start_line_new: start_line_new.parse::<u32>().ok()?.saturating_sub(1),
                count_new: count_new.parse().ok()?,
            })))
        } else if let Some(deleted_header) = line.strip_prefix("-") {
            Some(Self::Deletion(deleted_header))
        } else if line.is_empty() {
            Some(Self::Context(""))
        } else if let Some(context) = line.strip_prefix(" ") {
            Some(Self::Context(context))
        } else {
            Some(Self::Addition(line.strip_prefix("+")?))
        }
    }
}

impl<'a> Display for DiffLine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiffLine::OldPath { path } => write!(f, "--- {path}"),
            DiffLine::NewPath { path } => write!(f, "+++ {path}"),
            DiffLine::HunkHeader(Some(hunk_location)) => {
                write!(
                    f,
                    "@@ -{},{} +{},{} @@",
                    hunk_location.start_line_old + 1,
                    hunk_location.count_old,
                    hunk_location.start_line_new + 1,
                    hunk_location.count_new
                )
            }
            DiffLine::HunkHeader(None) => write!(f, "@@ ... @@"),
            DiffLine::Context(content) => write!(f, " {content}"),
            DiffLine::Deletion(content) => write!(f, "-{content}"),
            DiffLine::Addition(content) => write!(f, "+{content}"),
            DiffLine::Garbage(line) => write!(f, "{line}"),
        }
    }
}

fn parse_header_path<'a>(strip_prefix: &'static str, header: &'a str) -> Cow<'a, str> {
    if !header.contains(['"', '\\']) {
        let path = header.split_ascii_whitespace().next().unwrap_or(header);
        return Cow::Borrowed(path.strip_prefix(strip_prefix).unwrap_or(path));
    }

    let mut path = String::with_capacity(header.len());
    let mut in_quote = false;
    let mut chars = header.chars().peekable();
    let mut strip_prefix = Some(strip_prefix);

    while let Some(char) = chars.next() {
        if char == '"' {
            in_quote = !in_quote;
        } else if char == '\\' {
            let Some(&next_char) = chars.peek() else {
                break;
            };
            chars.next();
            path.push(next_char);
        } else if char.is_ascii_whitespace() && !in_quote {
            break;
        } else {
            path.push(char);
        }

        if let Some(prefix) = strip_prefix
            && path == prefix
        {
            strip_prefix.take();
            path.clear();
        }
    }

    Cow::Owned(path)
}

fn eat_required_whitespace(header: &str) -> Option<&str> {
    let trimmed = header.trim_ascii_start();

    if trimmed.len() == header.len() {
        None
    } else {
        Some(trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[test]
    fn parse_lines_simple() {
        let input = indoc! {"
            diff --git a/text.txt b/text.txt
            index 86c770d..a1fd855 100644
            --- a/file.txt
            +++ b/file.txt
            @@ -1,2 +1,3 @@
             context
            -deleted
            +inserted
            garbage

            --- b/file.txt
            +++ a/file.txt
        "};

        let lines = input.lines().map(DiffLine::parse).collect::<Vec<_>>();

        pretty_assertions::assert_eq!(
            lines,
            &[
                DiffLine::Garbage("diff --git a/text.txt b/text.txt"),
                DiffLine::Garbage("index 86c770d..a1fd855 100644"),
                DiffLine::OldPath {
                    path: "file.txt".into()
                },
                DiffLine::NewPath {
                    path: "file.txt".into()
                },
                DiffLine::HunkHeader(Some(HunkLocation {
                    start_line_old: 0,
                    count_old: 2,
                    start_line_new: 0,
                    count_new: 3
                })),
                DiffLine::Context("context"),
                DiffLine::Deletion("deleted"),
                DiffLine::Addition("inserted"),
                DiffLine::Garbage("garbage"),
                DiffLine::Context(""),
                DiffLine::OldPath {
                    path: "b/file.txt".into()
                },
                DiffLine::NewPath {
                    path: "a/file.txt".into()
                },
            ]
        );
    }

    #[test]
    fn file_header_extra_space() {
        let options = ["--- file", "---   file", "---\tfile"];

        for option in options {
            pretty_assertions::assert_eq!(
                DiffLine::parse(option),
                DiffLine::OldPath {
                    path: "file".into()
                },
                "{option}",
            );
        }
    }

    #[test]
    fn hunk_header_extra_space() {
        let options = [
            "@@ -1,2 +1,3 @@",
            "@@  -1,2  +1,3 @@",
            "@@\t-1,2\t+1,3\t@@",
            "@@ -1,2  +1,3 @@",
            "@@ -1,2   +1,3 @@",
            "@@ -1,2 +1,3   @@",
            "@@ -1,2 +1,3 @@ garbage",
        ];

        for option in options {
            pretty_assertions::assert_eq!(
                DiffLine::parse(option),
                DiffLine::HunkHeader(Some(HunkLocation {
                    start_line_old: 0,
                    count_old: 2,
                    start_line_new: 0,
                    count_new: 3
                })),
                "{option}",
            );
        }
    }

    #[test]
    fn hunk_header_without_location() {
        pretty_assertions::assert_eq!(DiffLine::parse("@@ ... @@"), DiffLine::HunkHeader(None));
    }

    #[test]
    fn test_parse_path() {
        assert_eq!(parse_header_path("a/", "foo.txt"), "foo.txt");
        assert_eq!(
            parse_header_path("a/", "foo/bar/baz.txt"),
            "foo/bar/baz.txt"
        );
        assert_eq!(parse_header_path("a/", "a/foo.txt"), "foo.txt");
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/baz.txt"),
            "foo/bar/baz.txt"
        );

        // Extra
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/baz.txt  2025"),
            "foo/bar/baz.txt"
        );
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/baz.txt\t2025"),
            "foo/bar/baz.txt"
        );
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/baz.txt \""),
            "foo/bar/baz.txt"
        );

        // Quoted
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/\"baz quox.txt\""),
            "foo/bar/baz quox.txt"
        );
        assert_eq!(
            parse_header_path("a/", "\"a/foo/bar/baz quox.txt\""),
            "foo/bar/baz quox.txt"
        );
        assert_eq!(
            parse_header_path("a/", "\"foo/bar/baz quox.txt\""),
            "foo/bar/baz quox.txt"
        );
        assert_eq!(parse_header_path("a/", "\"whatever 🤷\""), "whatever 🤷");
        assert_eq!(
            parse_header_path("a/", "\"foo/bar/baz quox.txt\"  2025"),
            "foo/bar/baz quox.txt"
        );
        // unescaped quotes are dropped
        assert_eq!(parse_header_path("a/", "foo/\"bar\""), "foo/bar");

        // Escaped
        assert_eq!(
            parse_header_path("a/", "\"foo/\\\"bar\\\"/baz.txt\""),
            "foo/\"bar\"/baz.txt"
        );
        assert_eq!(
            parse_header_path("a/", "\"C:\\\\Projects\\\\My App\\\\old file.txt\""),
            "C:\\Projects\\My App\\old file.txt"
        );
    }

    #[test]
    fn test_parse_diff_with_leading_and_trailing_garbage() {
        let diff = indoc! {"
            I need to make some changes.

            I'll change the following things:
            - one
              - two
            - three

            ```
            --- a/file.txt
            +++ b/file.txt
             one
            +AND
             two
            ```

            Summary of what I did:
            - one
              - two
            - three

            That's about it.
        "};

        let mut events = Vec::new();
        let mut parser = DiffParser::new(diff);
        while let Some(event) = parser.next().unwrap() {
            events.push(event);
        }

        assert_eq!(
            events,
            &[
                DiffEvent::Hunk {
                    path: "file.txt".into(),
                    hunk: Hunk {
                        context: "one\ntwo\n".into(),
                        edits: vec![Edit {
                            range: 4..4,
                            text: "AND\n".into()
                        }],
                    }
                },
                DiffEvent::FileEnd { renamed_to: None }
            ],
        )
    }

    #[gpui::test]
    async fn test_apply_diff_successful(cx: &mut TestAppContext) {
        let fs = init_test(cx);

        let buffer_1_text = indoc! {r#"
            one
            two
            three
            four
            five
        "# };

        let buffer_1_text_final = indoc! {r#"
            3
            4
            5
        "# };

        let buffer_2_text = indoc! {r#"
            six
            seven
            eight
            nine
            ten
        "# };

        let buffer_2_text_final = indoc! {r#"
            5
            six
            seven
            7.5
            eight
            nine
            ten
            11
        "# };

        fs.insert_tree(
            path!("/root"),
            json!({
                "file1": buffer_1_text,
                "file2": buffer_2_text,
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;

        let diff = indoc! {r#"
            --- a/file1
            +++ b/file1
             one
             two
            -three
            +3
             four
             five
            --- a/file1
            +++ b/file1
             3
            -four
            -five
            +4
            +5
            --- a/file1
            +++ b/file1
            -one
            -two
             3
             4
            --- a/file2
            +++ b/file2
            +5
             six
            --- a/file2
            +++ b/file2
             seven
            +7.5
             eight
            --- a/file2
            +++ b/file2
             ten
            +11
        "#};

        let _buffers = apply_diff(diff, &project, &mut cx.to_async())
            .await
            .unwrap();
        let buffer_1 = project
            .update(cx, |project, cx| {
                let project_path = project.find_project_path(path!("/root/file1"), cx).unwrap();
                project.open_buffer(project_path, cx)
            })
            .await
            .unwrap();

        buffer_1.read_with(cx, |buffer, _cx| {
            assert_eq!(buffer.text(), buffer_1_text_final);
        });
        let buffer_2 = project
            .update(cx, |project, cx| {
                let project_path = project.find_project_path(path!("/root/file2"), cx).unwrap();
                project.open_buffer(project_path, cx)
            })
            .await
            .unwrap();

        buffer_2.read_with(cx, |buffer, _cx| {
            assert_eq!(buffer.text(), buffer_2_text_final);
        });
    }

    #[gpui::test]
    async fn test_apply_diff_unique_via_previous_context(cx: &mut TestAppContext) {
        let fs = init_test(cx);

        let start = indoc! {r#"
            one
            two
            three
            four
            five

            four
            five
        "# };

        let end = indoc! {r#"
            one
            two
            3
            four
            5

            four
            five
        "# };

        fs.insert_tree(
            path!("/root"),
            json!({
                "file1": start,
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;

        let diff = indoc! {r#"
            --- a/file1
            +++ b/file1
             one
             two
            -three
            +3
             four
            -five
            +5
        "#};

        let _buffers = apply_diff(diff, &project, &mut cx.to_async())
            .await
            .unwrap();

        let buffer_1 = project
            .update(cx, |project, cx| {
                let project_path = project.find_project_path(path!("/root/file1"), cx).unwrap();
                project.open_buffer(project_path, cx)
            })
            .await
            .unwrap();

        buffer_1.read_with(cx, |buffer, _cx| {
            assert_eq!(buffer.text(), end);
        });
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<FakeFs> {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });

        FakeFs::new(cx.background_executor.clone())
    }

    #[test]
    fn test_extract_file_diff() {
        let multi_file_diff = indoc! {r#"
            diff --git a/file1.txt b/file1.txt
            index 1234567..abcdefg 100644
            --- a/file1.txt
            +++ b/file1.txt
            @@ -1,3 +1,4 @@
             line1
            +added line
             line2
             line3
            diff --git a/file2.txt b/file2.txt
            index 2345678..bcdefgh 100644
            --- a/file2.txt
            +++ b/file2.txt
            @@ -1,2 +1,2 @@
            -old line
            +new line
             unchanged
        "#};

        let file1_diff = extract_file_diff(multi_file_diff, "file1.txt").unwrap();
        assert_eq!(
            file1_diff,
            indoc! {r#"
                diff --git a/file1.txt b/file1.txt
                index 1234567..abcdefg 100644
                --- a/file1.txt
                +++ b/file1.txt
                @@ -1,3 +1,4 @@
                 line1
                +added line
                 line2
                 line3
            "#}
        );

        let file2_diff = extract_file_diff(multi_file_diff, "file2.txt").unwrap();
        assert_eq!(
            file2_diff,
            indoc! {r#"
                diff --git a/file2.txt b/file2.txt
                index 2345678..bcdefgh 100644
                --- a/file2.txt
                +++ b/file2.txt
                @@ -1,2 +1,2 @@
                -old line
                +new line
                 unchanged
            "#}
        );

        let result = extract_file_diff(multi_file_diff, "nonexistent.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_edits_for_diff() {
        let content = indoc! {"
            fn main() {
                let x = 1;
                let y = 2;
                println!(\"{} {}\", x, y);
            }
        "};

        let diff = indoc! {"
            --- a/file.rs
            +++ b/file.rs
            @@ -1,5 +1,5 @@
             fn main() {
            -    let x = 1;
            +    let x = 42;
                 let y = 2;
                 println!(\"{} {}\", x, y);
             }
        "};

        let edits = edits_for_diff(content, diff).unwrap();
        assert_eq!(edits.len(), 1);

        let (range, replacement) = &edits[0];
        // With sub-line diffing, the edit should start at "1" (the actual changed character)
        let expected_start = content.find("let x = 1;").unwrap() + "let x = ".len();
        assert_eq!(range.start, expected_start);
        // The deleted text is just "1"
        assert_eq!(range.end, expected_start + "1".len());
        // The replacement text
        assert_eq!(replacement, "42");

        // Verify the cursor would be positioned at the column of "1"
        let line_start = content[..range.start]
            .rfind('\n')
            .map(|p| p + 1)
            .unwrap_or(0);
        let cursor_column = range.start - line_start;
        // "    let x = " is 12 characters, so column 12
        assert_eq!(cursor_column, "    let x = ".len());
    }

    #[test]
    fn test_strip_diff_metadata() {
        let diff_with_metadata = indoc! {r#"
            diff --git a/file.txt b/file.txt
            index 1234567..abcdefg 100644
            --- a/file.txt
            +++ b/file.txt
            @@ -1,3 +1,4 @@
             context line
            -removed line
            +added line
             more context
        "#};

        let stripped = strip_diff_metadata(diff_with_metadata);

        assert_eq!(
            stripped,
            indoc! {r#"
                --- a/file.txt
                +++ b/file.txt
                @@ -1,3 +1,4 @@
                 context line
                -removed line
                +added line
                 more context
            "#}
        );
    }
}
