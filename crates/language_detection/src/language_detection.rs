use anyhow::Result;
use gpui::{App, Global};
use language::{BufferSnapshot, Language, LanguageRegistry};
use magika::ContentType;
use std::sync::Arc;

fn content_type_to_language_name(content_type: ContentType) -> Option<&'static str> {
    match content_type {
        ContentType::C => Some("c"),
        ContentType::Clojure => Some("clojure"),
        ContentType::Cpp => Some("cpp"),
        ContentType::Cs => Some("csharp"),
        ContentType::Css => Some("css"),
        ContentType::Dart => Some("dart"),
        ContentType::Diff => Some("diff"),
        ContentType::Dockerfile => Some("docker"),
        ContentType::Elixir => Some("elixir"),
        ContentType::Erlang => Some("erlang"),
        ContentType::Go => Some("go"),
        ContentType::Groovy => Some("groovy"),
        ContentType::Haskell => Some("haskell"),
        ContentType::Html => Some("html"),
        ContentType::Java => Some("java"),
        ContentType::Javascript => Some("javascript"),
        ContentType::Json => Some("json"),
        ContentType::Julia => Some("julia"),
        ContentType::Kotlin => Some("kotlin"),
        ContentType::Lua => Some("lua"),
        ContentType::Makefile => Some("makefile"),
        ContentType::Markdown => Some("markdown"),
        ContentType::Ocaml => Some("ocaml"),
        ContentType::Php => Some("php"),
        ContentType::Powershell => Some("powershell"),
        ContentType::Proto => Some("proto"),
        ContentType::Python => Some("python"),
        ContentType::R => Some("r"),
        ContentType::Rst => Some("rst"),
        ContentType::Ruby => Some("ruby"),
        ContentType::Rust => Some("rust"),
        ContentType::Scala => Some("scala"),
        ContentType::Shell => Some("sh"),
        ContentType::Sql => Some("sql"),
        ContentType::Swift => Some("swift"),
        ContentType::Toml => Some("toml"),
        ContentType::Txt => Some("plaintext"),
        ContentType::Typescript => Some("typescript"),
        ContentType::Vue => Some("vue"),
        ContentType::Xml => Some("xml"),
        ContentType::Yaml => Some("yaml"),
        ContentType::Yara => Some("yara"),
        ContentType::Zig => Some("zig"),
        _ => None,
    }
}

// pub fn init(cx: &mut App) {
//     cx.set_global(LanguageDetector {});
// }

// impl Global for LanguageDetector {}

pub struct LanguageDetector {}

impl LanguageDetector {
    pub async fn detect_language(
        buffer: BufferSnapshot,
        language_registry: Arc<LanguageRegistry>,
    ) -> Result<Arc<Language>> {
        let text_sample = extract_text_sample(buffer);

        let session = magika::Session::new();
        let result = match session {
            Ok(mut session) => session
                .identify_content_sync(text_sample.as_bytes())
                .map_err(|e| anyhow::Error::new(e)),
            Err(err) => {
                log::error!("Failed to create magika session: {}", err);
                Err(anyhow::Error::msg("Failed to create magika session"))
            }
        };

        match result {
            Ok(file_type) => {
                let content_type = file_type.content_type().unwrap();
                let Some(language_name) = content_type_to_language_name(content_type) else {
                    return Err(anyhow::Error::msg("Failed to detect language"));
                };
                language_registry.language_for_name(language_name).await
            }
            Err(err) => {
                log::error!("Failed to identify content type: {}", err);
                Err(err)
            }
        }
    }
}

fn extract_text_sample(buffer_handle: BufferSnapshot) -> String {
    const MAX_BYTES: usize = 8192; // ~3 screens of text

    let total_len = buffer_handle.len();
    let sample_len = total_len.min(MAX_BYTES);

    buffer_handle
        .text_for_range(0..sample_len)
        .collect::<String>()
}
