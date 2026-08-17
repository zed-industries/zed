use crate::check::inappropriate_handshake_message;
#[cfg(feature = "quic")]
use crate::check::inappropriate_message;
#[cfg(feature = "quic")]
use crate::common_state::Protocol;
#[cfg(feature = "secret_extraction")]
use crate::common_state::Side;
use crate::common_state::{CommonState, State};
use crate::conn::ConnectionRandoms;
use crate::enums::ProtocolVersion;
use crate::enums::{AlertDescription, ContentType, HandshakeType};
use crate::error::{Error, PeerIncompatible, PeerMisbehaved};
use crate::hash_hs::HandshakeHash;
use crate::key::Certificate;
#[cfg(feature = "logging")]
use crate::log::{debug, trace, warn};
use crate::msgs::codec::Codec;
use crate::msgs::enums::KeyUpdateRequest;
use crate::msgs::handshake::HandshakeMessagePayload;
use crate::msgs::handshake::HandshakePayload;
use crate::msgs::handshake::{NewSessionTicketExtension, NewSessionTicketPayloadTLS13};
use crate::msgs::message::{Message, MessagePayload};
use crate::msgs::persist;
use crate::rand;
use crate::server::ServerConfig;
#[cfg(feature = "secret_extraction")]
use crate::suites::PartiallyExtractedSecrets;
use crate::ticketer;
use crate::tls13::key_schedule::{KeyScheduleTraffic, KeyScheduleTrafficWithClientFinishedPending};
use crate::tls13::Tls13CipherSuite;
use crate::verify;

use super::hs::{self, HandshakeHashOrBuffer, ServerContext};
use super::server_conn::ServerConnectionData;

use std::sync::Arc;

use ring::constant_time;

pub(super) use client_hello::CompleteClientHelloHandling;

mod client_hello {
    use crate::enums::SignatureScheme;
    use crate::kx;
    use crate::msgs::base::{Payload, PayloadU8};
    use crate::msgs::ccs::ChangeCipherSpecPayload;
    use crate::msgs::enums::NamedGroup;
    use crate::msgs::enums::{Compression, PSKKeyExchangeMode};
    use crate::msgs::handshake::CertReqExtension;
    use crate::msgs::handshake::CertificateEntry;
    use crate::msgs::handshake::CertificateExtension;
    use crate::msgs::handshake::CertificatePayloadTLS13;
    use crate::msgs::handshake::CertificateRequestPayloadTLS13;
    use crate::msgs::handshake::CertificateStatus;
    use crate::msgs::handshake::ClientHelloPayload;
    use crate::msgs::handshake::HelloRetryExtension;
    use crate::msgs::handshake::HelloRetryRequest;
    use crate::msgs::handshake::KeyShareEntry;
    use crate::msgs::handshake::Random;
    use crate::msgs::handshake::ServerExtension;
    use crate::msgs::handshake::ServerHelloPayload;
    use crate::msgs::handshake::SessionId;
    use crate::server::common::ActiveCertifiedKey;
    use crate::sign;
    use crate::tls13::key_schedule::{
        KeyScheduleEarly, KeyScheduleHandshake, KeySchedulePreHandshake,
    };
    use crate::verify::DigitallySignedStruct;

    use super::*;

    #[derive(PartialEq)]
    pub(super) enum EarlyDataDecision {
        Disabled,
        RequestedButRejected,
        Accepted,
    }

    pub(in crate::server) struct CompleteClientHelloHandling {
        pub(in crate::server) config: Arc<ServerConfig>,
        pub(in crate::server) transcript: HandshakeHash,
        pub(in crate::server) suite: &'static Tls13CipherSuite,
        pub(in crate::server) randoms: ConnectionRandoms,
        pub(in crate::server) done_retry: bool,
        pub(in crate::server) send_tickets: usize,
        pub(in crate::server) extra_exts: Vec<ServerExtension>,
    }

    fn max_early_data_size(configured: u32) -> usize {
        if configured != 0 {
            configured as usize
        } else {
            // The relevant max_early_data_size may in fact be unknowable: if
            // we (the server) have turned off early_data but the client has
            // a stale ticket from when we allowed early_data: we'll naturally
            // reject early_data but need an upper bound on the amount of data
            // to drop.
            //
            // Use a single maximum-sized message.
            16384
        }
    }

    impl CompleteClientHelloHandling {
        fn check_binder(
            &self,
            suite: &'static Tls13CipherSuite,
            client_hello: &Message,
            psk: &[u8],
            binder: &[u8],
        ) -> bool {
            let binder_plaintext = match &client_hello.payload {
                MessagePayload::Handshake { parsed, .. } => {
                    parsed.get_encoding_for_binder_signing()
                }
                _ => unreachable!(),
            };

            let handshake_hash = self
                .transcript
                .get_hash_given(&binder_plaintext);

            let key_schedule = KeyScheduleEarly::new(suite, psk);
            let real_binder =
                key_schedule.resumption_psk_binder_key_and_sign_verify_data(&handshake_hash);

            constant_time::verify_slices_are_equal(real_binder.as_ref(), binder).is_ok()
        }

