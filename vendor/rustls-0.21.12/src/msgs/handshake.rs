#![allow(non_camel_case_types)]
use crate::dns_name::{DnsName, DnsNameRef};
use crate::enums::{CipherSuite, HandshakeType, ProtocolVersion, SignatureScheme};
use crate::error::InvalidMessage;
use crate::key;
#[cfg(feature = "logging")]
use crate::log::warn;
use crate::msgs::base::{Payload, PayloadU16, PayloadU24, PayloadU8};
use crate::msgs::codec::{self, Codec, ListLength, Reader, TlsListElement};
use crate::msgs::enums::{
    CertificateStatusType, ClientCertificateType, Compression, ECCurveType, ECPointFormat,
    ExtensionType, KeyUpdateRequest, NamedGroup, PSKKeyExchangeMode, ServerNameType,
};
use crate::rand;
use crate::verify::DigitallySignedStruct;

use std::collections;
use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

/// Create a newtype wrapper around a given type.
///
/// This is used to create newtypes for the various TLS message types which is used to wrap
/// the `PayloadU8` or `PayloadU16` types. This is typically used for types where we don't need
/// anything other than access to the underlying bytes.
macro_rules! wrapped_payload(
  ($(#[$comment:meta])* $name:ident, $inner:ident,) => {
    $(#[$comment])*
    #[derive(Clone, Debug)]
    pub struct $name($inner);

    impl From<Vec<u8>> for $name {
        fn from(v: Vec<u8>) -> Self {
            Self($inner::new(v))
        }
    }

    impl AsRef<[u8]> for $name {
        fn as_ref(&self) -> &[u8] {
            self.0.0.as_slice()
        }
    }

    impl Codec for $name {
        fn encode(&self, bytes: &mut Vec<u8>) {
            self.0.encode(bytes);
        }

        fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
            Ok(Self($inner::read(r)?))
        }
    }
  }
);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Random(pub [u8; 32]);

impl fmt::Debug for Random {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        super::base::hex(f, &self.0)
    }
}

static HELLO_RETRY_REQUEST_RANDOM: Random = Random([
    0xcf, 0x21, 0xad, 0x74, 0xe5, 0x9a, 0x61, 0x11, 0xbe, 0x1d, 0x8c, 0x02, 0x1e, 0x65, 0xb8, 0x91,
    0xc2, 0xa2, 0x11, 0x16, 0x7a, 0xbb, 0x8c, 0x5e, 0x07, 0x9e, 0x09, 0xe2, 0xc8, 0xa8, 0x33, 0x9c,
]);

static ZERO_RANDOM: Random = Random([0u8; 32]);

impl Codec for Random {
    fn encode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.0);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let bytes = match r.take(32) {
            Some(bytes) => bytes,
            None => return Err(InvalidMessage::MissingData("Random")),
        };

        let mut opaque = [0; 32];
        opaque.clone_from_slice(bytes);
        Ok(Self(opaque))
    }
}

impl Random {
    pub fn new() -> Result<Self, rand::GetRandomFailed> {
        let mut data = [0u8; 32];
        rand::fill_random(&mut data)?;
        Ok(Self(data))
    }

    pub fn write_slice(&self, bytes: &mut [u8]) {
        let buf = self.get_encoding();
        bytes.copy_from_slice(&buf);
    }
}

impl From<[u8; 32]> for Random {
    #[inline]
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

#[derive(Copy, Clone)]
pub struct SessionId {
    len: usize,
    data: [u8; 32],
}

impl fmt::Debug for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        super::base::hex(f, &self.data[..self.len])
    }
}

impl PartialEq for SessionId {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        let mut diff = 0u8;
        for i in 0..self.len {
            diff |= self.data[i] ^ other.data[i];
        }

        diff == 0u8
    }
}

impl Codec for SessionId {
    fn encode(&self, bytes: &mut Vec<u8>) {
        debug_assert!(self.len <= 32);
        bytes.push(self.len as u8);
        bytes.extend_from_slice(&self.data[..self.len]);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let len = u8::read(r)? as usize;
        if len > 32 {
            return Err(InvalidMessage::TrailingData("SessionID"));
        }

        let bytes = match r.take(len) {
            Some(bytes) => bytes,
            None => return Err(InvalidMessage::MissingData("SessionID")),
        };

        let mut out = [0u8; 32];
        out[..len].clone_from_slice(&bytes[..len]);
        Ok(Self { data: out, len })
    }
}

impl SessionId {
    pub fn random() -> Result<Self, rand::GetRandomFailed> {
        let mut data = [0u8; 32];
        rand::fill_random(&mut data)?;
        Ok(Self { data, len: 32 })
    }

