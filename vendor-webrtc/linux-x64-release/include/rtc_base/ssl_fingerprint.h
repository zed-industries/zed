/*
 *  Copyright 2012 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SSL_FINGERPRINT_H_
#define RTC_BASE_SSL_FINGERPRINT_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/rtc_certificate.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/ssl_identity.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

struct RTC_EXPORT SSLFingerprint {
  // TODO(steveanton): Remove once downstream projects have moved off of this.
  static SSLFingerprint* Create(absl::string_view algorithm,
                                const SSLIdentity* identity);
  // TODO(steveanton): Rename to Create once projects have migrated.
  static std::unique_ptr<SSLFingerprint> CreateUnique(
      absl::string_view algorithm,
      const SSLIdentity& identity);

  static std::unique_ptr<SSLFingerprint> Create(absl::string_view algorithm,
                                                const SSLCertificate& cert);

  // TODO(steveanton): Remove once downstream projects have moved off of this.
  static SSLFingerprint* CreateFromRfc4572(absl::string_view algorithm,
                                           absl::string_view fingerprint);
  // TODO(steveanton): Rename to CreateFromRfc4572 once projects have migrated.
  static std::unique_ptr<SSLFingerprint> CreateUniqueFromRfc4572(
      absl::string_view algorithm,
      absl::string_view fingerprint);

  // Creates a fingerprint from a certificate, using the same digest algorithm
  // as the certificate's signature.
  static std::unique_ptr<SSLFingerprint> CreateFromCertificate(
      const RTCCertificate& cert);

  SSLFingerprint(absl::string_view algorithm,
                 ArrayView<const uint8_t> digest_view);
  // TODO(steveanton): Remove once downstream projects have moved off of this.
  SSLFingerprint(absl::string_view algorithm,
                 const uint8_t* digest_in,
                 size_t digest_len);

  SSLFingerprint(const SSLFingerprint& from) = default;
  SSLFingerprint& operator=(const SSLFingerprint& from) = default;

  bool operator==(const SSLFingerprint& other) const;

  std::string GetRfc4572Fingerprint() const;

  std::string ToString() const;

  std::string algorithm;
  CopyOnWriteBuffer digest;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::SSLFingerprint;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_SSL_FINGERPRINT_H_
