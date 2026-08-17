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

#if !defined(OPENSSL_HEADER_BSSL_PKI_OCSP_H_)  && defined(__cplusplus)
#define OPENSSL_HEADER_BSSL_PKI_OCSP_H_

#include <openssl/base.h>   // IWYU pragma: export
#include <string_view>
#include <optional>

BSSL_NAMESPACE_BEGIN

// The revocation status indicated by an OCSP verification.  This value is
// histogrammed in Chrome, so do not re-order or change values, and add new
// values at the end.
enum class OCSPRevocationStatus {
  GOOD = 0,
  REVOKED = 1,
  UNKNOWN = 2,
  MAX_VALUE = UNKNOWN
};

// The result of OCSP verification. This always contains a ResponseStatus, which
// describes whether or not an OCSP response was provided, and response level
// errors. It optionally contains an OCSPRevocationStatus when |response_status
// = PROVIDED|. For example, a stapled OCSP response matching the certificate,
// and indicating a non-revoked status, will have |response_status = PROVIDED|
// and |revocation_status = GOOD|.
struct OPENSSL_EXPORT OCSPVerifyResult {
  bool operator==(const OCSPVerifyResult &other) const {
    if (response_status != other.response_status) {
      return false;
    }

    if (response_status == PROVIDED) {
      // |revocation_status| is only defined when |response_status| is PROVIDED.
      return revocation_status == other.revocation_status;
    }
    return true;
  }

  // This value is histogrammed in Chrome, so do not re-order or change values,
  // and add new values at the end.
  enum ResponseStatus {
    // OCSP verification was not checked on this connection.
    NOT_CHECKED = 0,

    // No OCSPResponse was stapled.
    MISSING = 1,

    // An up-to-date OCSP response was stapled and matched the certificate.
    PROVIDED = 2,

    // The stapled OCSP response did not have a SUCCESSFUL status.
    ERROR_RESPONSE = 3,

    // The OCSPResponseData field producedAt was outside the certificate
    // validity period.
    BAD_PRODUCED_AT = 4,

    // At least one OCSPSingleResponse was stapled, but none matched the
    // certificate.
    NO_MATCHING_RESPONSE = 5,

    // A matching OCSPSingleResponse was stapled, but was either expired or not
    // yet valid.
    INVALID_DATE = 6,

    // The OCSPResponse structure could not be parsed.
    PARSE_RESPONSE_ERROR = 7,

    // The OCSPResponseData structure could not be parsed.
    PARSE_RESPONSE_DATA_ERROR = 8,

    // Unhandled critical extension in either OCSPResponseData or
    // OCSPSingleResponse
    UNHANDLED_CRITICAL_EXTENSION = 9,
    RESPONSE_STATUS_MAX = UNHANDLED_CRITICAL_EXTENSION
  };

  ResponseStatus response_status = NOT_CHECKED;

  // The strictest CertStatus matching the certificate (REVOKED > UNKNOWN >
  // GOOD). Only valid if |response_status| = PROVIDED.
  OCSPRevocationStatus revocation_status = OCSPRevocationStatus::UNKNOWN;
};

// Checks the revocation status of the certificate |certificate_der| by using
// the DER-encoded |raw_response|.
//
// Returns GOOD if the OCSP response indicates the certificate is not revoked,
// REVOKED if it indicates it is revoked, or UNKNOWN for all other cases.
//
//  * |raw_response|: A DER encoded OCSPResponse.
//  * |certificate_der|: The certificate being checked for revocation.
//  * |issuer_certificate_der|: The certificate that signed |certificate_der|.
//        The caller must have already performed path verification.
//  * |verify_time_epoch_seconds|: The time as the difference in seconds from
//        the POSIX epoch to use when checking revocation status.
//  * |max_age_seconds|: The maximum age in seconds for a CRL, implemented as
//        time since the |thisUpdate| field in the CRL TBSCertList. Responses
//        older than |max_age_seconds| will be considered invalid.
//  * |response_details|: Additional details about failures.
[[nodiscard]] OPENSSL_EXPORT OCSPRevocationStatus CheckOCSP(
    std::string_view raw_response, std::string_view certificate_der,
    std::string_view issuer_certificate_der, int64_t verify_time_epoch_seconds,
    std::optional<int64_t> max_age_seconds,
    OCSPVerifyResult::ResponseStatus *response_details);

BSSL_NAMESPACE_END

#endif  // OPENSSL_HEADER_BSSL_PKI_OCSP_H_ && __cplusplus
