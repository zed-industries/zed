/// The `Sec-Websocket-Version` header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SecWebsocketVersion(u8);

impl SecWebsocketVersion {
    /// `Sec-Websocket-Version: 13`
    pub const V13: SecWebsocketVersion = SecWebsocketVersion(13);
}

impl ::Header for SecWebsocketVersion {
    fn name() -> &'static ::HeaderName {
        &::http::header::SEC_WEBSOCKET_VERSION
    }

    fn decode<'i, I: Iterator<Item = &'i ::HeaderValue>>(values: &mut I) -> Result<Self, ::Error> {
        values
            .next()
            .and_then(|value| {
                if value == "13" {
                    Some(SecWebsocketVersion::V13)
                } else {
                    None
                }
            })
            .ok_or_else(::Error::invalid)
    }

    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        debug_assert_eq!(self.0, 13);

        values.extend(::std::iter::once(::HeaderValue::from_static("13")));
    }
}

#[cfg(test)]
mod tests {
    use super::super::{test_decode, test_encode};
    use super::SecWebsocketVersion;

    #[test]
    fn decode_v13() {
        assert_eq!(
            test_decode::<SecWebsocketVersion>(&["13"]),
            Some(SecWebsocketVersion::V13),
        );
    }

    #[test]
    fn decode_fail() {
        assert_eq!(test_decode::<SecWebsocketVersion>(&["1"]), None,);
    }

    #[test]
    fn encode_v13() {
        let headers = test_encode(SecWebsocketVersion::V13);
        assert_eq!(headers["sec-websocket-version"], "13");
    }
}
