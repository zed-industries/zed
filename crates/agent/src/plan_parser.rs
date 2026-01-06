use agent_client_protocol as acp;
use regex::Regex;
use std::sync::LazyLock;

/// Detects and parses plans from agent text responses.
///
/// Supports multiple formats:
/// - Markdown checkboxes:
///   - `- [ ] task` (pending)
///   - `- [-]` or `- [~]` or `- [>]` (in progress)
///   - `- [x]` (completed)
/// - XML-style plans: `<plan><step status="pending">...</step></plan>`
pub struct PlanParser;

// Pre-compiled regex patterns for efficiency
// Captures: space (pending), x/X (completed), -/~/> (in progress)
static MARKDOWN_CHECKBOX_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^[-*]\s*\[([ xX~>-])\]\s*(.+)$").expect("Invalid regex pattern")
});

static XML_STEP_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<(?:step|task|entry)(?:\s+status="(\w+)")?>([^<]+)</(?:step|task|entry)>"#)
        .expect("Invalid regex pattern")
});

impl PlanParser {
    /// Attempts to extract a plan from agent text.
    /// Returns `Some(Plan)` if a plan is detected, `None` otherwise.
    pub fn try_parse(text: &str) -> Option<acp::Plan> {
        // Strategy 1: Detect XML-style plan blocks
        if let Some(plan) = Self::parse_xml_plan(text) {
            return Some(plan);
        }

        // Strategy 2: Detect markdown checkbox lists
        if let Some(plan) = Self::parse_markdown_plan(text) {
            return Some(plan);
        }

        None
    }

    /// Parses XML-style plan blocks.
    /// Supports: `<plan>`, `<steps>`, `<task_list>` as container tags.
    fn parse_xml_plan(text: &str) -> Option<acp::Plan> {
        let start_tags = ["<plan>", "<steps>", "<task_list>"];
        let end_tags = ["</plan>", "</steps>", "</task_list>"];

        for (start, end) in start_tags.iter().zip(end_tags.iter()) {
            if let (Some(start_idx), Some(end_idx)) = (text.find(start), text.find(end)) {
                if start_idx < end_idx {
                    let content = &text[start_idx + start.len()..end_idx];
                    return Self::parse_xml_entries(content);
                }
            }
        }
        None
    }

    /// Parses individual step/task entries from XML content.
    fn parse_xml_entries(content: &str) -> Option<acp::Plan> {
        let entries: Vec<acp::PlanEntry> = XML_STEP_PATTERN
            .captures_iter(content)
            .filter_map(|cap| {
                let status_str = cap.get(1).map(|m| m.as_str()).unwrap_or("pending");
                let entry_content = cap.get(2)?.as_str().trim();

                if entry_content.is_empty() {
                    return None;
                }

                let status = Self::parse_status(status_str);

                Some(acp::PlanEntry::new(
                    entry_content.to_string(),
                    acp::PlanEntryPriority::Medium,
                    status,
                ))
            })
            .collect();

        if entries.is_empty() {
            None
        } else {
            Some(acp::Plan::new(entries))
        }
    }

    /// Parses markdown checkbox lists.
    /// Supports:
    /// - `- [ ] pending task`
    /// - `- [-]` or `- [~]` or `- [>]` in progress task
    /// - `- [x]` completed task
    /// - `* [ ] pending task` (asterisk variant)
    fn parse_markdown_plan(text: &str) -> Option<acp::Plan> {
        let entries: Vec<acp::PlanEntry> = MARKDOWN_CHECKBOX_PATTERN
            .captures_iter(text)
            .filter_map(|cap| {
                let status_char = cap.get(1)?.as_str();
                let entry_content = cap.get(2)?.as_str().trim();

                if entry_content.is_empty() {
                    return None;
                }

                let status = match status_char {
                    " " => acp::PlanEntryStatus::Pending,
                    "x" | "X" => acp::PlanEntryStatus::Completed,
                    "-" | "~" | ">" => acp::PlanEntryStatus::InProgress,
                    _ => acp::PlanEntryStatus::Pending,
                };

                Some(acp::PlanEntry::new(
                    entry_content.to_string(),
                    acp::PlanEntryPriority::Medium,
                    status,
                ))
            })
            .collect();

        // Only return a plan if we have at least 2 entries to avoid false positives
        if entries.len() >= 2 {
            Some(acp::Plan::new(entries))
        } else {
            None
        }
    }

    /// Converts status strings to PlanEntryStatus enum.
    fn parse_status(status_str: &str) -> acp::PlanEntryStatus {
        match status_str.to_lowercase().as_str() {
            "completed" | "done" | "finished" => acp::PlanEntryStatus::Completed,
            "in_progress" | "inprogress" | "running" | "current" => {
                acp::PlanEntryStatus::InProgress
            }
            "pending" | "todo" | "waiting" => acp::PlanEntryStatus::Pending,
            _ => acp::PlanEntryStatus::Pending,
        }
    }

    /// Updates the status of a specific entry in a plan.
    /// Used when the agent indicates progress on a step.
    pub fn update_entry_status(
        plan: &acp::Plan,
        entry_index: usize,
        new_status: acp::PlanEntryStatus,
    ) -> acp::Plan {
        let mut entries = plan.entries.clone();
        if let Some(entry) = entries.get_mut(entry_index) {
            *entry = acp::PlanEntry::new(
                entry.content.clone(),
                entry.priority.clone(),
                new_status,
            );
        }
        acp::Plan::new(entries)
    }

