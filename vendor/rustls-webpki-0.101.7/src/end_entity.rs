// Copyright 2015-2021 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

#[cfg(feature = "alloc")]
use crate::subject_name::GeneralDnsNameRef;
use crate::{
    cert, signed_data, subject_name, verify_cert, CertRevocationList, Error, KeyUsage,
    SignatureAlgorithm, SubjectNameRef, Time, TrustAnchor,
};
#[allow(deprecated)]
use crate::{TlsClientTrustAnchors, TlsServerTrustAnchors};

/// An end-entity certificate.
///
/// Server certificate processing in a TLS connection consists of several
/// steps. All of these steps are necessary:
///
/// * `EndEntityCert.verify_is_valid_tls_server_cert`: Verify that the server's
///   certificate is currently valid *for use by a TLS server*.
/// * `EndEntityCert.verify_is_valid_for_subject_name`: Verify that the server's
///   certificate is valid for the host or IP address that is being connected to.
///
/// * `EndEntityCert.verify_signature`: Verify that the signature of server's
///   `ServerKeyExchange` message is valid for the server's certificate.
///
/// Client certificate processing in a TLS connection consists of analogous
/// steps. All of these steps are necessary:
///
/// * `EndEntityCert.verify_is_valid_tls_client_cert`: Verify that the client's
///   certificate is currently valid *for use by a TLS client*.
/// * `EndEntityCert.verify_signature`: Verify that the client's signature in
///   its `CertificateVerify` message is valid using the public key from the
///   client's certificate.
///
/// Although it would be less error-prone to combine all these steps into a
/// single function call, some significant optimizations are possible if the
/// three steps are processed separately (in parallel). It does not matter much
/// which order the steps are done in, but **all of these steps must completed
/// before application data is sent and before received application data is
/// processed**. `EndEntityCert::from` is an inexpensive operation and is
/// deterministic, so if these tasks are done in multiple threads, it is
/// probably best to just call `EndEntityCert::from` multiple times (before each
/// operation) for the same DER-encoded ASN.1 certificate bytes.
pub struct EndEntityCert<'a> {
    inner: cert::Cert<'a>,
}

impl<'a> TryFrom<&'a [u8]> for EndEntityCert<'a> {
    type Error = Error;

    /// Parse the ASN.1 DER-encoded X.509 encoding of the certificate
    /// `cert_der`.
    fn try_from(cert_der: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: cert::Cert::from_der(
                untrusted::Input::from(cert_der),
                cert::EndEntityOrCa::EndEntity,
            )?,
        })
    }
}

impl<'a> EndEntityCert<'a> {
    pub(super) fn inner(&self) -> &cert::Cert {
        &self.inner
    }

    fn verify_is_valid_cert(
        &self,
        supported_sig_algs: &[&SignatureAlgorithm],
        trust_anchors: &[TrustAnchor],
        intermediate_certs: &[&[u8]],
        time: Time,
        eku: KeyUsage,
        crls: &[&dyn CertRevocationList],
    ) -> Result<(), Error> {
        verify_cert::build_chain(
            &verify_cert::ChainOptions {
                eku,
                supported_sig_algs,
                trust_anchors,
                intermediate_certs,
                crls,
            },
            &self.inner,
            time,
        )
    }

    /// Verifies that the end-entity certificate is valid for use against the
    /// specified Extended Key Usage (EKU).
    ///
    /// * `supported_sig_algs` is the list of signature algorithms that are
    ///   trusted for use in certificate signatures; the end-entity certificate's
    ///   public key is not validated against this list.
    /// * `trust_anchors` is the list of root CAs to trust
    /// * `intermediate_certs` is the sequence of intermediate certificates that
    ///   the server sent in the TLS handshake.
    /// * `time` is the time for which the validation is effective (usually the
    ///   current time).
    /// * `usage` is the intended usage of the certificate, indicating what kind
    ///   of usage we're verifying the certificate for.
    /// * `crls` is the list of certificate revocation lists to check
    ///   the certificate against.
    pub fn verify_for_usage(
        &self,
        supported_sig_algs: &[&SignatureAlgorithm],
        trust_anchors: &[TrustAnchor],
        intermediate_certs: &[&[u8]],
        time: Time,
        usage: KeyUsage,
        crls: &[&dyn CertRevocationList],
    ) -> Result<(), Error> {
        self.verify_is_valid_cert(
            supported_sig_algs,
            trust_anchors,
            intermediate_certs,
            time,
            usage,
            crls,
        )
    }

