use anyhow::{anyhow, Result};
use std::fs;

pub fn get_dotenv_vars() -> Result<Vec<(String, String)>> {
    let mut vars = Vec::new();
    let env_content = fs::read_to_string("./crates/collab/.env.toml")
        .map_err(|_| anyhow!("no .env.toml file found"))?;

    add_vars(env_content, &mut vars)?;

    if let Ok(secret_content) = fs::read_to_string("./crates/collab/.env.secret.toml") {
        add_vars(secret_content, &mut vars)?;
    }

    Ok(vars)
}

pub fn load_dotenv() -> Result<()> {
    for (key, value) in get_dotenv_vars()? {
        std::env::set_var(key, value);
    }
    Ok(())
}

fn add_vars(env_content: String, vars: &mut Vec<(String, String)>) -> Result<()> {
    let env: toml::map::Map<String, toml::Value> = toml::de::from_str(&env_content)?;
    for (key, value) in env {
        let value = match value {
            toml::Value::String(value) => value,
            toml::Value::Integer(value) => value.to_string(),
            toml::Value::Float(value) => value.to_string(),
            _ => panic!("unsupported TOML value in .env.toml for key {}", key),
        };
        vars.push((key, value));
    }
    Ok(())
}
