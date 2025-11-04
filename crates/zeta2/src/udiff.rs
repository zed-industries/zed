use std::borrow::Cow;
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
use language::{Anchor, Buffer, BufferId, BufferSnapshot, OffsetRangeExt as _};
use project::Project;

#[derive(Debug, Default)]
struct HunkState {
    context: String,
    edits: Vec<Edit>,
}

#[derive(Debug)]
struct Edit {
    range: Range<usize>,
    text: String,
}

pub async fn parse_diff<'a>(
    diff: &str,
    get_buffer: impl Fn(&Path) -> Option<(&'a BufferSnapshot, &'a [Range<Anchor>])> + Send,
) -> Result<(BufferSnapshot, Vec<(Range<Anchor>, Arc<str>)>)> {
    let mut edited_buffer = None;
    let mut buffer_edits = Vec::new();
    process_diff(
        &(),
        diff,
        |path, _| get_buffer(path),
        async |buffer, renamed_to, edits| {
            if edited_buffer.is_some() {
                anyhow::bail!("edited more than one file");
            }
            if renamed_to.is_some() {
                anyhow::bail!("edit predictions cannot rename files");
            }
            edited_buffer = Some(buffer.clone());
            buffer_edits = edits;
            Ok(())
        },
    )
    .await?;
    Ok((edited_buffer.context("no files in diff")?, buffer_edits))
}

#[must_use]
pub async fn apply_diff(
    diff: &str,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<HashMap<BufferId, Entity<Buffer>>> {
    let mut included_files = HashMap::default();
    let mut opened_buffers = HashMap::default();
    for line in diff.lines() {
        let diff_line = DiffLine::parse(line);
        if let DiffLine::OldPath { path } = diff_line {
            let buffer = project
                .update(cx, |project, cx| {
                    let project_path = project
                        .find_project_path(path.as_ref(), cx)
                        .context("Failed to find worktree for new path")?;
                    anyhow::Ok(project.open_buffer(project_path, cx))
                })??
                .await?;

            let path = Arc::from(Path::new(path.as_ref()));
            let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
            opened_buffers.insert(snapshot.remote_id(), buffer.clone());
            included_files.insert(path, (buffer, snapshot));
        }
    }

    let ranges = [Anchor::MIN..Anchor::MAX];
    let cx_2 = cx.clone();

    process_diff(
        &mut included_files,
        diff,
        {
            |path, included_files| {
                let (buffer, snapshot) = included_files.get_mut(path)?;
                *snapshot = buffer
                    .read_with(&cx_2, |buffer, _| buffer.snapshot())
                    .ok()?;
                Some((&*snapshot, ranges.as_slice()))
            }
        },
        async |buffer, new_path, edits| {
            if let Some(new_path) = new_path {
                project
                    .update(cx, |project, cx| {
                        let new_project_path = project
                            .find_project_path(new_path, cx)
                            .context("Failed to find worktree for new path")?;
                        let project_file =
                            project::File::from_dyn(buffer.file()).context("Wrong file type")?;
                        anyhow::Ok(project.rename_entry(
                            project_file.entry_id.unwrap(),
                            new_project_path,
                            cx,
                        ))
                    })??
                    .await?;
            }

            let buffer = opened_buffers.get(&buffer.remote_id()).unwrap();
            buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            })
        },
    )
    .await?;
    Ok(opened_buffers)
}

