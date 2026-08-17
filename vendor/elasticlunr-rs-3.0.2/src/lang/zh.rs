use super::{common::RegexTrimmer, Language};
use crate::pipeline::{FnWrapper, Pipeline};

#[derive(Clone)]
pub struct Chinese {
    jieba: jieba_rs::Jieba,
}

impl Chinese {
    pub fn new() -> Self {
        Self {
            jieba: jieba_rs::Jieba::new(),
        }
    }
}

impl Language for Chinese {
    fn name(&self) -> String {
        "Chinese".into()
    }
    fn code(&self) -> String {
        "zh".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        self.jieba
            .cut_for_search(text, false)
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(RegexTrimmer::new("trimmer-zh", r"\p{Unified_Ideograph}\p{Latin}")),
                Box::new(FnWrapper("stopWordFilter-zh".into(), stop_word_filter)),
                Box::new(FnWrapper("stemmer-zh".into(), stemmer)),
            ],
        }
    }
}

// TODO: lunr.zh.js has a much larger set of stop words
fn stop_word_filter(token: String) -> Option<String> {
    match token.as_str() {
        "的" | "了" => None,
        _ => Some(token),
    }
}

// lunr.zh.js has an empty stemmer as well
fn stemmer(token: String) -> Option<String> {
    Some(token)
}
