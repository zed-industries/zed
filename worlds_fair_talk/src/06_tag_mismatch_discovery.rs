// EVAL 2: Models kept messing up XML tags

#[test]
fn eval_tag_handling() {
    eval(100, 0.95, "Replace function body", || async {
        let buffer = Buffer::local(
            "fn calculate() {\n    todo!()\n}"
        );

        let (task, _) = agent.edit(
            buffer.clone(),
            "Replace todo with actual implementation",
            &conversation,
        );

        // Real examples from our test suite:

        // From Claude Sonnet 3.7:
        // <old_text>
        // a
        // b
        // c
        // </new_text>    ❌ Wrong closing tag!
        // <new_text>
        // a
        // B
        // c
        // </old_text>    ❌ Wrong again!

        // From Claude Opus 4:
        // <old_text>
        // Lorem
        // </old_text>
        // <new_text>
        // LOREM
        // </edits>       ❌ Forgot to close new_text!
    });
}

// Initial pass rate: 60%

// PROMPT FIX:
// Added: "Always close all tags properly"

// After prompt fix: 95% pass rate
// That last 5% wouldn't budge, so we made the parser forgiving:

if &self.buffer[tag_range.clone()] != OLD_TEXT_END_TAG {
    self.metrics.mismatched_tags += 1;
    // Keep parsing anyway - don't let bad XML stop us
}
