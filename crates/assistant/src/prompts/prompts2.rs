#![allow(unused, dead_code)]
use editor::actions::SelectToStartOfParagraph;
use language::BufferSnapshot;
use std::{fmt::Write, ops::Range};

use fs::{CreateOptions, Fs};
use futures::StreamExt;
use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use util::{paths::PROMPTS_DIR, ResultExt};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct StaticPromptFrontmatter {
    title: String,
    version: String,
    author: String,
    languages: Vec<String>,
    dependencies: Vec<String>,
}

impl Default for StaticPromptFrontmatter {
    fn default() -> Self {
        Self {
            title: "New Prompt".to_string(),
            version: "1.0".to_string(),
            author: "No Author".to_string(),
            languages: vec!["*".to_string()],
            dependencies: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StaticPrompt2 {
    content: String,
    file_name: Option<String>,
}

impl StaticPrompt2 {
    pub fn new(content: String) -> Self {
        StaticPrompt2 {
            content,
            file_name: None,
        }
    }

    // pub fn load(fs: Arc<Fs>, file_name: String) -> anyhow::Result<Self> {
    //     todo!()
    // }

    // pub fn save(&self, fs: Arc<Fs>) -> anyhow::Result<()> {
    //     todo!()
    // }

    // pub fn rename(&self, new_file_name: String, fs: Arc<Fs>) -> anyhow::Result<()> {
    //     todo!()
    // }
}

impl StaticPrompt2 {
    pub fn update(&mut self, contents: String) -> &mut Self {
        self.content = contents;
        self
    }

    /// Sets the file name of the prompt
    pub fn file_name(&mut self, file_name: String) -> &mut Self {
        self.file_name = Some(file_name);
        self
    }

    /// Sets the file name based on the title of the prompt
    pub fn file_name_from_title(&mut self) -> &mut Self {
        let title = self.metadata().title;
        let file_name = title.to_lowercase().replace(" ", "_");
        self.file_name = Some(file_name);
        self
    }

    /// Returns the prompt's content
    pub fn content(&self) -> &String {
        &self.content
    }

    fn parse(&self) -> (StaticPromptFrontmatter, String) {
        let matter = Matter::<YAML>::new();
        let result = matter.parse(&self.content.as_str());

        let front_matter: StaticPromptFrontmatter =
            result.data.unwrap().deserialize().unwrap_or_default();

        let body = result.content;
        (front_matter, body)
    }

    pub fn metadata(&self) -> StaticPromptFrontmatter {
        self.parse().0
    }
}
