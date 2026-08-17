use super::*;

/// Handler for GitHub's releases API.
///
/// Created with [`RepoHandler::release_assets`].
pub struct ReleaseAssetsHandler<'octo, 'r> {
    handler: &'r RepoHandler<'octo>,
}

impl<'octo, 'r> ReleaseAssetsHandler<'octo, 'r> {
    pub(crate) fn new(parent: &'r RepoHandler<'octo>) -> Self {
        Self { handler: parent }
    }
    /// Gets the release asset using its id.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let release = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .release_assets()
    ///     .get(3)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, id: u64) -> Result<models::repos::Asset> {
        let route = format!("/{}/releases/assets/{id}", self.handler.repo, id = id,);

        self.handler.crab.get(route, None::<&()>).await
    }
    /// Creates a new [`UpdateReleaseAssetBuilder`] with `asset_id`.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let release = octocrab.repos("owner", "repo")
    ///     .release_assets()
    ///     .update(1)
    ///     // Optional Parameters
    ///     .name("asset_1.tar.gz")
    ///     .label("Asset 1")
    ///     .state(octocrab::params::repos::release_assets::State::Uploaded)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, asset_id: u64) -> UpdateReleaseAssetBuilder<'_, '_, '_, '_, '_> {
        UpdateReleaseAssetBuilder::new(self, asset_id)
    }

    /// Delete a release asset using its id.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let release = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .release_assets()
    ///     .delete(3)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, id: u64) -> Result<()> {
        let route = format!("/{}/releases/assets/{id}", self.handler.repo, id = id,);

        self.handler.crab._delete(route, None::<&()>).await?;
        Ok(())
    }

    /// Streams the binary contents of an asset.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use futures_util::StreamExt;
    ///
    /// let mut stream = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .release_assets()
    ///     .stream(42u64)
    ///     .await?;
    ///
    /// while let Some(chunk) = stream.next().await {
    ///     println!("{:?}", chunk);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "stream")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
    pub async fn stream(
        &self,
        id: u64,
    ) -> crate::Result<impl futures_core::Stream<Item = crate::Result<bytes::Bytes>>> {
        use futures_util::TryStreamExt;
        //use snafu::GenerateImplicitData;

        let route = format!("/{}/releases/assets/{id}", self.handler.repo, id = id,);

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        let builder = Builder::new()
            .method(http::Method::GET)
            .uri(uri)
            .header(http::header::ACCEPT, "application/octet-stream");
        let request = self.handler.crab.build_request(builder, None::<&()>)?;
        let response = self.handler.crab.execute(request).await?;
        let response = self.handler.crab.follow_location_to_data(response).await?;
        Ok(http_body_util::BodyStream::new(response.into_body())
            .try_filter_map(|frame| futures_util::future::ok(frame.into_data().ok())))
    }
}

/// A builder pattern struct for updating release assets.
///
/// created by [`ReleaseAssetsHandler::update`].
#[derive(serde::Serialize)]
pub struct UpdateReleaseAssetBuilder<'octo, 'repos, 'handler, 'name, 'label> {
    #[serde(skip)]
    handler: &'handler ReleaseAssetsHandler<'octo, 'repos>,
    #[serde(skip)]
    asset_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'name str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<&'label str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<crate::params::repos::release_assets::State>,
}

impl<'octo, 'repos, 'handler, 'name, 'label>
    UpdateReleaseAssetBuilder<'octo, 'repos, 'handler, 'name, 'label>
{
    pub(crate) fn new(
        handler: &'handler ReleaseAssetsHandler<'octo, 'repos>,
        asset_id: u64,
    ) -> Self {
        Self {
            handler,
            asset_id,
            name: None,
            label: None,
            state: None,
        }
    }

    /// The name of the release asset.
    pub fn name(mut self, name: &'name (impl AsRef<str> + ?Sized)) -> Self {
        self.name = Some(name.as_ref());
        self
    }

    /// The label of the release asset.
    pub fn label(mut self, label: &'label (impl AsRef<str> + ?Sized)) -> Self {
        self.label = Some(label.as_ref());
        self
    }

    /// The state of the release asset.
    pub fn state<A: Into<crate::params::repos::release_assets::State>>(
        mut self,
        state: impl Into<Option<A>>,
    ) -> Self {
        self.state = state.into().map(A::into);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::repos::Asset> {
        let route = format!(
            "/{repo}/releases/assets/{asset_id}",
            repo = self.handler.handler.repo,
            asset_id = self.asset_id,
        );
        self.handler.handler.crab.patch(route, Some(&self)).await
    }
}
