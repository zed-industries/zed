use anyhow::anyhow;
use tiktoken_rs::CoreBPE;
use util::ResultExt;

pub trait LanguageModel {
    fn name(&self) -> String;
    fn count_tokens(&self, content: &str) -> anyhow::Result<usize>;
    fn truncate(&self, content: &str, length: usize) -> anyhow::Result<String>;
    fn capacity(&self) -> anyhow::Result<usize>;
}

pub struct OpenAILanguageModel {
    name: String,
    bpe: Option<CoreBPE>,
}

impl OpenAILanguageModel {
    pub fn load(model_name: &str) -> Self {
        let bpe = tiktoken_rs::get_bpe_from_model(model_name).log_err();
        OpenAILanguageModel {
            name: model_name.to_string(),
            bpe,
        }
    }
}

impl LanguageModel for OpenAILanguageModel {
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
    fn truncate(&self, content: &str, length: usize) -> anyhow::Result<String> {
        if let Some(bpe) = &self.bpe {
            let tokens = bpe.encode_with_special_tokens(content);
            bpe.decode(tokens[..length].to_vec())
        } else {
            Err(anyhow!("bpe for open ai model was not retrieved"))
        }
    }
    fn capacity(&self) -> anyhow::Result<usize> {
        anyhow::Ok(tiktoken_rs::model::get_context_size(&self.name))
    }
}
