use anyhow::Result;
use cloud_api_types::{AuthenticatedUser, GetAuthenticatedUserResponse};
use futures::AsyncReadExt as _;
use http_client::{AsyncBody, HttpClientWithUrl, Method, Request};

pub struct CloudApiClient {
    user_id: i32,
    access_token: String,
    http_client: HttpClientWithUrl,
}

impl CloudApiClient {
    pub async fn get_authenticated_user(&self) -> Result<AuthenticatedUser> {
        let request = Request::builder()
            .method(Method::GET)
            .uri(
                self.http_client
                    .build_zed_cloud_url("/client/users/me", &[])?
                    .as_ref(),
            )
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("{} {}", self.user_id, self.access_token),
            )
            .body(AsyncBody::default())?;

        let mut response = self.http_client.send(request).await?;

        if !response.status().is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;

            anyhow::bail!(
                "Failed to get authenticated user.\nStatus: {:?}\nBody: {body}",
                response.status()
            )
        }

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        let response: GetAuthenticatedUserResponse = serde_json::from_str(&body)?;

        Ok(response.user)
    }
}
