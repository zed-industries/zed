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

#ifndef BSSL_PKI_PARSE_CERTIFICATE_H_
#define BSSL_PKI_PARSE_CERTIFICATE_H_

#include <stdint.h>

#include <map>
#include <memory>
#include <optional>
#include <vector>

#include <openssl/base.h>

#include "general_names.h"
#include "input.h"
#include "parse_values.h"

BSSL_NAMESPACE_BEGIN

namespace der {
class Parser;
}

class CertErrors;
struct ParsedTbsCertificate;

// Returns true if the given serial number (CertificateSerialNumber in RFC 5280)
// is valid:
//
//    CertificateSerialNumber  ::=  INTEGER
//
// The input to this function is the (unverified) value octets of the INTEGER.
// This function will verify that:
//
//   * The octets are a valid DER-encoding of an INTEGER (for instance, minimal
//     encoding length).
//
//   * No more than 20 octets are used.
//
// Note that it DOES NOT reject non-positive values (zero or negative).
//
// For reference, here is what RFC 5280 section 4.1.2.2 says:
//
//     Given the uniqueness requirements above, serial numbers can be
//     expected to contain long integers.  Certificate users MUST be able to
//     handle serialNumber values up to 20 octets.  Conforming CAs MUST NOT
//     use serialNumber values longer than 20 octets.
//
//     Note: Non-conforming CAs may issue certificates with serial numbers
//     that are negative or zero.  Certificate users SHOULD be prepared to
//     gracefully handle such certificates.
//
// |errors| must be a non-null destination for any errors/warnings. If
// |warnings_only| is set to true, then what would ordinarily be errors are
// instead added as warnings.
[[nodiscard]] OPENSSL_EXPORT bool VerifySerialNumber(der::Input value,
                                                     bool warnings_only,
                                                     CertErrors *errors);

// Consumes a "Time" value (as defined by RFC 5280) from |parser|. On success
// writes the result to |*out| and returns true. On failure no guarantees are
// made about the state of |parser|.
//
// From RFC 5280:
//
//     Time ::= CHOICE {
//          utcTime        UTCTime,
//          generalTime    GeneralizedTime }
[[nodiscard]] OPENSSL_EXPORT bool ReadUTCOrGeneralizedTime(
    der::Parser *parser, der::GeneralizedTime *out);

// Parses a DER-encoded "Validity" as specified by RFC 5280. Returns true on
// success and sets the results in |not_before| and |not_after|:
//
//       Validity ::= SEQUENCE {
//            notBefore      Time,
//            notAfter       Time }
//
// Note that upon success it is NOT guaranteed that |*not_before <= *not_after|.
[[nodiscard]] OPENSSL_EXPORT bool ParseValidity(
    der::Input validity_tlv, der::GeneralizedTime *not_before,
    der::GeneralizedTime *not_after);

struct OPENSSL_EXPORT ParseCertificateOptions {
  // If set to true, then parsing will skip checks on the certificate's serial
  // number. The only requirement will be that the serial number is an INTEGER,
  // however it is not required to be a valid DER-encoding (i.e. minimal
  // encoding), nor is it required to be constrained to any particular length.
  bool allow_invalid_serial_numbers = false;
};

// Parses a DER-encoded "Certificate" as specified by RFC 5280. Returns true on
// success and sets the results in the |out_*| parameters. On both the failure
// and success case, if |out_errors| was non-null it may contain extra error
// information.
//
// Note that on success the out parameters alias data from the input
// |certificate_tlv|.  Hence the output values are only valid as long as
// |certificate_tlv| remains valid.
//
// On failure the out parameters have an undefined state, except for
// out_errors. Some of them may have been updated during parsing, whereas
// others may not have been changed.
//
// The out parameters represent each field of the Certificate SEQUENCE:
//       Certificate  ::=  SEQUENCE  {
//
// The |out_tbs_certificate_tlv| parameter corresponds with "tbsCertificate"
// from RFC 5280:
//         tbsCertificate       TBSCertificate,
//
// This contains the full (unverified) Tag-Length-Value for a SEQUENCE. No
// guarantees are made regarding the value of this SEQUENCE.
// This can be further parsed using ParseTbsCertificate().
//
// The |out_signature_algorithm_tlv| parameter corresponds with
// "signatureAlgorithm" from RFC 5280:
//         signatureAlgorithm   AlgorithmIdentifier,
//
// This contains the full (unverified) Tag-Length-Value for a SEQUENCE. No
// guarantees are made regarding the value of this SEQUENCE.
// This can be further parsed using SignatureValue::Create().
//
// The |out_signature_value| parameter corresponds with "signatureValue" from
// RFC 5280:
//         signatureValue       BIT STRING  }
//
// Parsing guarantees that this is a valid BIT STRING.
[[nodiscard]] OPENSSL_EXPORT bool ParseCertificate(
    der::Input certificate_tlv, der::Input *out_tbs_certificate_tlv,
    der::Input *out_signature_algorithm_tlv,
    der::BitString *out_signature_value, CertErrors *out_errors);

