// Copyright 2023 The BoringSSL Authors
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

#if !defined(OPENSSL_HEADER_BSSL_PKI_CERTIFICATE_H_) && defined(__cplusplus)
#define OPENSSL_HEADER_BSSL_PKI_CERTIFICATE_H_

#include <memory>
#include <string>
#include <string_view>

#include <openssl/base.h>   // IWYU pragma: export
#include <openssl/span.h>

BSSL_NAMESPACE_BEGIN

struct CertificateInternals;

// Certificate represents a parsed X.509 certificate. It includes accessors for
// the various things that one might want to extract from a certificate,
class OPENSSL_EXPORT Certificate {
 public:
  Certificate(Certificate&& other);
  Certificate(const Certificate& other) = delete;
  ~Certificate();
  Certificate& operator=(const Certificate& other) = delete;

  // FromDER returns a certificate from an DER-encoded X.509 object in |der|.
  // In the event of a failure, it will return no value, and |out_diagnostic|
  // may be set to a string of human readable debugging information if
  // information abou the failure is available.
  static std::unique_ptr<Certificate> FromDER(
      bssl::Span<const uint8_t> der, std::string *out_diagnostic);

  // FromPEM returns a certificate from the first CERTIFICATE PEM block in
  // |pem|. In the event of a failure, it will return no value, and
  // |out_diagnostic| may be set to a string of human readable debugging
  // informtion if informaiton about the failuew is available.
  static std::unique_ptr<Certificate> FromPEM(
      std::string_view pem, std::string *out_diagnostic);

  // IsSelfIssued returns true if the certificate is "self-issued" per RFC 5280
  // section 6.1. I.e. that the subject and issuer names are equal after
  // canonicalization (and no other checks).
  //
  // Other contexts may have a different notion such as "self signed" which
  // may or may not be this, and may check other properties of the certificate.
  bool IsSelfIssued() const;

  // Validity specifies the temporal validity of a cerificate, expressed in
  // POSIX time values of seconds since the POSIX epoch. The certificate is
  // valid at POSIX time t in second granularity, where not_before <= t <=
  // not_after.
  struct Validity {
    int64_t not_before;
    int64_t not_after;
  };

  Validity GetValidity() const;

  // The binary, big-endian, DER representation of the certificate serial
  // number. It may include a leading 00 byte.
  bssl::Span<const uint8_t> GetSerialNumber() const;

 private:
  explicit Certificate(std::unique_ptr<CertificateInternals> internals);

  std::unique_ptr<CertificateInternals> internals_;
};

BSSL_NAMESPACE_END

#endif  // OPENSSL_HEADER_BSSL_PKI_CERTIFICATE_H_ && __cplusplus
