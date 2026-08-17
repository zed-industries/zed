use std::io::{self, IsTerminal, Read};

pub fn read_mermaid_or_default(example_name: &str, default_source: &str) -> io::Result<String> {
    let stdin = io::stdin();
    if stdin.is_terminal() {
        eprintln!(
            "{example_name}: no Mermaid source on stdin; using the built-in example. \
Pipe Mermaid source or redirect a .mmd file to render custom input."
        );
        return Ok(default_source.to_string());
    }

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    if input.trim().is_empty() {
        eprintln!("{example_name}: stdin was empty; using the built-in example.");
        Ok(default_source.to_string())
    } else {
        Ok(input)
    }
}
