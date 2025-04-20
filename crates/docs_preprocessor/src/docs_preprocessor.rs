use anyhow::Result;
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext as MdBookContext};
use settings::KeymapFile;
use std::sync::Arc;
use util::asset_str;

mod templates;

use templates::{ActionTemplate, KeybindingTemplate, Template};

pub struct PreprocessorContext {
    macos_keymap: Arc<KeymapFile>,
    linux_keymap: Arc<KeymapFile>,
    windows_keymap: Arc<KeymapFile>,
}

impl PreprocessorContext {
    pub fn new() -> Result<Self> {
        let macos_keymap = Arc::new(load_keymap("keymaps/default-macos.json")?);
        let linux_keymap = Arc::new(load_keymap("keymaps/default-linux.json")?);
        let windows_keymap = Arc::new(load_keymap("keymaps/default-windows.json")?);
        Ok(Self {
            macos_keymap,
            linux_keymap,
            windows_keymap,
        })
    }

    pub fn find_binding(&self, os: &str, action: &str) -> Option<String> {
        let keymap = match os {
            "macos" => &self.macos_keymap,
            "linux" => &self.linux_keymap,
            "windows" => &self.windows_keymap,
            _ => return None,
        };

        // Find the binding in reverse order, as the last binding takes precedence.
        keymap.sections().rev().find_map(|section| {
            section.bindings().rev().find_map(|(keystroke, a)| {
                if a.to_string() == action {
                    Some(keystroke.to_string())
                } else {
                    None
                }
            })
        })
    }
}

fn load_keymap(asset_path: &str) -> Result<KeymapFile> {
    let content = asset_str::<settings::SettingsAssets>(asset_path);
    KeymapFile::parse(content.as_ref())
}

pub struct ZedDocsPreprocessor {
    context: PreprocessorContext,
    templates: Vec<Box<dyn Template>>,
}

impl ZedDocsPreprocessor {
    pub fn new() -> Result<Self> {
        let context = PreprocessorContext::new()?;
        let templates: Vec<Box<dyn Template>> = vec![
            Box::new(KeybindingTemplate::new()),
            Box::new(ActionTemplate::new()),
        ];
        Ok(Self { context, templates })
    }

    fn process_content(&self, content: &str) -> String {
        let mut processed = content.to_string();
        for template in &self.templates {
            processed = template.process(&self.context, &processed);
        }
        processed
    }
}

impl Preprocessor for ZedDocsPreprocessor {
    fn name(&self) -> &str {
        "zed-docs-preprocessor"
    }

    fn run(&self, _ctx: &MdBookContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                chapter.content = self.process_content(&chapter.content);
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
