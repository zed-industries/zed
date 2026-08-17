use crate::api::users::UserHandler;
use crate::Page;

/// A builder pattern struct for listing a user's repositories.
///
/// created by [`UserHandler::repos`]
///
/// [`UserHandler::repos`]: ./struct.UserHandler.html#method.repos
#[derive(serde::Serialize)]
pub struct ListUserReposBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b UserHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<crate::params::users::repos::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<crate::params::repos::Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<crate::params::Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> ListUserReposBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b UserHandler<'octo>) -> Self {
        Self {
            handler,
            r#type: None,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
        }
    }

    /// Repository ownership type.
    pub fn r#type(mut self, r#type: impl Into<crate::params::users::repos::Type>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    /// What to sort results by.
    pub fn sort(mut self, sort: impl Into<crate::params::repos::Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// The direction of the sort.
    pub fn direction(mut self, direction: impl Into<crate::params::Direction>) -> Self {
        self.direction = Some(direction.into());
        self
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
    pub async fn send(self) -> crate::Result<Page<crate::models::Repository>> {
        let route = format!("/{user}/repos", user = self.handler.user);
        self.handler.crab.get(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.users("foo");
        let request = handler
            .repos()
            .r#type(crate::params::users::repos::Type::Member)
            .sort(crate::params::repos::Sort::Updated)
            .direction(crate::params::Direction::Ascending)
            .per_page(87)
            .page(3u8);

        assert_eq!(
            serde_json::to_value(request).unwrap(),
            serde_json::json!({
                "type": "member",
                "sort": "updated",
                "direction": "asc",
                "per_page": 87,
                "page": 3,
            })
        )
    }
}
