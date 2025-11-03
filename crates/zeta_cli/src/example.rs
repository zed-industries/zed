use std::{
    borrow::Cow,
    env,
    fmt::{self, Display},
    fs,
    io::Write,
    mem,
    path::{Path, PathBuf},
};

use anyhow::{Context as _, Result};
use clap::ValueEnum;
use gpui::http_client::Url;
use pulldown_cmark::CowStr;
use serde::{Deserialize, Serialize};

const CURSOR_POSITION_HEADING: &str = "Cursor Position";
const EDIT_HISTORY_HEADING: &str = "Edit History";
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
    pub cursor_path: PathBuf,
    pub cursor_position: String,
    pub edit_history: Vec<String>,
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
                name: path.file_name().unwrap_or_default().display().to_string(),
                example: serde_json::from_str(&content)?,
            }),
            Some("toml") => Ok(Self {
                name: path.file_name().unwrap_or_default().display().to_string(),
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
                cursor_path: PathBuf::new(),
                cursor_position: String::new(),
                edit_history: Vec::new(),
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
                    if current_section.eq_ignore_ascii_case(EDIT_HISTORY_HEADING) {
                        named.example.edit_history.push(mem::take(&mut text));
                    } else if current_section.eq_ignore_ascii_case(CURSOR_POSITION_HEADING) {
                        let path = PathBuf::from(block_info.trim());
                        named.example.cursor_path = path;
                        named.example.cursor_position = mem::take(&mut text);
                    } else if current_section.eq_ignore_ascii_case(EXPECTED_PATCH_HEADING) {
                        named.example.expected_patch = mem::take(&mut text);
                    } else if current_section.eq_ignore_ascii_case(EXPECTED_EXCERPTS_HEADING) {
                        let path = PathBuf::from(block_info.trim());
                        named.example.expected_excerpts.push(ExpectedExcerpt {
                            path,
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
        let worktrees_dir = env::current_dir()?.join("target").join("zeta-worktrees");
        let repos_dir = env::current_dir()?.join("target").join("zeta-repos");
        fs::create_dir_all(&repos_dir)?;
        fs::create_dir_all(&worktrees_dir)?;

        let (repo_owner, repo_name) = self.repo_name()?;

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

        run_git(
            &repo_dir,
            &["fetch", "--depth", "1", "origin", &self.example.revision],
        )
        .await?;

        let worktree_path = worktrees_dir.join(&self.name);

        if worktree_path.is_dir() {
            run_git(&worktree_path, &["clean", "--force", "-d"]).await?;
            run_git(&worktree_path, &["reset", "--hard", "HEAD"]).await?;
            run_git(&worktree_path, &["checkout", &self.example.revision]).await?;
        } else {
            let worktree_path_string = worktree_path.to_string_lossy();
            run_git(
                &repo_dir,
                &[
                    "worktree",
                    "add",
                    "-f",
                    &worktree_path_string,
                    &self.example.revision,
                ],
            )
            .await?;
        }

        Ok(worktree_path)
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

        write!(
            f,
            "## {CURSOR_POSITION_HEADING}\n\n`````{}\n{}`````\n",
            self.example.cursor_path.display(),
            self.example.cursor_position
        )?;
        write!(f, "## {EDIT_HISTORY_HEADING}\n\n")?;

        if !self.example.edit_history.is_empty() {
            write!(f, "`````diff\n")?;
            for item in &self.example.edit_history {
                write!(f, "{item}")?;
            }
            write!(f, "`````\n")?;
        }

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
