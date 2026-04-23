use std::{mem, ops::Range, path::Path, path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result, anyhow};
use collections::{HashMap, hash_map::Entry};
use gpui::{AsyncApp, Entity};
use language::{Anchor, Buffer, OffsetRangeExt as _, TextBufferSnapshot, text_diff};
use postage::stream::Stream as _;
use project::Project;
use util::{paths::PathStyle, rel_path::RelPath};
use worktree::Worktree;
use zeta_prompt::udiff::{
    DiffEvent, DiffParser, FileStatus, Hunk, disambiguate_by_line_number, find_context_candidates,
};

pub use zeta_prompt::udiff::{
    DiffLine, HunkLocation, apply_diff_to_string, apply_diff_to_string_with_hunk_offset,
    strip_diff_metadata, strip_diff_path_prefix,
};

#[derive(Clone, Debug)]
pub struct OpenedBuffers(HashMap<String, Entity<Buffer>>);

impl OpenedBuffers {
    pub fn get(&self, path: &str) -> Option<&Entity<Buffer>> {
        self.0.get(path)
    }

    pub fn buffers(&self) -> impl Iterator<Item = &Entity<Buffer>> {
        self.0.values()
    }
}

#[must_use]
pub async fn apply_diff(
    diff_str: &str,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<OpenedBuffers> {
    let worktree = project
        .read_with(cx, |project, cx| project.visible_worktrees(cx).next())
        .context("project has no worktree")?;

    let paths: Vec<_> = diff_str
        .lines()
        .filter_map(|line| {
            if let DiffLine::OldPath { path } = DiffLine::parse(line) {
                if path != "/dev/null" {
                    return Some(PathBuf::from(path.as_ref()));
                }
            }
            None
        })
        .collect();
    refresh_worktree_entries(&worktree, paths.iter().map(|p| p.as_path()), cx).await?;

    let mut included_files: HashMap<String, Entity<Buffer>> = HashMap::default();

    let mut diff = DiffParser::new(diff_str);
    let mut current_file = None;
    let mut edits: Vec<(std::ops::Range<Anchor>, Arc<str>)> = vec![];

    while let Some(event) = diff.next()? {
        match event {
            DiffEvent::Hunk { path, hunk, status } => {
                if status == FileStatus::Deleted {
                    let delete_task = project.update(cx, |project, cx| {
                        if let Some(path) = project.find_project_path(path.as_ref(), cx) {
                            project.delete_file(path, false, cx)
                        } else {
                            None
                        }
                    });

                    if let Some(delete_task) = delete_task {
                        delete_task.await?;
                    };

                    continue;
                }

                let buffer = match current_file {
                    None => {
                        let buffer = match included_files.entry(path.to_string()) {
                            Entry::Occupied(entry) => entry.get().clone(),
                            Entry::Vacant(entry) => {
                                let buffer: Entity<Buffer> = if status == FileStatus::Created {
                                    project
                                        .update(cx, |project, cx| {
                                            project.create_buffer(None, true, cx)
                                        })
                                        .await?
                                } else {
                                    let project_path = project
                                        .update(cx, |project, cx| {
                                            project.find_project_path(path.as_ref(), cx)
                                        })
                                        .with_context(|| format!("no such path: {}", path))?;
                                    project
                                        .update(cx, |project, cx| {
                                            project.open_buffer(project_path, cx)
                                        })
                                        .await?
                                };
                                entry.insert(buffer.clone());
                                buffer
                            }
                        };
                        current_file = Some(buffer);
                        current_file.as_ref().unwrap()
                    }
                    Some(ref current) => current,
                };

                buffer.read_with(cx, |buffer, _| {
                    edits.extend(resolve_hunk_edits_in_buffer(
                        hunk,
                        buffer,
                        &[Anchor::min_max_range_for_buffer(buffer.remote_id())],
                        status,
                    )?);
                    anyhow::Ok(())
                })?;
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
                        })?
                        .await?;
                }

                let edits = mem::take(&mut edits);
                buffer.update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                });
            }
        }
    }

    Ok(OpenedBuffers(included_files))
}

