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
}

impl PreprocessorContext {
    pub fn new() -> Result<Self> {
        let macos_keymap = Arc::new(load_keymap("keymaps/default-macos.json")?);
        let linux_keymap = Arc::new(load_keymap("keymaps/default-linux.json")?);
        Ok(Self {
            macos_keymap,
            linux_keymap,
        })
    }

    pub fn find_binding(&self, os: &str, action: &str) -> Option<String> {
        let keymap = match os {
            "macos" => &self.macos_keymap,
            "linux" => &self.linux_keymap,
            _ => return None,
        };

        // Find the binding in reverse order, as the last binding takes precedence.
        keymap.sections().rev().find_map(|section| {
            section.bindings().rev().find_map(|(keystroke, a)| {
                if name_for_action(a.to_string()) == action {
                    Some(keystroke.to_string())
                } else {
                    None
                }
            })
        })
    }
}

/// Removes any configurable options from the stringified action if existing,
/// ensuring that only the actual action name is returned. If the action consists
/// only of a string and nothing else, the string is returned as-is.
///
/// Example:
///
/// This will return the action name unmodified.
///
/// ```
/// let action_as_str = "assistant::Assist";
/// let action_name = name_for_action(action_as_str);
/// assert_eq!(action_name, "assistant::Assist");
/// ```
///
/// This will return the action name with any trailing options removed.
///
///
/// ```
/// let action_as_str = "\"editor::ToggleComments\", {\"advance_downwards\":false}";
/// let action_name = name_for_action(action_as_str);
/// assert_eq!(action_name, "editor::ToggleComments");
/// ```
fn name_for_action(action_as_str: String) -> String {
    action_as_str
        .split(",")
        .next()
        .map(|name| name.trim_matches('"').to_string())
        .unwrap_or(action_as_str)
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
