// Copyright 2015 Brian Smith.
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

use core::default::Default;
use core::ops::ControlFlow;

use crate::{
    cert::{Cert, EndEntityOrCa},
    der, public_values_eq, signed_data, subject_name, time, CertRevocationList, Error,
    SignatureAlgorithm, TrustAnchor,
};

pub(crate) struct ChainOptions<'a> {
    pub(crate) eku: KeyUsage,
    pub(crate) supported_sig_algs: &'a [&'a SignatureAlgorithm],
    pub(crate) trust_anchors: &'a [TrustAnchor<'a>],
    pub(crate) intermediate_certs: &'a [&'a [u8]],
    pub(crate) crls: &'a [&'a dyn CertRevocationList],
}

pub(crate) fn build_chain(opts: &ChainOptions, cert: &Cert, time: time::Time) -> Result<(), Error> {
    build_chain_inner(opts, cert, time, 0, &mut Budget::default()).map_err(|e| match e {
        ControlFlow::Break(err) => err,
        ControlFlow::Continue(err) => err,
    })
}

fn build_chain_inner(
    opts: &ChainOptions,
    cert: &Cert,
    time: time::Time,
    sub_ca_count: usize,
    budget: &mut Budget,
) -> Result<(), ControlFlow<Error, Error>> {
    let used_as_ca = used_as_ca(&cert.ee_or_ca);

    check_issuer_independent_properties(cert, time, used_as_ca, sub_ca_count, opts.eku.inner)?;

    // TODO: HPKP checks.

    match used_as_ca {
        UsedAsCa::Yes => {
            const MAX_SUB_CA_COUNT: usize = 6;

            if sub_ca_count >= MAX_SUB_CA_COUNT {
                return Err(Error::MaximumPathDepthExceeded.into());
            }
        }
        UsedAsCa::No => {
            assert_eq!(0, sub_ca_count);
        }
    }

    let result = loop_while_non_fatal_error(
        Error::UnknownIssuer,
        opts.trust_anchors,
        |trust_anchor: &TrustAnchor| {
            let trust_anchor_subject = untrusted::Input::from(trust_anchor.subject);
            if !public_values_eq(cert.issuer, trust_anchor_subject) {
                return Err(Error::UnknownIssuer.into());
            }

            // TODO: check_distrust(trust_anchor_subject, trust_anchor_spki)?;

            check_signed_chain(
                opts.supported_sig_algs,
                cert,
                trust_anchor,
                opts.crls,
                budget,
            )?;

            check_signed_chain_name_constraints(cert, trust_anchor, budget)?;

            Ok(())
        },
    );

    let err = match result {
        Ok(()) => return Ok(()),
        // Fatal errors should halt further path building.
        res @ Err(ControlFlow::Break(_)) => return res,
        // Non-fatal errors should be carried forward as the default_error for subsequent
        // loop_while_non_fatal_error processing and only returned once all other path-building
        // options have been exhausted.
        Err(ControlFlow::Continue(err)) => err,
    };

    loop_while_non_fatal_error(err, opts.intermediate_certs, |cert_der| {
        let potential_issuer =
            Cert::from_der(untrusted::Input::from(cert_der), EndEntityOrCa::Ca(cert))?;

        if !public_values_eq(potential_issuer.subject, cert.issuer) {
            return Err(Error::UnknownIssuer.into());
        }

        // Prevent loops; see RFC 4158 section 5.2.
        let mut prev = cert;
        loop {
            if public_values_eq(potential_issuer.spki.value(), prev.spki.value())
                && public_values_eq(potential_issuer.subject, prev.subject)
            {
                return Err(Error::UnknownIssuer.into());
            }
            match &prev.ee_or_ca {
                EndEntityOrCa::EndEntity => {
                    break;
                }
                EndEntityOrCa::Ca(child_cert) => {
                    prev = child_cert;
                }
            }
        }

        let next_sub_ca_count = match used_as_ca {
            UsedAsCa::No => sub_ca_count,
            UsedAsCa::Yes => sub_ca_count + 1,
        };

        budget.consume_build_chain_call()?;
        build_chain_inner(opts, &potential_issuer, time, next_sub_ca_count, budget)
    })
}