    pub fn empty() -> Self {
        Self {
            data: [0u8; 32],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[derive(Clone, Debug)]
pub struct UnknownExtension {
    pub typ: ExtensionType,
    pub payload: Payload,
}

impl UnknownExtension {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.payload.encode(bytes);
    }

    fn read(typ: ExtensionType, r: &mut Reader) -> Self {
        let payload = Payload::read(r);
        Self { typ, payload }
    }
}

impl TlsListElement for ECPointFormat {
    const SIZE_LEN: ListLength = ListLength::U8;
}

impl TlsListElement for NamedGroup {
    const SIZE_LEN: ListLength = ListLength::U16;
}

impl TlsListElement for SignatureScheme {
    const SIZE_LEN: ListLength = ListLength::U16;
}

#[derive(Clone, Debug)]
pub enum ServerNamePayload {
    HostName(DnsName),
    IpAddress(PayloadU16),
    Unknown(Payload),
}

impl ServerNamePayload {
    pub fn new_hostname(hostname: DnsName) -> Self {
        Self::HostName(hostname)
    }

    fn read_hostname(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let raw = PayloadU16::read(r)?;
        match DnsName::try_from_ascii(&raw.0) {
            Ok(dns_name) => Ok(Self::HostName(dns_name)),
            Err(_) => {
                let _ = IpAddr::from_str(&String::from_utf8_lossy(&raw.0))
                    .map_err(|_| InvalidMessage::InvalidServerName)?;
                Ok(Self::IpAddress(raw))
            }
        }
    }

    fn encode(&self, bytes: &mut Vec<u8>) {
        match *self {
            Self::HostName(ref name) => {
                (name.as_ref().len() as u16).encode(bytes);
                bytes.extend_from_slice(name.as_ref().as_bytes());
            }
            Self::IpAddress(ref r) => r.encode(bytes),
            Self::Unknown(ref r) => r.encode(bytes),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServerName {
    pub typ: ServerNameType,
    pub payload: ServerNamePayload,
}

impl Codec for ServerName {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.typ.encode(bytes);
        self.payload.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let typ = ServerNameType::read(r)?;

        let payload = match typ {
            ServerNameType::HostName => ServerNamePayload::read_hostname(r)?,
            _ => ServerNamePayload::Unknown(Payload::read(r)),
        };

        Ok(Self { typ, payload })
    }
}

impl TlsListElement for ServerName {
    const SIZE_LEN: ListLength = ListLength::U16;
}

pub trait ConvertServerNameList {
    fn has_duplicate_names_for_type(&self) -> bool;
    fn get_single_hostname(&self) -> Option<DnsNameRef>;
}

impl ConvertServerNameList for [ServerName] {
    /// RFC6066: "The ServerNameList MUST NOT contain more than one name of the same name_type."
    fn has_duplicate_names_for_type(&self) -> bool {
        let mut seen = collections::HashSet::new();

        for name in self {
            if !seen.insert(name.typ.get_u8()) {
                return true;
            }
        }

        false
    }

    fn get_single_hostname(&self) -> Option<DnsNameRef> {
        fn only_dns_hostnames(name: &ServerName) -> Option<DnsNameRef> {
            if let ServerNamePayload::HostName(ref dns) = name.payload {
                Some(dns.borrow())
            } else {
                None
            }
        }

        self.iter()
            .filter_map(only_dns_hostnames)
            .next()
    }
}

wrapped_payload!(ProtocolName, PayloadU8,);

impl TlsListElement for ProtocolName {
    const SIZE_LEN: ListLength = ListLength::U16;
}

pub trait ConvertProtocolNameList {
    fn from_slices(names: &[&[u8]]) -> Self;
    fn to_slices(&self) -> Vec<&[u8]>;
    fn as_single_slice(&self) -> Option<&[u8]>;
}

impl ConvertProtocolNameList for Vec<ProtocolName> {
    fn from_slices(names: &[&[u8]]) -> Self {
        let mut ret = Self::new();

        for name in names {
            ret.push(ProtocolName::from(name.to_vec()));
        }

        ret
    }

    fn to_slices(&self) -> Vec<&[u8]> {
        self.iter()
            .map(|proto| proto.as_ref())
            .collect::<Vec<&[u8]>>()
    }

    fn as_single_slice(&self) -> Option<&[u8]> {
        if self.len() == 1 {
            Some(self[0].as_ref())
        } else {
            None
        }
    }
}

// --- TLS 1.3 Key shares ---
#[derive(Clone, Debug)]
pub struct KeyShareEntry {
    pub group: NamedGroup,
    pub payload: PayloadU16,
}

impl KeyShareEntry {
    pub fn new(group: NamedGroup, payload: &[u8]) -> Self {
        Self {
            group,
            payload: PayloadU16::new(payload.to_vec()),
        }
    }
}

impl Codec for KeyShareEntry {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.group.encode(bytes);
        self.payload.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let group = NamedGroup::read(r)?;
        let payload = PayloadU16::read(r)?;

        Ok(Self { group, payload })
    }
}

// --- TLS 1.3 PresharedKey offers ---
#[derive(Clone, Debug)]
pub struct PresharedKeyIdentity {
    pub identity: PayloadU16,
    pub obfuscated_ticket_age: u32,
}

impl PresharedKeyIdentity {
    pub fn new(id: Vec<u8>, age: u32) -> Self {
        Self {
            identity: PayloadU16::new(id),
            obfuscated_ticket_age: age,
        }
    }
}

impl Codec for PresharedKeyIdentity {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.identity.encode(bytes);
        self.obfuscated_ticket_age.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        Ok(Self {
            identity: PayloadU16::read(r)?,
            obfuscated_ticket_age: u32::read(r)?,
        })
    }
}

impl TlsListElement for PresharedKeyIdentity {
    const SIZE_LEN: ListLength = ListLength::U16;
}

wrapped_payload!(PresharedKeyBinder, PayloadU8,);

impl TlsListElement for PresharedKeyBinder {
    const SIZE_LEN: ListLength = ListLength::U16;
}

#[derive(Clone, Debug)]
pub struct PresharedKeyOffer {
    pub identities: Vec<PresharedKeyIdentity>,
    pub binders: Vec<PresharedKeyBinder>,
}

impl PresharedKeyOffer {
    /// Make a new one with one entry.
    pub fn new(id: PresharedKeyIdentity, binder: Vec<u8>) -> Self {
        Self {
            identities: vec![id],
            binders: vec![PresharedKeyBinder::from(binder)],
        }
    }
}

impl Codec for PresharedKeyOffer {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.identities.encode(bytes);
        self.binders.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        Ok(Self {
            identities: Vec::read(r)?,
            binders: Vec::read(r)?,
        })
    }
}

// --- RFC6066 certificate status request ---
wrapped_payload!(ResponderId, PayloadU16,);

impl TlsListElement for ResponderId {
    const SIZE_LEN: ListLength = ListLength::U16;
}

#[derive(Clone, Debug)]
pub struct OCSPCertificateStatusRequest {
    pub responder_ids: Vec<ResponderId>,
    pub extensions: PayloadU16,
}

impl Codec for OCSPCertificateStatusRequest {
    fn encode(&self, bytes: &mut Vec<u8>) {
        CertificateStatusType::OCSP.encode(bytes);
        self.responder_ids.encode(bytes);
        self.extensions.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        Ok(Self {
            responder_ids: Vec::read(r)?,
            extensions: PayloadU16::read(r)?,
        })
    }
}

#[derive(Clone, Debug)]
pub enum CertificateStatusRequest {
    OCSP(OCSPCertificateStatusRequest),
    Unknown((CertificateStatusType, Payload)),
}

impl Codec for CertificateStatusRequest {
    fn encode(&self, bytes: &mut Vec<u8>) {
        match self {
            Self::OCSP(ref r) => r.encode(bytes),
            Self::Unknown((typ, payload)) => {
                typ.encode(bytes);
                payload.encode(bytes);
            }
        }
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let typ = CertificateStatusType::read(r)?;

        match typ {
            CertificateStatusType::OCSP => {
                let ocsp_req = OCSPCertificateStatusRequest::read(r)?;
                Ok(Self::OCSP(ocsp_req))
            }
            _ => {
                let data = Payload::read(r);
                Ok(Self::Unknown((typ, data)))
            }
        }
    }
}

impl CertificateStatusRequest {
    pub fn build_ocsp() -> Self {
        let ocsp = OCSPCertificateStatusRequest {
            responder_ids: Vec::new(),
            extensions: PayloadU16::empty(),
        };
        Self::OCSP(ocsp)
    }
}

// ---
// SCTs

wrapped_payload!(Sct, PayloadU16,);

impl TlsListElement for Sct {
    const SIZE_LEN: ListLength = ListLength::U16;
}

// ---

impl TlsListElement for PSKKeyExchangeMode {
    const SIZE_LEN: ListLength = ListLength::U8;
}

impl TlsListElement for KeyShareEntry {
    const SIZE_LEN: ListLength = ListLength::U16;
}

impl TlsListElement for ProtocolVersion {
    const SIZE_LEN: ListLength = ListLength::U8;
}

#[derive(Clone, Debug)]
pub enum ClientExtension {
    ECPointFormats(Vec<ECPointFormat>),
    NamedGroups(Vec<NamedGroup>),
    SignatureAlgorithms(Vec<SignatureScheme>),
    ServerName(Vec<ServerName>),
    SessionTicket(ClientSessionTicket),
    Protocols(Vec<ProtocolName>),
    SupportedVersions(Vec<ProtocolVersion>),
    KeyShare(Vec<KeyShareEntry>),
    PresharedKeyModes(Vec<PSKKeyExchangeMode>),
    PresharedKey(PresharedKeyOffer),
    Cookie(PayloadU16),
    ExtendedMasterSecretRequest,
    CertificateStatusRequest(CertificateStatusRequest),
    SignedCertificateTimestampRequest,
    TransportParameters(Vec<u8>),
    TransportParametersDraft(Vec<u8>),
    EarlyData,
    Unknown(UnknownExtension),
}

impl ClientExtension {
    pub fn get_type(&self) -> ExtensionType {
        match *self {
            Self::ECPointFormats(_) => ExtensionType::ECPointFormats,
            Self::NamedGroups(_) => ExtensionType::EllipticCurves,
            Self::SignatureAlgorithms(_) => ExtensionType::SignatureAlgorithms,
            Self::ServerName(_) => ExtensionType::ServerName,
            Self::SessionTicket(_) => ExtensionType::SessionTicket,
            Self::Protocols(_) => ExtensionType::ALProtocolNegotiation,
            Self::SupportedVersions(_) => ExtensionType::SupportedVersions,
            Self::KeyShare(_) => ExtensionType::KeyShare,
            Self::PresharedKeyModes(_) => ExtensionType::PSKKeyExchangeModes,
            Self::PresharedKey(_) => ExtensionType::PreSharedKey,
            Self::Cookie(_) => ExtensionType::Cookie,
            Self::ExtendedMasterSecretRequest => ExtensionType::ExtendedMasterSecret,
            Self::CertificateStatusRequest(_) => ExtensionType::StatusRequest,
            Self::SignedCertificateTimestampRequest => ExtensionType::SCT,
            Self::TransportParameters(_) => ExtensionType::TransportParameters,
            Self::TransportParametersDraft(_) => ExtensionType::TransportParametersDraft,
            Self::EarlyData => ExtensionType::EarlyData,
            Self::Unknown(ref r) => r.typ,
        }
    }
}

impl Codec for ClientExtension {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.get_type().encode(bytes);

        let mut sub: Vec<u8> = Vec::new();
        match *self {
            Self::ECPointFormats(ref r) => r.encode(&mut sub),
            Self::NamedGroups(ref r) => r.encode(&mut sub),
            Self::SignatureAlgorithms(ref r) => r.encode(&mut sub),
            Self::ServerName(ref r) => r.encode(&mut sub),
            Self::SessionTicket(ClientSessionTicket::Request)
            | Self::ExtendedMasterSecretRequest
            | Self::SignedCertificateTimestampRequest
            | Self::EarlyData => {}
            Self::SessionTicket(ClientSessionTicket::Offer(ref r)) => r.encode(&mut sub),
            Self::Protocols(ref r) => r.encode(&mut sub),
            Self::SupportedVersions(ref r) => r.encode(&mut sub),
            Self::KeyShare(ref r) => r.encode(&mut sub),
            Self::PresharedKeyModes(ref r) => r.encode(&mut sub),
            Self::PresharedKey(ref r) => r.encode(&mut sub),
            Self::Cookie(ref r) => r.encode(&mut sub),
            Self::CertificateStatusRequest(ref r) => r.encode(&mut sub),
            Self::TransportParameters(ref r) | Self::TransportParametersDraft(ref r) => {
                sub.extend_from_slice(r);
            }
            Self::Unknown(ref r) => r.encode(&mut sub),
        }

