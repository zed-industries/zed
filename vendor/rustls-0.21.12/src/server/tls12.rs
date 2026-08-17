use crate::check::inappropriate_message;
use crate::common_state::{CommonState, Side, State};
use crate::conn::ConnectionRandoms;
use crate::enums::ProtocolVersion;
use crate::enums::{AlertDescription, ContentType, HandshakeType};
use crate::error::{Error, PeerIncompatible, PeerMisbehaved};
use crate::hash_hs::HandshakeHash;
use crate::key::Certificate;
#[cfg(feature = "logging")]
use crate::log::{debug, trace};
use crate::msgs::base::Payload;
use crate::msgs::ccs::ChangeCipherSpecPayload;
use crate::msgs::codec::Codec;
use crate::msgs::handshake::{ClientECDHParams, HandshakeMessagePayload, HandshakePayload};
use crate::msgs::handshake::{NewSessionTicketPayload, SessionId};
use crate::msgs::message::{Message, MessagePayload};
use crate::msgs::persist;
#[cfg(feature = "secret_extraction")]
use crate::suites::PartiallyExtractedSecrets;
use crate::tls12::{self, ConnectionSecrets, Tls12CipherSuite};
use crate::{kx, ticketer, verify};

use super::common::ActiveCertifiedKey;
use super::hs::{self, ServerContext};
use super::server_conn::{ProducesTickets, ServerConfig, ServerConnectionData};

use ring::constant_time;

use std::sync::Arc;

pub(super) use client_hello::CompleteClientHelloHandling;

mod client_hello {
    use crate::enums::SignatureScheme;
    use crate::msgs::enums::ECPointFormat;
    use crate::msgs::enums::{ClientCertificateType, Compression};
    use crate::msgs::handshake::ClientExtension;
    use crate::msgs::handshake::ServerECDHParams;
    use crate::msgs::handshake::{CertificateRequestPayload, ClientSessionTicket, Random};
    use crate::msgs::handshake::{CertificateStatus, ECDHEServerKeyExchange};
    use crate::msgs::handshake::{ClientHelloPayload, ServerHelloPayload};
    use crate::msgs::handshake::{ServerExtension, ServerKeyExchangePayload};
    use crate::sign;
    use crate::verify::DigitallySignedStruct;

    use super::*;

    pub(in crate::server) struct CompleteClientHelloHandling {
        pub(in crate::server) config: Arc<ServerConfig>,
        pub(in crate::server) transcript: HandshakeHash,
        pub(in crate::server) session_id: SessionId,
        pub(in crate::server) suite: &'static Tls12CipherSuite,
        pub(in crate::server) using_ems: bool,
        pub(in crate::server) randoms: ConnectionRandoms,
        pub(in crate::server) send_ticket: bool,
        pub(in crate::server) extra_exts: Vec<ServerExtension>,
    }

