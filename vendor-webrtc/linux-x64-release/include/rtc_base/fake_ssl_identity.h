/*
 *  Copyright 2012 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_FAKE_SSL_IDENTITY_H_
#define RTC_BASE_FAKE_SSL_IDENTITY_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "rtc_base/buffer.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/ssl_identity.h"

namespace webrtc {

class FakeSSLCertificate : public SSLCertificate {
 public:
  // SHA-1 is the default digest algorithm because it is available in all build
  // configurations used for unit testing.
  explicit FakeSSLCertificate(absl::string_view pem_string);

  FakeSSLCertificate(const FakeSSLCertificate&);
  ~FakeSSLCertificate() override;

  // SSLCertificate implementation.
  std::unique_ptr<SSLCertificate> Clone() const override;
  std::string ToPEMString() const override;
  void ToDER(Buffer* der_buffer) const override;
  int64_t CertificateExpirationTime() const override;
  bool GetSignatureDigestAlgorithm(std::string* algorithm) const override;
  bool ComputeDigest(absl::string_view algorithm,
                     Buffer& digest) const override;

  void SetCertificateExpirationTime(int64_t expiration_time);

  void set_digest_algorithm(absl::string_view algorithm);

 private:
  std::string pem_string_;
  std::string digest_algorithm_;
  // Expiration time in seconds relative to epoch, 1970-01-01T00:00:00Z (UTC).
  int64_t expiration_time_;
};

class FakeSSLIdentity : public SSLIdentity {
 public:
  explicit FakeSSLIdentity(absl::string_view pem_string);
  // For a certificate chain.
  explicit FakeSSLIdentity(const std::vector<std::string>& pem_strings);
  explicit FakeSSLIdentity(const FakeSSLCertificate& cert);

  explicit FakeSSLIdentity(const FakeSSLIdentity& o);

  ~FakeSSLIdentity() override;

  // SSLIdentity implementation.
  const SSLCertificate& certificate() const override;
  const SSLCertChain& cert_chain() const override;
  // Not implemented.
  std::string PrivateKeyToPEMString() const override;
  // Not implemented.
  std::string PublicKeyToPEMString() const override;
  // Not implemented.
  virtual bool operator==(const SSLIdentity& other) const;

 private:
  std::unique_ptr<SSLIdentity> CloneInternal() const override;

  std::unique_ptr<SSLCertChain> cert_chain_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::FakeSSLCertificate;
using ::webrtc::FakeSSLIdentity;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_FAKE_SSL_IDENTITY_H_
