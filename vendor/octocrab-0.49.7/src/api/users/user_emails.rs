use crate::api::users::UserHandler;
use crate::models::UserEmailInfo;
use crate::{FromResponse, Page};

#[derive(serde::Serialize)]
pub struct UserEmailsOpsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b UserHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> UserEmailsOpsBuilder<'octo, 'b> {
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

    ///## List email addresses for the authenticated user
    ///works with the following token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "Email addresses" user permissions (read)
    ///
    ///```no_run
    ///  use octocrab::models::UserEmailInfo;
    /// use octocrab::{Page, Result};
    ///  async fn run() -> Result<Page<UserEmailInfo>> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .emails()
    ///        .per_page(42).page(3u32)
    ///        .list()
    ///        .await
    ///  }
    pub async fn list(&self) -> crate::Result<Page<crate::models::UserEmailInfo>> {
        let route = "/user/emails".to_string();
        self.handler.crab.get(route, Some(&self)).await
    }

    ///## List public email addresses for the authenticated user
    /// Lists your publicly visible email address, which you can set with the `Set primary email visibility`.
    ///works with the following token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "Email addresses" user permissions (read)
    ///  This method can be used without authentication or the aforementioned permissions if only public resources are requested.
    ///
    ///```no_run
    ///  use octocrab::models::UserEmailInfo;
    /// use octocrab::{Page, Result};
    ///  async fn run() -> Result<Page<UserEmailInfo>> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .emails()
    ///        .per_page(42).page(3u32)
    ///        .list_public()
    ///        .await
    ///  }
    pub async fn list_public(&self) -> crate::Result<Page<crate::models::UserEmailInfo>> {
        let route = "/user/public_emails".to_string();
        self.handler.crab.get(route, Some(&self)).await
    }

    ///## Add an email address(es) for the authenticated user
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "Email addresses" user permissions (write)
    ///
    ///```no_run
    ///  use octocrab::models::UserEmailInfo;
    /// use octocrab::Result;
    ///  async fn run() -> Result<Vec<UserEmailInfo>> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .emails()
    ///        .add(vec!["newemail1@mail.com".to_string(), "newemail2@mail.com".to_string()])
    ///        .await
    ///  }
    pub async fn add(
        &self,
        emails: Vec<String>,
    ) -> crate::Result<Vec<crate::models::UserEmailInfo>> {
        let route = "/user/emails".to_string();

        let params = serde_json::json!({
            "emails": serde_json::Value::from(emails),
        });
        let response = self.handler.crab._post(route, Some(&params)).await?;
        if response.status() != http::StatusCode::CREATED {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        <Vec<UserEmailInfo>>::from_response(crate::map_github_error(response).await?).await
    }

    ///## Delete an email address(es) for the authenticated user
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "Email addresses" user permissions (write)
    ///
    ///```no_run
    /// use octocrab::Result;
    ///  async fn run() -> Result<()> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .emails()
    ///        .delete(vec!["newemail1@mail.com".to_string(), "newemail2@mail.com".to_string()])
    ///        .await
    ///  }
    pub async fn delete(&self, emails: Vec<String>) -> crate::Result<()> {
        let route = "/user/emails".to_string();

        let params = serde_json::json!({
            "emails": serde_json::Value::from(emails),
        });
        let response = self.handler.crab._delete(route, Some(&params)).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        Ok(())
    }
}