        fn attempt_tls13_ticket_decryption(
            &mut self,
            ticket: &[u8],
        ) -> Option<persist::ServerSessionValue> {
            if self.config.ticketer.enabled() {
                self.config
                    .ticketer
                    .decrypt(ticket)
                    .and_then(|plain| persist::ServerSessionValue::read_bytes(&plain).ok())
            } else {
                self.config
                    .session_storage
                    .take(ticket)
                    .and_then(|plain| persist::ServerSessionValue::read_bytes(&plain).ok())
            }
        }

        pub(in crate::server) fn handle_client_hello(
            mut self,
            cx: &mut ServerContext<'_>,
            server_key: ActiveCertifiedKey,
            chm: &Message,
            client_hello: &ClientHelloPayload,
            mut sigschemes_ext: Vec<SignatureScheme>,
        ) -> hs::NextStateOrError {
            if client_hello.compression_methods.len() != 1 {
                return Err(cx.common.send_fatal_alert(
                    AlertDescription::IllegalParameter,
                    PeerMisbehaved::OfferedIncorrectCompressions,
                ));
            }

            let groups_ext = client_hello
                .get_namedgroups_extension()
                .ok_or_else(|| {
                    cx.common.send_fatal_alert(
                        AlertDescription::HandshakeFailure,
                        PeerIncompatible::NamedGroupsExtensionRequired,
                    )
                })?;

            let tls13_schemes = sign::supported_sign_tls13();
            sigschemes_ext.retain(|scheme| tls13_schemes.contains(scheme));

            let shares_ext = client_hello
                .get_keyshare_extension()
                .ok_or_else(|| {
                    cx.common.send_fatal_alert(
                        AlertDescription::HandshakeFailure,
                        PeerIncompatible::KeyShareExtensionRequired,
                    )
                })?;

            if client_hello.has_keyshare_extension_with_duplicates() {
                return Err(cx.common.send_fatal_alert(
                    AlertDescription::IllegalParameter,
                    PeerMisbehaved::OfferedDuplicateKeyShares,
                ));
            }

            let early_data_requested = client_hello.early_data_extension_offered();

            // EarlyData extension is illegal in second ClientHello
            if self.done_retry && early_data_requested {
                return Err({
                    cx.common.send_fatal_alert(
                        AlertDescription::IllegalParameter,
                        PeerMisbehaved::EarlyDataAttemptedInSecondClientHello,
                    )
                });
            }

            // choose a share that we support
            let chosen_share = self
                .config
                .kx_groups
                .iter()
                .find_map(|group| {
                    shares_ext
                        .iter()
                        .find(|share| share.group == group.name)
                });

            let chosen_share = match chosen_share {
                Some(s) => s,
                None => {
                    // We don't have a suitable key share.  Choose a suitable group and
                    // send a HelloRetryRequest.
                    let retry_group_maybe = self
                        .config
                        .kx_groups
                        .iter()
                        .find(|group| groups_ext.contains(&group.name))
                        .cloned();

                    self.transcript.add_message(chm);

                    if let Some(group) = retry_group_maybe {
                        if self.done_retry {
                            return Err(cx.common.send_fatal_alert(
                                AlertDescription::IllegalParameter,
                                PeerMisbehaved::RefusedToFollowHelloRetryRequest,
                            ));
                        }

                        emit_hello_retry_request(
                            &mut self.transcript,
                            self.suite,
                            client_hello.session_id,
                            cx.common,
                            group.name,
                        );
                        emit_fake_ccs(cx.common);

                        let skip_early_data = max_early_data_size(self.config.max_early_data_size);

                        let next = Box::new(hs::ExpectClientHello {
                            config: self.config,
                            transcript: HandshakeHashOrBuffer::Hash(self.transcript),
                            #[cfg(feature = "tls12")]
                            session_id: SessionId::empty(),
                            #[cfg(feature = "tls12")]
                            using_ems: false,
                            done_retry: true,
                            send_tickets: self.send_tickets,
                            extra_exts: self.extra_exts,
                        });

                        return if early_data_requested {
                            Ok(Box::new(ExpectAndSkipRejectedEarlyData {
                                skip_data_left: skip_early_data,
                                next,
                            }))
                        } else {
                            Ok(next)
                        };
                    }

                    return Err(cx.common.send_fatal_alert(
                        AlertDescription::HandshakeFailure,
                        PeerIncompatible::NoKxGroupsInCommon,
                    ));
                }
            };

            let mut chosen_psk_index = None;
            let mut resumedata = None;
            let time_now = ticketer::TimeBase::now()?;

            if let Some(psk_offer) = client_hello.get_psk() {
                if !client_hello.check_psk_ext_is_last() {
                    return Err(cx.common.send_fatal_alert(
                        AlertDescription::IllegalParameter,
                        PeerMisbehaved::PskExtensionMustBeLast,
                    ));
                }

                // "A client MUST provide a "psk_key_exchange_modes" extension if it
                //  offers a "pre_shared_key" extension. If clients offer
                //  "pre_shared_key" without a "psk_key_exchange_modes" extension,
                //  servers MUST abort the handshake." - RFC8446 4.2.9
                if client_hello.get_psk_modes().is_none() {
                    return Err(cx.common.send_fatal_alert(
                        AlertDescription::MissingExtension,
                        PeerMisbehaved::MissingPskModesExtension,
                    ));
                }

                if psk_offer.binders.is_empty() {
                    return Err(cx.common.send_fatal_alert(
                        AlertDescription::DecodeError,
                        PeerMisbehaved::MissingBinderInPskExtension,
                    ));
                }

                if psk_offer.binders.len() != psk_offer.identities.len() {
                    return Err(cx.common.send_fatal_alert(
                        AlertDescription::IllegalParameter,
                        PeerMisbehaved::PskExtensionWithMismatchedIdsAndBinders,
                    ));
                }

                for (i, psk_id) in psk_offer.identities.iter().enumerate() {
                    let resume = match self
                        .attempt_tls13_ticket_decryption(&psk_id.identity.0)
                        .map(|resumedata| {
                            resumedata.set_freshness(psk_id.obfuscated_ticket_age, time_now)
                        })
                        .filter(|resumedata| {
                            hs::can_resume(self.suite.into(), &cx.data.sni, false, resumedata)
                        }) {
                        Some(resume) => resume,
                        None => continue,
                    };

                    if !self.check_binder(
                        self.suite,
                        chm,
                        &resume.master_secret.0,
                        psk_offer.binders[i].as_ref(),
                    ) {
                        return Err(cx.common.send_fatal_alert(
                            AlertDescription::DecryptError,
                            PeerMisbehaved::IncorrectBinder,
                        ));
                    }

                    chosen_psk_index = Some(i);
                    resumedata = Some(resume);
                    break;
                }
            }

            if !client_hello.psk_mode_offered(PSKKeyExchangeMode::PSK_DHE_KE) {
                debug!("Client unwilling to resume, DHE_KE not offered");
                self.send_tickets = 0;
                chosen_psk_index = None;
                resumedata = None;
            } else {
                self.send_tickets = self.config.send_tls13_tickets;
            }

            if let Some(ref resume) = resumedata {
                cx.data.received_resumption_data = Some(resume.application_data.0.clone());
                cx.common
                    .peer_certificates
                    .clone_from(&resume.client_cert_chain);
            }

            let full_handshake = resumedata.is_none();
            self.transcript.add_message(chm);
            let key_schedule = emit_server_hello(
                &mut self.transcript,
                &self.randoms,
                self.suite,
                cx,
                &client_hello.session_id,
                chosen_share,
                chosen_psk_index,
                resumedata
                    .as_ref()
                    .map(|x| &x.master_secret.0[..]),
                &self.config,
            )?;
            if !self.done_retry {
                emit_fake_ccs(cx.common);
            }

            let (mut ocsp_response, mut sct_list) =
                (server_key.get_ocsp(), server_key.get_sct_list());
            let doing_early_data = emit_encrypted_extensions(
                &mut self.transcript,
                self.suite,
                cx,
                &mut ocsp_response,
                &mut sct_list,
                client_hello,
                resumedata.as_ref(),
                self.extra_exts,
                &self.config,
            )?;

            let doing_client_auth = if full_handshake {
                let client_auth =
                    emit_certificate_req_tls13(&mut self.transcript, cx, &self.config)?;
                emit_certificate_tls13(
                    &mut self.transcript,
                    cx.common,
                    server_key.get_cert(),
                    ocsp_response,
                    sct_list,
                );
                emit_certificate_verify_tls13(
                    &mut self.transcript,
                    cx.common,
                    server_key.get_key(),
                    &sigschemes_ext,
                )?;
                client_auth
            } else {
                false
            };

            // If we're not doing early data, then the next messages we receive
            // are encrypted with the handshake keys.
            match doing_early_data {
                EarlyDataDecision::Disabled => {
                    key_schedule.set_handshake_decrypter(None, cx.common);
                    cx.data.early_data.reject();
                }
                EarlyDataDecision::RequestedButRejected => {
                    debug!("Client requested early_data, but not accepted: switching to handshake keys with trial decryption");
                    key_schedule.set_handshake_decrypter(
                        Some(max_early_data_size(self.config.max_early_data_size)),
                        cx.common,
                    );
                    cx.data.early_data.reject();
                }
                EarlyDataDecision::Accepted => {
                    cx.data
                        .early_data
                        .accept(self.config.max_early_data_size as usize);
                }
            }

            cx.common.check_aligned_handshake()?;
            let key_schedule_traffic = emit_finished_tls13(
                &mut self.transcript,
                &self.randoms,
                cx,
                key_schedule,
                &self.config,
            );

            if !doing_client_auth && self.config.send_half_rtt_data {
                // Application data can be sent immediately after Finished, in one
                // flight.  However, if client auth is enabled, we don't want to send
                // application data to an unauthenticated peer.
                cx.common.start_outgoing_traffic();
            }

            if doing_client_auth {
                Ok(Box::new(ExpectCertificate {
                    config: self.config,
                    transcript: self.transcript,
                    suite: self.suite,
                    key_schedule: key_schedule_traffic,
                    send_tickets: self.send_tickets,
                }))
            } else if doing_early_data == EarlyDataDecision::Accepted && !cx.common.is_quic() {
                // Not used for QUIC: RFC 9001 §8.3: Clients MUST NOT send the EndOfEarlyData
                // message. A server MUST treat receipt of a CRYPTO frame in a 0-RTT packet as a
                // connection error of type PROTOCOL_VIOLATION.
                Ok(Box::new(ExpectEarlyData {
                    config: self.config,
                    transcript: self.transcript,
                    suite: self.suite,
                    key_schedule: key_schedule_traffic,
                    send_tickets: self.send_tickets,
                }))
            } else {
                Ok(Box::new(ExpectFinished {
                    config: self.config,
                    transcript: self.transcript,
                    suite: self.suite,
                    key_schedule: key_schedule_traffic,
                    send_tickets: self.send_tickets,
                }))
            }
        }
    }