pub async fn refresh_worktree_entries(
    worktree: &Entity<Worktree>,
    paths: impl IntoIterator<Item = &Path>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let mut rel_paths = Vec::new();
    for path in paths {
        if let Ok(rel_path) = RelPath::new(path, PathStyle::Posix) {
            rel_paths.push(rel_path.into_arc());
        }

        let path_without_root: PathBuf = path.components().skip(1).collect();
        if let Ok(rel_path) = RelPath::new(&path_without_root, PathStyle::Posix) {
            rel_paths.push(rel_path.into_arc());
        }
    }

    if !rel_paths.is_empty() {
        worktree
            .update(cx, |worktree, _| {
                worktree
                    .as_local()
                    .unwrap()
                    .refresh_entries_for_paths(rel_paths)
            })
            .recv()
            .await;
    }

    Ok(())
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
            DiffEvent::Hunk {
                mut hunk,
                path: _,
                status: _,
            } => {
                if hunk.context.is_empty() {
                    return Ok(Vec::new());
                }

                let candidates = find_context_candidates(content, &mut hunk);

                let Some(context_offset) =
                    disambiguate_by_line_number(&candidates, hunk.start_line, &|offset| {
                        content[..offset].matches('\n').count() as u32
                    })
                else {
                    return Ok(Vec::new());
                };

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

fn resolve_hunk_edits_in_buffer(
    mut hunk: Hunk,
    buffer: &TextBufferSnapshot,
    ranges: &[Range<Anchor>],
    status: FileStatus,
) -> Result<impl Iterator<Item = (Range<Anchor>, Arc<str>)>, anyhow::Error> {
    let context_offset = if status == FileStatus::Created || hunk.context.is_empty() {
        0
    } else {
        let mut candidates: Vec<usize> = Vec::new();
        for range in ranges {
            let range = range.to_offset(buffer);
            let text = buffer.text_for_range(range.clone()).collect::<String>();
            for ix in find_context_candidates(&text, &mut hunk) {
                candidates.push(range.start + ix);
            }
        }

        disambiguate_by_line_number(&candidates, hunk.start_line, &|offset| {
            buffer.offset_to_point(offset).row
        })
        .ok_or_else(|| {
            if candidates.is_empty() {
                anyhow!("Failed to match context:\n\n```\n{}```\n", hunk.context,)
            } else {
                anyhow!("Context is not unique enough:\n{}", hunk.context)
            }
        })?
    };

    if let Some(edit) = hunk.edits.iter().find(|edit| edit.range.end > buffer.len()) {
        return Err(anyhow!("Edit range {:?} exceeds buffer length", edit.range));
    }

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
    fn test_line_number_disambiguation() {
        // Test that line numbers from hunk headers are used to disambiguate
        // when context before the operation appears multiple times
        let content = indoc! {"
            repeated line
            first unique
            repeated line
            second unique
        "};

        // Context "repeated line" appears twice - line number selects first occurrence
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,2 +1,2 @@
             repeated line
            -first unique
            +REPLACED
        "};

        let result = edits_for_diff(content, diff).unwrap();
        assert_eq!(result.len(), 1);

        // The edit should replace "first unique" (after first "repeated line\n" at offset 14)
        let (range, text) = &result[0];
        assert_eq!(range.start, 14);
        assert_eq!(range.end, 26); // "first unique" is 12 bytes
        assert_eq!(text, "REPLACED");
    }

    #[test]
    fn test_line_number_disambiguation_second_match() {
        // Test disambiguation when the edit should apply to a later occurrence
        let content = indoc! {"
            repeated line
            first unique
            repeated line
            second unique
        "};

        // Context "repeated line" appears twice - line number selects second occurrence
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -3,2 +3,2 @@
             repeated line
            -second unique
            +REPLACED
        "};

        let result = edits_for_diff(content, diff).unwrap();
        assert_eq!(result.len(), 1);

        // The edit should replace "second unique" (after second "repeated line\n")
        // Offset: "repeated line\n" (14) + "first unique\n" (13) + "repeated line\n" (14) = 41
        let (range, text) = &result[0];
        assert_eq!(range.start, 41);
        assert_eq!(range.end, 54); // "second unique" is 13 bytes
        assert_eq!(text, "REPLACED");
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
    fn test_edits_for_diff_no_trailing_newline() {
        let content = "foo\nbar\nbaz";
        let diff = indoc! {"
            --- a/file.txt
            +++ b/file.txt
            @@ -1,3 +1,3 @@
             foo
            -bar
            +qux
             baz
        "};

        let result = edits_for_diff(content, diff).unwrap();
        assert_eq!(result.len(), 1);
        let (range, text) = &result[0];
        assert_eq!(&content[range.clone()], "bar");
        assert_eq!(text, "qux");
    }
}
