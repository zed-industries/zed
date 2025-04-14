use anyhow::Result;
use fs::Fs;
use serde_json::{Map, Value};

use std::sync::Arc;

pub struct VSCodeSettings {
    content: Map<String, Value>,
}

impl VSCodeSettings {
    pub fn from_str(content: &str) -> Result<Self> {
        Ok(Self {
            content: serde_json::from_str(content)?,
        })
    }

    pub async fn load_user_settings(fs: Arc<dyn Fs>) -> Result<Self> {
        let content = fs.load(paths::vscode_settings_file()).await?;
        Ok(Self {
            content: serde_json::from_str(&content)?,
        })
    }

    pub fn read_value(&self, setting: &str) -> Option<&Value> {
        if let Some(value) = self.content.get(setting) {
            return Some(value);
        }
        // TODO: check if it's in [platform] settings for current platform
        // TODO: deal with language specific settings
        None
    }

    pub fn bool_setting(&self, key: &str, setting: &mut Option<bool>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_bool) {
            *setting = Some(s)
        }
    }

    pub fn i32_setting(&self, key: &str, setting: &mut Option<i32>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_i64) {
            *setting = Some(s as i32)
        }
    }

    pub fn i64_setting(&self, key: &str, setting: &mut Option<i64>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_i64) {
            *setting = Some(s)
        }
    }

    pub fn u32_setting(&self, key: &str, setting: &mut Option<u32>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_u64) {
            *setting = Some(s as u32)
        }
    }

    pub fn u64_setting(&self, key: &str, setting: &mut Option<u64>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_u64) {
            *setting = Some(s)
        }
    }

    pub fn f32_setting(&self, key: &str, setting: &mut Option<f32>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_f64) {
            *setting = Some(s as f32)
        }
    }

    pub fn f64_setting(&self, key: &str, setting: &mut Option<f64>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_f64) {
            *setting = Some(s)
        }
    }
}

// fn read_vscode_settings(content: &str) -> Result<BTreeMap<String, String>> {
//     let nested: serde_json::Value = parse_json_with_comments(content)?;
//     fn helper(
//         flattened: &mut BTreeMap<String, String>,
//         prefix: &mut Vec<String>,
//         current: serde_json::Map<String, serde_json::Value>,
//     ) -> Result<()> {
//         for (k, v) in current {
//             if let Ok(map) = serde_json::from_value(v.clone()) {
//                 prefix.push(k);
//                 helper(flattened, prefix, map)?;
//                 prefix.pop();
//             } else {
//                 let existing =
//                     flattened.insert(format!("{}.{}", prefix.join("."), k), v.to_string());
//                 debug_assert!(existing.is_none());
//             }
//         }
//         Ok(())
//     }
//     let mut prefix = Vec::new();
//     let mut flattened = Default::default();
//     helper(&mut flattened, &mut prefix, serde_json::from_value(nested)?)?;
//     Ok(flattened)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_flatten_vscode_settings() {
//         let config = r#"{
//                 "a": { "b": 1, "c.d": 2, "e.f": {"g.h.i": 3} }
//                 }"#;
//         let expected: BTreeMap<String, String> =
//             [("a.b", "1"), ("a.c.d", "2"), ("a.e.f.g.h.i", "3")]
//                 .iter()
//                 .map(|&(k, v)| (k.to_owned(), v.to_owned()))
//                 .collect();
//         assert_eq!(expected, read_vscode_settings(config).unwrap());
//         assert!(read_vscode_settings("not_a_map").is_err());
//     }
// }
