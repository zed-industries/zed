#[cfg(feature = "logging")]
use crate::bs_debug;
use crate::check::inappropriate_handshake_message;
use crate::common_state::{CommonState, State};
use crate::conn::ConnectionRandoms;
use crate::enums::{AlertDescription, CipherSuite, ContentType, HandshakeType, ProtocolVersion};
use crate::error::{Error, PeerIncompatible, PeerMisbehaved};
use crate::hash_hs::HandshakeHashBuffer;
use crate::kx;
#[cfg(feature = "logging")]
use crate::log::{debug, trace};
use crate::msgs::base::Payload;
use crate::msgs::enums::{Compression, ExtensionType};
use crate::msgs::enums::{ECPointFormat, PSKKeyExchangeMode};
use crate::msgs::handshake::ConvertProtocolNameList;
use crate::msgs::handshake::{CertificateStatusRequest, ClientSessionTicket, Sct};
use crate::msgs::handshake::{ClientExtension, HasServerExtensions};
use crate::msgs::handshake::{ClientHelloPayload, HandshakeMessagePayload, HandshakePayload};
use crate::msgs::handshake::{HelloRetryRequest, KeyShareEntry};
use crate::msgs::handshake::{Random, SessionId};
use crate::msgs::message::{Message, MessagePayload};
use crate::msgs::persist;
use crate::ticketer::TimeBase;
use crate::tls13::key_schedule::KeyScheduleEarly;
use crate::SupportedCipherSuite;

#[cfg(feature = "tls12")]
use super::tls12;
use super::Tls12Resumption;
use crate::client::client_conn::ClientConnectionData;
use crate::client::common::ClientHelloDetails;
use crate::client::{tls13, ClientConfig, ServerName};

use std::ops::Deref;
use std::sync::Arc;

pub(super) type NextState = Box<dyn State<ClientConnectionData>>;
pub(super) type NextStateOrError = Result<NextState, Error>;
pub(super) type ClientContext<'a> = crate::common_state::Context<'a, ClientConnectionData>;

fn find_session(
    server_name: &ServerName,
    config: &ClientConfig,
    #[cfg(feature = "quic")] cx: &mut ClientContext<'_>,
) -> Option<persist::Retrieved<ClientSessionValue>> {
    #[allow(clippy::let_and_return, clippy::unnecessary_lazy_evaluations)]
    let found = config
        .resumption
        .store
        .take_tls13_ticket(server_name)
        .map(ClientSessionValue::Tls13)
        .or_else(|| {
            #[cfg(feature = "tls12")]
            {
                config
                    .resumption
                    .store
                    .tls12_session(server_name)
                    .map(ClientSessionValue::Tls12)
            }

            #[cfg(not(feature = "tls12"))]
            None
        })
        .and_then(|resuming| {
            let retrieved = persist::Retrieved::new(resuming, TimeBase::now().ok()?);
            match retrieved.has_expired() {
                false => Some(retrieved),
                true => None,
            }
        })
        .or_else(|| {
            debug!("No cached session for {:?}", server_name);
            None
        });

    #[cfg(feature = "quic")]
    if let Some(resuming) = &found {
        if cx.common.is_quic() {
            cx.common.quic.params = resuming
                .tls13()
                .map(|v| v.quic_params());
        }
    }

    found
}

