use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::fs;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
};
use util::serde::default_true;

use crate::thread::{EvalThread, EvalThreadMetadata, ThreadContext};

mod file_search;

pub fn all() -> Vec<Rc<dyn EvalThread>> {
    let mut threads: Vec<Rc<dyn EvalThread>> = vec![Rc::new(file_search::Thread)];

    for example_path in list_all_examples().unwrap() {
        threads.push(Rc::new(ExampleThread::load(&example_path).unwrap()));
    }

    threads
}

struct ExampleThread {
    metadata: EvalThreadMetadata,
    prompt: String,
    diff_criteria: String,
    thread_criteria: String,
}

impl ExampleThread {
    pub fn load(dir_path: &Path) -> Result<Self> {
        let name = Self::name_from_path(dir_path);
        let base_path = dir_path.join("base.toml");
        let prompt_path = dir_path.join("prompt.md");
        let diff_criteria_path = dir_path.join("diff_criteria.md");
        let thread_criteria_path = dir_path.join("thread_criteria.md");
        let thread_criteria = if thread_criteria_path.exists() {
            Some(fs::read_to_string(thread_criteria_path.clone())?)
        } else {
            None
        };

        let base: ExampleBase = toml::from_str(&fs::read_to_string(&base_path)?)?;

        let language_server = if base.require_lsp {
            Some(crate::thread::LanguageServer {
                file_extension: base
                    .language_extension
                    .expect("Language extension is required when require_lsp = true"),
                allow_preexisting_diagnostics: base.allow_preexisting_diagnostics,
            })
        } else {
            None
        };

        let metadata = EvalThreadMetadata {
            name,
            url: base.url,
            revision: base.revision,
            language_server,
            max_assertions: None,
        };

        Ok(ExampleThread {
            metadata,
            prompt: fs::read_to_string(prompt_path.clone())?,
            thread_criteria: thread_criteria.unwrap_or("".to_string()),
            diff_criteria: fs::read_to_string(diff_criteria_path.clone())?,
        })
    }

    pub fn name_from_path(path: &Path) -> String {
        path.file_name().unwrap().to_string_lossy().to_string()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExampleBase {
    pub url: String,
    pub revision: String,
    pub language_extension: Option<String>,
    pub insert_id: Option<String>,
    #[serde(default = "default_true")]
    pub require_lsp: bool,
    #[serde(default)]
    pub allow_preexisting_diagnostics: bool,
}

impl ExampleBase {
    pub fn repo_name(&self) -> String {
        self.url
            .split('/')
            .next_back()
            .unwrap_or(&"")
            .trim_end_matches(".git")
            .into()
    }
}

#[async_trait(?Send)]
impl EvalThread for ExampleThread {
    fn meta(&self) -> EvalThreadMetadata {
        self.metadata.clone()
    }

    async fn conversation(&self, cx: &mut ThreadContext) -> Result<()> {
        cx.push_user_message(&self.prompt);
        let _ = cx.run_to_end().await;
        Ok(())
    }

    fn diff_criteria(&self) -> String {
        self.diff_criteria.clone()
    }

    fn thread_criteria(&self) -> String {
        self.thread_criteria.clone()
    }
}

pub const EXAMPLES_DIR: &str = "./crates/eval/examples";

fn list_all_examples() -> Result<Vec<PathBuf>> {
    let path = std::fs::canonicalize(EXAMPLES_DIR).unwrap();
    let entries = std::fs::read_dir(path).unwrap();
    let mut result_paths = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            result_paths.push(path);
        }
    }
    Ok(result_paths)
}
