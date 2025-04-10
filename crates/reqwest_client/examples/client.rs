use std::time::Instant;

use futures::AsyncReadExt as _;
use futures::stream::FuturesUnordered;
use http_client::AsyncBody;
use http_client::HttpClient;
use reqwest_client::ReqwestClient;
use smol::stream::StreamExt;

fn main() {
    let app = gpui::Application::new();
    app.run(|cx| {
        cx.spawn(async move |cx| {
            let client = ReqwestClient::new();
            let start = Instant::now();
            let requests = [
                client.get("https://www.google.com/", AsyncBody::empty(), true),
                client.get("https://zed.dev/", AsyncBody::empty(), true),
                client.get("https://docs.rs/", AsyncBody::empty(), true),
            ];
            let mut requests = requests.into_iter().collect::<FuturesUnordered<_>>();
            while let Some(response) = requests.next().await {
                let mut body = String::new();
                response
                    .unwrap()
                    .into_body()
                    .read_to_string(&mut body)
                    .await
                    .unwrap();
                println!("{}", &body.len());
            }
            println!("{:?}", start.elapsed());

            cx.update(|cx| {
                cx.quit();
            })
            .ok();
        })
        .detach();
    })
}