pub(super) fn start_handshake(
    server_name: ServerName,
    extra_exts: Vec<ClientExtension>,
    config: Arc<ClientConfig>,
    cx: &mut ClientContext<'_>,
) -> NextStateOrError {
    let mut transcript_buffer = HandshakeHashBuffer::new();
    if config
        .client_auth_cert_resolver
        .has_certs()
    {
        transcript_buffer.set_client_auth_enabled();
    }

    let mut resuming = find_session(
        &server_name,
        &config,
        #[cfg(feature = "quic")]
        cx,
    );

    let key_share = if config.supports_version(ProtocolVersion::TLSv1_3) {
        Some(tls13::initial_key_share(&config, &server_name)?)
    } else {
        None
    };

    #[cfg_attr(not(feature = "tls12"), allow(unused_mut))]
    let mut session_id = None;
    if let Some(_resuming) = &mut resuming {
        #[cfg(feature = "tls12")]
        if let ClientSessionValue::Tls12(inner) = &mut _resuming.value {
            // If we have a ticket, we use the sessionid as a signal that
            // we're  doing an abbreviated handshake.  See section 3.4 in
            // RFC5077.
            if !inner.ticket().is_empty() {
                inner.session_id = SessionId::random()?;
            }
            session_id = Some(inner.session_id);
        }

        debug!("Resuming session");
    } else {
        debug!("Not resuming any session");
    }

    // https://tools.ietf.org/html/rfc8446#appendix-D.4
    // https://tools.ietf.org/html/draft-ietf-quic-tls-34#section-8.4
    let session_id = match session_id {
        Some(session_id) => session_id,
        None if cx.common.is_quic() => SessionId::empty(),
        None if !config.supports_version(ProtocolVersion::TLSv1_3) => SessionId::empty(),
        None => SessionId::random()?,
    };

    let may_send_sct_list = config.verifier.request_scts();
    Ok(emit_client_hello_for_retry(
        transcript_buffer,
        None,
        key_share,
        extra_exts,
        may_send_sct_list,
        None,
        ClientHelloInput {
            config,
            resuming,
            random: Random::new()?,
            #[cfg(feature = "tls12")]
            using_ems: false,
            sent_tls13_fake_ccs: false,
            hello: ClientHelloDetails::new(),
            session_id,
            server_name,
        },
        cx,
    ))
}

struct ExpectServerHello {
    input: ClientHelloInput,
    transcript_buffer: HandshakeHashBuffer,
    early_key_schedule: Option<KeyScheduleEarly>,
    offered_key_share: Option<kx::KeyExchange>,
    suite: Option<SupportedCipherSuite>,
}

struct ExpectServerHelloOrHelloRetryRequest {
    next: ExpectServerHello,
    extra_exts: Vec<ClientExtension>,
}

struct ClientHelloInput {
    config: Arc<ClientConfig>,
    resuming: Option<persist::Retrieved<ClientSessionValue>>,
    random: Random,
    #[cfg(feature = "tls12")]
    using_ems: bool,
    sent_tls13_fake_ccs: bool,
    hello: ClientHelloDetails,
    session_id: SessionId,
    server_name: ServerName,
}

