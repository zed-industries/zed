use collab::env::get_dotenv_vars;

fn main() -> anyhow::Result<()> {
    for (key, value) in get_dotenv_vars(".")? {
        println!("export {}=\"{}\"", key, value);
    }
    Ok(())
}
