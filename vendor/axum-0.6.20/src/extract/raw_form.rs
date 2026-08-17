use async_trait::async_trait;
use axum_core::extract::FromRequest;
use bytes::{Bytes, BytesMut};
use http::{Method, Request};

use super::{
    has_content_type,
    rejection::{InvalidFormContentType, RawFormRejection},
};

use crate::{body::HttpBody, BoxError};

/// Extractor that extracts raw form requests.
///
/// For `GET` requests it will extract the raw query. For other methods it extracts the raw
/// `application/x-www-form-urlencoded` encoded request body.
///
/// # Example
///
/// ```rust,no_run
/// use axum::{
///     extract::RawForm,
///     routing::get,
///     Router
/// };
///
/// async fn handler(RawForm(form): RawForm) {}
///
/// let app = Router::new().route("/", get(handler));
/// # async {
/// # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
/// # };
/// ```
#[derive(Debug)]
pub struct RawForm(pub Bytes);

#[async_trait]
impl<S, B> FromRequest<S, B> for RawForm
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = RawFormRejection;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        if req.method() == Method::GET {
            let mut bytes = BytesMut::new();

            if let Some(query) = req.uri().query() {
                bytes.extend(query.as_bytes());
            }

            Ok(Self(bytes.freeze()))
        } else {
            if !has_content_type(req.headers(), &mime::APPLICATION_WWW_FORM_URLENCODED) {
                return Err(InvalidFormContentType.into());
            }

            Ok(Self(Bytes::from_request(req, state).await?))
        }
    }
}

#[cfg(test)]
mod tests {
    use http::{header::CONTENT_TYPE, Request};

    use super::{InvalidFormContentType, RawForm, RawFormRejection};

    use crate::{
        body::{Bytes, Empty, Full},
        extract::FromRequest,
    };

    async fn check_query(uri: &str, value: &[u8]) {
        let req = Request::builder()
            .uri(uri)
            .body(Empty::<Bytes>::new())
            .unwrap();

        assert_eq!(RawForm::from_request(req, &()).await.unwrap().0, value);
    }

    async fn check_body(body: &'static [u8]) {
        let req = Request::post("http://example.com/test")
            .header(CONTENT_TYPE, mime::APPLICATION_WWW_FORM_URLENCODED.as_ref())
            .body(Full::new(Bytes::from(body)))
            .unwrap();

        assert_eq!(RawForm::from_request(req, &()).await.unwrap().0, body);
    }

    #[crate::test]
    async fn test_from_query() {
        check_query("http://example.com/test", b"").await;

        check_query("http://example.com/test?page=0&size=10", b"page=0&size=10").await;
    }

    #[crate::test]
    async fn test_from_body() {
        check_body(b"").await;

        check_body(b"username=user&password=secure%20password").await;
    }

    #[crate::test]
    async fn test_incorrect_content_type() {
        let req = Request::post("http://example.com/test")
            .body(Full::<Bytes>::from(Bytes::from("page=0&size=10")))
            .unwrap();

        assert!(matches!(
            RawForm::from_request(req, &()).await.unwrap_err(),
            RawFormRejection::InvalidFormContentType(InvalidFormContentType)
        ))
    }
}