fn emit_client_hello_for_retry(
    mut transcript_buffer: HandshakeHashBuffer,
    retryreq: Option<&HelloRetryRequest>,
    key_share: Option<kx::KeyExchange>,
    extra_exts: Vec<ClientExtension>,
    may_send_sct_list: bool,
    suite: Option<SupportedCipherSuite>,
    mut input: ClientHelloInput,
    cx: &mut ClientContext<'_>,
) -> NextState {
    let config = &input.config;
    let support_tls12 = config.supports_version(ProtocolVersion::TLSv1_2) && !cx.common.is_quic();
    let support_tls13 = config.supports_version(ProtocolVersion::TLSv1_3);

    let mut supported_versions = Vec::new();
    if support_tls13 {
        supported_versions.push(ProtocolVersion::TLSv1_3);
    }

    if support_tls12 {
        supported_versions.push(ProtocolVersion::TLSv1_2);
    }

    // should be unreachable thanks to config builder
    assert!(!supported_versions.is_empty());

    let mut exts = vec![
        ClientExtension::SupportedVersions(supported_versions),
        ClientExtension::ECPointFormats(ECPointFormat::SUPPORTED.to_vec()),
        ClientExtension::NamedGroups(
            config
                .kx_groups
                .iter()
                .map(|skxg| skxg.name)
                .collect(),
        ),
        ClientExtension::SignatureAlgorithms(
            config
                .verifier
                .supported_verify_schemes(),
        ),
        ClientExtension::ExtendedMasterSecretRequest,
        ClientExtension::CertificateStatusRequest(CertificateStatusRequest::build_ocsp()),
    ];

    if let (Some(sni_name), true) = (input.server_name.for_sni(), config.enable_sni) {
        exts.push(ClientExtension::make_sni(sni_name));
    }

    if may_send_sct_list {
        exts.push(ClientExtension::SignedCertificateTimestampRequest);
    }

    if let Some(key_share) = &key_share {
        debug_assert!(support_tls13);
        let key_share = KeyShareEntry::new(key_share.group(), key_share.pubkey.as_ref());
        exts.push(ClientExtension::KeyShare(vec![key_share]));
    }

    if let Some(cookie) = retryreq.and_then(HelloRetryRequest::get_cookie) {
        exts.push(ClientExtension::Cookie(cookie.clone()));
    }

    if support_tls13 {
        // We could support PSK_KE here too. Such connections don't
        // have forward secrecy, and are similar to TLS1.2 resumption.
        let psk_modes = vec![PSKKeyExchangeMode::PSK_DHE_KE];
        exts.push(ClientExtension::PresharedKeyModes(psk_modes));
    }

    if !config.alpn_protocols.is_empty() {
        exts.push(ClientExtension::Protocols(Vec::from_slices(
            &config
                .alpn_protocols
                .iter()
                .map(|proto| &proto[..])
                .collect::<Vec<_>>(),
        )));
    }

    // Extra extensions must be placed before the PSK extension
    exts.extend(extra_exts.iter().cloned());

    // Do we have a SessionID or ticket cached for this host?
    let tls13_session = prepare_resumption(&input.resuming, &mut exts, suite, cx, config);

    // Note what extensions we sent.
    input.hello.sent_extensions = exts
        .iter()
        .map(ClientExtension::get_type)
        .collect();

    let mut cipher_suites: Vec<_> = config
        .cipher_suites
        .iter()
        .map(|cs| cs.suite())
        .collect();
    // We don't do renegotiation at all, in fact.
    cipher_suites.push(CipherSuite::TLS_EMPTY_RENEGOTIATION_INFO_SCSV);

    let mut chp = HandshakeMessagePayload {
        typ: HandshakeType::ClientHello,
        payload: HandshakePayload::ClientHello(ClientHelloPayload {
            client_version: ProtocolVersion::TLSv1_2,
            random: input.random,
            session_id: input.session_id,
            cipher_suites,
            compression_methods: vec![Compression::Null],
            extensions: exts,
        }),
    };

    let early_key_schedule = if let Some(resuming) = tls13_session {
        let schedule = tls13::fill_in_psk_binder(&resuming, &transcript_buffer, &mut chp);
        Some((resuming.suite(), schedule))
    } else {
        None
    };

    let ch = Message {
        // "This value MUST be set to 0x0303 for all records generated
        //  by a TLS 1.3 implementation other than an initial ClientHello
        //  (i.e., one not generated after a HelloRetryRequest)"
        version: if retryreq.is_some() {
            ProtocolVersion::TLSv1_2
        } else {
            ProtocolVersion::TLSv1_0
        },
        payload: MessagePayload::handshake(chp),
    };

    if retryreq.is_some() {
        // send dummy CCS to fool middleboxes prior
        // to second client hello
        tls13::emit_fake_ccs(&mut input.sent_tls13_fake_ccs, cx.common);
    }

    trace!("Sending ClientHello {:#?}", ch);

    transcript_buffer.add_message(&ch);
    cx.common.send_msg(ch, false);

    // Calculate the hash of ClientHello and use it to derive EarlyTrafficSecret
    let early_key_schedule = early_key_schedule.map(|(resuming_suite, schedule)| {
        if !cx.data.early_data.is_enabled() {
            return schedule;
        }

        tls13::derive_early_traffic_secret(
            &*config.key_log,
            cx,
            resuming_suite,
            &schedule,
            &mut input.sent_tls13_fake_ccs,
            &transcript_buffer,
            &input.random.0,
        );
        schedule
    });

    let next = ExpectServerHello {
        input,
        transcript_buffer,
        early_key_schedule,
        offered_key_share: key_share,
        suite,
    };

    if support_tls13 && retryreq.is_none() {
        Box::new(ExpectServerHelloOrHelloRetryRequest { next, extra_exts })
    } else {
        Box::new(next)
    }
}

