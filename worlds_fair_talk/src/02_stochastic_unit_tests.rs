// When AI enters the equation, we need a new approach
// We test AI features by sampling their behavior:

#[test]
fn eval_extract_handle_command_output() {
    eval(100, 0.98, "Extract method from run_git_blame", || async {
        let agent = EditAgent::new(model);
        let buffer = Buffer::local(ORIGINAL_CODE);

        // AI reads the file and performs the edit
        let (task, events) = agent.edit(
            buffer.clone(),
            "Extract handle_command_output method",
            &conversation,
        );

        let result = task.await?;
        let edited_code = buffer.read().text();

        // Check: Did it create the method?
        let has_method = edited_code.contains("fn handle_command_output");
        // Check: Did it take the right parameter?
        let has_param = edited_code.contains("output: Output");
        // Check: Did it remove code from original location?
        let removed_from_original = !edited_code.contains("match output.status.code()");

        Ok(has_method && has_param && removed_from_original)
    });
}

// Success rates across models:
// Claude-3.7-sonnet: 98/100 ✅
// GPT-4.1:          100/100 ✅
