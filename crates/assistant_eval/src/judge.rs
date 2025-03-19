use crate::eval::EvalOutput;
use crate::headless_assistant::send_language_model_request;
use anyhow::anyhow;
use gpui::{App, Task};
use language_model::{
    LanguageModel, LanguageModelRequest, LanguageModelRequestMessage, MessageContent, Role,
};
use std::{path::Path, sync::Arc};

pub struct Judge {
    #[allow(dead_code)]
    pub original_diff: Option<String>,
    pub original_message: Option<String>,
    pub model: Arc<dyn LanguageModel>,
}

impl Judge {
    #[allow(dead_code)]
    pub async fn load(eval_path: &Path, model: Arc<dyn LanguageModel>) -> anyhow::Result<Judge> {
        let original_diff_path = eval_path.join("original.diff");
        let original_diff = smol::unblock(move || {
            if std::fs::exists(&original_diff_path)? {
                anyhow::Ok(Some(std::fs::read_to_string(&original_diff_path)?))
            } else {
                anyhow::Ok(None)
            }
        });

        let original_message_path = eval_path.join("original_message.txt");
        let original_message = smol::unblock(move || {
            if std::fs::exists(&original_message_path)? {
                anyhow::Ok(Some(std::fs::read_to_string(&original_message_path)?))
            } else {
                anyhow::Ok(None)
            }
        });

        Ok(Self {
            original_diff: original_diff.await?,
            original_message: original_message.await?,
            model,
        })
    }

    #[allow(dead_code)]
    pub fn run(&self, eval_output: &EvalOutput, cx: &mut App) -> Task<anyhow::Result<String>> {
        let Some(original_diff) = self.original_diff.as_ref() else {
            return Task::ready(Err(anyhow!("No original.diff found")));
        };

        // TODO: check for empty diff?
        let prompt = diff_comparison_prompt(&original_diff, &eval_output.diff);

        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text(prompt)],
                cache: false,
            }],
            temperature: Some(0.0),
            tools: Vec::new(),
            stop: Vec::new(),
        };

        let model = self.model.clone();
        cx.spawn(move |cx| send_language_model_request(model, request, cx))
    }

    // Add a new method that accepts a prompt from the original_message field
    pub fn run_with_prompt(&self, cx: &mut App) -> Task<anyhow::Result<String>> {
        let Some(prompt) = self.original_message.as_ref() else {
            return Task::ready(Err(anyhow!("No prompt provided in original_message")));
        };

        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text(prompt.clone())],
                cache: false,
            }],
            temperature: Some(0.0),
            tools: Vec::new(),
            stop: Vec::new(),
        };

        let model = self.model.clone();
        cx.spawn(move |cx| send_language_model_request(model, request, cx))
    }
}

#[allow(dead_code)]
pub fn code_comparison_prompt(language: &str, example_code: &str, generated_code: &str) -> String {
    format!(
        r#"# Code Similarity Evaluation Template

## Instructions

Compare the two code implementations and score them between 0.0 and 1.0 based on their functional similarity.
- 1.0 = Perfect functional match (both implementations produce identical results for all inputs)
- 0.0 = Completely different functionality or non-functional code

## Evaluation Criteria

Please consider the following aspects in order of importance:

1. **Functional Equivalence (60%)**
   - Do both implementations produce the same output for the same inputs?
   - Do they handle edge cases similarly?
   - Is the core logic equivalent despite differences in style?

2. **Algorithmic Approach (20%)**
   - Do they use similar algorithms or data structures?
   - Is the time and space complexity comparable?
   - Do they process data in a similar manner?

3. **Code Structure (15%)**
   - Are the functions/methods organized similarly?
   - Are control structures used in comparable ways?
   - Is the overall structure and flow of the code similar?

4. **Style and Conventions (5%)**
   - Do they follow similar naming conventions?
   - Is formatting and documentation approach similar?
   - Do they use language features in comparable ways?

## Input

Language: {0}

Example Implementation:
```
{1}
```

Generated Implementation:
```
{2}
```

## Output Format

THE ONLY OUTPUT SHOULD BE A SCORE BETWEEN 0.0 AND 1.0.

Example output:
0.85"#,
        language, example_code, generated_code
    )
}

#[allow(dead_code)]
pub fn diff_comparison_prompt(original_diff: &str, new_diff: &str) -> String {
    format!(
        r#"# Git Diff Similarity Evaluation Template

## Instructions

Compare the two diffs and score them between 0.0 and 1.0 based on their functional similarity.
- 1.0 = Perfect functional match (achieves identical results)
- 0.0 = No functional similarity whatsoever

## Evaluation Criteria

Please consider the following aspects in order of importance:

1. **Functional Equivalence (60%)**
   - Do both diffs achieve the same end result?
   - Are the changes functionally equivalent despite possibly using different approaches?
   - Do the modifications address the same issues or implement the same features?

2. **Logical Structure (20%)**
   - Are the logical flows similar?
   - Do the modifications affect the same code paths?
   - Are control structures (if/else, loops, etc.) modified in similar ways?

3. **Code Content (15%)**
   - Are similar lines added/removed?
   - Are the same variables, functions, or methods being modified?
   - Are the same APIs or libraries being used?

4. **File Layout (5%)**
   - Are the same files being modified?
   - Are changes occurring in similar locations within files?

## Input

Original Diff:
```git
{}
```

New Diff:
```git
{}
```

## Output Format

THE ONLY OUTPUT SHOULD BE A SCORE BETWEEN 0.0 AND 1.0.

Example output:
0.85"#,
        original_diff, new_diff
    )
}
