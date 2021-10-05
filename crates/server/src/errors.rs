use crate::{AppState, LayoutData, Request, RequestExt};
use async_trait::async_trait;
use serde::Serialize;
use std::sync::Arc;
use tide::http::mime;

pub struct Middleware;

#[async_trait]
impl tide::Middleware<Arc<AppState>> for Middleware {
    async fn handle(
        &self,
        mut request: Request,
        next: tide::Next<'_, Arc<AppState>>,
    ) -> tide::Result {
        let app = request.state().clone();
        let layout_data = request.layout_data().await?;

        let mut response = next.run(request).await;

        #[derive(Serialize)]
        struct ErrorData {
            #[serde(flatten)]
            layout: Arc<LayoutData>,
            status: u16,
            reason: &'static str,
        }

        if !response.status().is_success() {
            response.set_body(app.render_template(
                "error.hbs",
                &ErrorData {
                    layout: layout_data,
                    status: response.status().into(),
                    reason: response.status().canonical_reason(),
                },
            )?);
            response.set_content_type(mime::HTML);
        }

        Ok(response)
    }
}

// Allow tide Results to accept context like other Results do when
// using anyhow.
pub trait TideResultExt {
    fn context<C>(self, cx: C) -> Self
    where
        C: std::fmt::Display + Send + Sync + 'static;

    fn with_context<C, F>(self, f: F) -> Self
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T> TideResultExt for tide::Result<T> {
    fn context<C>(self, cx: C) -> Self
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| tide::Error::new(e.status(), e.into_inner().context(cx)))
    }

    fn with_context<C, F>(self, f: F) -> Self
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|e| tide::Error::new(e.status(), e.into_inner().context(f())))
    }
}
