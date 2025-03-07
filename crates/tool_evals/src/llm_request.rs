use anyhow::anyhow;
use futures::StreamExt;
use gpui::{AsyncApp, Entity};
use language_model::{
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, MessageContent, Role,
};

pub struct Eval {
    pub system_prompt: String,
    pub user_query: String,
    pub model_name: String,
}

/// Returns the full text response from the language model
pub async fn run_eval(
    registry: Entity<LanguageModelRegistry>,
    eval: &Eval,
    cx: &mut AsyncApp,
) -> anyhow::Result<String> {
    let model_result = cx.update(|cx| {
        registry
            .read(cx)
            .available_models(cx)
            .find(|model| model.id().0 == eval.model_name)
    })?;

    let Some(model) = model_result else {
        return Ok(format!(
            "No language model named {} was available. Available models: {}",
            eval.model_name,
            cx.update(|cx| {
                registry
                    .read(cx)
                    .available_models(cx)
                    .map(|model| model.id().0.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            })?
        ));
    };

    let request = LanguageModelRequest {
        messages: vec![
            LanguageModelRequestMessage {
                role: Role::System,
                content: vec![MessageContent::Text(eval.system_prompt.clone())],
                cache: false,
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text(eval.user_query.clone())],
                cache: false,
            },
        ],
        temperature: Some(0.0),
        tools: Vec::new(),
        stop: Vec::new(),
    };

    println!("Sending query to language model: {}", eval.user_query);

    // Stream the completion
    match model.stream_completion_text(request, &cx).await {
        Ok(mut stream) => {
            let mut full_response = String::new();

            // Process the response stream
            while let Some(chunk_result) = stream.stream.next().await {
                match chunk_result {
                    Ok(chunk_str) => {
                        full_response.push_str(&chunk_str);
                    }
                    Err(err) => {
                        return Err(anyhow!(
                            "Error receiving response from language model: {err}"
                        ));
                    }
                }
            }

            Ok(full_response)
        }
        Err(err) => Err(anyhow!(
            "Failed to get response from language model. Error was: {err}"
        )),
    }
}
