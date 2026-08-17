use crate::cipher::{MessageDecrypter, MessageEncrypter};
use crate::common_state::{CommonState, Side};
use crate::conn::ConnectionRandoms;
use crate::enums::{AlertDescription, CipherSuite, SignatureScheme};
use crate::error::{Error, InvalidMessage};
use crate::kx;
use crate::msgs::codec::{Codec, Reader};
use crate::msgs::handshake::KeyExchangeAlgorithm;
use crate::suites::{BulkAlgorithm, CipherSuiteCommon, SupportedCipherSuite};
#[cfg(feature = "secret_extraction")]
use crate::suites::{ConnectionTrafficSecrets, PartiallyExtractedSecrets};

use ring::aead;
use ring::digest::Digest;

use std::fmt;

mod cipher;
pub(crate) use cipher::{AesGcm, ChaCha20Poly1305, Tls12AeadAlgorithm};

mod prf;

/// The TLS1.2 ciphersuite TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256.
pub static TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256: SupportedCipherSuite =
    SupportedCipherSuite::Tls12(&Tls12CipherSuite {
        common: CipherSuiteCommon {
            suite: CipherSuite::TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,
            bulk: BulkAlgorithm::Chacha20Poly1305,
            aead_algorithm: &aead::CHACHA20_POLY1305,
        },
        kx: KeyExchangeAlgorithm::ECDHE,
        sign: TLS12_ECDSA_SCHEMES,
        fixed_iv_len: 12,
        explicit_nonce_len: 0,
        aead_alg: &ChaCha20Poly1305,
        hmac_algorithm: ring::hmac::HMAC_SHA256,
    });

/// The TLS1.2 ciphersuite TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256
pub static TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256: SupportedCipherSuite =
    SupportedCipherSuite::Tls12(&Tls12CipherSuite {
        common: CipherSuiteCommon {
            suite: CipherSuite::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
            bulk: BulkAlgorithm::Chacha20Poly1305,
            aead_algorithm: &aead::CHACHA20_POLY1305,
        },
        kx: KeyExchangeAlgorithm::ECDHE,
        sign: TLS12_RSA_SCHEMES,
        fixed_iv_len: 12,
        explicit_nonce_len: 0,
        aead_alg: &ChaCha20Poly1305,
        hmac_algorithm: ring::hmac::HMAC_SHA256,
    });

/// The TLS1.2 ciphersuite TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
pub static TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256: SupportedCipherSuite =
    SupportedCipherSuite::Tls12(&Tls12CipherSuite {
        common: CipherSuiteCommon {
            suite: CipherSuite::TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
            bulk: BulkAlgorithm::Aes128Gcm,
            aead_algorithm: &aead::AES_128_GCM,
        },
        kx: KeyExchangeAlgorithm::ECDHE,
        sign: TLS12_RSA_SCHEMES,
        fixed_iv_len: 4,
        explicit_nonce_len: 8,
        aead_alg: &AesGcm,
        hmac_algorithm: ring::hmac::HMAC_SHA256,
    });

/// The TLS1.2 ciphersuite TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384
pub static TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384: SupportedCipherSuite =
    SupportedCipherSuite::Tls12(&Tls12CipherSuite {
        common: CipherSuiteCommon {
            suite: CipherSuite::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
            bulk: BulkAlgorithm::Aes256Gcm,
            aead_algorithm: &aead::AES_256_GCM,
        },
        kx: KeyExchangeAlgorithm::ECDHE,
        sign: TLS12_RSA_SCHEMES,
        fixed_iv_len: 4,
        explicit_nonce_len: 8,
        aead_alg: &AesGcm,
        hmac_algorithm: ring::hmac::HMAC_SHA384,
    });

/// The TLS1.2 ciphersuite TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256
pub static TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: SupportedCipherSuite =
    SupportedCipherSuite::Tls12(&Tls12CipherSuite {
        common: CipherSuiteCommon {
            suite: CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
            bulk: BulkAlgorithm::Aes128Gcm,
            aead_algorithm: &aead::AES_128_GCM,
        },
        kx: KeyExchangeAlgorithm::ECDHE,
        sign: TLS12_ECDSA_SCHEMES,
        fixed_iv_len: 4,
        explicit_nonce_len: 8,
        aead_alg: &AesGcm,
        hmac_algorithm: ring::hmac::HMAC_SHA256,
    });

