use std::{
    borrow::Cow,
    env,
    fmt::{self, Display},
    fs,
    io::Write,
    mem,
    ops::Range,
    path::{Path, PathBuf},
};

use anyhow::{Context as _, Result};
use clap::ValueEnum;
use collections::HashSet;
use futures::AsyncWriteExt as _;
use gpui::{AsyncApp, Entity, http_client::Url};
use language::Buffer;
use project::{Project, ProjectPath};
use pulldown_cmark::CowStr;
use serde::{Deserialize, Serialize};

const UNCOMMITTED_DIFF_HEADING: &str = "Uncommitted Diff";
const EDIT_HISTORY_HEADING: &str = "Edit History";
const CURSOR_POSITION_HEADING: &str = "Cursor Position";
const EXPECTED_PATCH_HEADING: &str = "Expected Patch";
const EXPECTED_EXCERPTS_HEADING: &str = "Expected Excerpts";
const REPOSITORY_URL_FIELD: &str = "repository_url";
const REVISION_FIELD: &str = "revision";

#[derive(Debug)]
pub struct NamedExample {
    pub name: String,
    pub example: Example,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Example {
    pub repository_url: String,
    pub revision: String,
    pub uncommitted_diff: String,
    pub cursor_path: PathBuf,
    pub cursor_position: String,
    pub edit_history: String,
    pub expected_patch: String,
    pub expected_excerpts: Vec<ExpectedExcerpt>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpectedExcerpt {
    path: PathBuf,
    text: String,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ExampleFormat {
    Json,
    Toml,
    Md,
}

impl NamedExample {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        let ext = path.extension();

        match ext.and_then(|s| s.to_str()) {
            Some("json") => Ok(Self {
                name: path.file_stem().unwrap_or_default().display().to_string(),
                example: serde_json::from_str(&content)?,
            }),
            Some("toml") => Ok(Self {
                name: path.file_stem().unwrap_or_default().display().to_string(),
                example: toml::from_str(&content)?,
            }),
            Some("md") => Self::parse_md(&content),
            Some(_) => {
                anyhow::bail!("Unrecognized example extension: {}", ext.unwrap().display());
            }
            None => {
                anyhow::bail!(
                    "Failed to determine example type since the file does not have an extension."
                );
            }
        }
    }

    pub fn parse_md(input: &str) -> Result<Self> {
        use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};

        let parser = Parser::new(input);

        let mut named = NamedExample {
            name: String::new(),
            example: Example {
                repository_url: String::new(),
                revision: String::new(),
                uncommitted_diff: String::new(),
                cursor_path: PathBuf::new(),
                cursor_position: String::new(),
                edit_history: String::new(),
                expected_patch: String::new(),
                expected_excerpts: Vec::new(),
            },
        };

        let mut text = String::new();
        let mut current_section = String::new();
        let mut block_info: CowStr = "".into();

        for event in parser {
            match event {
                Event::Text(line) => {
                    text.push_str(&line);

                    if !named.name.is_empty()
                        && current_section.is_empty()
                        // in h1 section
                        && let Some((field, value)) = line.split_once('=')
                    {
                        match field.trim() {
                            REPOSITORY_URL_FIELD => {
                                named.example.repository_url = value.trim().to_string();
                            }
                            REVISION_FIELD => {
                                named.example.revision = value.trim().to_string();
                            }
                            _ => {
                                eprintln!("Warning: Unrecognized field `{field}`");
                            }
                        }
                    }
                }
                Event::End(TagEnd::Heading(HeadingLevel::H1)) => {
                    if !named.name.is_empty() {
                        anyhow::bail!(
                            "Found multiple H1 headings. There should only be one with the name of the example."
                        );
                    }
                    named.name = mem::take(&mut text);
                }
                Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
                    current_section = mem::take(&mut text);
                }
                Event::End(TagEnd::Heading(level)) => {
                    anyhow::bail!("Unexpected heading level: {level}");
                }
                Event::Start(Tag::CodeBlock(kind)) => {
                    match kind {
                        CodeBlockKind::Fenced(info) => {
                            block_info = info;
                        }
                        CodeBlockKind::Indented => {
                            anyhow::bail!("Unexpected indented codeblock");
                        }
                    };
                }
                Event::Start(_) => {
                    text.clear();
                    block_info = "".into();
                }
                Event::End(TagEnd::CodeBlock) => {
                    let block_info = block_info.trim();
                    if current_section.eq_ignore_ascii_case(UNCOMMITTED_DIFF_HEADING) {
                        named.example.uncommitted_diff = mem::take(&mut text);
                    } else if current_section.eq_ignore_ascii_case(EDIT_HISTORY_HEADING) {
                        named.example.edit_history.push_str(&mem::take(&mut text));
                    } else if current_section.eq_ignore_ascii_case(CURSOR_POSITION_HEADING) {
                        named.example.cursor_path = block_info.into();
                        named.example.cursor_position = mem::take(&mut text);
                    } else if current_section.eq_ignore_ascii_case(EXPECTED_PATCH_HEADING) {
                        named.example.expected_patch = mem::take(&mut text);
                    } else if current_section.eq_ignore_ascii_case(EXPECTED_EXCERPTS_HEADING) {
                        named.example.expected_excerpts.push(ExpectedExcerpt {
                            path: block_info.into(),
                            text: mem::take(&mut text),
                        });
                    } else {
                        eprintln!("Warning: Unrecognized section `{current_section:?}`")
                    }
                }
                _ => {}
            }
        }

        if named.example.cursor_path.as_path() == Path::new("")
            || named.example.cursor_position.is_empty()
        {
            anyhow::bail!("Missing cursor position codeblock");
        }

        Ok(named)
    }

