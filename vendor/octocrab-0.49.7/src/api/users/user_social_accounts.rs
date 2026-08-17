use crate::api::users::UserHandler;
use crate::models::SocialAccount;
use crate::{FromResponse, Page};

#[derive(serde::Serialize)]
pub struct UserSocialAccountsOpsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b UserHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> UserSocialAccountsOpsBuilder<'octo, 'b> {
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

    ///## List social accounts for the authenticated user
    ///
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token does not require any permissions.
    ///
    ///```no_run
    ///  use octocrab::models::SocialAccount;
    /// use octocrab::{Page, Result};
    ///  async fn run() -> Result<Page<SocialAccount>> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .social_accounts()
    ///        .per_page(42).page(3u32)
    ///        .list()
    ///        .await
    ///  }
    pub async fn list(&self) -> crate::Result<Page<crate::models::SocialAccount>> {
        let route = "/user/social_accounts".to_string();
        self.handler.crab.get(route, Some(&self)).await
    }

    ///## Add social accounts for the authenticated user
    ///OAuth app tokens and personal access tokens (classic) need the `user` scope
    ///
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "Profile" user permissions (write)
    ///
    ///```no_run
    /// use octocrab::models::SocialAccount;
    /// use octocrab::Result;
    ///  async fn run() -> Result<Vec<SocialAccount>> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .social_accounts()
    ///        .add(vec!["https://facebook.com/GitHub".to_string(),"https://www.youtube.com/@GitHub".to_string()])
    ///        .await
    ///  }
    pub async fn add(&self, account_urls: Vec<String>) -> crate::Result<Vec<SocialAccount>> {
        let route = "/user/social_accounts".to_string();

        let params = serde_json::json!({
            "account_urls": account_urls,
        });
        let response = self.handler.crab._post(route, Some(&params)).await?;
        if response.status() != http::StatusCode::CREATED {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        <Vec<SocialAccount>>::from_response(crate::map_github_error(response).await?).await
    }

    ///## Deletes one or more social accounts from the authenticated user's profile.
    //
    // OAuth app tokens and personal access tokens (classic) need the `user` scope
    ///
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "Profile" user permissions (write)
    ///
    ///```no_run
    /// use octocrab::Result;
    ///  async fn run() -> Result<()> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .social_accounts()
    ///        .delete(vec!["https://facebook.com/GitHub".to_string(),"https://www.youtube.com/@GitHub".to_string()])
    ///        .await
    ///  }
    pub async fn delete(&self, account_urls: Vec<String>) -> crate::Result<()> {
        let route = "/user/social_accounts".to_string();

        let params = serde_json::json!({
            "account_urls": account_urls,
        });

        let response = self.handler.crab._delete(route, Some(&params)).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        Ok(())
    }
}
