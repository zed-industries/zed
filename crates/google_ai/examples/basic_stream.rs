use futures::StreamExt;
use google_ai::*;

fn main() {
    let http = util::http::zed_client("");
    let host = "https://generativelanguage.googleapis.com";
    let gemini_api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");

    futures::executor::block_on(async {
        let mut response = stream_generate_content(
            &http,
            host,
            &gemini_api_key,
            GenerateContentRequest {
                contents: vec![Content {
                    parts: vec![Part::TextPart(TextPart {
                        text: "Write a long story about a magic backpack.".into(),
                    })],
                }],
                generation_config: None,
                safety_settings: None,
            },
        )
        .await
        .unwrap();

        while let Some(response) = response.next().await {
            dbg!(response);
        }
    })
}
