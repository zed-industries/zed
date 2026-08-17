use crate::cipher::{Iv, IvLen, MessageDecrypter};
use crate::common_state::{CommonState, Side};
use crate::error::Error;
use crate::msgs::base::PayloadU8;
#[cfg(feature = "quic")]
use crate::quic;
#[cfg(feature = "secret_extraction")]
use crate::suites::{ConnectionTrafficSecrets, PartiallyExtractedSecrets};
use crate::{KeyLog, Tls13CipherSuite};

/// Key schedule maintenance for TLS1.3
use ring::{
    aead,
    digest::{self, Digest},
    hkdf::{self, KeyType as _},
    hmac,
};

use super::{Tls13MessageDecrypter, Tls13MessageEncrypter};

/// The kinds of secret we can extract from `KeySchedule`.
#[derive(Debug, Clone, Copy, PartialEq)]
enum SecretKind {
    ResumptionPskBinderKey,
    ClientEarlyTrafficSecret,
    ClientHandshakeTrafficSecret,
    ServerHandshakeTrafficSecret,
    ClientApplicationTrafficSecret,
    ServerApplicationTrafficSecret,
    ExporterMasterSecret,
    ResumptionMasterSecret,
    DerivedSecret,
}

impl SecretKind {
    fn to_bytes(self) -> &'static [u8] {
        use self::SecretKind::*;
        match self {
            ResumptionPskBinderKey => b"res binder",
            ClientEarlyTrafficSecret => b"c e traffic",
            ClientHandshakeTrafficSecret => b"c hs traffic",
            ServerHandshakeTrafficSecret => b"s hs traffic",
            ClientApplicationTrafficSecret => b"c ap traffic",
            ServerApplicationTrafficSecret => b"s ap traffic",
            ExporterMasterSecret => b"exp master",
            ResumptionMasterSecret => b"res master",
            DerivedSecret => b"derived",
        }
    }

    fn log_label(self) -> Option<&'static str> {
        use self::SecretKind::*;
        Some(match self {
            ClientEarlyTrafficSecret => "CLIENT_EARLY_TRAFFIC_SECRET",
            ClientHandshakeTrafficSecret => "CLIENT_HANDSHAKE_TRAFFIC_SECRET",
            ServerHandshakeTrafficSecret => "SERVER_HANDSHAKE_TRAFFIC_SECRET",
            ClientApplicationTrafficSecret => "CLIENT_TRAFFIC_SECRET_0",
            ServerApplicationTrafficSecret => "SERVER_TRAFFIC_SECRET_0",
            ExporterMasterSecret => "EXPORTER_SECRET",
            _ => {
                return None;
            }
        })
    }
}

/// This is the TLS1.3 key schedule.  It stores the current secret and
/// the type of hash.  This isn't used directly; but only through the
/// typestates.
struct KeySchedule {
    current: hkdf::Prk,
    suite: &'static Tls13CipherSuite,
}

// We express the state of a contained KeySchedule using these
// typestates.  This means we can write code that cannot accidentally
// (e.g.) encrypt application data using a KeySchedule solely constructed
// with an empty or trivial secret, or extract the wrong kind of secrets
// at a given point.

/// KeySchedule for early data stage.
pub(crate) struct KeyScheduleEarly {
    ks: KeySchedule,
}

impl KeyScheduleEarly {
    pub(crate) fn new(suite: &'static Tls13CipherSuite, secret: &[u8]) -> Self {
        Self {
            ks: KeySchedule::new(suite, secret),
        }
    }

    pub(crate) fn client_early_traffic_secret(
        &self,
        hs_hash: &Digest,
        key_log: &dyn KeyLog,
        client_random: &[u8; 32],
        common: &mut CommonState,
    ) {
        let client_early_traffic_secret = self.ks.derive_logged_secret(
            SecretKind::ClientEarlyTrafficSecret,
            hs_hash.as_ref(),
            key_log,
            client_random,
        );

        match common.side {
            Side::Client => self
                .ks
                .set_encrypter(&client_early_traffic_secret, common),
            Side::Server => self
                .ks
                .set_decrypter(&client_early_traffic_secret, common),
        }

        #[cfg(feature = "quic")]
        if common.is_quic() {
            // If 0-RTT should be rejected, this will be clobbered by ExtensionProcessing
            // before the application can see.
            common.quic.early_secret = Some(client_early_traffic_secret);
        }
    }

    pub(crate) fn resumption_psk_binder_key_and_sign_verify_data(
        &self,
        hs_hash: &Digest,
    ) -> hmac::Tag {
        let resumption_psk_binder_key = self
            .ks
            .derive_for_empty_hash(SecretKind::ResumptionPskBinderKey);
        self.ks
            .sign_verify_data(&resumption_psk_binder_key, hs_hash)
    }
}