        (sub.len() as u16).encode(bytes);
        bytes.append(&mut sub);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let typ = ExtensionType::read(r)?;
        let len = u16::read(r)? as usize;
        let mut sub = r.sub(len)?;

        let ext = match typ {
            ExtensionType::ECPointFormats => Self::ECPointFormats(Vec::read(&mut sub)?),
            ExtensionType::EllipticCurves => Self::NamedGroups(Vec::read(&mut sub)?),
            ExtensionType::SignatureAlgorithms => Self::SignatureAlgorithms(Vec::read(&mut sub)?),
            ExtensionType::ServerName => Self::ServerName(Vec::read(&mut sub)?),
            ExtensionType::SessionTicket => {
                if sub.any_left() {
                    let contents = Payload::read(&mut sub);
                    Self::SessionTicket(ClientSessionTicket::Offer(contents))
                } else {
                    Self::SessionTicket(ClientSessionTicket::Request)
                }
            }
            ExtensionType::ALProtocolNegotiation => Self::Protocols(Vec::read(&mut sub)?),
            ExtensionType::SupportedVersions => Self::SupportedVersions(Vec::read(&mut sub)?),
            ExtensionType::KeyShare => Self::KeyShare(Vec::read(&mut sub)?),
            ExtensionType::PSKKeyExchangeModes => Self::PresharedKeyModes(Vec::read(&mut sub)?),
            ExtensionType::PreSharedKey => Self::PresharedKey(PresharedKeyOffer::read(&mut sub)?),
            ExtensionType::Cookie => Self::Cookie(PayloadU16::read(&mut sub)?),
            ExtensionType::ExtendedMasterSecret if !sub.any_left() => {
                Self::ExtendedMasterSecretRequest
            }
            ExtensionType::StatusRequest => {
                let csr = CertificateStatusRequest::read(&mut sub)?;
                Self::CertificateStatusRequest(csr)
            }
            ExtensionType::SCT if !sub.any_left() => Self::SignedCertificateTimestampRequest,
            ExtensionType::TransportParameters => Self::TransportParameters(sub.rest().to_vec()),
            ExtensionType::TransportParametersDraft => {
                Self::TransportParametersDraft(sub.rest().to_vec())
            }
            ExtensionType::EarlyData if !sub.any_left() => Self::EarlyData,
            _ => Self::Unknown(UnknownExtension::read(typ, &mut sub)),
        };

        sub.expect_empty("ClientExtension")
            .map(|_| ext)
    }
}

fn trim_hostname_trailing_dot_for_sni(dns_name: DnsNameRef) -> DnsName {
    let dns_name_str: &str = dns_name.as_ref();

    // RFC6066: "The hostname is represented as a byte string using
    // ASCII encoding without a trailing dot"
    if dns_name_str.ends_with('.') {
        let trimmed = &dns_name_str[0..dns_name_str.len() - 1];
        DnsNameRef::try_from(trimmed)
            .unwrap()
            .to_owned()
    } else {
        dns_name.to_owned()
    }
}

impl ClientExtension {
    /// Make a basic SNI ServerNameRequest quoting `hostname`.
    pub fn make_sni(dns_name: DnsNameRef) -> Self {
        let name = ServerName {
            typ: ServerNameType::HostName,
            payload: ServerNamePayload::new_hostname(trim_hostname_trailing_dot_for_sni(dns_name)),
        };

        Self::ServerName(vec![name])
    }
}

#[derive(Clone, Debug)]
pub enum ClientSessionTicket {
    Request,
    Offer(Payload),
}

#[derive(Clone, Debug)]
pub enum ServerExtension {
    ECPointFormats(Vec<ECPointFormat>),
    ServerNameAck,
    SessionTicketAck,
    RenegotiationInfo(PayloadU8),
    Protocols(Vec<ProtocolName>),
    KeyShare(KeyShareEntry),
    PresharedKey(u16),
    ExtendedMasterSecretAck,
    CertificateStatusAck,
    SignedCertificateTimestamp(Vec<Sct>),
    SupportedVersions(ProtocolVersion),
    TransportParameters(Vec<u8>),
    TransportParametersDraft(Vec<u8>),
    EarlyData,
    Unknown(UnknownExtension),
}

impl ServerExtension {
    pub fn get_type(&self) -> ExtensionType {
        match *self {
            Self::ECPointFormats(_) => ExtensionType::ECPointFormats,
            Self::ServerNameAck => ExtensionType::ServerName,
            Self::SessionTicketAck => ExtensionType::SessionTicket,
            Self::RenegotiationInfo(_) => ExtensionType::RenegotiationInfo,
            Self::Protocols(_) => ExtensionType::ALProtocolNegotiation,
            Self::KeyShare(_) => ExtensionType::KeyShare,
            Self::PresharedKey(_) => ExtensionType::PreSharedKey,
            Self::ExtendedMasterSecretAck => ExtensionType::ExtendedMasterSecret,
            Self::CertificateStatusAck => ExtensionType::StatusRequest,
            Self::SignedCertificateTimestamp(_) => ExtensionType::SCT,
            Self::SupportedVersions(_) => ExtensionType::SupportedVersions,
            Self::TransportParameters(_) => ExtensionType::TransportParameters,
            Self::TransportParametersDraft(_) => ExtensionType::TransportParametersDraft,
            Self::EarlyData => ExtensionType::EarlyData,
            Self::Unknown(ref r) => r.typ,
        }
    }
}

impl Codec for ServerExtension {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.get_type().encode(bytes);

        let mut sub: Vec<u8> = Vec::new();
        match *self {
            Self::ECPointFormats(ref r) => r.encode(&mut sub),
            Self::ServerNameAck
            | Self::SessionTicketAck
            | Self::ExtendedMasterSecretAck
            | Self::CertificateStatusAck
            | Self::EarlyData => {}
            Self::RenegotiationInfo(ref r) => r.encode(&mut sub),
            Self::Protocols(ref r) => r.encode(&mut sub),
            Self::KeyShare(ref r) => r.encode(&mut sub),
            Self::PresharedKey(r) => r.encode(&mut sub),
            Self::SignedCertificateTimestamp(ref r) => r.encode(&mut sub),
            Self::SupportedVersions(ref r) => r.encode(&mut sub),
            Self::TransportParameters(ref r) | Self::TransportParametersDraft(ref r) => {
                sub.extend_from_slice(r);
            }
            Self::Unknown(ref r) => r.encode(&mut sub),
        }

        (sub.len() as u16).encode(bytes);
        bytes.append(&mut sub);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let typ = ExtensionType::read(r)?;
        let len = u16::read(r)? as usize;
        let mut sub = r.sub(len)?;

        let ext = match typ {
            ExtensionType::ECPointFormats => Self::ECPointFormats(Vec::read(&mut sub)?),
            ExtensionType::ServerName => Self::ServerNameAck,
            ExtensionType::SessionTicket => Self::SessionTicketAck,
            ExtensionType::StatusRequest => Self::CertificateStatusAck,
            ExtensionType::RenegotiationInfo => Self::RenegotiationInfo(PayloadU8::read(&mut sub)?),
            ExtensionType::ALProtocolNegotiation => Self::Protocols(Vec::read(&mut sub)?),
            ExtensionType::KeyShare => Self::KeyShare(KeyShareEntry::read(&mut sub)?),
            ExtensionType::PreSharedKey => Self::PresharedKey(u16::read(&mut sub)?),
            ExtensionType::ExtendedMasterSecret => Self::ExtendedMasterSecretAck,
            ExtensionType::SCT => Self::SignedCertificateTimestamp(Vec::read(&mut sub)?),
            ExtensionType::SupportedVersions => {
                Self::SupportedVersions(ProtocolVersion::read(&mut sub)?)
            }
            ExtensionType::TransportParameters => Self::TransportParameters(sub.rest().to_vec()),
            ExtensionType::TransportParametersDraft => {
                Self::TransportParametersDraft(sub.rest().to_vec())
            }
            ExtensionType::EarlyData => Self::EarlyData,
            _ => Self::Unknown(UnknownExtension::read(typ, &mut sub)),
        };

        sub.expect_empty("ServerExtension")
            .map(|_| ext)
    }
}

impl ServerExtension {
    pub fn make_alpn(proto: &[&[u8]]) -> Self {
        Self::Protocols(Vec::from_slices(proto))
    }

    pub fn make_empty_renegotiation_info() -> Self {
        let empty = Vec::new();
        Self::RenegotiationInfo(PayloadU8::new(empty))
    }

    pub fn make_sct(sctl: Vec<u8>) -> Self {
        let scts = Vec::read_bytes(&sctl).expect("invalid SCT list");
        Self::SignedCertificateTimestamp(scts)
    }
}

