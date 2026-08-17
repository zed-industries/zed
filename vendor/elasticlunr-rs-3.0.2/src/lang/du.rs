use super::{
    common::{RustStemmer, StopWordFilter, RegexTrimmer},
    Language,
};
use crate::pipeline::Pipeline;
use rust_stemmers::Algorithm;

#[derive(Clone)]
pub struct Dutch {}

impl Dutch {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for Dutch {
    fn name(&self) -> String {
        "Dutch".into()
    }
    fn code(&self) -> String {
        "du".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(RegexTrimmer::new("trimmer-du", r"\p{Latin}")),
                Box::new(StopWordFilter::new("stopWordFilter-du", STOP_WORDS)),
                Box::new(RustStemmer::new("stemmer-du", Algorithm::Dutch)),
            ],
        }
    }
}

const STOP_WORDS: &[&str] = &[
    "", "aan", "al", "alles", "als", "altijd", "andere", "ben", "bij", "daar", "dan", "dat", "de",
    "der", "deze", "die", "dit", "doch", "doen", "door", "dus", "een", "eens", "en", "er", "ge",
    "geen", "geweest", "haar", "had", "heb", "hebben", "heeft", "hem", "het", "hier", "hij", "hoe",
    "hun", "iemand", "iets", "ik", "in", "is", "ja", "je", "kan", "kon", "kunnen", "maar", "me",
    "meer", "men", "met", "mij", "mijn", "moet", "na", "naar", "niet", "niets", "nog", "nu", "of",
    "om", "omdat", "onder", "ons", "ook", "op", "over", "reeds", "te", "tegen", "toch", "toen",
    "tot", "u", "uit", "uw", "van", "veel", "voor", "want", "waren", "was", "wat", "werd", "wezen",
    "wie", "wil", "worden", "wordt", "zal", "ze", "zelf", "zich", "zij", "zijn", "zo", "zonder",
    "zou",
];