fn check_signed_chain(
    supported_sig_algs: &[&SignatureAlgorithm],
    cert_chain: &Cert,
    trust_anchor: &TrustAnchor,
    crls: &[&dyn CertRevocationList],
    budget: &mut Budget,
) -> Result<(), ControlFlow<Error, Error>> {
    let mut spki_value = untrusted::Input::from(trust_anchor.spki);
    let mut issuer_subject = untrusted::Input::from(trust_anchor.subject);
    let mut issuer_key_usage = None; // TODO(XXX): Consider whether to track TrustAnchor KU.
    let mut cert = cert_chain;
    loop {
        signed_data::verify_signed_data(supported_sig_algs, spki_value, &cert.signed_data, budget)?;

        if !crls.is_empty() {
            check_crls(
                supported_sig_algs,
                cert,
                issuer_subject,
                spki_value,
                issuer_key_usage,
                crls,
                budget,
            )?;
        }

        match &cert.ee_or_ca {
            EndEntityOrCa::Ca(child_cert) => {
                spki_value = cert.spki.value();
                issuer_subject = cert.subject;
                issuer_key_usage = cert.key_usage;
                cert = child_cert;
            }
            EndEntityOrCa::EndEntity => {
                break;
            }
        }
    }

    Ok(())
}

fn check_signed_chain_name_constraints(
    cert_chain: &Cert,
    trust_anchor: &TrustAnchor,
    budget: &mut Budget,
) -> Result<(), ControlFlow<Error, Error>> {
    let mut cert = cert_chain;
    let mut name_constraints = trust_anchor
        .name_constraints
        .as_ref()
        .map(|der| untrusted::Input::from(der));

    loop {
        untrusted::read_all_optional(name_constraints, Error::BadDer, |value| {
            subject_name::check_name_constraints(value, cert, budget)
        })?;

        match &cert.ee_or_ca {
            EndEntityOrCa::Ca(child_cert) => {
                name_constraints = cert.name_constraints;
                cert = child_cert;
            }
            EndEntityOrCa::EndEntity => {
                break;
            }
        }
    }

    Ok(())
}

pub(crate) struct Budget {
    signatures: usize,
    build_chain_calls: usize,
    name_constraint_comparisons: usize,
}

impl Budget {
    #[inline]
    pub(crate) fn consume_signature(&mut self) -> Result<(), Error> {
        self.signatures = self
            .signatures
            .checked_sub(1)
            .ok_or(Error::MaximumSignatureChecksExceeded)?;
        Ok(())
    }

    #[inline]
    fn consume_build_chain_call(&mut self) -> Result<(), Error> {
        self.build_chain_calls = self
            .build_chain_calls
            .checked_sub(1)
            .ok_or(Error::MaximumPathBuildCallsExceeded)?;
        Ok(())
    }

    #[inline]
    pub(crate) fn consume_name_constraint_comparison(&mut self) -> Result<(), Error> {
        self.name_constraint_comparisons = self
            .name_constraint_comparisons
            .checked_sub(1)
            .ok_or(Error::MaximumNameConstraintComparisonsExceeded)?;
        Ok(())
    }
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            // This limit is taken from the remediation for golang CVE-2018-16875.  However,
            // note that golang subsequently implemented AKID matching due to this limit
            // being hit in real applications (see <https://github.com/spiffe/spire/issues/1004>).
            // So this may actually be too aggressive.
            signatures: 100,

            // This limit is taken from NSS libmozpkix, see:
            // <https://github.com/nss-dev/nss/blob/bb4a1d38dd9e92923525ac6b5ed0288479f3f3fc/lib/mozpkix/lib/pkixbuild.cpp#L381-L393>
            build_chain_calls: 200_000,

            // This limit is taken from golang crypto/x509's default, see:
            // <https://github.com/golang/go/blob/ac17bb6f13979f2ab9fcd45f0758b43ed72d0973/src/crypto/x509/verify.go#L588-L592>
            name_constraint_comparisons: 250_000,
        }
    }
}

