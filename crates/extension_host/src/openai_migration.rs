use credentials_provider::CredentialsProvider;
use gpui::App;

const OPENAI_EXTENSION_ID: &str = "openai";
const OPENAI_PROVIDER_ID: &str = "openai";
const OPENAI_DEFAULT_API_URL: &str = "https://api.openai.com/v1";

/// Migrates OpenAI API credentials from the old built-in provider location
/// to the new extension-based location.
///
/// This should only be called during auto-install of the extension.
pub fn migrate_openai_credentials_if_needed(extension_id: &str, cx: &mut App) {
    if extension_id != OPENAI_EXTENSION_ID {
        return;
    }

    let extension_credential_key = format!(
        "extension-llm-{}:{}",
        OPENAI_EXTENSION_ID, OPENAI_PROVIDER_ID
    );

    let credentials_provider = <dyn CredentialsProvider>::global(cx);

    cx.spawn(async move |cx| {
        // Read from old location
        let old_credential = credentials_provider
            .read_credentials(OPENAI_DEFAULT_API_URL, &cx)
            .await
            .ok()
            .flatten();

        let api_key = match old_credential {
            Some((_, key_bytes)) => match String::from_utf8(key_bytes) {
                Ok(key) if !key.is_empty() => key,
                Ok(_) => {
                    log::debug!("Existing OpenAI API key is empty, nothing to migrate");
                    return;
                }
                Err(_) => {
                    log::error!("Failed to decode OpenAI API key as UTF-8");
                    return;
                }
            },
            None => {
                log::debug!("No existing OpenAI API key found to migrate");
                return;
            }
        };

        log::info!("Migrating existing OpenAI API key to OpenAI extension");

        match credentials_provider
            .write_credentials(&extension_credential_key, "Bearer", api_key.as_bytes(), &cx)
            .await
        {
            Ok(()) => {
                log::info!("Successfully migrated OpenAI API key to extension");
            }
            Err(err) => {
                log::error!("Failed to migrate OpenAI API key: {}", err);
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
        let api_key = "sk-test-key-12345";

        cx.write_credentials(OPENAI_DEFAULT_API_URL, "Bearer", api_key.as_bytes());

        cx.update(|cx| {
            migrate_openai_credentials_if_needed(OPENAI_EXTENSION_ID, cx);
        });

        cx.run_until_parked();

        let migrated = cx.read_credentials("extension-llm-openai:openai");
        assert!(migrated.is_some(), "Credentials should have been migrated");
        let (username, password) = migrated.unwrap();
        assert_eq!(username, "Bearer");
        assert_eq!(String::from_utf8(password).unwrap(), api_key);
    }

    #[gpui::test]
    async fn test_no_migration_if_no_old_credentials(cx: &mut TestAppContext) {
        cx.update(|cx| {
            migrate_openai_credentials_if_needed(OPENAI_EXTENSION_ID, cx);
        });

        cx.run_until_parked();

        let credentials = cx.read_credentials("extension-llm-openai:openai");
        assert!(
            credentials.is_none(),
            "Should not create credentials if none existed"
        );
    }

    #[gpui::test]
    async fn test_skips_migration_for_other_extensions(cx: &mut TestAppContext) {
        let api_key = "sk-test-key";

        cx.write_credentials(OPENAI_DEFAULT_API_URL, "Bearer", api_key.as_bytes());

        cx.update(|cx| {
            migrate_openai_credentials_if_needed("some-other-extension", cx);
        });

        cx.run_until_parked();

        let credentials = cx.read_credentials("extension-llm-openai:openai");
        assert!(
            credentials.is_none(),
            "Should not migrate for other extensions"
        );
    }
}
