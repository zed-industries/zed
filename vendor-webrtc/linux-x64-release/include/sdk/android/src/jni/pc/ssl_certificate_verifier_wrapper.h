/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_PC_SSL_CERTIFICATE_VERIFIER_WRAPPER_H_
#define SDK_ANDROID_SRC_JNI_PC_SSL_CERTIFICATE_VERIFIER_WRAPPER_H_

#include <jni.h>

#include <vector>

#include "rtc_base/ssl_certificate.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace webrtc {
namespace jni {

// Wrapper for Java SSLCertifiacteVerifier class. Delegates method calls through
// JNI and wraps the encoder inside SSLCertificateVerifierWrapper.
class SSLCertificateVerifierWrapper : public SSLCertificateVerifier {
 public:
  SSLCertificateVerifierWrapper(
      JNIEnv* jni,
      const JavaRef<jobject>& ssl_certificate_verifier);
  ~SSLCertificateVerifierWrapper() override;

  bool Verify(const SSLCertificate& certificate) override;

 private:
  const ScopedJavaGlobalRef<jobject> ssl_certificate_verifier_;
};

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_SSL_CERTIFICATE_VERIFIER_WRAPPER_H_
