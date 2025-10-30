use std::{
    fmt::{self, Display},
    fs::File,
    io::{Read, Write},
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

const EDIT_HISTORY_HEADING: &str = "Edit History";
const EXPECTED_HUNKS_HEADING: &str = "Expected Hunks";
const EXPECTED_PATCH_HEADING: &str = "Expected Patch";
const EXPECTED_EXCERPTS_HEADING: &str = "Expected Excerpts";

pub struct NamedExample {
    name: String,
    example: Example,
}

#[derive(Serialize, Deserialize)]
pub struct Example {
    repository_url: String,
    commit: String,
    edit_history: Vec<String>,
    expected_hunks: Vec<String>,
    expected_patch: String,
    expected_excerpts: Vec<ExpectedExcerpt>,
}

#[derive(Serialize, Deserialize)]
pub struct ExpectedExcerpt {
    path: PathBuf,
    text: String,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ExampleFormat {
    Json,
    Toml,
    Md,
}

impl NamedExample {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let mut file = File::open(path)?;
        let ext = path.extension();

        match ext.map(|s| s.as_bytes()) {
            Some(b"json") => {
                let mut content = Vec::new();
                file.read_to_end(&mut content)?;
                Ok(Self {
                    name: path.file_name().unwrap_or_default().display().to_string(),
                    example: serde_json::from_slice(&content)?,
                })
            }
            Some(b"toml") => {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                Ok(Self {
                    name: path.file_name().unwrap_or_default().display().to_string(),
                    example: toml::from_str(&content)?,
                })
            }
            Some(b"md") => {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                Self::parse_md(&content)
            }
            Some(_) => {
                anyhow::bail!("Unrecognized example extension: {}", ext.unwrap().display());
            }
            None => {
                anyhow::bail!(
                    "Failed to determine example type since the file does not have an extension."
                );
            }
        }
    }

    pub fn parse_md(input: &str) -> Result<Self> {
        use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};

        let parser = Parser::new(input);

        let mut name = String::new();
        let mut repository_url = String::new();
        let mut commit = String::new();
        let mut edit_history = Vec::new();
        let mut expected_hunks = Vec::new();
        let mut expected_patch = String::new();
        let mut expected_excerpts = Vec::new();

        let mut current_heading_level: Option<HeadingLevel> = None;
        let mut current_heading_text = String::new();
        let mut current_section = String::new();
        let mut in_code_block = false;
        let mut current_code_block = String::new();
        let mut current_code_info = String::new();

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    current_heading_level = Some(level);
                    current_heading_text.clear();
                }
                Event::End(TagEnd::Heading(_)) => {
                    let heading_text = current_heading_text.trim();
                    if let Some(HeadingLevel::H1) = current_heading_level {
                        if !name.is_empty() {
                            anyhow::bail!(
                                "Found multiple H1 headings. There should only be one with the name of the example."
                            );
                        }
                        name = heading_text.to_string();
                    } else if let Some(HeadingLevel::H2) = current_heading_level {
                        current_section = heading_text.to_string();
                    }
                    current_heading_level = None;
                }
                Event::Start(Tag::CodeBlock(kind)) => {
                    in_code_block = true;
                    current_code_block.clear();
                    current_code_info = match kind {
                        CodeBlockKind::Fenced(info) => info.to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;

                    match current_section.as_str() {
                        EDIT_HISTORY_HEADING => {
                            edit_history.push(current_code_block.clone());
                        }
                        EXPECTED_HUNKS_HEADING => {
                            expected_hunks.push(current_code_block.clone());
                        }
                        EXPECTED_PATCH_HEADING => {
                            expected_patch = current_code_block.clone();
                        }
                        EXPECTED_EXCERPTS_HEADING => {
                            if let Some(path_start) = current_code_info.find("path=") {
                                let path_str = &current_code_info[path_start + 5..];
                                let path = PathBuf::from(path_str.trim());
                                expected_excerpts.push(ExpectedExcerpt {
                                    path,
                                    text: current_code_block.clone(),
                                });
                            }
                        }
                        _ => {}
                    }
                }
                Event::Text(text) => {
                    if let Some(_) = current_heading_level {
                        current_heading_text.push_str(&text);
                    } else if in_code_block {
                        current_code_block.push_str(&text);
                    } else if current_section.is_empty()
                        && let Some(eq_pos) = text.find('=')
                    {
                        let key = text[..eq_pos].trim();
                        let value = text[eq_pos + 1..].trim();
                        match key {
                            "repository_url" => repository_url = dbg!(value.to_string()),
                            "commit" => commit = value.to_string(),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if name.is_empty() {
            anyhow::bail!("Missing required H1 heading for example name");
        }

        if repository_url.is_empty() {
            anyhow::bail!("Missing required field: repository_url");
        }

        if commit.is_empty() {
            anyhow::bail!("Missing required field: commit");
        }

        Ok(Self {
            name,
            example: Example {
                repository_url,
                commit,
                edit_history,
                expected_hunks,
                expected_patch,
                expected_excerpts,
            },
        })
    }

    pub fn write(&self, format: ExampleFormat, mut out: impl Write) -> Result<()> {
        match format {
            ExampleFormat::Json => Ok(serde_json::to_writer(out, &self.example)?),
            ExampleFormat::Toml => {
                Ok(out.write_all(toml::to_string_pretty(&self.example)?.as_bytes())?)
            }
            ExampleFormat::Md => Ok(write!(out, "{}", self)?),
        }
    }
}

impl Display for NamedExample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "# {}\n\n", self.name)?;
        write!(f, "repository_url = {}\n", self.example.repository_url)?;
        write!(f, "commit = {}\n\n", self.example.commit)?;
        write!(f, "## {EDIT_HISTORY_HEADING}\n\n")?;

        if !self.example.edit_history.is_empty() {
            write!(f, "`````diff\n")?;
            for item in &self.example.edit_history {
                write!(f, "{item}")?;
            }
            write!(f, "`````\n")?;
        }

        if !self.example.expected_hunks.is_empty() {
            write!(f, "\n## {EXPECTED_HUNKS_HEADING}\n\n`````diff\n")?;
            for hunk in &self.example.expected_hunks {
                write!(f, "{hunk}")?;
            }
            write!(f, "`````\n")?;
        }

        if !self.example.expected_patch.is_empty() {
            write!(
                f,
                "\n## {EXPECTED_PATCH_HEADING}\n\n`````diff\n{}`````\n",
                self.example.expected_patch
            )?;
        }

        if !self.example.expected_excerpts.is_empty() {
            write!(f, "\n## {EXPECTED_EXCERPTS_HEADING}\n\n")?;

            for excerpt in &self.example.expected_excerpts {
                write!(
                    f,
                    "`````{}path={}\n{}`````\n\n",
                    excerpt
                        .path
                        .extension()
                        .map(|ext| format!("{} ", ext.to_string_lossy()))
                        .unwrap_or_default(),
                    excerpt.path.display(),
                    excerpt.text
                )?;
            }
        }

        Ok(())
    }
}
