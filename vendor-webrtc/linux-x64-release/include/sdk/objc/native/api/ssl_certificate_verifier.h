/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_API_SSL_CERTIFICATE_VERIFIER_H_
#define SDK_OBJC_NATIVE_API_SSL_CERTIFICATE_VERIFIER_H_

#include <memory>

#import "RTCSSLCertificateVerifier.h"
#include "rtc_base/ssl_certificate.h"

namespace webrtc {

std::unique_ptr<webrtc::SSLCertificateVerifier> ObjCToNativeCertificateVerifier(
    id<RTC_OBJC_TYPE(RTCSSLCertificateVerifier)> objc_certificate_verifier);

}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_API_SSL_CERTIFICATE_VERIFIER_H_
