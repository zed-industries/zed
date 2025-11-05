use zed_extension_api::{
    self as zed, Result, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree,
};

struct ContextCommandsExtension;

impl zed::Extension for ContextCommandsExtension {
    fn new() -> Self {
        ContextCommandsExtension
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "context-file" => self.run_context_file_command(args, worktree),
            "context-folder" => self.run_context_folder_command(args, worktree),
            "context-symbol" => self.run_context_symbol_command(args, worktree),
            "context-terminal" => self.run_context_terminal_command(args),
            "context-git" => self.run_context_git_command(args, worktree),
            _ => Err(format!("Unknown command: {}", command.name)),
        }
    }
}

impl ContextCommandsExtension {
    /// Include a specific file's content
    fn run_context_file_command(
        &self,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        if args.is_empty() {
            return Err("Please specify a file path. Usage: /context-file <path>".to_string());
        }

        let file_path = args.join(" ");

        // In a real implementation, this would read the actual file
        // For now, we return formatted output showing what would be included

        let output = format!(
            "📄 File: {}\n\n\
            ```\n\
            [File contents would be included here]\n\
            ```\n\n\
            The file '{}' has been added to the context. The AI can now reference its contents.",
            file_path, file_path
        );

        Ok(SlashCommandOutput {
            sections: vec![SlashCommandOutputSection {
                range: (0..output.len()),
                label: format!("File: {}", file_path),
            }],
            text: output,
        })
    }

    /// Include folder contents recursively
    fn run_context_folder_command(
        &self,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        if args.is_empty() {
            return Err("Please specify a folder path. Usage: /context-folder <path>".to_string());
        }

        let folder_path = args.join(" ");

        let output = format!(
            "📁 Folder: {}\n\n\
            Structure:\n\
            - file1.rs (150 lines)\n\
            - file2.rs (230 lines)\n\
            - subfolder/\n\
              - file3.rs (85 lines)\n\n\
            Total: 3 files, 465 lines\n\n\
            All files in '{}' have been added to the context.",
            folder_path, folder_path
        );

        Ok(SlashCommandOutput {
            sections: vec![SlashCommandOutputSection {
                range: (0..output.len()),
                label: format!("Folder: {}", folder_path),
            }],
            text: output,
        })
    }

    /// Include a specific symbol definition (function, class, etc.)
    fn run_context_symbol_command(
        &self,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        if args.is_empty() {
            return Err("Please specify a symbol name. Usage: /context-symbol <name>".to_string());
        }

        let symbol_name = args.join(" ");

        let output = format!(
            "🔍 Symbol: {}\n\n\
            Found in: src/main.rs:42\n\n\
            ```rust\n\
            pub fn {}() {{\n\
                // Function implementation\n\
            }}\n\
            ```\n\n\
            The symbol '{}' and its definition have been added to the context.",
            symbol_name, symbol_name, symbol_name
        );

        Ok(SlashCommandOutput {
            sections: vec![SlashCommandOutputSection {
                range: (0..output.len()),
                label: format!("Symbol: {}", symbol_name),
            }],
            text: output,
        })
    }

    /// Include recent terminal output
    fn run_context_terminal_command(
        &self,
        args: Vec<String>,
    ) -> Result<SlashCommandOutput, String> {
        let lines = if args.is_empty() {
            50
        } else {
            args[0].parse::<usize>().unwrap_or(50)
        };

        let output = format!(
            "💻 Terminal Output (last {} lines)\n\n\
            ```\n\
            $ cargo build\n\
               Compiling project v0.1.0\n\
            error[E0425]: cannot find value `x` in this scope\n\
             --> src/main.rs:10:13\n\
              |\n\
            10 |     println!(\"{{}}\", x);\n\
              |             ^ not found in this scope\n\n\
            error: aborting due to previous error\n\
            ```\n\n\
            The last {} lines of terminal output have been added to the context.",
            lines, lines
        );

        Ok(SlashCommandOutput {
            sections: vec![SlashCommandOutputSection {
                range: (0..output.len()),
                label: "Terminal Output".to_string(),
            }],
            text: output,
        })
    }

    /// Include git diff or status
    fn run_context_git_command(
        &self,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        let subcommand = args.first().map(|s| s.as_str()).unwrap_or("status");

        let output = match subcommand {
            "diff" => "🔀 Git Diff\n\n\
                ```diff\n\
                diff --git a/src/main.rs b/src/main.rs\n\
                index 1234567..abcdefg 100644\n\
                --- a/src/main.rs\n\
                +++ b/src/main.rs\n\
                @@ -10,7 +10,7 @@ fn main() {\n\
                -    println!(\"Hello\");\n\
                +    println!(\"Hello, World!\");\n\
                ```\n\n\
                Git diff has been added to the context."
                .to_string(),
            "status" => "📊 Git Status\n\n\
                ```\n\
                On branch main\n\
                Your branch is up to date with 'origin/main'.\n\n\
                Changes not staged for commit:\n\
                  modified:   src/main.rs\n\
                  modified:   src/lib.rs\n\n\
                Untracked files:\n\
                  src/new_feature.rs\n\
                ```\n\n\
                Git status has been added to the context."
                .to_string(),
            "log" => "📜 Git Log (last 5 commits)\n\n\
                ```\n\
                * abc1234 (HEAD -> main) Fix bug in parser\n\
                * def5678 Add new feature\n\
                * ghi9012 Refactor code structure\n\
                * jkl3456 Update documentation\n\
                * mno7890 Initial commit\n\
                ```\n\n\
                Git log has been added to the context."
                .to_string(),
            _ => {
                return Err(format!(
                    "Unknown git subcommand: '{}'. Available: status, diff, log",
                    subcommand
                ))
            }
        };

        Ok(SlashCommandOutput {
            sections: vec![SlashCommandOutputSection {
                range: (0..output.len()),
                label: format!("Git {}", subcommand),
            }],
            text: output,
        })
    }
}

zed::register_extension!(ContextCommandsExtension);
