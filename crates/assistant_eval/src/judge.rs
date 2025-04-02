use crate::eval::EvalOutput;
use crate::headless_assistant::send_language_model_request;
use anyhow::anyhow;
use gpui::{App, Task};
use language_model::{
    LanguageModel, LanguageModelRequest, LanguageModelRequestMessage, MessageContent, Role,
};
use std::{path::Path, sync::Arc};

pub struct Judge {
    pub original_diff: Option<String>,
    #[allow(dead_code)]
    pub original_message: Option<String>,
    pub model: Arc<dyn LanguageModel>,
}

impl Judge {
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
        cx.spawn(async move |cx| send_language_model_request(model, request, cx).await)
    }
}

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