// Parses a DER-encoded "TBSCertificate" as specified by RFC 5280. Returns true
// on success and sets the results in |out|. Certain invalid inputs may
// be accepted based on the provided |options|.
//
// If |errors| was non-null then any warnings/errors that occur during parsing
// are added to it.
//
// Note that on success |out| aliases data from the input |tbs_tlv|.
// Hence the fields of the ParsedTbsCertificate are only valid as long as
// |tbs_tlv| remains valid.
//
// On failure |out| has an undefined state. Some of its fields may have been
// updated during parsing, whereas others may not have been changed.
//
// Refer to the per-field documentation of ParsedTbsCertificate for details on
// what validity checks parsing performs.
//
//       TBSCertificate  ::=  SEQUENCE  {
//            version         [0]  EXPLICIT Version DEFAULT v1,
//            serialNumber         CertificateSerialNumber,
//            signature            AlgorithmIdentifier,
//            issuer               Name,
//            validity             Validity,
//            subject              Name,
//            subjectPublicKeyInfo SubjectPublicKeyInfo,
//            issuerUniqueID  [1]  IMPLICIT UniqueIdentifier OPTIONAL,
//                                 -- If present, version MUST be v2 or v3
//            subjectUniqueID [2]  IMPLICIT UniqueIdentifier OPTIONAL,
//                                 -- If present, version MUST be v2 or v3
//            extensions      [3]  EXPLICIT Extensions OPTIONAL
//                                 -- If present, version MUST be v3
//            }
[[nodiscard]] OPENSSL_EXPORT bool ParseTbsCertificate(
    der::Input tbs_tlv, const ParseCertificateOptions &options,
    ParsedTbsCertificate *out, CertErrors *errors);

// Represents a "Version" from RFC 5280:
//         Version  ::=  INTEGER  {  v1(0), v2(1), v3(2)  }
enum class CertificateVersion {
  V1,
  V2,
  V3,
};

// ParsedTbsCertificate contains pointers to the main fields of a DER-encoded
// RFC 5280 "TBSCertificate".
//
// ParsedTbsCertificate is expected to be filled by ParseTbsCertificate(), so
// subsequent field descriptions are in terms of what ParseTbsCertificate()
// sets.
struct OPENSSL_EXPORT ParsedTbsCertificate {
  ParsedTbsCertificate();
  ParsedTbsCertificate(ParsedTbsCertificate &&other);
  ParsedTbsCertificate &operator=(ParsedTbsCertificate &&other) = default;
  ~ParsedTbsCertificate();

  // Corresponds with "version" from RFC 5280:
  //         version         [0]  EXPLICIT Version DEFAULT v1,
  //
  // Parsing guarantees that the version is one of v1, v2, or v3.
  CertificateVersion version = CertificateVersion::V1;

  // Corresponds with "serialNumber" from RFC 5280:
  //         serialNumber         CertificateSerialNumber,
  //
  // This field specifically contains the content bytes of the INTEGER. So for
  // instance if the serial number was 1000 then this would contain bytes
  // {0x03, 0xE8}.
  //
  // The serial number may or may not be a valid DER-encoded INTEGER:
  //
  // If the option |allow_invalid_serial_numbers=true| was used during
  // parsing, then nothing further can be assumed about these bytes.
  //
  // Otherwise if |allow_invalid_serial_numbers=false| then in addition
  // to being a valid DER-encoded INTEGER, parsing guarantees that
  // the serial number is at most 20 bytes long. Parsing does NOT guarantee
  // that the integer is positive (might be zero or negative).
  der::Input serial_number;

