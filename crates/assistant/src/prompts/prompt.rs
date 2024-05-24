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
            title: "Untitled Prompt".to_string(),
            version: "1.0".to_string(),
            author: "No Author".to_string(),
            languages: vec!["*".to_string()],
            dependencies: vec![],
        }
    }
}

/// A static prompt that can be loaded into the prompt library
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
    #[serde(skip)]
    metadata: StaticPromptFrontmatter,
    content: String,
    file_name: Option<String>,
}

impl StaticPrompt {
    pub fn new(content: String, file_name: Option<String>) -> Self {
        let matter = Matter::<YAML>::new();
        let result = matter.parse(&content);

        let metadata = result
            .data
            .map_or_else(
                || Err(anyhow::anyhow!("Failed to parse frontmatter")),
                |data| {
                    let front_matter: StaticPromptFrontmatter = data.deserialize()?;
                    Ok(front_matter)
                },
            )
            .unwrap_or_else(|e| {
                if let Some(file_name) = &file_name {
                    log::error!("Failed to parse frontmatter for {}: {}", file_name, e);
                } else {
                    log::error!("Failed to parse frontmatter: {}", e);
                }
                StaticPromptFrontmatter::default()
            });

        StaticPrompt {
            content,
            file_name,
            metadata,
        }
    }
}

impl StaticPrompt {
    /// Sets the file name of the prompt
    pub fn _file_name(&mut self, file_name: String) -> &mut Self {
        self.file_name = Some(file_name);
        self
    }

    /// Returns the prompt's content
    pub fn content(&self) -> &String {
        &self.content
    }

    /// Returns the prompt's metadata
    pub fn _metadata(&self) -> &StaticPromptFrontmatter {
        &self.metadata
    }

    /// Returns the prompt's title
    pub fn title(&self) -> SharedString {
        self.metadata.title.clone().into()
    }

    pub fn body(&self) -> String {
        let matter = Matter::<YAML>::new();
        let result = matter.parse(self.content.as_str());
        result.content.clone()
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
