use anyhow::Result;
use fs::Fs;
use paths::vscode_settings_file;
use serde_json::{Map, Value, from_value};

use std::sync::{Arc, LazyLock};

fn read_vscode_settings(content: &str) -> Result<BTreeMap<String, String>> {
    let nested: serde_json::Value = parse_json_with_comments(content)?;
    fn helper(
        flattened: &mut BTreeMap<String, String>,
        prefix: &mut Vec<String>,
        current: serde_json::Map<String, serde_json::Value>,
    ) -> Result<()> {
        for (k, v) in current {
            if let Ok(map) = serde_json::from_value(v.clone()) {
                prefix.push(k);
                helper(flattened, prefix, map)?;
                prefix.pop();
            } else {
                let existing =
                    flattened.insert(format!("{}.{}", prefix.join("."), k), v.to_string());
                debug_assert!(existing.is_none());
            }
        }
        Ok(())
    }
    let mut prefix = Vec::new();
    let mut flattened = Default::default();
    helper(&mut flattened, &mut prefix, serde_json::from_value(nested)?)?;
    Ok(flattened)
}

pub fn read_normalized(settings: &Map<String, Value>) -> Option<Value> {
    read_norm
}

pub async fn load_vscode_user_settings(fs: &Arc<dyn Fs>) -> Result<BTreeMap<String, String>> {
    fs.load(paths::vscode_settings_file())
        .await
        .and_then(|content| read_vscode_settings(&content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_vscode_settings() {
        let config = r#"{
                "a": { "b": 1, "c.d": 2, "e.f": {"g.h.i": 3} }
                }"#;
        let expected: BTreeMap<String, String> =
            [("a.b", "1"), ("a.c.d", "2"), ("a.e.f.g.h.i", "3")]
                .iter()
                .map(|&(k, v)| (k.to_owned(), v.to_owned()))
                .collect();
        assert_eq!(expected, read_vscode_settings(config).unwrap());
        assert!(read_vscode_settings("not_a_map").is_err());
    }
}