    pub fn write(&self, format: ExampleFormat, mut out: impl Write) -> Result<()> {
        match format {
            ExampleFormat::Json => Ok(serde_json::to_writer(out, &self.example)?),
            ExampleFormat::Toml => {
                Ok(out.write_all(toml::to_string_pretty(&self.example)?.as_bytes())?)
            }
            ExampleFormat::Md => Ok(write!(out, "{}", self)?),
        }
    }

    #[allow(unused)]
    pub async fn setup_worktree(&self) -> Result<PathBuf> {
        let (repo_owner, repo_name) = self.repo_name()?;
        let file_name = self.file_name();

        let worktrees_dir = env::current_dir()?.join("target").join("zeta-worktrees");
        let repos_dir = env::current_dir()?.join("target").join("zeta-repos");
        fs::create_dir_all(&repos_dir)?;
        fs::create_dir_all(&worktrees_dir)?;

        let repo_dir = repos_dir.join(repo_owner.as_ref()).join(repo_name.as_ref());
        if !repo_dir.is_dir() {
            fs::create_dir_all(&repo_dir)?;
            run_git(&repo_dir, &["init"]).await?;
            run_git(
                &repo_dir,
                &["remote", "add", "origin", &self.example.repository_url],
            )
            .await?;
        }

        // Resolve the example to a revision, fetching it if needed.
        let revision = run_git(&repo_dir, &["rev-parse", &self.example.revision]).await;
        let revision = if let Ok(revision) = revision {
            revision
        } else {
            run_git(
                &repo_dir,
                &["fetch", "--depth", "1", "origin", &self.example.revision],
            )
            .await?;
            let revision = run_git(&repo_dir, &["rev-parse", "FETCH_HEAD"]).await?;
            if revision != self.example.revision {
                run_git(&repo_dir, &["tag", &self.example.revision, &revision]).await?;
            }
            revision
        };

        // Create the worktree for this example if needed.
        let worktree_path = worktrees_dir.join(&file_name);
        if worktree_path.is_dir() {
            run_git(&worktree_path, &["clean", "--force", "-d"]).await?;
            run_git(&worktree_path, &["reset", "--hard", "HEAD"]).await?;
            run_git(&worktree_path, &["checkout", revision.as_str()]).await?;
        } else {
            let worktree_path_string = worktree_path.to_string_lossy();
            run_git(&repo_dir, &["branch", "-f", &file_name, revision.as_str()]).await?;
            run_git(
                &repo_dir,
                &["worktree", "add", "-f", &worktree_path_string, &file_name],
            )
            .await?;
        }

        // Apply the uncommitted diff for this example.
        if !self.example.uncommitted_diff.is_empty() {
            let mut apply_process = smol::process::Command::new("git")
                .current_dir(&worktree_path)
                .args(&["apply", "-"])
                .stdin(std::process::Stdio::piped())
                .spawn()?;

            let mut stdin = apply_process.stdin.take().unwrap();
            stdin
                .write_all(self.example.uncommitted_diff.as_bytes())
                .await?;
            stdin.close().await?;
            drop(stdin);

            let apply_result = apply_process.output().await?;
            if !apply_result.status.success() {
                anyhow::bail!(
                    "Failed to apply uncommitted diff patch with status: {}\nstderr:\n{}\nstdout:\n{}",
                    apply_result.status,
                    String::from_utf8_lossy(&apply_result.stderr),
                    String::from_utf8_lossy(&apply_result.stdout),
                );
            }
        }

        Ok(worktree_path)
    }

