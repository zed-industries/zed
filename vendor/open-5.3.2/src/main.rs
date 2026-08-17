use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let path_or_url = match args.nth(1) {
        Some(arg) => arg,
        None => return Err("usage: open <path-or-url> [--with|-w program]".into()),
    };

    match args.next() {
        Some(arg) if arg == "--with" || arg == "-w" => {
            let program = args
                .next()
                .ok_or("--with must be followed by the program to use for opening")?;
            open::with(&path_or_url, program)
        }
        Some(arg) => return Err(format!("Argument '{arg}' is invalid").into()),
        None => open::that(&path_or_url),
    }?;

    println!("Opened '{}' successfully.", path_or_url);
    Ok(())
}
