use super::ResolvesClientCert;
#[cfg(feature = "logging")]
use crate::log::{debug, trace};
use crate::msgs::enums::ExtensionType;
use crate::msgs::handshake::{CertificatePayload, DistinguishedName};
use crate::msgs::handshake::{Sct, ServerExtension};
use crate::{sign, SignatureScheme};

use std::sync::Arc;

#[derive(Debug)]
pub(super) struct ServerCertDetails {
    pub(super) cert_chain: CertificatePayload,
    pub(super) ocsp_response: Vec<u8>,
    pub(super) scts: Option<Vec<Sct>>,
}

impl ServerCertDetails {
    pub(super) fn new(
        cert_chain: CertificatePayload,
        ocsp_response: Vec<u8>,
        scts: Option<Vec<Sct>>,
    ) -> Self {
        Self {
            cert_chain,
            ocsp_response,
            scts,
        }
    }

    pub(super) fn scts(&self) -> impl Iterator<Item = &[u8]> {
        self.scts
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|payload| payload.as_ref())
    }
}

pub(super) struct ClientHelloDetails {
    pub(super) sent_extensions: Vec<ExtensionType>,
}

impl ClientHelloDetails {
    pub(super) fn new() -> Self {
        Self {
            sent_extensions: Vec::new(),
        }
    }

    pub(super) fn server_may_send_sct_list(&self) -> bool {
        self.sent_extensions
            .contains(&ExtensionType::SCT)
    }

    pub(super) fn server_sent_unsolicited_extensions(
        &self,
        received_exts: &[ServerExtension],
        allowed_unsolicited: &[ExtensionType],
    ) -> bool {
        for ext in received_exts {
            let ext_type = ext.get_type();
            if !self.sent_extensions.contains(&ext_type) && !allowed_unsolicited.contains(&ext_type)
            {
                trace!("Unsolicited extension {:?}", ext_type);
                return true;
            }
        }

        false
    }
}

pub(super) enum ClientAuthDetails {
    /// Send an empty `Certificate` and no `CertificateVerify`.
    Empty { auth_context_tls13: Option<Vec<u8>> },
    /// Send a non-empty `Certificate` and a `CertificateVerify`.
    Verify {
        certkey: Arc<sign::CertifiedKey>,
        signer: Box<dyn sign::Signer>,
        auth_context_tls13: Option<Vec<u8>>,
    },
}

impl ClientAuthDetails {
    pub(super) fn resolve(
        resolver: &dyn ResolvesClientCert,
        canames: Option<&[DistinguishedName]>,
        sigschemes: &[SignatureScheme],
        auth_context_tls13: Option<Vec<u8>>,
    ) -> Self {
        let acceptable_issuers = canames
            .unwrap_or_default()
            .iter()
            .map(|p| p.as_ref())
            .collect::<Vec<&[u8]>>();

        if let Some(certkey) = resolver.resolve(&acceptable_issuers, sigschemes) {
            if let Some(signer) = certkey.key.choose_scheme(sigschemes) {
                debug!("Attempting client auth");
                return Self::Verify {
                    certkey,
                    signer,
                    auth_context_tls13,
                };
            }
        }

        debug!("Client auth requested but no cert/sigscheme available");
        Self::Empty { auth_context_tls13 }
    }
}
