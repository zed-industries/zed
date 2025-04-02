use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;

pub fn get_dotenv_vars(current_dir: impl AsRef<Path>) -> Result<Vec<(String, String)>> {
    let current_dir = current_dir.as_ref();

    let mut vars = Vec::new();
    let env_content = fs::read_to_string(current_dir.join(".env.toml"))
        .map_err(|_| anyhow!("no .env.toml file found"))?;

    add_vars(env_content, &mut vars)?;

    if let Ok(secret_content) = fs::read_to_string(current_dir.join(".env.secret.toml")) {
        add_vars(secret_content, &mut vars)?;
    }

    Ok(vars)
}

pub fn load_dotenv() -> Result<()> {
    for (key, value) in get_dotenv_vars("./crates/collab")? {
        unsafe { std::env::set_var(key, value) };
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
            toml::Value::Boolean(value) => value.to_string(),
            _ => panic!("unsupported TOML value in .env.toml for key {}", key),
        };
        vars.push((key, value));
    }
    Ok(())
}
