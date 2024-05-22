use language::BufferSnapshot;
use std::{fmt::Write, ops::Range};
use ui::SharedString;

use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StaticPromptFrontmatter {
    title: String,
    version: String,
    author: String,
    #[serde(default)]
    languages: Vec<String>,
    #[serde(default)]
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

    // pub fn version(&self) -> SharedString {
    //     self.version.clone().into()
    // }

    // pub fn author(&self) -> SharedString {
    //     self.author.clone().into()
    // }

    // pub fn languages(&self) -> Vec<SharedString> {
    //     self.languages
    //         .clone()
    //         .into_iter()
    //         .map(|s| s.into())
    //         .collect()
    // }

    // pub fn dependencies(&self) -> Vec<SharedString> {
    //     self.dependencies
    //         .clone()
    //         .into_iter()
    //         .map(|s| s.into())
    //         .collect()
    // }
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
pub struct StaticPrompt {
    content: String,
    file_name: Option<String>,
}

impl StaticPrompt {
    pub fn new(content: String) -> Self {
        StaticPrompt {
            content,
            file_name: None,
        }
    }

    pub fn title(&self) -> Option<SharedString> {
        self.metadata().map(|m| m.title())
    }

    // pub fn version(&self) -> Option<SharedString> {
    //     self.metadata().map(|m| m.version())
    // }

    // pub fn author(&self) -> Option<SharedString> {
    //     self.metadata().map(|m| m.author())
    // }

    // pub fn languages(&self) -> Vec<SharedString> {
    //     self.metadata().map(|m| m.languages()).unwrap_or_default()
    // }

    // pub fn dependencies(&self) -> Vec<SharedString> {
    //     self.metadata()
    //         .map(|m| m.dependencies())
    //         .unwrap_or_default()
    // }

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

impl StaticPrompt {
    // pub fn update(&mut self, contents: String) -> &mut Self {
    //     self.content = contents;
    //     self
    // }

    /// Sets the file name of the prompt
    pub fn file_name(&mut self, file_name: String) -> &mut Self {
        self.file_name = Some(file_name);
        self
    }

    /// Sets the file name of the prompt based on the title
    // pub fn file_name_from_title(&mut self) -> &mut Self {
    //     if let Some(title) = self.title() {
    //         let file_name = title.to_lowercase().replace(" ", "_");
    //         if !file_name.is_empty() {
    //             self.file_name = Some(file_name);
    //         }
    //     }
    //     self
    // }

    /// Returns the prompt's content
    pub fn content(&self) -> &String {
        &self.content
    }
    fn parse(&self) -> anyhow::Result<(StaticPromptFrontmatter, String)> {
        let matter = Matter::<YAML>::new();
        let result = matter.parse(self.content.as_str());
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

pub fn generate_content_prompt(
    user_prompt: String,
    language_name: Option<&str>,
    buffer: BufferSnapshot,
    range: Range<usize>,
    project_name: Option<String>,
) -> anyhow::Result<String> {
    let mut prompt = String::new();

    let content_type = match language_name {
        None | Some("Markdown" | "Plain Text") => {
            writeln!(prompt, "You are an expert engineer.")?;
            "Text"
        }
        Some(language_name) => {
            writeln!(prompt, "You are an expert {language_name} engineer.")?;
            writeln!(
                prompt,
                "Your answer MUST always and only be valid {}.",
                language_name
            )?;
            "Code"
        }
    };

    if let Some(project_name) = project_name {
        writeln!(
            prompt,
            "You are currently working inside the '{project_name}' project in code editor Zed."
        )?;
    }

    // Include file content.
    for chunk in buffer.text_for_range(0..range.start) {
        prompt.push_str(chunk);
    }

    if range.is_empty() {
        prompt.push_str("<|START|>");
    } else {
        prompt.push_str("<|START|");
    }

    for chunk in buffer.text_for_range(range.clone()) {
        prompt.push_str(chunk);
    }

    if !range.is_empty() {
        prompt.push_str("|END|>");
    }

    for chunk in buffer.text_for_range(range.end..buffer.len()) {
        prompt.push_str(chunk);
    }

    prompt.push('\n');

    if range.is_empty() {
        writeln!(
            prompt,
            "Assume the cursor is located where the `<|START|>` span is."
        )
        .unwrap();
        writeln!(
            prompt,
            "{content_type} can't be replaced, so assume your answer will be inserted at the cursor.",
        )
        .unwrap();
        writeln!(
            prompt,
            "Generate {content_type} based on the users prompt: {user_prompt}",
        )
        .unwrap();
    } else {
        writeln!(prompt, "Modify the user's selected {content_type} based upon the users prompt: '{user_prompt}'").unwrap();
        writeln!(prompt, "You must reply with only the adjusted {content_type} (within the '<|START|' and '|END|>' spans) not the entire file.").unwrap();
        writeln!(
            prompt,
            "Double check that you only return code and not the '<|START|' and '|END|'> spans"
        )
        .unwrap();
    }

    writeln!(prompt, "Never make remarks about the output.").unwrap();
    writeln!(
        prompt,
        "Do not return anything else, except the generated {content_type}."
    )
    .unwrap();

    Ok(prompt)
}
