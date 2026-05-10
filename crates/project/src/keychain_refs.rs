use std::sync::Arc;

use anyhow::{Context as _, Result};
use credentials_provider::CredentialsProvider;
use gpui::AsyncApp;
use serde_json::Value;

const KEYCHAIN_REF_KEY: &str = "$keychain";

pub const KEYCHAIN_URL_PREFIX: &str = "zed://context_servers/";

pub fn keychain_url_for(name: &str) -> String {
    format!("{KEYCHAIN_URL_PREFIX}{name}")
}

#[derive(Debug)]
pub struct MissingKeychainEntry {
    pub name: String,
    pub url: String,
}

impl std::fmt::Display for MissingKeychainEntry {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "keychain entry `{}` is not set (storage URL: {})",
            self.name, self.url
        )
    }
}

impl std::error::Error for MissingKeychainEntry {}

/// Walk a settings JSON value and replace every `{ "$keychain": "<name>" }`
/// object with the secret stored at `zed://context_servers/<name>` in the
/// system keychain. Cleartext values pass through unchanged.
///
/// Objects with any key alongside `$keychain` are treated as ordinary objects
/// and recursed into, so user JSON that happens to include a `$keychain` key
/// is never accidentally consumed.
///
/// Returns an `anyhow::Error` carrying [`MissingKeychainEntry`] when a
/// referenced secret has not been provisioned. Callers that want to react
/// specifically to the missing-secret case can downcast.
pub async fn resolve_keychain_refs(
    value: &mut Value,
    credentials_provider: &Arc<dyn CredentialsProvider>,
    cx: &AsyncApp,
) -> Result<()> {
    match value {
        Value::Object(map) => {
            if let Some(name) = keychain_ref_name(map) {
                let secret = read_secret(name, credentials_provider, cx).await?;
                *value = Value::String(secret);
                return Ok(());
            }
            for nested in map.values_mut() {
                Box::pin(resolve_keychain_refs(nested, credentials_provider, cx)).await?;
            }
        }
        Value::Array(items) => {
            for item in items {
                Box::pin(resolve_keychain_refs(item, credentials_provider, cx)).await?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn keychain_ref_name(map: &serde_json::Map<String, Value>) -> Option<&str> {
    if map.len() != 1 {
        return None;
    }
    map.get(KEYCHAIN_REF_KEY).and_then(Value::as_str)
}

async fn read_secret(
    name: &str,
    credentials_provider: &Arc<dyn CredentialsProvider>,
    cx: &AsyncApp,
) -> Result<String> {
    let url = keychain_url_for(name);
    let stored = credentials_provider
        .read_credentials(&url, cx)
        .await
        .with_context(|| format!("failed to read keychain entry `{name}`"))?;
    let (_username, password_bytes) = stored.ok_or_else(|| {
        anyhow::Error::new(MissingKeychainEntry {
            name: name.to_string(),
            url: url.clone(),
        })
    })?;
    String::from_utf8(password_bytes)
        .with_context(|| format!("keychain entry `{name}` is not valid UTF-8"))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Mutex;

    use gpui::TestAppContext;
    use serde_json::json;

    use super::*;

    struct FakeCredentialsProvider {
        entries: Mutex<HashMap<String, Vec<u8>>>,
    }

    impl FakeCredentialsProvider {
        fn new(entries: &[(&str, &[u8])]) -> Arc<dyn CredentialsProvider> {
            Arc::new(Self {
                entries: Mutex::new(
                    entries
                        .iter()
                        .map(|(key, bytes)| (key.to_string(), bytes.to_vec()))
                        .collect(),
                ),
            })
        }
    }

    impl CredentialsProvider for FakeCredentialsProvider {
        fn read_credentials<'a>(
            &'a self,
            url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>> {
            let url = url.to_string();
            Box::pin(async move {
                let entries = self
                    .entries
                    .lock()
                    .map_err(|_| anyhow::anyhow!("fake credentials lock poisoned"))?;
                Ok(entries
                    .get(&url)
                    .cloned()
                    .map(|bytes| ("zed".to_string(), bytes)))
            })
        }

        fn write_credentials<'a>(
            &'a self,
            _url: &'a str,
            _username: &'a str,
            _password: &'a [u8],
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            Box::pin(async { Ok(()) })
        }

        fn delete_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[gpui::test]
    async fn replaces_top_level_ref(cx: &mut TestAppContext) {
        let cx = cx.to_async();
        let credentials =
            FakeCredentialsProvider::new(&[("zed://context_servers/api_key", b"secret-token")]);
        let mut value = json!({ "$keychain": "api_key" });
        resolve_keychain_refs(&mut value, &credentials, &cx)
            .await
            .unwrap();
        assert_eq!(value, json!("secret-token"));
    }

    #[gpui::test]
    async fn replaces_nested_ref(cx: &mut TestAppContext) {
        let cx = cx.to_async();
        let credentials = FakeCredentialsProvider::new(&[(
            "zed://context_servers/github_pat",
            b"ghp_real_value",
        )]);
        let mut value = json!({
            "config": {
                "auth": { "token": { "$keychain": "github_pat" } },
                "timeout": 30
            }
        });
        resolve_keychain_refs(&mut value, &credentials, &cx)
            .await
            .unwrap();
        assert_eq!(
            value,
            json!({
                "config": {
                    "auth": { "token": "ghp_real_value" },
                    "timeout": 30
                }
            })
        );
    }

    #[gpui::test]
    async fn passes_through_when_no_refs(cx: &mut TestAppContext) {
        let cx = cx.to_async();
        let credentials = FakeCredentialsProvider::new(&[]);
        let original = json!({ "key": "literal", "n": 1, "items": [1, 2, 3] });
        let mut value = original.clone();
        resolve_keychain_refs(&mut value, &credentials, &cx)
            .await
            .unwrap();
        assert_eq!(value, original);
    }

    #[gpui::test]
    async fn missing_entry_returns_typed_error(cx: &mut TestAppContext) {
        let cx = cx.to_async();
        let credentials = FakeCredentialsProvider::new(&[]);
        let mut value = json!({ "$keychain": "nonexistent" });
        let err = resolve_keychain_refs(&mut value, &credentials, &cx)
            .await
            .expect_err("missing entry must error");
        let missing = err
            .downcast_ref::<MissingKeychainEntry>()
            .expect("error must downcast to MissingKeychainEntry");
        assert_eq!(missing.name, "nonexistent");
        assert_eq!(missing.url, "zed://context_servers/nonexistent");
    }

    #[gpui::test]
    async fn object_with_extra_keys_is_not_a_ref(cx: &mut TestAppContext) {
        let cx = cx.to_async();
        let credentials = FakeCredentialsProvider::new(&[]);
        let mut value = json!({
            "$keychain": "should_be_ignored",
            "comment": "this is data, not a ref"
        });
        let original = value.clone();
        resolve_keychain_refs(&mut value, &credentials, &cx)
            .await
            .unwrap();
        assert_eq!(value, original);
    }

    #[gpui::test]
    async fn ref_inside_array_is_resolved(cx: &mut TestAppContext) {
        let cx = cx.to_async();
        let credentials =
            FakeCredentialsProvider::new(&[("zed://context_servers/array_secret", b"resolved")]);
        let mut value = json!([
            "literal",
            { "$keychain": "array_secret" }
        ]);
        resolve_keychain_refs(&mut value, &credentials, &cx)
            .await
            .unwrap();
        assert_eq!(value, json!(["literal", "resolved"]));
    }
}