#[derive(Debug)]
pub struct ClientHelloPayload {
    pub client_version: ProtocolVersion,
    pub random: Random,
    pub session_id: SessionId,
    pub cipher_suites: Vec<CipherSuite>,
    pub compression_methods: Vec<Compression>,
    pub extensions: Vec<ClientExtension>,
}

impl Codec for ClientHelloPayload {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.client_version.encode(bytes);
        self.random.encode(bytes);
        self.session_id.encode(bytes);
        self.cipher_suites.encode(bytes);
        self.compression_methods.encode(bytes);

        if !self.extensions.is_empty() {
            self.extensions.encode(bytes);
        }
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let mut ret = Self {
            client_version: ProtocolVersion::read(r)?,
            random: Random::read(r)?,
            session_id: SessionId::read(r)?,
            cipher_suites: Vec::read(r)?,
            compression_methods: Vec::read(r)?,
            extensions: Vec::new(),
        };

        if r.any_left() {
            ret.extensions = Vec::read(r)?;
        }

        match (r.any_left(), ret.extensions.is_empty()) {
            (true, _) => Err(InvalidMessage::TrailingData("ClientHelloPayload")),
            (_, true) => Err(InvalidMessage::MissingData("ClientHelloPayload")),
            _ => Ok(ret),
        }
    }
}

impl TlsListElement for CipherSuite {
    const SIZE_LEN: ListLength = ListLength::U16;
}

impl TlsListElement for Compression {
    const SIZE_LEN: ListLength = ListLength::U8;
}

impl TlsListElement for ClientExtension {
    const SIZE_LEN: ListLength = ListLength::U16;
}

impl ClientHelloPayload {
    /// Returns true if there is more than one extension of a given
    /// type.
    pub fn has_duplicate_extension(&self) -> bool {
        let mut seen = collections::HashSet::new();

        for ext in &self.extensions {
            let typ = ext.get_type().get_u16();

            if seen.contains(&typ) {
                return true;
            }
            seen.insert(typ);
        }

        false
    }

    pub fn find_extension(&self, ext: ExtensionType) -> Option<&ClientExtension> {
        self.extensions
            .iter()
            .find(|x| x.get_type() == ext)
    }

    pub fn get_sni_extension(&self) -> Option<&[ServerName]> {
        let ext = self.find_extension(ExtensionType::ServerName)?;
        match *ext {
            // Does this comply with RFC6066?
            //
            // [RFC6066][] specifies that literal IP addresses are illegal in
            // `ServerName`s with a `name_type` of `host_name`.
            //
            // Some clients incorrectly send such extensions: we choose to
            // successfully parse these (into `ServerNamePayload::IpAddress`)
            // but then act like the client sent no `server_name` extension.
            //
            // [RFC6066]: https://datatracker.ietf.org/doc/html/rfc6066#section-3
            ClientExtension::ServerName(ref req)
                if !req
                    .iter()
                    .any(|name| matches!(name.payload, ServerNamePayload::IpAddress(_))) =>
            {
                Some(req)
            }
            _ => None,
        }
    }

    pub fn get_sigalgs_extension(&self) -> Option<&[SignatureScheme]> {
        let ext = self.find_extension(ExtensionType::SignatureAlgorithms)?;
        match *ext {
            ClientExtension::SignatureAlgorithms(ref req) => Some(req),
            _ => None,
        }
    }

    pub fn get_namedgroups_extension(&self) -> Option<&[NamedGroup]> {
        let ext = self.find_extension(ExtensionType::EllipticCurves)?;
        match *ext {
            ClientExtension::NamedGroups(ref req) => Some(req),
            _ => None,
        }
    }

    pub fn get_ecpoints_extension(&self) -> Option<&[ECPointFormat]> {
        let ext = self.find_extension(ExtensionType::ECPointFormats)?;
        match *ext {
            ClientExtension::ECPointFormats(ref req) => Some(req),
            _ => None,
        }
    }

    pub fn get_alpn_extension(&self) -> Option<&Vec<ProtocolName>> {
        let ext = self.find_extension(ExtensionType::ALProtocolNegotiation)?;
        match *ext {
            ClientExtension::Protocols(ref req) => Some(req),
            _ => None,
        }
    }

    pub fn get_quic_params_extension(&self) -> Option<Vec<u8>> {
        let ext = self
            .find_extension(ExtensionType::TransportParameters)
            .or_else(|| self.find_extension(ExtensionType::TransportParametersDraft))?;
        match *ext {
            ClientExtension::TransportParameters(ref bytes)
            | ClientExtension::TransportParametersDraft(ref bytes) => Some(bytes.to_vec()),
            _ => None,
        }
    }

    pub fn get_ticket_extension(&self) -> Option<&ClientExtension> {
        self.find_extension(ExtensionType::SessionTicket)
    }

    pub fn get_versions_extension(&self) -> Option<&[ProtocolVersion]> {
        let ext = self.find_extension(ExtensionType::SupportedVersions)?;
        match *ext {
            ClientExtension::SupportedVersions(ref vers) => Some(vers),
            _ => None,
        }
    }

    pub fn get_keyshare_extension(&self) -> Option<&[KeyShareEntry]> {
        let ext = self.find_extension(ExtensionType::KeyShare)?;
        match *ext {
            ClientExtension::KeyShare(ref shares) => Some(shares),
            _ => None,
        }
    }

