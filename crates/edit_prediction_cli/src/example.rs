use crate::{
    PredictionProvider, PromptFormat,
    metrics::ClassificationMetrics,
    paths::{REPOS_DIR, WORKTREES_DIR},
};
use anyhow::{Context as _, Result};
use edit_prediction::udiff::OpenedBuffers;
use gpui::Entity;
use http_client::Url;
use language::{Anchor, Buffer};
use project::Project;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{
    borrow::Cow,
    io::{Read, Write},
    mem,
    path::{Path, PathBuf},
};
use zeta_prompt::RelatedFile;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Example {
    #[serde(default)]
    pub name: String,
    pub repository_url: String,
    pub revision: String,
    pub uncommitted_diff: String,
    pub cursor_path: Arc<Path>,
    pub cursor_position: String,
    pub edit_history: String,
    pub expected_patch: String,

    /// The full content of the file where an edit is being predicted, and the
    /// actual cursor offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer: Option<ExampleBuffer>,

    /// The context retrieved for the prediction. This requires the worktree to
    /// be loaded and the language server to be started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ExampleContext>,

    /// The input and expected output from the edit prediction model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<ExamplePrompt>,

    /// The actual predictions from the model.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub predictions: Vec<ExamplePrediction>,

    /// The scores, for how well the actual predictions match the expected
    /// predictions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub score: Vec<ExampleScore>,

    /// The application state used to process this example.
    #[serde(skip)]
    pub state: Option<ExampleState>,
}

