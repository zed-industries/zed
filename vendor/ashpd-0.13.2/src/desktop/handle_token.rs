use std::{
    convert::TryFrom,
    fmt::{self, Debug, Display},
};

use serde::{Deserialize, Serialize};
use zbus::{names::OwnedMemberName, zvariant::Type};

/// A handle token is a DBus Object Path element.
///
/// Specified in the [`Request`](crate::desktop::Request)  or
/// [`Session`](crate::desktop::Session) object path following this format
/// `/org/freedesktop/portal/desktop/request/SENDER/TOKEN` where sender is the
/// caller's unique name and token is the [`HandleToken`].
///
/// A valid object path element must only contain the ASCII characters
/// `[A-Z][a-z][0-9]_`
#[derive(Serialize, Type, PartialEq, Eq, Hash, Clone)]
pub struct HandleToken(OwnedMemberName);

impl Display for HandleToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Debug for HandleToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("HandleToken")
            .field(&self.0.as_str())
            .finish()
    }
}

impl Default for HandleToken {
    fn default() -> Self {
        const ALPHANUMERIC: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

        let mut token = String::with_capacity(16); // "ashpd_" + 10 chars
        token.push_str("ashpd_");

        let mut rnd_bytes = [0u8; 10];
        getrandom::fill(&mut rnd_bytes).expect("failed to generate random bytes");

        for byte in rnd_bytes.iter() {
            let idx = (*byte as usize) % ALPHANUMERIC.len();
            token.push(ALPHANUMERIC[idx] as char);
        }

        Self(OwnedMemberName::try_from(token).unwrap())
    }
}

#[derive(Debug)]
pub struct HandleInvalidCharacter(char);

impl std::fmt::Display for HandleInvalidCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Invalid Character {}", self.0))
    }
}

impl std::error::Error for HandleInvalidCharacter {}

impl std::str::FromStr for HandleToken {
    type Err = HandleInvalidCharacter;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        for char in value.chars() {
            if !char.is_ascii_alphanumeric() && char != '_' {
                return Err(HandleInvalidCharacter(char));
            }
        }
        Ok(Self(OwnedMemberName::try_from(value).unwrap()))
    }
}

impl TryFrom<String> for HandleToken {
    type Error = HandleInvalidCharacter;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse::<Self>()
    }
}

impl TryFrom<&str> for HandleToken {
    type Error = HandleInvalidCharacter;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse::<Self>()
    }
}

#[cfg(feature = "backend")]
impl TryFrom<&zbus::zvariant::OwnedObjectPath> for HandleToken {
    type Error = HandleInvalidCharacter;

    fn try_from(value: &zbus::zvariant::OwnedObjectPath) -> Result<Self, Self::Error> {
        let base_segment = value
            .as_str()
            .split('/')
            .next_back()
            .expect("A valid request ObjectPath");
        HandleToken::try_from(base_segment)
    }
}

impl<'de> Deserialize<'de> for HandleToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let token = String::deserialize(deserializer)?;
        token
            .parse::<Self>()
            .map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}
#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::HandleToken;

    #[test]
    fn handle_token() {
        assert!(HandleToken::from_str("token").is_ok());

        let token = HandleToken::from_str("token2").unwrap();
        assert_eq!(token.to_string(), "token2".to_string());

        assert!(HandleToken::from_str("/test").is_err());

        assert!(HandleToken::from_str("تجربة").is_err());

        assert!(HandleToken::from_str("test_token").is_ok());

        HandleToken::default(); // ensure we don't panic
    }
}
