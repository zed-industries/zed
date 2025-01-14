use collab::env::get_dotenv_vars;

fn main() -> anyhow::Result<()> {
    for (key, value) in get_dotenv_vars(".")? {
        if option_env!("POWERSHELL").is_some() {
            println!("$env:{}=\"{}\"", key, value);
        } else {
            println!("export {}=\"{}\"", key, value);
        }
    }
    Ok(())
}