    fn emit_server_hello(
        transcript: &mut HandshakeHash,
        randoms: &ConnectionRandoms,
        suite: &'static Tls13CipherSuite,
        cx: &mut ServerContext<'_>,
        session_id: &SessionId,
        share: &KeyShareEntry,
        chosen_psk_idx: Option<usize>,
        resuming_psk: Option<&[u8]>,
        config: &ServerConfig,
    ) -> Result<KeyScheduleHandshake, Error> {
        let mut extensions = Vec::new();

        // Prepare key exchange
        let kx = kx::KeyExchange::choose(share.group, &config.kx_groups)
            .and_then(kx::KeyExchange::start)
            .ok_or(Error::FailedToGetRandomBytes)?;

        let kse = KeyShareEntry::new(share.group, kx.pubkey.as_ref());
        extensions.push(ServerExtension::KeyShare(kse));
        extensions.push(ServerExtension::SupportedVersions(ProtocolVersion::TLSv1_3));

        if let Some(psk_idx) = chosen_psk_idx {
            extensions.push(ServerExtension::PresharedKey(psk_idx as u16));
        }

        let sh = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::ServerHello,
                payload: HandshakePayload::ServerHello(ServerHelloPayload {
                    legacy_version: ProtocolVersion::TLSv1_2,
                    random: Random::from(randoms.server),
                    session_id: *session_id,
                    cipher_suite: suite.common.suite,
                    compression_method: Compression::Null,
                    extensions,
                }),
            }),
        };

        cx.common.check_aligned_handshake()?;

        let client_hello_hash = transcript.get_hash_given(&[]);

        trace!("sending server hello {:?}", sh);
        transcript.add_message(&sh);
        cx.common.send_msg(sh, false);

        // Start key schedule
        let key_schedule_pre_handshake = if let Some(psk) = resuming_psk {
            let early_key_schedule = KeyScheduleEarly::new(suite, psk);
            early_key_schedule.client_early_traffic_secret(
                &client_hello_hash,
                &*config.key_log,
                &randoms.client,
                cx.common,
            );

            KeySchedulePreHandshake::from(early_key_schedule)
        } else {
            KeySchedulePreHandshake::new(suite)
        };

        // Do key exchange
        let key_schedule = kx.complete(&share.payload.0, |secret| {
            key_schedule_pre_handshake.into_handshake(secret)
        })?;

        let handshake_hash = transcript.get_current_hash();
        let key_schedule = key_schedule.derive_server_handshake_secrets(
            handshake_hash,
            &*config.key_log,
            &randoms.client,
            cx.common,
        );

        Ok(key_schedule)
    }

    fn emit_fake_ccs(common: &mut CommonState) {
        if common.is_quic() {
            return;
        }
        let m = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::ChangeCipherSpec(ChangeCipherSpecPayload {}),
        };
        common.send_msg(m, false);
    }

    fn emit_hello_retry_request(
        transcript: &mut HandshakeHash,
        suite: &'static Tls13CipherSuite,
        session_id: SessionId,
        common: &mut CommonState,
        group: NamedGroup,
    ) {
        let mut req = HelloRetryRequest {
            legacy_version: ProtocolVersion::TLSv1_2,
            session_id,
            cipher_suite: suite.common.suite,
            extensions: Vec::new(),
        };

        req.extensions
            .push(HelloRetryExtension::KeyShare(group));
        req.extensions
            .push(HelloRetryExtension::SupportedVersions(
                ProtocolVersion::TLSv1_3,
            ));

        let m = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::HelloRetryRequest,
                payload: HandshakePayload::HelloRetryRequest(req),
            }),
        };

        trace!("Requesting retry {:?}", m);
        transcript.rollup_for_hrr();
        transcript.add_message(&m);
        common.send_msg(m, false);
    }

    #[cfg_attr(not(feature = "quic"), allow(clippy::needless_pass_by_ref_mut))]
    fn decide_if_early_data_allowed(
        cx: &mut ServerContext<'_>,
        client_hello: &ClientHelloPayload,
        resumedata: Option<&persist::ServerSessionValue>,
        suite: &'static Tls13CipherSuite,
        config: &ServerConfig,
    ) -> EarlyDataDecision {
        let early_data_requested = client_hello.early_data_extension_offered();
        let rejected_or_disabled = match early_data_requested {
            true => EarlyDataDecision::RequestedButRejected,
            false => EarlyDataDecision::Disabled,
        };

        let resume = match resumedata {
            Some(resume) => resume,
            None => {
                // never any early data if not resuming.
                return rejected_or_disabled;
            }
        };

        /* Non-zero max_early_data_size controls whether early_data is allowed at all.
         * We also require stateful resumption. */
        let early_data_configured = config.max_early_data_size > 0 && !config.ticketer.enabled();

        /* "For PSKs provisioned via NewSessionTicket, a server MUST validate
         *  that the ticket age for the selected PSK identity (computed by
         *  subtracting ticket_age_add from PskIdentity.obfuscated_ticket_age
         *  modulo 2^32) is within a small tolerance of the time since the ticket
         *  was issued (see Section 8)." -- this is implemented in ServerSessionValue::set_freshness()
         *  and related.
         *
         * "In order to accept early data, the server [...] MUST verify that the
         *  following values are the same as those associated with the
         *  selected PSK:
         *
         *  - The TLS version number
         *  - The selected cipher suite
         *  - The selected ALPN [RFC7301] protocol, if any"
         *
         * (RFC8446, 4.2.10) */
        let early_data_possible = early_data_requested
            && resume.is_fresh()
            && Some(resume.version) == cx.common.negotiated_version
            && resume.cipher_suite == suite.common.suite
            && resume.alpn.as_ref().map(|x| &x.0) == cx.common.alpn_protocol.as_ref();

        if early_data_configured && early_data_possible && !cx.data.early_data.was_rejected() {
            EarlyDataDecision::Accepted
        } else {
            #[cfg(feature = "quic")]
            if cx.common.is_quic() {
                // Clobber value set in tls13::emit_server_hello
                cx.common.quic.early_secret = None;
            }

            rejected_or_disabled
        }
    }

    fn emit_encrypted_extensions(
        transcript: &mut HandshakeHash,
        suite: &'static Tls13CipherSuite,
        cx: &mut ServerContext<'_>,
        ocsp_response: &mut Option<&[u8]>,
        sct_list: &mut Option<&[u8]>,
        hello: &ClientHelloPayload,
        resumedata: Option<&persist::ServerSessionValue>,
        extra_exts: Vec<ServerExtension>,
        config: &ServerConfig,
    ) -> Result<EarlyDataDecision, Error> {
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

        let early_data = decide_if_early_data_allowed(cx, hello, resumedata, suite, config);
        if early_data == EarlyDataDecision::Accepted {
            ep.exts.push(ServerExtension::EarlyData);
        }

        let ee = Message {
            version: ProtocolVersion::TLSv1_3,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::EncryptedExtensions,
                payload: HandshakePayload::EncryptedExtensions(ep.exts),
            }),
        };

        trace!("sending encrypted extensions {:?}", ee);
        transcript.add_message(&ee);
        cx.common.send_msg(ee, true);
        Ok(early_data)
    }

    fn emit_certificate_req_tls13(
        transcript: &mut HandshakeHash,
        cx: &mut ServerContext<'_>,
        config: &ServerConfig,
    ) -> Result<bool, Error> {
        if !config.verifier.offer_client_auth() {
            return Ok(false);
        }

        let mut cr = CertificateRequestPayloadTLS13 {
            context: PayloadU8::empty(),
            extensions: Vec::new(),
        };

        let schemes = config
            .verifier
            .supported_verify_schemes();
        cr.extensions
            .push(CertReqExtension::SignatureAlgorithms(schemes.to_vec()));

        let names = config
            .verifier
            .client_auth_root_subjects()
            .to_vec();

        if !names.is_empty() {
            cr.extensions
                .push(CertReqExtension::AuthorityNames(names));
        }

        let m = Message {
            version: ProtocolVersion::TLSv1_3,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::CertificateRequest,
                payload: HandshakePayload::CertificateRequestTLS13(cr),
            }),
        };

        trace!("Sending CertificateRequest {:?}", m);
        transcript.add_message(&m);
        cx.common.send_msg(m, true);
        Ok(true)
    }

    fn emit_certificate_tls13(
        transcript: &mut HandshakeHash,
        common: &mut CommonState,
        cert_chain: &[Certificate],
        ocsp_response: Option<&[u8]>,
        sct_list: Option<&[u8]>,
    ) {
        let mut cert_entries = vec![];
        for cert in cert_chain {
            let entry = CertificateEntry {
                cert: cert.to_owned(),
                exts: Vec::new(),
            };

            cert_entries.push(entry);
        }

        if let Some(end_entity_cert) = cert_entries.first_mut() {
            // Apply OCSP response to first certificate (we don't support OCSP
            // except for leaf certs).
            if let Some(ocsp) = ocsp_response {
                let cst = CertificateStatus::new(ocsp.to_owned());
                end_entity_cert
                    .exts
                    .push(CertificateExtension::CertificateStatus(cst));
            }

            // Likewise, SCT
            if let Some(sct_list) = sct_list {
                end_entity_cert
                    .exts
                    .push(CertificateExtension::make_sct(sct_list.to_owned()));
            }
        }

        let cert_body = CertificatePayloadTLS13::new(cert_entries);
        let c = Message {
            version: ProtocolVersion::TLSv1_3,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::Certificate,
                payload: HandshakePayload::CertificateTLS13(cert_body),
            }),
        };

        trace!("sending certificate {:?}", c);
        transcript.add_message(&c);
        common.send_msg(c, true);
    }

    fn emit_certificate_verify_tls13(
        transcript: &mut HandshakeHash,
        common: &mut CommonState,
        signing_key: &dyn sign::SigningKey,
        schemes: &[SignatureScheme],
    ) -> Result<(), Error> {
        let message = verify::construct_tls13_server_verify_message(&transcript.get_current_hash());

        let signer = signing_key
            .choose_scheme(schemes)
            .ok_or_else(|| {
                common.send_fatal_alert(
                    AlertDescription::HandshakeFailure,
                    PeerIncompatible::NoSignatureSchemesInCommon,
                )
            })?;

        let scheme = signer.scheme();
        let sig = signer.sign(&message)?;

        let cv = DigitallySignedStruct::new(scheme, sig);

        let m = Message {
            version: ProtocolVersion::TLSv1_3,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::CertificateVerify,
                payload: HandshakePayload::CertificateVerify(cv),
            }),
        };

        trace!("sending certificate-verify {:?}", m);
        transcript.add_message(&m);
        common.send_msg(m, true);
        Ok(())
    }

    fn emit_finished_tls13(
        transcript: &mut HandshakeHash,
        randoms: &ConnectionRandoms,
        cx: &mut ServerContext<'_>,
        key_schedule: KeyScheduleHandshake,
        config: &ServerConfig,
    ) -> KeyScheduleTrafficWithClientFinishedPending {
        let handshake_hash = transcript.get_current_hash();
        let verify_data = key_schedule.sign_server_finish(&handshake_hash);
        let verify_data_payload = Payload::new(verify_data.as_ref());

        let m = Message {
            version: ProtocolVersion::TLSv1_3,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::Finished,
                payload: HandshakePayload::Finished(verify_data_payload),
            }),
        };

        trace!("sending finished {:?}", m);
        transcript.add_message(&m);
        let hash_at_server_fin = transcript.get_current_hash();
        cx.common.send_msg(m, true);

        // Now move to application data keys.  Read key change is deferred until
        // the Finish message is received & validated.
        key_schedule.into_traffic_with_client_finished_pending(
            hash_at_server_fin,
            &*config.key_log,
            &randoms.client,
            cx.common,
        )
    }
}

