use rpassword::ConfigBuilder;

fn main() {
    println!("=== prompt_password_with_config(...) with no output ===");
    println!("Note: No prompt will be shown, since the output is discarded.");
    println!("Just type your password and press Enter.");
    let config = ConfigBuilder::new()
        .password_feedback_hide()
        .output_discard()
        .build();
    match rpassword::prompt_password_with_config("Password: ", config) {
        Ok(pass) => println!("You entered: '{}'", pass),
        Err(e) => eprintln!("Error: {}", e),
    }
}
