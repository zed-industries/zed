//! The markdown API
use crate::error::HttpSnafu;
use http::request::Builder;
use http::{Method, Uri};
use snafu::ResultExt;

use crate::Octocrab;

/// Handler for GitHub's markdown API.
///
/// Created with [`Octocrab::markdown`].
pub struct MarkdownHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> MarkdownHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Render an arbitrary Markdown document.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// let markdown = octocrab::instance()
    ///     .markdown()
    ///     .render("Comment referencing issue #404")
    ///     .mode(params::markdown::Mode::Gfm)
    ///     .context("owner/repo")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn render<'r, 'text>(
        &'r self,
        text: &'text (impl AsRef<str> + ?Sized),
    ) -> RenderMarkdownBuilder<'octo, 'r, 'text> {
        RenderMarkdownBuilder::new(self, text.as_ref())
    }

    /// Render a Markdown document in raw mode.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// let markdown = octocrab::instance()
    ///     .markdown()
    ///     .render_raw("~~_**Octocrab**_~~")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn render_raw(&self, text: impl Into<String>) -> crate::Result<String> {
        let uri = Uri::builder()
            .path_and_query("/markdown/raw")
            .build()
            .context(HttpSnafu)?;
        let mut request = Builder::new().uri(uri).method(Method::POST);
        request = request.header(http::header::CONTENT_TYPE, "text/x-markdown");

        let request = self.crab.build_request(request, Some(&text.into()))?;

        self.crab
            .body_to_string(self.crab.execute(request).await?)
            .await
    }
}

#[derive(serde::Serialize)]
pub struct RenderMarkdownBuilder<'octo, 'r, 'text> {
    #[serde(skip)]
    handler: &'r MarkdownHandler<'octo>,
    text: &'text str,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<crate::params::markdown::Mode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>,
}

impl<'octo, 'r, 'text> RenderMarkdownBuilder<'octo, 'r, 'text> {
    pub(crate) fn new(handler: &'r MarkdownHandler<'octo>, text: &'text str) -> Self {
        Self {
            handler,
            text,
            mode: None,
            context: None,
        }
    }

    /// The repository context to use when creating references in `Mode::Gfm`.
    /// Omit this parameter when using markdown mode.
    pub fn context<A: Into<String>>(mut self, context: impl Into<Option<A>>) -> Self {
        self.context = context.into().map(A::into);
        self
    }

    /// The rendering mode.
    pub fn mode(mut self, mode: impl Into<Option<crate::params::markdown::Mode>>) -> Self {
        self.mode = mode.into();
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> crate::Result<String> {
        let uri = Uri::builder()
            .path_and_query("/markdown")
            .build()
            .context(HttpSnafu)?;
        self.handler
            .crab
            .body_to_string(self.handler.crab._post(uri, Some(&self)).await?)
            .await
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::instance();
        let handler = octocrab.markdown();
        let render = handler
            .render("**Markdown**")
            .mode(crate::params::markdown::Mode::Gfm)
            .context("owner/repo");

        assert_eq!(
            serde_json::to_value(render).unwrap(),
            serde_json::json!({
                "text": "**Markdown**",
                "mode": "gfm",
                "context": "owner/repo",
            })
        )
    }
}
