use std::sync::Arc;

use jni::{
    objects::{JObject, JValue},
    strings::JavaStr,
    JNIEnv,
};
use once_cell::sync::OnceCell;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerifier};
use rustls::crypto::{verify_tls12_signature, verify_tls13_signature, CryptoProvider};
use rustls::pki_types;
use rustls::Error::InvalidCertificate;
use rustls::{
    CertificateError, DigitallySignedStruct, Error as TlsError, OtherError, SignatureScheme,
};

use super::{log_server_cert, ALLOWED_EKUS};
use crate::android::{with_context, CachedClass};

static CERT_VERIFIER_CLASS: CachedClass =
    CachedClass::new("org/rustls/platformverifier/CertificateVerifier");

// Find the `ByteArray (Uint8 [])` class.
static BYTE_ARRAY_CLASS: CachedClass = CachedClass::new("[B");

static STRING_CLASS: CachedClass = CachedClass::new("java/lang/String");

// Note: Keep these in sync with the Kotlin enum.
#[derive(Debug)]
enum VerifierStatus {
    Ok,
    Unavailable,
    Expired,
    UnknownCert,
    Revoked,
    InvalidEncoding,
    InvalidExtension,
}

// Android's certificate verifier ignores this outright and this is considered the
// official recommendation. See https://bugs.chromium.org/p/chromium/issues/detail?id=627154.
const AUTH_TYPE: &str = "RSA";

/// A TLS certificate verifier that utilizes the Android platform verifier.
#[derive(Debug)]
pub struct Verifier {
    /// Testing only: The root CA certificate to trust.
    #[cfg(any(test, feature = "ffi-testing"))]
    test_only_root_ca_override: Option<Vec<u8>>,
    pub(super) crypto_provider: OnceCell<Arc<CryptoProvider>>,
}

impl Default for Verifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(test, feature = "ffi-testing"))]
impl Drop for Verifier {
    fn drop(&mut self) {
        with_context::<_, ()>(|cx, env| {
            env.call_static_method(CERT_VERIFIER_CLASS.get(cx)?, "clearMockRoots", "()V", &[])?
                .v()?;
            Ok(())
        })
        .expect("failed to clear test roots")
    }
}

impl Verifier {
    /// Creates a new instance of a TLS certificate verifier that utilizes the
    /// Android certificate facilities.
    ///
    /// A [`CryptoProvider`] must be set with
    /// [`set_provider`][Verifier::set_provider]/[`with_provider`][Verifier::with_provider] or
    /// [`CryptoProvider::install_default`] before the verifier can be used.
    pub fn new() -> Self {
        Self {
            #[cfg(any(test, feature = "ffi-testing"))]
            test_only_root_ca_override: None,
            crypto_provider: OnceCell::new(),
        }
    }

    /// Creates a test-only TLS certificate verifier which trusts our fake root CA cert.
    #[cfg(any(test, feature = "ffi-testing"))]
    pub(crate) fn new_with_fake_root(root: &[u8]) -> Self {
        Self {
            test_only_root_ca_override: Some(root.into()),
            crypto_provider: OnceCell::new(),
        }
    }

