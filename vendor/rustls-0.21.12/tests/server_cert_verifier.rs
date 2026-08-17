//! Tests for configuring and using a [`ServerCertVerifier`] for a client.

#![cfg(feature = "dangerous_configuration")]

mod common;
use crate::common::{
    do_handshake, do_handshake_until_both_error, make_client_config_with_versions,
    make_pair_for_arc_configs, make_server_config, ErrorFromPeer, ALL_KEY_TYPES,
};
use rustls::client::{
    HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier, WebPkiVerifier,
};
use rustls::DigitallySignedStruct;
use rustls::{AlertDescription, Certificate, Error, InvalidMessage, SignatureScheme};
use std::sync::Arc;

#[test]
fn client_can_override_certificate_verification() {
    for kt in ALL_KEY_TYPES.iter() {
        let verifier = Arc::new(MockServerVerifier::accepts_anything());

        let server_config = Arc::new(make_server_config(*kt));

        for version in rustls::ALL_VERSIONS {
            let mut client_config = make_client_config_with_versions(*kt, &[version]);
            client_config
                .dangerous()
                .set_certificate_verifier(verifier.clone());

            let (mut client, mut server) =
                make_pair_for_arc_configs(&Arc::new(client_config), &server_config);
            do_handshake(&mut client, &mut server);
        }
    }
}

#[test]
fn client_can_override_certificate_verification_and_reject_certificate() {
    for kt in ALL_KEY_TYPES.iter() {
        let verifier = Arc::new(MockServerVerifier::rejects_certificate(
            Error::InvalidMessage(InvalidMessage::HandshakePayloadTooLarge),
        ));

        let server_config = Arc::new(make_server_config(*kt));

        for version in rustls::ALL_VERSIONS {
            let mut client_config = make_client_config_with_versions(*kt, &[version]);
            client_config
                .dangerous()
                .set_certificate_verifier(verifier.clone());

            let (mut client, mut server) =
                make_pair_for_arc_configs(&Arc::new(client_config), &server_config);
            let errs = do_handshake_until_both_error(&mut client, &mut server);
            assert_eq!(
                errs,
                Err(vec![
                    ErrorFromPeer::Client(Error::InvalidMessage(
                        InvalidMessage::HandshakePayloadTooLarge,
                    )),
                    ErrorFromPeer::Server(Error::AlertReceived(AlertDescription::HandshakeFailure)),
                ]),
            );
        }
    }
}

#[cfg(feature = "tls12")]
#[test]
fn client_can_override_certificate_verification_and_reject_tls12_signatures() {
    for kt in ALL_KEY_TYPES.iter() {
        let mut client_config = make_client_config_with_versions(*kt, &[&rustls::version::TLS12]);
        let verifier = Arc::new(MockServerVerifier::rejects_tls12_signatures(
            Error::InvalidMessage(InvalidMessage::HandshakePayloadTooLarge),
        ));

        client_config
            .dangerous()
            .set_certificate_verifier(verifier);

        let server_config = Arc::new(make_server_config(*kt));

        let (mut client, mut server) =
            make_pair_for_arc_configs(&Arc::new(client_config), &server_config);
        let errs = do_handshake_until_both_error(&mut client, &mut server);
        assert_eq!(
            errs,
            Err(vec![
                ErrorFromPeer::Client(Error::InvalidMessage(
                    InvalidMessage::HandshakePayloadTooLarge,
                )),
                ErrorFromPeer::Server(Error::AlertReceived(AlertDescription::HandshakeFailure)),
            ]),
        );
    }
}

#[test]
fn client_can_override_certificate_verification_and_reject_tls13_signatures() {
    for kt in ALL_KEY_TYPES.iter() {
        let mut client_config = make_client_config_with_versions(*kt, &[&rustls::version::TLS13]);
        let verifier = Arc::new(MockServerVerifier::rejects_tls13_signatures(
            Error::InvalidMessage(InvalidMessage::HandshakePayloadTooLarge),
        ));

        client_config
            .dangerous()
            .set_certificate_verifier(verifier);

        let server_config = Arc::new(make_server_config(*kt));

        let (mut client, mut server) =
            make_pair_for_arc_configs(&Arc::new(client_config), &server_config);
        let errs = do_handshake_until_both_error(&mut client, &mut server);
        assert_eq!(
            errs,
            Err(vec![
                ErrorFromPeer::Client(Error::InvalidMessage(
                    InvalidMessage::HandshakePayloadTooLarge,
                )),
                ErrorFromPeer::Server(Error::AlertReceived(AlertDescription::HandshakeFailure)),
            ]),
        );
    }
}