    impl CompleteClientHelloHandling {
        pub(in crate::server) fn handle_client_hello(
            mut self,
            cx: &mut ServerContext<'_>,
            server_key: ActiveCertifiedKey,
            chm: &Message,
            client_hello: &ClientHelloPayload,
            sigschemes_ext: Vec<SignatureScheme>,
            tls13_enabled: bool,
        ) -> hs::NextStateOrError {
            // -- TLS1.2 only from hereon in --
            self.transcript.add_message(chm);

            if client_hello.ems_support_offered() {
                self.using_ems = true;
            }

            let groups_ext = client_hello
                .get_namedgroups_extension()
                .ok_or_else(|| {
                    cx.common.send_fatal_alert(
                        AlertDescription::HandshakeFailure,
                        PeerIncompatible::NamedGroupsExtensionRequired,
                    )
                })?;
            let ecpoints_ext = client_hello
                .get_ecpoints_extension()
                .ok_or_else(|| {
                    cx.common.send_fatal_alert(
                        AlertDescription::HandshakeFailure,
                        PeerIncompatible::EcPointsExtensionRequired,
                    )
                })?;

            trace!("namedgroups {:?}", groups_ext);
            trace!("ecpoints {:?}", ecpoints_ext);

            if !ecpoints_ext.contains(&ECPointFormat::Uncompressed) {
                return Err(cx.common.send_fatal_alert(
                    AlertDescription::IllegalParameter,
                    PeerIncompatible::UncompressedEcPointsRequired,
                ));
            }

            // -- If TLS1.3 is enabled, signal the downgrade in the server random
            if tls13_enabled {
                self.randoms.server[24..].copy_from_slice(&tls12::DOWNGRADE_SENTINEL);
            }

            // -- Check for resumption --
            // We can do this either by (in order of preference):
            // 1. receiving a ticket that decrypts
            // 2. receiving a sessionid that is in our cache
            //
            // If we receive a ticket, the sessionid won't be in our
            // cache, so don't check.
            //
            // If either works, we end up with a ServerConnectionValue
            // which is passed to start_resumption and concludes
            // our handling of the ClientHello.
            //
            let mut ticket_received = false;
            let resume_data = client_hello
                .get_ticket_extension()
                .and_then(|ticket_ext| match ticket_ext {
                    ClientExtension::SessionTicket(ClientSessionTicket::Offer(ticket)) => {
                        Some(ticket)
                    }
                    _ => None,
                })
                .and_then(|ticket| {
                    ticket_received = true;
                    debug!("Ticket received");
                    let data = self.config.ticketer.decrypt(&ticket.0);
                    if data.is_none() {
                        debug!("Ticket didn't decrypt");
                    }
                    data
                })
                .or_else(|| {
                    // Perhaps resume?  If we received a ticket, the sessionid
                    // does not correspond to a real session.
                    if client_hello.session_id.is_empty() || ticket_received {
                        return None;
                    }

                    self.config
                        .session_storage
                        .get(&client_hello.session_id.get_encoding())
                })
                .and_then(|x| persist::ServerSessionValue::read_bytes(&x).ok())
                .filter(|resumedata| {
                    hs::can_resume(self.suite.into(), &cx.data.sni, self.using_ems, resumedata)
                });

            if let Some(data) = resume_data {
                return self.start_resumption(cx, client_hello, &client_hello.session_id, data);
            }

            // Now we have chosen a ciphersuite, we can make kx decisions.
            let sigschemes = self
                .suite
                .resolve_sig_schemes(&sigschemes_ext);

            if sigschemes.is_empty() {
                return Err(cx.common.send_fatal_alert(
                    AlertDescription::HandshakeFailure,
                    PeerIncompatible::NoSignatureSchemesInCommon,
                ));
            }

            let group = self
                .config
                .kx_groups
                .iter()
                .find(|skxg| groups_ext.contains(&skxg.name))
                .cloned()
                .ok_or_else(|| {
                    cx.common.send_fatal_alert(
                        AlertDescription::HandshakeFailure,
                        PeerIncompatible::NoKxGroupsInCommon,
                    )
                })?;

            let ecpoint = ECPointFormat::SUPPORTED
                .iter()
                .find(|format| ecpoints_ext.contains(format))
                .cloned()
                .ok_or_else(|| {
                    cx.common.send_fatal_alert(
                        AlertDescription::HandshakeFailure,
                        PeerIncompatible::NoEcPointFormatsInCommon,
                    )
                })?;

            debug_assert_eq!(ecpoint, ECPointFormat::Uncompressed);

            let (mut ocsp_response, mut sct_list) =
                (server_key.get_ocsp(), server_key.get_sct_list());

            // If we're not offered a ticket or a potential session ID, allocate a session ID.
            if !self.config.session_storage.can_cache() {
                self.session_id = SessionId::empty();
            } else if self.session_id.is_empty() && !ticket_received {
                self.session_id = SessionId::random()?;
            }

            self.send_ticket = emit_server_hello(
                &self.config,
                &mut self.transcript,
                cx,
                self.session_id,
                self.suite,
                self.using_ems,
                &mut ocsp_response,
                &mut sct_list,
                client_hello,
                None,
                &self.randoms,
                self.extra_exts,
            )?;
            emit_certificate(&mut self.transcript, cx.common, server_key.get_cert());
            if let Some(ocsp_response) = ocsp_response {
                emit_cert_status(&mut self.transcript, cx.common, ocsp_response);
            }
            let server_kx = emit_server_kx(
                &mut self.transcript,
                cx.common,
                sigschemes,
                group,
                server_key.get_key(),
                &self.randoms,
            )?;
            let doing_client_auth = emit_certificate_req(&self.config, &mut self.transcript, cx)?;
            emit_server_hello_done(&mut self.transcript, cx.common);

            if doing_client_auth {
                Ok(Box::new(ExpectCertificate {
                    config: self.config,
                    transcript: self.transcript,
                    randoms: self.randoms,
                    session_id: self.session_id,
                    suite: self.suite,
                    using_ems: self.using_ems,
                    server_kx,
                    send_ticket: self.send_ticket,
                }))
            } else {
                Ok(Box::new(ExpectClientKx {
                    config: self.config,
                    transcript: self.transcript,
                    randoms: self.randoms,
                    session_id: self.session_id,
                    suite: self.suite,
                    using_ems: self.using_ems,
                    server_kx,
                    client_cert: None,
                    send_ticket: self.send_ticket,
                }))
            }
        }