    pub fn has_keyshare_extension_with_duplicates(&self) -> bool {
        if let Some(entries) = self.get_keyshare_extension() {
            let mut seen = collections::HashSet::new();

            for kse in entries {
                let grp = kse.group.get_u16();

                if !seen.insert(grp) {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_psk(&self) -> Option<&PresharedKeyOffer> {
        let ext = self.find_extension(ExtensionType::PreSharedKey)?;
        match *ext {
            ClientExtension::PresharedKey(ref psk) => Some(psk),
            _ => None,
        }
    }

    pub fn check_psk_ext_is_last(&self) -> bool {
        self.extensions
            .last()
            .map_or(false, |ext| ext.get_type() == ExtensionType::PreSharedKey)
    }

    pub fn get_psk_modes(&self) -> Option<&[PSKKeyExchangeMode]> {
        let ext = self.find_extension(ExtensionType::PSKKeyExchangeModes)?;
        match *ext {
            ClientExtension::PresharedKeyModes(ref psk_modes) => Some(psk_modes),
            _ => None,
        }
    }

    pub fn psk_mode_offered(&self, mode: PSKKeyExchangeMode) -> bool {
        self.get_psk_modes()
            .map(|modes| modes.contains(&mode))
            .unwrap_or(false)
    }

    pub fn set_psk_binder(&mut self, binder: impl Into<Vec<u8>>) {
        let last_extension = self.extensions.last_mut();
        if let Some(ClientExtension::PresharedKey(ref mut offer)) = last_extension {
            offer.binders[0] = PresharedKeyBinder::from(binder.into());
        }
    }

    pub fn ems_support_offered(&self) -> bool {
        self.find_extension(ExtensionType::ExtendedMasterSecret)
            .is_some()
    }

    pub fn early_data_extension_offered(&self) -> bool {
        self.find_extension(ExtensionType::EarlyData)
            .is_some()
    }
}

#[derive(Debug)]
pub enum HelloRetryExtension {
    KeyShare(NamedGroup),
    Cookie(PayloadU16),
    SupportedVersions(ProtocolVersion),
    Unknown(UnknownExtension),
}

impl HelloRetryExtension {
    pub fn get_type(&self) -> ExtensionType {
        match *self {
            Self::KeyShare(_) => ExtensionType::KeyShare,
            Self::Cookie(_) => ExtensionType::Cookie,
            Self::SupportedVersions(_) => ExtensionType::SupportedVersions,
            Self::Unknown(ref r) => r.typ,
        }
    }
}

impl Codec for HelloRetryExtension {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.get_type().encode(bytes);

        let mut sub: Vec<u8> = Vec::new();
        match *self {
            Self::KeyShare(ref r) => r.encode(&mut sub),
            Self::Cookie(ref r) => r.encode(&mut sub),
            Self::SupportedVersions(ref r) => r.encode(&mut sub),
            Self::Unknown(ref r) => r.encode(&mut sub),
        }

        (sub.len() as u16).encode(bytes);
        bytes.append(&mut sub);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let typ = ExtensionType::read(r)?;
        let len = u16::read(r)? as usize;
        let mut sub = r.sub(len)?;

        let ext = match typ {
            ExtensionType::KeyShare => Self::KeyShare(NamedGroup::read(&mut sub)?),
            ExtensionType::Cookie => Self::Cookie(PayloadU16::read(&mut sub)?),
            ExtensionType::SupportedVersions => {
                Self::SupportedVersions(ProtocolVersion::read(&mut sub)?)
            }
            _ => Self::Unknown(UnknownExtension::read(typ, &mut sub)),
        };

        sub.expect_empty("HelloRetryExtension")
            .map(|_| ext)
    }
}

impl TlsListElement for HelloRetryExtension {
    const SIZE_LEN: ListLength = ListLength::U16;
}

#[derive(Debug)]
pub struct HelloRetryRequest {
    pub legacy_version: ProtocolVersion,
    pub session_id: SessionId,
    pub cipher_suite: CipherSuite,
    pub extensions: Vec<HelloRetryExtension>,
}

impl Codec for HelloRetryRequest {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.legacy_version.encode(bytes);
        HELLO_RETRY_REQUEST_RANDOM.encode(bytes);
        self.session_id.encode(bytes);
        self.cipher_suite.encode(bytes);
        Compression::Null.encode(bytes);
        self.extensions.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let session_id = SessionId::read(r)?;
        let cipher_suite = CipherSuite::read(r)?;
        let compression = Compression::read(r)?;

        if compression != Compression::Null {
            return Err(InvalidMessage::UnsupportedCompression);
        }

        Ok(Self {
            legacy_version: ProtocolVersion::Unknown(0),
            session_id,
            cipher_suite,
            extensions: Vec::read(r)?,
        })
    }
}

impl HelloRetryRequest {
    /// Returns true if there is more than one extension of a given
    /// type.
    pub fn has_duplicate_extension(&self) -> bool {
        let mut seen = collections::HashSet::new();

        for ext in &self.extensions {
            let typ = ext.get_type().get_u16();

            if seen.contains(&typ) {
                return true;
            }
            seen.insert(typ);
        }

        false
    }

    pub fn has_unknown_extension(&self) -> bool {
        self.extensions.iter().any(|ext| {
            ext.get_type() != ExtensionType::KeyShare
                && ext.get_type() != ExtensionType::SupportedVersions
                && ext.get_type() != ExtensionType::Cookie
        })
    }

    fn find_extension(&self, ext: ExtensionType) -> Option<&HelloRetryExtension> {
        self.extensions
            .iter()
            .find(|x| x.get_type() == ext)
    }

    pub fn get_requested_key_share_group(&self) -> Option<NamedGroup> {
        let ext = self.find_extension(ExtensionType::KeyShare)?;
        match *ext {
            HelloRetryExtension::KeyShare(grp) => Some(grp),
            _ => None,
        }
    }

    pub fn get_cookie(&self) -> Option<&PayloadU16> {
        let ext = self.find_extension(ExtensionType::Cookie)?;
        match *ext {
            HelloRetryExtension::Cookie(ref ck) => Some(ck),
            _ => None,
        }
    }

    pub fn get_supported_versions(&self) -> Option<ProtocolVersion> {
        let ext = self.find_extension(ExtensionType::SupportedVersions)?;
        match *ext {
            HelloRetryExtension::SupportedVersions(ver) => Some(ver),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ServerHelloPayload {
    pub legacy_version: ProtocolVersion,
    pub random: Random,
    pub session_id: SessionId,
    pub cipher_suite: CipherSuite,
    pub compression_method: Compression,
    pub extensions: Vec<ServerExtension>,
}

impl Codec for ServerHelloPayload {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.legacy_version.encode(bytes);
        self.random.encode(bytes);

        self.session_id.encode(bytes);
        self.cipher_suite.encode(bytes);
        self.compression_method.encode(bytes);

        if !self.extensions.is_empty() {
            self.extensions.encode(bytes);
        }
    }

    // minus version and random, which have already been read.
    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let session_id = SessionId::read(r)?;
        let suite = CipherSuite::read(r)?;
        let compression = Compression::read(r)?;

        // RFC5246:
        // "The presence of extensions can be detected by determining whether
        //  there are bytes following the compression_method field at the end of
        //  the ServerHello."
        let extensions = if r.any_left() { Vec::read(r)? } else { vec![] };

        let ret = Self {
            legacy_version: ProtocolVersion::Unknown(0),
            random: ZERO_RANDOM,
            session_id,
            cipher_suite: suite,
            compression_method: compression,
            extensions,
        };

        r.expect_empty("ServerHelloPayload")
            .map(|_| ret)
    }
}

impl HasServerExtensions for ServerHelloPayload {
    fn get_extensions(&self) -> &[ServerExtension] {
        &self.extensions
    }
}

impl ServerHelloPayload {
    pub fn get_key_share(&self) -> Option<&KeyShareEntry> {
        let ext = self.find_extension(ExtensionType::KeyShare)?;
        match *ext {
            ServerExtension::KeyShare(ref share) => Some(share),
            _ => None,
        }
    }

    pub fn get_psk_index(&self) -> Option<u16> {
        let ext = self.find_extension(ExtensionType::PreSharedKey)?;
        match *ext {
            ServerExtension::PresharedKey(ref index) => Some(*index),
            _ => None,
        }
    }

    pub fn get_ecpoints_extension(&self) -> Option<&[ECPointFormat]> {
        let ext = self.find_extension(ExtensionType::ECPointFormats)?;
        match *ext {
            ServerExtension::ECPointFormats(ref fmts) => Some(fmts),
            _ => None,
        }
    }

    pub fn ems_support_acked(&self) -> bool {
        self.find_extension(ExtensionType::ExtendedMasterSecret)
            .is_some()
    }

    pub fn get_sct_list(&self) -> Option<&[Sct]> {
        let ext = self.find_extension(ExtensionType::SCT)?;
        match *ext {
            ServerExtension::SignedCertificateTimestamp(ref sctl) => Some(sctl),
            _ => None,
        }
    }

    pub fn get_supported_versions(&self) -> Option<ProtocolVersion> {
        let ext = self.find_extension(ExtensionType::SupportedVersions)?;
        match *ext {
            ServerExtension::SupportedVersions(vers) => Some(vers),
            _ => None,
        }
    }
}

pub type CertificatePayload = Vec<key::Certificate>;

impl TlsListElement for key::Certificate {
    const SIZE_LEN: ListLength = ListLength::U24 { max: 0x1_0000 };
}

// TLS1.3 changes the Certificate payload encoding.
// That's annoying. It means the parsing is not
// context-free any more.

#[derive(Debug)]
pub enum CertificateExtension {
    CertificateStatus(CertificateStatus),
    SignedCertificateTimestamp(Vec<Sct>),
    Unknown(UnknownExtension),
}

impl CertificateExtension {
    pub fn get_type(&self) -> ExtensionType {
        match *self {
            Self::CertificateStatus(_) => ExtensionType::StatusRequest,
            Self::SignedCertificateTimestamp(_) => ExtensionType::SCT,
            Self::Unknown(ref r) => r.typ,
        }
    }

    pub fn make_sct(sct_list: Vec<u8>) -> Self {
        let sctl = Vec::read_bytes(&sct_list).expect("invalid SCT list");
        Self::SignedCertificateTimestamp(sctl)
    }

    pub fn get_cert_status(&self) -> Option<&Vec<u8>> {
        match *self {
            Self::CertificateStatus(ref cs) => Some(&cs.ocsp_response.0),
            _ => None,
        }
    }

    pub fn get_sct_list(&self) -> Option<&[Sct]> {
        match *self {
            Self::SignedCertificateTimestamp(ref sctl) => Some(sctl),
            _ => None,
        }
    }
}

impl Codec for CertificateExtension {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.get_type().encode(bytes);

        let mut sub: Vec<u8> = Vec::new();
        match *self {
            Self::CertificateStatus(ref r) => r.encode(&mut sub),
            Self::SignedCertificateTimestamp(ref r) => r.encode(&mut sub),
            Self::Unknown(ref r) => r.encode(&mut sub),
        }

        (sub.len() as u16).encode(bytes);
        bytes.append(&mut sub);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let typ = ExtensionType::read(r)?;
        let len = u16::read(r)? as usize;
        let mut sub = r.sub(len)?;

        let ext = match typ {
            ExtensionType::StatusRequest => {
                let st = CertificateStatus::read(&mut sub)?;
                Self::CertificateStatus(st)
            }
            ExtensionType::SCT => Self::SignedCertificateTimestamp(Vec::read(&mut sub)?),
            _ => Self::Unknown(UnknownExtension::read(typ, &mut sub)),
        };

        sub.expect_empty("CertificateExtension")
            .map(|_| ext)
    }
}

impl TlsListElement for CertificateExtension {
    const SIZE_LEN: ListLength = ListLength::U16;
}

#[derive(Debug)]
pub struct CertificateEntry {
    pub cert: key::Certificate,
    pub exts: Vec<CertificateExtension>,
}

impl Codec for CertificateEntry {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.cert.encode(bytes);
        self.exts.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        Ok(Self {
            cert: key::Certificate::read(r)?,
            exts: Vec::read(r)?,
        })
    }
}

impl CertificateEntry {
    pub fn new(cert: key::Certificate) -> Self {
        Self {
            cert,
            exts: Vec::new(),
        }
    }

