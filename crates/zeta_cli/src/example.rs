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
            Some(b"json") => Ok(Self {
                name: path.file_name().unwrap_or_default().display().to_string(),
                example: serde_json::from_reader(file)?,
            }),
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
                anyhow::bail!("md todo");
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
        // use pulldown_cmark::{Event, Parser};

        // let parser = Parser::new(input);

        // for event in parser {
        //     match event {
        //         Event::Start(tag) => {}
        //         Event::End(tag_end) => {}
        //         Event::Text(cow_str) => {}
        //         Event::Code(cow_str) => {
        //             dbg!(cow_str);
        //         }
        //         Event::InlineMath(cow_str) => {}
        //         Event::DisplayMath(cow_str) => {}
        //         Event::Html(cow_str) => {}
        //         Event::InlineHtml(cow_str) => {}
        //         Event::FootnoteReference(cow_str) => {}
        //         Event::SoftBreak => {}
        //         Event::HardBreak => {}
        //         Event::Rule => {}
        //         Event::TaskListMarker(_) => {}
        //     }
        // }

        todo!();
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
        write!(f, "respository_url = {}\n", self.example.repository_url)?;
        write!(f, "commit = {}\n\n", self.example.commit)?;
        write!(f, "## Edit history\n\n")?;

        if !self.example.edit_history.is_empty() {
            write!(f, "`````diff\n")?;
            for item in &self.example.edit_history {
                write!(f, "{item}")?;
            }
            write!(f, "`````\n")?;
        }

        if !self.example.expected_hunks.is_empty() {
            write!(f, "\n## Expected Hunks\n\n`````diff\n")?;
            for hunk in &self.example.expected_hunks {
                write!(f, "{hunk}")?;
            }
            write!(f, "`````\n")?;
        }

        if !self.example.expected_patch.is_empty() {
            write!(
                f,
                "\n## Expected Patch\n\n`````diff\n{}`````\n",
                self.example.expected_patch
            )?;
        }

        if !self.example.expected_excerpts.is_empty() {
            write!(f, "\n## Expected Excerpts\n\n")?;

            for excerpt in &self.example.expected_excerpts {
                write!(
                    f,
                    "`````{}path={}\n{}`````\n\n",
                    excerpt
                        .path
                        .extension()
                        .map(|ext| format!("{} ", ext.to_string_lossy().to_string()))
                        .unwrap_or_default(),
                    excerpt.path.display(),
                    excerpt.text
                )?;
            }
        }

        Ok(())
    }
}
