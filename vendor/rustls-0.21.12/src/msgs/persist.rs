use crate::dns_name::DnsName;
use crate::enums::{CipherSuite, ProtocolVersion};
use crate::error::InvalidMessage;
use crate::key;
use crate::msgs::base::{PayloadU16, PayloadU8};
use crate::msgs::codec::{Codec, Reader};
use crate::msgs::handshake::CertificatePayload;
use crate::msgs::handshake::SessionId;
use crate::ticketer::TimeBase;
#[cfg(feature = "tls12")]
use crate::tls12::Tls12CipherSuite;
use crate::tls13::Tls13CipherSuite;

use std::cmp;
#[cfg(feature = "tls12")]
use std::mem;

pub struct Retrieved<T> {
    pub value: T,
    retrieved_at: TimeBase,
}

impl<T> Retrieved<T> {
    pub fn new(value: T, retrieved_at: TimeBase) -> Self {
        Self {
            value,
            retrieved_at,
        }
    }

    pub fn map<M>(&self, f: impl FnOnce(&T) -> Option<&M>) -> Option<Retrieved<&M>> {
        Some(Retrieved {
            value: f(&self.value)?,
            retrieved_at: self.retrieved_at,
        })
    }
}

impl Retrieved<&Tls13ClientSessionValue> {
    pub fn obfuscated_ticket_age(&self) -> u32 {
        let age_secs = self
            .retrieved_at
            .as_secs()
            .saturating_sub(self.value.common.epoch);
        let age_millis = age_secs as u32 * 1000;
        age_millis.wrapping_add(self.value.age_add)
    }
}

impl<T: std::ops::Deref<Target = ClientSessionCommon>> Retrieved<T> {
    pub fn has_expired(&self) -> bool {
        let common = &*self.value;
        common.lifetime_secs != 0
            && common
                .epoch
                .saturating_add(u64::from(common.lifetime_secs))
                < self.retrieved_at.as_secs()
    }
}

impl<T> std::ops::Deref for Retrieved<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug)]
pub struct Tls13ClientSessionValue {
    suite: &'static Tls13CipherSuite,
    age_add: u32,
    max_early_data_size: u32,
    pub(crate) common: ClientSessionCommon,
    #[cfg(feature = "quic")]
    quic_params: PayloadU16,
}

impl Tls13ClientSessionValue {
    pub(crate) fn new(
        suite: &'static Tls13CipherSuite,
        ticket: Vec<u8>,
        secret: Vec<u8>,
        server_cert_chain: Vec<key::Certificate>,
        time_now: TimeBase,
        lifetime_secs: u32,
        age_add: u32,
        max_early_data_size: u32,
    ) -> Self {
        Self {
            suite,
            age_add,
            max_early_data_size,
            common: ClientSessionCommon::new(
                ticket,
                secret,
                time_now,
                lifetime_secs,
                server_cert_chain,
            ),
            #[cfg(feature = "quic")]
            quic_params: PayloadU16(Vec::new()),
        }
    }

    pub fn max_early_data_size(&self) -> u32 {
        self.max_early_data_size
    }

    pub fn suite(&self) -> &'static Tls13CipherSuite {
        self.suite
    }

    #[doc(hidden)]
    /// Test only: rewind epoch by `delta` seconds.
    pub fn rewind_epoch(&mut self, delta: u32) {
        self.common.epoch -= delta as u64;
    }

    #[cfg(feature = "quic")]
    pub fn set_quic_params(&mut self, quic_params: &[u8]) {
        self.quic_params = PayloadU16(quic_params.to_vec());
    }

    #[cfg(feature = "quic")]
    pub fn quic_params(&self) -> Vec<u8> {
        self.quic_params.0.clone()
    }
}

impl std::ops::Deref for Tls13ClientSessionValue {
    type Target = ClientSessionCommon;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

#[derive(Debug, Clone)]
pub struct Tls12ClientSessionValue {
    #[cfg(feature = "tls12")]
    suite: &'static Tls12CipherSuite,
    #[cfg(feature = "tls12")]
    pub(crate) session_id: SessionId,
    #[cfg(feature = "tls12")]
    extended_ms: bool,
    #[doc(hidden)]
    #[cfg(feature = "tls12")]
    pub(crate) common: ClientSessionCommon,
}

#[cfg(feature = "tls12")]
impl Tls12ClientSessionValue {
    pub(crate) fn new(
        suite: &'static Tls12CipherSuite,
        session_id: SessionId,
        ticket: Vec<u8>,
        master_secret: Vec<u8>,
        server_cert_chain: Vec<key::Certificate>,
        time_now: TimeBase,
        lifetime_secs: u32,
        extended_ms: bool,
    ) -> Self {
        Self {
            suite,
            session_id,
            extended_ms,
            common: ClientSessionCommon::new(
                ticket,
                master_secret,
                time_now,
                lifetime_secs,
                server_cert_chain,
            ),
        }
    }

