use crate::api::users::UserHandler;
use crate::models;

#[derive(serde::Serialize)]
pub struct BlockedUsersBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b UserHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> BlockedUsersBuilder<'octo, 'b> {
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

    ///## List users blocked by the authenticated user
    ///works with the following token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The token must have the following permission set: `blocking:read`
    ///
    ///```no_run
    ///  use octocrab::models::SimpleUser;
    ///  async fn run() -> octocrab::Result<Vec<SimpleUser>> {
    ///    let blocked_users = octocrab::instance()
    ///        .users("current_user")
    ///        .blocks()
    ///        .per_page(42).page(3u32)
    ///        .list()
    ///        .await?;
    ///    Ok(blocked_users.items)
    ///  }
    pub async fn list(&self) -> crate::Result<crate::Page<models::SimpleUser>> {
        let route = "/user/blocks".to_string();
        self.handler.crab.get(route, None::<&()>).await
    }
}
