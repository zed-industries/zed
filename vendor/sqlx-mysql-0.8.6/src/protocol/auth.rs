use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, Copy, Clone)]
// These have all the same suffix but they match the auth plugin names.
#[allow(clippy::enum_variant_names)]
pub enum AuthPlugin {
    MySqlNativePassword,
    CachingSha2Password,
    Sha256Password,
    MySqlClearPassword,
}

impl AuthPlugin {
    pub(crate) fn name(self) -> &'static str {
        match self {
            AuthPlugin::MySqlNativePassword => "mysql_native_password",
            AuthPlugin::CachingSha2Password => "caching_sha2_password",
            AuthPlugin::Sha256Password => "sha256_password",
            AuthPlugin::MySqlClearPassword => "mysql_clear_password",
        }
    }
}

impl FromStr for AuthPlugin {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mysql_native_password" => Ok(AuthPlugin::MySqlNativePassword),
            "caching_sha2_password" => Ok(AuthPlugin::CachingSha2Password),
            "sha256_password" => Ok(AuthPlugin::Sha256Password),
            "mysql_clear_password" => Ok(AuthPlugin::MySqlClearPassword),

            _ => Err(err_protocol!("unknown authentication plugin: {}", s)),
        }
    }
}
