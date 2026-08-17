use rpassword::ConfigBuilder;
use std::io::{self, Write};

fn prompt(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap(); // need to flush because print!() doesn't flush
}

fn main() {
    println!("=== read_password_with_config(...) with Hide mode (default behavior) ===");
    let config = ConfigBuilder::new().password_feedback_hide().build();
    prompt("Password: ");
    match rpassword::read_password_with_config(config) {
        Ok(pass) => println!("You entered: '{}'", pass),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== read_password_with_config(...) with Mask('*') mode ===");
    let config = ConfigBuilder::new().password_feedback_mask('*').build();
    prompt("Password: ");
    match rpassword::read_password_with_config(config) {
        Ok(pass) => println!("You entered: '{}'", pass),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== read_password_with_config(...) with Mask('#') mode ===");
    let config = ConfigBuilder::new().password_feedback_mask('#').build();
    prompt("Password: ");
    match rpassword::read_password_with_config(config) {
        Ok(pass) => println!("You entered: '{}'", pass),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== read_password_with_config(...) with PartialMask('*', 3) mode ===");
    let config = ConfigBuilder::new()
        .password_feedback_partial_mask('*', 3)
        .build();
    prompt("Password: ");
    match rpassword::read_password_with_config(config) {
        Ok(pass) => println!("You entered: '{}'", pass),
        Err(e) => eprintln!("Error: {}", e),
    }
}
