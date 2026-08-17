/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_BORINGSSL_IDENTITY_H_
#define RTC_BASE_BORINGSSL_IDENTITY_H_

#include <openssl/ossl_typ.h>

#include <ctime>
#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "rtc_base/boringssl_certificate.h"
#include "rtc_base/openssl_key_pair.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/ssl_identity.h"

namespace webrtc {

// Holds a keypair and certificate together, and a method to generate them
// consistently. Uses CRYPTO_BUFFER instead of X509, which offers binary size
// and memory improvements.
class BoringSSLIdentity final : public SSLIdentity {
 public:
  static std::unique_ptr<BoringSSLIdentity> CreateWithExpiration(
      absl::string_view common_name,
      const KeyParams& key_params,
      time_t certificate_lifetime);
  static std::unique_ptr<BoringSSLIdentity> CreateForTest(
      const SSLIdentityParams& params);
  static std::unique_ptr<SSLIdentity> CreateFromPEMStrings(
      absl::string_view private_key,
      absl::string_view certificate);
  static std::unique_ptr<SSLIdentity> CreateFromPEMChainStrings(
      absl::string_view private_key,
      absl::string_view certificate_chain);
  ~BoringSSLIdentity() override;

  BoringSSLIdentity(const BoringSSLIdentity&) = delete;
  BoringSSLIdentity& operator=(const BoringSSLIdentity&) = delete;

  const BoringSSLCertificate& certificate() const override;
  const SSLCertChain& cert_chain() const override;

  // Configure an SSL context object to use our key and certificate.
  bool ConfigureIdentity(SSL_CTX* ctx);

  std::string PrivateKeyToPEMString() const override;
  std::string PublicKeyToPEMString() const override;
  bool operator==(const BoringSSLIdentity& other) const;
  bool operator!=(const BoringSSLIdentity& other) const;

 private:
  BoringSSLIdentity(std::unique_ptr<OpenSSLKeyPair> key_pair,
                    std::unique_ptr<BoringSSLCertificate> certificate);
  BoringSSLIdentity(std::unique_ptr<OpenSSLKeyPair> key_pair,
                    std::unique_ptr<SSLCertChain> cert_chain);
  std::unique_ptr<SSLIdentity> CloneInternal() const override;

  static std::unique_ptr<BoringSSLIdentity> CreateInternal(
      const SSLIdentityParams& params);

  std::unique_ptr<OpenSSLKeyPair> key_pair_;
  std::unique_ptr<SSLCertChain> cert_chain_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::BoringSSLIdentity;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_BORINGSSL_IDENTITY_H_
