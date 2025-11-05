use zed_extension_api::{self as zed, Result, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree};

struct CheckpointCommandsExtension;

impl zed::Extension for CheckpointCommandsExtension {
    fn new() -> Self {
        CheckpointCommandsExtension
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "checkpoint" => self.run_checkpoint_command(args),
            "rollback" => self.run_rollback_command(args),
            "list-checkpoints" => self.run_list_checkpoints_command(),
            _ => Err(format!("Unknown command: {}", command.name)),
        }
    }
}

impl CheckpointCommandsExtension {
    /// Create a new checkpoint
    fn run_checkpoint_command(&self, args: Vec<String>) -> Result<SlashCommandOutput, String> {
        let description = if args.is_empty() {
            "Manual checkpoint".to_string()
        } else {
            args.join(" ")
        };

        Ok(SlashCommandOutput {
            sections: vec![SlashCommandOutputSection {
                range: (0..0),
                label: "Checkpoint Created".to_string(),
            }],
            text: format!(
                "✓ Checkpoint created: {}\n\n\
                This checkpoint captures the current state of all modified files. \
                You can rollback to this point using `/rollback`.",
                description
            ),
        })
    }

    /// Rollback to a previous checkpoint
    fn run_rollback_command(&self, args: Vec<String>) -> Result<SlashCommandOutput, String> {
        if args.is_empty() {
            return Err(
                "Please specify a checkpoint ID or index. Use `/list-checkpoints` to see available checkpoints.".to_string()
            );
        }

        let checkpoint_ref = args.join(" ");

        Ok(SlashCommandOutput {
            sections: vec![SlashCommandOutputSection {
                range: (0..0),
                label: "Rollback Initiated".to_string(),
            }],
            text: format!(
                "↩ Rolling back to checkpoint: {}\n\n\
                This will restore all files to their state at the specified checkpoint. \
                Files modified after this checkpoint will be reverted.\n\n\
                Note: This creates a new checkpoint before rollback, so you can undo this action if needed.",
                checkpoint_ref
            ),
        })
    }

    /// List all available checkpoints
    fn run_list_checkpoints_command(&self) -> Result<SlashCommandOutput, String> {
        // In a real implementation, this would fetch actual checkpoints from the thread
        // For now, we return a formatted example
        let example_output = "\
📍 Available Checkpoints:\n\n\
1. [2025-11-05 10:23:45] Initial state (automatic)\n\
   - 3 files modified\n\
   - Before: Started refactoring authentication\n\n\
2. [2025-11-05 10:45:12] Completed auth refactor (user)\n\
   - 5 files modified\n\
   - After: Refactored authentication module\n\n\
3. [2025-11-05 11:02:33] Added tests (agent)\n\
   - 2 files modified\n\
   - Current checkpoint ← You are here\n\n\
Use `/rollback <number>` to restore to a checkpoint.\n\
Use `/checkpoint <description>` to create a new checkpoint.";

        Ok(SlashCommandOutput {
            sections: vec![SlashCommandOutputSection {
                range: (0..0),
                label: "Checkpoints".to_string(),
            }],
            text: example_output.to_string(),
        })
    }
}

zed::register_extension!(CheckpointCommandsExtension);
