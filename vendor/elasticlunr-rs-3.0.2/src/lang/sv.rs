use super::{
    common::{RustStemmer, StopWordFilter, RegexTrimmer},
    Language,
};
use crate::pipeline::Pipeline;
use rust_stemmers::Algorithm;

#[derive(Clone)]
pub struct Swedish {}

impl Swedish {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for Swedish {
    fn name(&self) -> String {
        "Swedish".into()
    }
    fn code(&self) -> String {
        "sv".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(RegexTrimmer::new("trimmer-sv", r"\p{Latin}")),
                Box::new(StopWordFilter::new("stopWordFilter-sv", STOP_WORDS)),
                Box::new(RustStemmer::new("stemmer-sv", Algorithm::Swedish)),
            ],
        }
    }
}

const STOP_WORDS: &[&str] = &[
    "", "alla", "allt", "att", "av", "blev", "bli", "blir", "blivit", "de", "dem", "den", "denna",
    "deras", "dess", "dessa", "det", "detta", "dig", "din", "dina", "ditt", "du", "där", "då",
    "efter", "ej", "eller", "en", "er", "era", "ert", "ett", "från", "för", "ha", "hade", "han",
    "hans", "har", "henne", "hennes", "hon", "honom", "hur", "här", "i", "icke", "ingen", "inom",
    "inte", "jag", "ju", "kan", "kunde", "man", "med", "mellan", "men", "mig", "min", "mina",
    "mitt", "mot", "mycket", "ni", "nu", "när", "någon", "något", "några", "och", "om", "oss",
    "på", "samma", "sedan", "sig", "sin", "sina", "sitta", "själv", "skulle", "som", "så", "sådan",
    "sådana", "sådant", "till", "under", "upp", "ut", "utan", "vad", "var", "vara", "varför",
    "varit", "varje", "vars", "vart", "vem", "vi", "vid", "vilka", "vilkas", "vilken", "vilket",
    "vår", "våra", "vårt", "än", "är", "åt", "över",
];