// Zero-sized marker type representing positive assertion that revocation status was checked
// for a certificate and the result was that the certificate is not revoked.
struct CertNotRevoked(());

impl CertNotRevoked {
    // Construct a CertNotRevoked marker.
    fn assertion() -> Self {
        Self(())
    }
}

fn check_crls(
    supported_sig_algs: &[&SignatureAlgorithm],
    cert: &Cert,
    issuer_subject: untrusted::Input,
    issuer_spki: untrusted::Input,
    issuer_ku: Option<untrusted::Input>,
    crls: &[&dyn CertRevocationList],
    budget: &mut Budget,
) -> Result<Option<CertNotRevoked>, Error> {
    assert!(public_values_eq(cert.issuer, issuer_subject));

    let crl = match crls
        .iter()
        .find(|candidate_crl| candidate_crl.issuer() == cert.issuer())
    {
        Some(crl) => crl,
        None => return Ok(None),
    };

    // Verify the CRL signature with the issuer SPKI.
    // TODO(XXX): consider whether we can refactor so this happens once up-front, instead
    //            of per-lookup.
    //            https://github.com/rustls/webpki/issues/81
    // Note: The `verify_signature` method is part of a public trait in the exported API.
    //       We can't add a budget argument to that fn in a semver compatible way and so must
    //       consume signature budget here before calling verify_signature.
    budget.consume_signature()?;
    crl.verify_signature(supported_sig_algs, issuer_spki.as_slice_less_safe())
        .map_err(crl_signature_err)?;

    // Verify that if the issuer has a KeyUsage bitstring it asserts cRLSign.
    KeyUsageMode::CrlSign.check(issuer_ku)?;

    // Try to find the cert serial in the verified CRL contents.
    let cert_serial = cert.serial.as_slice_less_safe();
    match crl.find_serial(cert_serial)? {
        None => Ok(Some(CertNotRevoked::assertion())),
        Some(_) => Err(Error::CertRevoked),
    }
}

// When verifying CRL signed data we want to disambiguate the context of possible errors by mapping
// them to CRL specific variants that a consumer can use to tell the issue was with the CRL's
// signature, not a certificate.
fn crl_signature_err(err: Error) -> Error {
    match err {
        Error::UnsupportedSignatureAlgorithm => Error::UnsupportedCrlSignatureAlgorithm,
        Error::UnsupportedSignatureAlgorithmForPublicKey => {
            Error::UnsupportedCrlSignatureAlgorithmForPublicKey
        }
        Error::InvalidSignatureForPublicKey => Error::InvalidCrlSignatureForPublicKey,
        _ => err,
    }
}

fn check_issuer_independent_properties(
    cert: &Cert,
    time: time::Time,
    used_as_ca: UsedAsCa,
    sub_ca_count: usize,
    eku: ExtendedKeyUsage,
) -> Result<(), Error> {
    // TODO: check_distrust(trust_anchor_subject, trust_anchor_spki)?;
    // TODO: Check signature algorithm like mozilla::pkix.
    // TODO: Check SPKI like mozilla::pkix.
    // TODO: check for active distrust like mozilla::pkix.

    // For cert validation, we ignore the KeyUsage extension. For CA
    // certificates, BasicConstraints.cA makes KeyUsage redundant. Firefox
    // and other common browsers do not check KeyUsage for end-entities,
    // though it would be kind of nice to ensure that a KeyUsage without
    // the keyEncipherment bit could not be used for RSA key exchange.

    cert.validity
        .read_all(Error::BadDer, |value| check_validity(value, time))?;
    untrusted::read_all_optional(cert.basic_constraints, Error::BadDer, |value| {
        check_basic_constraints(value, used_as_ca, sub_ca_count)
    })?;
    untrusted::read_all_optional(cert.eku, Error::BadDer, |value| eku.check(value))?;

    Ok(())
}

