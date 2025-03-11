pub fn generate_diff_comparison_template(original_diff: &str, new_diff: &str) -> String {
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
