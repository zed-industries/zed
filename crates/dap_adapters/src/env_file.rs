use anyhow::Result;
use collections::HashMap;
use fs::Fs;
use log::warn;
use serde_json::{Map, Value};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

/// Expands a VS Code-style `envFile` into concrete environment variables.
///
/// Loaded values are merged into both the outgoing DAP configuration's `env` object and the
/// adapter process environment map. Explicit values already present in `config["env"]` override
/// values loaded from `envFile` in the DAP configuration.
///
/// Relative `envFile` paths are resolved against `cwd`. After processing, the raw `envFile`
/// attribute is removed so adapters that do not support it do not receive it unchanged.
pub(crate) async fn apply_env_file(
    config: &mut Map<String, Value>,
    envs: &mut HashMap<String, String>,
    cwd: Option<&Path>,
    fs: Arc<dyn Fs>,
    adapter_name: &'static str,
) -> Result<()> {
    let Some(env_file) = config.get("envFile") else {
        return Ok(());
    };

    let env_files = match env_file {
        Value::Array(values) => values
            .iter()
            .map(|value| value.as_str())
            .collect::<Vec<_>>(),
        Value::String(value) => vec![Some(value.as_str())],
        _ => return Ok(()),
    };

    let rebase_path = |path: PathBuf| {
        if path.is_absolute() {
            Some(path)
        } else {
            cwd.map(|base_path| base_path.join(path))
        }
    };

    let mut env_file_vars = HashMap::default();
    for path in env_files {
        let Some(path) = path
            .and_then(|value| PathBuf::from_str(value).ok())
            .and_then(rebase_path)
        else {
            continue;
        };

        if let Ok(file) = fs.open_sync(&path).await {
            let file_envs: HashMap<String, String> = dotenvy::from_read_iter(file)
                .filter_map(Result::ok)
                .collect();
            envs.extend(
                file_envs
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone())),
            );
            env_file_vars.extend(file_envs);
        } else {
            warn!("While starting {adapter_name} debug session: failed to read env file {path:?}");
        }
    }

    let mut env_obj = serde_json::Map::new();
    for (key, value) in env_file_vars {
        env_obj.insert(key, Value::String(value));
    }

    if let Some(existing_env) = config.get("env").and_then(|value| value.as_object()) {
        for (key, value) in existing_env {
            env_obj.insert(key.clone(), value.clone());
        }
    }

    if !env_obj.is_empty() {
        config.insert("env".to_string(), Value::Object(env_obj));
    }

    config.remove("envFile");
    Ok(())
}
