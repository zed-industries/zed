use reqwest::StatusCode;
use rpc::LanguageModelProvider;

use crate::llm::LlmTokenClaims;
use crate::{Config, Error, Result};

pub fn authorize_access_to_language_model(
    config: &Config,
    claims: &LlmTokenClaims,
    country_code: Option<String>,
    provider: LanguageModelProvider,
    model: &str,
) -> Result<()> {
    authorize_access_for_country(config, country_code, provider)?;
    authorize_access_to_model(claims, provider, model)?;
    Ok(())
}

fn authorize_access_to_model(
    claims: &LlmTokenClaims,
    provider: LanguageModelProvider,
    model: &str,
) -> Result<()> {
    if claims.is_staff {
        return Ok(());
    }

    match (provider, model) {
        (LanguageModelProvider::Anthropic, model) if model.starts_with("claude-3-5-sonnet") => {
            Ok(())
        }
        _ => Err(Error::http(
            StatusCode::FORBIDDEN,
            format!("access to model {model:?} is not included in your plan"),
        ))?,
    }
}

fn authorize_access_for_country(
    config: &Config,
    country_code: Option<String>,
    provider: LanguageModelProvider,
) -> Result<()> {
    // In development we won't have the `CF-IPCountry` header, so we can't check
    // the country code.
    //
    // This shouldn't be necessary, as anyone running in development will need to provide
    // their own API credentials in order to use an LLM provider.
    if config.is_development() {
        return Ok(());
    }

    // https://developers.cloudflare.com/fundamentals/reference/http-request-headers/#cf-ipcountry
    let country_code = match country_code.as_deref() {
        // `XX` - Used for clients without country code data.
        None | Some("XX") => Err(Error::http(
            StatusCode::BAD_REQUEST,
            "no country code".to_string(),
        ))?,
        // `T1` - Used for clients using the Tor network.
        Some("T1") => Err(Error::http(
            StatusCode::FORBIDDEN,
            format!("access to {provider:?} models is not available over Tor"),
        ))?,
        Some(country_code) => country_code,
    };

    let is_country_supported_by_provider = match provider {
        LanguageModelProvider::Anthropic => anthropic::is_supported_country(country_code),
        LanguageModelProvider::OpenAi => open_ai::is_supported_country(country_code),
        LanguageModelProvider::Google => google_ai::is_supported_country(country_code),
        LanguageModelProvider::Zed => true,
    };
    if !is_country_supported_by_provider {
        Err(Error::http(
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            format!("access to {provider:?} models is not available in your region"),
        ))?
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::response::IntoResponse;
    use pretty_assertions::assert_eq;
    use rpc::proto::Plan;

    use super::*;

    #[gpui::test]
    async fn test_authorize_access_to_language_model_with_supported_country(
        _cx: &mut gpui::TestAppContext,
    ) {
        let config = Config::test();

        let claims = LlmTokenClaims {
            user_id: 99,
            plan: Plan::ZedPro,
            is_staff: true,
            ..Default::default()
        };

        let cases = vec![
            (LanguageModelProvider::Anthropic, "US"), // United States
            (LanguageModelProvider::Anthropic, "GB"), // United Kingdom
            (LanguageModelProvider::OpenAi, "US"),    // United States
            (LanguageModelProvider::OpenAi, "GB"),    // United Kingdom
            (LanguageModelProvider::Google, "US"),    // United States
            (LanguageModelProvider::Google, "GB"),    // United Kingdom
        ];

        for (provider, country_code) in cases {
            authorize_access_to_language_model(
                &config,
                &claims,
                Some(country_code.into()),
                provider,
                "the-model",
            )
            .unwrap_or_else(|_| {
                panic!("expected authorization to return Ok for {provider:?}: {country_code}")
            })
        }
    }

    #[gpui::test]
    async fn test_authorize_access_to_language_model_with_unsupported_country(
        _cx: &mut gpui::TestAppContext,
    ) {
        let config = Config::test();

        let claims = LlmTokenClaims {
            user_id: 99,
            plan: Plan::ZedPro,
            ..Default::default()
        };

        let cases = vec![
            (LanguageModelProvider::Anthropic, "AF"), // Afghanistan
            (LanguageModelProvider::Anthropic, "BY"), // Belarus
            (LanguageModelProvider::Anthropic, "CF"), // Central African Republic
            (LanguageModelProvider::Anthropic, "CN"), // China
            (LanguageModelProvider::Anthropic, "CU"), // Cuba
            (LanguageModelProvider::Anthropic, "ER"), // Eritrea
            (LanguageModelProvider::Anthropic, "ET"), // Ethiopia
            (LanguageModelProvider::Anthropic, "IR"), // Iran
            (LanguageModelProvider::Anthropic, "KP"), // North Korea
            (LanguageModelProvider::Anthropic, "XK"), // Kosovo
            (LanguageModelProvider::Anthropic, "LY"), // Libya
            (LanguageModelProvider::Anthropic, "MM"), // Myanmar
            (LanguageModelProvider::Anthropic, "RU"), // Russia
            (LanguageModelProvider::Anthropic, "SO"), // Somalia
            (LanguageModelProvider::Anthropic, "SS"), // South Sudan
            (LanguageModelProvider::Anthropic, "SD"), // Sudan
            (LanguageModelProvider::Anthropic, "SY"), // Syria
            (LanguageModelProvider::Anthropic, "VE"), // Venezuela
            (LanguageModelProvider::Anthropic, "YE"), // Yemen
            (LanguageModelProvider::OpenAi, "KP"),    // North Korea
            (LanguageModelProvider::Google, "KP"),    // North Korea
        ];

        for (provider, country_code) in cases {
            let error_response = authorize_access_to_language_model(
                &config,
                &claims,
                Some(country_code.into()),
                provider,
                "the-model",
            )
            .expect_err(&format!(
                "expected authorization to return an error for {provider:?}: {country_code}"
            ))
            .into_response();

            assert_eq!(
                error_response.status(),
                StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
            );
            let response_body = hyper::body::to_bytes(error_response.into_body())
                .await
                .unwrap()
                .to_vec();
            assert_eq!(
                String::from_utf8(response_body).unwrap(),
                format!("access to {provider:?} models is not available in your region")
            );
        }
    }

    #[gpui::test]
    async fn test_authorize_access_to_language_model_with_tor(_cx: &mut gpui::TestAppContext) {
        let config = Config::test();

        let claims = LlmTokenClaims {
            user_id: 99,
            plan: Plan::ZedPro,
            ..Default::default()
        };

        let cases = vec![
            (LanguageModelProvider::Anthropic, "T1"), // Tor
            (LanguageModelProvider::OpenAi, "T1"),    // Tor
            (LanguageModelProvider::Google, "T1"),    // Tor
            (LanguageModelProvider::Zed, "T1"),       // Tor
        ];

        for (provider, country_code) in cases {
            let error_response = authorize_access_to_language_model(
                &config,
                &claims,
                Some(country_code.into()),
                provider,
                "the-model",
            )
            .expect_err(&format!(
                "expected authorization to return an error for {provider:?}: {country_code}"
            ))
            .into_response();

            assert_eq!(error_response.status(), StatusCode::FORBIDDEN);
            let response_body = hyper::body::to_bytes(error_response.into_body())
                .await
                .unwrap()
                .to_vec();
            assert_eq!(
                String::from_utf8(response_body).unwrap(),
                format!("access to {provider:?} models is not available over Tor")
            );
        }
    }

    #[gpui::test]
    async fn test_authorize_access_to_language_model_based_on_plan() {
        let config = Config::test();

        let test_cases = vec![
            // Pro plan should have access to claude-3.5-sonnet
            (
                Plan::ZedPro,
                LanguageModelProvider::Anthropic,
                "claude-3-5-sonnet",
                true,
            ),
            // Free plan should have access to claude-3.5-sonnet
            (
                Plan::Free,
                LanguageModelProvider::Anthropic,
                "claude-3-5-sonnet",
                true,
            ),
            // Pro plan should NOT have access to other Anthropic models
            (
                Plan::ZedPro,
                LanguageModelProvider::Anthropic,
                "claude-3-opus",
                false,
            ),
        ];

        for (plan, provider, model, expected_access) in test_cases {
            let claims = LlmTokenClaims {
                plan,
                ..Default::default()
            };

            let result = authorize_access_to_language_model(
                &config,
                &claims,
                Some("US".into()),
                provider,
                model,
            );

            if expected_access {
                assert!(
                    result.is_ok(),
                    "Expected access to be granted for plan {:?}, provider {:?}, model {}",
                    plan,
                    provider,
                    model
                );
            } else {
                let error = result.expect_err(&format!(
                    "Expected access to be denied for plan {:?}, provider {:?}, model {}",
                    plan, provider, model
                ));
                let response = error.into_response();
                assert_eq!(response.status(), StatusCode::FORBIDDEN);
            }
        }
    }

    #[gpui::test]
    async fn test_authorize_access_to_language_model_for_staff() {
        let config = Config::test();

        let claims = LlmTokenClaims {
            is_staff: true,
            ..Default::default()
        };

        // Staff should have access to all models
        let test_cases = vec![
            (LanguageModelProvider::Anthropic, "claude-3-5-sonnet"),
            (LanguageModelProvider::Anthropic, "claude-2"),
            (LanguageModelProvider::Anthropic, "claude-123-agi"),
            (LanguageModelProvider::OpenAi, "gpt-4"),
            (LanguageModelProvider::Google, "gemini-pro"),
        ];

        for (provider, model) in test_cases {
            let result = authorize_access_to_language_model(
                &config,
                &claims,
                Some("US".into()),
                provider,
                model,
            );

            assert!(
                result.is_ok(),
                "Expected staff to have access to provider {:?}, model {}",
                provider,
                model
            );
        }
    }
}
