use std::{
    borrow::Cow,
    cell::RefCell,
    fmt::{self, Display},
    fs,
    io::Write,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context as _, Result, anyhow};
use clap::ValueEnum;
use cloud_zeta2_prompt::CURSOR_MARKER;
use collections::{HashMap, HashSet};
use edit_prediction_context::Line;
use futures::{
    AsyncWriteExt as _,
    lock::{Mutex, OwnedMutexGuard},
};
use gpui::{AsyncApp, Entity, http_client::Url};
use language::{Anchor, Buffer};
use project::{Project, ProjectPath};
use pulldown_cmark::CowStr;
use serde::{Deserialize, Serialize};
use util::{paths::PathStyle, rel_path::RelPath};
use zeta2::udiff::OpenedBuffers;

use crate::paths::{REPOS_DIR, WORKTREES_DIR};

const UNCOMMITTED_DIFF_HEADING: &str = "Uncommitted Diff";
const EDIT_HISTORY_HEADING: &str = "Edit History";
const CURSOR_POSITION_HEADING: &str = "Cursor Position";
const EXPECTED_PATCH_HEADING: &str = "Expected Patch";
const EXPECTED_EXCERPTS_HEADING: &str = "Expected Excerpts";
const REPOSITORY_URL_FIELD: &str = "repository_url";
const REVISION_FIELD: &str = "revision";