struct ExpectAndSkipRejectedEarlyData {
    skip_data_left: usize,
    next: Box<hs::ExpectClientHello>,
}

impl State<ServerConnectionData> for ExpectAndSkipRejectedEarlyData {
    fn handle(mut self: Box<Self>, cx: &mut ServerContext<'_>, m: Message) -> hs::NextStateOrError {
        /* "The server then ignores early data by skipping all records with an external
         *  content type of "application_data" (indicating that they are encrypted),
         *  up to the configured max_early_data_size."
         * (RFC8446, 14.2.10) */
        if let MessagePayload::ApplicationData(ref skip_data) = m.payload {
            if skip_data.0.len() <= self.skip_data_left {
                self.skip_data_left -= skip_data.0.len();
                return Ok(self);
            }
        }

        self.next.handle(cx, m)
    }
}

struct ExpectCertificate {
    config: Arc<ServerConfig>,
    transcript: HandshakeHash,
    suite: &'static Tls13CipherSuite,
    key_schedule: KeyScheduleTrafficWithClientFinishedPending,
    send_tickets: usize,
}

impl State<ServerConnectionData> for ExpectCertificate {
    fn handle(mut self: Box<Self>, cx: &mut ServerContext<'_>, m: Message) -> hs::NextStateOrError {
        let certp = require_handshake_msg!(
            m,
            HandshakeType::Certificate,
            HandshakePayload::CertificateTLS13
        )?;
        self.transcript.add_message(&m);

        // We don't send any CertificateRequest extensions, so any extensions
        // here are illegal.
        if certp.any_entry_has_extension() {
            return Err(PeerMisbehaved::UnsolicitedCertExtension.into());
        }

        let client_cert = certp.convert();

        let mandatory = self
            .config
            .verifier
            .client_auth_mandatory();

        let (end_entity, intermediates) = match client_cert.split_first() {
            None => {
                if !mandatory {
                    debug!("client auth requested but no certificate supplied");
                    self.transcript.abandon_client_auth();
                    return Ok(Box::new(ExpectFinished {
                        config: self.config,
                        suite: self.suite,
                        key_schedule: self.key_schedule,
                        transcript: self.transcript,
                        send_tickets: self.send_tickets,
                    }));
                }

                return Err(cx.common.send_fatal_alert(
                    AlertDescription::CertificateRequired,
                    Error::NoCertificatesPresented,
                ));
            }
            Some(chain) => chain,
        };

        let now = std::time::SystemTime::now();
        self.config
            .verifier
            .verify_client_cert(end_entity, intermediates, now)
            .map_err(|err| {
                cx.common
                    .send_cert_verify_error_alert(err)
            })?;

        Ok(Box::new(ExpectCertificateVerify {
            config: self.config,
            suite: self.suite,
            transcript: self.transcript,
            key_schedule: self.key_schedule,
            client_cert,
            send_tickets: self.send_tickets,
        }))
    }
}

