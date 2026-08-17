/// The `Sec-Websocket-Key` header.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SecWebsocketKey(pub(super) ::HeaderValue);

derive_header! {
    SecWebsocketKey(_),
    name: SEC_WEBSOCKET_KEY
}