/// Prepare resumption with the session state retrieved from storage.
///
/// This function will push onto `exts` to
///
/// (a) request a new ticket if we don't have one,
/// (b) send our TLS 1.2 ticket after retrieving an 1.2 session,
/// (c) send a request for 1.3 early data if allowed and
/// (d) send a 1.3 preshared key if we have one.
///
/// For resumption to work, the currently negotiated cipher suite (if available) must be
/// able to resume from the resuming session's cipher suite.
///
/// If 1.3 resumption can continue, returns the 1.3 session value for further processing.
fn prepare_resumption<'a>(
    resuming: &'a Option<persist::Retrieved<ClientSessionValue>>,
    exts: &mut Vec<ClientExtension>,
    suite: Option<SupportedCipherSuite>,
    cx: &mut ClientContext<'_>,
    config: &ClientConfig,
) -> Option<persist::Retrieved<&'a persist::Tls13ClientSessionValue>> {
    // Check whether we're resuming with a non-empty ticket.
    let resuming = match resuming {
        Some(resuming) if !resuming.ticket().is_empty() => resuming,
        _ => {
            if config.supports_version(ProtocolVersion::TLSv1_3)
                || config.resumption.tls12_resumption == Tls12Resumption::SessionIdOrTickets
            {
                // If we don't have a ticket, request one.
                exts.push(ClientExtension::SessionTicket(ClientSessionTicket::Request));
            }
            return None;
        }
    };

    let tls13 = match resuming.map(|csv| csv.tls13()) {
        Some(tls13) => tls13,
        None => {
            // TLS 1.2; send the ticket if we have support this protocol version
            if config.supports_version(ProtocolVersion::TLSv1_2)
                && config.resumption.tls12_resumption == Tls12Resumption::SessionIdOrTickets
            {
                exts.push(ClientExtension::SessionTicket(ClientSessionTicket::Offer(
                    Payload::new(resuming.ticket()),
                )));
            }
            return None; // TLS 1.2, so nothing to return here
        }
    };

    if !config.supports_version(ProtocolVersion::TLSv1_3) {
        return None;
    }

    // If the server selected TLS 1.2, we can't resume.
    let suite = match suite {
        Some(SupportedCipherSuite::Tls13(suite)) => Some(suite),
        #[cfg(feature = "tls12")]
        Some(SupportedCipherSuite::Tls12(_)) => return None,
        None => None,
    };

    // If the selected cipher suite can't select from the session's, we can't resume.
    if let Some(suite) = suite {
        suite.can_resume_from(tls13.suite())?;
    }

    tls13::prepare_resumption(config, cx, &tls13, exts, suite.is_some());
    Some(tls13)
}

pub(super) fn process_alpn_protocol(
    common: &mut CommonState,
    config: &ClientConfig,
    proto: Option<&[u8]>,
) -> Result<(), Error> {
    common.alpn_protocol = proto.map(ToOwned::to_owned);

    if let Some(alpn_protocol) = &common.alpn_protocol {
        if !config
            .alpn_protocols
            .contains(alpn_protocol)
        {
            return Err(common.send_fatal_alert(
                AlertDescription::IllegalParameter,
                PeerMisbehaved::SelectedUnofferedApplicationProtocol,
            ));
        }
    }

    #[cfg(feature = "quic")]
    {
        // RFC 9001 says: "While ALPN only specifies that servers use this alert, QUIC clients MUST
        // use error 0x0178 to terminate a connection when ALPN negotiation fails." We judge that
        // the user intended to use ALPN (rather than some out-of-band protocol negotiation
        // mechanism) iff any ALPN protocols were configured. This defends against badly-behaved
        // servers which accept a connection that requires an application-layer protocol they do not
        // understand.
        if common.is_quic() && common.alpn_protocol.is_none() && !config.alpn_protocols.is_empty() {
            return Err(common.send_fatal_alert(
                AlertDescription::NoApplicationProtocol,
                Error::NoApplicationProtocol,
            ));
        }
    }

    debug!(
        "ALPN protocol is {:?}",
        common
            .alpn_protocol
            .as_ref()
            .map(|v| bs_debug::BsDebug(v))
    );
    Ok(())
}

pub(super) fn sct_list_is_invalid(scts: &[Sct]) -> bool {
    scts.is_empty()
        || scts
            .iter()
            .any(|sct| sct.as_ref().is_empty())
}

