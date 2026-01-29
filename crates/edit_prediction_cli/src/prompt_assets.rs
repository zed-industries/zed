use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "src/prompts"]
struct EmbeddedPrompts;

const PROMPTS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/prompts");

pub fn get_prompt(name: &str) -> Cow<'static, str> {
    let filesystem_path = Path::new(PROMPTS_DIR).join(name);
    if let Ok(contents) = std::fs::read_to_string(&filesystem_path) {
        return Cow::Owned(contents);
    }

    match EmbeddedPrompts::get(name) {
        Some(file) => match file.data {
            Cow::Borrowed(bytes) => {
                Cow::Borrowed(std::str::from_utf8(bytes).expect("prompt file is not valid UTF-8"))
            }
            Cow::Owned(bytes) => {
                Cow::Owned(String::from_utf8(bytes).expect("prompt file is not valid UTF-8"))
            }
        },
        None => panic!("prompt file not found: {name}"),
    }
}