/// Pre-handshake key schedule
///
/// The inner `KeySchedule` is either constructed without any secrets based on ths HKDF algorithm
/// or is extracted from a `KeyScheduleEarly`. This can then be used to derive the `KeyScheduleHandshakeStart`.
pub(crate) struct KeySchedulePreHandshake {
    ks: KeySchedule,
}

impl KeySchedulePreHandshake {
    pub(crate) fn new(suite: &'static Tls13CipherSuite) -> Self {
        Self {
            ks: KeySchedule::new_with_empty_secret(suite),
        }
    }

    pub(crate) fn into_handshake(mut self, secret: &[u8]) -> KeyScheduleHandshakeStart {
        self.ks.input_secret(secret);
        KeyScheduleHandshakeStart { ks: self.ks }
    }
}

impl From<KeyScheduleEarly> for KeySchedulePreHandshake {
    fn from(KeyScheduleEarly { ks }: KeyScheduleEarly) -> Self {
        Self { ks }
    }
}

/// KeySchedule during handshake.
pub(crate) struct KeyScheduleHandshakeStart {
    ks: KeySchedule,
}

impl KeyScheduleHandshakeStart {
    pub(crate) fn derive_client_handshake_secrets(
        mut self,
        early_data_enabled: bool,
        hs_hash: Digest,
        suite: &'static Tls13CipherSuite,
        key_log: &dyn KeyLog,
        client_random: &[u8; 32],
        common: &mut CommonState,
    ) -> KeyScheduleHandshake {
        debug_assert_eq!(common.side, Side::Client);
        // Suite might have changed due to resumption
        self.ks.suite = suite;
        let new = self.into_handshake(hs_hash, key_log, client_random, common);

        // Decrypt with the peer's key, encrypt with our own key
        new.ks
            .set_decrypter(&new.server_handshake_traffic_secret, common);

        if !early_data_enabled {
            // Set the client encryption key for handshakes if early data is not used
            new.ks
                .set_encrypter(&new.client_handshake_traffic_secret, common);
        }

        new
    }

    pub(crate) fn derive_server_handshake_secrets(
        self,
        hs_hash: Digest,
        key_log: &dyn KeyLog,
        client_random: &[u8; 32],
        common: &mut CommonState,
    ) -> KeyScheduleHandshake {
        debug_assert_eq!(common.side, Side::Server);
        let new = self.into_handshake(hs_hash, key_log, client_random, common);

        // Set up to encrypt with handshake secrets, but decrypt with early_data keys.
        // If not doing early_data after all, this is corrected later to the handshake
        // keys (now stored in key_schedule).
        new.ks
            .set_encrypter(&new.server_handshake_traffic_secret, common);
        new
    }

    fn into_handshake(
        self,
        hs_hash: Digest,
        key_log: &dyn KeyLog,
        client_random: &[u8; 32],
        _common: &mut CommonState,
    ) -> KeyScheduleHandshake {
        // Use an empty handshake hash for the initial handshake.
        let client_secret = self.ks.derive_logged_secret(
            SecretKind::ClientHandshakeTrafficSecret,
            hs_hash.as_ref(),
            key_log,
            client_random,
        );

        let server_secret = self.ks.derive_logged_secret(
            SecretKind::ServerHandshakeTrafficSecret,
            hs_hash.as_ref(),
            key_log,
            client_random,
        );

        #[cfg(feature = "quic")]
        if _common.is_quic() {
            _common.quic.hs_secrets = Some(quic::Secrets::new(
                client_secret.clone(),
                server_secret.clone(),
                self.ks.suite,
                _common.side,
                _common.quic.version,
            ));
        }

        KeyScheduleHandshake {
            ks: self.ks,
            client_handshake_traffic_secret: client_secret,
            server_handshake_traffic_secret: server_secret,
        }
    }
}

pub(crate) struct KeyScheduleHandshake {
    ks: KeySchedule,
    client_handshake_traffic_secret: hkdf::Prk,
    server_handshake_traffic_secret: hkdf::Prk,
}

impl KeyScheduleHandshake {
    pub(crate) fn sign_server_finish(&self, hs_hash: &Digest) -> hmac::Tag {
        self.ks
            .sign_finish(&self.server_handshake_traffic_secret, hs_hash)
    }

    pub(crate) fn set_handshake_encrypter(&self, common: &mut CommonState) {
        debug_assert_eq!(common.side, Side::Client);
        self.ks
            .set_encrypter(&self.client_handshake_traffic_secret, common);
    }

    pub(crate) fn set_handshake_decrypter(
        &self,
        skip_requested: Option<usize>,
        common: &mut CommonState,
    ) {
        debug_assert_eq!(common.side, Side::Server);
        let secret = &self.client_handshake_traffic_secret;
        match skip_requested {
            None => self.ks.set_decrypter(secret, common),
            Some(max_early_data_size) => common
                .record_layer
                .set_message_decrypter_with_trial_decryption(
                    self.ks
                        .derive_decrypter(&self.client_handshake_traffic_secret),
                    max_early_data_size,
                ),
        }
    }

