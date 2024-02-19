use futures::StreamExt;
use open_ai::*;

fn main() {
    let http = util::http::zed_client("");
    let host = "https://api.openai.com/v1";
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    futures::executor::block_on(async {
        let mut response = stream_completion(
            &http,
            host,
            &api_key,
            OpenAiRequest {
                model: OpenAiModel::Four,
                messages: vec![OpenAiRequestMessage {
                    role: Role::User,
                    content: "Write a long story about a magic backpack.".into(),
                }],
                stream: true,
                stop: vec![],
                temperature: 1.,
            },
        )
        .await
        .unwrap();

        while let Some(response) = response.next().await {
            dbg!(response);
        }
    })
}