struct ExpectCertificateVerify {
    config: Arc<ServerConfig>,
    transcript: HandshakeHash,
    suite: &'static Tls13CipherSuite,
    key_schedule: KeyScheduleTrafficWithClientFinishedPending,
    client_cert: Vec<Certificate>,
    send_tickets: usize,
}

impl State<ServerConnectionData> for ExpectCertificateVerify {
    fn handle(mut self: Box<Self>, cx: &mut ServerContext<'_>, m: Message) -> hs::NextStateOrError {
        let rc = {
            let sig = require_handshake_msg!(
                m,
                HandshakeType::CertificateVerify,
                HandshakePayload::CertificateVerify
            )?;
            let handshake_hash = self.transcript.get_current_hash();
            self.transcript.abandon_client_auth();
            let certs = &self.client_cert;
            let msg = verify::construct_tls13_client_verify_message(&handshake_hash);

            self.config
                .verifier
                .verify_tls13_signature(&msg, &certs[0], sig)
        };

        if let Err(e) = rc {
            return Err(cx
                .common
                .send_cert_verify_error_alert(e));
        }

        trace!("client CertificateVerify OK");
        cx.common.peer_certificates = Some(self.client_cert);

        self.transcript.add_message(&m);
        Ok(Box::new(ExpectFinished {
            config: self.config,
            suite: self.suite,
            key_schedule: self.key_schedule,
            transcript: self.transcript,
            send_tickets: self.send_tickets,
        }))
    }
}

