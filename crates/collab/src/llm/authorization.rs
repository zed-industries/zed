use anyhow::anyhow;
use reqwest::StatusCode;
use rpc::LanguageModelProvider;

use crate::llm::LlmTokenClaims;
use crate::{Config, Error, Result};

pub fn authorize_access_to_language_model(
    config: &Config,
    _claims: LlmTokenClaims,
    country_code: Option<String>,
    provider: LanguageModelProvider,
    model: &str,
) -> Result<()> {
    authorize_access_for_country(config, country_code, provider, model)?;

    Ok(())
}

fn authorize_access_for_country(
    config: &Config,
    country_code: Option<String>,
    provider: LanguageModelProvider,
    _model: &str,
) -> Result<()> {
    // In development we won't have the `CF-IPCountry` header, so we can't check
    // the country code.
    //
    // This shouldn't be necessary, as anyone running in development will need to provide
    // their own API credentials in order to use an LLM provider.
    if config.is_development() {
        return Ok(());
    }

    let country_code = country_code.ok_or_else(|| anyhow!("no country code provided"))?;
    // https://developers.cloudflare.com/fundamentals/reference/http-request-headers/#cf-ipcountry
    match country_code.as_str() {
        // `XX` - Used for clients without country code data.
        "XX" => Err(anyhow!("no country code provided"))?,
        // `T1` - Used for clients using the Tor network.
        "T1" => Err(anyhow!("Tor access is not permitted"))?,
        _ => {}
    }

    let is_country_supported_by_provider = match provider {
        LanguageModelProvider::Anthropic => anthropic::is_supported_country(&country_code),
        LanguageModelProvider::OpenAi => open_ai::is_supported_country(&country_code),
        LanguageModelProvider::Google => google_ai::is_supported_country(&country_code),
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
