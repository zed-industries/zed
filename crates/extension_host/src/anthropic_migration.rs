use credentials_provider::CredentialsProvider;
use gpui::App;

const ANTHROPIC_EXTENSION_ID: &str = "anthropic";
const ANTHROPIC_PROVIDER_ID: &str = "anthropic";
const ANTHROPIC_DEFAULT_API_URL: &str = "https://api.anthropic.com";

pub fn migrate_anthropic_credentials_if_needed(extension_id: &str, cx: &mut App) {
    if extension_id != ANTHROPIC_EXTENSION_ID {
        return;
    }

    let extension_credential_key = format!(
        "extension-llm-{}:{}",
        ANTHROPIC_EXTENSION_ID, ANTHROPIC_PROVIDER_ID
    );

    let credentials_provider = <dyn CredentialsProvider>::global(cx);

    cx.spawn(async move |cx| {
        let existing_credential = credentials_provider
            .read_credentials(&extension_credential_key, &cx)
            .await
            .ok()
            .flatten();

        if existing_credential.is_some() {
            log::debug!("Anthropic extension already has credentials, skipping migration");
            return;
        }

        let old_credential = credentials_provider
            .read_credentials(ANTHROPIC_DEFAULT_API_URL, &cx)
            .await
            .ok()
            .flatten();

        let api_key = match old_credential {
            Some((_, key_bytes)) => match String::from_utf8(key_bytes) {
                Ok(key) => key,
                Err(_) => {
                    log::error!("Failed to decode Anthropic API key as UTF-8");
                    return;
                }
            },
            None => {
                log::debug!("No existing Anthropic API key found to migrate");
                return;
            }
        };

        log::info!("Migrating existing Anthropic API key to Anthropic extension");

        match credentials_provider
            .write_credentials(&extension_credential_key, "Bearer", api_key.as_bytes(), &cx)
            .await
        {
            Ok(()) => {
                log::info!("Successfully migrated Anthropic API key to extension");
            }
            Err(err) => {
                log::error!("Failed to migrate Anthropic API key: {}", err);
            }
        }
    })
    .detach();
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_migrates_credentials_from_old_location(cx: &mut TestAppContext) {
        let api_key = "sk-ant-test-key-12345";

        cx.write_credentials(ANTHROPIC_DEFAULT_API_URL, "Bearer", api_key.as_bytes());

        cx.update(|cx| {
            migrate_anthropic_credentials_if_needed(ANTHROPIC_EXTENSION_ID, cx);
        });

        cx.run_until_parked();

        let migrated = cx.read_credentials("extension-llm-anthropic:anthropic");
        assert!(migrated.is_some(), "Credentials should have been migrated");
        let (username, password) = migrated.unwrap();
        assert_eq!(username, "Bearer");
        assert_eq!(String::from_utf8(password).unwrap(), api_key);
    }

    #[gpui::test]
    async fn test_skips_migration_if_extension_already_has_credentials(cx: &mut TestAppContext) {
        let old_api_key = "sk-ant-old-key";
        let existing_key = "sk-ant-existing-key";

        cx.write_credentials(ANTHROPIC_DEFAULT_API_URL, "Bearer", old_api_key.as_bytes());
        cx.write_credentials(
            "extension-llm-anthropic:anthropic",
            "Bearer",
            existing_key.as_bytes(),
        );

        cx.update(|cx| {
            migrate_anthropic_credentials_if_needed(ANTHROPIC_EXTENSION_ID, cx);
        });

        cx.run_until_parked();

        let credentials = cx.read_credentials("extension-llm-anthropic:anthropic");
        let (_, password) = credentials.unwrap();
        assert_eq!(
            String::from_utf8(password).unwrap(),
            existing_key,
            "Should not overwrite existing credentials"
        );
    }

    #[gpui::test]
    async fn test_skips_migration_if_no_old_credentials(cx: &mut TestAppContext) {
        cx.update(|cx| {
            migrate_anthropic_credentials_if_needed(ANTHROPIC_EXTENSION_ID, cx);
        });

        cx.run_until_parked();

        let credentials = cx.read_credentials("extension-llm-anthropic:anthropic");
        assert!(
            credentials.is_none(),
            "Should not create credentials if none existed"
        );
    }

    #[gpui::test]
    async fn test_skips_migration_for_other_extensions(cx: &mut TestAppContext) {
        let api_key = "sk-ant-test-key";

        cx.write_credentials(ANTHROPIC_DEFAULT_API_URL, "Bearer", api_key.as_bytes());

        cx.update(|cx| {
            migrate_anthropic_credentials_if_needed("some-other-extension", cx);
        });

        cx.run_until_parked();

        let credentials = cx.read_credentials("extension-llm-anthropic:anthropic");
        assert!(
            credentials.is_none(),
            "Should not migrate for other extensions"
        );
    }
}
