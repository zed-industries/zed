use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    size: Option<u32>,
}

fn main() {
    match envy::from_env::<Config>() {
        Ok(config) => println!("provided config.size {:?}", config.size),
        Err(err) => println!("error parsing config from env: {}", err),
    }
}
