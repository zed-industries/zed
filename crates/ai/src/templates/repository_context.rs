use std::{ops::Range, path::PathBuf};

use gpui::{AsyncAppContext, ModelHandle};
use language::{Anchor, Buffer};

pub struct PromptCodeSnippet {
    path: Option<PathBuf>,
    language_name: Option<String>,
    content: String,
}

impl PromptCodeSnippet {
    pub fn new(buffer: ModelHandle<Buffer>, range: Range<Anchor>, cx: &AsyncAppContext) -> Self {
        let (content, language_name, file_path) = buffer.read_with(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            let content = snapshot.text_for_range(range.clone()).collect::<String>();

            let language_name = buffer
                .language()
                .and_then(|language| Some(language.name().to_string()));

            let file_path = buffer
                .file()
                .and_then(|file| Some(file.path().to_path_buf()));

            (content, language_name, file_path)
        });

        PromptCodeSnippet {
            path: file_path,
            language_name,
            content,
        }
    }
}

impl ToString for PromptCodeSnippet {
    fn to_string(&self) -> String {
        let path = self
            .path
            .as_ref()
            .and_then(|path| Some(path.to_string_lossy().to_string()))
            .unwrap_or("".to_string());
        let language_name = self.language_name.clone().unwrap_or("".to_string());
        let content = self.content.clone();

        format!("The below code snippet may be relevant from file: {path}\n```{language_name}\n{content}\n```")
    }
}
