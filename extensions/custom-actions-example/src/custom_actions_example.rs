use zed_extension_api::{self as zed, Result};

struct CustomActionsExampleExtension;

impl zed::Extension for CustomActionsExampleExtension {
    fn new() -> Self {
        Self
    }

    fn run_action(&self, action: String, arguments: Vec<String>) -> Result<String, String> {
        match action.as_str() {
            "hello" => Ok("Hello from the custom actions example extension!".to_string()),
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

zed::register_extension!(CustomActionsExampleExtension);
