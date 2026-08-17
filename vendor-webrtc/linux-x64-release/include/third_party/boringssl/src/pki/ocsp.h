// Copyright 2016 The Chromium Authors
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

#ifndef BSSL_PKI_OCSP_H_
#define BSSL_PKI_OCSP_H_

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include <openssl/base.h>
#include <openssl/pki/ocsp.h>

#include "input.h"
#include "parse_values.h"
#include "parser.h"
#include "signature_algorithm.h"

BSSL_NAMESPACE_BEGIN

class ParsedCertificate;

// OCSPCertID contains a representation of a DER-encoded RFC 6960 "CertID".
//
// CertID ::= SEQUENCE {
//    hashAlgorithm           AlgorithmIdentifier,
//    issuerNameHash          OCTET STRING, -- Hash of issuer's DN
//    issuerKeyHash           OCTET STRING, -- Hash of issuer's public key
//    serialNumber            CertificateSerialNumber
// }
struct OPENSSL_EXPORT OCSPCertID {
  OCSPCertID();
  ~OCSPCertID();

  DigestAlgorithm hash_algorithm;
  der::Input issuer_name_hash;
  der::Input issuer_key_hash;
  der::Input serial_number;
};

// OCSPCertStatus contains a representation of a DER-encoded RFC 6960
// "CertStatus". |revocation_time| and |has_reason| are only valid when
// |status| is REVOKED. |revocation_reason| is only valid when |has_reason| is
// true.
//
// CertStatus ::= CHOICE {
//      good        [0]     IMPLICIT NULL,
//      revoked     [1]     IMPLICIT RevokedInfo,
//      unknown     [2]     IMPLICIT UnknownInfo
// }
//
// RevokedInfo ::= SEQUENCE {
//      revocationTime              GeneralizedTime,
//      revocationReason    [0]     EXPLICIT CRLReason OPTIONAL
// }
//
// UnknownInfo ::= NULL
//
// CRLReason ::= ENUMERATED {
//      unspecified             (0),
//      keyCompromise           (1),
//      cACompromise            (2),
//      affiliationChanged      (3),
//      superseded              (4),
//      cessationOfOperation    (5),
//      certificateHold         (6),
//           -- value 7 is not used
//      removeFromCRL           (8),
//      privilegeWithdrawn      (9),
//      aACompromise           (10)
// }
// (from RFC 5280)
struct OCSPCertStatus {
  // Correspond to the values of CRLReason
  enum class RevocationReason {
    UNSPECIFIED = 0,
    KEY_COMPROMISE = 1,
    CA_COMPROMISE = 2,
    AFFILIATION_CHANGED = 3,
    SUPERSEDED = 4,
    CESSATION_OF_OPERATION = 5,
    CERTIFICATE_HOLD = 6,
    UNUSED = 7,
    REMOVE_FROM_CRL = 8,
    PRIVILEGE_WITHDRAWN = 9,
    AA_COMPROMISE = 10,

    LAST = AA_COMPROMISE,
  };

  OCSPRevocationStatus status;
  der::GeneralizedTime revocation_time;
  bool has_reason;
  RevocationReason revocation_reason;
};

// OCSPSingleResponse contains a representation of a DER-encoded RFC 6960
// "SingleResponse". The |cert_id_tlv| and |extensions| fields are pointers to
// the original object and are only valid as long as it is alive. They also
// aren't verified until they are parsed. |next_update| is only valid if
// |has_next_update| is true and |extensions| is only valid if |has_extensions|
// is true.
//
// SingleResponse ::= SEQUENCE {
//      certID                       CertID,
//      certStatus                   CertStatus,
//      thisUpdate                   GeneralizedTime,
//      nextUpdate         [0]       EXPLICIT GeneralizedTime OPTIONAL,
//      singleExtensions   [1]       EXPLICIT Extensions OPTIONAL
// }
struct OPENSSL_EXPORT OCSPSingleResponse {
  OCSPSingleResponse();
  ~OCSPSingleResponse();

  der::Input cert_id_tlv;
  OCSPCertStatus cert_status;
  der::GeneralizedTime this_update;
  bool has_next_update;
  der::GeneralizedTime next_update;
  bool has_extensions;
  der::Input extensions;
};

// OCSPResponseData contains a representation of a DER-encoded RFC 6960
// "ResponseData". The |responses| and |extensions| fields are pointers to the
// original object and are only valid as long as it is alive. They also aren't
// verified until they are parsed into OCSPSingleResponse and ParsedExtensions.
// |extensions| is only valid if |has_extensions| is true.
//
// ResponseData ::= SEQUENCE {
//      version              [0] EXPLICIT Version DEFAULT v1,
//      responderID              ResponderID,
//      producedAt               GeneralizedTime,
//      responses                SEQUENCE OF SingleResponse,
//      responseExtensions   [1] EXPLICIT Extensions OPTIONAL
// }
struct OPENSSL_EXPORT OCSPResponseData {
  enum class ResponderType { NAME, KEY_HASH };

  struct ResponderID {
    ResponderType type;
    der::Input name;
    der::Input key_hash;
  };

  OCSPResponseData();
  ~OCSPResponseData();

  uint8_t version;
  OCSPResponseData::ResponderID responder_id;
  der::GeneralizedTime produced_at;
  std::vector<der::Input> responses;
  bool has_extensions;
  der::Input extensions;
};