    /// Marks the first pending entry as in progress.
    /// Useful when starting work on a plan.
    pub fn start_next_entry(plan: &acp::Plan) -> acp::Plan {
        let mut entries = plan.entries.clone();
        let mut found_in_progress = false;

        for entry in entries.iter() {
            if entry.status == acp::PlanEntryStatus::InProgress {
                found_in_progress = true;
                break;
            }
        }

        if !found_in_progress {
            for entry in entries.iter_mut() {
                if entry.status == acp::PlanEntryStatus::Pending {
                    *entry = acp::PlanEntry::new(
                        entry.content.clone(),
                        entry.priority.clone(),
                        acp::PlanEntryStatus::InProgress,
                    );
                    break;
                }
            }
        }

        acp::Plan::new(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown_plan() {
        let text = r#"
Here's my plan to implement this feature:

- [ ] Analyze the existing codebase
- [ ] Create the new module
- [ ] Implement the main function
- [ ] Write unit tests
"#;

        let plan = PlanParser::try_parse(text).expect("Should parse plan");
        assert_eq!(plan.entries.len(), 4);
        assert_eq!(plan.entries[0].status, acp::PlanEntryStatus::Pending);
        assert_eq!(plan.entries[0].content, "Analyze the existing codebase");
    }

    #[test]
    fn test_parse_markdown_plan_with_completed() {
        let text = r#"
- [x] First step (done)
- [x] Second step (done)  
- [ ] Third step (pending)
- [ ] Fourth step (pending)
"#;

        let plan = PlanParser::try_parse(text).expect("Should parse plan");
        assert_eq!(plan.entries.len(), 4);
        assert_eq!(plan.entries[0].status, acp::PlanEntryStatus::Completed);
        assert_eq!(plan.entries[1].status, acp::PlanEntryStatus::Completed);
        assert_eq!(plan.entries[2].status, acp::PlanEntryStatus::Pending);
    }

    #[test]
    fn test_parse_markdown_plan_with_in_progress() {
        let text = r#"
- [x] First step (completed)
- [-] Second step (in progress)
- [ ] Third step (pending)
- [ ] Fourth step (pending)
"#;

        let plan = PlanParser::try_parse(text).expect("Should parse plan");
        assert_eq!(plan.entries.len(), 4);
        assert_eq!(plan.entries[0].status, acp::PlanEntryStatus::Completed);
        assert_eq!(plan.entries[1].status, acp::PlanEntryStatus::InProgress);
        assert_eq!(plan.entries[2].status, acp::PlanEntryStatus::Pending);
        assert_eq!(plan.entries[3].status, acp::PlanEntryStatus::Pending);
    }

    #[test]
    fn test_parse_markdown_plan_with_tilde_in_progress() {
        // Also support [~] and [>] as in-progress markers
        let text = r#"
- [~] Step using tilde
- [>] Step using arrow
"#;

        let plan = PlanParser::try_parse(text).expect("Should parse plan");
        assert_eq!(plan.entries.len(), 2);
        assert_eq!(plan.entries[0].status, acp::PlanEntryStatus::InProgress);
        assert_eq!(plan.entries[1].status, acp::PlanEntryStatus::InProgress);
    }

    #[test]
    fn test_parse_xml_plan() {
        let text = r#"
I'll work on this step by step:

<plan>
<step status="completed">Analyze the requirements</step>
<step status="in_progress">Implement the feature</step>
<step status="pending">Write documentation</step>
</plan>

Let me start with the implementation.
"#;

        let plan = PlanParser::try_parse(text).expect("Should parse plan");
        assert_eq!(plan.entries.len(), 3);
        assert_eq!(plan.entries[0].status, acp::PlanEntryStatus::Completed);
        assert_eq!(plan.entries[1].status, acp::PlanEntryStatus::InProgress);
        assert_eq!(plan.entries[2].status, acp::PlanEntryStatus::Pending);
    }

    #[test]
    fn test_parse_steps_tag() {
        let text = r#"
<steps>
<step>First thing to do</step>
<step status="pending">Second thing</step>
</steps>
"#;

        let plan = PlanParser::try_parse(text).expect("Should parse plan");
        assert_eq!(plan.entries.len(), 2);
    }

    #[test]
    fn test_no_plan_in_regular_text() {
        let text = "This is just a regular response without any plan structure.";
        assert!(PlanParser::try_parse(text).is_none());
    }

    #[test]
    fn test_single_checkbox_not_a_plan() {
        // A single checkbox shouldn't be considered a plan
        let text = "- [ ] Just one item";
        assert!(PlanParser::try_parse(text).is_none());
    }

    #[test]
    fn test_update_entry_status() {
        let plan = acp::Plan::new(vec![
            acp::PlanEntry::new(
                "Step 1".to_string(),
                acp::PlanEntryPriority::Medium,
                acp::PlanEntryStatus::Pending,
            ),
            acp::PlanEntry::new(
                "Step 2".to_string(),
                acp::PlanEntryPriority::Medium,
                acp::PlanEntryStatus::Pending,
            ),
        ]);

        let updated = PlanParser::update_entry_status(&plan, 0, acp::PlanEntryStatus::Completed);
        assert_eq!(updated.entries[0].status, acp::PlanEntryStatus::Completed);
        assert_eq!(updated.entries[1].status, acp::PlanEntryStatus::Pending);
    }

    #[test]
    fn test_start_next_entry() {
        let plan = acp::Plan::new(vec![
            acp::PlanEntry::new(
                "Step 1".to_string(),
                acp::PlanEntryPriority::Medium,
                acp::PlanEntryStatus::Pending,
            ),
            acp::PlanEntry::new(
                "Step 2".to_string(),
                acp::PlanEntryPriority::Medium,
                acp::PlanEntryStatus::Pending,
            ),
        ]);

        let updated = PlanParser::start_next_entry(&plan);
        assert_eq!(updated.entries[0].status, acp::PlanEntryStatus::InProgress);
        assert_eq!(updated.entries[1].status, acp::PlanEntryStatus::Pending);
    }
}
