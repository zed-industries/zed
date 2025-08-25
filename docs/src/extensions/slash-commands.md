# Slash Commands

Extensions may provide slash commands for use in the Assistant.

## Example extension

To see a working example of an extension that provides slash commands, check out the [`slash-commands-example` extension](https://github.com/zed-industries/zed/tree/main/extensions/slash-commands-example).

This extension can be [installed as a dev extension](./developing-extensions.md#developing-an-extension-locally) if you want to try it out for yourself.

## Defining slash commands

A given extension may provide one or more slash commands. Each slash command must be registered in the `extension.toml`.

For example, here is an extension that provides two slash commands: `/echo` and `/pick-one`:

```toml
[slash_commands.echo]
description = "echoes the provided input"
requires_argument = true

[slash_commands.pick-one]
description = "pick one of three options"
requires_argument = true
```

Each slash command may define the following properties:

- `description`: A description of the slash command that will be shown when completing available commands.
- `requires_argument`: Indicates whether a slash command requires at least one argument to run.

## Implementing slash command behavior

To implement behavior for your slash commands, implement `run_slash_command` for your extension.

This method accepts the slash command that will be run, the list of arguments passed to it, and an optional `Worktree`.

This method returns `SlashCommandOutput`, which contains the textual output of the command in the `text` field. The output may also define `SlashCommandOutputSection`s that contain ranges into the output. These sections are then rendered as creases in the Assistant's context editor.

Your extension should `match` on the command name (without the leading `/`) and then execute behavior accordingly:

```rs
impl zed::Extension for MyExtension {
    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "echo" => {
                if args.is_empty() {
                    return Err("nothing to echo".to_string());
                }

                let text = args.join(" ");

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "Echo".to_string(),
                    }],
                    text,
                })
            }
            "pick-one" => {
                let Some(selection) = args.first() else {
                    return Err("no option selected".to_string());
                };

                match selection.as_str() {
                    "option-1" | "option-2" | "option-3" => {}
                    invalid_option => {
                        return Err(format!("{invalid_option} is not a valid option"));
                    }
                }

                let text = format!("You chose {selection}.");

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: format!("Pick One: {selection}"),
                    }],
                    text,
                })
            }
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }
}
```

## Auto-completing slash command arguments

For slash commands that have arguments, you may also choose to implement `complete_slash_command_argument` to provide completions for your slash commands.

This method accepts the slash command that will be run and the list of arguments passed to it. It returns a list of `SlashCommandArgumentCompletion`s that will be shown in the completion menu.

A `SlashCommandArgumentCompletion` consists of the following properties:

- `label`: The label that will be shown in the completion menu.
- `new_text`: The text that will be inserted when the completion is accepted.
- `run_command`: Whether the slash command will be run when the completion is accepted.

Once again, your extension should `match` on the command name (without the leading `/`) and return the desired argument completions:

```rs
impl zed::Extension for MyExtension {
    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "echo" => Ok(vec![]),
            "pick-one" => Ok(vec![
                SlashCommandArgumentCompletion {
                    label: "Option One".to_string(),
                    new_text: "option-1".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Option Two".to_string(),
                    new_text: "option-2".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Option Three".to_string(),
                    new_text: "option-3".to_string(),
                    run_command: true,
                },
            ]),
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }
}
```
