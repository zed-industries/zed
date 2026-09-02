use std::sync::OnceLock;

#[cfg(target_os = "ios")]
use std::sync::Arc;

use rustls::ClientConfig;
#[cfg(target_os = "ios")]
use rustls::{
    CertificateError, DigitallySignedStruct, Error as TlsError, RootCertStore, SignatureScheme,
    client::{
        WebPkiServerVerifier,
        danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    },
    pki_types::{CertificateDer, ServerName, UnixTime},
};
use rustls_platform_verifier::ConfigVerifierExt;

static TLS_CONFIG: OnceLock<rustls::ClientConfig> = OnceLock::new();

pub fn tls_config() -> ClientConfig {
    TLS_CONFIG
        .get_or_init(|| {
            // rustls uses the `aws_lc_rs` provider by default
            // This only errors if the default provider has already
            // been installed. We can ignore this `Result`.
            rustls::crypto::aws_lc_rs::default_provider()
                .install_default()
                .ok();

            #[cfg(target_os = "ios")]
            {
                ios_tls_config()
            }

            #[cfg(not(target_os = "ios"))]
            {
                ClientConfig::with_platform_verifier().unwrap_or_else(|error| {
                    log::error!(
                        "failed to load platform TLS certificate verifier, falling back to bundled webpki roots: {error}"
                    );
                    bundled_tls_config()
                })
            }
        })
        .clone()
}

fn bundled_tls_config() -> ClientConfig {
    ClientConfig::builder()
        .with_root_certificates(rustls::RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
        })
        .with_no_client_auth()
}

#[cfg(target_os = "ios")]
fn ios_tls_config() -> ClientConfig {
    log::info!("initializing iOS TLS with the platform verifier and bundled-root fallback");
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let webpki_verifier = match WebPkiServerVerifier::builder(Arc::new(root_store)).build() {
        Ok(verifier) => verifier,
        Err(error) => {
            log::error!("failed to initialize iOS WebPKI certificate fallback: {error}");
            return ClientConfig::with_platform_verifier().unwrap_or_else(|platform_error| {
                log::error!(
                    "failed to load the iOS platform TLS certificate verifier: {platform_error}"
                );
                bundled_tls_config()
            });
        }
    };
    let config_builder = ClientConfig::builder();
    let platform = match rustls_platform_verifier::Verifier::new(
        config_builder.crypto_provider().clone(),
    ) {
        Ok(platform) => platform,
        Err(error) => {
            log::error!(
                "failed to load the iOS platform TLS certificate verifier, falling back to bundled webpki roots: {error}"
            );
            return bundled_tls_config();
        }
    };
    let verifier = IosCertificateVerifier {
        platform,
        webpki: webpki_verifier,
    };
    config_builder
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(verifier))
        .with_no_client_auth()
}

/// Retains the iOS system verifier while tolerating incomplete system root stores.
///
/// Only an unknown issuer is retried against bundled public roots. Name,
/// validity, revocation, and every other platform verification failure remain
/// authoritative.
#[cfg(target_os = "ios")]
#[derive(Debug)]
struct IosCertificateVerifier {
    platform: rustls_platform_verifier::Verifier,
    webpki: Arc<WebPkiServerVerifier>,
}

#[cfg(target_os = "ios")]
impl ServerCertVerifier for IosCertificateVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        server_name: &ServerName<'_>,
        ocsp_response: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, TlsError> {
        match self.platform.verify_server_cert(
            end_entity,
            intermediates,
            server_name,
            ocsp_response,
            now,
        ) {
            Err(TlsError::InvalidCertificate(CertificateError::UnknownIssuer)) => {
                log::warn!(
                    "iOS platform TLS verification reported an unknown issuer for {server_name:?}; retrying with {} intermediate certificate(s) against bundled public roots",
                    intermediates.len()
                );
                match self.webpki.verify_server_cert(
                    end_entity,
                    intermediates,
                    server_name,
                    ocsp_response,
                    now,
                ) {
                    Ok(verified) => {
                        log::info!(
                            "bundled public roots accepted the TLS certificate for {server_name:?}"
                        );
                        Ok(verified)
                    }
                    Err(error) => {
                        log::error!(
                            "bundled public roots also rejected the TLS certificate for {server_name:?}: {error}"
                        );
                        Err(TlsError::General(format!(
                            "the iOS platform verifier reported an unknown issuer and bundled public roots also rejected the certificate: {error}"
                        )))
                    }
                }
            }
            result => result,
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        certificate: &CertificateDer<'_>,
        signature: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, TlsError> {
        self.platform
            .verify_tls12_signature(message, certificate, signature)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        certificate: &CertificateDer<'_>,
        signature: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, TlsError> {
        self.platform
            .verify_tls13_signature(message, certificate, signature)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.platform.supported_verify_schemes()
    }
}