    pub fn has_duplicate_extension(&self) -> bool {
        let mut seen = collections::HashSet::new();

        for ext in &self.exts {
            let typ = ext.get_type().get_u16();

            if seen.contains(&typ) {
                return true;
            }
            seen.insert(typ);
        }

        false
    }

    pub fn has_unknown_extension(&self) -> bool {
        self.exts.iter().any(|ext| {
            ext.get_type() != ExtensionType::StatusRequest && ext.get_type() != ExtensionType::SCT
        })
    }

    pub fn get_ocsp_response(&self) -> Option<&Vec<u8>> {
        self.exts
            .iter()
            .find(|ext| ext.get_type() == ExtensionType::StatusRequest)
            .and_then(CertificateExtension::get_cert_status)
    }

    pub fn get_scts(&self) -> Option<&[Sct]> {
        self.exts
            .iter()
            .find(|ext| ext.get_type() == ExtensionType::SCT)
            .and_then(CertificateExtension::get_sct_list)
    }
}

impl TlsListElement for CertificateEntry {
    const SIZE_LEN: ListLength = ListLength::U24 { max: 0x1_0000 };
}

#[derive(Debug)]
pub struct CertificatePayloadTLS13 {
    pub context: PayloadU8,
    pub entries: Vec<CertificateEntry>,
}

impl Codec for CertificatePayloadTLS13 {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.context.encode(bytes);
        self.entries.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        Ok(Self {
            context: PayloadU8::read(r)?,
            entries: Vec::read(r)?,
        })
    }
}

impl CertificatePayloadTLS13 {
    pub fn new(entries: Vec<CertificateEntry>) -> Self {
        Self {
            context: PayloadU8::empty(),
            entries,
        }
    }

    pub fn any_entry_has_duplicate_extension(&self) -> bool {
        for entry in &self.entries {
            if entry.has_duplicate_extension() {
                return true;
            }
        }

        false
    }

    pub fn any_entry_has_unknown_extension(&self) -> bool {
        for entry in &self.entries {
            if entry.has_unknown_extension() {
                return true;
            }
        }

        false
    }

    pub fn any_entry_has_extension(&self) -> bool {
        for entry in &self.entries {
            if !entry.exts.is_empty() {
                return true;
            }
        }

        false
    }

    pub fn get_end_entity_ocsp(&self) -> Vec<u8> {
        self.entries
            .first()
            .and_then(CertificateEntry::get_ocsp_response)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_end_entity_scts(&self) -> Option<&[Sct]> {
        self.entries
            .first()
            .and_then(CertificateEntry::get_scts)
    }

    pub fn convert(&self) -> CertificatePayload {
        let mut ret = Vec::new();
        for entry in &self.entries {
            ret.push(entry.cert.clone());
        }
        ret
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyExchangeAlgorithm {
    BulkOnly,
    DH,
    DHE,
    RSA,
    ECDH,
    ECDHE,
}

// We don't support arbitrary curves.  It's a terrible
// idea and unnecessary attack surface.  Please,
// get a grip.
#[derive(Debug)]
pub struct ECParameters {
    pub curve_type: ECCurveType,
    pub named_group: NamedGroup,
}

impl Codec for ECParameters {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.curve_type.encode(bytes);
        self.named_group.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let ct = ECCurveType::read(r)?;
        if ct != ECCurveType::NamedCurve {
            return Err(InvalidMessage::UnsupportedCurveType);
        }

        let grp = NamedGroup::read(r)?;

        Ok(Self {
            curve_type: ct,
            named_group: grp,
        })
    }
}

#[derive(Debug)]
pub struct ClientECDHParams {
    pub public: PayloadU8,
}

impl Codec for ClientECDHParams {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.public.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let pb = PayloadU8::read(r)?;
        Ok(Self { public: pb })
    }
}

#[derive(Debug)]
pub struct ServerECDHParams {
    pub curve_params: ECParameters,
    pub public: PayloadU8,
}

impl ServerECDHParams {
    pub fn new(named_group: NamedGroup, pubkey: &[u8]) -> Self {
        Self {
            curve_params: ECParameters {
                curve_type: ECCurveType::NamedCurve,
                named_group,
            },
            public: PayloadU8::new(pubkey.to_vec()),
        }
    }
}

impl Codec for ServerECDHParams {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.curve_params.encode(bytes);
        self.public.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let cp = ECParameters::read(r)?;
        let pb = PayloadU8::read(r)?;

        Ok(Self {
            curve_params: cp,
            public: pb,
        })
    }
}

#[derive(Debug)]
pub struct ECDHEServerKeyExchange {
    pub params: ServerECDHParams,
    pub dss: DigitallySignedStruct,
}

impl Codec for ECDHEServerKeyExchange {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.params.encode(bytes);
        self.dss.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let params = ServerECDHParams::read(r)?;
        let dss = DigitallySignedStruct::read(r)?;

        Ok(Self { params, dss })
    }
}

#[derive(Debug)]
pub enum ServerKeyExchangePayload {
    ECDHE(ECDHEServerKeyExchange),
    Unknown(Payload),
}

impl Codec for ServerKeyExchangePayload {
    fn encode(&self, bytes: &mut Vec<u8>) {
        match *self {
            Self::ECDHE(ref x) => x.encode(bytes),
            Self::Unknown(ref x) => x.encode(bytes),
        }
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        // read as Unknown, fully parse when we know the
        // KeyExchangeAlgorithm
        Ok(Self::Unknown(Payload::read(r)))
    }
}

impl ServerKeyExchangePayload {
    pub fn unwrap_given_kxa(&self, kxa: KeyExchangeAlgorithm) -> Option<ECDHEServerKeyExchange> {
        if let Self::Unknown(ref unk) = *self {
            let mut rd = Reader::init(&unk.0);

            let result = match kxa {
                KeyExchangeAlgorithm::ECDHE => ECDHEServerKeyExchange::read(&mut rd),
                _ => return None,
            };

            if !rd.any_left() {
                return result.ok();
            };
        }

        None
    }
}

// -- EncryptedExtensions (TLS1.3 only) --

impl TlsListElement for ServerExtension {
    const SIZE_LEN: ListLength = ListLength::U16;
}

pub trait HasServerExtensions {
    fn get_extensions(&self) -> &[ServerExtension];

    /// Returns true if there is more than one extension of a given
    /// type.
    fn has_duplicate_extension(&self) -> bool {
        let mut seen = collections::HashSet::new();

        for ext in self.get_extensions() {
            let typ = ext.get_type().get_u16();

            if seen.contains(&typ) {
                return true;
            }
            seen.insert(typ);
        }

        false
    }

    fn find_extension(&self, ext: ExtensionType) -> Option<&ServerExtension> {
        self.get_extensions()
            .iter()
            .find(|x| x.get_type() == ext)
    }

    fn get_alpn_protocol(&self) -> Option<&[u8]> {
        let ext = self.find_extension(ExtensionType::ALProtocolNegotiation)?;
        match *ext {
            ServerExtension::Protocols(ref protos) => protos.as_single_slice(),
            _ => None,
        }
    }

    fn get_quic_params_extension(&self) -> Option<Vec<u8>> {
        let ext = self
            .find_extension(ExtensionType::TransportParameters)
            .or_else(|| self.find_extension(ExtensionType::TransportParametersDraft))?;
        match *ext {
            ServerExtension::TransportParameters(ref bytes)
            | ServerExtension::TransportParametersDraft(ref bytes) => Some(bytes.to_vec()),
            _ => None,
        }
    }

    fn early_data_extension_offered(&self) -> bool {
        self.find_extension(ExtensionType::EarlyData)
            .is_some()
    }
}

impl HasServerExtensions for Vec<ServerExtension> {
    fn get_extensions(&self) -> &[ServerExtension] {
        self
    }
}

impl TlsListElement for ClientCertificateType {
    const SIZE_LEN: ListLength = ListLength::U8;
}

wrapped_payload!(
    /// A `DistinguishedName` is a `Vec<u8>` wrapped in internal types.
    ///
    /// It contains the DER or BER encoded [`Subject` field from RFC 5280](https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.6)
    /// for a single certificate. The Subject field is [encoded as an RFC 5280 `Name`](https://datatracker.ietf.org/doc/html/rfc5280#page-116).
    /// It can be decoded using [x509-parser's FromDer trait](https://docs.rs/x509-parser/latest/x509_parser/prelude/trait.FromDer.html).
    ///
    /// ```ignore
    /// for name in distinguished_names {
    ///     use x509_parser::prelude::FromDer;
    ///     println!("{}", x509_parser::x509::X509Name::from_der(&name.0)?.1);
    /// }
    /// ```
    DistinguishedName,
    PayloadU16,
);

