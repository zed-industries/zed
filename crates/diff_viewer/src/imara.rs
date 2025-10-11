use imara_diff::{Algorithm, Diff, InternedInput};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct ImaraDiffBlock {
    pub left_range: Range<usize>,
    pub right_range: Range<usize>,
    pub operation: ImaraBlockOperation,
    pub semantic_similarity: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImaraBlockOperation {
    Insert,
    Delete,
    Modify,
}

#[derive(Debug, Clone)]
pub struct ImaraConfig {
    pub algorithm: Algorithm,
}

impl Default for ImaraConfig {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::Histogram,
        }
    }
}

impl ImaraDiffBlock {
    pub fn new(
        left_range: Range<usize>,
        right_range: Range<usize>,
        operation: ImaraBlockOperation,
    ) -> Self {
        Self {
            left_range,
            right_range,
            operation,
            semantic_similarity: None,
        }
    }

    pub fn with_similarity(mut self, similarity: f32) -> Self {
        self.semantic_similarity = Some(similarity);
        self
    }
}

#[derive(Debug, Clone)]
pub struct ImaraDiffAnalysis {
    pub blocks: Vec<ImaraDiffBlock>,
}

fn calculate_semantic_similarity(old_lines: &[&str], new_lines: &[&str]) -> f32 {
    if old_lines.is_empty() || new_lines.is_empty() {
        return 0.0;
    }

    let old_text = old_lines.join("\n");
    let new_text = new_lines.join("\n");

    if old_text == new_text {
        return 100.0;
    }

    // Simple similarity calculation based on common characters
    let old_chars: std::collections::HashSet<char> = old_text.chars().collect();
    let new_chars: std::collections::HashSet<char> = new_text.chars().collect();

    let intersection = old_chars.intersection(&new_chars).count();
    let union = old_chars.union(&new_chars).count();

    if union == 0 {
        0.0
    } else {
        (intersection as f32 / union as f32) * 100.0
    }
}

pub fn compute_imara_diff(
    old_content: &str,
    new_content: &str,
    config: &ImaraConfig,
) -> ImaraDiffAnalysis {
    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();

    let input = InternedInput::new(old_content, new_content);
    let mut diff = Diff::compute(config.algorithm, &input);
    diff.postprocess_lines(&input);

    let mut blocks = Vec::new();

    // Process hunks to build blocks and line mappings
    for hunk in diff.hunks() {
        let old_range = hunk.before.start as usize..hunk.before.end as usize;
        let new_range = hunk.after.start as usize..hunk.after.end as usize;

        let operation = if old_range.is_empty() {
            ImaraBlockOperation::Insert
        } else if new_range.is_empty() {
            ImaraBlockOperation::Delete
        } else {
            ImaraBlockOperation::Modify
        };

        // Calculate semantic similarity for the hunk
        let old_hunk_lines: Vec<&str> = if old_range.is_empty() {
            Vec::new()
        } else {
            old_lines[old_range.clone()].to_vec()
        };

        let new_hunk_lines: Vec<&str> = if new_range.is_empty() {
            Vec::new()
        } else {
            new_lines[new_range.clone()].to_vec()
        };

        let similarity = calculate_semantic_similarity(&old_hunk_lines, &new_hunk_lines);

        let block = ImaraDiffBlock::new(old_range.clone(), new_range.clone(), operation)
            .with_similarity(similarity);

        blocks.push(block);
    }

    ImaraDiffAnalysis { blocks }
}

pub fn compute_imara_diff_default(old_content: &str, new_content: &str) -> ImaraDiffAnalysis {
    compute_imara_diff(old_content, new_content, &ImaraConfig::default())
}