#[derive(Debug, Clone)]
pub struct NamedExample {
    pub name: String,
    pub example: Example,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Example {
    pub repository_url: String,
    pub revision: String,
    pub uncommitted_diff: String,
    pub cursor_path: PathBuf,
    pub cursor_position: String,
    pub edit_history: String,
    pub expected_patch: String,
    pub expected_excerpts: ExpectedExcerpts,
}

pub type ActualExcerpt = Excerpt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Excerpt {
    pub path: PathBuf,
    pub text: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExpectedExcerpts {
    pub alternatives: Vec<ExpectedExcerptSet>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExpectedExcerptSet {
    pub excerpts: Vec<ExpectedExcerpt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpectedExcerpt {
    pub path: PathBuf,
    pub text: String,
    pub required_lines: Vec<Line>,
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
                expected_excerpts: ExpectedExcerpts {
                    alternatives: vec![ExpectedExcerptSet::default()],
                },
            },
        };

        let mut text = String::new();
        let mut block_info: CowStr = "".into();

        #[derive(PartialEq)]
        enum Section {
            UncommittedDiff,
            EditHistory,
            CursorPosition,
            ExpectedExcerpts,
            ExpectedPatch,
            Other,
        }

        let mut current_section = Section::Other;

        for event in parser {
            match event {
                Event::Text(line) => {
                    text.push_str(&line);

                    if !named.name.is_empty()
                        && current_section == Section::Other
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
                    let title = mem::take(&mut text);
                    current_section = if title.eq_ignore_ascii_case(UNCOMMITTED_DIFF_HEADING) {
                        Section::UncommittedDiff
                    } else if title.eq_ignore_ascii_case(EDIT_HISTORY_HEADING) {
                        Section::EditHistory
                    } else if title.eq_ignore_ascii_case(CURSOR_POSITION_HEADING) {
                        Section::CursorPosition
                    } else if title.eq_ignore_ascii_case(EXPECTED_PATCH_HEADING) {
                        Section::ExpectedPatch
                    } else if title.eq_ignore_ascii_case(EXPECTED_EXCERPTS_HEADING) {
                        Section::ExpectedExcerpts
                    } else {
                        eprintln!("Warning: Unrecognized section `{title:?}`");
                        Section::Other
                    };
                }
                Event::End(TagEnd::Heading(HeadingLevel::H3)) => {
                    mem::take(&mut text);
                    match current_section {
                        Section::ExpectedExcerpts => {
                            let expected_excerpts = &mut named.example.expected_excerpts;
                            let last_alternative = expected_excerpts.alternatives.last().unwrap();
                            if !last_alternative.excerpts.is_empty() {
                                expected_excerpts.alternatives.push(Default::default());
                            }
                        }
                        _ => {}
                    }
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
                    match current_section {
                        Section::UncommittedDiff => {
                            named.example.uncommitted_diff = mem::take(&mut text);
                        }
                        Section::EditHistory => {
                            named.example.edit_history.push_str(&mem::take(&mut text));
                        }
                        Section::CursorPosition => {
                            named.example.cursor_path = block_info.into();
                            named.example.cursor_position = mem::take(&mut text);
                        }
                        Section::ExpectedExcerpts => {
                            let text = mem::take(&mut text);
                            for excerpt in text.split("\n…\n") {
                                let (mut text, required_lines) = extract_required_lines(&excerpt);
                                if !text.ends_with('\n') {
                                    text.push('\n');
                                }
                                named
                                    .example
                                    .expected_excerpts
                                    .alternatives
                                    .last_mut()
                                    .unwrap()
                                    .excerpts
                                    .push(ExpectedExcerpt {
                                        path: block_info.into(),
                                        text,
                                        required_lines,
                                    });
                            }
                        }
                        Section::ExpectedPatch => {
                            named.example.expected_patch = mem::take(&mut text);
                        }
                        Section::Other => {}
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

    pub async fn setup_worktree(&self) -> Result<PathBuf> {
        let (repo_owner, repo_name) = self.repo_name()?;
        let file_name = self.file_name();

        fs::create_dir_all(&*REPOS_DIR)?;
        fs::create_dir_all(&*WORKTREES_DIR)?;

        let repo_dir = REPOS_DIR.join(repo_owner.as_ref()).join(repo_name.as_ref());
        let repo_lock = lock_repo(&repo_dir).await;

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
        let worktree_path = WORKTREES_DIR.join(&file_name);
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
        drop(repo_lock);

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

    pub async fn cursor_position(
        &self,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<(Entity<Buffer>, Anchor)> {
        let worktree = project.read_with(cx, |project, cx| {
            project.visible_worktrees(cx).next().unwrap()
        })?;
        let cursor_path = RelPath::new(&self.example.cursor_path, PathStyle::Posix)?.into_arc();
        let cursor_buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(
                    ProjectPath {
                        worktree_id: worktree.read(cx).id(),
                        path: cursor_path,
                    },
                    cx,
                )
            })?
            .await?;
        let cursor_offset_within_excerpt = self
            .example
            .cursor_position
            .find(CURSOR_MARKER)
            .ok_or_else(|| anyhow!("missing cursor marker"))?;
        let mut cursor_excerpt = self.example.cursor_position.clone();
        cursor_excerpt.replace_range(
            cursor_offset_within_excerpt..(cursor_offset_within_excerpt + CURSOR_MARKER.len()),
            "",
        );
        let excerpt_offset = cursor_buffer.read_with(cx, |buffer, _cx| {
            let text = buffer.text();

            let mut matches = text.match_indices(&cursor_excerpt);
            let Some((excerpt_offset, _)) = matches.next() else {
                anyhow::bail!(
                    "Cursor excerpt did not exist in buffer.\nExcerpt:\n\n{cursor_excerpt}\nBuffer text:\n{text}\n"
                );
            };
            assert!(matches.next().is_none());

            Ok(excerpt_offset)
        })??;

        let cursor_offset = excerpt_offset + cursor_offset_within_excerpt;
        let cursor_anchor =
            cursor_buffer.read_with(cx, |buffer, _| buffer.anchor_after(cursor_offset))?;
        Ok((cursor_buffer, cursor_anchor))
    }

    #[must_use]
    pub async fn apply_edit_history(
        &self,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<OpenedBuffers<'_>> {
        zeta2::udiff::apply_diff(&self.example.edit_history, project, cx).await
    }
}

fn extract_required_lines(text: &str) -> (String, Vec<Line>) {
    const MARKER: &str = "[ZETA]";
    let mut new_text = String::new();
    let mut required_lines = Vec::new();
    let mut skipped_lines = 0_u32;

    for (row, mut line) in text.split('\n').enumerate() {
        if let Some(marker_column) = line.find(MARKER) {
            let mut strip_column = marker_column;

            while strip_column > 0 {
                let prev_char = line[strip_column - 1..].chars().next().unwrap();
                if prev_char.is_whitespace() || ['/', '#'].contains(&prev_char) {
                    strip_column -= 1;
                } else {
                    break;
                }
            }

            let metadata = &line[marker_column + MARKER.len()..];
            if metadata.contains("required") {
                required_lines.push(Line(row as u32 - skipped_lines));
            }

            if strip_column == 0 {
                skipped_lines += 1;
                continue;
            }

            line = &line[..strip_column];
        }

        new_text.push_str(line);
        new_text.push('\n');
    }

    new_text.pop();

    (new_text, required_lines)
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

        if !self.example.expected_excerpts.alternatives.is_empty() {
            write!(f, "\n## {EXPECTED_EXCERPTS_HEADING}\n\n")?;

            for (i, excerpt_set) in self
                .example
                .expected_excerpts
                .alternatives
                .iter()
                .enumerate()
            {
                write!(f, "\n### Option {}\n\n", i + 1)?;

                for excerpt in &excerpt_set.excerpts {
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
        }

        Ok(())
    }
}

thread_local! {
    static REPO_LOCKS: RefCell<HashMap<PathBuf, Arc<Mutex<()>>>> = RefCell::new(HashMap::default());
}

#[must_use]
pub async fn lock_repo(path: impl AsRef<Path>) -> OwnedMutexGuard<()> {
    REPO_LOCKS
        .with(|cell| {
            cell.borrow_mut()
                .entry(path.as_ref().to_path_buf())
                .or_default()
                .clone()
        })
        .lock_owned()
        .await
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

    #[test]
    fn test_extract_required_lines() {
        let input = indoc! {"
            zero
            one // [ZETA] required
            two
            // [ZETA] something
            three
            four # [ZETA] required
            five
        "};

        let expected_updated_input = indoc! {"
            zero
            one
            two
            three
            four
            five
        "};

        let expected_required_lines = vec![Line(1), Line(4)];

        let (updated_input, required_lines) = extract_required_lines(input);
        assert_eq!(updated_input, expected_updated_input);
        assert_eq!(required_lines, expected_required_lines);
    }
}