/// The TLS1.2 ciphersuite TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384
pub static TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: SupportedCipherSuite =
    SupportedCipherSuite::Tls12(&Tls12CipherSuite {
        common: CipherSuiteCommon {
            suite: CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
            bulk: BulkAlgorithm::Aes256Gcm,
            aead_algorithm: &aead::AES_256_GCM,
        },
        kx: KeyExchangeAlgorithm::ECDHE,
        sign: TLS12_ECDSA_SCHEMES,
        fixed_iv_len: 4,
        explicit_nonce_len: 8,
        aead_alg: &AesGcm,
        hmac_algorithm: ring::hmac::HMAC_SHA384,
    });

static TLS12_ECDSA_SCHEMES: &[SignatureScheme] = &[
    SignatureScheme::ED25519,
    SignatureScheme::ECDSA_NISTP521_SHA512,
    SignatureScheme::ECDSA_NISTP384_SHA384,
    SignatureScheme::ECDSA_NISTP256_SHA256,
];

static TLS12_RSA_SCHEMES: &[SignatureScheme] = &[
    SignatureScheme::RSA_PSS_SHA512,
    SignatureScheme::RSA_PSS_SHA384,
    SignatureScheme::RSA_PSS_SHA256,
    SignatureScheme::RSA_PKCS1_SHA512,
    SignatureScheme::RSA_PKCS1_SHA384,
    SignatureScheme::RSA_PKCS1_SHA256,
];

/// A TLS 1.2 cipher suite supported by rustls.
pub struct Tls12CipherSuite {
    /// Common cipher suite fields.
    pub common: CipherSuiteCommon,
    pub(crate) hmac_algorithm: ring::hmac::Algorithm,
    /// How to exchange/agree keys.
    pub kx: KeyExchangeAlgorithm,

    /// How to sign messages for authentication.
    pub sign: &'static [SignatureScheme],

    /// How long the fixed part of the 'IV' is.
    ///
    /// This isn't usually an IV, but we continue the
    /// terminology misuse to match the standard.
    pub fixed_iv_len: usize,

    /// This is a non-standard extension which extends the
    /// key block to provide an initial explicit nonce offset,
    /// in a deterministic and safe way.  GCM needs this,
    /// chacha20poly1305 works this way by design.
    pub explicit_nonce_len: usize,

    pub(crate) aead_alg: &'static dyn Tls12AeadAlgorithm,
}

impl Tls12CipherSuite {
    /// Resolve the set of supported `SignatureScheme`s from the
    /// offered `SupportedSignatureSchemes`.  If we return an empty
    /// set, the handshake terminates.
    pub fn resolve_sig_schemes(&self, offered: &[SignatureScheme]) -> Vec<SignatureScheme> {
        self.sign
            .iter()
            .filter(|pref| offered.contains(pref))
            .cloned()
            .collect()
    }

    /// Which hash function to use with this suite.
    pub(crate) fn hash_algorithm(&self) -> &'static ring::digest::Algorithm {
        self.hmac_algorithm.digest_algorithm()
    }
}

impl From<&'static Tls12CipherSuite> for SupportedCipherSuite {
    fn from(s: &'static Tls12CipherSuite) -> Self {
        Self::Tls12(s)
    }
}

impl PartialEq for Tls12CipherSuite {
    fn eq(&self, other: &Self) -> bool {
        self.common.suite == other.common.suite
    }
}

impl fmt::Debug for Tls12CipherSuite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tls12CipherSuite")
            .field("suite", &self.common.suite)
            .field("bulk", &self.common.bulk)
            .finish()
    }
}

/// TLS1.2 per-connection keying material
pub(crate) struct ConnectionSecrets {
    pub(crate) randoms: ConnectionRandoms,
    suite: &'static Tls12CipherSuite,
    pub(crate) master_secret: [u8; 48],
}

impl ConnectionSecrets {
    pub(crate) fn from_key_exchange(
        kx: kx::KeyExchange,
        peer_pub_key: &[u8],
        ems_seed: Option<Digest>,
        randoms: ConnectionRandoms,
        suite: &'static Tls12CipherSuite,
    ) -> Result<Self, Error> {
        let mut ret = Self {
            randoms,
            suite,
            master_secret: [0u8; 48],
        };

        let (label, seed) = match ems_seed {
            Some(seed) => ("extended master secret", Seed::Ems(seed)),
            None => (
                "master secret",
                Seed::Randoms(join_randoms(&ret.randoms.client, &ret.randoms.server)),
            ),
        };

        kx.complete(peer_pub_key, |secret| {
            prf::prf(
                &mut ret.master_secret,
                suite.hmac_algorithm,
                secret,
                label.as_bytes(),
                seed.as_ref(),
            );
        })?;

        Ok(ret)
    }