    pub(crate) fn into_traffic_with_client_finished_pending(
        self,
        hs_hash: Digest,
        key_log: &dyn KeyLog,
        client_random: &[u8; 32],
        common: &mut CommonState,
    ) -> KeyScheduleTrafficWithClientFinishedPending {
        debug_assert_eq!(common.side, Side::Server);

        let traffic = KeyScheduleTraffic::new(self.ks, hs_hash, key_log, client_random);
        let (_client_secret, server_secret) = (
            &traffic.current_client_traffic_secret,
            &traffic.current_server_traffic_secret,
        );

        traffic
            .ks
            .set_encrypter(server_secret, common);

        #[cfg(feature = "quic")]
        if common.is_quic() {
            common.quic.traffic_secrets = Some(quic::Secrets::new(
                _client_secret.clone(),
                server_secret.clone(),
                traffic.ks.suite,
                common.side,
                common.quic.version,
            ));
        }

        KeyScheduleTrafficWithClientFinishedPending {
            handshake_client_traffic_secret: self.client_handshake_traffic_secret,
            traffic,
        }
    }

    pub(crate) fn into_pre_finished_client_traffic(
        self,
        pre_finished_hash: Digest,
        handshake_hash: Digest,
        key_log: &dyn KeyLog,
        client_random: &[u8; 32],
    ) -> (KeyScheduleClientBeforeFinished, hmac::Tag) {
        let traffic = KeyScheduleTraffic::new(self.ks, pre_finished_hash, key_log, client_random);
        let tag = traffic
            .ks
            .sign_finish(&self.client_handshake_traffic_secret, &handshake_hash);
        (KeyScheduleClientBeforeFinished { traffic }, tag)
    }
}

pub(crate) struct KeyScheduleClientBeforeFinished {
    traffic: KeyScheduleTraffic,
}

impl KeyScheduleClientBeforeFinished {
    pub(crate) fn into_traffic(self, common: &mut CommonState) -> KeyScheduleTraffic {
        debug_assert_eq!(common.side, Side::Client);
        let (client_secret, server_secret) = (
            &self
                .traffic
                .current_client_traffic_secret,
            &self
                .traffic
                .current_server_traffic_secret,
        );

        self.traffic
            .ks
            .set_decrypter(server_secret, common);
        self.traffic
            .ks
            .set_encrypter(client_secret, common);

        #[cfg(feature = "quic")]
        if common.is_quic() {
            common.quic.traffic_secrets = Some(quic::Secrets::new(
                client_secret.clone(),
                server_secret.clone(),
                self.traffic.ks.suite,
                common.side,
                common.quic.version,
            ));
        }

        self.traffic
    }
}

/// KeySchedule during traffic stage, retaining the ability to calculate the client's
/// finished verify_data. The traffic stage key schedule can be extracted from it
/// through signing the client finished hash.
pub(crate) struct KeyScheduleTrafficWithClientFinishedPending {
    handshake_client_traffic_secret: hkdf::Prk,
    traffic: KeyScheduleTraffic,
}

impl KeyScheduleTrafficWithClientFinishedPending {
    pub(crate) fn update_decrypter(&self, common: &mut CommonState) {
        debug_assert_eq!(common.side, Side::Server);
        self.traffic
            .ks
            .set_decrypter(&self.handshake_client_traffic_secret, common);
    }

    pub(crate) fn sign_client_finish(
        self,
        hs_hash: &Digest,
        common: &mut CommonState,
    ) -> (KeyScheduleTraffic, hmac::Tag) {
        debug_assert_eq!(common.side, Side::Server);
        let tag = self
            .traffic
            .ks
            .sign_finish(&self.handshake_client_traffic_secret, hs_hash);

        // Install keying to read future messages.
        self.traffic.ks.set_decrypter(
            &self
                .traffic
                .current_client_traffic_secret,
            common,
        );

        (self.traffic, tag)
    }
}

/// KeySchedule during traffic stage.  All traffic & exporter keys are guaranteed
/// to be available.
pub(crate) struct KeyScheduleTraffic {
    ks: KeySchedule,
    current_client_traffic_secret: hkdf::Prk,
    current_server_traffic_secret: hkdf::Prk,
    current_exporter_secret: hkdf::Prk,
}

