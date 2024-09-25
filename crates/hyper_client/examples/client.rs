use futures::AsyncReadExt;
use http_client::{AsyncBody, HttpClient};
use hyper_client::UreqHttpClient;

fn main() {
    gpui::App::headless().run(|cx| {
        dbg!(std::thread::current().id());
        cx.spawn(|cx| async move {
            let resp = UreqHttpClient::new(cx.background_executor().clone())
                .get("http://zed.dev", AsyncBody::empty(), false)
                .await
                .unwrap();

            let mut body = String::new();
            resp.into_body().read_to_string(&mut body).await.unwrap();
            dbg!(&body.len());
        })
        .detach();
    })
}
