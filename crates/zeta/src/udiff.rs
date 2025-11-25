use std::borrow::Cow;
use std::fmt::Display;
use std::sync::Arc;
use std::{
    fmt::{Debug, Write},
    mem,
    ops::Range,
    path::Path,
};

use anyhow::Context as _;
use anyhow::Result;
use anyhow::anyhow;
use collections::HashMap;
use gpui::AsyncApp;
use gpui::Entity;
use language::{Anchor, Buffer, BufferSnapshot, OffsetRangeExt as _, TextBufferSnapshot};
use project::Project;

pub async fn parse_diff<'a>(
    diff_str: &'a str,
    get_buffer: impl Fn(&Path) -> Option<(&'a BufferSnapshot, &'a [Range<Anchor>])> + Send,
) -> Result<(&'a BufferSnapshot, Vec<(Range<Anchor>, Arc<str>)>)> {
    let mut diff = DiffParser::new(diff_str);
    let mut edited_buffer = None;
    let mut edits = Vec::new();

    while let Some(event) = diff.next()? {
        match event {
            DiffEvent::Hunk {
                path: file_path,
                hunk,
            } => {
                let (buffer, ranges) = match edited_buffer {
                    None => {
                        edited_buffer = get_buffer(&Path::new(file_path.as_ref()));
                        edited_buffer
                            .as_ref()
                            .context("Model tried to edit a file that wasn't included")?
                    }
                    Some(ref current) => current,
                };

                edits.extend(
                    resolve_hunk_edits_in_buffer(hunk, &buffer.text, ranges)
                        .with_context(|| format!("Diff:\n{diff_str}"))?,
                );
            }
            DiffEvent::FileEnd { renamed_to } => {
                let (buffer, _) = edited_buffer
                    .take()
                    .context("Got a FileEnd event before an Hunk event")?;

                if renamed_to.is_some() {
                    anyhow::bail!("edit predictions cannot rename files");
                }

                if diff.next()?.is_some() {
                    anyhow::bail!("Edited more than one file");
                }

                return Ok((buffer, edits));
            }
        }
    }

    Err(anyhow::anyhow!("No EOF"))
}

#[derive(Debug)]
pub struct OpenedBuffers<'a>(#[allow(unused)] HashMap<Cow<'a, str>, Entity<Buffer>>);

#[must_use]
pub async fn apply_diff<'a>(
    diff_str: &'a str,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<OpenedBuffers<'a>> {
    let mut included_files = HashMap::default();

    for line in diff_str.lines() {
        let diff_line = DiffLine::parse(line);

        if let DiffLine::OldPath { path } = diff_line {
            let buffer = project
                .update(cx, |project, cx| {
                    let project_path =
                        project
                            .find_project_path(path.as_ref(), cx)
                            .with_context(|| {
                                format!("Failed to find worktree for new path: {}", path)
                            })?;
                    anyhow::Ok(project.open_buffer(project_path, cx))
                })??
                .await?;

            included_files.insert(path, buffer);
        }
    }

    let ranges = [Anchor::MIN..Anchor::MAX];

    let mut diff = DiffParser::new(diff_str);
    let mut current_file = None;
    let mut edits = vec![];

    while let Some(event) = diff.next()? {
        match event {
            DiffEvent::Hunk {
                path: file_path,
                hunk,
            } => {
                let (buffer, ranges) = match current_file {
                    None => {
                        let buffer = included_files
                            .get_mut(&file_path)
                            .expect("Opened all files in diff");

                        current_file = Some((buffer, ranges.as_slice()));
                        current_file.as_ref().unwrap()
                    }
                    Some(ref current) => current,
                };

                buffer.read_with(cx, |buffer, _| {
                    edits.extend(
                        resolve_hunk_edits_in_buffer(hunk, buffer, ranges)
                            .with_context(|| format!("Diff:\n{diff_str}"))?,
                    );
                    anyhow::Ok(())
                })??;
            }
            DiffEvent::FileEnd { renamed_to } => {
                let (buffer, _) = current_file
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
    use language::Point;
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
            --- a/root/file1
            +++ b/root/file1
             one
             two
            -three
            +3
             four
             five
            --- a/root/file1
            +++ b/root/file1
             3
            -four
            -five
            +4
            +5
            --- a/root/file1
            +++ b/root/file1
            -one
            -two
             3
             4
            --- a/root/file2
            +++ b/root/file2
            +5
             six
            --- a/root/file2
            +++ b/root/file2
             seven
            +7.5
             eight
            --- a/root/file2
            +++ b/root/file2
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
    async fn test_apply_diff_non_unique(cx: &mut TestAppContext) {
        let fs = init_test(cx);

        let buffer_1_text = indoc! {r#"
            one
            two
            three
            four
            five
            one
            two
            three
            four
            five
        "# };

        fs.insert_tree(
            path!("/root"),
            json!({
                "file1": buffer_1_text,
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/file1"), cx)
            })
            .await
            .unwrap();
        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let diff = indoc! {r#"
            --- a/root/file1
            +++ b/root/file1
             one
             two
            -three
            +3
             four
             five
        "#};

        let final_text = indoc! {r#"
            one
            two
            three
            four
            five
            one
            two
            3
            four
            five
        "#};

        apply_diff(diff, &project, &mut cx.to_async())
            .await
            .expect_err("Non-unique edits should fail");

        let ranges = [buffer_snapshot.anchor_before(Point::new(1, 0))
            ..buffer_snapshot.anchor_after(buffer_snapshot.max_point())];

        let (edited_snapshot, edits) = parse_diff(diff, |_path| Some((&buffer_snapshot, &ranges)))
            .await
            .unwrap();

        assert_eq!(edited_snapshot.remote_id(), buffer_snapshot.remote_id());
        buffer.update(cx, |buffer, cx| {
            buffer.edit(edits, None, cx);
            assert_eq!(buffer.text(), final_text);
        });
    }

    #[gpui::test]
    async fn test_parse_diff_with_edits_within_line(cx: &mut TestAppContext) {
        let fs = init_test(cx);

        let buffer_1_text = indoc! {r#"
            one two three four
            five six seven eight
            nine ten eleven twelve
        "# };

        fs.insert_tree(
            path!("/root"),
            json!({
                "file1": buffer_1_text,
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/file1"), cx)
            })
            .await
            .unwrap();
        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let diff = indoc! {r#"
            --- a/root/file1
            +++ b/root/file1
             one two three four
            -five six seven eight
            +five SIX seven eight!
             nine ten eleven twelve
        "#};

        let (buffer, edits) = parse_diff(diff, |_path| {
            Some((&buffer_snapshot, &[(Anchor::MIN..Anchor::MAX)] as &[_]))
        })
        .await
        .unwrap();

        let edits = edits
            .into_iter()
            .map(|(range, text)| (range.to_point(&buffer), text))
            .collect::<Vec<_>>();
        assert_eq!(
            edits,
            &[
                (Point::new(1, 5)..Point::new(1, 8), "SIX".into()),
                (Point::new(1, 20)..Point::new(1, 20), "!".into())
            ]
        );
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
            --- a/root/file1
            +++ b/root/file1
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
}