#[test]
fn client_can_override_certificate_verification_and_offer_no_signature_schemes() {
    for kt in ALL_KEY_TYPES.iter() {
        let verifier = Arc::new(MockServerVerifier::offers_no_signature_schemes());

        let server_config = Arc::new(make_server_config(*kt));

        for version in rustls::ALL_VERSIONS {
            let mut client_config = make_client_config_with_versions(*kt, &[version]);
            client_config
                .dangerous()
                .set_certificate_verifier(verifier.clone());

            let (mut client, mut server) =
                make_pair_for_arc_configs(&Arc::new(client_config), &server_config);
            let errs = do_handshake_until_both_error(&mut client, &mut server);
            assert_eq!(
                errs,
                Err(vec![
                    ErrorFromPeer::Server(Error::PeerIncompatible(
                        rustls::PeerIncompatible::NoSignatureSchemesInCommon
                    )),
                    ErrorFromPeer::Client(Error::AlertReceived(AlertDescription::HandshakeFailure)),
                ])
            );
        }
    }
}

pub struct MockServerVerifier {
    cert_rejection_error: Option<Error>,
    tls12_signature_error: Option<Error>,
    tls13_signature_error: Option<Error>,
    wants_scts: bool,
    signature_schemes: Vec<SignatureScheme>,
}

impl ServerCertVerifier for MockServerVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &rustls::Certificate,
        intermediates: &[rustls::Certificate],
        server_name: &rustls::ServerName,
        scts: &mut dyn Iterator<Item = &[u8]>,
        oscp_response: &[u8],
        now: std::time::SystemTime,
    ) -> Result<ServerCertVerified, Error> {
        let scts: Vec<Vec<u8>> = scts.map(|x| x.to_owned()).collect();
        println!(
            "verify_server_cert({:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            end_entity, intermediates, server_name, scts, oscp_response, now
        );
        if let Some(error) = &self.cert_rejection_error {
            Err(error.clone())
        } else {
            Ok(ServerCertVerified::assertion())
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &Certificate,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        println!(
            "verify_tls12_signature({:?}, {:?}, {:?})",
            message, cert, dss
        );
        if let Some(error) = &self.tls12_signature_error {
            Err(error.clone())
        } else {
            Ok(HandshakeSignatureValid::assertion())
        }
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &Certificate,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        println!(
            "verify_tls13_signature({:?}, {:?}, {:?})",
            message, cert, dss
        );
        if let Some(error) = &self.tls13_signature_error {
            Err(error.clone())
        } else {
            Ok(HandshakeSignatureValid::assertion())
        }
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.signature_schemes.clone()
    }

    fn request_scts(&self) -> bool {
        println!("request_scts? {:?}", self.wants_scts);
        self.wants_scts
    }
}

impl MockServerVerifier {
    pub fn accepts_anything() -> Self {
        MockServerVerifier {
            cert_rejection_error: None,
            ..Default::default()
        }
    }

    pub fn rejects_certificate(err: Error) -> Self {
        MockServerVerifier {
            cert_rejection_error: Some(err),
            ..Default::default()
        }
    }

    pub fn rejects_tls12_signatures(err: Error) -> Self {
        MockServerVerifier {
            tls12_signature_error: Some(err),
            ..Default::default()
        }
    }

    pub fn rejects_tls13_signatures(err: Error) -> Self {
        MockServerVerifier {
            tls13_signature_error: Some(err),
            ..Default::default()
        }
    }

    pub fn offers_no_signature_schemes() -> Self {
        MockServerVerifier {
            signature_schemes: vec![],
            ..Default::default()
        }
    }
}

impl Default for MockServerVerifier {
    fn default() -> Self {
        MockServerVerifier {
            cert_rejection_error: None,
            tls12_signature_error: None,
            tls13_signature_error: None,
            wants_scts: false,
            signature_schemes: WebPkiVerifier::verification_schemes(),
        }
    }
}
