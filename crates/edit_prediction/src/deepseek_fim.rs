use anyhow::{Context as _, Result};
use cloud_llm_client::predict_edits_v3::RawCompletionResponse;
use futures::AsyncReadExt as _;
use gpui::http_client;
use language::language_settings::OpenAiCompatibleEditPredictionSettings;
use serde_json::json;
use std::sync::Arc;

pub(crate) fn is_supported_api_url(api_url: &str) -> bool {
    matches!(
        api_url,
        "https://api.deepseek.com/v1/completions"
            | "https://api.deepseek.com/v1/chat/completions"
            | "https://api.deepseek.com/beta/completions"
    )
}

pub(crate) async fn try_request(
    settings: &OpenAiCompatibleEditPredictionSettings,
    prefix: &str,
    suffix: &str,
    api_key: Option<Arc<str>>,
    http_client: &Arc<dyn http_client::HttpClient>,
) -> Result<Option<(String, String)>> {
    if !is_supported_api_url(&settings.api_url) {
        return Ok(None);
    }

    let request_body = json!({
        "model": settings.model,
        "prompt": prefix,
        "suffix": suffix,
        "max_tokens": settings.max_output_tokens,
    });
    let mut request_builder = http_client::Request::builder()
        .method(http_client::Method::POST)
        .uri("https://api.deepseek.com/beta/completions")
        .header("Content-Type", "application/json");
    if let Some(api_key) = api_key {
        request_builder = request_builder.header("Authorization", format!("Bearer {api_key}"));
    }
    let request = request_builder.body(http_client::AsyncBody::from(serde_json::to_string(
        &request_body,
    )?))?;
    let mut response = http_client.send(request).await?;
    let status = response.status();
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;
    if !status.is_success() {
        anyhow::bail!("DeepSeek FIM request failed: {status} - {body}");
    }
    let response: RawCompletionResponse =
        serde_json::from_str(&body).context("Failed to parse DeepSeek FIM response")?;
    let choice = response
        .choices
        .into_iter()
        .next()
        .context("DeepSeek FIM response contained no choices")?;
    Ok(Some((choice.text, response.id)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fim::send_fim_request;
    use gpui::http_client::FakeHttpClient;
    use language::EditPredictionPromptFormat;
    use serde_json::Value;

    #[test]
    fn official_deepseek_models_use_native_fim_with_separate_suffix() -> Result<()> {
        futures::executor::block_on(async {
            for (api_url, model) in [
                ("https://api.deepseek.com/v1/completions", "deepseek-flash"),
                (
                    "https://api.deepseek.com/v1/chat/completions",
                    "deepseek-v4-pro",
                ),
                ("https://api.deepseek.com/beta/completions", "deepseek-next"),
            ] {
                let http_client: Arc<dyn http_client::HttpClient> = FakeHttpClient::create(
                    move |mut request| async move {
                        assert_eq!(
                            request.uri().to_string(),
                            "https://api.deepseek.com/beta/completions"
                        );
                        let mut body = String::new();
                        request.body_mut().read_to_string(&mut body).await?;
                        let body: Value = serde_json::from_str(&body)?;
                        assert_eq!(
                            body,
                            json!({
                                "model": model,
                                "prompt": "def square(number):\n    return number ",
                                "suffix": " number\n",
                                "max_tokens": 32
                            })
                        );
                        Ok(http_client::Response::builder().status(200).body(json!({
                            "id": "native-fim", "object": "text_completion", "created": 0,
                            "model": model,
                            "choices": [{"text": "*", "finish_reason": "stop"}],
                            "usage": {"prompt_tokens": 10, "completion_tokens": 1, "total_tokens": 11}
                        }).to_string().into())?)
                    },
                );
                let settings = OpenAiCompatibleEditPredictionSettings {
                    model: model.into(),
                    api_url: api_url.into(),
                    max_output_tokens: 32,
                    ..Default::default()
                };
                assert_eq!(
                    send_fim_request(
                        settings::EditPredictionProvider::OpenAiCompatibleApi,
                        EditPredictionPromptFormat::DeepseekCoder,
                        &settings,
                        "def square(number):\n    return number ",
                        " number\n",
                        None,
                        &http_client
                    )
                    .await?,
                    ("*".into(), "native-fim".into()),
                    "{api_url}"
                );
            }
            Ok(())
        })
    }

    #[test]
    fn self_hosted_deepseek_coder_keeps_legacy_fim_format() -> Result<()> {
        futures::executor::block_on(async {
            let http_client: Arc<dyn http_client::HttpClient> =
                FakeHttpClient::create(|mut request| async move {
                    assert_eq!(
                        request.uri().to_string(),
                        "https://example.com/v1/completions"
                    );
                    let mut body = String::new();
                    request.body_mut().read_to_string(&mut body).await?;
                    let body: Value = serde_json::from_str(&body)?;
                    assert_eq!(
                        body["prompt"],
                        "<｜fim▁begin｜>before<｜fim▁hole｜>after<｜fim▁end｜>"
                    );
                    assert!(body.get("suffix").is_none());
                    Ok(http_client::Response::builder().status(200).body(json!({
                        "id": "legacy-fim", "object": "text_completion", "created": 0,
                        "model": "deepseek-coder-v2",
                        "choices": [{"text": "missing", "finish_reason": "stop"}],
                        "usage": {"prompt_tokens": 10, "completion_tokens": 1, "total_tokens": 11}
                    }).to_string().into())?)
                });
            let settings = OpenAiCompatibleEditPredictionSettings {
                model: "deepseek-coder-v2".into(),
                api_url: "https://example.com/v1/completions".into(),
                max_output_tokens: 32,
                ..Default::default()
            };
            assert_eq!(
                send_fim_request(
                    settings::EditPredictionProvider::OpenAiCompatibleApi,
                    EditPredictionPromptFormat::DeepseekCoder,
                    &settings,
                    "before",
                    "after",
                    None,
                    &http_client
                )
                .await?,
                ("missing".into(), "legacy-fim".into())
            );
            Ok(())
        })
    }

    #[test]
    fn non_official_url_does_not_infer_an_unknown_model() {
        assert_eq!(
            crate::fim::infer_prompt_format_for_api(
                "deepseek-next",
                "https://example.com/v1/completions"
            ),
            None
        );
    }
}