impl TlsListElement for DistinguishedName {
    const SIZE_LEN: ListLength = ListLength::U16;
}

#[derive(Debug)]
pub struct CertificateRequestPayload {
    pub certtypes: Vec<ClientCertificateType>,
    pub sigschemes: Vec<SignatureScheme>,
    pub canames: Vec<DistinguishedName>,
}

impl Codec for CertificateRequestPayload {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.certtypes.encode(bytes);
        self.sigschemes.encode(bytes);
        self.canames.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let certtypes = Vec::read(r)?;
        let sigschemes = Vec::read(r)?;
        let canames = Vec::read(r)?;

        if sigschemes.is_empty() {
            warn!("meaningless CertificateRequest message");
            Err(InvalidMessage::NoSignatureSchemes)
        } else {
            Ok(Self {
                certtypes,
                sigschemes,
                canames,
            })
        }
    }
}

#[derive(Debug)]
pub enum CertReqExtension {
    SignatureAlgorithms(Vec<SignatureScheme>),
    AuthorityNames(Vec<DistinguishedName>),
    Unknown(UnknownExtension),
}

impl CertReqExtension {
    pub fn get_type(&self) -> ExtensionType {
        match *self {
            Self::SignatureAlgorithms(_) => ExtensionType::SignatureAlgorithms,
            Self::AuthorityNames(_) => ExtensionType::CertificateAuthorities,
            Self::Unknown(ref r) => r.typ,
        }
    }
}

impl Codec for CertReqExtension {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.get_type().encode(bytes);

        let mut sub: Vec<u8> = Vec::new();
        match *self {
            Self::SignatureAlgorithms(ref r) => r.encode(&mut sub),
            Self::AuthorityNames(ref r) => r.encode(&mut sub),
            Self::Unknown(ref r) => r.encode(&mut sub),
        }

        (sub.len() as u16).encode(bytes);
        bytes.append(&mut sub);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let typ = ExtensionType::read(r)?;
        let len = u16::read(r)? as usize;
        let mut sub = r.sub(len)?;

        let ext = match typ {
            ExtensionType::SignatureAlgorithms => {
                let schemes = Vec::read(&mut sub)?;
                if schemes.is_empty() {
                    return Err(InvalidMessage::NoSignatureSchemes);
                }
                Self::SignatureAlgorithms(schemes)
            }
            ExtensionType::CertificateAuthorities => {
                let cas = Vec::read(&mut sub)?;
                Self::AuthorityNames(cas)
            }
            _ => Self::Unknown(UnknownExtension::read(typ, &mut sub)),
        };

        sub.expect_empty("CertReqExtension")
            .map(|_| ext)
    }
}

impl TlsListElement for CertReqExtension {
    const SIZE_LEN: ListLength = ListLength::U16;
}

#[derive(Debug)]
pub struct CertificateRequestPayloadTLS13 {
    pub context: PayloadU8,
    pub extensions: Vec<CertReqExtension>,
}

impl Codec for CertificateRequestPayloadTLS13 {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.context.encode(bytes);
        self.extensions.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let context = PayloadU8::read(r)?;
        let extensions = Vec::read(r)?;

        Ok(Self {
            context,
            extensions,
        })
    }
}

impl CertificateRequestPayloadTLS13 {
    pub fn find_extension(&self, ext: ExtensionType) -> Option<&CertReqExtension> {
        self.extensions
            .iter()
            .find(|x| x.get_type() == ext)
    }

    pub fn get_sigalgs_extension(&self) -> Option<&[SignatureScheme]> {
        let ext = self.find_extension(ExtensionType::SignatureAlgorithms)?;
        match *ext {
            CertReqExtension::SignatureAlgorithms(ref sa) => Some(sa),
            _ => None,
        }
    }

    pub fn get_authorities_extension(&self) -> Option<&[DistinguishedName]> {
        let ext = self.find_extension(ExtensionType::CertificateAuthorities)?;
        match *ext {
            CertReqExtension::AuthorityNames(ref an) => Some(an),
            _ => None,
        }
    }
}

// -- NewSessionTicket --
#[derive(Debug)]
pub struct NewSessionTicketPayload {
    pub lifetime_hint: u32,
    pub ticket: PayloadU16,
}

impl NewSessionTicketPayload {
    pub fn new(lifetime_hint: u32, ticket: Vec<u8>) -> Self {
        Self {
            lifetime_hint,
            ticket: PayloadU16::new(ticket),
        }
    }
}

impl Codec for NewSessionTicketPayload {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.lifetime_hint.encode(bytes);
        self.ticket.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let lifetime = u32::read(r)?;
        let ticket = PayloadU16::read(r)?;

        Ok(Self {
            lifetime_hint: lifetime,
            ticket,
        })
    }
}

// -- NewSessionTicket electric boogaloo --
#[derive(Debug)]
pub enum NewSessionTicketExtension {
    EarlyData(u32),
    Unknown(UnknownExtension),
}

impl NewSessionTicketExtension {
    pub fn get_type(&self) -> ExtensionType {
        match *self {
            Self::EarlyData(_) => ExtensionType::EarlyData,
            Self::Unknown(ref r) => r.typ,
        }
    }
}

impl Codec for NewSessionTicketExtension {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.get_type().encode(bytes);

        let mut sub: Vec<u8> = Vec::new();
        match *self {
            Self::EarlyData(r) => r.encode(&mut sub),
            Self::Unknown(ref r) => r.encode(&mut sub),
        }

        (sub.len() as u16).encode(bytes);
        bytes.append(&mut sub);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let typ = ExtensionType::read(r)?;
        let len = u16::read(r)? as usize;
        let mut sub = r.sub(len)?;

        let ext = match typ {
            ExtensionType::EarlyData => Self::EarlyData(u32::read(&mut sub)?),
            _ => Self::Unknown(UnknownExtension::read(typ, &mut sub)),
        };

        sub.expect_empty("NewSessionTicketExtension")
            .map(|_| ext)
    }
}

impl TlsListElement for NewSessionTicketExtension {
    const SIZE_LEN: ListLength = ListLength::U16;
}

#[derive(Debug)]
pub struct NewSessionTicketPayloadTLS13 {
    pub lifetime: u32,
    pub age_add: u32,
    pub nonce: PayloadU8,
    pub ticket: PayloadU16,
    pub exts: Vec<NewSessionTicketExtension>,
}

impl NewSessionTicketPayloadTLS13 {
    pub fn new(lifetime: u32, age_add: u32, nonce: Vec<u8>, ticket: Vec<u8>) -> Self {
        Self {
            lifetime,
            age_add,
            nonce: PayloadU8::new(nonce),
            ticket: PayloadU16::new(ticket),
            exts: vec![],
        }
    }

    pub fn has_duplicate_extension(&self) -> bool {
        let mut seen = collections::HashSet::new();

        for ext in &self.exts {
            let typ = ext.get_type().get_u16();

            if seen.contains(&typ) {
                return true;
            }
            seen.insert(typ);
        }

        false
    }

    pub fn find_extension(&self, ext: ExtensionType) -> Option<&NewSessionTicketExtension> {
        self.exts
            .iter()
            .find(|x| x.get_type() == ext)
    }

    pub fn get_max_early_data_size(&self) -> Option<u32> {
        let ext = self.find_extension(ExtensionType::EarlyData)?;
        match *ext {
            NewSessionTicketExtension::EarlyData(ref sz) => Some(*sz),
            _ => None,
        }
    }
}

impl Codec for NewSessionTicketPayloadTLS13 {
    fn encode(&self, bytes: &mut Vec<u8>) {
        self.lifetime.encode(bytes);
        self.age_add.encode(bytes);
        self.nonce.encode(bytes);
        self.ticket.encode(bytes);
        self.exts.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let lifetime = u32::read(r)?;
        let age_add = u32::read(r)?;
        let nonce = PayloadU8::read(r)?;
        let ticket = PayloadU16::read(r)?;
        let exts = Vec::read(r)?;

        Ok(Self {
            lifetime,
            age_add,
            nonce,
            ticket,
            exts,
        })
    }
}

// -- RFC6066 certificate status types

/// Only supports OCSP
#[derive(Debug)]
pub struct CertificateStatus {
    pub ocsp_response: PayloadU24,
}

impl Codec for CertificateStatus {
    fn encode(&self, bytes: &mut Vec<u8>) {
        CertificateStatusType::OCSP.encode(bytes);
        self.ocsp_response.encode(bytes);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        let typ = CertificateStatusType::read(r)?;

        match typ {
            CertificateStatusType::OCSP => Ok(Self {
                ocsp_response: PayloadU24::read(r)?,
            }),
            _ => Err(InvalidMessage::InvalidCertificateStatusType),
        }
    }
}

