use rpassword::prompt_password;

fn main() {
    println!("=== prompt_password() ===");
    match prompt_password("Password: ") {
        Ok(pass) => println!("You entered: '{}'", pass),
        Err(e) => eprintln!("Error: {}", e),
    }
}
