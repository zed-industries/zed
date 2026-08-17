use super::{common::RegexTrimmer, Language};
use crate::pipeline::{FnWrapper, Pipeline};
use lindera::tokenizer::{Tokenizer, TokenizerConfig};
use lindera_core::viterbi::Mode;

#[derive(Clone)]
pub struct Japanese {
    tokenizer: Tokenizer,
}

impl Japanese {
    pub fn new() -> Self {
        let config = TokenizerConfig {
            mode: Mode::Decompose(Default::default()),
            ..Default::default()
        };
        Self::with_config(config)
    }

    pub fn with_config(config: TokenizerConfig) -> Self {
        // NB: unwrap() is okay since the errors are only related to user-supplied dictionaries.
        let tokenizer = Tokenizer::with_config(config).unwrap();
        Self { tokenizer }
    }
}

impl Language for Japanese {
    fn name(&self) -> String {
        "Japanese".into()
    }
    fn code(&self) -> String {
        "ja".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        self.tokenizer
            .tokenize(text)
            .unwrap()
            .into_iter()
            .filter_map(|tok| match tok.detail.get(0).map(|d| d.as_str()) {
                Some("助詞") | Some("助動詞") | Some("記号") | Some("UNK") => None,
                _ => Some(tok.text.to_string()),
            })
            .collect()
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(RegexTrimmer::new("trimmer-ja", WORD_CHARS)),
                Box::new(FnWrapper("stemmer-ja".into(), stemmer)),
            ],
        }
    }
}

const WORD_CHARS: &str = r"0-9A-Za-z\p{Hiragana}\p{Katakana}\p{Unified_Ideograph}";

fn stemmer(token: String) -> Option<String> {
    Some(token)
}

#[cfg(test)]
mod tests {
    use crate::pipeline::PipelineFn;
    use super::*;

    #[test]
    fn test_trimmer() {
        let trimmer = RegexTrimmer::new("trimmer-ja".into(), WORD_CHARS);
        assert_eq!(
            trimmer.filter("  こんにちは、世界！".to_string()),
            Some("こんにちは、世界".to_string())
        );
    }
}
