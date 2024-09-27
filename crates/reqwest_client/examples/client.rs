use async_ureq::AsyncUreq;
use futures::AsyncReadExt;
use http_client::{AsyncBody, HttpClient};

#[tokio::main]
async fn main() {
    let resp = ReqwestClient::new()
    .get("http://zed.dev", AsyncBody::empty(), true)
    .await
    .unwrap();

    let mut body = String::new();
    resp.into_body().read_to_string(&mut body).await.unwrap();
    dbg!(&body);
}
