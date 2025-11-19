use anyhow::Result;
use gpui::{App, AppContext, Task};
use language::{BufferSnapshot, Language, LanguageRegistry};
use magika::ContentType;
use parking_lot::Mutex;
use std::sync::Arc;

fn content_type_to_language_name(content_type: ContentType) -> Option<&'static str> {
    match content_type {
        ContentType::C => Some("c"),
        ContentType::Clojure => Some("clojure"),
        ContentType::Cpp => Some("c++"),
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

pub struct LanguageDetector {
    session: Option<Arc<Mutex<magika::Session>>>,
}

impl LanguageDetector {
    pub fn new() -> Self {
        let session = magika::Session::new().ok();

        Self {
            session: session.map(|s| Arc::new(Mutex::new(s))),
        }
    }

    pub fn detect_language(
        &self,
        buffer: BufferSnapshot,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Task<Result<Arc<Language>>> {
        let session = self.session.clone();

        cx.background_spawn(async move {
            let Some(session) = session else {
                return Err(anyhow::Error::msg("No session found"));
            };

            let text_sample = extract_text_sample(buffer);

            let result = session
                .lock()
                .identify_content_sync(text_sample.as_bytes())
                .map_err(|e| anyhow::Error::new(e));

            match result {
                Ok(file_type) => {
                    let content_type = file_type.content_type().unwrap();
                    let Some(language_name) = content_type_to_language_name(content_type) else {
                        return Err(anyhow::Error::msg("Failed to detect language"));
                    };

                    println!("contenttype: {}", content_type.info().label);
                    println!("languagename: {}", language_name);

                    language_registry.language_for_name(language_name).await
                }
                Err(err) => {
                    log::error!("Failed to identify content type: {}", err);
                    Err(anyhow::Error::msg("test"))
                }
            }
        })
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
