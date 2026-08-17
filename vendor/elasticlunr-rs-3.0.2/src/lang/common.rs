use crate::pipeline::PipelineFn;
use regex::Regex;
use std::collections::HashSet;

#[derive(Clone)]
pub struct StopWordFilter {
    name: String,
    stop_words: HashSet<String>,
}

impl StopWordFilter {
    pub fn new(name: &str, stop_words: &[&str]) -> Self {
        Self {
            name: name.into(),
            stop_words: stop_words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl PipelineFn for StopWordFilter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn filter(&self, token: String) -> Option<String> {
        if self.stop_words.contains(&token) {
            None
        } else {
            Some(token)
        }
    }
}

#[derive(Clone)]
pub struct RegexTrimmer {
    name: String,
    trimmer: Regex,
}

impl RegexTrimmer {
    pub fn new(name: &str, word_chars: &str) -> Self {
        let name = name.into();
        let trimmer = Regex::new(&format!("^[^{0}]+|[^{0}]+$", word_chars)).unwrap();
        Self { name, trimmer }
    }
}

impl PipelineFn for RegexTrimmer {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn filter(&self, token: String) -> Option<String> {
        let result = self.trimmer.replace_all(&token, "");
        if result.is_empty() {
            None
        } else if result == token {
            Some(token)
        } else {
            Some(result.into())
        }
    }
}

#[cfg(feature = "rust-stemmers")]
pub struct RustStemmer {
    name: String,
    stemmer: rust_stemmers::Stemmer,
}

#[cfg(feature = "rust-stemmers")]
impl RustStemmer {
    pub fn new(name: &str, algo: rust_stemmers::Algorithm) -> Self {
        Self {
            name: name.into(),
            stemmer: rust_stemmers::Stemmer::create(algo),
        }
    }
}

#[cfg(feature = "rust-stemmers")]
impl PipelineFn for RustStemmer {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn filter(&self, token: String) -> Option<String> {
        let result = self.stemmer.stem(&token);
        if result.is_empty() {
            None
        } else if result == token {
            Some(token)
        } else {
            Some(result.into())
        }
    }
}
