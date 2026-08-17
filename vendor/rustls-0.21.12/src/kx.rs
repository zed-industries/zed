use std::fmt;

use crate::error::{Error, PeerMisbehaved};
use crate::msgs::enums::NamedGroup;

/// An in-progress key exchange.  This has the algorithm,
/// our private key, and our public key.
pub(crate) struct KeyExchange {
    skxg: &'static SupportedKxGroup,
    privkey: ring::agreement::EphemeralPrivateKey,
    pub(crate) pubkey: ring::agreement::PublicKey,
}

impl KeyExchange {
    /// Choose a SupportedKxGroup by name, from a list of supported groups.
    pub(crate) fn choose(
        name: NamedGroup,
        supported: &[&'static SupportedKxGroup],
    ) -> Option<&'static SupportedKxGroup> {
        supported
            .iter()
            .find(|skxg| skxg.name == name)
            .cloned()
    }

    /// Start a key exchange, using the given SupportedKxGroup.
    ///
    /// This generates an ephemeral key pair and stores it in the returned KeyExchange object.
    pub(crate) fn start(skxg: &'static SupportedKxGroup) -> Option<Self> {
        let rng = ring::rand::SystemRandom::new();
        let ours =
            ring::agreement::EphemeralPrivateKey::generate(skxg.agreement_algorithm, &rng).ok()?;

        let pubkey = ours.compute_public_key().ok()?;

        Some(Self {
            skxg,
            privkey: ours,
            pubkey,
        })
    }

    /// Return the group being used.
    pub(crate) fn group(&self) -> NamedGroup {
        self.skxg.name
    }

    /// Completes the key exchange, given the peer's public key.
    ///
    /// The shared secret is passed into the closure passed down in `f`, and the result of calling
    /// `f` is returned to the caller.
    pub(crate) fn complete<T>(self, peer: &[u8], f: impl FnOnce(&[u8]) -> T) -> Result<T, Error> {
        let peer_key = ring::agreement::UnparsedPublicKey::new(self.skxg.agreement_algorithm, peer);
        ring::agreement::agree_ephemeral(self.privkey, &peer_key, f)
            .map_err(|_| PeerMisbehaved::InvalidKeyShare.into())
    }
}

/// A key-exchange group supported by rustls.
///
/// All possible instances of this class are provided by the library in
/// the `ALL_KX_GROUPS` array.
pub struct SupportedKxGroup {
    /// The IANA "TLS Supported Groups" name of the group
    pub name: NamedGroup,

    /// The corresponding ring agreement::Algorithm
    agreement_algorithm: &'static ring::agreement::Algorithm,
}

impl fmt::Debug for SupportedKxGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)
    }
}

/// Ephemeral ECDH on curve25519 (see RFC7748)
pub static X25519: SupportedKxGroup = SupportedKxGroup {
    name: NamedGroup::X25519,
    agreement_algorithm: &ring::agreement::X25519,
};

/// Ephemeral ECDH on secp256r1 (aka NIST-P256)
pub static SECP256R1: SupportedKxGroup = SupportedKxGroup {
    name: NamedGroup::secp256r1,
    agreement_algorithm: &ring::agreement::ECDH_P256,
};

/// Ephemeral ECDH on secp384r1 (aka NIST-P384)
pub static SECP384R1: SupportedKxGroup = SupportedKxGroup {
    name: NamedGroup::secp384r1,
    agreement_algorithm: &ring::agreement::ECDH_P384,
};

/// A list of all the key exchange groups supported by rustls.
pub static ALL_KX_GROUPS: [&SupportedKxGroup; 3] = [&X25519, &SECP256R1, &SECP384R1];
