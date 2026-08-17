use dialoguer::{theme::ColorfulTheme, Input};

fn main() {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your name")
        .interact_text()
        .unwrap();

    println!("Hello {}!", input);

    let mail: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your email")
        .validate_with({
            let mut force = None;
            move |input: &String| -> Result<(), &str> {
                if input.contains('@') || (force.as_ref() == Some(input)) {
                    Ok(())
                } else {
                    force = Some(input.clone());
                    Err("This is not a mail address; type the same value again to force use")
                }
            }
        })
        .interact_text()
        .unwrap();

    println!("Email: {}", mail);

    let mail: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your planet")
        .default("Earth".to_string())
        .interact_text()
        .unwrap();

    println!("Planet: {}", mail);

    let mail: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your galaxy")
        .with_initial_text("Milky Way".to_string())
        .interact_text()
        .unwrap();

    println!("Galaxy: {}", mail);
}
