use pki_types::{CertificateDer, TrustAnchor};

use crate::cert::{Cert, lenient_certificate_serial_number};
use crate::der;
use crate::error::{DerTypeId, Error};

/// Interprets the given pre-validated DER-encoded certificate as a `TrustAnchor`.
///
/// This function extracts the components of a trust anchor (see [RFC 5280 6.1.1]) from
/// an X.509 certificate obtained from a source trusted to have appropriately validated
/// the subject name, public key, and name constraints in the certificate, for example your
/// operating system's trust store.
///
/// No additional checks on the content of the certificate, including whether it is self-signed,
/// or has a basic constraints extension indicating the `cA` boolean is true, will be performed.
/// [RFC 5280 6.2] notes:
/// > Implementations that use self-signed certificates to specify trust
/// > anchor information are free to process or ignore such information.
///
/// This function is intended for users constructing `TrustAnchor`'s from existing trust stores
/// that express trust anchors as X.509 certificates. It should **not** be used to treat an
/// end-entity certificate as a `TrustAnchor` in an effort to validate the same end-entity
/// certificate during path building. Webpki has no support for self-signed certificates.
///
/// [RFC 5280 6.1.1]: <https://datatracker.ietf.org/doc/html/rfc5280#section-6.1.1>
/// [RFC 5280 6.2]: <https://www.rfc-editor.org/rfc/rfc5280#section-6.2>
pub fn anchor_from_trusted_cert<'a>(
    cert: &'a CertificateDer<'a>,
) -> Result<TrustAnchor<'a>, Error> {
    let cert_der = untrusted::Input::from(cert.as_ref());

    // v1 certificates will result in `Error::BadDer` because `parse_cert` will
    // expect a version field that isn't there. In that case, try to parse the
    // certificate using a special parser for v1 certificates. Notably, that
    // parser doesn't allow extensions, so there's no need to worry about
    // embedded name constraints in a v1 certificate.
    match Cert::for_trust_anchor(cert_der) {
        Ok(cert) => Ok(TrustAnchor::from(cert)),
        Err(Error::UnsupportedCertVersion) => {
            extract_trust_anchor_from_v1_cert_der(cert_der).or(Err(Error::BadDer))
        }
        Err(err) => Err(err),
    }
}

/// Parses a v1 certificate directly into a TrustAnchor.
fn extract_trust_anchor_from_v1_cert_der(
    cert_der: untrusted::Input<'_>,
) -> Result<TrustAnchor<'_>, Error> {
    // X.509 Certificate: https://tools.ietf.org/html/rfc5280#section-4.1.
    cert_der.read_all(Error::BadDer, |cert_der| {
        der::nested(
            cert_der,
            der::Tag::Sequence,
            Error::TrailingData(DerTypeId::TrustAnchorV1),
            |cert_der| {
                let anchor = der::nested(
                    cert_der,
                    der::Tag::Sequence,
                    Error::TrailingData(DerTypeId::TrustAnchorV1TbsCertificate),
                    |tbs| {
                        // The version number field does not appear in v1 certificates.
                        lenient_certificate_serial_number(tbs)?;

                        skip(tbs, der::Tag::Sequence)?; // signature.
                        skip(tbs, der::Tag::Sequence)?; // issuer.
                        skip(tbs, der::Tag::Sequence)?; // validity.
                        let subject = der::expect_tag(tbs, der::Tag::Sequence)?;
                        let spki = der::expect_tag(tbs, der::Tag::Sequence)?;

                        Ok(TrustAnchor {
                            subject: subject.as_slice_less_safe().into(),
                            subject_public_key_info: spki.as_slice_less_safe().into(),
                            name_constraints: None,
                        })
                    },
                );

                // read and discard signatureAlgorithm + signature
                skip(cert_der, der::Tag::Sequence)?;
                skip(cert_der, der::Tag::BitString)?;

                anchor
            },
        )
    })
}

impl<'a> From<Cert<'a>> for TrustAnchor<'a> {
    fn from(cert: Cert<'a>) -> Self {
        Self {
            subject: cert.subject.as_slice_less_safe().into(),
            subject_public_key_info: cert.spki.as_slice_less_safe().into(),
            name_constraints: cert
                .name_constraints
                .map(|nc| nc.as_slice_less_safe().into()),
        }
    }
}

fn skip(input: &mut untrusted::Reader<'_>, tag: der::Tag) -> Result<(), Error> {
    der::expect_tag(input, tag).map(|_| ())
}

#[cfg(test)]
mod tests {
    use rcgen::{CertificateParams, CustomExtension, KeyPair};

    use super::*;

    // OID 1.2.3.4 -- not under the id-ce arc, so `ExtensionOid::lookup` returns `None`.
    // This exercises the unknown-OID path in `remember_extension`.
    #[test]
    fn anchor_ignores_critical_extension_with_unknown_oid() {
        let der = cert_with_critical_extension(&[1, 2, 3, 4]);
        anchor_from_trusted_cert(&der)
            .expect("critical extension with unknown OID should be ignored for trust anchors");
    }

    // OID 2.5.29.99 -- under the id-ce arc, so `ExtensionOid::lookup` returns
    // `Some(Standard(99))`, but 99 isn't handled in `remember_cert_extension`.
    // This exercises the unsupported-extension path in `remember_cert_extension`.
    #[test]
    fn anchor_ignores_critical_extension_with_unknown_id_ce_oid() {
        let der = cert_with_critical_extension(&[2, 5, 29, 99]);
        anchor_from_trusted_cert(&der).expect(
            "critical id-ce extension with unknown OID should be ignored for trust anchors",
        );
    }

    fn cert_with_critical_extension(oid: &[u64]) -> CertificateDer<'static> {
        let mut params = CertificateParams::new(vec!["example.com".into()]).unwrap();
        let mut ext = CustomExtension::from_oid_content(oid, vec![1, 2]);
        ext.set_criticality(true);
        params.custom_extensions.push(ext);

        let key = KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256).unwrap();
        let cert = params.self_signed(&key).unwrap();
        CertificateDer::from(cert.der().to_vec())
    }
}
