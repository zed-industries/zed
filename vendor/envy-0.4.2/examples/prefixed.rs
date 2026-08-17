use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    bar: Option<String>,
}

fn main() {
    match envy::prefixed("FOO_").from_env::<Config>() {
        Ok(config) => println!("provided config.bar {:?}", config.bar),
        Err(err) => println!("error parsing config from env: {}", err),
    }
}
