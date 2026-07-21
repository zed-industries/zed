use gpui::{App, AppContext, Task};
use language::{BufferSnapshot, Language, LanguageRegistry};
use std::sync::Arc;

const SAMPLE_BLOCK_SIZE: usize = 4096;
const MIN_LANGUAGE_DETECTION_CONFIDENCE: f32 = 0.2;
const MIN_LANGUAGE_DETECTION_CONFIDENCE_GAP: f32 = 0.2;
const MAX_CURRENT_LANGUAGE_CONFIDENCE: f32 = 0.3;
const MIN_LANGUAGE_SWITCH_CONFIDENCE_GAP: f32 = 0.5;

fn language_registry_key(language: betlang::Language) -> Option<&'static str> {
    let language_name = match language {
        betlang::Language::Asm => "Assembly",
        betlang::Language::C => "C",
        betlang::Language::Clojure => "Clojure",
        betlang::Language::CMake => "CMake",
        betlang::Language::Cobol => "COBOL",
        betlang::Language::Cpp => "C++",
        betlang::Language::Cs => "CSharp",
        betlang::Language::Css => "CSS",
        betlang::Language::Dart => "Dart",
        betlang::Language::Dockerfile => "Dockerfile",
        betlang::Language::Elixir => "Elixir",
        betlang::Language::Erlang => "Erlang",
        betlang::Language::Gemfile => "Ruby",
        betlang::Language::Gemspec => "Ruby",
        betlang::Language::Go => "Go",
        betlang::Language::Gradle => "Groovy",
        betlang::Language::Groovy => "Groovy",
        betlang::Language::Haskell => "Haskell",
        betlang::Language::Html => "HTML",
        betlang::Language::Ini => "ini",
        betlang::Language::Java => "Java",
        betlang::Language::JavaScript => "JavaScript",
        betlang::Language::Json => "JSON",
        betlang::Language::Julia => "Julia",
        betlang::Language::Kotlin => "Kotlin",
        betlang::Language::Lua => "Lua",
        betlang::Language::Markdown => "Markdown",
        betlang::Language::ObjectiveC => "Objective-C",
        betlang::Language::Ocaml => "OCaml",
        betlang::Language::Perl => "Perl",
        betlang::Language::Php => "PHP",
        betlang::Language::Powershell => "PowerShell",
        betlang::Language::Python => "Python",
        betlang::Language::R => "R",
        betlang::Language::Ruby => "Ruby",
        betlang::Language::Rust => "Rust",
        betlang::Language::Scala => "Scala",
        betlang::Language::Shell => "Shell Script",
        betlang::Language::Sql => "SQL",
        betlang::Language::Swift => "Swift",
        betlang::Language::Toml => "TOML",
        betlang::Language::TypeScript => "TypeScript",
        betlang::Language::Verilog => "SystemVerilog",
        betlang::Language::Xml => "XML",
        betlang::Language::Yaml => "YAML",
        _ => return None,
    };
    Some(language_name)
}

pub fn detect_language(
    buffer: BufferSnapshot,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut App,
) -> Task<Option<Arc<Language>>> {
    let source = extract_sample(&buffer);
    let current_language_name = buffer.language().map(|language| language.name());
    cx.background_spawn(async move {
        let detection = betlang::detect(source);
        let (mut pending_languages, mut confirmed_languages) = (Vec::new(), Vec::new());
        // As in VS Code, retain only high-confidence candidate groups followed by a clear confidence gap.
        for (score, language) in detection.top_languages() {
            if pending_languages.last().is_some_and(|(previous_score, _)| {
                *previous_score - score >= MIN_LANGUAGE_DETECTION_CONFIDENCE_GAP
            }) {
                confirmed_languages.append(&mut pending_languages);
            }
            if score < MIN_LANGUAGE_DETECTION_CONFIDENCE {
                break;
            }
            pending_languages.push((score, language));
        }
        let detected_language =
            confirmed_languages
                .into_iter()
                .find_map(|(score, model_language)| {
                    language_registry
                        .available_language_for_name(language_registry_key(model_language)?)
                        .map(|language| (score, language))
                });
        let current_language_score = current_language_name.as_ref().and_then(|current_name| {
            detection
                .top_languages()
                .find_map(|(score, model_language)| {
                    language_registry_key(model_language)
                        .is_some_and(|name| name == current_name.as_ref())
                        .then_some(score)
                })
        });
        let Some((score, language)) = detected_language else {
            return None;
        };
        if current_language_name.is_some_and(|current_name| current_name == language.name()) {
            return None;
        }
        // Prefer the current language unless another candidate has a clear confidence advantage.
        if current_language_score.is_some_and(|current_score| {
            score
                <= current_score.min(MAX_CURRENT_LANGUAGE_CONFIDENCE)
                    + MIN_LANGUAGE_SWITCH_CONFIDENCE_GAP
        }) {
            return None;
        }
        language_registry
            .load_language(&language)
            .await
            .ok()
            .and_then(|language| language.ok())
    })
}

fn extract_sample(buffer: &BufferSnapshot) -> Vec<u8> {
    let source_length = buffer.len();
    let ranges = if source_length <= SAMPLE_BLOCK_SIZE * 2 {
        vec![0..source_length]
    } else {
        vec![
            0..SAMPLE_BLOCK_SIZE,
            source_length - SAMPLE_BLOCK_SIZE..source_length,
        ]
    };

    ranges
        .into_iter()
        .flat_map(|range| buffer.bytes_in_range(range))
        .flat_map(|chunk| chunk.iter().copied())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn detects_rust_source() {
        let detection = betlang::detect("fn main() { println!(\"hello\"); }");
        assert_eq!(detection.language(), Some(betlang::Language::Rust));
    }
}
