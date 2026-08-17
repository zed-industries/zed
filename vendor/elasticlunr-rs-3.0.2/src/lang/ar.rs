use super::Language;
use crate::pipeline::{Pipeline, PipelineFn};
use regex::Regex;

/// Arabic Language
/// 
/// Designed to be compatibile with the included Javascript implementation. See `js/lunr.ar.js`.
pub struct Arabic {}

impl Arabic {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for Arabic {
    fn name(&self) -> String {
        "Arabic".into()
    }
    fn code(&self) -> String {
        "ar".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![Box::new(Stemmer::new())],
        }
    }
}

struct Stemmer {
    diacritics: Regex,
    alefs: Regex,
}

impl Stemmer {
    pub fn new() -> Self {
        let diacritics = Regex::new("[\u{0640}\u{064b}-\u{065b}]").unwrap();
        let alefs = Regex::new("[\u{0622}\u{0623}\u{0625}\u{0671}\u{0649}]").unwrap();
        Self { diacritics, alefs }
    }
}

impl PipelineFn for Stemmer {
    fn name(&self) -> String {
        "stemmer-ar".into()
    }

    fn filter(&self, token: String) -> Option<String> {
        // remove diacritics and elongating character
        let result = self.diacritics.replace(&token, "");
        // replace all variations of alef (آأإٱى) to a plain alef (ا)
        let result = self.alefs.replace(&result, "\u{0627}");
        if result.is_empty() {
            None
        } else if result == token {
            Some(token)
        } else {
            Some(result.into())
        }
    }
}