// https://tools.ietf.org/html/rfc5280#section-4.1.2.5
fn check_validity(input: &mut untrusted::Reader, time: time::Time) -> Result<(), Error> {
    let not_before = der::time_choice(input)?;
    let not_after = der::time_choice(input)?;

    if not_before > not_after {
        return Err(Error::InvalidCertValidity);
    }
    if time < not_before {
        return Err(Error::CertNotValidYet);
    }
    if time > not_after {
        return Err(Error::CertExpired);
    }

    // TODO: mozilla::pkix allows the TrustDomain to check not_before and
    // not_after, to enforce things like a maximum validity period. We should
    // do something similar.

    Ok(())
}

#[derive(Clone, Copy, PartialEq)]
enum UsedAsCa {
    Yes,
    No,
}

fn used_as_ca(ee_or_ca: &EndEntityOrCa) -> UsedAsCa {
    match ee_or_ca {
        EndEntityOrCa::EndEntity => UsedAsCa::No,
        EndEntityOrCa::Ca(..) => UsedAsCa::Yes,
    }
}

// https://tools.ietf.org/html/rfc5280#section-4.2.1.9
fn check_basic_constraints(
    input: Option<&mut untrusted::Reader>,
    used_as_ca: UsedAsCa,
    sub_ca_count: usize,
) -> Result<(), Error> {
    let (is_ca, path_len_constraint) = match input {
        Some(input) => {
            let is_ca = der::optional_boolean(input)?;

            // https://bugzilla.mozilla.org/show_bug.cgi?id=985025: RFC 5280
            // says that a certificate must not have pathLenConstraint unless
            // it is a CA certificate, but some real-world end-entity
            // certificates have pathLenConstraint.
            let path_len_constraint = if !input.at_end() {
                let value = der::small_nonnegative_integer(input)?;
                Some(usize::from(value))
            } else {
                None
            };

            (is_ca, path_len_constraint)
        }
        None => (false, None),
    };

    match (used_as_ca, is_ca, path_len_constraint) {
        (UsedAsCa::No, true, _) => Err(Error::CaUsedAsEndEntity),
        (UsedAsCa::Yes, false, _) => Err(Error::EndEntityUsedAsCa),
        (UsedAsCa::Yes, true, Some(len)) if sub_ca_count > len => {
            Err(Error::PathLenConstraintViolated)
        }
        _ => Ok(()),
    }
}

/// The expected key usage of a certificate.
///
/// This type represents the expected key usage of an end entity certificate. Although for most
/// kinds of certificates the extended key usage extension is optional (and so certificates
/// not carrying a particular value in the EKU extension are acceptable). If the extension
/// is present, the certificate MUST only be used for one of the purposes indicated.
///
/// <https://www.rfc-editor.org/rfc/rfc5280#section-4.2.1.12>
#[derive(Clone, Copy)]
pub struct KeyUsage {
    inner: ExtendedKeyUsage,
}

impl KeyUsage {
    /// Construct a new [`KeyUsage`] as appropriate for server certificate authentication.
    ///
    /// As specified in <https://www.rfc-editor.org/rfc/rfc5280#section-4.2.1.12>, this does not require the certificate to specify the eKU extension.
    pub const fn server_auth() -> Self {
        Self {
            inner: ExtendedKeyUsage::RequiredIfPresent(EKU_SERVER_AUTH),
        }
    }

    /// Construct a new [`KeyUsage`] as appropriate for client certificate authentication.
    ///
    /// As specified in <>, this does not require the certificate to specify the eKU extension.
    pub const fn client_auth() -> Self {
        Self {
            inner: ExtendedKeyUsage::RequiredIfPresent(EKU_CLIENT_AUTH),
        }
    }

    /// Construct a new [`KeyUsage`] requiring a certificate to support the specified OID.
    pub const fn required(oid: &'static [u8]) -> Self {
        Self {
            inner: ExtendedKeyUsage::Required(KeyPurposeId::new(oid)),
        }
    }
}

/// Extended Key Usage (EKU) of a certificate.
#[derive(Clone, Copy)]
enum ExtendedKeyUsage {
    /// The certificate must contain the specified [`KeyPurposeId`] as EKU.
    Required(KeyPurposeId),

    /// If the certificate has EKUs, then the specified [`KeyPurposeId`] must be included.
    RequiredIfPresent(KeyPurposeId),
}

