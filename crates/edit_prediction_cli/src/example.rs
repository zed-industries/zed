use crate::PredictionProvider;
use crate::paths::WORKTREES_DIR;
use anyhow::{Context as _, Result};
use collections::HashMap;
use edit_prediction::example_spec::ExampleSpec;
use edit_prediction::udiff::OpenedBuffers;
use gpui::Entity;
use http_client::Url;
use language::{Anchor, Buffer};
use project::Project;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::VecDeque,
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
};
use zeta_prompt::RelatedFile;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Example {
    #[serde(flatten)]
    pub spec: ExampleSpec,

    /// The full content of the file where an edit is being predicted, and the
    /// actual cursor offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_inputs: Option<ExamplePromptInputs>,

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
pub struct ExamplePromptInputs {
    pub content: String,
    pub cursor_row: u32,
    pub cursor_column: u32,
    pub cursor_offset: usize,
    pub edit_history: Vec<Arc<zeta_prompt::Event>>,
    pub related_files: Option<Vec<RelatedFile>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExamplePrompt {
    pub input: String,
    pub expected_output: String,
    pub provider: PredictionProvider,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExamplePrediction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_patch: Option<String>,
    pub actual_output: String,
    pub provider: PredictionProvider,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExampleScore {
    pub delta_chr_f: f32,
}

impl Example {
    pub fn repo_name(&self) -> Result<RepoName<'_>> {
        // git@github.com:owner/repo.git
        if self.spec.repository_url.contains('@') {
            let (owner, repo) = self
                .spec
                .repository_url
                .split_once(':')
                .context("expected : in git url")?
                .1
                .split_once('/')
                .context("expected / in git url")?;
            Ok(RepoName {
                owner: Cow::Borrowed(owner),
                name: Cow::Borrowed(repo.trim_end_matches(".git")),
            })
        // http://github.com/owner/repo.git
        } else {
            let url = Url::parse(&self.spec.repository_url)?;
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

            Ok(RepoName {
                owner: Cow::Owned(owner),
                name: Cow::Owned(repo),
            })
        }
    }
}

pub struct RepoName<'a> {
    pub owner: Cow<'a, str>,
    pub name: Cow<'a, str>,
}

impl RepoName<'_> {
    pub fn worktree_path(&self) -> PathBuf {
        WORKTREES_DIR
            .join(self.owner.as_ref())
            .join(self.name.as_ref())
    }
}

pub fn read_example_files(inputs: &[PathBuf]) -> Vec<Example> {
    let mut examples = Vec::new();

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
                if example.spec.name.is_empty() {
                    example.spec.name = filename;
                }
                examples.push(example);
            }
            "jsonl" => examples.extend(
                content
                    .lines()
                    .enumerate()
                    .map(|(line_ix, line)| {
                        let mut example =
                            serde_json::from_str::<Example>(line).unwrap_or_else(|error| {
                                panic!(
                                    "Failed to parse example on {}:{}\n{error}",
                                    path.display(),
                                    line_ix + 1
                                )
                            });
                        if example.spec.name.is_empty() {
                            example.spec.name = format!("{filename}-{line_ix}")
                        }
                        example
                    })
                    .collect::<Vec<Example>>(),
            ),
            "md" => {
                let mut example = parse_markdown_example(&content).unwrap();
                if example.spec.name.is_empty() {
                    example.spec.name = filename;
                }
                examples.push(example);
            }
            ext => {
                panic!("{} has invalid example extension `{ext}`", path.display())
            }
        }
    }

    examples
}

pub fn sort_examples_by_repo_and_rev(examples: &mut [Example]) {
    examples.sort_by(|a, b| {
        a.spec
            .repository_url
            .cmp(&b.spec.repository_url)
            .then(b.spec.revision.cmp(&a.spec.revision))
    });
}

pub fn group_examples_by_repo(examples: Vec<Example>) -> VecDeque<Vec<Example>> {
    let mut examples_by_repo = HashMap::default();
    for example in examples {
        examples_by_repo
            .entry(example.spec.repository_url.clone())
            .or_insert_with(Vec::new)
            .push(example);
    }
    examples_by_repo.into_values().collect()
}

fn parse_markdown_example(input: &str) -> Result<Example> {
    let spec = ExampleSpec::from_markdown(input)?;
    Ok(Example {
        spec,
        prompt_inputs: None,
        prompt: None,
        predictions: Vec::new(),
        score: Vec::new(),
        state: None,
    })
}