        fn start_resumption(
            mut self,
            cx: &mut ServerContext<'_>,
            client_hello: &ClientHelloPayload,
            id: &SessionId,
            resumedata: persist::ServerSessionValue,
        ) -> hs::NextStateOrError {
            debug!("Resuming connection");

            if resumedata.extended_ms && !self.using_ems {
                return Err(cx.common.send_fatal_alert(
                    AlertDescription::IllegalParameter,
                    PeerMisbehaved::ResumptionAttemptedWithVariedEms,
                ));
            }

            self.session_id = *id;
            self.send_ticket = emit_server_hello(
                &self.config,
                &mut self.transcript,
                cx,
                self.session_id,
                self.suite,
                self.using_ems,
                &mut None,
                &mut None,
                client_hello,
                Some(&resumedata),
                &self.randoms,
                self.extra_exts,
            )?;

            let secrets = ConnectionSecrets::new_resume(
                self.randoms,
                self.suite,
                &resumedata.master_secret.0,
            );
            self.config.key_log.log(
                "CLIENT_RANDOM",
                &secrets.randoms.client,
                &secrets.master_secret,
            );
            cx.common
                .start_encryption_tls12(&secrets, Side::Server);
            cx.common.peer_certificates = resumedata.client_cert_chain;

            if self.send_ticket {
                emit_ticket(
                    &secrets,
                    &mut self.transcript,
                    self.using_ems,
                    cx,
                    &*self.config.ticketer,
                )?;
            }
            emit_ccs(cx.common);
            cx.common
                .record_layer
                .start_encrypting();
            emit_finished(&secrets, &mut self.transcript, cx.common);

            Ok(Box::new(ExpectCcs {
                config: self.config,
                secrets,
                transcript: self.transcript,
                session_id: self.session_id,
                using_ems: self.using_ems,
                resuming: true,
                send_ticket: self.send_ticket,
            }))
        }
    }

