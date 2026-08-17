//! Defines the pipeline which processes text for inclusion in the index. Most users do not need
//! to use this module directly.

use serde::ser::{Serialize, SerializeSeq, Serializer};

pub trait PipelineFn {
    fn name(&self) -> String;

    fn filter(&self, token: String) -> Option<String>;
}

#[derive(Clone)]
pub struct FnWrapper(pub String, pub fn(String) -> Option<String>);

impl PipelineFn for FnWrapper {
    fn name(&self) -> String {
        self.0.clone()
    }

    fn filter(&self, token: String) -> Option<String> {
        (self.1)(token)
    }
}

/// A sequence of `PipelineFn`s which are run on tokens to prepare them for searching.
#[derive(Deserialize)]
pub struct Pipeline {
    #[serde(skip_deserializing)]
    pub queue: Vec<Box<dyn PipelineFn>>,
}

impl Serialize for Pipeline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.queue.len()))?;
        for elem in &self.queue {
            seq.serialize_element(&elem.name())?;
        }
        seq.end()
    }
}

impl Pipeline {
    /// Run the Pipeline against the given vector of tokens. The returned vector may be shorter
    /// than the input if a pipeline function returns `None` for a token.
    pub fn run(&self, tokens: Vec<String>) -> Vec<String> {
        let mut ret = vec![];
        for token in tokens {
            let mut token = Some(token);
            for func in &self.queue {
                if let Some(t) = token {
                    token = func.filter(t);
                } else {
                    break;
                }
            }
            if let Some(t) = token {
                ret.push(t);
            }
        }
        ret
    }
}
