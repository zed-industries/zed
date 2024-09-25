use anyhow::Error;
use futures::future::BoxFuture;
use http::{Response, Uri};
use hyper_util::client::legacy::{connect::HttpConnector, Client};

struct HyperBasedHttpClient {
    client: Client<HttpConnector, Vec<u8>>,
}

struct Executor {
    executor: gpui::BackgroundExecutor,
}
impl impl HyperBasedHttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder().build(HttpConnector::new()),
        }
    }
}

impl HttpClient for HyperBasedHttpClient {
    fn proxy(&self) -> Option<&Uri> {
        None
    }

    fn send(
        &self,
        request: HttpRequest,
        method: &str,
    ) -> BoxFuture<'static, Result<Response, Error>> {
        let request = request.into_hyper_request(method);
        Box::pin(async move {
            let response = self.client.request(request).await?;
            Ok(response.into())
        })
    }

    fn send_response(&self, response: HttpResponse) -> BoxFuture<'static, Result<(), Error>> {
        let response = response.into_hyper_response();
        Box::pin(async move {
            let _ = self.client.request(response).await?;
            Ok(())
        })
    }
}
