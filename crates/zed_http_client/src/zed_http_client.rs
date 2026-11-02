pub use http_client::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("download denied")]
    DownloadDenied,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<http::Error> for Error {
    fn from(error: http::Error) -> Self {
        Self::Other(error.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{executor::block_on, future::BoxFuture};
    use std::sync::Arc;

    struct DeniedHttpClient;

    impl HttpClient<Error> for DeniedHttpClient {
        fn user_agent(&self) -> Option<&http::HeaderValue> {
            None
        }

        fn proxy(&self) -> Option<&Url> {
            None
        }

        fn send(
            &self,
            _request: Request<AsyncBody>,
        ) -> BoxFuture<'static, Result<Response<AsyncBody>, Error>> {
            Box::pin(async { Err(Error::DownloadDenied) })
        }
    }

    #[test]
    fn download_denied_survives_helpers_and_wrappers() -> anyhow::Result<()> {
        block_on(async {
            let client: Arc<dyn HttpClient<Error>> = Arc::new(DeniedHttpClient);
            let clients: [Arc<dyn HttpClient<Error>>; 3] = [
                client.clone(),
                Arc::new(HttpClientWithProxy::new_url(client.clone(), None)),
                Arc::new(HttpClientWithUrl::new_url(
                    client,
                    "https://example.com",
                    None,
                )),
            ];

            for client in clients {
                let request = Request::get("https://example.com/tool").body(AsyncBody::empty())?;
                assert!(matches!(
                    client.send(request).await,
                    Err(Error::DownloadDenied)
                ));
                assert!(matches!(
                    client
                        .get("https://example.com/tool", AsyncBody::empty(), true)
                        .await,
                    Err(Error::DownloadDenied)
                ));
                assert!(matches!(
                    client
                        .post_json("https://example.com/tool", "{}".into())
                        .await,
                    Err(Error::DownloadDenied)
                ));
            }
            Ok(())
        })
    }

    #[test]
    fn malformed_requests_are_not_download_denials() {
        block_on(async {
            let client: Arc<dyn HttpClient<Error>> = Arc::new(HttpClientWithUrl::new_url(
                Arc::new(DeniedHttpClient),
                "https://example.com",
                None,
            ));

            for response in [
                client.get("http://[", AsyncBody::empty(), true),
                client.post_json("http://[", "{}".into()),
            ] {
                match response.await {
                    Err(Error::Other(error)) => assert!(error.is::<http::Error>()),
                    _ => panic!("expected the HTTP request construction error"),
                }
            }
        });
    }
}
