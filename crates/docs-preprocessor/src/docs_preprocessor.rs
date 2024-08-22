use anyhow::Result;
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;
use settings::{KeymapFile, SettingsAssets};
use util::asset_str;

/// A preprocessor for the Zed documentation.
pub struct ZedDocsPreprocessor {
    macos_keymap: KeymapFile,
    linux_keymap: KeymapFile,
}

impl ZedDocsPreprocessor {
    pub fn new() -> Result<Self> {
        let macos_keymap = Self::load_keymap("keymaps/default-macos.json")?;
        let linux_keymap = Self::load_keymap("keymaps/default-linux.json")?;

        Ok(Self {
            macos_keymap,
            linux_keymap,
        })
    }

    fn load_keymap(asset_path: &str) -> Result<KeymapFile> {
        let content = asset_str::<SettingsAssets>(asset_path);
        KeymapFile::parse(content.as_ref())
    }

    fn find_binding(&self, keymap: &KeymapFile, action: &str) -> Option<String> {
        keymap.blocks().iter().find_map(|block| {
            block.bindings().iter().find_map(|(keystroke, a)| {
                if a.to_string() == action {
                    Some(keystroke.to_string())
                } else {
                    None
                }
            })
        })
    }

    fn process_keybindings(&self, content: &str) -> String {
        let keybinding_regex = Regex::new(r"\{\{#keybinding key: (.+?)\}\}").unwrap();

        keybinding_regex
            .replace_all(content, |caps: &regex::Captures| {
                let action = &caps[1];
                let macos_binding = self
                    .find_binding(&self.macos_keymap, action)
                    .unwrap_or_default();
                let linux_binding = self
                    .find_binding(&self.linux_keymap, action)
                    .unwrap_or_default();

                format!("<kbd class=\"keybinding\">{macos_binding}|{linux_binding}</kbd>")
            })
            .to_string()
    }
}

impl Preprocessor for ZedDocsPreprocessor {
    fn name(&self) -> &str {
        "zed-docs-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                chapter.content = self.process_keybindings(&chapter.content);
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