    pub(crate) fn take_ticket(&mut self) -> Vec<u8> {
        mem::take(&mut self.common.ticket.0)
    }

    pub(crate) fn extended_ms(&self) -> bool {
        self.extended_ms
    }

    pub(crate) fn suite(&self) -> &'static Tls12CipherSuite {
        self.suite
    }

    #[doc(hidden)]
    /// Test only: rewind epoch by `delta` seconds.
    pub fn rewind_epoch(&mut self, delta: u32) {
        self.common.epoch -= delta as u64;
    }
}

#[cfg(feature = "tls12")]
impl std::ops::Deref for Tls12ClientSessionValue {
    type Target = ClientSessionCommon;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

#[derive(Debug, Clone)]
pub struct ClientSessionCommon {
    ticket: PayloadU16,
    secret: PayloadU8,
    epoch: u64,
    lifetime_secs: u32,
    server_cert_chain: CertificatePayload,
}

impl ClientSessionCommon {
    fn new(
        ticket: Vec<u8>,
        secret: Vec<u8>,
        time_now: TimeBase,
        lifetime_secs: u32,
        server_cert_chain: Vec<key::Certificate>,
    ) -> Self {
        Self {
            ticket: PayloadU16(ticket),
            secret: PayloadU8(secret),
            epoch: time_now.as_secs(),
            lifetime_secs: cmp::min(lifetime_secs, MAX_TICKET_LIFETIME),
            server_cert_chain,
        }
    }

    pub(crate) fn server_cert_chain(&self) -> &[key::Certificate] {
        self.server_cert_chain.as_ref()
    }

    pub(crate) fn secret(&self) -> &[u8] {
        self.secret.0.as_ref()
    }

    pub(crate) fn ticket(&self) -> &[u8] {
        self.ticket.0.as_ref()
    }
}

static MAX_TICKET_LIFETIME: u32 = 7 * 24 * 60 * 60;

/// This is the maximum allowed skew between server and client clocks, over
/// the maximum ticket lifetime period.  This encompasses TCP retransmission
/// times in case packet loss occurs when the client sends the ClientHello
/// or receives the NewSessionTicket, _and_ actual clock skew over this period.
static MAX_FRESHNESS_SKEW_MS: u32 = 60 * 1000;

// --- Server types ---
pub type ServerSessionKey = SessionId;

#[derive(Debug)]
pub struct ServerSessionValue {
    pub sni: Option<DnsName>,
    pub version: ProtocolVersion,
    pub cipher_suite: CipherSuite,
    pub master_secret: PayloadU8,
    pub extended_ms: bool,
    pub client_cert_chain: Option<CertificatePayload>,
    pub alpn: Option<PayloadU8>,
    pub application_data: PayloadU16,
    pub creation_time_sec: u64,
    pub age_obfuscation_offset: u32,
    freshness: Option<bool>,
}

impl Codec for ServerSessionValue {
    fn encode(&self, bytes: &mut Vec<u8>) {
        if let Some(ref sni) = self.sni {
            1u8.encode(bytes);
            let sni_bytes: &str = sni.as_ref();
            PayloadU8::new(Vec::from(sni_bytes)).encode(bytes);
        } else {
            0u8.encode(bytes);
        }
        self.version.encode(bytes);
        self.cipher_suite.encode(bytes);
        self.master_secret.encode(bytes);
        (u8::from(self.extended_ms)).encode(bytes);
        if let Some(ref chain) = self.client_cert_chain {
            1u8.encode(bytes);
            chain.encode(bytes);
        } else {
            0u8.encode(bytes);
        }
        if let Some(ref alpn) = self.alpn {
            1u8.encode(bytes);
            alpn.encode(bytes);
        } else {
            0u8.encode(bytes);
        }
        self.application_data.encode(bytes);
        self.creation_time_sec.encode(bytes);
        self.age_obfuscation_offset
            .encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let has_sni = u8::read(r)?;
        let sni = if has_sni == 1 {
            let dns_name = PayloadU8::read(r)?;
            let dns_name = match DnsName::try_from_ascii(&dns_name.0) {
                Ok(dns_name) => dns_name,
                Err(_) => return Err(InvalidMessage::InvalidServerName),
            };

            Some(dns_name)
        } else {
            None
        };

        let v = ProtocolVersion::read(r)?;
        let cs = CipherSuite::read(r)?;
        let ms = PayloadU8::read(r)?;
        let ems = u8::read(r)?;
        let has_ccert = u8::read(r)? == 1;
        let ccert = if has_ccert {
            Some(CertificatePayload::read(r)?)
        } else {
            None
        };
        let has_alpn = u8::read(r)? == 1;
        let alpn = if has_alpn {
            Some(PayloadU8::read(r)?)
        } else {
            None
        };
        let application_data = PayloadU16::read(r)?;
        let creation_time_sec = u64::read(r)?;
        let age_obfuscation_offset = u32::read(r)?;

        Ok(Self {
            sni,
            version: v,
            cipher_suite: cs,
            master_secret: ms,
            extended_ms: ems == 1u8,
            client_cert_chain: ccert,
            alpn,
            application_data,
            creation_time_sec,
            age_obfuscation_offset,
            freshness: None,
        })
    }
}

impl ServerSessionValue {
    pub fn new(
        sni: Option<&DnsName>,
        v: ProtocolVersion,
        cs: CipherSuite,
        ms: Vec<u8>,
        client_cert_chain: Option<CertificatePayload>,
        alpn: Option<Vec<u8>>,
        application_data: Vec<u8>,
        creation_time: TimeBase,
        age_obfuscation_offset: u32,
    ) -> Self {
        Self {
            sni: sni.cloned(),
            version: v,
            cipher_suite: cs,
            master_secret: PayloadU8::new(ms),
            extended_ms: false,
            client_cert_chain,
            alpn: alpn.map(PayloadU8::new),
            application_data: PayloadU16::new(application_data),
            creation_time_sec: creation_time.as_secs(),
            age_obfuscation_offset,
            freshness: None,
        }
    }

