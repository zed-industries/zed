use crate::api::users::UserHandler;
use crate::Page;

/// A builder pattern struct for listing a users followers
///
/// created by [`UserHandler::followers`]
#[derive(serde::Serialize)]
pub struct ListUserFollowerBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r UserHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> ListUserFollowerBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b UserHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<crate::models::Follower>> {
        // build the route to get this users followers
        let route = format!("/{}/followers", self.handler.user);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// A builder pattern struct for listing who a user is following
///
/// created by [`UserHandler::following`]
#[derive(serde::Serialize)]
pub struct ListUserFollowingBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r UserHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> ListUserFollowingBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b UserHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<crate::models::Followee>> {
        // build the route to get this users followers
        let route = format!("/{}/following", self.handler.user);
        self.handler.crab.get(route, Some(&self)).await
    }
}
