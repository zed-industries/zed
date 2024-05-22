#![allow(unused, dead_code)]
use editor::actions::SelectToStartOfParagraph;
use language::BufferSnapshot;
use std::{fmt::Write, ops::Range};
use ui::SharedString;

use fs::{CreateOptions, Fs};
use futures::StreamExt;
use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use util::{paths::PROMPTS_DIR, ResultExt};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StaticPromptFrontmatter {
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

impl StaticPromptFrontmatter {
    pub fn title(&self) -> SharedString {
        self.title.clone().into()
    }

    pub fn version(&self) -> SharedString {
        self.version.clone().into()
    }

    pub fn author(&self) -> SharedString {
        self.author.clone().into()
    }

    pub fn languages(&self) -> Vec<SharedString> {
        self.languages
            .clone()
            .into_iter()
            .map(|s| s.into())
            .collect()
    }

    pub fn dependencies(&self) -> Vec<SharedString> {
        self.dependencies
            .clone()
            .into_iter()
            .map(|s| s.into())
            .collect()
    }
}

/// A statuc prompt that can be loaded into the prompt library
/// from Markdown with a frontmatter header
///
/// Examples:
///
/// ### Globally available prompt
///
/// ```markdown
/// ---
/// title: Foo
/// version: 1.0
/// author: Jane Kim <jane@kim.com
/// languages: ["*"]
/// dependencies: []
/// ---
///
/// Foo and bar are terms used in programming to describe generic concepts.
/// ```
///
/// ### Language-specific prompt
///
/// ```markdown
/// ---
/// title: UI with GPUI
/// version: 1.0
/// author: Nate Butler <iamnbutler@gmail.com>
/// languages: ["rust"]
/// dependencies: ["gpui"]
/// ---
///
/// When building a UI with GPUI, ensure you...
/// ```
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

    pub fn title(&self) -> Option<SharedString> {
        self.metadata().map(|m| m.title())
    }

    pub fn version(&self) -> Option<SharedString> {
        self.metadata().map(|m| m.version())
    }

    pub fn author(&self) -> Option<SharedString> {
        self.metadata().map(|m| m.author())
    }

    pub fn languages(&self) -> Vec<SharedString> {
        self.metadata().map(|m| m.languages()).unwrap_or_default()
    }

    pub fn dependencies(&self) -> Vec<SharedString> {
        self.metadata()
            .map(|m| m.dependencies())
            .unwrap_or_default()
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

    pub fn file_name_from_title(&mut self) -> &mut Self {
        if let Some(title) = self.title() {
            let file_name = title.to_lowercase().replace(" ", "_");
            if !file_name.is_empty() {
                self.file_name = Some(file_name);
            }
        }
        self
    }

    /// Returns the prompt's content
    pub fn content(&self) -> &String {
        &self.content
    }
    fn parse(&self) -> anyhow::Result<(StaticPromptFrontmatter, String)> {
        let matter = Matter::<YAML>::new();
        let result = matter.parse(&self.content.as_str());
        match result.data {
            Some(data) => {
                let front_matter: StaticPromptFrontmatter = data.deserialize()?;
                let body = result.content;
                Ok((front_matter, body))
            }
            None => Err(anyhow::anyhow!("Failed to parse frontmatter")),
        }
    }

    pub fn metadata(&self) -> Option<StaticPromptFrontmatter> {
        self.parse().ok().map(|(front_matter, _)| front_matter)
    }
}
