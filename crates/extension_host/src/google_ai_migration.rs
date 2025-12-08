use credentials_provider::CredentialsProvider;
use gpui::App;

const GOOGLE_AI_EXTENSION_ID: &str = "google-ai";
const GOOGLE_AI_PROVIDER_ID: &str = "google-ai";
const GOOGLE_AI_DEFAULT_API_URL: &str = "https://generativelanguage.googleapis.com";

pub fn migrate_google_ai_credentials_if_needed(extension_id: &str, cx: &mut App) {
    if extension_id != GOOGLE_AI_EXTENSION_ID {
        return;
    }

    let extension_credential_key = format!(
        "extension-llm-{}:{}",
        GOOGLE_AI_EXTENSION_ID, GOOGLE_AI_PROVIDER_ID
    );

    let credentials_provider = <dyn CredentialsProvider>::global(cx);

    cx.spawn(async move |cx| {
        let existing_credential = credentials_provider
            .read_credentials(&extension_credential_key, &cx)
            .await
            .ok()
            .flatten();

        if existing_credential.is_some() {
            log::debug!("Google AI extension already has credentials, skipping migration");
            return;
        }

        let old_credential = credentials_provider
            .read_credentials(GOOGLE_AI_DEFAULT_API_URL, &cx)
            .await
            .ok()
            .flatten();

        let api_key = match old_credential {
            Some((_, key_bytes)) => match String::from_utf8(key_bytes) {
                Ok(key) => key,
                Err(_) => {
                    log::error!("Failed to decode Google AI API key as UTF-8");
                    return;
                }
            },
            None => {
                log::debug!("No existing Google AI API key found to migrate");
                return;
            }
        };

        log::info!("Migrating existing Google AI API key to Google AI extension");

        match credentials_provider
            .write_credentials(&extension_credential_key, "Bearer", api_key.as_bytes(), &cx)
            .await
        {
            Ok(()) => {
                log::info!("Successfully migrated Google AI API key to extension");
            }
            Err(err) => {
                log::error!("Failed to migrate Google AI API key: {}", err);
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
        let api_key = "AIzaSy-test-key-12345";

        cx.write_credentials(GOOGLE_AI_DEFAULT_API_URL, "Bearer", api_key.as_bytes());

        cx.update(|cx| {
            migrate_google_ai_credentials_if_needed(GOOGLE_AI_EXTENSION_ID, cx);
        });

        cx.run_until_parked();

        let migrated = cx.read_credentials("extension-llm-google-ai:google-ai");
        assert!(migrated.is_some(), "Credentials should have been migrated");
        let (username, password) = migrated.unwrap();
        assert_eq!(username, "Bearer");
        assert_eq!(String::from_utf8(password).unwrap(), api_key);
    }

    #[gpui::test]
    async fn test_skips_migration_if_extension_already_has_credentials(cx: &mut TestAppContext) {
        let old_api_key = "AIzaSy-old-key";
        let existing_key = "AIzaSy-existing-key";

        cx.write_credentials(GOOGLE_AI_DEFAULT_API_URL, "Bearer", old_api_key.as_bytes());
        cx.write_credentials(
            "extension-llm-google-ai:google-ai",
            "Bearer",
            existing_key.as_bytes(),
        );

        cx.update(|cx| {
            migrate_google_ai_credentials_if_needed(GOOGLE_AI_EXTENSION_ID, cx);
        });

        cx.run_until_parked();

        let credentials = cx.read_credentials("extension-llm-google-ai:google-ai");
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
            migrate_google_ai_credentials_if_needed(GOOGLE_AI_EXTENSION_ID, cx);
        });

        cx.run_until_parked();

        let credentials = cx.read_credentials("extension-llm-google-ai:google-ai");
        assert!(
            credentials.is_none(),
            "Should not create credentials if none existed"
        );
    }

    #[gpui::test]
    async fn test_skips_migration_for_other_extensions(cx: &mut TestAppContext) {
        let api_key = "AIzaSy-test-key";

        cx.write_credentials(GOOGLE_AI_DEFAULT_API_URL, "Bearer", api_key.as_bytes());

        cx.update(|cx| {
            migrate_google_ai_credentials_if_needed("some-other-extension", cx);
        });

        cx.run_until_parked();

        let credentials = cx.read_credentials("extension-llm-google-ai:google-ai");
        assert!(
            credentials.is_none(),
            "Should not migrate for other extensions"
        );
    }
}
