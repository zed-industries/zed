/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_CRYPTO_CRYPTO_OPTIONS_H_
#define API_CRYPTO_CRYPTO_OPTIONS_H_

#include <vector>

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// CryptoOptions defines advanced cryptographic settings for native WebRTC.
// These settings must be passed into PeerConnectionFactoryInterface::Options
// and are only applicable to native use cases of WebRTC.
struct RTC_EXPORT CryptoOptions {
  CryptoOptions();
  CryptoOptions(const CryptoOptions& other);
  ~CryptoOptions();

  // Helper method to return an instance of the CryptoOptions with GCM crypto
  // suites disabled. This method should be used instead of depending on current
  // default values set by the constructor.
  static CryptoOptions NoGcm();

  // Returns a list of the supported DTLS-SRTP Crypto suites based on this set
  // of crypto options.
  std::vector<int> GetSupportedDtlsSrtpCryptoSuites() const;

  bool operator==(const CryptoOptions& other) const;
  bool operator!=(const CryptoOptions& other) const;

  // SRTP Related Peer Connection options.
  struct Srtp {
    // Enable GCM crypto suites from RFC 7714 for SRTP. GCM will only be used
    // if both sides enable it.
    bool enable_gcm_crypto_suites = true;

    // If set to true, the (potentially insecure) crypto cipher
    // kSrtpAes128CmSha1_32 will be included in the list of supported ciphers
    // during negotiation. It will only be used if both peers support it and no
    // other ciphers get preferred.
    bool enable_aes128_sha1_32_crypto_cipher = false;

    // The most commonly used cipher. Can be disabled, mostly for testing
    // purposes.
    bool enable_aes128_sha1_80_crypto_cipher = true;

    // This feature enables encrypting RTP header extensions using RFC 6904, if
    // requested. For this to work the Chromium field trial
    // `kWebRtcEncryptedRtpHeaderExtensions` must be enabled.
    bool enable_encrypted_rtp_header_extensions = true;
  } srtp;

  // Options to be used when the FrameEncryptor / FrameDecryptor APIs are used.
  struct SFrame {
    // If set all RtpSenders must have an FrameEncryptor attached to them before
    // they are allowed to send packets. All RtpReceivers must have a
    // FrameDecryptor attached to them before they are able to receive packets.
    bool require_frame_encryption = false;
  } sframe;
};

}  // namespace webrtc

#endif  // API_CRYPTO_CRYPTO_OPTIONS_H_
