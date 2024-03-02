use std::cmp::Reverse;
use std::ops::Range;
use std::sync::Arc;

use language::BufferSnapshot;
use util::ResultExt;

use crate::models::LanguageModel;
use crate::prompts::repository_context::PromptCodeSnippet;

pub(crate) enum PromptFileType {
    Text,
    Code,
}

// TODO: Set this up to manage for defaults well
pub struct PromptArguments {
    pub model: Arc<dyn LanguageModel>,
    pub user_prompt: Option<String>,
    pub language_name: Option<String>,
    pub project_name: Option<String>,
    pub snippets: Vec<PromptCodeSnippet>,
    pub reserved_tokens: usize,
    pub buffer: Option<BufferSnapshot>,
    pub selected_range: Option<Range<usize>>,
}

impl PromptArguments {
    pub(crate) fn get_file_type(&self) -> PromptFileType {
        if self
            .language_name
            .as_ref()
            .map(|name| !["Markdown", "Plain Text"].contains(&name.as_str()))
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
#[derive(PartialEq, Eq, Ord)]
pub enum PromptPriority {
    /// Ignores truncation.
    Mandatory,
    /// Truncates based on priority.
    Ordered { order: usize },
}

impl PartialOrd for PromptPriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Mandatory, Self::Mandatory) => Some(std::cmp::Ordering::Equal),
            (Self::Mandatory, Self::Ordered { .. }) => Some(std::cmp::Ordering::Greater),
            (Self::Ordered { .. }, Self::Mandatory) => Some(std::cmp::Ordering::Less),
            (Self::Ordered { order: a }, Self::Ordered { order: b }) => b.partial_cmp(a),
        }
    }
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
        let separator = "\n";
        let separator_tokens = self.args.model.count_tokens(separator)?;
        let mut sorted_indices = (0..self.templates.len()).collect::<Vec<_>>();
        sorted_indices.sort_by_key(|&i| Reverse(&self.templates[i].0));

        let mut tokens_outstanding = if truncate {
            Some(self.args.model.capacity()? - self.args.reserved_tokens)
        } else {
            None
        };

        let mut prompts = vec!["".to_string(); sorted_indices.len()];
        for idx in sorted_indices {
            let (_, template) = &self.templates[idx];

            if let Some((template_prompt, prompt_token_count)) =
                template.generate(&self.args, tokens_outstanding).log_err()
            {
                if template_prompt != "" {
                    prompts[idx] = template_prompt;

                    if let Some(remaining_tokens) = tokens_outstanding {
                        let new_tokens = prompt_token_count + separator_tokens;
                        tokens_outstanding = if remaining_tokens > new_tokens {
                            Some(remaining_tokens - new_tokens)
                        } else {
                            Some(0)
                        };
                    }
                }
            }
        }

        prompts.retain(|x| x != "");

        let full_prompt = prompts.join(separator);
        let total_token_count = self.args.model.count_tokens(&full_prompt)?;
        anyhow::Ok((prompts.join(separator), total_token_count))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::models::TruncationDirection;
    use crate::test::FakeLanguageModel;

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

                let mut token_count = args.model.count_tokens(&content)?;
                if let Some(max_token_length) = max_token_length {
                    if token_count > max_token_length {
                        content = args.model.truncate(
                            &content,
                            max_token_length,
                            TruncationDirection::End,
                        )?;
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

                let mut token_count = args.model.count_tokens(&content)?;
                if let Some(max_token_length) = max_token_length {
                    if token_count > max_token_length {
                        content = args.model.truncate(
                            &content,
                            max_token_length,
                            TruncationDirection::End,
                        )?;
                        token_count = max_token_length;
                    }
                }

                anyhow::Ok((content, token_count))
            }
        }

        let model: Arc<dyn LanguageModel> = Arc::new(FakeLanguageModel { capacity: 100 });
        let args = PromptArguments {
            model: model.clone(),
            language_name: None,
            project_name: None,
            snippets: Vec::new(),
            reserved_tokens: 0,
            buffer: None,
            selected_range: None,
            user_prompt: None,
        };

        let templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)> = vec![
            (
                PromptPriority::Ordered { order: 0 },
                Box::new(TestPromptTemplate {}),
            ),
            (
                PromptPriority::Ordered { order: 1 },
                Box::new(TestLowPriorityTemplate {}),
            ),
        ];
        let chain = PromptChain::new(args, templates);

        let (prompt, token_count) = chain.generate(false).unwrap();

        assert_eq!(
            prompt,
            "This is a test prompt template\nThis is a low priority test prompt template"
                .to_string()
        );

        assert_eq!(model.count_tokens(&prompt).unwrap(), token_count);

        // Testing with Truncation Off
        // Should ignore capacity and return all prompts
        let model: Arc<dyn LanguageModel> = Arc::new(FakeLanguageModel { capacity: 20 });
        let args = PromptArguments {
            model: model.clone(),
            language_name: None,
            project_name: None,
            snippets: Vec::new(),
            reserved_tokens: 0,
            buffer: None,
            selected_range: None,
            user_prompt: None,
        };

        let templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)> = vec![
            (
                PromptPriority::Ordered { order: 0 },
                Box::new(TestPromptTemplate {}),
            ),
            (
                PromptPriority::Ordered { order: 1 },
                Box::new(TestLowPriorityTemplate {}),
            ),
        ];
        let chain = PromptChain::new(args, templates);

        let (prompt, token_count) = chain.generate(false).unwrap();

        assert_eq!(
            prompt,
            "This is a test prompt template\nThis is a low priority test prompt template"
                .to_string()
        );

        assert_eq!(model.count_tokens(&prompt).unwrap(), token_count);

        // Testing with Truncation Off
        // Should ignore capacity and return all prompts
        let capacity = 20;
        let model: Arc<dyn LanguageModel> = Arc::new(FakeLanguageModel { capacity });
        let args = PromptArguments {
            model: model.clone(),
            language_name: None,
            project_name: None,
            snippets: Vec::new(),
            reserved_tokens: 0,
            buffer: None,
            selected_range: None,
            user_prompt: None,
        };

        let templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)> = vec![
            (
                PromptPriority::Ordered { order: 0 },
                Box::new(TestPromptTemplate {}),
            ),
            (
                PromptPriority::Ordered { order: 1 },
                Box::new(TestLowPriorityTemplate {}),
            ),
            (
                PromptPriority::Ordered { order: 2 },
                Box::new(TestLowPriorityTemplate {}),
            ),
        ];
        let chain = PromptChain::new(args, templates);

        let (prompt, token_count) = chain.generate(true).unwrap();

        assert_eq!(prompt, "This is a test promp".to_string());
        assert_eq!(token_count, capacity);

        // Change Ordering of Prompts Based on Priority
        let capacity = 120;
        let reserved_tokens = 10;
        let model: Arc<dyn LanguageModel> = Arc::new(FakeLanguageModel { capacity });
        let args = PromptArguments {
            model: model.clone(),
            language_name: None,
            project_name: None,
            snippets: Vec::new(),
            reserved_tokens,
            buffer: None,
            selected_range: None,
            user_prompt: None,
        };
        let templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)> = vec![
            (
                PromptPriority::Mandatory,
                Box::new(TestLowPriorityTemplate {}),
            ),
            (
                PromptPriority::Ordered { order: 0 },
                Box::new(TestPromptTemplate {}),
            ),
            (
                PromptPriority::Ordered { order: 1 },
                Box::new(TestLowPriorityTemplate {}),
            ),
        ];
        let chain = PromptChain::new(args, templates);

        let (prompt, token_count) = chain.generate(true).unwrap();

        assert_eq!(
            prompt,
            "This is a low priority test prompt template\nThis is a test prompt template\nThis is a low priority test prompt "
                .to_string()
        );
        assert_eq!(token_count, capacity - reserved_tokens);
    }
}
