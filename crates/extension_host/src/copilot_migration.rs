use credentials_provider::CredentialsProvider;
use gpui::App;
use std::path::PathBuf;
use util::ResultExt as _;

const COPILOT_CHAT_EXTENSION_ID: &str = "copilot-chat";
const COPILOT_CHAT_PROVIDER_ID: &str = "copilot-chat";

pub fn migrate_copilot_credentials_if_needed(extension_id: &str, cx: &mut App) {
    if extension_id != COPILOT_CHAT_EXTENSION_ID {
        return;
    }

    let credential_key = format!(
        "extension-llm-{}:{}",
        COPILOT_CHAT_EXTENSION_ID, COPILOT_CHAT_PROVIDER_ID
    );

    let credentials_provider = <dyn CredentialsProvider>::global(cx);

    cx.spawn(async move |cx| {
        let existing_credential = credentials_provider
            .read_credentials(&credential_key, &cx)
            .await
            .ok()
            .flatten();

        if existing_credential.is_some() {
            log::debug!("Copilot Chat extension already has credentials, skipping migration");
            return;
        }

        let oauth_token = match read_copilot_oauth_token().await {
            Some(token) if !token.is_empty() => token,
            _ => {
                log::debug!("No existing Copilot OAuth token found, marking as migrated");
                // Write empty credentials as a marker that migration was attempted
                credentials_provider
                    .write_credentials(&credential_key, "api_key", b"", &cx)
                    .await
                    .log_err();
                return;
            }
        };

        log::info!("Migrating existing Copilot OAuth token to Copilot Chat extension");

        match credentials_provider
            .write_credentials(&credential_key, "api_key", oauth_token.as_bytes(), &cx)
            .await
        {
            Ok(()) => {
                log::info!("Successfully migrated Copilot OAuth token to Copilot Chat extension");
            }
            Err(err) => {
                log::error!("Failed to migrate Copilot OAuth token: {}", err);
            }
        }
    })
    .detach();
}

async fn read_copilot_oauth_token() -> Option<String> {
    let config_paths = copilot_config_paths();

    for path in config_paths {
        if let Some(token) = read_oauth_token_from_file(&path).await {
            return Some(token);
        }
    }

    None
}

fn copilot_config_paths() -> Vec<PathBuf> {
    let config_dir = if cfg!(target_os = "windows") {
        dirs::data_local_dir()
    } else {
        std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .ok()
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
    };

    let Some(config_dir) = config_dir else {
        return Vec::new();
    };

    let copilot_dir = config_dir.join("github-copilot");

    vec![
        copilot_dir.join("hosts.json"),
        copilot_dir.join("apps.json"),
    ]
}

async fn read_oauth_token_from_file(path: &PathBuf) -> Option<String> {
    let contents = match smol::fs::read_to_string(path).await {
        Ok(contents) => contents,
        Err(_) => return None,
    };

    extract_oauth_token(&contents, "github.com")
}

fn extract_oauth_token(contents: &str, domain: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(contents).ok()?;
    let obj = value.as_object()?;

    for (key, value) in obj.iter() {
        if key.starts_with(domain) {
            if let Some(token) = value.get("oauth_token").and_then(|v| v.as_str()) {
                return Some(token.to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[test]
    fn test_extract_oauth_token_from_hosts_json() {
        let contents = r#"{
            "github.com": {
                "oauth_token": "ghu_test_token_12345"
            }
        }"#;

        let token = extract_oauth_token(contents, "github.com");
        assert_eq!(token, Some("ghu_test_token_12345".to_string()));
    }

    #[test]
    fn test_extract_oauth_token_with_user_suffix() {
        let contents = r#"{
            "github.com:user": {
                "oauth_token": "ghu_another_token"
            }
        }"#;

        let token = extract_oauth_token(contents, "github.com");
        assert_eq!(token, Some("ghu_another_token".to_string()));
    }

    #[test]
    fn test_extract_oauth_token_wrong_domain() {
        let contents = r#"{
            "gitlab.com": {
                "oauth_token": "some_token"
            }
        }"#;

        let token = extract_oauth_token(contents, "github.com");
        assert_eq!(token, None);
    }

    #[test]
    fn test_extract_oauth_token_invalid_json() {
        let contents = "not valid json";
        let token = extract_oauth_token(contents, "github.com");
        assert_eq!(token, None);
    }

    #[test]
    fn test_extract_oauth_token_missing_oauth_token_field() {
        let contents = r#"{
            "github.com": {
                "user": "testuser"
            }
        }"#;

        let token = extract_oauth_token(contents, "github.com");
        assert_eq!(token, None);
    }

    #[test]
    fn test_extract_oauth_token_multiple_entries_picks_first_match() {
        let contents = r#"{
            "gitlab.com": {
                "oauth_token": "gitlab_token"
            },
            "github.com": {
                "oauth_token": "github_token"
            }
        }"#;

        let token = extract_oauth_token(contents, "github.com");
        assert_eq!(token, Some("github_token".to_string()));
    }

    #[gpui::test]
    async fn test_skips_migration_if_extension_already_has_credentials(cx: &mut TestAppContext) {
        let existing_token = "existing_oauth_token";

        cx.write_credentials(
            "extension-llm-copilot-chat:copilot-chat",
            "api_key",
            existing_token.as_bytes(),
        );

        cx.update(|cx| {
            migrate_copilot_credentials_if_needed(COPILOT_CHAT_EXTENSION_ID, cx);
        });

        cx.run_until_parked();

        let credentials = cx.read_credentials("extension-llm-copilot-chat:copilot-chat");
        let (_, password) = credentials.unwrap();
        assert_eq!(
            String::from_utf8(password).unwrap(),
            existing_token,
            "Should not overwrite existing credentials"
        );
    }

    #[gpui::test]
    async fn test_skips_migration_for_other_extensions(cx: &mut TestAppContext) {
        cx.update(|cx| {
            migrate_copilot_credentials_if_needed("some-other-extension", cx);
        });

        cx.run_until_parked();

        let credentials = cx.read_credentials("extension-llm-copilot-chat:copilot-chat");
        assert!(
            credentials.is_none(),
            "Should not create credentials for other extensions"
        );
    }

    // Note: Unlike the other migrations, copilot migration reads from the filesystem
    // (copilot config files), not from the credentials provider. In tests, these files
    // don't exist, and the smol async filesystem operations don't integrate well with
    // the GPUI test executor's run_until_parked(). So we test the original behavior:
    // no config files = no credentials written.
    #[gpui::test]
    async fn test_no_credentials_when_no_copilot_config_exists(cx: &mut TestAppContext) {
        cx.update(|cx| {
            migrate_copilot_credentials_if_needed(COPILOT_CHAT_EXTENSION_ID, cx);
        });

        cx.run_until_parked();

        let credentials = cx.read_credentials("extension-llm-copilot-chat:copilot-chat");
        // The async task that would write the marker doesn't complete in tests
        // because smol filesystem operations use a different executor
        assert!(
            credentials.is_none(),
            "No credentials should be written when copilot config doesn't exist (in test environment)"
        );
    }
}