impl State<ClientConnectionData> for ExpectServerHello {
    fn handle(mut self: Box<Self>, cx: &mut ClientContext<'_>, m: Message) -> NextStateOrError {
        let server_hello =
            require_handshake_msg!(m, HandshakeType::ServerHello, HandshakePayload::ServerHello)?;
        trace!("We got ServerHello {:#?}", server_hello);

        use crate::ProtocolVersion::{TLSv1_2, TLSv1_3};
        let config = &self.input.config;
        let tls13_supported = config.supports_version(TLSv1_3);

        let server_version = if server_hello.legacy_version == TLSv1_2 {
            server_hello
                .get_supported_versions()
                .unwrap_or(server_hello.legacy_version)
        } else {
            server_hello.legacy_version
        };

        let version = match server_version {
            TLSv1_3 if tls13_supported => TLSv1_3,
            TLSv1_2 if config.supports_version(TLSv1_2) => {
                if cx.data.early_data.is_enabled() && cx.common.early_traffic {
                    // The client must fail with a dedicated error code if the server
                    // responds with TLS 1.2 when offering 0-RTT.
                    return Err(PeerMisbehaved::OfferedEarlyDataWithOldProtocolVersion.into());
                }

                if server_hello
                    .get_supported_versions()
                    .is_some()
                {
                    return Err({
                        cx.common.send_fatal_alert(
                            AlertDescription::IllegalParameter,
                            PeerMisbehaved::SelectedTls12UsingTls13VersionExtension,
                        )
                    });
                }

                TLSv1_2
            }
            _ => {
                let reason = match server_version {
                    TLSv1_2 | TLSv1_3 => PeerIncompatible::ServerTlsVersionIsDisabledByOurConfig,
                    _ => PeerIncompatible::ServerDoesNotSupportTls12Or13,
                };
                return Err(cx
                    .common
                    .send_fatal_alert(AlertDescription::ProtocolVersion, reason));
            }
        };

        if server_hello.compression_method != Compression::Null {
            return Err({
                cx.common.send_fatal_alert(
                    AlertDescription::IllegalParameter,
                    PeerMisbehaved::SelectedUnofferedCompression,
                )
            });
        }

        if server_hello.has_duplicate_extension() {
            return Err(cx.common.send_fatal_alert(
                AlertDescription::DecodeError,
                PeerMisbehaved::DuplicateServerHelloExtensions,
            ));
        }

        let allowed_unsolicited = [ExtensionType::RenegotiationInfo];
        if self
            .input
            .hello
            .server_sent_unsolicited_extensions(&server_hello.extensions, &allowed_unsolicited)
        {
            return Err(cx.common.send_fatal_alert(
                AlertDescription::UnsupportedExtension,
                PeerMisbehaved::UnsolicitedServerHelloExtension,
            ));
        }

        cx.common.negotiated_version = Some(version);

        // Extract ALPN protocol
        if !cx.common.is_tls13() {
            process_alpn_protocol(cx.common, config, server_hello.get_alpn_protocol())?;
        }

        // If ECPointFormats extension is supplied by the server, it must contain
        // Uncompressed.  But it's allowed to be omitted.
        if let Some(point_fmts) = server_hello.get_ecpoints_extension() {
            if !point_fmts.contains(&ECPointFormat::Uncompressed) {
                return Err(cx.common.send_fatal_alert(
                    AlertDescription::HandshakeFailure,
                    PeerMisbehaved::ServerHelloMustOfferUncompressedEcPoints,
                ));
            }
        }

        let suite = config
            .find_cipher_suite(server_hello.cipher_suite)
            .ok_or_else(|| {
                cx.common.send_fatal_alert(
                    AlertDescription::HandshakeFailure,
                    PeerMisbehaved::SelectedUnofferedCipherSuite,
                )
            })?;

        if version != suite.version().version {
            return Err({
                cx.common.send_fatal_alert(
                    AlertDescription::IllegalParameter,
                    PeerMisbehaved::SelectedUnusableCipherSuiteForVersion,
                )
            });
        }

        match self.suite {
            Some(prev_suite) if prev_suite != suite => {
                return Err({
                    cx.common.send_fatal_alert(
                        AlertDescription::IllegalParameter,
                        PeerMisbehaved::SelectedDifferentCipherSuiteAfterRetry,
                    )
                });
            }
            _ => {
                debug!("Using ciphersuite {:?}", suite);
                self.suite = Some(suite);
                cx.common.suite = Some(suite);
            }
        }

        // Start our handshake hash, and input the server-hello.
        let mut transcript = self
            .transcript_buffer
            .start_hash(suite.hash_algorithm());
        transcript.add_message(&m);

        let randoms = ConnectionRandoms::new(self.input.random, server_hello.random);
        // For TLS1.3, start message encryption using
        // handshake_traffic_secret.
        match suite {
            SupportedCipherSuite::Tls13(suite) => {
                #[allow(clippy::bind_instead_of_map)]
                let resuming_session = self
                    .input
                    .resuming
                    .and_then(|resuming| match resuming.value {
                        ClientSessionValue::Tls13(inner) => Some(inner),
                        #[cfg(feature = "tls12")]
                        ClientSessionValue::Tls12(_) => None,
                    });

                tls13::handle_server_hello(
                    self.input.config,
                    cx,
                    server_hello,
                    resuming_session,
                    self.input.server_name,
                    randoms,
                    suite,
                    transcript,
                    self.early_key_schedule,
                    self.input.hello,
                    // We always send a key share when TLS 1.3 is enabled.
                    self.offered_key_share.unwrap(),
                    self.input.sent_tls13_fake_ccs,
                )
            }
            #[cfg(feature = "tls12")]
            SupportedCipherSuite::Tls12(suite) => {
                let resuming_session = self
                    .input
                    .resuming
                    .and_then(|resuming| match resuming.value {
                        ClientSessionValue::Tls12(inner) => Some(inner),
                        ClientSessionValue::Tls13(_) => None,
                    });

                tls12::CompleteServerHelloHandling {
                    config: self.input.config,
                    resuming_session,
                    server_name: self.input.server_name,
                    randoms,
                    using_ems: self.input.using_ems,
                    transcript,
                }
                .handle_server_hello(cx, suite, server_hello, tls13_supported)
            }
        }
    }
}