impl ExtendedKeyUsage {
    // https://tools.ietf.org/html/rfc5280#section-4.2.1.12
    fn check(&self, input: Option<&mut untrusted::Reader>) -> Result<(), Error> {
        let input = match (input, self) {
            (Some(input), _) => input,
            (None, Self::RequiredIfPresent(_)) => return Ok(()),
            (None, Self::Required(_)) => return Err(Error::RequiredEkuNotFound),
        };

        loop {
            let value = der::expect_tag_and_get_value(input, der::Tag::OID)?;
            if self.key_purpose_id_equals(value) {
                input.skip_to_end();
                break;
            }

            if input.at_end() {
                return Err(Error::RequiredEkuNotFound);
            }
        }

        Ok(())
    }

    fn key_purpose_id_equals(&self, value: untrusted::Input<'_>) -> bool {
        public_values_eq(
            match self {
                ExtendedKeyUsage::Required(eku) => *eku,
                ExtendedKeyUsage::RequiredIfPresent(eku) => *eku,
            }
            .oid_value,
            value,
        )
    }
}

/// An OID value indicating an Extended Key Usage (EKU) key purpose.
#[derive(Clone, Copy)]
struct KeyPurposeId {
    oid_value: untrusted::Input<'static>,
}

impl KeyPurposeId {
    /// Construct a new [`KeyPurposeId`].
    ///
    /// `oid` is the OBJECT IDENTIFIER in bytes.
    const fn new(oid: &'static [u8]) -> Self {
        Self {
            oid_value: untrusted::Input::from(oid),
        }
    }
}

impl PartialEq<Self> for KeyPurposeId {
    fn eq(&self, other: &Self) -> bool {
        public_values_eq(self.oid_value, other.oid_value)
    }
}

impl Eq for KeyPurposeId {}

// id-pkix            OBJECT IDENTIFIER ::= { 1 3 6 1 5 5 7 }
// id-kp              OBJECT IDENTIFIER ::= { id-pkix 3 }

// id-kp-serverAuth   OBJECT IDENTIFIER ::= { id-kp 1 }
#[allow(clippy::identity_op)] // TODO: Make this clearer
const EKU_SERVER_AUTH: KeyPurposeId = KeyPurposeId::new(&[(40 * 1) + 3, 6, 1, 5, 5, 7, 3, 1]);

// id-kp-clientAuth   OBJECT IDENTIFIER ::= { id-kp 2 }
#[allow(clippy::identity_op)] // TODO: Make this clearer
const EKU_CLIENT_AUTH: KeyPurposeId = KeyPurposeId::new(&[(40 * 1) + 3, 6, 1, 5, 5, 7, 3, 2]);

// https://www.rfc-editor.org/rfc/rfc5280#section-4.2.1.3
#[repr(u8)]
#[derive(Clone, Copy)]
enum KeyUsageMode {
    // DigitalSignature = 0,
    // ContentCommitment = 1,
    // KeyEncipherment = 2,
    // DataEncipherment = 3,
    // KeyAgreement = 4,
    // CertSign = 5,
    CrlSign = 6,
    // EncipherOnly = 7,
    // DecipherOnly = 8,
}

impl KeyUsageMode {
    // https://www.rfc-editor.org/rfc/rfc5280#section-4.2.1.3
    fn check(self, input: Option<untrusted::Input>) -> Result<(), Error> {
        let bit_string = match input {
            Some(input) => input,
            // While RFC 5280 requires KeyUsage be present, historically the absence of a KeyUsage
            // has been treated as "Any Usage". We follow that convention here and assume the absence
            // of KeyUsage implies the required_ku_bit_if_present we're checking for.
            None => return Ok(()),
        };

        let flags = der::bit_string_flags(&mut untrusted::Reader::new(bit_string))?;
        #[allow(clippy::as_conversions)] // u8 always fits in usize.
        match flags.bit_set(self as usize) {
            true => Ok(()),
            false => Err(Error::IssuerNotCrlSigner),
        }
    }
}

