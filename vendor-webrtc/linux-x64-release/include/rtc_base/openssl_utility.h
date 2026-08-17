/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_OPENSSL_UTILITY_H_
#define RTC_BASE_OPENSSL_UTILITY_H_

#include <openssl/ossl_typ.h>

#include <string>

#include "absl/strings/string_view.h"

namespace webrtc {
// The openssl namespace holds static helper methods. All methods related
// to OpenSSL that are commonly used and don't require global state should be
// placed here.
namespace openssl {

#ifdef OPENSSL_IS_BORINGSSL
// Does minimal parsing of a certificate (only verifying the presence of major
// fields), primarily for the purpose of extracting the relevant out
// parameters. Any that the caller is uninterested in can be null.
bool ParseCertificate(CRYPTO_BUFFER* cert_buffer,
                      CBS* signature_algorithm_oid,
                      int64_t* expiration_time);
#endif

// Verifies that the hostname provided matches that in the peer certificate
// attached to this SSL state.
// TODO(crbug.com/webrtc/11710): When OS certificate verification is available,
// skip compiling this as it adds a dependency on OpenSSL X509 objects, which we
// are trying to avoid in favor of CRYPTO_BUFFERs (see crbug.com/webrtc/11410).
bool VerifyPeerCertMatchesHost(SSL* ssl, absl::string_view host);

// Logs all the errors in the OpenSSL errror queue from the current thread. A
// prefix can be provided for context.
void LogSSLErrors(absl::string_view prefix);

#ifndef WEBRTC_EXCLUDE_BUILT_IN_SSL_ROOT_CERTS
// Attempt to add the certificates from the loader into the SSL_CTX. False is
// returned only if there are no certificates returned from the loader or none
// of them can be added to the TrustStore for the provided context.
bool LoadBuiltinSSLRootCertificates(SSL_CTX* ssl_ctx);
#endif  // WEBRTC_EXCLUDE_BUILT_IN_SSL_ROOT_CERTS

#ifdef OPENSSL_IS_BORINGSSL
CRYPTO_BUFFER_POOL* GetBufferPool();
#endif

}  // namespace openssl
}  // namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
namespace openssl {

#ifndef WEBRTC_EXCLUDE_BUILT_IN_SSL_ROOT_CERTS
using ::webrtc::openssl::LoadBuiltinSSLRootCertificates;
#endif

using ::webrtc::openssl::LogSSLErrors;
using ::webrtc::openssl::VerifyPeerCertMatchesHost;

#ifdef OPENSSL_IS_BORINGSSL
using ::webrtc::openssl::GetBufferPool;
using ::webrtc::openssl::ParseCertificate;
#endif

}  // namespace openssl
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_OPENSSL_UTILITY_H_
