use dialoguer::{theme::ColorfulTheme, BasicHistory, Input};
use std::process;

fn main() {
    println!("Use 'exit' to quit the prompt");
    println!("In this example, history is limited to 8 entries and contains no duplicates");
    println!("Use the Up/Down arrows to scroll through history");
    println!();

    let mut history = BasicHistory::new().max_entries(8).no_duplicates(true);

    loop {
        if let Ok(cmd) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("dialoguer")
            .history_with(&mut history)
            .interact_text()
        {
            if cmd == "exit" {
                process::exit(0);
            }
            println!("Entered {}", cmd);
        }
    }
}
