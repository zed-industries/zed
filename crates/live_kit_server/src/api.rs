use crate::{proto, token};
use anyhow::{anyhow, Result};
use hyper::{client::HttpConnector, header::AUTHORIZATION, Method, Request, Uri};
use std::future::Future;

pub struct Client {
    http: hyper::Client<HttpConnector>,
    uri: Uri,
    key: String,
    secret: String,
}

impl Client {
    pub fn new(uri: Uri, key: String, secret: String) -> Self {
        assert!(uri.scheme().is_some(), "base uri must have a scheme");
        assert!(uri.authority().is_some(), "base uri must have an authority");
        Self {
            http: hyper::Client::new(),
            uri,
            key,
            secret,
        }
    }

    pub fn create_room(&self, name: String) -> impl Future<Output = Result<proto::Room>> {
        let token = token::create(
            &self.key,
            &self.secret,
            None,
            token::VideoGrant {
                room_create: Some(true),
                ..Default::default()
            },
        );

        let client = self.http.clone();
        let uri = Uri::builder()
            .scheme(self.uri.scheme().unwrap().clone())
            .authority(self.uri.authority().unwrap().clone())
            .path_and_query("twirp/livekit.RoomService/CreateRoom")
            .build();
        async move {
            let token = token?;
            let uri = uri?;
            let body = proto::CreateRoomRequest {
                name: todo!(),
                empty_timeout: todo!(),
                max_participants: todo!(),
                node_id: todo!(),
                metadata: todo!(),
                egress: todo!(),
            };
            let mut request = Request::builder()
                .uri(uri)
                .method(Method::POST)
                .header(AUTHORIZATION, format!("Bearer {}", token))
                .body(body);
            Err(anyhow!("yeah"))
        }
    }
}
