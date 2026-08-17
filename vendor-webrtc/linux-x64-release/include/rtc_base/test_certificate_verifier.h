/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_TEST_CERTIFICATE_VERIFIER_H_
#define RTC_BASE_TEST_CERTIFICATE_VERIFIER_H_

#include "rtc_base/ssl_certificate.h"

namespace webrtc {

class TestCertificateVerifier : public SSLCertificateVerifier {
 public:
  TestCertificateVerifier() = default;
  ~TestCertificateVerifier() override = default;

  bool Verify(const SSLCertificate& certificate) override {
    call_count_++;
    return verify_certificate_;
  }

  size_t call_count_ = 0;
  bool verify_certificate_ = true;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::TestCertificateVerifier;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_TEST_CERTIFICATE_VERIFIER_H_