// --- Process (any number of) early ApplicationData messages,
//     followed by a terminating handshake EndOfEarlyData message ---

struct ExpectEarlyData {
    config: Arc<ServerConfig>,
    transcript: HandshakeHash,
    suite: &'static Tls13CipherSuite,
    key_schedule: KeyScheduleTrafficWithClientFinishedPending,
    send_tickets: usize,
}

impl State<ServerConnectionData> for ExpectEarlyData {
    fn handle(mut self: Box<Self>, cx: &mut ServerContext<'_>, m: Message) -> hs::NextStateOrError {
        match m.payload {
            MessagePayload::ApplicationData(payload) => {
                match cx
                    .data
                    .early_data
                    .take_received_plaintext(payload)
                {
                    true => Ok(self),
                    false => Err(cx.common.send_fatal_alert(
                        AlertDescription::UnexpectedMessage,
                        PeerMisbehaved::TooMuchEarlyDataReceived,
                    )),
                }
            }
            MessagePayload::Handshake {
                parsed:
                    HandshakeMessagePayload {
                        typ: HandshakeType::EndOfEarlyData,
                        payload: HandshakePayload::EndOfEarlyData,
                    },
                ..
            } => {
                self.key_schedule
                    .update_decrypter(cx.common);
                self.transcript.add_message(&m);
                Ok(Box::new(ExpectFinished {
                    config: self.config,
                    suite: self.suite,
                    key_schedule: self.key_schedule,
                    transcript: self.transcript,
                    send_tickets: self.send_tickets,
                }))
            }
            payload => Err(inappropriate_handshake_message(
                &payload,
                &[ContentType::ApplicationData, ContentType::Handshake],
                &[HandshakeType::EndOfEarlyData],
            )),
        }
    }
}