async fn process_diff<'a, T>(
    mut payload: T,
    diff: &str,
    get_buffer: impl for<'b> Fn(&Path, &'b mut T) -> Option<(&'b BufferSnapshot, &'b [Range<Anchor>])>,
    mut on_buffer: impl AsyncFnMut(
        &BufferSnapshot,
        Option<&Path>,
        Vec<(Range<Anchor>, Arc<str>)>,
    ) -> Result<()>,
) -> Result<()> {
    // let mut current_file: Option<(std::path::PathBuf, &BufferSnapshot, &[Range<Anchor>])> = None;
    let mut old_path = None;
    let mut new_path = None;
    let mut hunk = HunkState::default();
    let mut edits = Vec::new();

    let mut diff_lines = diff.lines().map(DiffLine::parse).peekable();
    while let Some(diff_line) = diff_lines.next() {
        match diff_line {
            DiffLine::OldPath { path } => {
                old_path = Some(path);
            }
            DiffLine::NewPath { path } => {
                if old_path.is_none() {
                    anyhow::bail!(
                        "Found a new path header (`+++`) before an (`---`) old path header"
                    );
                }
                new_path = Some(path)
            }
            DiffLine::Context(ctx) => {
                writeln!(&mut hunk.context, "{ctx}")?;
            }
            DiffLine::Deletion(del) => {
                let range = hunk.context.len()..hunk.context.len() + del.len() + '\n'.len_utf8();
                if let Some(last_edit) = hunk.edits.last_mut()
                    && last_edit.range.end == range.start
                {
                    last_edit.range.end = range.end;
                } else {
                    hunk.edits.push(Edit {
                        range,
                        text: String::new(),
                    });
                }
                writeln!(&mut hunk.context, "{del}")?;
            }
            DiffLine::Addition(add) => {
                let range = hunk.context.len()..hunk.context.len();
                if let Some(last_edit) = hunk.edits.last_mut()
                    && last_edit.range.end == range.start
                {
                    writeln!(&mut last_edit.text, "{add}").unwrap();
                } else {
                    hunk.edits.push(Edit {
                        range,
                        text: format!("{add}\n"),
                    });
                }
            }
            DiffLine::HunkHeader(_) | DiffLine::Garbage => {}
        }

        let (at_hunk_end, at_file_end) = match diff_lines.peek() {
            Some(DiffLine::OldPath { .. }) | None => (true, true),
            Some(DiffLine::HunkHeader(_)) => (true, false),
            _ => (false, false),
        };

        if at_hunk_end {
            let hunk = mem::take(&mut hunk);

            let Some(old_path) = old_path.as_ref() else {
                anyhow::bail!("Missing old path (`---`) header")
            };
            let old_path = Path::new(old_path.as_ref());
            let (buffer, ranges) = get_buffer(old_path, &mut payload).context("")?;

            // TODO is it worth using project search?
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
                offset.ok_or_else(|| {
                    anyhow!(
                        "Failed to match context:\n{}\n\nBuffer:\n{}",
                        hunk.context,
                        buffer.text(),
                    )
                })
            }?;

            edits.extend(hunk.edits.into_iter().flat_map(|edit| {
                let old_text = buffer
                    .text_for_range(
                        context_offset + edit.range.start..context_offset + edit.range.end,
                    )
                    .collect::<String>();
                let edits_within_hunk = language::text_diff(&old_text, &edit.text);
                edits_within_hunk
                    .into_iter()
                    .map(move |(inner_range, inner_text)| {
                        (
                            buffer
                                .anchor_after(context_offset + edit.range.start + inner_range.start)
                                ..buffer.anchor_before(
                                    context_offset + edit.range.start + inner_range.end,
                                ),
                            inner_text,
                        )
                    })
            }));

            if at_file_end {
                let Some(new_path) = new_path.take() else {
                    anyhow::bail!("Missing new path (`+++`) header")
                };
                let new_path = Path::new(new_path.as_ref());
                let renamed_to = if old_path != new_path {
                    Some(new_path)
                } else {
                    None
                };
                on_buffer(buffer, renamed_to, mem::take(&mut edits)).await?;
            }
        }
    }

    anyhow::Ok(())
}

#[derive(Debug, PartialEq)]
pub enum DiffLine<'a> {
    OldPath { path: Cow<'a, str> },
    NewPath { path: Cow<'a, str> },
    HunkHeader(Option<HunkLocation>),
    Context(&'a str),
    Deletion(&'a str),
    Addition(&'a str),
    Garbage,
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
        Self::try_parse(line).unwrap_or(Self::Garbage)
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

            let (start_line_old, header) = header.strip_prefix('-')?.split_once(',')?;
            let mut parts = header.split_ascii_whitespace();
            let count_old = parts.next()?;
            let (start_line_new, count_new) = parts.next()?.strip_prefix('+')?.split_once(',')?;

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
                DiffLine::Garbage,
                DiffLine::Garbage,
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
                DiffLine::Garbage,
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
            Project::init_settings(cx);
            language::init(cx);
        });

        FakeFs::new(cx.background_executor.clone())
    }
}
