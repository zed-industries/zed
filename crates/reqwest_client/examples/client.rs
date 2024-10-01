use futures::AsyncReadExt as _;
use http_client::AsyncBody;
use http_client::HttpClient;
use reqwest_client::ReqwestClient;

#[tokio::main]
async fn main() {
    let resp = ReqwestClient::new()
        .get("http://zed.dev", AsyncBody::empty(), true)
        .await
        .unwrap();

    let mut body = String::new();
    resp.into_body().read_to_string(&mut body).await.unwrap();
    println!("{}", &body);
}