// --- Process client's Finished ---
fn get_server_session_value(
    transcript: &HandshakeHash,
    suite: &'static Tls13CipherSuite,
    key_schedule: &KeyScheduleTraffic,
    cx: &ServerContext<'_>,
    nonce: &[u8],
    time_now: ticketer::TimeBase,
    age_obfuscation_offset: u32,
) -> persist::ServerSessionValue {
    let version = ProtocolVersion::TLSv1_3;

    let handshake_hash = transcript.get_current_hash();
    let secret =
        key_schedule.resumption_master_secret_and_derive_ticket_psk(&handshake_hash, nonce);

    persist::ServerSessionValue::new(
        cx.data.sni.as_ref(),
        version,
        suite.common.suite,
        secret,
        cx.common.peer_certificates.clone(),
        cx.common.alpn_protocol.clone(),
        cx.data.resumption_data.clone(),
        time_now,
        age_obfuscation_offset,
    )
}

struct ExpectFinished {
    config: Arc<ServerConfig>,
    transcript: HandshakeHash,
    suite: &'static Tls13CipherSuite,
    key_schedule: KeyScheduleTrafficWithClientFinishedPending,
    send_tickets: usize,
}

impl ExpectFinished {
    fn emit_ticket(
        transcript: &HandshakeHash,
        suite: &'static Tls13CipherSuite,
        cx: &mut ServerContext<'_>,
        key_schedule: &KeyScheduleTraffic,
        config: &ServerConfig,
    ) -> Result<(), Error> {
        let nonce = rand::random_vec(32)?;
        let now = ticketer::TimeBase::now()?;
        let age_add = rand::random_u32()?;
        let plain =
            get_server_session_value(transcript, suite, key_schedule, cx, &nonce, now, age_add)
                .get_encoding();

        let stateless = config.ticketer.enabled();
        let (ticket, lifetime) = if stateless {
            let ticket = match config.ticketer.encrypt(&plain) {
                Some(t) => t,
                None => return Ok(()),
            };
            (ticket, config.ticketer.lifetime())
        } else {
            let id = rand::random_vec(32)?;
            let stored = config
                .session_storage
                .put(id.clone(), plain);
            if !stored {
                trace!("resumption not available; not issuing ticket");
                return Ok(());
            }
            let stateful_lifetime = 24 * 60 * 60; // this is a bit of a punt
            (id, stateful_lifetime)
        };

        let mut payload = NewSessionTicketPayloadTLS13::new(lifetime, age_add, nonce, ticket);

        if config.max_early_data_size > 0 {
            if !stateless {
                payload
                    .exts
                    .push(NewSessionTicketExtension::EarlyData(
                        config.max_early_data_size,
                    ));
            } else {
                // We implement RFC8446 section 8.1: by enforcing that 0-RTT is
                // only possible if using stateful resumption
                warn!("early_data with stateless resumption is not allowed");
            }
        }

        let m = Message {
            version: ProtocolVersion::TLSv1_3,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::NewSessionTicket,
                payload: HandshakePayload::NewSessionTicketTLS13(payload),
            }),
        };

        trace!("sending new ticket {:?} (stateless: {})", m, stateless);
        cx.common.send_msg(m, true);
        Ok(())
    }
}

