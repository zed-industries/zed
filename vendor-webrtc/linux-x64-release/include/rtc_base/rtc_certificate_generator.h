/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_RTC_CERTIFICATE_GENERATOR_H_
#define RTC_BASE_RTC_CERTIFICATE_GENERATOR_H_

#include <stdint.h>

#include <optional>

#include "absl/functional/any_invocable.h"
#include "api/scoped_refptr.h"
#include "rtc_base/rtc_certificate.h"
#include "rtc_base/ssl_identity.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/thread.h"

namespace webrtc {

// Generates `RTCCertificate`s.
// See `RTCCertificateGenerator` for the WebRTC repo's implementation.
class RTCCertificateGeneratorInterface {
 public:
  // Functor that will be called when certificate is generated asynchroniosly.
  // Called with nullptr as the parameter on failure.
  using Callback =
      absl::AnyInvocable<void(scoped_refptr<webrtc::RTCCertificate>) &&>;

  virtual ~RTCCertificateGeneratorInterface() = default;

  // Generates a certificate asynchronously on the worker thread.
  // Must be called on the signaling thread. The `callback` is invoked with the
  // result on the signaling thread. `exipres_ms` optionally specifies for how
  // long we want the certificate to be valid, but the implementation may choose
  // its own restrictions on the expiration time.
  virtual void GenerateCertificateAsync(
      const KeyParams& key_params,
      const std::optional<uint64_t>& expires_ms,
      Callback callback) = 0;
};

// Standard implementation of `RTCCertificateGeneratorInterface`.
// The static function `GenerateCertificate` generates a certificate on the
// current thread. The `RTCCertificateGenerator` instance generates certificates
// asynchronously on the worker thread with `GenerateCertificateAsync`.
class RTC_EXPORT RTCCertificateGenerator
    : public RTCCertificateGeneratorInterface {
 public:
  // Generates a certificate on the current thread. Returns null on failure.
  // If `expires_ms` is specified, the certificate will expire in approximately
  // that many milliseconds from now. `expires_ms` is limited to a year, a
  // larger value than that is clamped down to a year. If `expires_ms` is not
  // specified, a default expiration time is used.
  static scoped_refptr<RTCCertificate> GenerateCertificate(
      const KeyParams& key_params,
      const std::optional<uint64_t>& expires_ms);

  RTCCertificateGenerator(Thread* signaling_thread, Thread* worker_thread);
  ~RTCCertificateGenerator() override {}

  // `RTCCertificateGeneratorInterface` overrides.
  // If `expires_ms` is specified, the certificate will expire in approximately
  // that many milliseconds from now. `expires_ms` is limited to a year, a
  // larger value than that is clamped down to a year. If `expires_ms` is not
  // specified, a default expiration time is used.
  void GenerateCertificateAsync(const KeyParams& key_params,
                                const std::optional<uint64_t>& expires_ms,
                                Callback callback) override;

 private:
  Thread* const signaling_thread_;
  Thread* const worker_thread_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::RTCCertificateGenerator;
using ::webrtc::RTCCertificateGeneratorInterface;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_RTC_CERTIFICATE_GENERATOR_H_
