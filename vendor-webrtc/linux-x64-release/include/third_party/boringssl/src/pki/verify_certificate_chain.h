// Copyright 2015 The Chromium Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef BSSL_PKI_VERIFY_CERTIFICATE_CHAIN_H_
#define BSSL_PKI_VERIFY_CERTIFICATE_CHAIN_H_

#include <set>

#include <openssl/base.h>
#include <openssl/evp.h>
#include <openssl/pki/signature_verify_cache.h>

#include "cert_errors.h"
#include "input.h"
#include "parsed_certificate.h"

BSSL_NAMESPACE_BEGIN

namespace der {
struct GeneralizedTime;
}

struct CertificateTrust;

// The key purpose (extended key usage) to check for during verification.
enum class KeyPurpose {
  ANY_EKU,
  SERVER_AUTH,
  CLIENT_AUTH,
  SERVER_AUTH_STRICT,  // Skip ANY_EKU when checking, require EKU present in
                       // certificate.
  SERVER_AUTH_STRICT_LEAF,  // Same as above, but only for leaf cert.
  CLIENT_AUTH_STRICT,  // Skip ANY_EKU when checking, require EKU present in
                       // certificate.
  CLIENT_AUTH_STRICT_LEAF,  // Same as above, but only for leaf cert.
  RCS_MLS_CLIENT_AUTH,      // Client auth for RCS-MLS.
  C2PA_TIMESTAMPING,    // Leaf can sign timestamps for C2PA.
  C2PA_MANIFEST,        // Leaf can sign manifests for C2PA.
};

enum class InitialExplicitPolicy {
  kFalse,
  kTrue,
};

enum class InitialPolicyMappingInhibit {
  kFalse,
  kTrue,
};

enum class InitialAnyPolicyInhibit {
  kFalse,
  kTrue,
};

// VerifyCertificateChainDelegate exposes delegate methods used when verifying a
// chain.
class OPENSSL_EXPORT VerifyCertificateChainDelegate {
 public:
  // Implementations should return true if |signature_algorithm| is allowed for
  // certificate signing, false otherwise. When false is returned, the caller
  // will add a high severity error of kUnacceptableSignatureAlgorithm to
  // |errors|. When returning false, implementations can optionally add warnings
  // to errors to |errors| with details on why it was rejected.  Implementations
  // may add any further details on why the signature algorithm was deemed
  // unacceptable by adding warnings to |errors|.
  virtual bool IsSignatureAlgorithmAcceptable(
      SignatureAlgorithm signature_algorithm, CertErrors *errors) = 0;

  // Implementations should return true if |public_key| is acceptable, false
  // otherwise. This is called for each certificate in the chain, including the
  // target certificate.  When false is returned, the caller will add a high
  // severity error of kUnacceptablePublicKey to |errors|. When returning false,
  // implementations may add any further details on why the public key was
  // deemed unacceptable by adding warnings to |errors|.  |public_key| can be
  // assumed to be non-null.
  virtual bool IsPublicKeyAcceptable(EVP_PKEY *public_key,
                                     CertErrors *errors) = 0;

  // This is called during verification to obtain a pointer to a signature
  // verification cache if one exists. nullptr may be returned indicating there
  // is no verification cache.
  virtual SignatureVerifyCache *GetVerifyCache() = 0;

  // This is called to determine if PreCertificates should be accepted, for the
  // purpose of validating issued PreCertificates in a path. Most callers should
  // return false here. This should never return true for TLS certificate
  // validation. If this function returns true the CT precertificate poison
  // extension will not prevent the certificate from being validated.
  virtual bool AcceptPreCertificates() = 0;

  virtual ~VerifyCertificateChainDelegate();
};

