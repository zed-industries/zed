use futures::AsyncReadExt;
use http_client::{AsyncBody, HttpClient};
use ureq_client::UreqClient;

fn main() {
    gpui::App::headless().run(|cx| {
        println!("{:?}", std::thread::current().id());
        cx.spawn(|cx| async move {
            let resp = UreqClient::new(
                None,
                "Conrad's bot".to_string(),
                cx.background_executor().clone(),
            )
            .get("http://zed.dev", AsyncBody::empty(), true)
            .await
            .unwrap();

            let mut body = String::new();
            resp.into_body().read_to_string(&mut body).await.unwrap();
            println!("{}", body);
        })
        .detach();
    })
}
