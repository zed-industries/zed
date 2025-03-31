pub mod proto;
pub mod token;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use prost::Message;
use reqwest::header::CONTENT_TYPE;
use std::{future::Future, sync::Arc, time::Duration};

#[async_trait]
pub trait Client: Send + Sync {
    fn url(&self) -> &str;
    async fn create_room(&self, name: String) -> Result<()>;
    async fn delete_room(&self, name: String) -> Result<()>;
    async fn remove_participant(&self, room: String, identity: String) -> Result<()>;
    async fn update_participant(
        &self,
        room: String,
        identity: String,
        permission: proto::ParticipantPermission,
    ) -> Result<()>;
    fn room_token(&self, room: &str, identity: &str) -> Result<String>;
    fn guest_token(&self, room: &str, identity: &str) -> Result<String>;
}

pub struct LiveKitParticipantUpdate {}

#[derive(Clone)]
pub struct LiveKitClient {
    http: reqwest::Client,
    url: Arc<str>,
    key: Arc<str>,
    secret: Arc<str>,
}

impl LiveKitClient {
    pub fn new(mut url: String, key: String, secret: String) -> Self {
        if url.ends_with('/') {
            url.pop();
        }

        Self {
            http: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
            url: url.into(),
            key: key.into(),
            secret: secret.into(),
        }
    }

    fn request<Req, Res>(
        &self,
        path: &str,
        grant: token::VideoGrant,
        body: Req,
    ) -> impl Future<Output = Result<Res>>
    where
        Req: Message,
        Res: Default + Message,
    {
        let client = self.http.clone();
        let token = token::create(&self.key, &self.secret, None, grant);
        let url = format!("{}/{}", self.url, path);
        log::info!("Request {}: {:?}", url, body);
        async move {
            let token = token?;
            let response = client
                .post(&url)
                .header(CONTENT_TYPE, "application/protobuf")
                .bearer_auth(token)
                .body(body.encode_to_vec())
                .send()
                .await?;

            if response.status().is_success() {
                log::info!("Response {}: {:?}", url, response.status());
                Ok(Res::decode(response.bytes().await?)?)
            } else {
                log::error!("Response {}: {:?}", url, response.status());
                Err(anyhow!(
                    "POST {} failed with status code {:?}, {:?}",
                    url,
                    response.status(),
                    response.text().await
                ))
            }
        }
    }
}

#[async_trait]
impl Client for LiveKitClient {
    fn url(&self) -> &str {
        &self.url
    }

    async fn create_room(&self, name: String) -> Result<()> {
        let _: proto::Room = self
            .request(
                "twirp/livekit.RoomService/CreateRoom",
                token::VideoGrant {
                    room_create: Some(true),
                    ..Default::default()
                },
                proto::CreateRoomRequest {
                    name,
                    ..Default::default()
                },
            )
            .await?;
        Ok(())
    }

    async fn delete_room(&self, name: String) -> Result<()> {
        let _: proto::DeleteRoomResponse = self
            .request(
                "twirp/livekit.RoomService/DeleteRoom",
                token::VideoGrant {
                    room_create: Some(true),
                    ..Default::default()
                },
                proto::DeleteRoomRequest { room: name },
            )
            .await?;
        Ok(())
    }

    async fn remove_participant(&self, room: String, identity: String) -> Result<()> {
        let _: proto::RemoveParticipantResponse = self
            .request(
                "twirp/livekit.RoomService/RemoveParticipant",
                token::VideoGrant::to_admin(&room),
                proto::RoomParticipantIdentity {
                    room: room.clone(),
                    identity,
                },
            )
            .await?;
        Ok(())
    }

    async fn update_participant(
        &self,
        room: String,
        identity: String,
        permission: proto::ParticipantPermission,
    ) -> Result<()> {
        let _: proto::ParticipantInfo = self
            .request(
                "twirp/livekit.RoomService/UpdateParticipant",
                token::VideoGrant::to_admin(&room),
                proto::UpdateParticipantRequest {
                    room: room.clone(),
                    identity,
                    metadata: "".to_string(),
                    permission: Some(permission),
                },
            )
            .await?;
        Ok(())
    }

    fn room_token(&self, room: &str, identity: &str) -> Result<String> {
        token::create(
            &self.key,
            &self.secret,
            Some(identity),
            token::VideoGrant::to_join(room),
        )
    }

    fn guest_token(&self, room: &str, identity: &str) -> Result<String> {
        token::create(
            &self.key,
            &self.secret,
            Some(identity),
            token::VideoGrant::for_guest(room),
        )
    }
}