  // Corresponds with "signatureAlgorithm" from RFC 5280:
  //         signatureAlgorithm   AlgorithmIdentifier,
  //
  // This contains the full (unverified) Tag-Length-Value for a SEQUENCE. No
  // guarantees are made regarding the value of this SEQUENCE.
  //
  // This can be further parsed using SignatureValue::Create().
  der::Input signature_algorithm_tlv;

  // Corresponds with "issuer" from RFC 5280:
  //         issuer               Name,
  //
  // This contains the full (unverified) Tag-Length-Value for a SEQUENCE. No
  // guarantees are made regarding the value of this SEQUENCE.
  der::Input issuer_tlv;

  // Corresponds with "validity" from RFC 5280:
  //         validity             Validity,
  //
  // Where Validity is defined as:
  //
  //   Validity ::= SEQUENCE {
  //        notBefore      Time,
  //        notAfter       Time }
  //
  // Parsing guarantees that notBefore (validity_not_before) and notAfter
  // (validity_not_after) are valid DER-encoded dates, however it DOES NOT
  // gurantee anything about their values. For instance notAfter could be
  // before notBefore, or the dates could indicate an expired certificate.
  // Consumers are responsible for testing expiration.
  der::GeneralizedTime validity_not_before;
  der::GeneralizedTime validity_not_after;

  // Corresponds with "subject" from RFC 5280:
  //         subject              Name,
  //
  // This contains the full (unverified) Tag-Length-Value for a SEQUENCE. No
  // guarantees are made regarding the value of this SEQUENCE.
  der::Input subject_tlv;

  // Corresponds with "subjectPublicKeyInfo" from RFC 5280:
  //         subjectPublicKeyInfo SubjectPublicKeyInfo,
  //
  // This contains the full (unverified) Tag-Length-Value for a SEQUENCE. No
  // guarantees are made regarding the value of this SEQUENCE.
  der::Input spki_tlv;

  // Corresponds with "issuerUniqueID" from RFC 5280:
  //         issuerUniqueID  [1]  IMPLICIT UniqueIdentifier OPTIONAL,
  //                              -- If present, version MUST be v2 or v3
  //
  // Parsing guarantees that if issuer_unique_id is present it is a valid BIT
  // STRING, and that the version is either v2 or v3
  std::optional<der::BitString> issuer_unique_id;

  // Corresponds with "subjectUniqueID" from RFC 5280:
  //         subjectUniqueID [2]  IMPLICIT UniqueIdentifier OPTIONAL,
  //                              -- If present, version MUST be v2 or v3
  //
  // Parsing guarantees that if subject_unique_id is present it is a valid BIT
  // STRING, and that the version is either v2 or v3
  std::optional<der::BitString> subject_unique_id;

  // Corresponds with "extensions" from RFC 5280:
  //         extensions      [3]  EXPLICIT Extensions OPTIONAL
  //                              -- If present, version MUST be v3
  //
  //
  // This contains the full (unverified) Tag-Length-Value for a SEQUENCE. No
  // guarantees are made regarding the value of this SEQUENCE. (Note that the
  // EXPLICIT outer tag is stripped.)
  //
  // Parsing guarantees that if extensions is present the version is v3.
  std::optional<der::Input> extensions_tlv;
};

// ParsedExtension represents a parsed "Extension" from RFC 5280. It contains
// der:Inputs which are not owned so the associated data must be kept alive.
//
//    Extension  ::=  SEQUENCE  {
//            extnID      OBJECT IDENTIFIER,
//            critical    BOOLEAN DEFAULT FALSE,
//            extnValue   OCTET STRING
//                        -- contains the DER encoding of an ASN.1 value
//                        -- corresponding to the extension type identified
//                        -- by extnID
//            }
struct OPENSSL_EXPORT ParsedExtension {
  der::Input oid;
  // |value| will contain the contents of the OCTET STRING. For instance for
  // basicConstraints it will be the TLV for a SEQUENCE.
  der::Input value;
  bool critical = false;
};

