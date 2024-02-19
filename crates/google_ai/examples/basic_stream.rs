use futures::AsyncReadExt;
use util::http::{zed_client, AsyncBody, HttpClient};

// curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:streamGenerateContent?key=${GEMINI_API_KEY}" \
//         -H 'Content-Type: application/json' \
//         --no-buffer \
//         -d '{ "contents":[{"parts":[{"text": "Write long a story about a magic backpack."}]}]}' \
//         2> /dev/null

fn main() {
    let host = "https://generativelanguage.googleapis.com";
    let http = zed_client(host);
    let gemini_api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");

    // Now you can use `client` to send HTTP requests.
    // For example, to send a POST request with JSON body, you could do something like this:

    let uri = format!(
        "{}/v1beta/models/gemini-pro:streamGenerateContent?key={}",
        host, gemini_api_key
    );
    let body =
        r#"{ "contents":[{"parts":[{"text": "Write a long story about a magic backpack."}]}]}"#;

    dbg!(&uri);
    dbg!(&body);

    futures::executor::block_on(async {
        dbg!("request...");

        let response = http
            .post_json(&uri, AsyncBody::from(body.to_string()))
            .await
            .unwrap();

        dbg!("got response");

        // Read chunks of the response body and print them out.
        let mut stream = response.into_body();

        dbg!("into body");

        let mut start = 0;
        let mut buffer = Vec::new();
        loop {
            match stream.read(&mut buffer).await {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    println!("New chunk: {:?}", &buffer[start..start + n]);
                    start += n;
                }
                Err(error) => {
                    eprintln!("Error: {}", error);
                    break;
                }
            }
        }

        println!("Response body: {:?}", String::from_utf8_lossy(&buffer));
    });
}