impl State<ServerConnectionData> for ExpectFinished {
    fn handle(mut self: Box<Self>, cx: &mut ServerContext<'_>, m: Message) -> hs::NextStateOrError {
        let finished =
            require_handshake_msg!(m, HandshakeType::Finished, HandshakePayload::Finished)?;

        let handshake_hash = self.transcript.get_current_hash();
        let (key_schedule_traffic, expect_verify_data) = self
            .key_schedule
            .sign_client_finish(&handshake_hash, cx.common);

        let fin = constant_time::verify_slices_are_equal(expect_verify_data.as_ref(), &finished.0)
            .map_err(|_| {
                warn!("Finished wrong");
                cx.common
                    .send_fatal_alert(AlertDescription::DecryptError, Error::DecryptError)
            })
            .map(|_| verify::FinishedMessageVerified::assertion())?;

        // nb. future derivations include Client Finished, but not the
        // main application data keying.
        self.transcript.add_message(&m);

        cx.common.check_aligned_handshake()?;

        for _ in 0..self.send_tickets {
            Self::emit_ticket(
                &self.transcript,
                self.suite,
                cx,
                &key_schedule_traffic,
                &self.config,
            )?;
        }

        // Application data may now flow, even if we have client auth enabled.
        cx.common.start_traffic();

        #[cfg(feature = "quic")]
        {
            if cx.common.protocol == Protocol::Quic {
                return Ok(Box::new(ExpectQuicTraffic {
                    key_schedule: key_schedule_traffic,
                    _fin_verified: fin,
                }));
            }
        }

        Ok(Box::new(ExpectTraffic {
            key_schedule: key_schedule_traffic,
            _fin_verified: fin,
        }))
    }
}

// --- Process traffic ---
struct ExpectTraffic {
    key_schedule: KeyScheduleTraffic,
    _fin_verified: verify::FinishedMessageVerified,
}

impl ExpectTraffic {
    fn handle_key_update(
        &mut self,
        common: &mut CommonState,
        key_update_request: &KeyUpdateRequest,
    ) -> Result<(), Error> {
        #[cfg(feature = "quic")]
        {
            if let Protocol::Quic = common.protocol {
                return Err(common.send_fatal_alert(
                    AlertDescription::UnexpectedMessage,
                    PeerMisbehaved::KeyUpdateReceivedInQuicConnection,
                ));
            }
        }

        common.check_aligned_handshake()?;

        if common.should_update_key(key_update_request)? {
            self.key_schedule
                .update_encrypter_and_notify(common);
        }

        // Update our read-side keys.
        self.key_schedule
            .update_decrypter(common);
        Ok(())
    }
}

impl State<ServerConnectionData> for ExpectTraffic {
    fn handle(mut self: Box<Self>, cx: &mut ServerContext, m: Message) -> hs::NextStateOrError {
        match m.payload {
            MessagePayload::ApplicationData(payload) => cx
                .common
                .take_received_plaintext(payload),
            MessagePayload::Handshake {
                parsed:
                    HandshakeMessagePayload {
                        payload: HandshakePayload::KeyUpdate(key_update),
                        ..
                    },
                ..
            } => self.handle_key_update(cx.common, &key_update)?,
            payload => {
                return Err(inappropriate_handshake_message(
                    &payload,
                    &[ContentType::ApplicationData, ContentType::Handshake],
                    &[HandshakeType::KeyUpdate],
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
        self.key_schedule
            .export_keying_material(output, label, context)
    }

    #[cfg(feature = "secret_extraction")]
    fn extract_secrets(&self) -> Result<PartiallyExtractedSecrets, Error> {
        self.key_schedule
            .extract_secrets(Side::Server)
    }
}

#[cfg(feature = "quic")]
struct ExpectQuicTraffic {
    key_schedule: KeyScheduleTraffic,
    _fin_verified: verify::FinishedMessageVerified,
}

#[cfg(feature = "quic")]
impl State<ServerConnectionData> for ExpectQuicTraffic {
    fn handle(self: Box<Self>, _cx: &mut ServerContext<'_>, m: Message) -> hs::NextStateOrError {
        // reject all messages
        Err(inappropriate_message(&m.payload, &[]))
    }

    fn export_keying_material(
        &self,
        output: &mut [u8],
        label: &[u8],
        context: Option<&[u8]>,
    ) -> Result<(), Error> {
        self.key_schedule
            .export_keying_material(output, label, context)
    }
}