impl KeyScheduleTraffic {
    fn new(
        mut ks: KeySchedule,
        hs_hash: Digest,
        key_log: &dyn KeyLog,
        client_random: &[u8; 32],
    ) -> Self {
        ks.input_empty();

        let current_client_traffic_secret = ks.derive_logged_secret(
            SecretKind::ClientApplicationTrafficSecret,
            hs_hash.as_ref(),
            key_log,
            client_random,
        );

        let current_server_traffic_secret = ks.derive_logged_secret(
            SecretKind::ServerApplicationTrafficSecret,
            hs_hash.as_ref(),
            key_log,
            client_random,
        );

        let current_exporter_secret = ks.derive_logged_secret(
            SecretKind::ExporterMasterSecret,
            hs_hash.as_ref(),
            key_log,
            client_random,
        );

        Self {
            ks,
            current_client_traffic_secret,
            current_server_traffic_secret,
            current_exporter_secret,
        }
    }

    pub(crate) fn update_encrypter_and_notify(&mut self, common: &mut CommonState) {
        let secret = self.next_application_traffic_secret(common.side);
        common.enqueue_key_update_notification();
        self.ks.set_encrypter(&secret, common);
    }

    pub(crate) fn update_decrypter(&mut self, common: &mut CommonState) {
        let secret = self.next_application_traffic_secret(common.side.peer());
        self.ks.set_decrypter(&secret, common);
    }

    pub(crate) fn next_application_traffic_secret(&mut self, side: Side) -> hkdf::Prk {
        let current = match side {
            Side::Client => &mut self.current_client_traffic_secret,
            Side::Server => &mut self.current_server_traffic_secret,
        };

        let secret = self.ks.derive_next(current);
        *current = secret.clone();
        secret
    }

    pub(crate) fn resumption_master_secret_and_derive_ticket_psk(
        &self,
        hs_hash: &Digest,
        nonce: &[u8],
    ) -> Vec<u8> {
        let resumption_master_secret = self.ks.derive(
            self.ks.algorithm(),
            SecretKind::ResumptionMasterSecret,
            hs_hash.as_ref(),
        );
        self.ks
            .derive_ticket_psk(&resumption_master_secret, nonce)
    }

    pub(crate) fn export_keying_material(
        &self,
        out: &mut [u8],
        label: &[u8],
        context: Option<&[u8]>,
    ) -> Result<(), Error> {
        self.ks
            .export_keying_material(&self.current_exporter_secret, out, label, context)
    }

    #[cfg(feature = "secret_extraction")]
    pub(crate) fn extract_secrets(&self, side: Side) -> Result<PartiallyExtractedSecrets, Error> {
        fn expand<const KEY_LEN: usize, const IV_LEN: usize>(
            secret: &hkdf::Prk,
        ) -> Result<([u8; KEY_LEN], [u8; IV_LEN]), Error> {
            let mut key = [0u8; KEY_LEN];
            let mut iv = [0u8; IV_LEN];

            hkdf_expand_info(secret, PayloadU8Len(key.len()), b"key", &[], |okm| {
                okm.fill(&mut key)
            })
            .map_err(|_| Error::General("hkdf_expand_info failed".to_string()))?;

            hkdf_expand_info(secret, PayloadU8Len(iv.len()), b"iv", &[], |okm| {
                okm.fill(&mut iv)
            })
            .map_err(|_| Error::General("hkdf_expand_info failed".to_string()))?;

            Ok((key, iv))
        }

        let client_secrets;
        let server_secrets;

        let algo = self.ks.suite.common.aead_algorithm;
        if algo == &aead::AES_128_GCM {
            let extract = |secret: &hkdf::Prk| -> Result<ConnectionTrafficSecrets, Error> {
                let (key, iv_in) = expand::<16, 12>(secret)?;

                let mut salt = [0u8; 4];
                salt.copy_from_slice(&iv_in[..4]);

                let mut iv = [0u8; 8];
                iv.copy_from_slice(&iv_in[4..]);

                Ok(ConnectionTrafficSecrets::Aes128Gcm { key, salt, iv })
            };

            client_secrets = extract(&self.current_client_traffic_secret)?;
            server_secrets = extract(&self.current_server_traffic_secret)?;
        } else if algo == &aead::AES_256_GCM {
            let extract = |secret: &hkdf::Prk| -> Result<ConnectionTrafficSecrets, Error> {
                let (key, iv_in) = expand::<32, 12>(secret)?;

                let mut salt = [0u8; 4];
                salt.copy_from_slice(&iv_in[..4]);

                let mut iv = [0u8; 8];
                iv.copy_from_slice(&iv_in[4..]);

                Ok(ConnectionTrafficSecrets::Aes256Gcm { key, salt, iv })
            };

            client_secrets = extract(&self.current_client_traffic_secret)?;
            server_secrets = extract(&self.current_server_traffic_secret)?;
        } else if algo == &aead::CHACHA20_POLY1305 {
            let extract = |secret: &hkdf::Prk| -> Result<ConnectionTrafficSecrets, Error> {
                let (key, iv) = expand::<32, 12>(secret)?;
                Ok(ConnectionTrafficSecrets::Chacha20Poly1305 { key, iv })
            };

            client_secrets = extract(&self.current_client_traffic_secret)?;
            server_secrets = extract(&self.current_server_traffic_secret)?;
        } else {
            return Err(Error::General(format!(
                "exporting secrets for {:?}: unimplemented",
                algo
            )));
        }

        let (tx, rx) = match side {
            Side::Client => (client_secrets, server_secrets),
            Side::Server => (server_secrets, client_secrets),
        };
        Ok(PartiallyExtractedSecrets { tx, rx })
    }
}