// Parses a DER-encoded "Extension" as specified by RFC 5280. Returns true on
// success and sets the results in |out|.
//
// Note that on success |out| aliases data from the input |extension_tlv|.
// Hence the fields of the ParsedExtension are only valid as long as
// |extension_tlv| remains valid.
//
// On failure |out| has an undefined state. Some of its fields may have been
// updated during parsing, whereas others may not have been changed.
[[nodiscard]] OPENSSL_EXPORT bool ParseExtension(der::Input extension_tlv,
                                                 ParsedExtension *out);

// From RFC 5280:
//
//     id-ce-subjectKeyIdentifier OBJECT IDENTIFIER ::=  { id-ce 14 }
//
// In dotted notation: 2.5.29.14
inline constexpr uint8_t kSubjectKeyIdentifierOid[] = {0x55, 0x1d, 0x0e};

// From RFC 5280:
//
//     id-ce-keyUsage OBJECT IDENTIFIER ::=  { id-ce 15 }
//
// In dotted notation: 2.5.29.15
inline constexpr uint8_t kKeyUsageOid[] = {0x55, 0x1d, 0x0f};

// From RFC 5280:
//
//     id-ce-subjectAltName OBJECT IDENTIFIER ::=  { id-ce 17 }
//
// In dotted notation: 2.5.29.17
inline constexpr uint8_t kSubjectAltNameOid[] = {0x55, 0x1d, 0x11};

// From RFC 5280:
//
//     id-ce-basicConstraints OBJECT IDENTIFIER ::=  { id-ce 19 }
//
// In dotted notation: 2.5.29.19
inline constexpr uint8_t kBasicConstraintsOid[] = {0x55, 0x1d, 0x13};

// From RFC 5280:
//
//     id-ce-nameConstraints OBJECT IDENTIFIER ::=  { id-ce 30 }
//
// In dotted notation: 2.5.29.30
inline constexpr uint8_t kNameConstraintsOid[] = {0x55, 0x1d, 0x1e};

// From RFC 5280:
//
//     id-ce-certificatePolicies OBJECT IDENTIFIER ::=  { id-ce 32 }
//
// In dotted notation: 2.5.29.32
inline constexpr uint8_t kCertificatePoliciesOid[] = {0x55, 0x1d, 0x20};

// From RFC 5280:
//
//     id-ce-authorityKeyIdentifier OBJECT IDENTIFIER ::=  { id-ce 35 }
//
// In dotted notation: 2.5.29.35
inline constexpr uint8_t kAuthorityKeyIdentifierOid[] = {0x55, 0x1d, 0x23};

// From RFC 5280:
//
//     id-ce-policyConstraints OBJECT IDENTIFIER ::=  { id-ce 36 }
//
// In dotted notation: 2.5.29.36
inline constexpr uint8_t kPolicyConstraintsOid[] = {0x55, 0x1d, 0x24};

// From RFC 5280:
//
//     id-ce-extKeyUsage OBJECT IDENTIFIER ::= { id-ce 37 }
//
// In dotted notation: 2.5.29.37
inline constexpr uint8_t kExtKeyUsageOid[] = {0x55, 0x1d, 0x25};

// From RFC 5280:
//
//     id-pe-authorityInfoAccess OBJECT IDENTIFIER ::= { id-pe 1 }
//
// In dotted notation: 1.3.6.1.5.5.7.1.1
inline constexpr uint8_t kAuthorityInfoAccessOid[] = {0x2B, 0x06, 0x01, 0x05,
                                                      0x05, 0x07, 0x01, 0x01};

// From RFC 5280:
//
//     id-ad-caIssuers OBJECT IDENTIFIER ::= { id-ad 2 }
//
// In dotted notation: 1.3.6.1.5.5.7.48.2
inline constexpr uint8_t kAdCaIssuersOid[] = {0x2B, 0x06, 0x01, 0x05,
                                              0x05, 0x07, 0x30, 0x02};