    fn emit_server_hello(
        config: &ServerConfig,
        transcript: &mut HandshakeHash,
        cx: &mut ServerContext<'_>,
        session_id: SessionId,
        suite: &'static Tls12CipherSuite,
        using_ems: bool,
        ocsp_response: &mut Option<&[u8]>,
        sct_list: &mut Option<&[u8]>,
        hello: &ClientHelloPayload,
        resumedata: Option<&persist::ServerSessionValue>,
        randoms: &ConnectionRandoms,
        extra_exts: Vec<ServerExtension>,
    ) -> Result<bool, Error> {
        let mut ep = hs::ExtensionProcessing::new();
        ep.process_common(
            config,
            cx,
            ocsp_response,
            sct_list,
            hello,
            resumedata,
            extra_exts,
        )?;
        ep.process_tls12(config, hello, using_ems);

        let sh = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::ServerHello,
                payload: HandshakePayload::ServerHello(ServerHelloPayload {
                    legacy_version: ProtocolVersion::TLSv1_2,
                    random: Random::from(randoms.server),
                    session_id,
                    cipher_suite: suite.common.suite,
                    compression_method: Compression::Null,
                    extensions: ep.exts,
                }),
            }),
        };

        trace!("sending server hello {:?}", sh);
        transcript.add_message(&sh);
        cx.common.send_msg(sh, false);
        Ok(ep.send_ticket)
    }

    fn emit_certificate(
        transcript: &mut HandshakeHash,
        common: &mut CommonState,
        cert_chain: &[Certificate],
    ) {
        let c = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::Certificate,
                payload: HandshakePayload::Certificate(cert_chain.to_owned()),
            }),
        };

        transcript.add_message(&c);
        common.send_msg(c, false);
    }

    fn emit_cert_status(transcript: &mut HandshakeHash, common: &mut CommonState, ocsp: &[u8]) {
        let st = CertificateStatus::new(ocsp.to_owned());

        let c = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::CertificateStatus,
                payload: HandshakePayload::CertificateStatus(st),
            }),
        };

        transcript.add_message(&c);
        common.send_msg(c, false);
    }

    fn emit_server_kx(
        transcript: &mut HandshakeHash,
        common: &mut CommonState,
        sigschemes: Vec<SignatureScheme>,
        skxg: &'static kx::SupportedKxGroup,
        signing_key: &dyn sign::SigningKey,
        randoms: &ConnectionRandoms,
    ) -> Result<kx::KeyExchange, Error> {
        let kx = kx::KeyExchange::start(skxg).ok_or(Error::FailedToGetRandomBytes)?;
        let secdh = ServerECDHParams::new(skxg.name, kx.pubkey.as_ref());

        let mut msg = Vec::new();
        msg.extend(randoms.client);
        msg.extend(randoms.server);
        secdh.encode(&mut msg);

        let signer = signing_key
            .choose_scheme(&sigschemes)
            .ok_or_else(|| Error::General("incompatible signing key".to_string()))?;
        let sigscheme = signer.scheme();
        let sig = signer.sign(&msg)?;

        let skx = ServerKeyExchangePayload::ECDHE(ECDHEServerKeyExchange {
            params: secdh,
            dss: DigitallySignedStruct::new(sigscheme, sig),
        });

        let m = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::ServerKeyExchange,
                payload: HandshakePayload::ServerKeyExchange(skx),
            }),
        };

        transcript.add_message(&m);
        common.send_msg(m, false);
        Ok(kx)
    }

    fn emit_certificate_req(
        config: &ServerConfig,
        transcript: &mut HandshakeHash,
        cx: &mut ServerContext<'_>,
    ) -> Result<bool, Error> {
        let client_auth = &config.verifier;

        if !client_auth.offer_client_auth() {
            return Ok(false);
        }

        let verify_schemes = client_auth.supported_verify_schemes();

        let names = config
            .verifier
            .client_auth_root_subjects()
            .to_vec();

        let cr = CertificateRequestPayload {
            certtypes: vec![
                ClientCertificateType::RSASign,
                ClientCertificateType::ECDSASign,
            ],
            sigschemes: verify_schemes,
            canames: names,
        };

        let m = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::CertificateRequest,
                payload: HandshakePayload::CertificateRequest(cr),
            }),
        };

        trace!("Sending CertificateRequest {:?}", m);
        transcript.add_message(&m);
        cx.common.send_msg(m, false);
        Ok(true)
    }

    fn emit_server_hello_done(transcript: &mut HandshakeHash, common: &mut CommonState) {
        let m = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::ServerHelloDone,
                payload: HandshakePayload::ServerHelloDone,
            }),
        };

        transcript.add_message(&m);
        common.send_msg(m, false);
    }
}

// --- Process client's Certificate for client auth ---
struct ExpectCertificate {
    config: Arc<ServerConfig>,
    transcript: HandshakeHash,
    randoms: ConnectionRandoms,
    session_id: SessionId,
    suite: &'static Tls12CipherSuite,
    using_ems: bool,
    server_kx: kx::KeyExchange,
    send_ticket: bool,
}

