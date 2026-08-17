// Copyright 2024 The BoringSSL Authors
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

#if !defined(OPENSSL_HEADER_BSSL_PKI_VERIFY_ERROR_H_) && defined(__cplusplus)
#define OPENSSL_HEADER_BSSL_PKI_VERIFY_ERROR_H_

#include <openssl/base.h>   // IWYU pragma: export

#include <string>

BSSL_NAMESPACE_BEGIN

// VerifyError describes certificate chain validation result.
class OPENSSL_EXPORT VerifyError {
 public:
  VerifyError() = default;
  VerifyError(const VerifyError &other) = default;
  VerifyError &operator=(const VerifyError &other) = default;

  // Code is the representation of a single error that we could
  // find.
  enum class StatusCode {
    // PATH_VERIFIED means there were no errors, the certificate chain is valid.
    PATH_VERIFIED,

    // CERTIFICATE_INVALID_SIGNATURE means that the certificate's signature
    // failed to verify.
    CERTIFICATE_INVALID_SIGNATURE,

    // CERTIFICATE_UNSUPPORTED_KEY means that the certificate's key type and/or
    // size is not supported.
    CERTIFICATE_UNSUPPORTED_KEY,

    // CERTIFICATE_UNSUPPORTED_SIGNATURE ALGORITHM means that the signature
    // algorithm is not supported.
    CERTIFICATE_UNSUPPORTED_SIGNATURE_ALGORITHM,

    // CERTIFICATE_REVOKED means that the certificate has been revoked.
    CERTIFICATE_REVOKED,

    // CERTIFICATE_NO_REVOCATION_MECHANISM means that revocation checking was
    // required and no revocation mechanism was given for the certificate
    CERTIFICATE_NO_REVOCATION_MECHANISM,

    // CERTIFICATE_UNABLE_TO_CHECK_REVOCATION means that revocation checking was
    // required and we were unable to check if the certificate was revoked via
    // any revocation mechanism.
    CERTIFICATE_UNABLE_TO_CHECK_REVOCATION,

    // CERTIFICATE_EXPIRED means that the validation time is after the
    // certificate's |notAfter| timestamp.
    CERTIFICATE_EXPIRED,

    // CERTIFICATE_NOT_YET_VALID means that the validation time is before the
    // certificate's |notBefore| timestamp.
    CERTIFICATE_NOT_YET_VALID,

    // CERTIFICATE_NO_MATCHING_EKU means that the certificate's EKU does not
    // allow the certificate to be used for the intended purpose.
    CERTIFICATE_NO_MATCHING_EKU,

    // CERTIFICATE_INVALID means that the certificate was structurally
    // invalid, or invalid for some different reason than the above.
    CERTIFICATE_INVALID,

    // PATH_NOT_FOUND means that no path could be found from the leaf
    // certificate to any trust anchor.
    PATH_NOT_FOUND,

    // PATH_ITERATION_COUNT_EXCEEDED means that the iteration limit for path
    // building  was hit and so the search for a valid path terminated early.
    PATH_ITERATION_COUNT_EXCEEDED,

    // PATH_DEADLINE_EXCEEDED means that the time limit for path building
    // was hit and so the search for a valid path terminated early.
    PATH_DEADLINE_EXCEEDED,

    // PATH_DEPTH_LIMIT_REACHED means that path building was not able to find a
    // path within the configured depth limit for verification.
    PATH_DEPTH_LIMIT_REACHED,

    // PATH_MULTIPLE_ERRORS indicates that there are multiple fatal
    // errors present on the certificate chain, so that a single error could
    // not be reported.
    PATH_MULTIPLE_ERRORS,

    // VERIFICATION_FAILURE means that something is wrong with the returned path
    // that is not specific to a single certificate. There are many possible
    // reasons for a verification to fail.
    VERIFICATION_FAILURE,
  };

  VerifyError(StatusCode code, ptrdiff_t offset, std::string diagnostic);

  // Code returns the indicated error code for the certificate path.
  StatusCode Code() const;

  // Index returns the certificate in the chain for which the error first
  // occured, starting with 0 for the leaf certificate. Later certificates in
  // the chain may also exhibit the same error. If the error is not specific to
  // a certificate, -1 is returned.
  ptrdiff_t Index() const;

  // DiagnosticString returns a string of diagnostic information related to this
  // verification attempt. The string aims to be useful to debugging, but it is
  // not stable and may not be processed programmatically or asserted on in
  // tests. The string may be empty if no diagnostic information was available.
  //
  // The DiagnosticString is specifically not guaranteed to be unchanging for
  // any given error code, as the diagnostic error message can contain
  // information specific to the verification attempt and chain presented, due
  // to there being multiple possible ways for, as an example, a certificate to
  // be invalid, or that we are unable to build a path to a trust anchor.
  //
  // Needless to say, one should not attempt to parse the string that is
  // returned.
  const std::string &DiagnosticString() const;

 private:
  ptrdiff_t offset_ = -1;
  StatusCode code_ = StatusCode::VERIFICATION_FAILURE;
  std::string diagnostic_;
};

BSSL_NAMESPACE_END

#endif  // OPENSSL_HEADER_BSSL_PKI_VERIFY_ERROR_H_