// From RFC 5280:
//
//     id-ad-ocsp OBJECT IDENTIFIER ::= { id-ad 1 }
//
// In dotted notation: 1.3.6.1.5.5.7.48.1
inline constexpr uint8_t kAdOcspOid[] = {0x2B, 0x06, 0x01, 0x05,
                                         0x05, 0x07, 0x30, 0x01};

// From RFC 5280:
//
//     id-ce-cRLDistributionPoints OBJECT IDENTIFIER ::=  { id-ce 31 }
//
// In dotted notation: 2.5.29.31
inline constexpr uint8_t kCrlDistributionPointsOid[] = {0x55, 0x1d, 0x1f};

// From RFC 6962:
//
// critical poison extension.
//
// In dotted notation 1.3.6.1.4.1.11129.2.4.3
inline constexpr uint8_t kCtPoisonOid[] = {0x2B, 0x06, 0x01, 0x04, 0x01,
                                           0xD6, 0x79, 0x02, 0x04, 0x03};

// From
// https://learn.microsoft.com/en-us/windows/win32/seccertenroll/supported-extensions#msapplicationpolicies
//
// OID: XCN_OID_APPLICATION_CERT_POLICIES (1.3.6.1.4.1.311.21.10)
inline constexpr uint8_t kMSApplicationPoliciesOid[] = {
    0x2b, 0x06, 0x01, 0x04, 0x01, 0x82, 0x37, 0x15, 0x0a};

// From GSMA RCC.16 v1.0 End-to-End Encryption Specification.
// id-gsmaRCSE2EE OBJECT IDENTIFIER ::=  { joint-iso-itu-t(2)
// international-organizations(23) gsma(146) rcs(2) rcsE2EE (1)}
// (Note this spec incorrectly says id-appleDraftRCSE2EE in place of
// id-gmsaRCSE2EE in several places)
//
// From GSMA RCC.16 v1.0 End-to-End Encryption Specification section A.2.8.8,
// and A.3.8.9.
// id-participantInformation OBJECT IDENTIFIER ::= { id-gmsaRCS2EE 4 }
// In dotted notation: 2.23.146.2.1.4
inline constexpr uint8_t kRcsMlsParticipantInformation[] = {0x67, 0x81, 0x12,
                                                            0x02, 0x01, 0x04};

// From GSMA RCC.16 v1.0 End-to-End Encryption Specification.
// id-gsmaRCSE2EE OBJECT IDENTIFIER ::=  { joint-iso-itu-t(2)
// international-organizations(23) gsma(146) rcs(2) rcsE2EE (1)}
// (Note this spec incorrectly says id-appleDraftRCSE2EE in place of
// id-gmsaRCSE2EE in several places)
//
// From GSMA RCC.16 v1.0 End-to-End Encryption Specification section A.2.8.8,
// and A.3.8.10.
// id-acsParticipantInformation OBJECT IDENTIFIER ::= { id-gmsaRCS2EE 5 }
// In dotted notation: 2.23.146.2.1.5
inline constexpr uint8_t kRcsMlsAcsParticipantInformation[] = {
    0x67, 0x81, 0x12, 0x02, 0x01, 0x05};

// Parses the Extensions sequence as defined by RFC 5280. Extensions are added
// to the map |extensions| keyed by the OID. Parsing guarantees that each OID
// is unique. Note that certificate verification must consume each extension
// marked as critical.
//
// Returns true on success and fills |extensions|. The output will reference
// bytes in |extensions_tlv|, so that data must be kept alive.
// On failure |extensions| may be partially written to and should not be used.
[[nodiscard]] OPENSSL_EXPORT bool ParseExtensions(
    der::Input extensions_tlv,
    std::map<der::Input, ParsedExtension> *extensions);

// Removes the extension with OID |oid| from |unconsumed_extensions| and fills
// |extension| with the matching extension value. If there was no extension
// matching |oid| then returns |false|.
[[nodiscard]] OPENSSL_EXPORT bool ConsumeExtension(
    der::Input oid,
    std::map<der::Input, ParsedExtension> *unconsumed_extensions,
    ParsedExtension *extension);

struct ParsedBasicConstraints {
  bool is_ca = false;
  bool has_path_len = false;
  uint8_t path_len = 0;
};