    /// Verifies that the end-entity certificate is valid for use by a TLS
    /// server.
    ///
    /// `supported_sig_algs` is the list of signature algorithms that are
    /// trusted for use in certificate signatures; the end-entity certificate's
    /// public key is not validated against this list. `trust_anchors` is the
    /// list of root CAs to trust. `intermediate_certs` is the sequence of
    /// intermediate certificates that the server sent in the TLS handshake.
    /// `time` is the time for which the validation is effective (usually the
    /// current time).
    #[allow(deprecated)]
    #[deprecated(
        since = "0.101.2",
        note = "The per-usage trust anchor representations and verification functions are deprecated in \
        favor of the general-purpose `TrustAnchor` type and `EndEntity::verify_for_usage` function. \
        The new `verify_for_usage` function expresses trust anchor and end entity purpose with the \
        key usage argument."
    )]
    pub fn verify_is_valid_tls_server_cert(
        &self,
        supported_sig_algs: &[&SignatureAlgorithm],
        &TlsServerTrustAnchors(trust_anchors): &TlsServerTrustAnchors,
        intermediate_certs: &[&[u8]],
        time: Time,
    ) -> Result<(), Error> {
        self.verify_is_valid_cert(
            supported_sig_algs,
            trust_anchors,
            intermediate_certs,
            time,
            KeyUsage::server_auth(),
            &[],
        )
    }

    /// Verifies that the end-entity certificate is valid for use by a TLS
    /// client.
    ///
    /// `supported_sig_algs` is the list of signature algorithms that are
    /// trusted for use in certificate signatures; the end-entity certificate's
    /// public key is not validated against this list. `trust_anchors` is the
    /// list of root CAs to trust. `intermediate_certs` is the sequence of
    /// intermediate certificates that the client sent in the TLS handshake.
    /// `cert` is the purported end-entity certificate of the client. `time` is
    /// the time for which the validation is effective (usually the current
    /// time).
    #[allow(deprecated)]
    #[deprecated(
        since = "0.101.2",
        note = "The per-usage trust anchor representations and verification functions are deprecated in \
        favor of the general-purpose `TrustAnchor` type and `EndEntity::verify_for_usage` function. \
        The new `verify_for_usage` function expresses trust anchor and end entity purpose with the \
        key usage argument."
    )]
    pub fn verify_is_valid_tls_client_cert(
        &self,
        supported_sig_algs: &[&SignatureAlgorithm],
        &TlsClientTrustAnchors(trust_anchors): &TlsClientTrustAnchors,
        intermediate_certs: &[&[u8]],
        time: Time,
        crls: &[&dyn CertRevocationList],
    ) -> Result<(), Error> {
        self.verify_is_valid_cert(
            supported_sig_algs,
            trust_anchors,
            intermediate_certs,
            time,
            KeyUsage::client_auth(),
            crls,
        )
    }

    /// Verifies that the certificate is valid for the given Subject Name.
    pub fn verify_is_valid_for_subject_name(
        &self,
        subject_name: SubjectNameRef,
    ) -> Result<(), Error> {
        subject_name::verify_cert_subject_name(self, subject_name)
    }

    /// Verifies the signature `signature` of message `msg` using the
    /// certificate's public key.
    ///
    /// `signature_alg` is the algorithm to use to
    /// verify the signature; the certificate's public key is verified to be
    /// compatible with this algorithm.
    ///
    /// For TLS 1.2, `signature` corresponds to TLS's
    /// `DigitallySigned.signature` and `signature_alg` corresponds to TLS's
    /// `DigitallySigned.algorithm` of TLS type `SignatureAndHashAlgorithm`. In
    /// TLS 1.2 a single `SignatureAndHashAlgorithm` may map to multiple
    /// `SignatureAlgorithm`s. For example, a TLS 1.2
    /// `SignatureAndHashAlgorithm` of (ECDSA, SHA-256) may map to any or all
    /// of {`ECDSA_P256_SHA256`, `ECDSA_P384_SHA256`}, depending on how the TLS
    /// implementation is configured.
    ///
    /// For current TLS 1.3 drafts, `signature_alg` corresponds to TLS's
    /// `algorithm` fields of type `SignatureScheme`. There is (currently) a
    /// one-to-one correspondence between TLS 1.3's `SignatureScheme` and
    /// `SignatureAlgorithm`.
    pub fn verify_signature(
        &self,
        signature_alg: &SignatureAlgorithm,
        msg: &[u8],
        signature: &[u8],
    ) -> Result<(), Error> {
        signed_data::verify_signature(
            signature_alg,
            self.inner.spki.value(),
            untrusted::Input::from(msg),
            untrusted::Input::from(signature),
        )
    }

    /// Returns a list of the DNS names provided in the subject alternative names extension
    ///
    /// This function must not be used to implement custom DNS name verification.
    /// Verification functions are already provided as `verify_is_valid_for_dns_name`
    /// and `verify_is_valid_for_at_least_one_dns_name`.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn dns_names(&'a self) -> Result<impl Iterator<Item = GeneralDnsNameRef<'a>>, Error> {
        subject_name::list_cert_dns_names(self)
    }
}

