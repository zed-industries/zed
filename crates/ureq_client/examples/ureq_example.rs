use http_client::{AsyncBody, HttpClient};
use ureq_client::UreqClient;

fn main() {
    gpui::App::headless().run(|cx| {
        println!("{:?}", std::thread::current().id());
        cx.spawn(|cx| async move {
            let client = UreqClient::new(
                None,
                "Conrad's bot".to_string(),
                cx.background_executor().clone(),
            );

            let resp = client
                .get("http://zed.dev", AsyncBody::empty(), false)
                .await
                .unwrap();

            let mut body = String::new();
            futures::AsyncReadExt::read_to_string(&mut resp.into_body(), &mut body)
                .await
                .unwrap();
            println!("{}", body);

            // Test sync read
            let resp = client
                .get("http://zed.dev", AsyncBody::empty(), false)
                .await
                .unwrap();
            let mut body = String::new();
            std::io::Read::read_to_string(&mut resp.into_body(), &mut body).unwrap();
            println!("{}", body);

            cx.update(|cx| {
                cx.quit();
            })
        })
        .detach();
    })
}
