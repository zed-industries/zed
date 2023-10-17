use std::fmt::Write;
use std::{cmp::Reverse, sync::Arc};

use util::ResultExt;

use crate::templates::repository_context::PromptCodeSnippet;

pub trait LanguageModel {
    fn name(&self) -> String;
    fn count_tokens(&self, content: &str) -> usize;
    fn truncate(&self, content: &str, length: usize) -> String;
    fn capacity(&self) -> usize;
}

pub(crate) enum PromptFileType {
    Text,
    Code,
}

// TODO: Set this up to manage for defaults well
pub struct PromptArguments {
    pub model: Arc<dyn LanguageModel>,
    pub language_name: Option<String>,
    pub project_name: Option<String>,
    pub snippets: Vec<PromptCodeSnippet>,
    pub reserved_tokens: usize,
}

impl PromptArguments {
    pub(crate) fn get_file_type(&self) -> PromptFileType {
        if self
            .language_name
            .as_ref()
            .and_then(|name| Some(!["Markdown", "Plain Text"].contains(&name.as_str())))
            .unwrap_or(true)
        {
            PromptFileType::Code
        } else {
            PromptFileType::Text
        }
    }
}

pub trait PromptTemplate {
    fn generate(
        &self,
        args: &PromptArguments,
        max_token_length: Option<usize>,
    ) -> anyhow::Result<(String, usize)>;
}

#[repr(i8)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PromptPriority {
    Low,
    Medium,
    High,
}

pub struct PromptChain {
    args: PromptArguments,
    templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)>,
}

