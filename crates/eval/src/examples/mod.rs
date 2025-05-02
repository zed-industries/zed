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

use crate::example::{Example, ExampleContext, ExampleMetadata, JudgeAssertion};

mod add_arg_to_trait_method;
mod code_block_citations;
mod file_search;
mod planets;

pub fn all(examples_dir: &Path) -> Vec<Rc<dyn Example>> {
    let mut threads: Vec<Rc<dyn Example>> = vec![
        Rc::new(file_search::FileSearchExample),
        Rc::new(add_arg_to_trait_method::AddArgToTraitMethod),
        Rc::new(code_block_citations::CodeBlockCitations),
        Rc::new(planets::Planets),
    ];

    for example_path in list_declarative_examples(examples_dir).unwrap() {
        threads.push(Rc::new(DeclarativeExample::load(&example_path).unwrap()));
    }

    threads
}

struct DeclarativeExample {
    metadata: ExampleMetadata,
    prompt: String,
    diff_assertions: Vec<JudgeAssertion>,
    thread_assertions: Vec<JudgeAssertion>,
}

impl DeclarativeExample {
    pub fn load(example_path: &Path) -> Result<Self> {
        let name = Self::name_from_path(example_path);
        let base: ExampleToml = toml::from_str(&fs::read_to_string(&example_path)?)?;

        let language_server = if base.require_lsp {
            Some(crate::example::LanguageServer {
                file_extension: base
                    .language_extension
                    .expect("Language extension is required when require_lsp = true"),
                allow_preexisting_diagnostics: base.allow_preexisting_diagnostics,
            })
        } else {
            None
        };

        let metadata = ExampleMetadata {
            name,
            url: base.url,
            revision: base.revision,
            language_server,
            max_assertions: None,
        };

        Ok(DeclarativeExample {
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
impl Example for DeclarativeExample {
    fn meta(&self) -> ExampleMetadata {
        self.metadata.clone()
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
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

fn list_declarative_examples(examples_dir: &Path) -> Result<Vec<PathBuf>> {
    let path = std::fs::canonicalize(examples_dir).unwrap();
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
