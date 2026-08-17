use crate::api::users::UserHandler;
use crate::models::SshSigningKey;
use crate::{FromResponse, Page};

#[derive(serde::Serialize)]
pub struct UserSshSigningKeysOpsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b UserHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> UserSshSigningKeysOpsBuilder<'octo, 'b> {
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

    ///## List SSH signing keys for the authenticated user
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///OAuth app tokens and personal access tokens (classic) need the `read:ssh_signing_key` scope
    ///
    ///The fine-grained token must have the following permission set:
    ///* "SSH signing keys" user permissions (read)
    ///
    ///```no_run
    ///  use octocrab::models::SshSigningKey;
    /// use octocrab::{Page, Result};
    ///  async fn run() -> Result<Page<SshSigningKey>> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .ssh_signing_keys()
    ///        .per_page(42).page(3u32)
    ///        .list()
    ///        .await
    ///  }
    pub async fn list(&self) -> crate::Result<Page<crate::models::SshSigningKey>> {
        let route = "/user/ssh_signing_keys".to_string();
        self.handler.crab.get(route, Some(&self)).await
    }

    ///## Get extended details for an SSH signing key for the authenticated user
    ///
    ///OAuth app tokens and personal access tokens (classic) need the `read:ssh_signing_key` scope to use this method.
    ///
    ///works with the following token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "SSH signing keys" user permissions (read)
    ///
    ///```no_run
    ///  use octocrab::models::SshSigningKey;
    /// use octocrab::Result;
    ///  async fn run() -> Result<SshSigningKey> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .ssh_signing_keys()
    ///        .get(42)
    ///        .await
    ///  }
    pub async fn get(&self, ssh_signing_key_id: u64) -> crate::Result<SshSigningKey> {
        let route = format!("/user/ssh_signing_keys/{ssh_signing_key_id}");
        self.handler.crab.get(route, None::<&()>).await
    }

    ///## Create a SSH signing key for the authenticated user
    /// OAuth app tokens and personal access tokens (classic) need the `write:ssh_signing_key` scope to use this method.
    ///
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "SSH signing keys" user permissions (write)
    ///
    ///```no_run
    /// use octocrab::models::SshSigningKey;
    /// use octocrab::Result;
    ///  async fn run() -> Result<SshSigningKey> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .ssh_signing_keys()
    ///        .add("ssh-rsa AAAAB3NzaC1yc2EAAA".to_string(), "2Sg8iYjAxxmI2LvUXpJjkYrMxURPc8r+dB7TJyvv1234".to_string())
    ///        .await
    ///  }
    pub async fn add(&self, title: String, key: String) -> crate::Result<SshSigningKey> {
        let route = "/user/ssh_signing_keys".to_string();

        let params = serde_json::json!({
            "title": title,
            "key": key,
        });
        let response = self.handler.crab._post(route, Some(&params)).await?;
        if response.status() != http::StatusCode::CREATED {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        <SshSigningKey>::from_response(crate::map_github_error(response).await?).await
    }

    ///## Delete an SSH signing key for the authenticated user
    /// OAuth app tokens and personal access tokens (classic) need the `admin:ssh_signing_key` scope
    ///
    ///works with the following fine-grained token types:
    ///[GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    ///[Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    ///The fine-grained token must have the following permission set:
    ///* "SSH signing keys" user permissions (write)
    ///
    ///```no_run
    /// use octocrab::Result;
    ///  async fn run() -> Result<()> {
    ///    octocrab::instance()
    ///        .users("current_user")
    ///        .ssh_signing_keys()
    ///        .delete(42)
    ///        .await
    ///  }
    pub async fn delete(&self, ssh_signing_key_id: u64) -> crate::Result<()> {
        let route = format!("/user/ssh_signing_keys/{ssh_signing_key_id}");

        let response = self.handler.crab._delete(route, None::<&()>).await?;
        if response.status() != http::StatusCode::NO_CONTENT {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        Ok(())
    }
}