impl PromptChain {
    pub fn new(
        args: PromptArguments,
        templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)>,
    ) -> Self {
        PromptChain { args, templates }
    }

    pub fn generate(&self, truncate: bool) -> anyhow::Result<(String, usize)> {
        // Argsort based on Prompt Priority
        let seperator = "\n";
        let seperator_tokens = self.args.model.count_tokens(seperator);
        let mut sorted_indices = (0..self.templates.len()).collect::<Vec<_>>();
        sorted_indices.sort_by_key(|&i| Reverse(&self.templates[i].0));

        let mut prompts = Vec::new();

        // If Truncate
        let mut tokens_outstanding = if truncate {
            Some(self.args.model.capacity() - self.args.reserved_tokens)
        } else {
            None
        };

        for idx in sorted_indices {
            let (_, template) = &self.templates[idx];
            if let Some((template_prompt, prompt_token_count)) =
                template.generate(&self.args, tokens_outstanding).log_err()
            {
                println!(
                    "GENERATED PROMPT ({:?}): {:?}",
                    &prompt_token_count, &template_prompt
                );
                if template_prompt != "" {
                    prompts.push(template_prompt);

                    if let Some(remaining_tokens) = tokens_outstanding {
                        let new_tokens = prompt_token_count + seperator_tokens;
                        tokens_outstanding = if remaining_tokens > new_tokens {
                            Some(remaining_tokens - new_tokens)
                        } else {
                            Some(0)
                        };
                    }
                }
            }
        }

        let full_prompt = prompts.join(seperator);
        let total_token_count = self.args.model.count_tokens(&full_prompt);
        anyhow::Ok((prompts.join(seperator), total_token_count))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    pub fn test_prompt_chain() {
        struct TestPromptTemplate {}
        impl PromptTemplate for TestPromptTemplate {
            fn generate(
                &self,
                args: &PromptArguments,
                max_token_length: Option<usize>,
            ) -> anyhow::Result<(String, usize)> {
                let mut content = "This is a test prompt template".to_string();

                let mut token_count = args.model.count_tokens(&content);
                if let Some(max_token_length) = max_token_length {
                    if token_count > max_token_length {
                        content = args.model.truncate(&content, max_token_length);
                        token_count = max_token_length;
                    }
                }

                anyhow::Ok((content, token_count))
            }
        }

        struct TestLowPriorityTemplate {}
        impl PromptTemplate for TestLowPriorityTemplate {
            fn generate(
                &self,
                args: &PromptArguments,
                max_token_length: Option<usize>,
            ) -> anyhow::Result<(String, usize)> {
                let mut content = "This is a low priority test prompt template".to_string();

                let mut token_count = args.model.count_tokens(&content);
                if let Some(max_token_length) = max_token_length {
                    if token_count > max_token_length {
                        content = args.model.truncate(&content, max_token_length);
                        token_count = max_token_length;
                    }
                }

                anyhow::Ok((content, token_count))
            }
        }

        #[derive(Clone)]
        struct DummyLanguageModel {
            capacity: usize,
        }

        impl DummyLanguageModel {
            fn set_capacity(&mut self, capacity: usize) {
                self.capacity = capacity
            }
        }

        impl LanguageModel for DummyLanguageModel {
            fn name(&self) -> String {
                "dummy".to_string()
            }
            fn count_tokens(&self, content: &str) -> usize {
                content.chars().collect::<Vec<char>>().len()
            }
            fn truncate(&self, content: &str, length: usize) -> String {
                content.chars().collect::<Vec<char>>()[..length]
                    .into_iter()
                    .collect::<String>()
            }
            fn capacity(&self) -> usize {
                self.capacity
            }
        }

        let model: Arc<dyn LanguageModel> = Arc::new(DummyLanguageModel { capacity: 100 });
        let args = PromptArguments {
            model: model.clone(),
            language_name: None,
            project_name: None,
            snippets: Vec::new(),
            reserved_tokens: 0,
        };

        let templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)> = vec![
            (PromptPriority::High, Box::new(TestPromptTemplate {})),
            (PromptPriority::Medium, Box::new(TestLowPriorityTemplate {})),
        ];
        let chain = PromptChain::new(args, templates);

        let (prompt, token_count) = chain.generate(false).unwrap();

        assert_eq!(
            prompt,
            "This is a test prompt template\nThis is a low priority test prompt template"
                .to_string()
        );

        assert_eq!(model.count_tokens(&prompt), token_count);

        // Testing with Truncation Off
        // Should ignore capacity and return all prompts
        let model: Arc<dyn LanguageModel> = Arc::new(DummyLanguageModel { capacity: 20 });
        let args = PromptArguments {
            model: model.clone(),
            language_name: None,
            project_name: None,
            snippets: Vec::new(),
            reserved_tokens: 0,
        };

        let templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)> = vec![
            (PromptPriority::High, Box::new(TestPromptTemplate {})),
            (PromptPriority::Medium, Box::new(TestLowPriorityTemplate {})),
        ];
        let chain = PromptChain::new(args, templates);

        let (prompt, token_count) = chain.generate(false).unwrap();

        assert_eq!(
            prompt,
            "This is a test prompt template\nThis is a low priority test prompt template"
                .to_string()
        );

        assert_eq!(model.count_tokens(&prompt), token_count);

        // Testing with Truncation Off
        // Should ignore capacity and return all prompts
        let capacity = 20;
        let model: Arc<dyn LanguageModel> = Arc::new(DummyLanguageModel { capacity });
        let args = PromptArguments {
            model: model.clone(),
            language_name: None,
            project_name: None,
            snippets: Vec::new(),
            reserved_tokens: 0,
        };

        let templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)> = vec![
            (PromptPriority::High, Box::new(TestPromptTemplate {})),
            (PromptPriority::Medium, Box::new(TestLowPriorityTemplate {})),
            (PromptPriority::Low, Box::new(TestLowPriorityTemplate {})),
        ];
        let chain = PromptChain::new(args, templates);

        let (prompt, token_count) = chain.generate(true).unwrap();

        assert_eq!(prompt, "This is a test promp".to_string());
        assert_eq!(token_count, capacity);

        // Change Ordering of Prompts Based on Priority
        let capacity = 120;
        let reserved_tokens = 10;
        let model: Arc<dyn LanguageModel> = Arc::new(DummyLanguageModel { capacity });
        let args = PromptArguments {
            model: model.clone(),
            language_name: None,
            project_name: None,
            snippets: Vec::new(),
            reserved_tokens,
        };
        let templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)> = vec![
            (PromptPriority::Medium, Box::new(TestPromptTemplate {})),
            (PromptPriority::High, Box::new(TestLowPriorityTemplate {})),
            (PromptPriority::Low, Box::new(TestLowPriorityTemplate {})),
        ];
        let chain = PromptChain::new(args, templates);

        let (prompt, token_count) = chain.generate(true).unwrap();
        println!("TOKEN COUNT: {:?}", token_count);

        assert_eq!(
            prompt,
            "This is a low priority test prompt template\nThis is a test prompt template\nThis is a low priority test prompt "
                .to_string()
        );
        assert_eq!(token_count, capacity - reserved_tokens);
    }
}