// Parses the BasicConstraints extension as defined by RFC 5280:
//
//    BasicConstraints ::= SEQUENCE {
//         cA                      BOOLEAN DEFAULT FALSE,
//         pathLenConstraint       INTEGER (0..MAX) OPTIONAL }
//
// The maximum allowed value of pathLenConstraints will be whatever can fit
// into a uint8_t.
[[nodiscard]] OPENSSL_EXPORT bool ParseBasicConstraints(
    der::Input basic_constraints_tlv, ParsedBasicConstraints *out);

// KeyUsageBit contains the index for a particular key usage. The index is
// measured from the most significant bit of a bit string.
//
// From RFC 5280 section 4.2.1.3:
//
//     KeyUsage ::= BIT STRING {
//          digitalSignature        (0),
//          nonRepudiation          (1), -- recent editions of X.509 have
//                               -- renamed this bit to contentCommitment
//          keyEncipherment         (2),
//          dataEncipherment        (3),
//          keyAgreement            (4),
//          keyCertSign             (5),
//          cRLSign                 (6),
//          encipherOnly            (7),
//          decipherOnly            (8) }
enum KeyUsageBit {
  KEY_USAGE_BIT_DIGITAL_SIGNATURE = 0,
  KEY_USAGE_BIT_NON_REPUDIATION = 1,
  KEY_USAGE_BIT_KEY_ENCIPHERMENT = 2,
  KEY_USAGE_BIT_DATA_ENCIPHERMENT = 3,
  KEY_USAGE_BIT_KEY_AGREEMENT = 4,
  KEY_USAGE_BIT_KEY_CERT_SIGN = 5,
  KEY_USAGE_BIT_CRL_SIGN = 6,
  KEY_USAGE_BIT_ENCIPHER_ONLY = 7,
  KEY_USAGE_BIT_DECIPHER_ONLY = 8,
};

// Parses the KeyUsage extension as defined by RFC 5280. Returns true on
// success, and |key_usage| will alias data in |key_usage_tlv|. On failure
// returns false, and |key_usage| may have been modified.
//
// In addition to validating that key_usage_tlv is a BIT STRING, this does
// additional KeyUsage specific validations such as requiring at least 1 bit to
// be set.
//
// To test if a particular key usage is set, call, e.g.:
//     key_usage->AssertsBit(KEY_USAGE_BIT_DIGITAL_SIGNATURE);
[[nodiscard]] OPENSSL_EXPORT bool ParseKeyUsage(der::Input key_usage_tlv,
                                                der::BitString *key_usage);

struct AuthorityInfoAccessDescription {
  // The accessMethod DER OID value.
  der::Input access_method_oid;
  // The accessLocation DER TLV.
  der::Input access_location;
};
// Parses the Authority Information Access extension defined by RFC 5280.
// Returns true on success, and |out_access_descriptions| will alias data
// in |authority_info_access_tlv|.On failure returns false, and
// out_access_descriptions may have been partially filled.
//
// No validation is performed on the contents of the
// AuthorityInfoAccessDescription fields.
[[nodiscard]] OPENSSL_EXPORT bool ParseAuthorityInfoAccess(
    der::Input authority_info_access_tlv,
    std::vector<AuthorityInfoAccessDescription> *out_access_descriptions);

// Parses the Authority Information Access extension defined by RFC 5280,
// extracting the caIssuers URIs and OCSP URIs.
//
// Returns true on success, and |out_ca_issuers_uris| and |out_ocsp_uris| will
// alias data in |authority_info_access_tlv|. On failure returns false, and
// |out_ca_issuers_uris| and |out_ocsp_uris| may have been partially filled.
//
// |out_ca_issuers_uris| is filled with the accessLocations of type
// uniformResourceIdentifier for the accessMethod id-ad-caIssuers.
// |out_ocsp_uris| is filled with the accessLocations of type
// uniformResourceIdentifier for the accessMethod id-ad-ocsp.
//
// The values in |out_ca_issuers_uris| and |out_ocsp_uris| are checked to be
// IA5String (ASCII strings), but no other validation is performed on them.
//
// accessMethods other than id-ad-caIssuers and id-ad-ocsp are silently ignored.
// accessLocation types other than uniformResourceIdentifier are silently
// ignored.
[[nodiscard]] OPENSSL_EXPORT bool ParseAuthorityInfoAccessURIs(
    der::Input authority_info_access_tlv,
    std::vector<std::string_view> *out_ca_issuers_uris,
    std::vector<std::string_view> *out_ocsp_uris);

