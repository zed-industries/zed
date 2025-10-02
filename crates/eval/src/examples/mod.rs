use agent_settings::AgentProfileId;
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
mod comment_translation;
mod file_change_notification;
mod file_search;
mod grep_params_escapement;
mod overwrite_file;
mod planets;

pub fn all(examples_dir: &Path) -> Vec<Rc<dyn Example>> {
    let mut threads: Vec<Rc<dyn Example>> = vec![
        Rc::new(file_search::FileSearchExample),
        Rc::new(add_arg_to_trait_method::AddArgToTraitMethod),
        Rc::new(code_block_citations::CodeBlockCitations),
        Rc::new(planets::Planets),
        Rc::new(comment_translation::CommentTranslation),
        Rc::new(overwrite_file::FileOverwriteExample),
        Rc::new(file_change_notification::FileChangeNotificationExample),
        Rc::new(grep_params_escapement::GrepParamsEscapementExample),
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
        let example_dir = example_path.parent().unwrap();

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

        let profile_id = if let Some(profile_name) = base.profile_name {
            AgentProfileId(profile_name.into())
        } else {
            AgentProfileId::default()
        };

        let existing_thread_json = if let Some(path) = base.existing_thread_path {
            let content = fs::read_to_string(example_dir.join(&path))
                .unwrap_or_else(|_| panic!("Failed to read existing thread file: {}", path));
            Some(content)
        } else {
            None
        };

        let metadata = ExampleMetadata {
            name,
            url: base.url,
            revision: base.revision,
            language_server,
            max_assertions: None,
            profile_id,
            existing_thread_json,
            max_turns: base.max_turns,
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
        path.file_stem().unwrap().to_string_lossy().into_owned()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExampleToml {
    pub url: String,
    pub revision: String,
    pub language_extension: Option<String>,
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    pub insert_id: Option<String>,
    #[serde(default = "default_true")]
    pub require_lsp: bool,
    #[serde(default)]
    pub allow_preexisting_diagnostics: bool,
    pub prompt: String,
    #[serde(default)]
    pub profile_name: Option<String>,
    #[serde(default)]
    pub diff_assertions: BTreeMap<String, String>,
    #[serde(default)]
    pub thread_assertions: BTreeMap<String, String>,
    #[serde(default)]
    pub existing_thread_path: Option<String>,
    #[serde(default)]
    pub max_turns: Option<u32>,
}

#[async_trait(?Send)]
impl Example for DeclarativeExample {
    fn meta(&self) -> ExampleMetadata {
        self.metadata.clone()
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        cx.push_user_message(&self.prompt);
        let max_turns = self.metadata.max_turns.unwrap_or(1000);
        let _ = cx.run_turns(max_turns).await;
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
