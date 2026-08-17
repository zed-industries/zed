use super::*;
use crate::error::HttpSnafu;
use http::Uri;
use snafu::ResultExt;

/// A builder pattern struct for listing hooks deliveries.
///
/// created by [`HooksHandler::retry_delivery`]
///
/// [`HooksHandler::retry_delivery`]: ./struct.HooksHandler.html#method.retry_delivery
#[derive(serde::Serialize)]
pub struct RetryDeliveryBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r HooksHandler<'octo>,
    #[serde(skip)]
    hook_id: HookId,
    #[serde(skip)]
    delivery_id: HookDeliveryId,
}
impl<'octo, 'r> RetryDeliveryBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r HooksHandler<'octo>,
        hook_id: HookId,
        delivery_id: HookDeliveryId,
    ) -> Self {
        Self {
            handler,
            hook_id,
            delivery_id,
        }
    }

    /// Send the actual request.
    pub async fn send(self) -> crate::Result<()> {
        let route = match self.handler.repo.clone() {
            Some(repo) => format!(
                "/repos/{}/{}/hooks/{}/deliveries/{}/attempts",
                self.handler.owner, repo, self.hook_id, self.delivery_id
            ),
            None => format!(
                "/orgs/{}/hooks/{}/deliveries/{}/attempts",
                self.handler.owner, self.hook_id, self.delivery_id
            ),
        };

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.handler.crab._post(uri, None::<&()>).await?)
            .await
            .map(drop)
    }
}
