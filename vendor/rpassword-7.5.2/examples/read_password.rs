use rpassword::read_password;
use std::io::{self, Write};

fn prompt(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap(); // need to flush because print!() doesn't flush
}

fn main() {
    println!("=== read_password() ===");
    prompt("Password: ");
    match read_password() {
        Ok(pass) => println!("You entered: '{}'", pass),
        Err(e) => eprintln!("Error: {}", e),
    }
}
