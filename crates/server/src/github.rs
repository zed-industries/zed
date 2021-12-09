use crate::expiring::Expiring;
use anyhow::{anyhow, Context};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    future::Future,
    sync::Arc,
    time::{Duration, Instant},
};
use surf::{http::Method, RequestBuilder, Url};

#[derive(Debug, Deserialize, Serialize)]
pub struct Release {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub name: String,
    pub url: String,
}

pub struct AppClient {
    id: usize,
    private_key: String,
    jwt_bearer_header: Expiring<String>,
}

#[derive(Deserialize)]
struct Installation {
    #[allow(unused)]
    id: usize,
}

impl AppClient {
    #[cfg(test)]
    pub fn test() -> Arc<Self> {
        Arc::new(Self {
            id: Default::default(),
            private_key: Default::default(),
            jwt_bearer_header: Default::default(),
        })
    }

    pub fn new(id: usize, private_key: String) -> Arc<Self> {
        Arc::new(Self {
            id,
            private_key,
            jwt_bearer_header: Default::default(),
        })
    }

    pub async fn repo(self: &Arc<Self>, nwo: String) -> tide::Result<RepoClient> {
        let installation: Installation = self
            .request(
                Method::Get,
                &format!("/repos/{}/installation", &nwo),
                |refresh| self.bearer_header(refresh),
            )
            .await?;

        Ok(RepoClient {
            app: self.clone(),
            nwo,
            installation_id: installation.id,
            installation_token_header: Default::default(),
        })
    }

    pub fn user(self: &Arc<Self>, access_token: String) -> UserClient {
        UserClient {
            app: self.clone(),
            access_token,
        }
    }

    async fn request<T, F, G>(
        &self,
        method: Method,
        path: &str,
        get_auth_header: F,
    ) -> tide::Result<T>
    where
        T: DeserializeOwned,
        F: Fn(bool) -> G,
        G: Future<Output = tide::Result<String>>,
    {
        let mut retried = false;

        loop {
            let response = RequestBuilder::new(
                method,
                Url::parse(&format!("https://api.github.com{}", path))?,
            )
            .header("Accept", "application/vnd.github.v3+json")
            .header("Authorization", get_auth_header(retried).await?)
            .recv_json()
            .await;

            if let Err(error) = response.as_ref() {
                if error.status() == 401 && !retried {
                    retried = true;
                    continue;
                }
            }

            return response;
        }
    }

    async fn bearer_header(&self, refresh: bool) -> tide::Result<String> {
        if refresh {
            self.jwt_bearer_header.clear().await;
        }

        self.jwt_bearer_header
            .get_or_refresh(|| async {
                use jwt_simple::{algorithms::RS256KeyPair, prelude::*};
                use std::time;

                let key_pair = RS256KeyPair::from_pem(&self.private_key)
                    .with_context(|| format!("invalid private key {:?}", self.private_key))?;
                let mut claims = Claims::create(Duration::from_mins(10));
                claims.issued_at = Some(Clock::now_since_epoch() - Duration::from_mins(1));
                claims.issuer = Some(self.id.to_string());
                let token = key_pair.sign(claims).context("failed to sign claims")?;
                let expires_at = time::Instant::now() + time::Duration::from_secs(9 * 60);

                Ok((format!("Bearer {}", token), expires_at))
            })
            .await
    }

    async fn installation_token_header(
        &self,
        header: &Expiring<String>,
        installation_id: usize,
        refresh: bool,
    ) -> tide::Result<String> {
        if refresh {
            header.clear().await;
        }

        header
            .get_or_refresh(|| async {
                #[derive(Debug, Deserialize)]
                struct AccessToken {
                    token: String,
                }

                let access_token: AccessToken = self
                    .request(
                        Method::Post,
                        &format!("/app/installations/{}/access_tokens", installation_id),
                        |refresh| self.bearer_header(refresh),
                    )
                    .await?;

                let header = format!("Token {}", access_token.token);
                let expires_at = Instant::now() + Duration::from_secs(60 * 30);

                Ok((header, expires_at))
            })
            .await
    }
}

pub struct RepoClient {
    app: Arc<AppClient>,
    nwo: String,
    installation_id: usize,
    installation_token_header: Expiring<String>,
}

impl RepoClient {
    #[cfg(test)]
    pub fn test(app_client: &Arc<AppClient>) -> Self {
        Self {
            app: app_client.clone(),
            nwo: String::new(),
            installation_id: 0,
            installation_token_header: Default::default(),
        }
    }

    pub async fn releases(&self) -> tide::Result<Vec<Release>> {
        self.get(&format!("/repos/{}/releases?per_page=100", self.nwo))
            .await
    }

    pub async fn release_asset(&self, tag: &str, name: &str) -> tide::Result<surf::Body> {
        let release: Release = self
            .get(&format!("/repos/{}/releases/tags/{}", self.nwo, tag))
            .await?;

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == name)
            .ok_or_else(|| anyhow!("no asset found with name {}", name))?;

        let request = surf::get(&asset.url)
            .header("Accept", "application/octet-stream'")
            .header(
                "Authorization",
                self.installation_token_header(false).await?,
            );

        let client = surf::client();
        let mut response = client.send(request).await?;

        // Avoid using `surf::middleware::Redirect` because that type forwards
        // the original request headers to the redirect URI. In this case, the
        // redirect will be to S3, which forbids us from supplying an
        // `Authorization` header.
        if response.status().is_redirection() {
            if let Some(url) = response.header("location") {
                let request = surf::get(url.as_str()).header("Accept", "application/octet-stream");
                response = client.send(request).await?;
            }
        }

        if !response.status().is_success() {
            Err(anyhow!("failed to fetch release asset {} {}", tag, name))?;
        }

        Ok(response.take_body())
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> tide::Result<T> {
        self.request::<T>(Method::Get, path).await
    }

    async fn request<T: DeserializeOwned>(&self, method: Method, path: &str) -> tide::Result<T> {
        Ok(self
            .app
            .request(method, path, |refresh| {
                self.installation_token_header(refresh)
            })
            .await?)
    }

    async fn installation_token_header(&self, refresh: bool) -> tide::Result<String> {
        self.app
            .installation_token_header(
                &self.installation_token_header,
                self.installation_id,
                refresh,
            )
            .await
    }
}

pub struct UserClient {
    app: Arc<AppClient>,
    access_token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub login: String,
    pub avatar_url: String,
}

impl UserClient {
    pub async fn details(&self) -> tide::Result<User> {
        Ok(self
            .app
            .request(Method::Get, "/user", |_| async {
                Ok(self.access_token_header())
            })
            .await?)
    }

    fn access_token_header(&self) -> String {
        format!("Token {}", self.access_token)
    }
}
