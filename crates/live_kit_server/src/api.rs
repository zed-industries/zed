use crate::token;
use hyper::{client::HttpConnector, Request, Uri};

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
            uri: uri,
            key,
            secret,
        }
    }

    pub fn create_room(&self) {
        // let mut uri = url.clone();
        // uri.set_path_and_query()

        let uri = Uri::builder()
            .scheme(self.uri.scheme().unwrap().clone())
            .authority(self.uri.authority().unwrap().clone())
            .path_and_query("twirp/livekit.RoomService/CreateRoom")
            .build();

        // token::create(api_key, secret_key, room_name, participant_name)
        self.http.request(req)
    }
}
