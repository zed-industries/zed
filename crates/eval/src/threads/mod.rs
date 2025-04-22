use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
};
use util::serde::default_true;

use crate::thread::{EvalThread, EvalThreadMetadata, JudgeAssertion, ThreadContext};

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
    diff_assertions: Vec<JudgeAssertion>,
    thread_assertions: Vec<JudgeAssertion>,
}

impl ExampleThread {
    pub fn load(example_path: &Path) -> Result<Self> {
        let name = Self::name_from_path(example_path);
        let base: ExampleToml = toml::from_str(&fs::read_to_string(&example_path)?)?;

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
            prompt: base.prompt,
            thread_assertions: base
                .thread_assertions
                .into_iter()
                .map(|(id, description)| JudgeAssertion { id, description })
                .collect(),
            diff_assertions: base
                .diff_assertions
                .into_iter()
                .map(|(id, description)| JudgeAssertion { id, description })
                .collect(),
        })
    }

    pub fn name_from_path(path: &Path) -> String {
        path.file_stem().unwrap().to_string_lossy().to_string()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExampleToml {
    pub url: String,
    pub revision: String,
    pub language_extension: Option<String>,
    pub insert_id: Option<String>,
    #[serde(default = "default_true")]
    pub require_lsp: bool,
    #[serde(default)]
    pub allow_preexisting_diagnostics: bool,
    pub prompt: String,
    #[serde(default)]
    pub diff_assertions: BTreeMap<String, String>,
    #[serde(default)]
    pub thread_assertions: BTreeMap<String, String>,
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

    fn diff_assertions(&self) -> Vec<JudgeAssertion> {
        self.diff_assertions.clone()
    }

    fn thread_assertions(&self) -> Vec<JudgeAssertion> {
        self.thread_assertions.clone()
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
        if path.extension() == Some("toml".as_ref()) {
            result_paths.push(path);
        }
    }
    Ok(result_paths)
}