fn loop_while_non_fatal_error<V>(
    default_error: Error,
    values: V,
    mut f: impl FnMut(V::Item) -> Result<(), ControlFlow<Error, Error>>,
) -> Result<(), ControlFlow<Error, Error>>
where
    V: IntoIterator,
{
    let mut error = default_error;
    for v in values {
        match f(v) {
            Ok(()) => return Ok(()),
            // Fatal errors should halt further looping.
            res @ Err(ControlFlow::Break(_)) => return res,
            // Non-fatal errors should be ranked by specificity and only returned
            // once all other path-building options have been exhausted.
            Err(ControlFlow::Continue(new_error)) => error = error.most_specific(new_error),
        }
    }
    Err(error.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use crate::test_utils::{make_end_entity, make_issuer};

    #[test]
    fn eku_key_purpose_id() {
        assert!(ExtendedKeyUsage::RequiredIfPresent(EKU_SERVER_AUTH)
            .key_purpose_id_equals(EKU_SERVER_AUTH.oid_value))
    }

    #[cfg(feature = "alloc")]
    enum TrustAnchorIsActualIssuer {
        Yes,
        No,
    }

    #[cfg(feature = "alloc")]
    fn build_degenerate_chain(
        intermediate_count: usize,
        trust_anchor_is_actual_issuer: TrustAnchorIsActualIssuer,
        budget: Option<Budget>,
    ) -> ControlFlow<Error, Error> {
        let ca_cert = make_issuer("Bogus Subject", None);
        let ca_cert_der = ca_cert.serialize_der().unwrap();

        let mut intermediates = Vec::with_capacity(intermediate_count);
        let mut issuer = ca_cert;
        for _ in 0..intermediate_count {
            let intermediate = make_issuer("Bogus Subject", None);
            let intermediate_der = intermediate.serialize_der_with_signer(&issuer).unwrap();
            intermediates.push(intermediate_der);
            issuer = intermediate;
        }

        if let TrustAnchorIsActualIssuer::No = trust_anchor_is_actual_issuer {
            intermediates.pop();
        }

        verify_chain(
            &ca_cert_der,
            &intermediates,
            &make_end_entity(&issuer),
            budget,
        )
        .unwrap_err()
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_too_many_signatures() {
        assert!(matches!(
            build_degenerate_chain(5, TrustAnchorIsActualIssuer::Yes, None),
            ControlFlow::Break(Error::MaximumSignatureChecksExceeded)
        ));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_too_many_path_calls() {
        assert!(matches!(
            build_degenerate_chain(
                10,
                TrustAnchorIsActualIssuer::No,
                Some(Budget {
                    // Crafting a chain that will expend the build chain calls budget without
                    // first expending the signature checks budget is tricky, so we artificially
                    // inflate the signature limit to make this test easier to write.
                    signatures: usize::MAX,
                    ..Budget::default()
                })
            ),
            ControlFlow::Break(Error::MaximumPathBuildCallsExceeded)
        ));
    }

    #[cfg(feature = "alloc")]
    fn build_linear_chain(chain_length: usize) -> Result<(), ControlFlow<Error, Error>> {
        let ca_cert = make_issuer(format!("Bogus Subject {chain_length}"), None);
        let ca_cert_der = ca_cert.serialize_der().unwrap();

        let mut intermediates = Vec::with_capacity(chain_length);
        let mut issuer = ca_cert;
        for i in 0..chain_length {
            let intermediate = make_issuer(format!("Bogus Subject {i}"), None);
            let intermediate_der = intermediate.serialize_der_with_signer(&issuer).unwrap();
            intermediates.push(intermediate_der);
            issuer = intermediate;
        }

        verify_chain(
            &ca_cert_der,
            &intermediates,
            &make_end_entity(&issuer),
            None,
        )
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn longest_allowed_path() {
        assert!(build_linear_chain(1).is_ok());
        assert!(build_linear_chain(2).is_ok());
        assert!(build_linear_chain(3).is_ok());
        assert!(build_linear_chain(4).is_ok());
        assert!(build_linear_chain(5).is_ok());
        assert!(build_linear_chain(6).is_ok());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn path_too_long() {
        assert!(matches!(
            build_linear_chain(7),
            Err(ControlFlow::Continue(Error::MaximumPathDepthExceeded))
        ));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn name_constraint_budget() {
        // Issue a trust anchor that imposes name constraints. The constraint should match
        // the end entity certificate SAN.
        let ca_cert = make_issuer(
            "Constrained Root",
            Some(rcgen::NameConstraints {
                permitted_subtrees: vec![rcgen::GeneralSubtree::DnsName(".com".into())],
                excluded_subtrees: vec![],
            }),
        );
        let ca_cert_der = ca_cert.serialize_der().unwrap();

        // Create a series of intermediate issuers. We'll only use one in the actual built path,
        // helping demonstrate that the name constraint budget is not expended checking certificates
        // that are not part of the path we compute.
        const NUM_INTERMEDIATES: usize = 5;
        let mut intermediates = Vec::with_capacity(NUM_INTERMEDIATES);
        for i in 0..NUM_INTERMEDIATES {
            intermediates.push(make_issuer(format!("Intermediate {i}"), None));
        }

        // Each intermediate should be issued by the trust anchor.
        let mut intermediates_der = Vec::with_capacity(NUM_INTERMEDIATES);
        for intermediate in &intermediates {
            intermediates_der.push(intermediate.serialize_der_with_signer(&ca_cert).unwrap());
        }

        // Create an end-entity cert that is issued by the last of the intermediates.
        let ee_cert = make_end_entity(intermediates.last().unwrap());

        // We use a custom budget to make it easier to write a test, otherwise it is tricky to
        // stuff enough names/constraints into the potential chains while staying within the path
        // depth limit and the build chain call limit.
        let passing_budget = Budget {
            // One comparison against the intermediate's distinguished name.
            // One comparison against the EE's distinguished name.
            // One comparison against the EE's SAN.
            //  = 3 total comparisons.
            name_constraint_comparisons: 3,
            ..Budget::default()
        };

        // Validation should succeed with the name constraint comparison budget allocated above.
        // This shows that we're not consuming budget on unused intermediates: we didn't budget
        // enough comparisons for that to pass the overall chain building.
        assert!(verify_chain(
            &ca_cert_der,
            &intermediates_der,
            &ee_cert,
            Some(passing_budget),
        )
        .is_ok());

        let failing_budget = Budget {
            // See passing_budget: 2 comparisons is not sufficient.
            name_constraint_comparisons: 2,
            ..Budget::default()
        };
        // Validation should fail when the budget is smaller than the number of comparisons performed
        // on the validated path. This demonstrates we properly fail path building when too many
        // name constraint comparisons occur.
        let result = verify_chain(
            &ca_cert_der,
            &intermediates_der,
            &ee_cert,
            Some(failing_budget),
        );

        assert!(matches!(
            result,
            Err(ControlFlow::Break(
                Error::MaximumNameConstraintComparisonsExceeded
            ))
        ));
    }

    #[cfg(feature = "alloc")]
    fn verify_chain(
        trust_anchor_der: &[u8],
        intermediates_der: &[Vec<u8>],
        ee_cert_der: &[u8],
        budget: Option<Budget>,
    ) -> Result<(), ControlFlow<Error, Error>> {
        use crate::ECDSA_P256_SHA256;
        use crate::{EndEntityCert, Time};

        let anchors = &[TrustAnchor::try_from_cert_der(trust_anchor_der).unwrap()];
        let time = Time::from_seconds_since_unix_epoch(0x1fed_f00d);
        let cert = EndEntityCert::try_from(ee_cert_der).unwrap();
        let intermediates_der = intermediates_der
            .iter()
            .map(|x| x.as_ref())
            .collect::<Vec<_>>();

        build_chain_inner(
            &ChainOptions {
                eku: KeyUsage::server_auth(),
                supported_sig_algs: &[&ECDSA_P256_SHA256],
                trust_anchors: anchors,
                intermediate_certs: &intermediates_der,
                crls: &[],
            },
            cert.inner(),
            time,
            0,
            &mut budget.unwrap_or_default(),
        )
    }
}