    pub(crate) fn new_resume(
        randoms: ConnectionRandoms,
        suite: &'static Tls12CipherSuite,
        master_secret: &[u8],
    ) -> Self {
        let mut ret = Self {
            randoms,
            suite,
            master_secret: [0u8; 48],
        };
        ret.master_secret
            .copy_from_slice(master_secret);
        ret
    }

    /// Make a `MessageCipherPair` based on the given supported ciphersuite `scs`,
    /// and the session's `secrets`.
    pub(crate) fn make_cipher_pair(&self, side: Side) -> MessageCipherPair {
        fn split_key<'a>(
            key_block: &'a [u8],
            alg: &'static aead::Algorithm,
        ) -> (aead::LessSafeKey, &'a [u8]) {
            // Might panic if the key block is too small.
            let (key, rest) = key_block.split_at(alg.key_len());
            // Won't panic because its only prerequisite is that `key` is `alg.key_len()` bytes long.
            let key = aead::UnboundKey::new(alg, key).unwrap();
            (aead::LessSafeKey::new(key), rest)
        }

        // Make a key block, and chop it up.
        // nb. we don't implement any ciphersuites with nonzero mac_key_len.
        let key_block = self.make_key_block();

        let suite = self.suite;
        let scs = &suite.common;

        let (client_write_key, key_block) = split_key(&key_block, scs.aead_algorithm);
        let (server_write_key, key_block) = split_key(key_block, scs.aead_algorithm);
        let (client_write_iv, key_block) = key_block.split_at(suite.fixed_iv_len);
        let (server_write_iv, extra) = key_block.split_at(suite.fixed_iv_len);

        let (write_key, write_iv, read_key, read_iv) = match side {
            Side::Client => (
                client_write_key,
                client_write_iv,
                server_write_key,
                server_write_iv,
            ),
            Side::Server => (
                server_write_key,
                server_write_iv,
                client_write_key,
                client_write_iv,
            ),
        };

        (
            suite
                .aead_alg
                .decrypter(read_key, read_iv),
            suite
                .aead_alg
                .encrypter(write_key, write_iv, extra),
        )
    }

    fn make_key_block(&self) -> Vec<u8> {
        let suite = &self.suite;
        let common = &self.suite.common;

        let len =
            (common.aead_algorithm.key_len() + suite.fixed_iv_len) * 2 + suite.explicit_nonce_len;

        let mut out = vec![0u8; len];

        // NOTE: opposite order to above for no good reason.
        // Don't design security protocols on drugs, kids.
        let randoms = join_randoms(&self.randoms.server, &self.randoms.client);
        prf::prf(
            &mut out,
            self.suite.hmac_algorithm,
            &self.master_secret,
            b"key expansion",
            &randoms,
        );

        out
    }

    pub(crate) fn suite(&self) -> &'static Tls12CipherSuite {
        self.suite
    }

    pub(crate) fn get_master_secret(&self) -> Vec<u8> {
        let mut ret = Vec::new();
        ret.extend_from_slice(&self.master_secret);
        ret
    }

    fn make_verify_data(&self, handshake_hash: &Digest, label: &[u8]) -> Vec<u8> {
        let mut out = vec![0u8; 12];

        prf::prf(
            &mut out,
            self.suite.hmac_algorithm,
            &self.master_secret,
            label,
            handshake_hash.as_ref(),
        );
        out
    }

    pub(crate) fn client_verify_data(&self, handshake_hash: &Digest) -> Vec<u8> {
        self.make_verify_data(handshake_hash, b"client finished")
    }

    pub(crate) fn server_verify_data(&self, handshake_hash: &Digest) -> Vec<u8> {
        self.make_verify_data(handshake_hash, b"server finished")
    }

    pub(crate) fn export_keying_material(
        &self,
        output: &mut [u8],
        label: &[u8],
        context: Option<&[u8]>,
    ) {
        let mut randoms = Vec::new();
        randoms.extend_from_slice(&self.randoms.client);
        randoms.extend_from_slice(&self.randoms.server);
        if let Some(context) = context {
            assert!(context.len() <= 0xffff);
            (context.len() as u16).encode(&mut randoms);
            randoms.extend_from_slice(context);
        }

        prf::prf(
            output,
            self.suite.hmac_algorithm,
            &self.master_secret,
            label,
            &randoms,
        );
    }

    #[cfg(feature = "secret_extraction")]
    pub(crate) fn extract_secrets(&self, side: Side) -> Result<PartiallyExtractedSecrets, Error> {
        // Make a key block, and chop it up
        let key_block = self.make_key_block();

        let suite = self.suite;
        let algo = suite.common.aead_algorithm;

        let (client_key, key_block) = key_block.split_at(algo.key_len());
        let (server_key, key_block) = key_block.split_at(algo.key_len());
        let (client_iv, key_block) = key_block.split_at(suite.fixed_iv_len);
        let (server_iv, extra) = key_block.split_at(suite.fixed_iv_len);

        // A key/IV pair (fixed IV len is 4 for GCM, 12 for Chacha)
        struct Pair<'a> {
            key: &'a [u8],
            iv: &'a [u8],
        }

        let client_pair = Pair {
            key: client_key,
            iv: client_iv,
        };
        let server_pair = Pair {
            key: server_key,
            iv: server_iv,
        };

        let (client_secrets, server_secrets) = if algo == &aead::AES_128_GCM {
            let extract = |pair: Pair| -> ConnectionTrafficSecrets {
                let mut key = [0u8; 16];
                key.copy_from_slice(pair.key);

                let mut salt = [0u8; 4];
                salt.copy_from_slice(pair.iv);

                let mut iv = [0u8; 8];
                iv.copy_from_slice(&extra[..8]);

                ConnectionTrafficSecrets::Aes128Gcm { key, salt, iv }
            };

            (extract(client_pair), extract(server_pair))
        } else if algo == &aead::AES_256_GCM {
            let extract = |pair: Pair| -> ConnectionTrafficSecrets {
                let mut key = [0u8; 32];
                key.copy_from_slice(pair.key);

                let mut salt = [0u8; 4];
                salt.copy_from_slice(pair.iv);

                let mut iv = [0u8; 8];
                iv.copy_from_slice(&extra[..8]);

                ConnectionTrafficSecrets::Aes256Gcm { key, salt, iv }
            };

            (extract(client_pair), extract(server_pair))
        } else if algo == &aead::CHACHA20_POLY1305 {
            let extract = |pair: Pair| -> ConnectionTrafficSecrets {
                let mut key = [0u8; 32];
                key.copy_from_slice(pair.key);

                let mut iv = [0u8; 12];
                iv.copy_from_slice(pair.iv);

                ConnectionTrafficSecrets::Chacha20Poly1305 { key, iv }
            };

            (extract(client_pair), extract(server_pair))
        } else {
            return Err(Error::General(format!(
                "exporting secrets for {:?}: unimplemented",
                algo
            )));
        };

        let (tx, rx) = match side {
            Side::Client => (client_secrets, server_secrets),
            Side::Server => (server_secrets, client_secrets),
        };
        Ok(PartiallyExtractedSecrets { tx, rx })
    }
}