impl CertificateStatus {
    pub fn new(ocsp: Vec<u8>) -> Self {
        Self {
            ocsp_response: PayloadU24::new(ocsp),
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.ocsp_response.0
    }
}

#[derive(Debug)]
pub enum HandshakePayload {
    HelloRequest,
    ClientHello(ClientHelloPayload),
    ServerHello(ServerHelloPayload),
    HelloRetryRequest(HelloRetryRequest),
    Certificate(CertificatePayload),
    CertificateTLS13(CertificatePayloadTLS13),
    ServerKeyExchange(ServerKeyExchangePayload),
    CertificateRequest(CertificateRequestPayload),
    CertificateRequestTLS13(CertificateRequestPayloadTLS13),
    CertificateVerify(DigitallySignedStruct),
    ServerHelloDone,
    EndOfEarlyData,
    ClientKeyExchange(Payload),
    NewSessionTicket(NewSessionTicketPayload),
    NewSessionTicketTLS13(NewSessionTicketPayloadTLS13),
    EncryptedExtensions(Vec<ServerExtension>),
    KeyUpdate(KeyUpdateRequest),
    Finished(Payload),
    CertificateStatus(CertificateStatus),
    MessageHash(Payload),
    Unknown(Payload),
}

impl HandshakePayload {
    fn encode(&self, bytes: &mut Vec<u8>) {
        use self::HandshakePayload::*;
        match *self {
            HelloRequest | ServerHelloDone | EndOfEarlyData => {}
            ClientHello(ref x) => x.encode(bytes),
            ServerHello(ref x) => x.encode(bytes),
            HelloRetryRequest(ref x) => x.encode(bytes),
            Certificate(ref x) => x.encode(bytes),
            CertificateTLS13(ref x) => x.encode(bytes),
            ServerKeyExchange(ref x) => x.encode(bytes),
            ClientKeyExchange(ref x) => x.encode(bytes),
            CertificateRequest(ref x) => x.encode(bytes),
            CertificateRequestTLS13(ref x) => x.encode(bytes),
            CertificateVerify(ref x) => x.encode(bytes),
            NewSessionTicket(ref x) => x.encode(bytes),
            NewSessionTicketTLS13(ref x) => x.encode(bytes),
            EncryptedExtensions(ref x) => x.encode(bytes),
            KeyUpdate(ref x) => x.encode(bytes),
            Finished(ref x) => x.encode(bytes),
            CertificateStatus(ref x) => x.encode(bytes),
            MessageHash(ref x) => x.encode(bytes),
            Unknown(ref x) => x.encode(bytes),
        }
    }
}

#[derive(Debug)]
pub struct HandshakeMessagePayload {
    pub typ: HandshakeType,
    pub payload: HandshakePayload,
}

impl Codec for HandshakeMessagePayload {
    fn encode(&self, bytes: &mut Vec<u8>) {
        // encode payload to learn length
        let mut sub: Vec<u8> = Vec::new();
        self.payload.encode(&mut sub);

        // output type, length, and encoded payload
        match self.typ {
            HandshakeType::HelloRetryRequest => HandshakeType::ServerHello,
            _ => self.typ,
        }
        .encode(bytes);
        codec::u24(sub.len() as u32).encode(bytes);
        bytes.append(&mut sub);
    }

    fn read(r: &mut Reader) -> Result<Self, InvalidMessage> {
        Self::read_version(r, ProtocolVersion::TLSv1_2)
    }
}

impl HandshakeMessagePayload {
    pub fn read_version(r: &mut Reader, vers: ProtocolVersion) -> Result<Self, InvalidMessage> {
        let mut typ = HandshakeType::read(r)?;
        let len = codec::u24::read(r)?.0 as usize;
        let mut sub = r.sub(len)?;

        let payload = match typ {
            HandshakeType::HelloRequest if sub.left() == 0 => HandshakePayload::HelloRequest,
            HandshakeType::ClientHello => {
                HandshakePayload::ClientHello(ClientHelloPayload::read(&mut sub)?)
            }
            HandshakeType::ServerHello => {
                let version = ProtocolVersion::read(&mut sub)?;
                let random = Random::read(&mut sub)?;

                if random == HELLO_RETRY_REQUEST_RANDOM {
                    let mut hrr = HelloRetryRequest::read(&mut sub)?;
                    hrr.legacy_version = version;
                    typ = HandshakeType::HelloRetryRequest;
                    HandshakePayload::HelloRetryRequest(hrr)
                } else {
                    let mut shp = ServerHelloPayload::read(&mut sub)?;
                    shp.legacy_version = version;
                    shp.random = random;
                    HandshakePayload::ServerHello(shp)
                }
            }
            HandshakeType::Certificate if vers == ProtocolVersion::TLSv1_3 => {
                let p = CertificatePayloadTLS13::read(&mut sub)?;
                HandshakePayload::CertificateTLS13(p)
            }
            HandshakeType::Certificate => {
                HandshakePayload::Certificate(CertificatePayload::read(&mut sub)?)
            }
            HandshakeType::ServerKeyExchange => {
                let p = ServerKeyExchangePayload::read(&mut sub)?;
                HandshakePayload::ServerKeyExchange(p)
            }
            HandshakeType::ServerHelloDone => {
                sub.expect_empty("ServerHelloDone")?;
                HandshakePayload::ServerHelloDone
            }
            HandshakeType::ClientKeyExchange => {
                HandshakePayload::ClientKeyExchange(Payload::read(&mut sub))
            }
            HandshakeType::CertificateRequest if vers == ProtocolVersion::TLSv1_3 => {
                let p = CertificateRequestPayloadTLS13::read(&mut sub)?;
                HandshakePayload::CertificateRequestTLS13(p)
            }
            HandshakeType::CertificateRequest => {
                let p = CertificateRequestPayload::read(&mut sub)?;
                HandshakePayload::CertificateRequest(p)
            }
            HandshakeType::CertificateVerify => {
                HandshakePayload::CertificateVerify(DigitallySignedStruct::read(&mut sub)?)
            }
            HandshakeType::NewSessionTicket if vers == ProtocolVersion::TLSv1_3 => {
                let p = NewSessionTicketPayloadTLS13::read(&mut sub)?;
                HandshakePayload::NewSessionTicketTLS13(p)
            }
            HandshakeType::NewSessionTicket => {
                let p = NewSessionTicketPayload::read(&mut sub)?;
                HandshakePayload::NewSessionTicket(p)
            }
            HandshakeType::EncryptedExtensions => {
                HandshakePayload::EncryptedExtensions(Vec::read(&mut sub)?)
            }
            HandshakeType::KeyUpdate => {
                HandshakePayload::KeyUpdate(KeyUpdateRequest::read(&mut sub)?)
            }
            HandshakeType::EndOfEarlyData => {
                sub.expect_empty("EndOfEarlyData")?;
                HandshakePayload::EndOfEarlyData
            }
            HandshakeType::Finished => HandshakePayload::Finished(Payload::read(&mut sub)),
            HandshakeType::CertificateStatus => {
                HandshakePayload::CertificateStatus(CertificateStatus::read(&mut sub)?)
            }
            HandshakeType::MessageHash => {
                // does not appear on the wire
                return Err(InvalidMessage::UnexpectedMessage("MessageHash"));
            }
            HandshakeType::HelloRetryRequest => {
                // not legal on wire
                return Err(InvalidMessage::UnexpectedMessage("HelloRetryRequest"));
            }
            _ => HandshakePayload::Unknown(Payload::read(&mut sub)),
        };

        sub.expect_empty("HandshakeMessagePayload")
            .map(|_| Self { typ, payload })
    }

    pub fn build_key_update_notify() -> Self {
        Self {
            typ: HandshakeType::KeyUpdate,
            payload: HandshakePayload::KeyUpdate(KeyUpdateRequest::UpdateNotRequested),
        }
    }

    pub fn get_encoding_for_binder_signing(&self) -> Vec<u8> {
        let mut ret = self.get_encoding();

        let binder_len = match self.payload {
            HandshakePayload::ClientHello(ref ch) => match ch.extensions.last() {
                Some(ClientExtension::PresharedKey(ref offer)) => {
                    let mut binders_encoding = Vec::new();
                    offer
                        .binders
                        .encode(&mut binders_encoding);
                    binders_encoding.len()
                }
                _ => 0,
            },
            _ => 0,
        };

        let ret_len = ret.len() - binder_len;
        ret.truncate(ret_len);
        ret
    }

    pub fn build_handshake_hash(hash: &[u8]) -> Self {
        Self {
            typ: HandshakeType::MessageHash,
            payload: HandshakePayload::MessageHash(Payload::new(hash.to_vec())),
        }
    }
}
