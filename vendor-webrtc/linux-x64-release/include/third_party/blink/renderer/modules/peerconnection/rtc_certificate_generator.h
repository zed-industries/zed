// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_CERTIFICATE_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_CERTIFICATE_GENERATOR_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/webrtc/api/peer_connection_interface.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class ExecutionContext;

using RTCCertificateCallback =
    base::OnceCallback<void(webrtc::scoped_refptr<webrtc::RTCCertificate>)>;

// Chromium's WebRTCCertificateGenerator implementation; uses the
// PeerConnectionIdentityStore/SSLIdentity::Generate to generate the identity,
// webrtc::RTCCertificate and blink::RTCCertificate.
class MODULES_EXPORT RTCCertificateGenerator {
 public:
  RTCCertificateGenerator() {}

  RTCCertificateGenerator(const RTCCertificateGenerator&) = delete;
  RTCCertificateGenerator& operator=(const RTCCertificateGenerator&) = delete;

  ~RTCCertificateGenerator() {}

  // Start generating a certificate asynchronously. |observer| is invoked on the
  // same thread that called generateCertificate when the operation is
  // completed.
  void GenerateCertificate(
      const webrtc::KeyParams& key_params,
      blink::RTCCertificateCallback completion_callback,
      ExecutionContext& context,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  void GenerateCertificateWithExpiration(
      const webrtc::KeyParams& key_params,
      uint64_t expires_ms,
      blink::RTCCertificateCallback completion_callback,
      ExecutionContext& context,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  // Determines if the parameters are supported by |GenerateCertificate|.
  // For example, if the number of bits of some parameter is too small or too
  // large we may want to reject it for security or performance reasons.
  bool IsSupportedKeyParams(const webrtc::KeyParams& key_params);

  // Creates a certificate from the PEM strings. See also
  // |webrtc::RTCCertificate::ToPEM|.
  webrtc::scoped_refptr<webrtc::RTCCertificate> FromPEM(String pem_private_key,
                                                        String pem_certificate);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_CERTIFICATE_GENERATOR_H_
