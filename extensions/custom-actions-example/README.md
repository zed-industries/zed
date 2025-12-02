# Custom Actions Example Extension

This extension demonstrates how to implement custom actions in Zed extensions.

## What are Custom Actions?

Custom actions allow extensions to register global actions that can be invoked through:
- The command palette
- Keybindings
- Other Zed features

## Actions Provided

This example extension registers three custom actions:

### `hello`
Prints a simple hello message.

**Usage:**
```
Action: custom-actions-example::hello
Arguments: (none)
Output: "Hello from the custom actions example extension!"
```

### `greet`
Greets a person by name.

**Usage:**
```
Action: custom-actions-example::greet
Arguments: [name]
Output: "Hello, {name}! Welcome to Zed."
```

**Example:**
```
Arguments: ["Alice"]
Output: "Hello, Alice! Welcome to Zed."
```

### `reverse_text`
Reverses the provided text.

**Usage:**
```
Action: custom-actions-example::reverse_text
Arguments: [text...]
Output: Reversed text
```

**Example:**
```
Arguments: ["Hello", "World"]
Output: "dlroW olleH"
```

## Implementation

### Declaring Actions

Actions are declared in `extension.toml`:

```toml
[custom_actions.hello]
description = "Prints a hello message"

[custom_actions.greet]
description = "Greets a person by name"

[custom_actions.reverse_text]
description = "Reverses the provided text"
```

### Implementing Actions

Actions are implemented by overriding the `run_action` method in the `Extension` trait:

```rust
impl zed::Extension for CustomActionsExampleExtension {
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

## Building

To build this extension:

```bash
cd extensions/custom-actions-example
cargo build --target wasm32-wasip2 --release
```

## Testing

Once loaded in Zed, the actions will be available in the command palette with the prefix `custom-actions-example::`.

For example, you can invoke:
- `custom-actions-example::hello`
- `custom-actions-example::greet`
- `custom-actions-example::reverse_text`

## API Version

This extension requires Zed extension API v0.6.0 or later, as custom actions were introduced in that version.
