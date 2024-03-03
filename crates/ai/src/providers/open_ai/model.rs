use anyhow::anyhow;
use tiktoken_rs::CoreBPE;

use crate::models::{LanguageModel, TruncationDirection};

use super::open_ai_bpe_tokenizer;

#[derive(Clone)]
pub struct OpenAiLanguageModel {
    name: String,
    bpe: Option<CoreBPE>,
}

impl OpenAiLanguageModel {
    pub fn load(model_name: &str) -> Self {
        let bpe = tiktoken_rs::get_bpe_from_model(model_name)
            .unwrap_or(open_ai_bpe_tokenizer().to_owned());
        OpenAiLanguageModel {
            name: model_name.to_string(),
            bpe: Some(bpe),
        }
    }
}

impl LanguageModel for OpenAiLanguageModel {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn count_tokens(&self, content: &str) -> anyhow::Result<usize> {
        if let Some(bpe) = &self.bpe {
            anyhow::Ok(bpe.encode_with_special_tokens(content).len())
        } else {
            Err(anyhow!("bpe for open ai model was not retrieved"))
        }
    }
    fn truncate(
        &self,
        content: &str,
        length: usize,
        direction: TruncationDirection,
    ) -> anyhow::Result<String> {
        if let Some(bpe) = &self.bpe {
            let tokens = bpe.encode_with_special_tokens(content);
            if tokens.len() > length {
                match direction {
                    TruncationDirection::End => bpe.decode(tokens[..length].to_vec()),
                    TruncationDirection::Start => bpe.decode(tokens[length..].to_vec()),
                }
            } else {
                bpe.decode(tokens)
            }
        } else {
            Err(anyhow!("bpe for open ai model was not retrieved"))
        }
    }
    fn capacity(&self) -> anyhow::Result<usize> {
        anyhow::Ok(tiktoken_rs::model::get_context_size(&self.name))
    }
}