impl State<ServerConnectionData> for ExpectCertificate {
    fn handle(mut self: Box<Self>, cx: &mut ServerContext<'_>, m: Message) -> hs::NextStateOrError {
        self.transcript.add_message(&m);
        let cert_chain = require_handshake_msg_move!(
            m,
            HandshakeType::Certificate,
            HandshakePayload::Certificate
        )?;

        // If we can't determine if the auth is mandatory, abort
        let mandatory = self
            .config
            .verifier
            .client_auth_mandatory();

        trace!("certs {:?}", cert_chain);

        let client_cert = match cert_chain.split_first() {
            None if mandatory => {
                return Err(cx.common.send_fatal_alert(
                    AlertDescription::CertificateRequired,
                    Error::NoCertificatesPresented,
                ));
            }
            None => {
                debug!("client auth requested but no certificate supplied");
                self.transcript.abandon_client_auth();
                None
            }
            Some((end_entity, intermediates)) => {
                let now = std::time::SystemTime::now();
                self.config
                    .verifier
                    .verify_client_cert(end_entity, intermediates, now)
                    .map_err(|err| {
                        cx.common
                            .send_cert_verify_error_alert(err)
                    })?;

                Some(cert_chain)
            }
        };

        Ok(Box::new(ExpectClientKx {
            config: self.config,
            transcript: self.transcript,
            randoms: self.randoms,
            session_id: self.session_id,
            suite: self.suite,
            using_ems: self.using_ems,
            server_kx: self.server_kx,
            client_cert,
            send_ticket: self.send_ticket,
        }))
    }
}

// --- Process client's KeyExchange ---
struct ExpectClientKx {
    config: Arc<ServerConfig>,
    transcript: HandshakeHash,
    randoms: ConnectionRandoms,
    session_id: SessionId,
    suite: &'static Tls12CipherSuite,
    using_ems: bool,
    server_kx: kx::KeyExchange,
    client_cert: Option<Vec<Certificate>>,
    send_ticket: bool,
}

impl State<ServerConnectionData> for ExpectClientKx {
    fn handle(mut self: Box<Self>, cx: &mut ServerContext<'_>, m: Message) -> hs::NextStateOrError {
        let client_kx = require_handshake_msg!(
            m,
            HandshakeType::ClientKeyExchange,
            HandshakePayload::ClientKeyExchange
        )?;
        self.transcript.add_message(&m);
        let ems_seed = self
            .using_ems
            .then(|| self.transcript.get_current_hash());

        // Complete key agreement, and set up encryption with the
        // resulting premaster secret.
        let peer_kx_params =
            tls12::decode_ecdh_params::<ClientECDHParams>(cx.common, &client_kx.0)?;
        let secrets = ConnectionSecrets::from_key_exchange(
            self.server_kx,
            &peer_kx_params.public.0,
            ems_seed,
            self.randoms,
            self.suite,
        )?;

        self.config.key_log.log(
            "CLIENT_RANDOM",
            &secrets.randoms.client,
            &secrets.master_secret,
        );
        cx.common
            .start_encryption_tls12(&secrets, Side::Server);

        if let Some(client_cert) = self.client_cert {
            Ok(Box::new(ExpectCertificateVerify {
                config: self.config,
                secrets,
                transcript: self.transcript,
                session_id: self.session_id,
                using_ems: self.using_ems,
                client_cert,
                send_ticket: self.send_ticket,
            }))
        } else {
            Ok(Box::new(ExpectCcs {
                config: self.config,
                secrets,
                transcript: self.transcript,
                session_id: self.session_id,
                using_ems: self.using_ems,
                resuming: false,
                send_ticket: self.send_ticket,
            }))
        }
    }
}

// --- Process client's certificate proof ---
struct ExpectCertificateVerify {
    config: Arc<ServerConfig>,
    secrets: ConnectionSecrets,
    transcript: HandshakeHash,
    session_id: SessionId,
    using_ems: bool,
    client_cert: Vec<Certificate>,
    send_ticket: bool,
}

