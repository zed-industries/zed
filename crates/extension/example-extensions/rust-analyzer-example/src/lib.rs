mod bindings;

struct Component;

impl bindings::Guest for Component {
    fn get_language_server_command() -> Result<bindings::Command, wit_bindgen::rt::string::String> {
        let _release = bindings::latest_github_release("rust-lang/rust-analyzer")?;

        Ok(bindings::Command {
            command: "path/to/rust-analyzer".to_string(),
            args: vec!["--stdio".into()],
            env: vec![],
        })
    }
}