impl ExpectServerHelloOrHelloRetryRequest {
    fn into_expect_server_hello(self) -> NextState {
        Box::new(self.next)
    }

    fn handle_hello_retry_request(
        self,
        cx: &mut ClientContext<'_>,
        m: Message,
    ) -> NextStateOrError {
        let hrr = require_handshake_msg!(
            m,
            HandshakeType::HelloRetryRequest,
            HandshakePayload::HelloRetryRequest
        )?;
        trace!("Got HRR {:?}", hrr);

        cx.common.check_aligned_handshake()?;

        let cookie = hrr.get_cookie();
        let req_group = hrr.get_requested_key_share_group();

        // We always send a key share when TLS 1.3 is enabled.
        let offered_key_share = self.next.offered_key_share.unwrap();

        // A retry request is illegal if it contains no cookie and asks for
        // retry of a group we already sent.
        if cookie.is_none() && req_group == Some(offered_key_share.group()) {
            return Err({
                cx.common.send_fatal_alert(
                    AlertDescription::IllegalParameter,
                    PeerMisbehaved::IllegalHelloRetryRequestWithOfferedGroup,
                )
            });
        }

        // Or has an empty cookie.
        if let Some(cookie) = cookie {
            if cookie.0.is_empty() {
                return Err({
                    cx.common.send_fatal_alert(
                        AlertDescription::IllegalParameter,
                        PeerMisbehaved::IllegalHelloRetryRequestWithEmptyCookie,
                    )
                });
            }
        }

        // Or has something unrecognised
        if hrr.has_unknown_extension() {
            return Err(cx.common.send_fatal_alert(
                AlertDescription::UnsupportedExtension,
                PeerIncompatible::ServerSentHelloRetryRequestWithUnknownExtension,
            ));
        }

        // Or has the same extensions more than once
        if hrr.has_duplicate_extension() {
            return Err({
                cx.common.send_fatal_alert(
                    AlertDescription::IllegalParameter,
                    PeerMisbehaved::DuplicateHelloRetryRequestExtensions,
                )
            });
        }

        // Or asks us to change nothing.
        if cookie.is_none() && req_group.is_none() {
            return Err({
                cx.common.send_fatal_alert(
                    AlertDescription::IllegalParameter,
                    PeerMisbehaved::IllegalHelloRetryRequestWithNoChanges,
                )
            });
        }

        // Or does not echo the session_id from our ClientHello:
        //
        // > the HelloRetryRequest has the same format as a ServerHello message,
        // > and the legacy_version, legacy_session_id_echo, cipher_suite, and
        // > legacy_compression_method fields have the same meaning
        // <https://www.rfc-editor.org/rfc/rfc8446#section-4.1.4>
        //
        // and
        //
        // > A client which receives a legacy_session_id_echo field that does not
        // > match what it sent in the ClientHello MUST abort the handshake with an
        // > "illegal_parameter" alert.
        // <https://www.rfc-editor.org/rfc/rfc8446#section-4.1.3>
        if hrr.session_id != self.next.input.session_id {
            return Err({
                cx.common.send_fatal_alert(
                    AlertDescription::IllegalParameter,
                    PeerMisbehaved::IllegalHelloRetryRequestWithWrongSessionId,
                )
            });
        }

        // Or asks us to talk a protocol we didn't offer, or doesn't support HRR at all.
        match hrr.get_supported_versions() {
            Some(ProtocolVersion::TLSv1_3) => {
                cx.common.negotiated_version = Some(ProtocolVersion::TLSv1_3);
            }
            _ => {
                return Err({
                    cx.common.send_fatal_alert(
                        AlertDescription::IllegalParameter,
                        PeerMisbehaved::IllegalHelloRetryRequestWithUnsupportedVersion,
                    )
                });
            }
        }

        // Or asks us to use a ciphersuite we didn't offer.
        let config = &self.next.input.config;
        let cs = match config.find_cipher_suite(hrr.cipher_suite) {
            Some(cs) => cs,
            None => {
                return Err({
                    cx.common.send_fatal_alert(
                        AlertDescription::IllegalParameter,
                        PeerMisbehaved::IllegalHelloRetryRequestWithUnofferedCipherSuite,
                    )
                });
            }
        };

        // HRR selects the ciphersuite.
        cx.common.suite = Some(cs);

        // This is the draft19 change where the transcript became a tree
        let transcript = self
            .next
            .transcript_buffer
            .start_hash(cs.hash_algorithm());
        let mut transcript_buffer = transcript.into_hrr_buffer();
        transcript_buffer.add_message(&m);

        // Early data is not allowed after HelloRetryrequest
        if cx.data.early_data.is_enabled() {
            cx.data.early_data.rejected();
        }

        let may_send_sct_list = self
            .next
            .input
            .hello
            .server_may_send_sct_list();

        let key_share = match req_group {
            Some(group) if group != offered_key_share.group() => {
                let group = kx::KeyExchange::choose(group, &config.kx_groups).ok_or_else(|| {
                    cx.common.send_fatal_alert(
                        AlertDescription::IllegalParameter,
                        PeerMisbehaved::IllegalHelloRetryRequestWithUnofferedNamedGroup,
                    )
                })?;
                kx::KeyExchange::start(group).ok_or(Error::FailedToGetRandomBytes)?
            }
            _ => offered_key_share,
        };

        Ok(emit_client_hello_for_retry(
            transcript_buffer,
            Some(hrr),
            Some(key_share),
            self.extra_exts,
            may_send_sct_list,
            Some(cs),
            self.next.input,
            cx,
        ))
    }
}

