use std::{
    fmt::{self, Display},
    io::Write,
    mem,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::ValueEnum;
use pulldown_cmark::CowStr;
use serde::{Deserialize, Serialize};

const CURSOR_POSITION_HEADING: &str = "Cursor Position";
const EDIT_HISTORY_HEADING: &str = "Edit History";
const EXPECTED_PATCH_HEADING: &str = "Expected Patch";
const EXPECTED_EXCERPTS_HEADING: &str = "Expected Excerpts";
const REPOSITORY_URL_FIELD: &str = "repository_url";
const REVISION_FIELD: &str = "revision";

pub struct NamedExample {
    name: String,
    example: Example,
}

#[derive(Serialize, Deserialize)]
pub struct Example {
    repository_url: String,
    commit: String,
    cursor_path: PathBuf,
    cursor_position: String,
    edit_history: Vec<String>,
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
        let content = std::fs::read_to_string(path)?;
        let ext = path.extension();

        match ext.map(|s| s.as_bytes()) {
            Some(b"json") => Ok(Self {
                name: path.file_name().unwrap_or_default().display().to_string(),
                example: serde_json::from_str(&content)?,
            }),
            Some(b"toml") => Ok(Self {
                name: path.file_name().unwrap_or_default().display().to_string(),
                example: toml::from_str(&content)?,
            }),
            Some(b"md") => Self::parse_md(&content),
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

        let mut named = NamedExample {
            name: String::new(),
            example: Example {
                repository_url: String::new(),
                commit: String::new(),
                cursor_path: PathBuf::new(),
                cursor_position: String::new(),
                edit_history: Vec::new(),
                expected_patch: String::new(),
                expected_excerpts: Vec::new(),
            },
        };

        let mut text = String::new();
        let mut current_section = String::new();
        let mut block_info: CowStr = "".into();

        for event in parser {
            match event {
                Event::Text(line) => {
                    text.push_str(&line);

                    if !named.name.is_empty()
                        && current_section.is_empty()
                        // in h1 section
                        && let Some((field, value)) = line.split_once('=')
                    {
                        match field {
                            REPOSITORY_URL_FIELD => {
                                named.example.repository_url = value.to_string();
                            }
                            REVISION_FIELD => {
                                named.example.commit = value.to_string();
                            }
                            _ => {
                                eprintln!("Warning: Unrecognized field `{field}`");
                            }
                        }
                    }
                }
                Event::End(TagEnd::Heading(HeadingLevel::H1)) => {
                    if !named.name.is_empty() {
                        anyhow::bail!(
                            "Found multiple H1 headings. There should only be one with the name of the example."
                        );
                    }
                    named.name = mem::take(&mut text);
                }
                Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
                    current_section = mem::take(&mut text);
                }
                Event::End(TagEnd::Heading(level)) => {
                    anyhow::bail!("Unexpected heading level: {level}");
                }
                Event::Start(Tag::CodeBlock(kind)) => {
                    match kind {
                        CodeBlockKind::Fenced(info) => {
                            block_info = info;
                        }
                        CodeBlockKind::Indented => {
                            anyhow::bail!("Unexpected indented codeblock");
                        }
                    };
                }
                Event::Start(_) => {
                    text.clear();
                    block_info = "".into();
                }
                Event::End(TagEnd::CodeBlock) => {
                    if current_section.eq_ignore_ascii_case(EDIT_HISTORY_HEADING) {
                        named.example.edit_history.push(mem::take(&mut text));
                    } else if current_section.eq_ignore_ascii_case(CURSOR_POSITION_HEADING) {
                        let path = PathBuf::from(block_info.trim());
                        named.example.cursor_path = path;
                        named.example.cursor_position = mem::take(&mut text);
                    } else if current_section.eq_ignore_ascii_case(EXPECTED_PATCH_HEADING) {
                        named.example.expected_patch = mem::take(&mut text);
                    } else if current_section.eq_ignore_ascii_case(EXPECTED_EXCERPTS_HEADING) {
                        let path = PathBuf::from(block_info.trim());
                        named.example.expected_excerpts.push(ExpectedExcerpt {
                            path,
                            text: mem::take(&mut text),
                        });
                    } else {
                        eprintln!("Warning: Unrecognized section `{current_section:?}`")
                    }
                }
                _ => {}
            }
        }

        if named.example.cursor_path.as_path() == Path::new("")
            || named.example.cursor_position.is_empty()
        {
            anyhow::bail!("Missing cursor position codeblock");
        }

        Ok(named)
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
        write!(
            f,
            "{REPOSITORY_URL_FIELD} = {}\n",
            self.example.repository_url
        )?;
        write!(f, "{REVISION_FIELD} = {}\n\n", self.example.commit)?;

        write!(
            f,
            "## {CURSOR_POSITION_HEADING}\n\n`````{}\n{}`````\n",
            self.example.cursor_path.display(),
            self.example.cursor_position
        )?;
        write!(f, "## {EDIT_HISTORY_HEADING}\n\n")?;

        if !self.example.edit_history.is_empty() {
            write!(f, "`````diff\n")?;
            for item in &self.example.edit_history {
                write!(f, "{item}")?;
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
                    "`````{}{}\n{}`````\n\n",
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
