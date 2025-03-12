use crate::eval::EvalOutput;
use language_model::LanguageModelProviderId;
use std::path::Path;

pub struct Judge {
    pub original_diff: Option<String>,
    pub original_message: Option<String>,
    pub provider_id: LanguageModelProviderId,
    pub model_name: String,
}

impl Judge {
    pub fn load(
        eval_path: &Path,
        provider_id: LanguageModelProviderId,
        model_name: String,
    ) -> anyhow::Result<Judge> {
        // TODO: "original" seems confusing - rename?
        let original_diff_path = eval_path.join("original.diff");
        let original_diff = if std::fs::exists(&original_diff_path)? {
            Some(std::fs::read_to_string(&original_diff_path)?)
        } else {
            None
        };

        let original_message_path = eval_path.join("original_message.txt");
        let original_message = if std::fs::exists(&original_message_path)? {
            Some(std::fs::read_to_string(&original_message_path)?)
        } else {
            None
        };

        Ok(Self {
            original_diff,
            original_message,
            provider_id,
            model_name,
        })
    }

    pub fn run(&self, eval_output: &EvalOutput) -> anyhow::Result<()> {
        // todo! also compare last message, to handle Q/A eval.

        Ok(())
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