    pub fn set_extended_ms_used(&mut self) {
        self.extended_ms = true;
    }

    pub fn set_freshness(mut self, obfuscated_client_age_ms: u32, time_now: TimeBase) -> Self {
        let client_age_ms = obfuscated_client_age_ms.wrapping_sub(self.age_obfuscation_offset);
        let server_age_ms = (time_now
            .as_secs()
            .saturating_sub(self.creation_time_sec) as u32)
            .saturating_mul(1000);

        let age_difference = if client_age_ms < server_age_ms {
            server_age_ms - client_age_ms
        } else {
            client_age_ms - server_age_ms
        };

        self.freshness = Some(age_difference <= MAX_FRESHNESS_SKEW_MS);
        self
    }

    pub fn is_fresh(&self) -> bool {
        self.freshness.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::*;

    #[test]
    fn serversessionvalue_is_debug() {
        let ssv = ServerSessionValue::new(
            None,
            ProtocolVersion::TLSv1_3,
            CipherSuite::TLS13_AES_128_GCM_SHA256,
            vec![1, 2, 3],
            None,
            None,
            vec![4, 5, 6],
            TimeBase::now().unwrap(),
            0x12345678,
        );
        println!("{:?}", ssv);
    }

    #[test]
    fn serversessionvalue_no_sni() {
        let bytes = [
            0x00, 0x03, 0x03, 0xc0, 0x23, 0x03, 0x01, 0x02, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x12, 0x23, 0x34, 0x45, 0x56, 0x67, 0x78, 0x89, 0xfe, 0xed, 0xf0, 0x0d,
        ];
        let mut rd = Reader::init(&bytes);
        let ssv = ServerSessionValue::read(&mut rd).unwrap();
        assert_eq!(ssv.get_encoding(), bytes);
    }

    #[test]
    fn serversessionvalue_with_cert() {
        let bytes = [
            0x00, 0x03, 0x03, 0xc0, 0x23, 0x03, 0x01, 0x02, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x12, 0x23, 0x34, 0x45, 0x56, 0x67, 0x78, 0x89, 0xfe, 0xed, 0xf0, 0x0d,
        ];
        let mut rd = Reader::init(&bytes);
        let ssv = ServerSessionValue::read(&mut rd).unwrap();
        assert_eq!(ssv.get_encoding(), bytes);
    }
}
