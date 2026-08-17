// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_INTERNALS_RTC_CERTIFICATE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_INTERNALS_RTC_CERTIFICATE_H_

#include "third_party/blink/renderer/modules/peerconnection/rtc_certificate.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Internals;

class InternalsRTCCertificate {
  STATIC_ONLY(InternalsRTCCertificate);

 public:
  static bool rtcCertificateEquals(Internals&,
                                   RTCCertificate*,
                                   RTCCertificate*);
};

}  // blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_INTERNALS_RTC_CERTIFICATE_H_