#[derive(Clone, Debug)]
pub struct ExampleState {
    pub project: Entity<Project>,
    pub buffer: Entity<Buffer>,
    pub cursor_position: Anchor,
    pub _open_buffers: OpenedBuffers,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExampleContext {
    pub files: Arc<[RelatedFile]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExampleBuffer {
    pub content: String,
    pub cursor_row: u32,
    pub cursor_column: u32,
    pub cursor_offset: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExamplePrompt {
    pub input: String,
    pub expected_output: String,
    pub format: PromptFormat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExamplePrediction {
    pub actual_patch: String,
    pub actual_output: String,
    pub provider: PredictionProvider,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExampleScore {
    pub delta_chr_f: f32,
    pub line_match: ClassificationMetrics,
}

impl Example {
    fn repo_name(&self) -> Result<(Cow<'_, str>, Cow<'_, str>)> {
        // git@github.com:owner/repo.git
        if self.repository_url.contains('@') {
            let (owner, repo) = self
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
            let url = Url::parse(&self.repository_url)?;
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

    pub fn worktree_path(&self) -> PathBuf {
        WORKTREES_DIR
            .join(&self.name)
            .join(self.repo_name().unwrap().1.as_ref())
    }

    pub fn repo_path(&self) -> PathBuf {
        let (repo_owner, repo_name) = self.repo_name().expect("failed to get repo name");
        REPOS_DIR.join(repo_owner.as_ref()).join(repo_name.as_ref())
    }
}

pub fn read_examples(inputs: &[PathBuf]) -> Vec<Example> {
    let mut examples = Vec::new();

    let stdin_path: PathBuf = PathBuf::from("-");

    let inputs = if inputs.is_empty() {
        &[stdin_path]
    } else {
        inputs
    };

    for path in inputs {
        let is_stdin = path.as_path() == Path::new("-");
        let content = if is_stdin {
            let mut buffer = String::new();
            std::io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read from stdin");
            buffer
        } else {
            std::fs::read_to_string(path)
                .unwrap_or_else(|_| panic!("Failed to read path: {:?}", &path))
        };
        let filename = path.file_stem().unwrap().to_string_lossy().to_string();
        let ext = if !is_stdin {
            path.extension()
                .map(|ext| ext.to_string_lossy().to_string())
                .unwrap_or_else(|| panic!("{} should have an extension", path.display()))
        } else {
            "jsonl".to_string()
        };

        match ext.as_ref() {
            "json" => {
                let mut example =
                    serde_json::from_str::<Example>(&content).unwrap_or_else(|error| {
                        panic!("Failed to parse example file: {}\n{error}", path.display())
                    });
                if example.name.is_empty() {
                    example.name = filename;
                }
                examples.push(example);
            }
            "jsonl" => examples.extend(
                content
                    .lines()
                    .enumerate()
                    .map(|(line_ix, line)| {
                        let mut example =
                            serde_json::from_str::<Example>(line).unwrap_or_else(|_| {
                                panic!(
                                    "Failed to parse example on {}:{}",
                                    path.display(),
                                    line_ix + 1
                                )
                            });
                        if example.name.is_empty() {
                            example.name = format!("{filename}-{line_ix}")
                        }
                        example
                    })
                    .collect::<Vec<Example>>(),
            ),
            "md" => {
                examples.push(parse_markdown_example(filename, &content).unwrap());
            }
            ext => {
                panic!("{} has invalid example extension `{ext}`", path.display())
            }
        }
    }
    examples
}

pub fn write_examples(examples: &[Example], output_path: Option<&PathBuf>) {
    let mut content = String::new();
    for example in examples {
        let line = serde_json::to_string(example).unwrap();
        content.push_str(&line);
        content.push('\n');
    }
    if let Some(output_path) = output_path {
        std::fs::write(output_path, content).expect("Failed to write examples");
    } else {
        std::io::stdout().write_all(&content.as_bytes()).unwrap();
    }
}

fn parse_markdown_example(id: String, input: &str) -> Result<Example> {
    use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Parser, Tag, TagEnd};

    const UNCOMMITTED_DIFF_HEADING: &str = "Uncommitted Diff";
    const EDIT_HISTORY_HEADING: &str = "Edit History";
    const CURSOR_POSITION_HEADING: &str = "Cursor Position";
    const EXPECTED_PATCH_HEADING: &str = "Expected Patch";
    const EXPECTED_CONTEXT_HEADING: &str = "Expected Context";
    const REPOSITORY_URL_FIELD: &str = "repository_url";
    const REVISION_FIELD: &str = "revision";

    let parser = Parser::new(input);

    let mut example = Example {
        name: id,
        repository_url: String::new(),
        revision: String::new(),
        uncommitted_diff: String::new(),
        cursor_path: PathBuf::new().into(),
        cursor_position: String::new(),
        edit_history: String::new(),
        expected_patch: String::new(),
        buffer: None,
        context: None,
        prompt: None,
        predictions: Vec::new(),
        score: Vec::new(),
        state: None,
    };

    let mut name = String::new();
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

                if let Some((field, value)) = line.split_once('=') {
                    match field.trim() {
                        REPOSITORY_URL_FIELD => {
                            example.repository_url = value.trim().to_string();
                        }
                        REVISION_FIELD => {
                            example.revision = value.trim().to_string();
                        }
                        _ => {}
                    }
                }
            }
            Event::End(TagEnd::Heading(HeadingLevel::H1)) => {
                if !name.is_empty() {
                    anyhow::bail!(
                        "Found multiple H1 headings. There should only be one with the name of the example."
                    );
                }
                name = mem::take(&mut text);
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
                } else if title.eq_ignore_ascii_case(EXPECTED_CONTEXT_HEADING) {
                    Section::ExpectedExcerpts
                } else {
                    Section::Other
                };
            }
            Event::End(TagEnd::Heading(HeadingLevel::H3)) => {
                mem::take(&mut text);
            }
            Event::End(TagEnd::Heading(HeadingLevel::H4)) => {
                mem::take(&mut text);
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
                        example.uncommitted_diff = mem::take(&mut text);
                    }
                    Section::EditHistory => {
                        example.edit_history.push_str(&mem::take(&mut text));
                    }
                    Section::CursorPosition => {
                        example.cursor_path = Path::new(block_info).into();
                        example.cursor_position = mem::take(&mut text);
                    }
                    Section::ExpectedExcerpts => {
                        mem::take(&mut text);
                    }
                    Section::ExpectedPatch => {
                        example.expected_patch = mem::take(&mut text);
                    }
                    Section::Other => {}
                }
            }
            _ => {}
        }
    }
    if example.cursor_path.as_ref() == Path::new("") || example.cursor_position.is_empty() {
        anyhow::bail!("Missing cursor position codeblock");
    }

    Ok(example)
}
