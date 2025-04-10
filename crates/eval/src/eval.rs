use anyhow::Result;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};
mod agent;

#[derive(Debug, Deserialize)]
pub struct ExampleBase {
    pub path: PathBuf,
    pub revision: String,
}

#[derive(Debug)]
pub struct Example {
    pub base: ExampleBase,

    /// Content of the prompt.md file
    pub prompt: String,

    /// Content of the rubric.md file
    pub rubric: String,
}

impl Example {
    /// Load an example from a directory containing base.toml, prompt.md, and rubric.md
    pub fn load_from_directory<P: AsRef<Path>>(dir_path: P) -> Result<Self> {
        let base_path = dir_path.as_ref().join("base.toml");
        let prompt_path = dir_path.as_ref().join("prompt.md");
        let rubric_path = dir_path.as_ref().join("rubric.md");

        let mut base: ExampleBase = toml::from_str(&fs::read_to_string(&base_path)?)?;
        base.path = base.path.canonicalize()?;

        Ok(Example {
            base,
            prompt: fs::read_to_string(prompt_path)?,
            rubric: fs::read_to_string(rubric_path)?,
        })
    }

    /// Set up the example by checking out the specified Git revision
    pub fn setup(&self) -> Result<()> {
        use std::process::Command;

        // Check if the directory exists
        let path = Path::new(&self.base.path);
        anyhow::ensure!(path.exists(), "Path does not exist: {:?}", self.base.path);

        // Change to the project directory and checkout the specified revision
        let output = Command::new("git")
            .current_dir(&self.base.path)
            .arg("checkout")
            .arg(&self.base.revision)
            .output()?;
        anyhow::ensure!(
            output.status.success(),
            "Failed to checkout revision {}: {}",
            self.base.revision,
            String::from_utf8_lossy(&output.stderr),
        );

        Ok(())
    }
}

fn main() -> Result<()> {
    let example =
        Example::load_from_directory("./crates/eval/examples/find_and_replace_diff_card")?;
    example.setup()?;
    Ok(())
}