// VerifyCertificateChain() verifies an ordered certificate path in accordance
// with RFC 5280's "Certification Path Validation" algorithm (section 6).
//
// -----------------------------------------
// Deviations from RFC 5280
// -----------------------------------------
//
//   * If Extended Key Usage appears on intermediates, it is treated as
//     a restriction on subordinate certificates.
//   * No revocation checking is performed.
//
// -----------------------------------------
// Additional responsibilities of the caller
// -----------------------------------------
//
// After successful path verification, the caller is responsible for
// subsequently checking:
//
//  * The end-entity's KeyUsage before using its SPKI.
//  * The end-entity's name/subjectAltName. Name constraints from intermediates
//    will have already been applied, so it is sufficient to check the
//    end-entity for a match. The caller MUST NOT check hostnames on the
//    commonName field because this implementation does not apply dnsName
//    constraints on commonName.
//
// ---------
// Inputs
// ---------
//
//   certs:
//     A non-empty chain of DER-encoded certificates, listed in the
//     "forward" direction. The first certificate is the target
//     certificate to verify, and the last certificate has trustedness
//     given by |last_cert_trust| (generally a trust anchor).
//
//      * certs[0] is the target certificate to verify.
//      * certs[i+1] holds the certificate that issued cert_chain[i].
//      * certs[N-1] the root certificate
//
//     Note that THIS IS NOT identical in meaning to the same named
//     "certs" input defined in RFC 5280 section 6.1.1.a. The differences
//     are:
//
//      * The order of certificates is reversed
//      * In RFC 5280 "certs" DOES NOT include the trust anchor
//
//   last_cert_trust:
//     Trustedness of |certs.back()|. The trustedness of |certs.back()|
//     MUST BE decided by the caller -- this function takes it purely as
//     an input. Moreover, the CertificateTrust can be used to specify
//     trust anchor constraints.
//
//     This combined with |certs.back()| (the root certificate) fills a
//     similar role to "trust anchor information" defined in RFC 5280
//     section 6.1.1.d.
//
//   delegate:
//     |delegate| must be non-null. It is used to answer policy questions such
//     as whether a signature algorithm is acceptable, or a public key is strong
//     enough.
//
//   time:
//     The UTC time to use for expiration checks. This is equivalent to
//     the input from RFC 5280 section 6.1.1:
//
//       (b)  the current date/time.
//
//   required_key_purpose:
//     The key purpose that the target certificate needs to be valid for.
//
//   user_initial_policy_set:
//     This is equivalent to the same named input in RFC 5280 section
//     6.1.1:
//
//       (c)  user-initial-policy-set: A set of certificate policy
//            identifiers naming the policies that are acceptable to the
//            certificate user. The user-initial-policy-set contains the
//            special value any-policy if the user is not concerned about
//            certificate policy.
//
//   initial_policy_mapping_inhibit:
//     This is equivalent to the same named input in RFC 5280 section
//     6.1.1:
//
//       (e)  initial-policy-mapping-inhibit, which indicates if policy
//            mapping is allowed in the certification path.
//
//   initial_explicit_policy:
//     This is equivalent to the same named input in RFC 5280 section
//     6.1.1:
//
//       (f)  initial-explicit-policy, which indicates if the path must be
//            valid for at least one of the certificate policies in the
//            user-initial-policy-set.
//
//   initial_any_policy_inhibit:
//     This is equivalent to the same named input in RFC 5280 section
//     6.1.1:
//
//       (g)  initial-any-policy-inhibit, which indicates whether the
//            anyPolicy OID should be processed if it is included in a
//            certificate.
//
// ---------
// Outputs
// ---------
//
//   user_constrained_policy_set:
//     Can be null. If non-null, |user_constrained_policy_set| will be filled
//     with the matching policies (intersected with user_initial_policy_set).
//     This is equivalent to the same named output in X.509 section 10.2.
//     Note that it is OK for this to point to input user_initial_policy_set.
//
//   errors:
//     Must be non-null. The set of errors/warnings encountered while
//     validating the path are appended to this structure. If verification
//     failed, then there is guaranteed to be at least 1 high severity error
//     written to |errors|.
//
// -------------------------
// Trust Anchor constraints
// -------------------------
//
// Conceptually, VerifyCertificateChain() sets RFC 5937's
// "enforceTrustAnchorConstraints" to true.
//
// One specifies trust anchor constraints using the |last_cert_trust|
// parameter in conjunction with extensions appearing in |certs.back()|.
//
// The trust anchor |certs.back()| is always passed as a certificate to
// this function, however the manner in which that certificate is
// interpreted depends on |last_cert_trust|:
//
// TRUSTED_ANCHOR:
//
// No properties from the root certificate, other than its Subject and
// SPKI, are checked during verification. This is the usual
// interpretation for a "trust anchor".
//
// enforce_anchor_expiry=true:
//
// The validity period of the root is checked, in addition to Subject and SPKI.
//
// enforce_anchor_constraints=true:
//
// Only a subset of extensions and properties from the certificate are checked.
// In general, constraints encoded by extensions are only enforced if the
// extension is present.
//
//  * Signature:             No
//  * Validity (expiration): No
//  * Key usage:             Yes
//  * Extended key usage:    Yes (required if required_key_purpose is STRICT)
//  * Basic constraints:     Yes
//  * Name constraints:      Yes
//  * Certificate policies:  Yes
//  * Policy Mappings:       Yes
//  * inhibitAnyPolicy:      Yes
//  * PolicyConstraints:     Yes
//
// The presence of any other unrecognized extension marked as critical fails
// validation.
OPENSSL_EXPORT void VerifyCertificateChain(
    const ParsedCertificateList &certs, const CertificateTrust &last_cert_trust,
    VerifyCertificateChainDelegate *delegate, const der::GeneralizedTime &time,
    KeyPurpose required_key_purpose,
    InitialExplicitPolicy initial_explicit_policy,
    const std::set<der::Input> &user_initial_policy_set,
    InitialPolicyMappingInhibit initial_policy_mapping_inhibit,
    InitialAnyPolicyInhibit initial_any_policy_inhibit,
    std::set<der::Input> *user_constrained_policy_set, CertPathErrors *errors);

// Returns true if `cert` is self-signed. Returns false `cert` is not
// self-signed or there was an error. If `errors` is non-null, it will contain
// additional information about the problem. If `cache` is non-null, it will be
// used to cache the signature verification step.
OPENSSL_EXPORT bool VerifyCertificateIsSelfSigned(const ParsedCertificate &cert,
                                                  SignatureVerifyCache *cache,
                                                  CertErrors *errors);

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_VERIFY_CERTIFICATE_CHAIN_H_
