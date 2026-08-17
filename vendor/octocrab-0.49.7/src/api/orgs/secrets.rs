use http::StatusCode;

use super::OrgHandler;
use crate::models::orgs::secrets::{CreateOrganizationSecret, CreateOrganizationSecretResponse};

/// A client to GitHub's organization API.
///
/// Created with [`crate::Octocrab::orgs`].
pub struct OrgSecretsHandler<'octo> {
    org: &'octo OrgHandler<'octo>,
}

impl<'octo> OrgSecretsHandler<'octo> {
    pub(crate) fn new(org: &'octo OrgHandler<'octo>) -> Self {
        Self { org }
    }

    fn owner(&self) -> &String {
        &self.org.owner
    }

    /// Lists all secrets available in an organization without revealing their encrypted values.
    /// You must authenticate using an access token with the admin:org scope to use this endpoint.
    /// GitHub Apps must have the secrets organization permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let org = octocrab.orgs("owner");
    /// let secrets = org.secrets();
    /// let all_secrets = secrets.get_secrets().await?;
    /// # Ok(())
    /// # }
    pub async fn get_secrets(
        &self,
    ) -> crate::Result<crate::models::orgs::secrets::OrganizationSecrets> {
        let route = format!("/orgs/{org}/actions/secrets", org = self.owner());
        self.org.crab.get(route, None::<&()>).await
    }

    // Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets.
    // You must authenticate using an access token with the admin:org scope to use this endpoint.
    // GitHub Apps must have the secrets organization permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let org = octocrab.orgs("owner");
    /// let secrets = org.secrets();
    /// let public_key = secrets.get_public_key().await?;
    /// # Ok(())
    /// # }
    pub async fn get_public_key(&self) -> crate::Result<crate::models::PublicKey> {
        let route = format!("/orgs/{org}/actions/secrets/public-key", org = self.owner());
        self.org.crab.get(route, None::<&()>).await
    }

    /// Gets a specific secret from the organization without revealing its encrypted values.
    /// You must authenticate using an access token with the admin:org scope to use this endpoint.
    /// GitHub Apps must have the secrets organization permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let org = octocrab.orgs("owner");
    /// let secrets = org.secrets();
    /// let secret_info = secrets.get_secret("TOKEN").await?;
    /// # Ok(())
    /// # }
    pub async fn get_secret(
        &self,
        secret_name: impl AsRef<str>,
    ) -> crate::Result<crate::models::orgs::secrets::OrganizationSecret> {
        let route = format!(
            "/orgs/{org}/actions/secrets/{secret_name}",
            org = self.owner(),
            secret_name = secret_name.as_ref()
        );
        self.org.crab.get(route, None::<&()>).await
    }

    /// Creates or updates an organization secret with an encrypted value.
    /// Encrypt your secret using [`crypto_box`](https://crates.io/crates/crypto_box).
    /// You must authenticate using an access token with the admin:org scope to use this endpoint.
    /// GitHub Apps must have the secrets organization permission to use this endpoint
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::models::orgs::secrets::{
    ///     CreateOrganizationSecret, CreateOrganizationSecretResponse,
    ///     Visibility
    /// };
    ///
    /// let org = octocrab.orgs("owner");
    /// let secrets = org.secrets();
    /// let result = secrets.create_or_update_secret("GH_TOKEN", &CreateOrganizationSecret{
    ///    key_id: "123456",
    ///    encrypted_value: "some-b64-encrypted-string",
    ///    visibility: Visibility::Selected,
    ///    selected_repository_ids: None,
    /// }).await?;
    ///
    /// match result {
    ///    CreateOrganizationSecretResponse::Created => println!("Created secret!"),
    ///    CreateOrganizationSecretResponse::Updated => println!("Updated secret!"),
    /// }
    /// # Ok(())
    /// # }
    pub async fn create_or_update_secret(
        &self,
        secret_name: impl AsRef<str>,
        secret: &CreateOrganizationSecret<'_>,
    ) -> crate::Result<crate::models::orgs::secrets::CreateOrganizationSecretResponse> {
        let route = format!(
            "/orgs/{org}/actions/secrets/{secret_name}",
            org = self.owner(),
            secret_name = secret_name.as_ref()
        );

        let resp = {
            let resp = self.org.crab._put(route, Some(secret)).await?;
            crate::map_github_error(resp).await?
        };

        match resp.status() {
            StatusCode::CREATED => Ok(CreateOrganizationSecretResponse::Created),
            StatusCode::NO_CONTENT => Ok(CreateOrganizationSecretResponse::Updated),
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

    /// Deletes an organization secret.
    /// You must authenticate using an access token with the admin:org scope to use this endpoint.
    /// GitHub Apps must have the secrets organization permission to use this endpoint
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let org = octocrab.orgs("owner");
    /// let secrets = org.secrets();
    ///
    /// secrets.delete_secret("GH_TOKEN").await?;
    ///
    /// # Ok(())
    /// # }
    pub async fn delete_secret(&self, secret_name: impl AsRef<str>) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/actions/secrets/{secret_name}",
            org = self.owner(),
            secret_name = secret_name.as_ref()
        );

        let resp = self.org.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}
