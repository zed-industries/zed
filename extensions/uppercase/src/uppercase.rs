use serde_json::json;
use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, settings::LspSettings, Result, Buffer};

struct UppercaseExtension {
}

impl UppercaseExtension {
}

impl zed::Extension for UppercaseExtension {
    fn new() -> Self {
        Self {
        }
    }

    fn run_editor_action(&self, name: String, buffer: Buffer) {
        dbg!("hi!");
    }
}

zed::register_extension!(UppercaseExtension);