impl KeySchedule {
    fn new(suite: &'static Tls13CipherSuite, secret: &[u8]) -> Self {
        let zeroes = [0u8; digest::MAX_OUTPUT_LEN];
        let salt = hkdf::Salt::new(suite.hkdf_algorithm, &zeroes[..suite.hkdf_algorithm.len()]);
        Self {
            current: salt.extract(secret),
            suite,
        }
    }

    fn set_encrypter(&self, secret: &hkdf::Prk, common: &mut CommonState) {
        let key = derive_traffic_key(secret, self.suite.common.aead_algorithm);
        let iv = derive_traffic_iv(secret);

        common
            .record_layer
            .set_message_encrypter(Box::new(Tls13MessageEncrypter {
                enc_key: aead::LessSafeKey::new(key),
                iv,
            }));
    }

    fn set_decrypter(&self, secret: &hkdf::Prk, common: &mut CommonState) {
        common
            .record_layer
            .set_message_decrypter(self.derive_decrypter(secret));
    }

    fn derive_decrypter(&self, secret: &hkdf::Prk) -> Box<dyn MessageDecrypter> {
        let key = derive_traffic_key(secret, self.suite.common.aead_algorithm);
        let iv = derive_traffic_iv(secret);
        Box::new(Tls13MessageDecrypter {
            dec_key: aead::LessSafeKey::new(key),
            iv,
        })
    }

    #[inline]
    fn algorithm(&self) -> hkdf::Algorithm {
        self.suite.hkdf_algorithm
    }

    fn new_with_empty_secret(suite: &'static Tls13CipherSuite) -> Self {
        let zeroes = [0u8; digest::MAX_OUTPUT_LEN];
        Self::new(suite, &zeroes[..suite.hkdf_algorithm.len()])
    }

    /// Input the empty secret.
    fn input_empty(&mut self) {
        let zeroes = [0u8; digest::MAX_OUTPUT_LEN];
        self.input_secret(&zeroes[..self.suite.hkdf_algorithm.len()]);
    }

    /// Input the given secret.
    fn input_secret(&mut self, secret: &[u8]) {
        let salt: hkdf::Salt = self.derive_for_empty_hash(SecretKind::DerivedSecret);
        self.current = salt.extract(secret);
    }

    /// Derive a secret of given `kind`, using current handshake hash `hs_hash`.
    fn derive<T, L>(&self, key_type: L, kind: SecretKind, hs_hash: &[u8]) -> T
    where
        T: for<'a> From<hkdf::Okm<'a, L>>,
        L: hkdf::KeyType,
    {
        hkdf_expand(&self.current, key_type, kind.to_bytes(), hs_hash)
    }

    fn derive_logged_secret(
        &self,
        kind: SecretKind,
        hs_hash: &[u8],
        key_log: &dyn KeyLog,
        client_random: &[u8; 32],
    ) -> hkdf::Prk {
        let log_label = kind
            .log_label()
            .expect("not a loggable secret");
        if key_log.will_log(log_label) {
            let secret = self
                .derive::<PayloadU8, _>(
                    PayloadU8Len(self.suite.hkdf_algorithm.len()),
                    kind,
                    hs_hash,
                )
                .into_inner();
            key_log.log(log_label, client_random, &secret);
        }
        self.derive(self.suite.hkdf_algorithm, kind, hs_hash)
    }

    /// Derive a secret of given `kind` using the hash of the empty string
    /// for the handshake hash.  Useful only for
    /// `SecretKind::ResumptionPSKBinderKey` and
    /// `SecretKind::DerivedSecret`.
    fn derive_for_empty_hash<T>(&self, kind: SecretKind) -> T
    where
        T: for<'a> From<hkdf::Okm<'a, hkdf::Algorithm>>,
    {
        let digest_alg = self
            .suite
            .hkdf_algorithm
            .hmac_algorithm()
            .digest_algorithm();
        let empty_hash = digest::digest(digest_alg, &[]);
        self.derive(self.suite.hkdf_algorithm, kind, empty_hash.as_ref())
    }

    /// Sign the finished message consisting of `hs_hash` using a current
    /// traffic secret.
    fn sign_finish(&self, base_key: &hkdf::Prk, hs_hash: &Digest) -> hmac::Tag {
        self.sign_verify_data(base_key, hs_hash)
    }