// OCSPResponse contains a representation of a DER-encoded RFC 6960
// "OCSPResponse" and the corresponding "BasicOCSPResponse". The |data| field
// is a pointer to the original object and are only valid as long is it is
// alive. The |data| field isn't verified until it is parsed into an
// OCSPResponseData. |data|, |signature_algorithm|, |signature|, and
// |has_certs| is only valid if |status| is SUCCESSFUL. |certs| is only valid
// if |has_certs| is true.
//
// OCSPResponse ::= SEQUENCE {
//      responseStatus         OCSPResponseStatus,
//      responseBytes          [0] EXPLICIT ResponseBytes OPTIONAL
// }
//
// ResponseBytes ::=       SEQUENCE {
//      responseType   OBJECT IDENTIFIER,
//      response       OCTET STRING
// }
//
// BasicOCSPResponse       ::= SEQUENCE {
//      tbsResponseData      ResponseData,
//      signatureAlgorithm   AlgorithmIdentifier,
//      signature            BIT STRING,
//      certs            [0] EXPLICIT SEQUENCE OF Certificate OPTIONAL
// }
//
// OCSPResponseStatus ::= ENUMERATED {
//     successful            (0),  -- Response has valid confirmations
//     malformedRequest      (1),  -- Illegal confirmation request
//     internalError         (2),  -- Internal error in issuer
//     tryLater              (3),  -- Try again later
//                                 -- (4) is not used
//     sigRequired           (5),  -- Must sign the request
//     unauthorized          (6)   -- Request unauthorized
// }
struct OPENSSL_EXPORT OCSPResponse {
  // Correspond to the values of OCSPResponseStatus
  enum class ResponseStatus {
    SUCCESSFUL = 0,
    MALFORMED_REQUEST = 1,
    INTERNAL_ERROR = 2,
    TRY_LATER = 3,
    UNUSED = 4,
    SIG_REQUIRED = 5,
    UNAUTHORIZED = 6,

    LAST = UNAUTHORIZED,
  };

  OCSPResponse();
  ~OCSPResponse();

  ResponseStatus status;
  der::Input data;
  SignatureAlgorithm signature_algorithm;
  der::BitString signature;
  bool has_certs;
  std::vector<der::Input> certs;
};

// From RFC 6960:
//
// id-pkix-ocsp           OBJECT IDENTIFIER ::= { id-ad-ocsp }
// id-pkix-ocsp-basic     OBJECT IDENTIFIER ::= { id-pkix-ocsp 1 }
//
// In dotted notation: 1.3.6.1.5.5.7.48.1.1
inline constexpr uint8_t kBasicOCSPResponseOid[] = {
    0x2b, 0x06, 0x01, 0x05, 0x05, 0x07, 0x30, 0x01, 0x01};

// Parses a DER-encoded OCSP "CertID" as specified by RFC 6960. Returns true on
// success and sets the results in |out|.
//
// On failure |out| has an undefined state. Some of its fields may have been
// updated during parsing, whereas others may not have been changed.
OPENSSL_EXPORT bool ParseOCSPCertID(der::Input raw_tlv, OCSPCertID *out);

// Parses a DER-encoded OCSP "SingleResponse" as specified by RFC 6960. Returns
// true on success and sets the results in |out|. The resulting |out|
// references data from |raw_tlv| and is only valid for the lifetime of
// |raw_tlv|.
//
// On failure |out| has an undefined state. Some of its fields may have been
// updated during parsing, whereas others may not have been changed.
OPENSSL_EXPORT bool ParseOCSPSingleResponse(der::Input raw_tlv,
                                            OCSPSingleResponse *out);

// Parses a DER-encoded OCSP "ResponseData" as specified by RFC 6960. Returns
// true on success and sets the results in |out|. The resulting |out|
// references data from |raw_tlv| and is only valid for the lifetime of
// |raw_tlv|.
//
// On failure |out| has an undefined state. Some of its fields may have been
// updated during parsing, whereas others may not have been changed.
OPENSSL_EXPORT bool ParseOCSPResponseData(der::Input raw_tlv,
                                          OCSPResponseData *out);

// Parses a DER-encoded "OCSPResponse" as specified by RFC 6960. Returns true
// on success and sets the results in |out|. The resulting |out|
// references data from |raw_tlv| and is only valid for the lifetime of
// |raw_tlv|.
//
// On failure |out| has an undefined state. Some of its fields may have been
// updated during parsing, whereas others may not have been changed.
OPENSSL_EXPORT bool ParseOCSPResponse(der::Input raw_tlv, OCSPResponse *out);

// Checks the revocation status of |certificate| by using the DER-encoded
// |raw_response|.
//
// Arguments are the same as above, except that it takes already parsed
// instances of the certificate and issuer certificate.
[[nodiscard]] OPENSSL_EXPORT OCSPRevocationStatus CheckOCSP(
    std::string_view raw_response, const ParsedCertificate *certificate,
    const ParsedCertificate *issuer_certificate,
    int64_t verify_time_epoch_seconds, std::optional<int64_t> max_age_seconds,
    OCSPVerifyResult::ResponseStatus *response_details);

// Creates a DER-encoded OCSPRequest for |cert|. The request is fairly basic:
//  * No signature
//  * No requestorName
//  * No extensions
//  * Uses SHA1 for all hashes.
//
// Returns true on success and fills |request_der| with the resulting bytes.
OPENSSL_EXPORT bool CreateOCSPRequest(const ParsedCertificate *cert,
                                      const ParsedCertificate *issuer,
                                      std::vector<uint8_t> *request_der);

// Creates a URL to issue a GET request for OCSP information for |cert|.
OPENSSL_EXPORT std::optional<std::string> CreateOCSPGetURL(
    const ParsedCertificate *cert, const ParsedCertificate *issuer,
    std::string_view ocsp_responder_url);

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_OCSP_H_