impl State<ServerConnectionData> for ExpectCertificateVerify {
    fn handle(mut self: Box<Self>, cx: &mut ServerContext<'_>, m: Message) -> hs::NextStateOrError {
        let rc = {
            let sig = require_handshake_msg!(
                m,
                HandshakeType::CertificateVerify,
                HandshakePayload::CertificateVerify
            )?;

            match self.transcript.take_handshake_buf() {
                Some(msgs) => {
                    let certs = &self.client_cert;
                    self.config
                        .verifier
                        .verify_tls12_signature(&msgs, &certs[0], sig)
                }
                None => {
                    // This should be unreachable; the handshake buffer was initialized with
                    // client authentication if the verifier wants to offer it.
                    // `transcript.abandon_client_auth()` can extract it, but its only caller in
                    // this flow will also set `ExpectClientKx::client_cert` to `None`, making it
                    // impossible to reach this state.
                    return Err(cx.common.send_fatal_alert(
                        AlertDescription::AccessDenied,
                        Error::General("client authentication not set up".into()),
                    ));
                }
            }
        };

        if let Err(e) = rc {
            return Err(cx
                .common
                .send_cert_verify_error_alert(e));
        }

        trace!("client CertificateVerify OK");
        cx.common.peer_certificates = Some(self.client_cert);

        self.transcript.add_message(&m);
        Ok(Box::new(ExpectCcs {
            config: self.config,
            secrets: self.secrets,
            transcript: self.transcript,
            session_id: self.session_id,
            using_ems: self.using_ems,
            resuming: false,
            send_ticket: self.send_ticket,
        }))
    }
}

// --- Process client's ChangeCipherSpec ---
struct ExpectCcs {
    config: Arc<ServerConfig>,
    secrets: ConnectionSecrets,
    transcript: HandshakeHash,
    session_id: SessionId,
    using_ems: bool,
    resuming: bool,
    send_ticket: bool,
}

impl State<ServerConnectionData> for ExpectCcs {
    fn handle(self: Box<Self>, cx: &mut ServerContext<'_>, m: Message) -> hs::NextStateOrError {
        match m.payload {
            MessagePayload::ChangeCipherSpec(..) => {}
            payload => {
                return Err(inappropriate_message(
                    &payload,
                    &[ContentType::ChangeCipherSpec],
                ))
            }
        }

        // CCS should not be received interleaved with fragmented handshake-level
        // message.
        cx.common.check_aligned_handshake()?;

        cx.common
            .record_layer
            .start_decrypting();
        Ok(Box::new(ExpectFinished {
            config: self.config,
            secrets: self.secrets,
            transcript: self.transcript,
            session_id: self.session_id,
            using_ems: self.using_ems,
            resuming: self.resuming,
            send_ticket: self.send_ticket,
        }))
    }
}

// --- Process client's Finished ---
fn get_server_connection_value_tls12(
    secrets: &ConnectionSecrets,
    using_ems: bool,
    cx: &ServerContext<'_>,
    time_now: ticketer::TimeBase,
) -> persist::ServerSessionValue {
    let version = ProtocolVersion::TLSv1_2;
    let secret = secrets.get_master_secret();

    let mut v = persist::ServerSessionValue::new(
        cx.data.sni.as_ref(),
        version,
        secrets.suite().common.suite,
        secret,
        cx.common.peer_certificates.clone(),
        cx.common.alpn_protocol.clone(),
        cx.data.resumption_data.clone(),
        time_now,
        0,
    );

    if using_ems {
        v.set_extended_ms_used();
    }

    v
}

fn emit_ticket(
    secrets: &ConnectionSecrets,
    transcript: &mut HandshakeHash,
    using_ems: bool,
    cx: &mut ServerContext<'_>,
    ticketer: &dyn ProducesTickets,
) -> Result<(), Error> {
    let time_now = ticketer::TimeBase::now()?;
    let plain = get_server_connection_value_tls12(secrets, using_ems, cx, time_now).get_encoding();

    // If we can't produce a ticket for some reason, we can't
    // report an error. Send an empty one.
    let ticket = ticketer
        .encrypt(&plain)
        .unwrap_or_default();
    let ticket_lifetime = ticketer.lifetime();

    let m = Message {
        version: ProtocolVersion::TLSv1_2,
        payload: MessagePayload::handshake(HandshakeMessagePayload {
            typ: HandshakeType::NewSessionTicket,
            payload: HandshakePayload::NewSessionTicket(NewSessionTicketPayload::new(
                ticket_lifetime,
                ticket,
            )),
        }),
    };

    transcript.add_message(&m);
    cx.common.send_msg(m, false);
    Ok(())
}