// ParsedDistributionPoint represents a parsed DistributionPoint from RFC 5280.
//
//   DistributionPoint ::= SEQUENCE {
//    distributionPoint       [0]     DistributionPointName OPTIONAL,
//    reasons                 [1]     ReasonFlags OPTIONAL,
//    cRLIssuer               [2]     GeneralNames OPTIONAL }
struct OPENSSL_EXPORT ParsedDistributionPoint {
  ParsedDistributionPoint();
  ParsedDistributionPoint(ParsedDistributionPoint &&other);
  ~ParsedDistributionPoint();

  // The parsed fullName, if distributionPoint was present and was a fullName.
  std::unique_ptr<GeneralNames> distribution_point_fullname;

  // If present, the DER encoded value of the nameRelativeToCRLIssuer field.
  // This should be a RelativeDistinguishedName, but the parser does not
  // validate it.
  std::optional<der::Input> distribution_point_name_relative_to_crl_issuer;

  // If present, the DER encoded value of the reasons field. This should be a
  // ReasonFlags bitString, but the parser does not validate it.
  std::optional<der::Input> reasons;

  // If present, the DER encoded value of the cRLIssuer field. This should be a
  // GeneralNames, but the parser does not validate it.
  std::optional<der::Input> crl_issuer;
};

// Parses the value of a CRL Distribution Points extension (sequence of
// DistributionPoint). Return true on success, and fills |distribution_points|
// with values that reference data in |distribution_points_tlv|.
[[nodiscard]] OPENSSL_EXPORT bool ParseCrlDistributionPoints(
    der::Input distribution_points_tlv,
    std::vector<ParsedDistributionPoint> *distribution_points);

// Represents the AuthorityKeyIdentifier extension defined by RFC 5280 section
// 4.2.1.1.
//
//    AuthorityKeyIdentifier ::= SEQUENCE {
//       keyIdentifier             [0] KeyIdentifier           OPTIONAL,
//       authorityCertIssuer       [1] GeneralNames            OPTIONAL,
//       authorityCertSerialNumber [2] CertificateSerialNumber OPTIONAL  }
//
//    KeyIdentifier ::= OCTET STRING
struct OPENSSL_EXPORT ParsedAuthorityKeyIdentifier {
  ParsedAuthorityKeyIdentifier();
  ~ParsedAuthorityKeyIdentifier();
  ParsedAuthorityKeyIdentifier(ParsedAuthorityKeyIdentifier &&other);
  ParsedAuthorityKeyIdentifier &operator=(ParsedAuthorityKeyIdentifier &&other);

  // The keyIdentifier, which is an OCTET STRING.
  std::optional<der::Input> key_identifier;

  // The authorityCertIssuer, which should be a GeneralNames, but this is not
  // enforced by ParseAuthorityKeyIdentifier.
  std::optional<der::Input> authority_cert_issuer;

  // The DER authorityCertSerialNumber, which should be a
  // CertificateSerialNumber (an INTEGER) but this is not enforced by
  // ParseAuthorityKeyIdentifier.
  std::optional<der::Input> authority_cert_serial_number;
};

// Parses the value of an authorityKeyIdentifier extension. Returns true on
// success and fills |authority_key_identifier| with values that reference data
// in |extension_value|. On failure the state of |authority_key_identifier| is
// not guaranteed.
[[nodiscard]] OPENSSL_EXPORT bool ParseAuthorityKeyIdentifier(
    der::Input extension_value,
    ParsedAuthorityKeyIdentifier *authority_key_identifier);

// Parses the value of a subjectKeyIdentifier extension. Returns true on
// success and |subject_key_identifier| references data in |extension_value|.
// On failure the state of |subject_key_identifier| is not guaranteed.
[[nodiscard]] OPENSSL_EXPORT bool ParseSubjectKeyIdentifier(
    der::Input extension_value, der::Input *subject_key_identifier);

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_PARSE_CERTIFICATE_H_