enum Seed {
    Ems(Digest),
    Randoms([u8; 64]),
}

impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Ems(seed) => seed.as_ref(),
            Self::Randoms(randoms) => randoms.as_ref(),
        }
    }
}

fn join_randoms(first: &[u8; 32], second: &[u8; 32]) -> [u8; 64] {
    let mut randoms = [0u8; 64];
    randoms[..32].copy_from_slice(first);
    randoms[32..].copy_from_slice(second);
    randoms
}

type MessageCipherPair = (Box<dyn MessageDecrypter>, Box<dyn MessageEncrypter>);

pub(crate) fn decode_ecdh_params<T: Codec>(
    common: &mut CommonState,
    kx_params: &[u8],
) -> Result<T, Error> {
    let mut rd = Reader::init(kx_params);
    let ecdh_params = T::read(&mut rd)?;
    match rd.any_left() {
        false => Ok(ecdh_params),
        true => Err(common.send_fatal_alert(
            AlertDescription::DecodeError,
            InvalidMessage::InvalidDhParams,
        )),
    }
}

pub(crate) const DOWNGRADE_SENTINEL: [u8; 8] = [0x44, 0x4f, 0x57, 0x4e, 0x47, 0x52, 0x44, 0x01];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgs::handshake::{ClientECDHParams, ServerECDHParams};

    #[test]
    fn server_ecdhe_remaining_bytes() {
        let key = kx::KeyExchange::start(&kx::X25519).unwrap();
        let server_params = ServerECDHParams::new(key.group(), key.pubkey.as_ref());
        let mut server_buf = Vec::new();
        server_params.encode(&mut server_buf);
        server_buf.push(34);

        let mut common = CommonState::new(Side::Client);
        assert!(decode_ecdh_params::<ServerECDHParams>(&mut common, &server_buf).is_err());
    }

    #[test]
    fn client_ecdhe_invalid() {
        let mut common = CommonState::new(Side::Server);
        assert!(decode_ecdh_params::<ClientECDHParams>(&mut common, &[34]).is_err());
    }
}