fn emit_ccs(common: &mut CommonState) {
    let m = Message {
        version: ProtocolVersion::TLSv1_2,
        payload: MessagePayload::ChangeCipherSpec(ChangeCipherSpecPayload {}),
    };

    common.send_msg(m, false);
}

fn emit_finished(
    secrets: &ConnectionSecrets,
    transcript: &mut HandshakeHash,
    common: &mut CommonState,
) {
    let vh = transcript.get_current_hash();
    let verify_data = secrets.server_verify_data(&vh);
    let verify_data_payload = Payload::new(verify_data);

    let f = Message {
        version: ProtocolVersion::TLSv1_2,
        payload: MessagePayload::handshake(HandshakeMessagePayload {
            typ: HandshakeType::Finished,
            payload: HandshakePayload::Finished(verify_data_payload),
        }),
    };

    transcript.add_message(&f);
    common.send_msg(f, true);
}

struct ExpectFinished {
    config: Arc<ServerConfig>,
    secrets: ConnectionSecrets,
    transcript: HandshakeHash,
    session_id: SessionId,
    using_ems: bool,
    resuming: bool,
    send_ticket: bool,
}

impl State<ServerConnectionData> for ExpectFinished {
    fn handle(mut self: Box<Self>, cx: &mut ServerContext<'_>, m: Message) -> hs::NextStateOrError {
        let finished =
            require_handshake_msg!(m, HandshakeType::Finished, HandshakePayload::Finished)?;

        cx.common.check_aligned_handshake()?;

        let vh = self.transcript.get_current_hash();
        let expect_verify_data = self.secrets.client_verify_data(&vh);

        let _fin_verified =
            constant_time::verify_slices_are_equal(&expect_verify_data, &finished.0)
                .map_err(|_| {
                    cx.common
                        .send_fatal_alert(AlertDescription::DecryptError, Error::DecryptError)
                })
                .map(|_| verify::FinishedMessageVerified::assertion())?;

        // Save connection, perhaps
        if !self.resuming && !self.session_id.is_empty() {
            let time_now = ticketer::TimeBase::now()?;
            let value =
                get_server_connection_value_tls12(&self.secrets, self.using_ems, cx, time_now);

            let worked = self
                .config
                .session_storage
                .put(self.session_id.get_encoding(), value.get_encoding());
            if worked {
                debug!("Session saved");
            } else {
                debug!("Session not saved");
            }
        }

        // Send our CCS and Finished.
        self.transcript.add_message(&m);
        if !self.resuming {
            if self.send_ticket {
                emit_ticket(
                    &self.secrets,
                    &mut self.transcript,
                    self.using_ems,
                    cx,
                    &*self.config.ticketer,
                )?;
            }
            emit_ccs(cx.common);
            cx.common
                .record_layer
                .start_encrypting();
            emit_finished(&self.secrets, &mut self.transcript, cx.common);
        }

        cx.common.start_traffic();
        Ok(Box::new(ExpectTraffic {
            secrets: self.secrets,
            _fin_verified,
        }))
    }
}

// --- Process traffic ---
struct ExpectTraffic {
    secrets: ConnectionSecrets,
    _fin_verified: verify::FinishedMessageVerified,
}

impl ExpectTraffic {}

impl State<ServerConnectionData> for ExpectTraffic {
    fn handle(self: Box<Self>, cx: &mut ServerContext<'_>, m: Message) -> hs::NextStateOrError {
        match m.payload {
            MessagePayload::ApplicationData(payload) => cx
                .common
                .take_received_plaintext(payload),
            payload => {
                return Err(inappropriate_message(
                    &payload,
                    &[ContentType::ApplicationData],
                ));
            }
        }
        Ok(self)
    }

    fn export_keying_material(
        &self,
        output: &mut [u8],
        label: &[u8],
        context: Option<&[u8]>,
    ) -> Result<(), Error> {
        self.secrets
            .export_keying_material(output, label, context);
        Ok(())
    }

    #[cfg(feature = "secret_extraction")]
    fn extract_secrets(&self) -> Result<PartiallyExtractedSecrets, Error> {
        self.secrets
            .extract_secrets(Side::Server)
    }
}