    /// Sign the finished message consisting of `hs_hash` using the key material
    /// `base_key`.
    fn sign_verify_data(&self, base_key: &hkdf::Prk, hs_hash: &Digest) -> hmac::Tag {
        let hmac_alg = self
            .suite
            .hkdf_algorithm
            .hmac_algorithm();
        let hmac_key = hkdf_expand(base_key, hmac_alg, b"finished", &[]);
        hmac::sign(&hmac_key, hs_hash.as_ref())
    }

    /// Derive the next application traffic secret, returning it.
    fn derive_next(&self, base_key: &hkdf::Prk) -> hkdf::Prk {
        hkdf_expand(base_key, self.suite.hkdf_algorithm, b"traffic upd", &[])
    }

    /// Derive the PSK to use given a resumption_master_secret and
    /// ticket_nonce.
    fn derive_ticket_psk(&self, rms: &hkdf::Prk, nonce: &[u8]) -> Vec<u8> {
        let payload: PayloadU8 = hkdf_expand(
            rms,
            PayloadU8Len(self.suite.hkdf_algorithm.len()),
            b"resumption",
            nonce,
        );
        payload.into_inner()
    }

    fn export_keying_material(
        &self,
        current_exporter_secret: &hkdf::Prk,
        out: &mut [u8],
        label: &[u8],
        context: Option<&[u8]>,
    ) -> Result<(), Error> {
        let digest_alg = self
            .suite
            .hkdf_algorithm
            .hmac_algorithm()
            .digest_algorithm();

        let h_empty = digest::digest(digest_alg, &[]);
        let secret: hkdf::Prk = hkdf_expand(
            current_exporter_secret,
            self.suite.hkdf_algorithm,
            label,
            h_empty.as_ref(),
        );

        let h_context = digest::digest(digest_alg, context.unwrap_or(&[]));

        // TODO: Test what happens when this fails
        hkdf_expand_info(
            &secret,
            PayloadU8Len(out.len()),
            b"exporter",
            h_context.as_ref(),
            |okm| okm.fill(out),
        )
        .map_err(|_| Error::General("exporting too much".to_string()))
    }
}

pub(crate) fn hkdf_expand<T, L>(secret: &hkdf::Prk, key_type: L, label: &[u8], context: &[u8]) -> T
where
    T: for<'a> From<hkdf::Okm<'a, L>>,
    L: hkdf::KeyType,
{
    hkdf_expand_info(secret, key_type, label, context, |okm| okm.into())
}

fn hkdf_expand_info<F, T, L>(
    secret: &hkdf::Prk,
    key_type: L,
    label: &[u8],
    context: &[u8],
    f: F,
) -> T
where
    F: for<'b> FnOnce(hkdf::Okm<'b, L>) -> T,
    L: hkdf::KeyType,
{
    const LABEL_PREFIX: &[u8] = b"tls13 ";

    let output_len = u16::to_be_bytes(key_type.len() as u16);
    let label_len = u8::to_be_bytes((LABEL_PREFIX.len() + label.len()) as u8);
    let context_len = u8::to_be_bytes(context.len() as u8);

    let info = &[
        &output_len[..],
        &label_len[..],
        LABEL_PREFIX,
        label,
        &context_len[..],
        context,
    ];
    let okm = secret.expand(info, key_type).unwrap();

    f(okm)
}

pub(crate) struct PayloadU8Len(pub(crate) usize);
impl hkdf::KeyType for PayloadU8Len {
    fn len(&self) -> usize {
        self.0
    }
}

impl From<hkdf::Okm<'_, PayloadU8Len>> for PayloadU8 {
    fn from(okm: hkdf::Okm<PayloadU8Len>) -> Self {
        let mut r = vec![0u8; okm.len().0];
        okm.fill(&mut r[..]).unwrap();
        Self::new(r)
    }
}

pub(crate) fn derive_traffic_key(
    secret: &hkdf::Prk,
    aead_algorithm: &'static aead::Algorithm,
) -> aead::UnboundKey {
    hkdf_expand(secret, aead_algorithm, b"key", &[])
}

pub(crate) fn derive_traffic_iv(secret: &hkdf::Prk) -> Iv {
    hkdf_expand(secret, IvLen, b"iv", &[])
}

#[cfg(test)]
mod test {
    use super::{derive_traffic_iv, derive_traffic_key, KeySchedule, SecretKind};
    use crate::tls13::TLS13_CHACHA20_POLY1305_SHA256_INTERNAL;
    use crate::KeyLog;
    use ring::aead;