impl State<ClientConnectionData> for ExpectServerHelloOrHelloRetryRequest {
    fn handle(self: Box<Self>, cx: &mut ClientContext<'_>, m: Message) -> NextStateOrError {
        match m.payload {
            MessagePayload::Handshake {
                parsed:
                    HandshakeMessagePayload {
                        payload: HandshakePayload::ServerHello(..),
                        ..
                    },
                ..
            } => self
                .into_expect_server_hello()
                .handle(cx, m),
            MessagePayload::Handshake {
                parsed:
                    HandshakeMessagePayload {
                        payload: HandshakePayload::HelloRetryRequest(..),
                        ..
                    },
                ..
            } => self.handle_hello_retry_request(cx, m),
            payload => Err(inappropriate_handshake_message(
                &payload,
                &[ContentType::Handshake],
                &[HandshakeType::ServerHello, HandshakeType::HelloRetryRequest],
            )),
        }
    }
}

enum ClientSessionValue {
    Tls13(persist::Tls13ClientSessionValue),
    #[cfg(feature = "tls12")]
    Tls12(persist::Tls12ClientSessionValue),
}

impl ClientSessionValue {
    fn common(&self) -> &persist::ClientSessionCommon {
        match self {
            Self::Tls13(inner) => &inner.common,
            #[cfg(feature = "tls12")]
            Self::Tls12(inner) => &inner.common,
        }
    }

    fn tls13(&self) -> Option<&persist::Tls13ClientSessionValue> {
        match self {
            Self::Tls13(v) => Some(v),
            #[cfg(feature = "tls12")]
            Self::Tls12(_) => None,
        }
    }
}

impl Deref for ClientSessionValue {
    type Target = persist::ClientSessionCommon;

    fn deref(&self) -> &Self::Target {
        self.common()
    }
}
