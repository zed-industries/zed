use dialoguer::{theme::ColorfulTheme, History, Input};
use std::{collections::VecDeque, process};

fn main() {
    println!("Use 'exit' to quit the prompt");
    println!("In this example, history is limited to 4 entries");
    println!("Use the Up/Down arrows to scroll through history");
    println!();

    let mut history = MyHistory::default();

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

struct MyHistory {
    max: usize,
    history: VecDeque<String>,
}

impl Default for MyHistory {
    fn default() -> Self {
        MyHistory {
            max: 4,
            history: VecDeque::new(),
        }
    }
}

impl<T: ToString> History<T> for MyHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.history.get(pos).cloned()
    }

    fn write(&mut self, val: &T) {
        if self.history.len() == self.max {
            self.history.pop_back();
        }
        self.history.push_front(val.to_string());
    }
}