    #[test]
    fn test_vectors() {
        /* These test vectors generated with OpenSSL. */
        let hs_start_hash = [
            0xec, 0x14, 0x7a, 0x06, 0xde, 0xa3, 0xc8, 0x84, 0x6c, 0x02, 0xb2, 0x23, 0x8e, 0x41,
            0xbd, 0xdc, 0x9d, 0x89, 0xf9, 0xae, 0xa1, 0x7b, 0x5e, 0xfd, 0x4d, 0x74, 0x82, 0xaf,
            0x75, 0x88, 0x1c, 0x0a,
        ];

        let hs_full_hash = [
            0x75, 0x1a, 0x3d, 0x4a, 0x14, 0xdf, 0xab, 0xeb, 0x68, 0xe9, 0x2c, 0xa5, 0x91, 0x8e,
            0x24, 0x08, 0xb9, 0xbc, 0xb0, 0x74, 0x89, 0x82, 0xec, 0x9c, 0x32, 0x30, 0xac, 0x30,
            0xbb, 0xeb, 0x23, 0xe2,
        ];

        let ecdhe_secret = [
            0xe7, 0xb8, 0xfe, 0xf8, 0x90, 0x3b, 0x52, 0x0c, 0xb9, 0xa1, 0x89, 0x71, 0xb6, 0x9d,
            0xd4, 0x5d, 0xca, 0x53, 0xce, 0x2f, 0x12, 0xbf, 0x3b, 0xef, 0x93, 0x15, 0xe3, 0x12,
            0x71, 0xdf, 0x4b, 0x40,
        ];

        let client_hts = [
            0x61, 0x7b, 0x35, 0x07, 0x6b, 0x9d, 0x0e, 0x08, 0xcf, 0x73, 0x1d, 0x94, 0xa8, 0x66,
            0x14, 0x78, 0x41, 0x09, 0xef, 0x25, 0x55, 0x51, 0x92, 0x1d, 0xd4, 0x6e, 0x04, 0x01,
            0x35, 0xcf, 0x46, 0xab,
        ];

        let client_hts_key = [
            0x62, 0xd0, 0xdd, 0x00, 0xf6, 0x96, 0x19, 0xd3, 0xb8, 0x19, 0x3a, 0xb4, 0xa0, 0x95,
            0x85, 0xa7,
        ];

        let client_hts_iv = [
            0xff, 0xf7, 0x5d, 0xf5, 0xad, 0x35, 0xd5, 0xcb, 0x3c, 0x53, 0xf3, 0xa9,
        ];

        let server_hts = [
            0xfc, 0xf7, 0xdf, 0xe6, 0x4f, 0xa2, 0xc0, 0x4f, 0x62, 0x35, 0x38, 0x7f, 0x43, 0x4e,
            0x01, 0x42, 0x23, 0x36, 0xd9, 0xc0, 0x39, 0xde, 0x68, 0x47, 0xa0, 0xb9, 0xdd, 0xcf,
            0x29, 0xa8, 0x87, 0x59,
        ];

        let server_hts_key = [
            0x04, 0x67, 0xf3, 0x16, 0xa8, 0x05, 0xb8, 0xc4, 0x97, 0xee, 0x67, 0x04, 0x7b, 0xbc,
            0xbc, 0x54,
        ];

        let server_hts_iv = [
            0xde, 0x83, 0xa7, 0x3e, 0x9d, 0x81, 0x4b, 0x04, 0xc4, 0x8b, 0x78, 0x09,
        ];

        let client_ats = [
            0xc1, 0x4a, 0x6d, 0x79, 0x76, 0xd8, 0x10, 0x2b, 0x5a, 0x0c, 0x99, 0x51, 0x49, 0x3f,
            0xee, 0x87, 0xdc, 0xaf, 0xf8, 0x2c, 0x24, 0xca, 0xb2, 0x14, 0xe8, 0xbe, 0x71, 0xa8,
            0x20, 0x6d, 0xbd, 0xa5,
        ];

        let client_ats_key = [
            0xcc, 0x9f, 0x5f, 0x98, 0x0b, 0x5f, 0x10, 0x30, 0x6c, 0xba, 0xd7, 0xbe, 0x98, 0xd7,
            0x57, 0x2e,
        ];

        let client_ats_iv = [
            0xb8, 0x09, 0x29, 0xe8, 0xd0, 0x2c, 0x70, 0xf6, 0x11, 0x62, 0xed, 0x6b,
        ];

        let server_ats = [
            0x2c, 0x90, 0x77, 0x38, 0xd3, 0xf8, 0x37, 0x02, 0xd1, 0xe4, 0x59, 0x8f, 0x48, 0x48,
            0x53, 0x1d, 0x9f, 0x93, 0x65, 0x49, 0x1b, 0x9f, 0x7f, 0x52, 0xc8, 0x22, 0x29, 0x0d,
            0x4c, 0x23, 0x21, 0x92,
        ];

        let server_ats_key = [
            0x0c, 0xb2, 0x95, 0x62, 0xd8, 0xd8, 0x8f, 0x48, 0xb0, 0x2c, 0xbf, 0xbe, 0xd7, 0xe6,
            0x2b, 0xb3,
        ];

        let server_ats_iv = [
            0x0d, 0xb2, 0x8f, 0x98, 0x85, 0x86, 0xa1, 0xb7, 0xe4, 0xd5, 0xc6, 0x9c,
        ];

        let mut ks = KeySchedule::new_with_empty_secret(TLS13_CHACHA20_POLY1305_SHA256_INTERNAL);
        ks.input_secret(&ecdhe_secret);

        assert_traffic_secret(
            &ks,
            SecretKind::ClientHandshakeTrafficSecret,
            &hs_start_hash,
            &client_hts,
            &client_hts_key,
            &client_hts_iv,
        );

        assert_traffic_secret(
            &ks,
            SecretKind::ServerHandshakeTrafficSecret,
            &hs_start_hash,
            &server_hts,
            &server_hts_key,
            &server_hts_iv,
        );

        ks.input_empty();

        assert_traffic_secret(
            &ks,
            SecretKind::ClientApplicationTrafficSecret,
            &hs_full_hash,
            &client_ats,
            &client_ats_key,
            &client_ats_iv,
        );

        assert_traffic_secret(
            &ks,
            SecretKind::ServerApplicationTrafficSecret,
            &hs_full_hash,
            &server_ats,
            &server_ats_key,
            &server_ats_iv,
        );
    }

