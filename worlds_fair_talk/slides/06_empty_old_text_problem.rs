// EVAL 1: The empty old_text bug

#[test]
fn eval_insert_at_beginning() {
    eval(100, 0.95, "Add import at start of file", || async {
        let buffer = Buffer::local(
            "fn main() {\n    println!(\"Hello\");\n}"
        );

        let (task, _) = agent.edit(
            buffer.clone(),
            "Add 'use std::fs;' import at the beginning",
            &conversation,
        );

        // What models were generating:
        // <old_text></old_text>
        // <new_text>use std::fs;\n\n</new_text>

        // The bug we discovered:
        let matcher = StreamingFuzzyMatcher::new(buffer.snapshot());
        let range = matcher.find_text("");  // Empty string
        assert_eq!(range, Some(0..0));  // Matches at position 0

        // But empty string also matches at position 1, 2, 3...
        // Result: Import inserted multiple times randomly!

        // Actual output: "ufn use std::fs;\n\nmain() {\n  use std::fs;\n\n  println!(\"Hello\");\n}"
    });
}

// Pass rate: 0% - completely broken

// PROMPT FIX:
// Added to instructions: "`<old_text>` cannot be empty"

// After fix: 99% pass rate