    fn verify_certificate(
        &self,
        end_entity: &pki_types::CertificateDer<'_>,
        intermediates: &[pki_types::CertificateDer<'_>],
        server_name: &pki_types::ServerName<'_>,
        ocsp_response: Option<&[u8]>,
        now: pki_types::UnixTime,
    ) -> Result<(), TlsError> {
        let certificate_chain = std::iter::once(end_entity)
            .chain(intermediates)
            .map(|cert| cert.as_ref())
            .enumerate();

        // Convert the unix timestamp into milliseconds, expressed as
        // an i64 to later be converted into a Java Long used for a Date
        // constructor.
        let now: i64 = (now.as_secs() * 1000)
            .try_into()
            .map_err(|_| TlsError::FailedToGetCurrentTime)?;

        let verification_result = with_context(|cx, env| {
            // We don't provide an initial element so that the array filling can be cleaner.
            // It's valid to provide a `null` value. Ref: https://docs.oracle.com/en/java/javase/13/docs/specs/jni/functions.html -> NewObjectArray
            let cert_list = {
                let array = env.new_object_array(
                    (intermediates.len() + 1).try_into().unwrap(),
                    BYTE_ARRAY_CLASS.get(cx)?,
                    JObject::null(),
                )?;

                for (idx, cert) in certificate_chain {
                    let idx = idx.try_into().unwrap();
                    let cert_buffer = env.byte_array_from_slice(cert)?;
                    env.set_object_array_element(&array, idx, cert_buffer)?
                }

                array
            };

            let allowed_ekus = {
                let array = env.new_object_array(
                    ALLOWED_EKUS.len().try_into().unwrap(),
                    STRING_CLASS.get(cx)?,
                    JObject::null(),
                )?;

                for (idx, eku) in ALLOWED_EKUS.iter().enumerate() {
                    let idx = idx.try_into().unwrap();
                    let eku = env.new_string(eku)?;
                    env.set_object_array_element(&array, idx, eku)?;
                }

                array
            };

            let ocsp_response = match ocsp_response {
                Some(b) => env.byte_array_from_slice(b)?,
                None => JObject::null().into(),
            };

            #[cfg(any(test, feature = "ffi-testing"))]
            {
                if let Some(mock_root) = &self.test_only_root_ca_override {
                    let mock_root = env.byte_array_from_slice(mock_root)?;
                    env.call_static_method(
                        CERT_VERIFIER_CLASS.get(cx)?,
                        "addMockRoot",
                        "([B)V",
                        &[JValue::from(&mock_root)],
                    )?
                    .v()
                    .expect("failed to add test root")
                }
            }

            const VERIFIER_CALL: &str = concat!(
                '(',
                "Landroid/content/Context;",
                "Ljava/lang/String;",
                "Ljava/lang/String;",
                "[Ljava/lang/String;",
                "[B",
                'J',
                "[[B",
                ')',
                "Lorg/rustls/platformverifier/VerificationResult;"
            );

            let result = env
                .call_static_method(
                    CERT_VERIFIER_CLASS.get(cx)?,
                    "verifyCertificateChain",
                    VERIFIER_CALL,
                    &[
                        JValue::from(cx.application_context()),
                        JValue::from(&env.new_string(server_name.to_str())?),
                        JValue::from(&env.new_string(AUTH_TYPE)?),
                        JValue::from(&JObject::from(allowed_ekus)),
                        JValue::from(&ocsp_response),
                        JValue::Long(now),
                        JValue::from(&JObject::from(cert_list)),
                    ],
                )?
                .l()?;

            Ok(extract_result_info(env, result))
        });

        match verification_result {
            Ok((status, maybe_msg)) => {
                // `maybe_msg` is safe to log as its exactly what the system told us.
                //
                // The branches which unwrap it will never fail since the Kotlin side always sets it
                // for the variants.
                match status {
                    VerifierStatus::Ok => {
                        // If everything else was OK, check the hostname.
                        rustls::client::verify_server_name(
                            &rustls::server::ParsedCertificate::try_from(end_entity)?,
                            server_name,
                        )
                    }
                    VerifierStatus::Unavailable => Err(TlsError::General(String::from(
                        "No system trust stores available",
                    ))),
                    VerifierStatus::Expired => Err(InvalidCertificate(CertificateError::Expired)),
                    VerifierStatus::UnknownCert => {
                        log::warn!("certificate was not trusted: {}", maybe_msg.unwrap());
                        Err(InvalidCertificate(CertificateError::UnknownIssuer))
                    }
                    VerifierStatus::Revoked => {
                        log::warn!("certificate was revoked: {}", maybe_msg.unwrap());
                        Err(InvalidCertificate(CertificateError::Revoked))
                    }
                    VerifierStatus::InvalidEncoding => {
                        Err(InvalidCertificate(CertificateError::BadEncoding))
                    }
                    VerifierStatus::InvalidExtension => Err(InvalidCertificate(
                        CertificateError::Other(OtherError(std::sync::Arc::new(super::EkuError))),
                    )),
                }
            }
            Err(e) => Err(TlsError::General(format!(
                "failed to call native verifier: {e:?}",
            ))),
        }
    }
}

fn extract_result_info(
    env: &mut JNIEnv<'_>,
    result: JObject<'_>,
) -> (VerifierStatus, Option<String>) {
    let status_code = env
        .get_field(&result, "code", "I")
        .and_then(|code| code.i())
        .unwrap();

    let status = match status_code {
        0 => VerifierStatus::Ok,
        1 => VerifierStatus::Unavailable,
        2 => VerifierStatus::Expired,
        3 => VerifierStatus::UnknownCert,
        4 => VerifierStatus::Revoked,
        5 => VerifierStatus::InvalidEncoding,
        6 => VerifierStatus::InvalidExtension,
        i => unreachable!("unknown status code: {i}"),
    };

    // Extract the `String?`.
    let msg = env
        .get_field(result, "message", "Ljava/lang/String;")
        .and_then(|m| m.l())
        .map(|s| {
            if s.is_null() {
                None
            } else {
                JavaStr::from_env(env, &s.into()).ok().map(String::from)
            }
        })
        .unwrap();
    (status, msg)
}

impl ServerCertVerifier for Verifier {
    fn verify_server_cert(
        &self,
        end_entity: &pki_types::CertificateDer<'_>,
        intermediates: &[pki_types::CertificateDer<'_>],
        server_name: &pki_types::ServerName,
        ocsp_response: &[u8],
        now: pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, TlsError> {
        log_server_cert(end_entity);

        let ocsp_data = if !ocsp_response.is_empty() {
            Some(ocsp_response)
        } else {
            None
        };

        match self.verify_certificate(end_entity, intermediates, server_name, ocsp_data, now) {
            Ok(()) => Ok(rustls::client::danger::ServerCertVerified::assertion()),
            Err(e) => {
                // This error only tells us what the system errored with, so it doesn't leak anything
                // sensitive.
                log::error!("failed to verify TLS certificate: {}", e);
                Err(e)
            }
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &pki_types::CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, TlsError> {
        verify_tls12_signature(
            message,
            cert,
            dss,
            &self.get_provider().signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &pki_types::CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, TlsError> {
        verify_tls13_signature(
            message,
            cert,
            dss,
            &self.get_provider().signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.get_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }
}
