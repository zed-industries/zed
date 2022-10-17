use crate::{proto, token};
use anyhow::{anyhow, Result};
use prost::Message;
use reqwest::header::CONTENT_TYPE;
use std::{future::Future, sync::Arc};

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    uri: Arc<str>,
    key: Arc<str>,
    secret: Arc<str>,
}

impl Client {
    pub fn new(mut uri: String, key: String, secret: String) -> Self {
        if uri.ends_with('/') {
            uri.pop();
        }

        Self {
            http: reqwest::Client::new(),
            uri: uri.into(),
            key: key.into(),
            secret: secret.into(),
        }
    }

    pub fn create_room(&self, name: String) -> impl Future<Output = Result<proto::Room>> {
        self.request(
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
    }

    pub fn delete_room(&self, name: String) -> impl Future<Output = Result<()>> {
        let response = self.request(
            "twirp/livekit.RoomService/DeleteRoom",
            token::VideoGrant {
                room_create: Some(true),
                ..Default::default()
            },
            proto::DeleteRoomRequest { room: name },
        );
        async move {
            let _: proto::DeleteRoomResponse = response.await?;
            Ok(())
        }
    }

    pub fn remove_participant(
        &self,
        room: String,
        identity: String,
    ) -> impl Future<Output = Result<()>> {
        let response = self.request(
            "twirp/livekit.RoomService/RemoveParticipant",
            token::VideoGrant {
                room_admin: Some(true),
                ..Default::default()
            },
            proto::RoomParticipantIdentity { room, identity },
        );
        async move {
            let _: proto::RemoveParticipantResponse = response.await?;
            Ok(())
        }
    }

    pub fn room_token_for_user(&self, room: &str, identity: &str) -> Result<String> {
        token::create(
            &self.key,
            &self.secret,
            Some(identity),
            token::VideoGrant {
                room: Some(room),
                room_join: Some(true),
                can_publish: Some(true),
                can_subscribe: Some(true),
                ..Default::default()
            },
        )
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
        let uri = format!("{}/{}", self.uri, path);
        async move {
            let token = token?;
            let response = client
                .post(&uri)
                .header(CONTENT_TYPE, "application/protobuf")
                .bearer_auth(token)
                .body(body.encode_to_vec())
                .send()
                .await?;
            if response.status().is_success() {
                Ok(Res::decode(response.bytes().await?)?)
            } else {
                Err(anyhow!(
                    "POST {} failed with status code {:?}, {:?}",
                    uri,
                    response.status(),
                    response.text().await
                ))
            }
        }
    }
}
