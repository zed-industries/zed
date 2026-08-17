use http::StatusCode;

use super::RepoHandler;
use crate::models::repos::secrets::{CreateRepositorySecret, CreateRepositorySecretResponse};

/// A client to GitHub's repository secrets API.
///
/// Created with [`RepoHandler`].
pub struct RepoSecretsHandler<'octo> {
    handler: &'octo RepoHandler<'octo>,
}

impl<'octo> RepoSecretsHandler<'octo> {
    pub(crate) fn new(repo: &'octo RepoHandler<'octo>) -> Self {
        Self { handler: repo }
    }

    /// Lists all secrets available in a repository without revealing their encrypted values.
    /// You must authenticate using an access token with the `repo` scope to use this endpoint.
    /// GitHub Apps must have the `secrets` repository permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let all_secrets = octocrab.repos("owner", "repo")
    ///     .secrets()
    ///     .get_secrets()
    ///     .await?;
    /// # Ok(())
    /// # }
    pub async fn get_secrets(
        &self,
    ) -> crate::Result<crate::models::repos::secrets::RepositorySecrets> {
        let route = format!("/{}/actions/secrets", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Gets your public key, which you need to encrypt secrets.
    /// You need to encrypt a secret before you can create or update secrets.
    /// Anyone with read access to the repository can use this endpoint.
    /// If the repository is private you must use an access token with the `repo` scope.
    /// GitHub Apps must have the `secrets` repository permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let public_key = octocrab.repos("owner", "repo")
    ///     .secrets()
    ///     .get_public_key()
    ///     .await?;
    /// # Ok(())
    /// # }
    pub async fn get_public_key(&self) -> crate::Result<crate::models::PublicKey> {
        let route = format!("/{}/actions/secrets/public-key", self.handler.repo);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Gets a single repository secret without revealing its encrypted value.
    /// You must authenticate using an access token with the `repo` scope to use this endpoint.
    /// GitHub Apps must have the `secrets` repository permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let secret_info = octocrab.repos("owner", "repo")
    ///     .secrets()
    ///     .get_secret("TOKEN")
    ///     .await?;
    /// # Ok(())
    /// # }
    pub async fn get_secret(
        &self,
        secret_name: impl AsRef<str>,
    ) -> crate::Result<crate::models::repos::secrets::RepositorySecret> {
        let route = format!(
            "/{}/actions/secrets/{secret_name}",
            self.handler.repo,
            secret_name = secret_name.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates or updates a repository secret with an encrypted value.
    /// Encrypt your secret using [`crypto_box`](https://crates.io/crates/crypto_box).
    /// You must authenticate using an access token with the `repo` scope to use this endpoint.
    /// GitHub Apps must have the `secrets` repository permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::models::repos::secrets::{CreateRepositorySecret, CreateRepositorySecretResponse};
    ///
    /// let result = octocrab.repos("owner", "repo")
    ///     .secrets()
    ///     .create_or_update_secret("GH_TOKEN", &CreateRepositorySecret{
    ///         key_id: "123456",
    ///         encrypted_value: "some-b64-encrypted-string",
    ///     })
    ///     .await?;
    ///
    /// match result {
    ///    CreateRepositorySecretResponse::Created => println!("Created secret!"),
    ///    CreateRepositorySecretResponse::Updated => println!("Updated secret!"),
    /// }
    /// # Ok(())
    /// # }
    pub async fn create_or_update_secret(
        &self,
        secret_name: impl AsRef<str>,
        secret: &CreateRepositorySecret<'_>,
    ) -> crate::Result<CreateRepositorySecretResponse> {
        let route = format!(
            "/{}/actions/secrets/{secret_name}",
            self.handler.repo,
            secret_name = secret_name.as_ref()
        );

        let resp = {
            let resp = self.handler.crab._put(route, Some(secret)).await?;
            crate::map_github_error(resp).await?
        };

        match resp.status() {
            StatusCode::CREATED => Ok(CreateRepositorySecretResponse::Created),
            StatusCode::NO_CONTENT => Ok(CreateRepositorySecretResponse::Updated),
            status_code => Err(crate::Error::Other {
                source: format!(
                    "Unexpected status code from request: {}",
                    status_code.as_str()
                )
                .into(),
                backtrace: snafu::Backtrace::capture(),
            }),
        }
    }

    /// Deletes a secret in an organization using the secret name.
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    /// GitHub Apps must have the `secrets` organization permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let repo = octocrab.repos("owner", "repo")
    ///     .secrets()
    ///     .delete_secret("GH_TOKEN")
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    pub async fn delete_secret(&self, secret_name: impl AsRef<str>) -> crate::Result<()> {
        let route = format!(
            "/{}/actions/secrets/{secret_name}",
            self.handler.repo,
            secret_name = secret_name.as_ref()
        );

        let resp = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}
