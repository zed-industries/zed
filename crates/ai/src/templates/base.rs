use std::cmp::Reverse;

use crate::templates::repository_context::PromptCodeSnippet;

pub(crate) enum PromptFileType {
    Text,
    Code,
}

#[derive(Default)]
pub struct PromptArguments {
    pub model_name: String,
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
    fn generate(&self, args: &PromptArguments, max_token_length: Option<usize>) -> String;
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
        // templates.sort_by(|a, b| a.0.cmp(&b.0));

        PromptChain { args, templates }
    }

    pub fn generate(&self, truncate: bool) -> anyhow::Result<String> {
        // Argsort based on Prompt Priority
        let mut sorted_indices = (0..self.templates.len()).collect::<Vec<_>>();
        sorted_indices.sort_by_key(|&i| Reverse(&self.templates[i].0));

        println!("{:?}", sorted_indices);

        let mut prompts = Vec::new();
        for (_, template) in &self.templates {
            prompts.push(template.generate(&self.args, None));
        }

        anyhow::Ok(prompts.join("\n"))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    pub fn test_prompt_chain() {
        struct TestPromptTemplate {}
        impl PromptTemplate for TestPromptTemplate {
            fn generate(&self, args: &PromptArguments, max_token_length: Option<usize>) -> String {
                "This is a test prompt template".to_string()
            }
        }

        struct TestLowPriorityTemplate {}
        impl PromptTemplate for TestLowPriorityTemplate {
            fn generate(&self, args: &PromptArguments, max_token_length: Option<usize>) -> String {
                "This is a low priority test prompt template".to_string()
            }
        }

        let args = PromptArguments {
            model_name: "gpt-4".to_string(),
            ..Default::default()
        };

        let templates: Vec<(PromptPriority, Box<dyn PromptTemplate>)> = vec![
            (PromptPriority::High, Box::new(TestPromptTemplate {})),
            (PromptPriority::Medium, Box::new(TestLowPriorityTemplate {})),
        ];
        let chain = PromptChain::new(args, templates);

        let prompt = chain.generate(false);
        println!("{:?}", prompt);
        panic!();
    }
}