    fn file_name(&self) -> String {
        self.name
            .chars()
            .map(|c| {
                if c.is_whitespace() {
                    '-'
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .collect()
    }

    #[allow(unused)]
    fn repo_name(&self) -> Result<(Cow<'_, str>, Cow<'_, str>)> {
        // git@github.com:owner/repo.git
        if self.example.repository_url.contains('@') {
            let (owner, repo) = self
                .example
                .repository_url
                .split_once(':')
                .context("expected : in git url")?
                .1
                .split_once('/')
                .context("expected / in git url")?;
            Ok((
                Cow::Borrowed(owner),
                Cow::Borrowed(repo.trim_end_matches(".git")),
            ))
        // http://github.com/owner/repo.git
        } else {
            let url = Url::parse(&self.example.repository_url)?;
            let mut segments = url.path_segments().context("empty http url")?;
            let owner = segments
                .next()
                .context("expected owner path segment")?
                .to_string();
            let repo = segments
                .next()
                .context("expected repo path segment")?
                .trim_end_matches(".git")
                .to_string();
            assert!(segments.next().is_none());

            Ok((owner.into(), repo.into()))
        }
    }

    #[must_use]
    pub async fn apply_edit_history(
        &self,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<HashSet<Entity<Buffer>>> {
        apply_diff(&self.example.edit_history, project, cx).await
    }
}

async fn run_git(repo_path: &Path, args: &[&str]) -> Result<String> {
    let output = smol::process::Command::new("git")
        .current_dir(repo_path)
        .args(args)
        .output()
        .await?;

    anyhow::ensure!(
        output.status.success(),
        "`git {}` within `{}` failed with status: {}\nstderr:\n{}\nstdout:\n{}",
        args.join(" "),
        repo_path.display(),
        output.status,
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout),
    );
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

impl Display for NamedExample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "# {}\n\n", self.name)?;
        write!(
            f,
            "{REPOSITORY_URL_FIELD} = {}\n",
            self.example.repository_url
        )?;
        write!(f, "{REVISION_FIELD} = {}\n\n", self.example.revision)?;

        write!(f, "## {UNCOMMITTED_DIFF_HEADING}\n\n")?;
        write!(f, "`````diff\n")?;
        write!(f, "{}", self.example.uncommitted_diff)?;
        write!(f, "`````\n")?;

        if !self.example.edit_history.is_empty() {
            write!(f, "`````diff\n{}`````\n", self.example.edit_history)?;
        }

        write!(
            f,
            "## {CURSOR_POSITION_HEADING}\n\n`````{}\n{}`````\n",
            self.example.cursor_path.display(),
            self.example.cursor_position
        )?;
        write!(f, "## {EDIT_HISTORY_HEADING}\n\n")?;

        if !self.example.expected_patch.is_empty() {
            write!(
                f,
                "\n## {EXPECTED_PATCH_HEADING}\n\n`````diff\n{}`````\n",
                self.example.expected_patch
            )?;
        }

        if !self.example.expected_excerpts.is_empty() {
            write!(f, "\n## {EXPECTED_EXCERPTS_HEADING}\n\n")?;

            for excerpt in &self.example.expected_excerpts {
                write!(
                    f,
                    "`````{}{}\n{}`````\n\n",
                    excerpt
                        .path
                        .extension()
                        .map(|ext| format!("{} ", ext.to_string_lossy()))
                        .unwrap_or_default(),
                    excerpt.path.display(),
                    excerpt.text
                )?;
            }
        }

        Ok(())
    }
}

#[must_use]
pub async fn apply_diff(
    diff: &str,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<HashSet<Entity<Buffer>>> {
    use cloud_llm_client::udiff::DiffLine;
    use std::fmt::Write;

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

    let mut old_path = None;
    let mut new_path = None;
    let mut hunk = HunkState::default();
    let mut diff_lines = diff.lines().map(DiffLine::parse).peekable();
    let mut open_buffers = HashSet::default();

    while let Some(diff_line) = diff_lines.next() {
        match diff_line {
            DiffLine::OldPath { path } => old_path = Some(path),
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

        let at_hunk_end = match diff_lines.peek() {
            Some(DiffLine::OldPath { .. }) | Some(DiffLine::HunkHeader(_)) | None => true,
            _ => false,
        };

        if at_hunk_end {
            let hunk = mem::take(&mut hunk);

            let Some(old_path) = old_path.as_deref() else {
                anyhow::bail!("Missing old path (`---`) header")
            };

            let Some(new_path) = new_path.as_deref() else {
                anyhow::bail!("Missing new path (`+++`) header")
            };

            let buffer = project
                .update(cx, |project, cx| {
                    let project_path = project
                        .find_project_path(old_path, cx)
                        .context("Failed to find old_path in project")?;

                    anyhow::Ok(project.open_buffer(project_path, cx))
                })??
                .await?;
            open_buffers.insert(buffer.clone());

            if old_path != new_path {
                project
                    .update(cx, |project, cx| {
                        let project_file = project::File::from_dyn(buffer.read(cx).file()).unwrap();
                        let new_path = ProjectPath {
                            worktree_id: project_file.worktree_id(cx),
                            path: project_file.path.clone(),
                        };
                        project.rename_entry(project_file.entry_id.unwrap(), new_path, cx)
                    })?
                    .await?;
            }

            // TODO is it worth using project search?
            buffer.update(cx, |buffer, cx| {
                let context_offset = if hunk.context.is_empty() {
                    0
                } else {
                    let text = buffer.text();
                    if let Some(offset) = text.find(&hunk.context) {
                        if text[offset + 1..].contains(&hunk.context) {
                            anyhow::bail!("Context is not unique enough:\n{}", hunk.context);
                        }
                        offset
                    } else {
                        anyhow::bail!(
                            "Failed to match context:\n{}\n\nBuffer:\n{}",
                            hunk.context,
                            text
                        );
                    }
                };

                buffer.edit(
                    hunk.edits.into_iter().map(|edit| {
                        (
                            context_offset + edit.range.start..context_offset + edit.range.end,
                            edit.text,
                        )
                    }),
                    None,
                    cx,
                );

                anyhow::Ok(())
            })??;
        }
    }

    anyhow::Ok(open_buffers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::fs::FakeFs;
    use gpui::TestAppContext;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_apply_diff_successful(cx: &mut TestAppContext) {
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

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            language::init(cx);
        });

        let fs = FakeFs::new(cx.background_executor.clone());
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

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            language::init(cx);
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "file1": buffer_1_text,
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
        "#};

        apply_diff(diff, &project, &mut cx.to_async())
            .await
            .expect_err("Non-unique edits should fail");
    }

    #[gpui::test]
    async fn test_apply_diff_unique_via_previous_context(cx: &mut TestAppContext) {
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

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            language::init(cx);
        });

        let fs = FakeFs::new(cx.background_executor.clone());
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
}
