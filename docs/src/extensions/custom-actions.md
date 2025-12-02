# Custom Actions

Extensions may provide custom actions that can be invoked from the command palette or via keybindings.

## Example extension

To see a working example of an extension that provides custom actions, check out the [`custom-actions-example` extension](https://github.com/zed-industries/zed/tree/main/extensions/custom-actions-example).

This extension can be [installed as a dev extension](./developing-extensions.md#developing-an-extension-locally) if you want to try it out for yourself.

## Defining custom actions

A given extension may provide one or more custom actions. Each custom action must be registered in the `extension.toml`.

For example, here is an extension that provides three custom actions: `hello`, `greet`, and `reverse_text`:

```toml
[custom_actions.hello]
description = "Prints a hello message"

[custom_actions.greet]
description = "Greets a person by name"

[custom_actions.reverse_text]
description = "Reverses the provided text"
```

Each custom action may define the following properties:

- `description`: A description of the custom action that will be shown in the command palette.

## Implementing custom action behavior

To implement behavior for your custom actions, implement `run_action` for your extension.

This method accepts the action name as a string and the list of arguments passed to it.

This method returns a `Result<String, String>`, where the `Ok` variant contains a success message that will be shown to the user in a toast notification, and the `Err` variant contains an error message.

Your extension should `match` on the action name and then execute behavior accordingly:

```rs
impl zed::Extension for MyExtension {
    fn run_action(&self, action: String, arguments: Vec<String>) -> Result<String, String> {
        match action.as_str() {
            "hello" => {
                Ok("Hello from the custom actions example extension!".to_string())
            }
            "greet" => {
                if arguments.is_empty() {
                    Err("The 'greet' action requires a name argument".to_string())
                } else {
                    let name = &arguments[0];
                    Ok(format!("Hello, {}! Welcome to Zed.", name))
                }
            }
            "reverse_text" => {
                if arguments.is_empty() {
                    Err("The 'reverse_text' action requires text to reverse".to_string())
                } else {
                    let text = arguments.join(" ");
                    let reversed: String = text.chars().rev().collect();
                    Ok(reversed)
                }
            }
            _ => Err(format!("Unknown action: {}", action)),
        }
    }
}
```

## Using custom actions

Once an extension with custom actions is installed, the actions will appear in the command palette with the format:

```
<extension-id>: <action-name> (extension)
```

For example, the actions from the example above (with extension ID `custom-actions-example`) would appear as:

- `custom actions example: hello` (extension)
- `custom actions example: greet` (extension)
- `custom actions example: reverse text` (extension)

Note: Underscores and hyphens in extension IDs and action names are automatically replaced with spaces for better readability in the command palette. The actual extension ID is still `custom-actions-example` and the action is `reverse_text`.

Users can invoke these actions from the command palette. When an action completes successfully, a toast notification will display the result message. If an action fails, an error toast will be shown with the error message.

## Binding custom actions to keymaps

Custom actions can be bound to keyboard shortcuts in your keymap file. The action name format in keymaps is:

```
extension::{extension-id}::{action-name}
```

For example, to bind the `hello` action from the `custom-actions-example` extension to `ctrl-shift-h`:

```json
{
  "bindings": {
    "ctrl-shift-h": "extension::custom-actions-example::hello"
  }
}
```

You can also pass arguments to custom actions in keymaps using the array format:

```json
{
  "bindings": {
    "ctrl-shift-g": ["extension::custom-actions-example::greet", "World"]
  }
}
```

The second element in the array is the list of arguments that will be passed to the action. Arguments can be:
- A single string: `"argument"`
- Multiple strings: `["arg1", "arg2", "arg3"]`

## Best practices

- Keep action names short and descriptive, using snake_case (e.g., `reverse_text` instead of `reverseText`)
- Provide clear, helpful error messages when actions fail or when required arguments are missing
- Return concise success messages that confirm what the action accomplished
- Always handle the case for unknown actions by returning an error
- Validate arguments before processing them to provide better user feedback
