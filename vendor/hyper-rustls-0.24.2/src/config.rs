use rustls::client::WantsTransparencyPolicyOrClientCert;
use rustls::{ClientConfig, ConfigBuilder, WantsVerifier};

/// Methods for configuring roots
///
/// This adds methods (gated by crate features) for easily configuring
/// TLS server roots a rustls ClientConfig will trust.
pub trait ConfigBuilderExt {
    /// This configures the platform's trusted certs, as implemented by
    /// rustls-native-certs
    #[cfg(feature = "rustls-native-certs")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rustls-native-certs")))]
    fn with_native_roots(self) -> ConfigBuilder<ClientConfig, WantsTransparencyPolicyOrClientCert>;

    /// This configures the webpki roots, which are Mozilla's set of
    /// trusted roots as packaged by webpki-roots.
    #[cfg(feature = "webpki-roots")]
    #[cfg_attr(docsrs, doc(cfg(feature = "webpki-roots")))]
    fn with_webpki_roots(self) -> ConfigBuilder<ClientConfig, WantsTransparencyPolicyOrClientCert>;
}

impl ConfigBuilderExt for ConfigBuilder<ClientConfig, WantsVerifier> {
    #[cfg(feature = "rustls-native-certs")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rustls-native-certs")))]
    #[cfg_attr(not(feature = "logging"), allow(unused_variables))]
    fn with_native_roots(self) -> ConfigBuilder<ClientConfig, WantsTransparencyPolicyOrClientCert> {
        let mut roots = rustls::RootCertStore::empty();
        let mut valid_count = 0;
        let mut invalid_count = 0;

        for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs")
        {
            let cert = rustls::Certificate(cert.0);
            match roots.add(&cert) {
                Ok(_) => valid_count += 1,
                Err(err) => {
                    crate::log::trace!("invalid cert der {:?}", cert.0);
                    crate::log::debug!("certificate parsing failed: {:?}", err);
                    invalid_count += 1
                }
            }
        }
        crate::log::debug!(
            "with_native_roots processed {} valid and {} invalid certs",
            valid_count,
            invalid_count
        );
        assert!(!roots.is_empty(), "no CA certificates found");

        self.with_root_certificates(roots)
    }

    #[cfg(feature = "webpki-roots")]
    #[cfg_attr(docsrs, doc(cfg(feature = "webpki-roots")))]
    fn with_webpki_roots(self) -> ConfigBuilder<ClientConfig, WantsTransparencyPolicyOrClientCert> {
        let mut roots = rustls::RootCertStore::empty();
        roots.add_trust_anchors(
            webpki_roots::TLS_SERVER_ROOTS
                .iter()
                .map(|ta| {
                    rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                        ta.subject,
                        ta.spki,
                        ta.name_constraints,
                    )
                }),
        );
        self.with_root_certificates(roots)
    }
}
