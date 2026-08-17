#[cfg(feature = "logging")]
use crate::log::{debug, trace};
use crate::x509;
use crate::{key, DistinguishedName};
use crate::{CertificateError, Error};

/// A trust anchor, commonly known as a "Root Certificate."
#[derive(Debug, Clone)]
pub struct OwnedTrustAnchor {
    subject_dn_header_len: usize,
    subject_dn: DistinguishedName,
    spki: Vec<u8>,
    name_constraints: Option<Vec<u8>>,
}

impl OwnedTrustAnchor {
    /// Get a `webpki::TrustAnchor` by borrowing the owned elements.
    pub(crate) fn to_trust_anchor(&self) -> webpki::TrustAnchor {
        webpki::TrustAnchor {
            subject: &self.subject_dn.as_ref()[self.subject_dn_header_len..],
            spki: &self.spki,
            name_constraints: self.name_constraints.as_deref(),
        }
    }

    /// Constructs an `OwnedTrustAnchor` from its components.
    ///
    /// All inputs are DER-encoded.
    ///
    /// `subject` is the [Subject] field of the trust anchor *without* the outer SEQUENCE
    /// encoding.
    ///
    /// `spki` is the [SubjectPublicKeyInfo] field of the trust anchor.
    ///
    /// `name_constraints` is the [Name Constraints] to
    /// apply for this trust anchor, if any.
    ///
    /// [Subject]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.6
    /// [SubjectPublicKeyInfo]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.7
    /// [Name Constraints]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.10
    pub fn from_subject_spki_name_constraints(
        subject: impl Into<Vec<u8>>,
        spki: impl Into<Vec<u8>>,
        name_constraints: Option<impl Into<Vec<u8>>>,
    ) -> Self {
        let (subject_dn, subject_dn_header_len) = {
            let mut subject = subject.into();
            let before_len = subject.len();
            x509::wrap_in_sequence(&mut subject);
            let header_len = subject.len().saturating_sub(before_len);
            (DistinguishedName::from(subject), header_len)
        };
        Self {
            subject_dn_header_len,
            subject_dn,
            spki: spki.into(),
            name_constraints: name_constraints.map(|x| x.into()),
        }
    }

    /// Return the subject field including its outer SEQUENCE encoding.
    ///
    /// This can be decoded using [x509-parser's FromDer trait](https://docs.rs/x509-parser/latest/x509_parser/prelude/trait.FromDer.html).
    ///
    /// ```ignore
    /// use x509_parser::prelude::FromDer;
    /// println!("{}", x509_parser::x509::X509Name::from_der(anchor.subject())?.1);
    /// ```
    pub fn subject(&self) -> &DistinguishedName {
        &self.subject_dn
    }
}

/// A container for root certificates able to provide a root-of-trust
/// for connection authentication.
#[derive(Debug, Clone)]
pub struct RootCertStore {
    /// The list of roots.
    pub roots: Vec<OwnedTrustAnchor>,
}

impl RootCertStore {
    /// Make a new, empty `RootCertStore`.
    pub fn empty() -> Self {
        Self { roots: Vec::new() }
    }

    /// Return true if there are no certificates.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Say how many certificates are in the container.
    pub fn len(&self) -> usize {
        self.roots.len()
    }

    /// Add a single DER-encoded certificate to the store.
    ///
    /// This is suitable for a small set of root certificates that are expected to parse
    /// successfully. For large collections of roots (for example from a system store) it
    /// is expected that some of them might not be valid according to the rules rustls
    /// implements. As long as a relatively limited number of certificates are affected,
    /// this should not be a cause for concern. Use [`RootCertStore::add_parsable_certificates`]
    /// in order to add as many valid roots as possible and to understand how many certificates
    /// have been diagnosed as malformed.
    pub fn add(&mut self, der: &key::Certificate) -> Result<(), Error> {
        self.add_internal(&der.0)
    }

    /// Adds all the given TrustAnchors `anchors`.  This does not
    /// fail.
    pub fn add_trust_anchors(&mut self, trust_anchors: impl Iterator<Item = OwnedTrustAnchor>) {
        self.roots.extend(trust_anchors);
    }

    /// Adds all the given TrustAnchors `anchors`.  This does not
    /// fail.
    #[deprecated(since = "0.21.6", note = "Please use `add_trust_anchors` instead")]
    pub fn add_server_trust_anchors(
        &mut self,
        trust_anchors: impl Iterator<Item = OwnedTrustAnchor>,
    ) {
        self.add_trust_anchors(trust_anchors);
    }

    /// Parse the given DER-encoded certificates and add all that can be parsed
    /// in a best-effort fashion.
    ///
    /// This is because large collections of root certificates often
    /// include ancient or syntactically invalid certificates.
    ///
    /// Returns the number of certificates added, and the number that were ignored.
    pub fn add_parsable_certificates(&mut self, der_certs: &[impl AsRef<[u8]>]) -> (usize, usize) {
        let mut valid_count = 0;
        let mut invalid_count = 0;

        for der_cert in der_certs {
            #[cfg_attr(not(feature = "logging"), allow(unused_variables))]
            match self.add_internal(der_cert.as_ref()) {
                Ok(_) => valid_count += 1,
                Err(err) => {
                    trace!("invalid cert der {:?}", der_cert.as_ref());
                    debug!("certificate parsing failed: {:?}", err);
                    invalid_count += 1;
                }
            }
        }

        debug!(
            "add_parsable_certificates processed {} valid and {} invalid certs",
            valid_count, invalid_count
        );

        (valid_count, invalid_count)
    }

    fn add_internal(&mut self, der: &[u8]) -> Result<(), Error> {
        let ta = webpki::TrustAnchor::try_from_cert_der(der)
            .map_err(|_| Error::InvalidCertificate(CertificateError::BadEncoding))?;
        self.roots
            .push(OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            ));
        Ok(())
    }
}

mod tests {
    #[test]
    fn ownedtrustanchor_subject_is_correctly_encoding_dn() {
        let subject = b"subject".to_owned();
        let ota = super::OwnedTrustAnchor::from_subject_spki_name_constraints(
            subject,
            b"".to_owned(),
            None::<Vec<u8>>,
        );
        let expected_prefix = vec![ring::io::der::Tag::Sequence as u8, subject.len() as u8];
        assert_eq!(
            ota.subject().as_ref(),
            [expected_prefix, subject.to_vec()].concat()
        );
    }
}
