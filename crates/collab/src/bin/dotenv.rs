use anyhow::anyhow;
use std::fs;

fn main() -> anyhow::Result<()> {
    let env: toml::map::Map<String, toml::Value> = toml::de::from_str(
        &fs::read_to_string("./.env.toml").map_err(|_| anyhow!("no .env.toml file found"))?,
    )?;

    for (key, value) in env {
        let value = match value {
            toml::Value::String(value) => value,
            toml::Value::Integer(value) => value.to_string(),
            toml::Value::Float(value) => value.to_string(),
            _ => panic!("unsupported TOML value in .env.toml for key {}", key),
        };
        println!("export {}=\"{}\"", key, value);
    }

    Ok(())
}