    fn assert_traffic_secret(
        ks: &KeySchedule,
        kind: SecretKind,
        hash: &[u8],
        expected_traffic_secret: &[u8],
        expected_key: &[u8],
        expected_iv: &[u8],
    ) {
        struct Log<'a>(&'a [u8]);
        impl KeyLog for Log<'_> {
            fn log(&self, _label: &str, _client_random: &[u8], secret: &[u8]) {
                assert_eq!(self.0, secret);
            }
        }
        let log = Log(expected_traffic_secret);
        let traffic_secret = ks.derive_logged_secret(kind, hash, &log, &[0; 32]);

        // Since we can't test key equality, we test the output of sealing with the key instead.
        let aead_alg = &aead::AES_128_GCM;
        let key = derive_traffic_key(&traffic_secret, aead_alg);
        let seal_output = seal_zeroes(key);
        let expected_key = aead::UnboundKey::new(aead_alg, expected_key).unwrap();
        let expected_seal_output = seal_zeroes(expected_key);
        assert_eq!(seal_output, expected_seal_output);
        assert!(seal_output.len() >= 48); // Sanity check.

        let iv = derive_traffic_iv(&traffic_secret);
        assert_eq!(iv.value(), expected_iv);
    }

    fn seal_zeroes(key: aead::UnboundKey) -> Vec<u8> {
        let key = aead::LessSafeKey::new(key);
        let mut seal_output = vec![0; 32];
        key.seal_in_place_append_tag(
            aead::Nonce::assume_unique_for_key([0; aead::NONCE_LEN]),
            aead::Aad::empty(),
            &mut seal_output,
        )
        .unwrap();
        seal_output
    }
}

#[cfg(bench)]
mod benchmarks {
    #[bench]
    fn bench_sha256(b: &mut test::Bencher) {
        use super::{derive_traffic_iv, derive_traffic_key, KeySchedule, SecretKind};
        use crate::tls13::TLS13_CHACHA20_POLY1305_SHA256_INTERNAL;
        use crate::KeyLog;
        use ring::aead;

        fn extract_traffic_secret(ks: &KeySchedule, kind: SecretKind) {
            struct Log;

            impl KeyLog for Log {
                fn log(&self, _label: &str, _client_random: &[u8], _secret: &[u8]) {}
            }

            let aead_alg = &aead::CHACHA20_POLY1305;
            let hash = [0u8; 32];
            let traffic_secret = ks.derive_logged_secret(kind, &hash, &Log, &[0u8; 32]);
            test::black_box(derive_traffic_key(&traffic_secret, aead_alg));
            test::black_box(derive_traffic_iv(&traffic_secret));
        }

        b.iter(|| {
            let mut ks =
                KeySchedule::new_with_empty_secret(TLS13_CHACHA20_POLY1305_SHA256_INTERNAL);
            ks.input_secret(&[0u8; 32]);

            extract_traffic_secret(&ks, SecretKind::ClientHandshakeTrafficSecret);
            extract_traffic_secret(&ks, SecretKind::ServerHandshakeTrafficSecret);

            ks.input_empty();

            extract_traffic_secret(&ks, SecretKind::ClientApplicationTrafficSecret);
            extract_traffic_secret(&ks, SecretKind::ServerApplicationTrafficSecret);
        });
    }
}
