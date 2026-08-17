use hyper::{body::HttpBody as _, Client};
use hyper_tls::HttpsConnector;
use tokio::io::{self, AsyncWriteExt as _};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut res = client.get("https://hyper.rs".parse()?).await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    while let Some(chunk) = res.body_mut().data().await {
        let chunk = chunk?;
        io::stdout().write_all(&chunk).await?
    }
    Ok(())
}
