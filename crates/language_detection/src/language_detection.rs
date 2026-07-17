use gpui::{App, AppContext, Task};
use language::{BufferSnapshot, Language, LanguageRegistry};
use std::sync::Arc;

const SAMPLE_BLOCK_SIZE: usize = 4096;

fn language_registry_key(language: betlang::Language) -> &'static str {
    match language {
        betlang::Language::ObjectiveC => "Objective-C",
        betlang::Language::Shell => "Shell Script",
        _ => language.slug(),
    }
}

pub fn detect_language(
    buffer: BufferSnapshot,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut App,
) -> Task<Option<(Arc<Language>, f32)>> {
    let source = extract_sample(&buffer);
    cx.background_spawn(async move {
        let detection = betlang::detect(source);
        let (confidence, language) = detection.top_languages().next()?;
        let language = language_registry
            .language_for_name_or_extension(language_registry_key(language))
            .await
            .ok()?;

        Some((language, confidence))
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
