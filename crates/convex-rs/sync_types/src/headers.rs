use headers::{
    authorization::Credentials,
    HeaderValue,
};

pub static DEPRECATION_STATE_HEADER_NAME: &str = "x-convex-deprecation-state";
pub static DEPRECATION_MSG_HEADER_NAME: &str = "x-convex-deprecation-message";

#[derive(Debug)]
pub struct ConvexAdminAuthorization(HeaderValue);

impl ConvexAdminAuthorization {
    pub fn from_admin_key(admin_key: &str) -> anyhow::Result<Self> {
        Ok(Self(HeaderValue::from_str(&format!("Convex {admin_key}"))?))
    }
}

impl Credentials for ConvexAdminAuthorization {
    const SCHEME: &'static str = "Convex";

    fn decode(value: &HeaderValue) -> Option<Self> {
        debug_assert!(
            value.as_bytes().starts_with(b"Convex "),
            "HeaderValue to decode should start with \"Convex ..\", received = {value:?}",
        );

        Some(Self(value.clone()))
    }

    fn encode(&self) -> HeaderValue {
        (&self.0).into()
    }
}