#[cfg(feature = "alloc")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    // This test reproduces https://github.com/rustls/webpki/issues/167 --- an
    // end-entity cert where the common name is a `PrintableString` rather than
    // a `UTF8String` cannot iterate over its subject alternative names.
    #[test]
    fn printable_string_common_name() {
        const DNS_NAME: &str = "test.example.com";

        let issuer = test_utils::make_issuer("Test", None);

        let ee_cert_der = {
            let mut params = rcgen::CertificateParams::new(vec![DNS_NAME.to_string()]);
            // construct a certificate that uses `PrintableString` as the
            // common name value, rather than `UTF8String`.
            params.distinguished_name.push(
                rcgen::DnType::CommonName,
                rcgen::DnValue::PrintableString("example.com".to_string()),
            );
            params.is_ca = rcgen::IsCa::ExplicitNoCa;
            params.alg = test_utils::RCGEN_SIGNATURE_ALG;
            let cert = rcgen::Certificate::from_params(params)
                .expect("failed to make ee cert (this is a test bug)");
            cert.serialize_der_with_signer(&issuer)
                .expect("failed to serialize signed ee cert (this is a test bug)")
        };

        expect_dns_name(&ee_cert_der, DNS_NAME);
    }

    // This test reproduces https://github.com/rustls/webpki/issues/167 --- an
    // end-entity cert where the common name is an empty SEQUENCE.
    #[test]
    fn empty_sequence_common_name() {
        // handcrafted cert DER produced using `ascii2der`, since `rcgen` is
        // unwilling to generate this particular weird cert.
        let ee_cert_der = include_bytes!("../tests/misc/empty_sequence_common_name.der").as_slice();
        expect_dns_name(ee_cert_der, "example.com");
    }

    fn expect_dns_name(der: &[u8], name: &str) {
        let cert =
            EndEntityCert::try_from(der).expect("should parse end entity certificate correctly");

        let mut names = cert
            .dns_names()
            .expect("should get all DNS names correctly for end entity cert");
        assert_eq!(names.next().map(<&str>::from), Some(name));
        assert_eq!(names.next().map(<&str>::from), None);
    }
}
