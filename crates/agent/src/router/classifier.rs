//! Prompt classifier — determines the task type from a user message
//! using keyword-based rules (Phase 1) and optionally a cheap LLM
//! fallback (Phase 2).

use crate::router::TaskType;

pub struct TaskClassifier;

impl TaskClassifier {
    /// Classify a user prompt into a TaskType.
    ///
    /// Phase 1 — rules-based: fast, zero token cost.
    /// Phase 2 — (future) cheap LLM fallback when rules are ambiguous.
    pub fn classify(&self, prompt: &str) -> TaskType {
        let lower = prompt.to_lowercase();

        // ── Rule set (priority order) ──

        // Review: check, review, audit, verify, validate, CR
        if contains_any(&lower, &[
            "review ", "code review", "cr ", "audit ", "verify ",
            "validate ", "check my", "check the", "proofread",
            "does this look", "is this correct",
        ]) {
            return TaskType::Review;
        }

        // Vision: image, screenshot, look at this, UI, visual
        if contains_any(&lower, &[
            "screenshot", "image", "look at this", "take a look at",
            "what does this", "ui ", "visual", "picture of",
        ]) {
            return TaskType::Vision;
        }

        // Planning: architecture, design, plan, strategy, break down
        if contains_any(&lower, &[
            "plan ", "architect", "design ", "break down", "task list",
            "strategy", "outline ", "approach", "milestone", "roadmap",
            "how should i", "what's the best way",
        ]) && contains_few(&lower, &["code", "implement", "write ", "edit "])
        {
            return TaskType::Planning;
        }

        // Terminal: commands, build, test, deploy, run
        if contains_any(&lower, &[
            "run ", "build ", "`", "cargo ", "npm ", "pnpm ", "yarn ",
            "deploy", "compile", "install ", "test ",
            "in the terminal", "in terminal",
        ]) && contains_few(&lower, &["edit", "change", "refactor", "add ", "create "])
        {
            return TaskType::Terminal;
        }

        // Research: search, find, look up, documentation, what is, how does
        if contains_any(&lower, &[
            "search ", "find ", "look up", "lookup",
            "what is", "what are", "how does", "how do",
            "documentation", "docs for", "research",
            "explain ", "tell me about",
        ]) {
            return TaskType::Research;
        }

        // Edit: changes to code
        if contains_any(&lower, &[
            "edit ", "change ", "refactor", "rewrite", "fix ", "update ",
            "add ", "create ", "implement", "modify", "remove ",
            "write a function", "write code",
        ]) {
            return TaskType::Edit;
        }

        // General: everything else
        TaskType::General
    }
}

fn contains_any(text: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|p| text.contains(p))
}

/// Returns true if `text` contains at most one of the `patterns`.
/// Used to distinguish terminal-heavy prompts from mixed ones.
fn contains_few(text: &str, patterns: &[&str]) -> bool {
    patterns.iter().filter(|p| text.contains(*p)).count() <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_terminal() {
        let classifier = TaskClassifier;
        assert_eq!(classifier.classify("Run cargo test"), TaskType::Terminal);
        assert_eq!(classifier.classify("npm run build"), TaskType::Terminal);
        assert_eq!(classifier.classify("deploy to staging"), TaskType::Terminal);
        assert_eq!(classifier.classify("install the dependencies with pnpm"), TaskType::Terminal);
    }

    #[test]
    fn test_classify_research() {
        let classifier = TaskClassifier;
        assert_eq!(classifier.classify("What is a monad?"), TaskType::Research);
        assert_eq!(classifier.classify("Search for Rust async patterns"), TaskType::Research);
        assert_eq!(classifier.classify("Look up the Tauri documentation"), TaskType::Research);
        assert_eq!(classifier.classify("Explain how async/await works"), TaskType::Research);
    }

    #[test]
    fn test_classify_edit() {
        let classifier = TaskClassifier;
        assert_eq!(classifier.classify("Add error handling to the parser"), TaskType::Edit);
        assert_eq!(classifier.classify("Refactor the auth module"), TaskType::Edit);
        assert_eq!(classifier.classify("Fix the login bug"), TaskType::Edit);
        assert_eq!(classifier.classify("Create a new route handler"), TaskType::Edit);
    }

    #[test]
    fn test_classify_planning() {
        let classifier = TaskClassifier;
        assert_eq!(classifier.classify("Design the architecture for a microservice"), TaskType::Planning);
        assert_eq!(classifier.classify("Plan the implementation of the auth system"), TaskType::Planning);
        assert_eq!(classifier.classify("Break down the task list for the API"), TaskType::Planning);
        assert_eq!(classifier.classify("What's the best approach for this?"), TaskType::Planning);
    }

    #[test]
    fn test_classify_vision() {
        let classifier = TaskClassifier;
        assert_eq!(classifier.classify("What does this screenshot show?"), TaskType::Vision);
        assert_eq!(classifier.classify("Look at this image and describe it"), TaskType::Vision);
        assert_eq!(classifier.classify("Analyze this UI mockup"), TaskType::Vision);
    }

    #[test]
    fn test_classify_review() {
        let classifier = TaskClassifier;
        assert_eq!(classifier.classify("Review this code for bugs"), TaskType::Review);
        assert_eq!(classifier.classify("Code review the auth module"), TaskType::Review);
        assert_eq!(classifier.classify("Verify the changes look correct"), TaskType::Review);
        assert_eq!(classifier.classify("Check my implementation"), TaskType::Review);
    }

    #[test]
    fn test_classify_general() {
        let classifier = TaskClassifier;
        assert_eq!(classifier.classify("Hello"), TaskType::General);
        assert_eq!(classifier.classify("What do you think about this code?"), TaskType::General);
    }

    #[test]
    fn test_terminal_vs_edit() {
        // "Build the feature" should be Edit, not Terminal
        let classifier = TaskClassifier;
        assert_eq!(classifier.classify("Build the authentication feature"), TaskType::Edit);
        // "Run build" should be Terminal
        assert_eq!(classifier.classify("Run cargo build"), TaskType::Terminal);
    }
}
