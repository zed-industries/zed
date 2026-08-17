// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{
    fs,
    path::{Path, PathBuf},
};

use lazy_static::lazy_static;
use log::{error, trace};
use regex::Regex;

lazy_static! {
    static ref NORMALIZE_NAME: Regex = Regex::new(r"[-_.]+")
        .expect("Error generating RegEx for poetry project name normalization");
}

#[derive(Debug)]
pub struct PyProjectToml {
    pub name: String,
}

impl PyProjectToml {
    fn new(name: String, file: PathBuf) -> Self {
        // Source from https://github.com/python-poetry/poetry-core/blob/a2c068227358984d835c9684de723b046bdcd67a/src/poetry/core/_vendor/packaging/utils.py#L46-L51
        // normalized_name = re.sub(r"[-_.]+", "-", name).lower()
        let normalized_name = NORMALIZE_NAME
            .replace_all(&name.to_lowercase(), "-")
            .chars()
            .collect::<String>();

        trace!("Poetry project: {:?} with name {:?}", file, normalized_name);
        PyProjectToml {
            name: normalized_name,
        }
    }
    pub fn find(path: &Path) -> Option<Self> {
        trace!("Finding poetry file in {:?}", path);
        parse(&path.join("pyproject.toml"))
    }
}

fn parse(file: &Path) -> Option<PyProjectToml> {
    trace!("Parsing poetry file: {:?}", file);
    let contents = fs::read_to_string(file).ok()?;
    parse_contents(&contents, file)
}

fn parse_contents(contents: &str, file: &Path) -> Option<PyProjectToml> {
    match toml::from_str::<toml::Value>(contents) {
        Ok(value) => {
            let mut name = None;
            if let Some(tool) = value.get("tool") {
                if let Some(poetry) = tool.get("poetry") {
                    if let Some(name_value) = poetry.get("name") {
                        name = name_value.as_str().map(|s| s.to_string());
                    }
                }
            }

            match name {
                Some(name) => Some(PyProjectToml::new(name, file.into())),
                None => {
                    trace!(
                        "Poetry project name not found in {:?}, trying the new format",
                        file
                    );
                    let mut name = None;
                    if let Some(project) = value.get("project") {
                        if let Some(name_value) = project.get("name") {
                            name = name_value.as_str().map(|s| s.to_string());
                        }
                    }
                    name.map(|name| PyProjectToml::new(name, file.into()))
                }
            }
        }
        Err(e) => {
            error!("Error parsing toml file: {:?}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn extract_name_from_pyproject_toml() {
        let cfg = r#"
[tool.poetry]
name = "poetry-demo"
version = "0.1.0"
description = ""
authors = ["User Name <bogus.user@some.email.com>"]
readme = "README.md"

[tool.poetry.dependencies]
python = "^3.12"


[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
"#;
        assert_eq!(
            parse_contents(cfg, Path::new("pyproject.toml"))
                .unwrap()
                .name,
            "poetry-demo"
        );
    }

    #[test]
    fn extract_normalized_name_from_pyproject_toml() {
        let cfg = r#"
[tool.poetry]
name = "poetry_.demo"
version = "0.1.0"
description = ""
authors = ["User Name <bogus.user@some.email.com>"]
readme = "README.md"

[tool.poetry.dependencies]
python = "^3.12"


[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
"#;
        assert_eq!(
            parse_contents(cfg, Path::new("pyproject.toml"))
                .unwrap()
                .name,
            "poetry-demo"
        );
    }
}
