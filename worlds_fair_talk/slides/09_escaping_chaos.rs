// EVAL 4: Models kept escaping characters inside XML tags

#[test]
fn eval_string_escaping() {
    eval(100, 0.95, "Edit string with quotes", || async {
        let buffer = Buffer::local(
            r#"let msg = "Hello, world!";"#
        );

        // Ask to change the message
        let (task, _) = agent.edit(
            buffer.clone(),
            "Change message to include user's name",
            &conversation,
        );

        // What models would generate:
        // <old_text>let msg = &quot;Hello, world!&quot;;</old_text>
        // Or worse:
        // <old_text>let msg = \"Hello, world!\";</old_text>

        // Fuzzy matcher can't find escaped version in buffer!
    });
}

// Even more chaos with newlines:
// Buffer:    "fn test() {\n    println!(\"hi\");\n}"
// Model:     "<old_text>fn test() {\\n    println!(\\"hi\\");\\n}</old_text>"

// This was particularly bad for Gemini (only 35% pass rate!)

// PROMPT FIX (from commit ab017129d8):
// Added: "Do not escape quotes, newlines, or other characters within tags"

// Impact across models:
// Gemini-2.5-pro: 35% → 86% (massive!)
// Claude-3.7:     96% → 98%
// GPT-4.1:        81% → 100%

// One line fixed Gemini's worst failure mode
