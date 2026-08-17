/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * Objective-C bindings for webrtc::CryptoOptions. This API had to be flattened
 * as Objective-C doesn't support nested structures.
 */
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCCryptoOptions) : NSObject

/**
 * Enable GCM crypto suites from RFC 7714 for SRTP. GCM will only be used
 * if both sides enable it
 */
@property(nonatomic, assign) BOOL srtpEnableGcmCryptoSuites;
/**
 * If set to true, the (potentially insecure) crypto cipher
 * kSrtpAes128CmSha1_32 will be included in the list of supported ciphers
 * during negotiation. It will only be used if both peers support it and no
 * other ciphers get preferred.
 */
@property(nonatomic, assign) BOOL srtpEnableAes128Sha1_32CryptoCipher;
/**
 * If set to true, encrypted RTP header extensions as defined in RFC 6904
 * will be negotiated. They will only be used if both peers support them.
 */
@property(nonatomic, assign) BOOL srtpEnableEncryptedRtpHeaderExtensions;

/**
 * If set all RtpSenders must have an FrameEncryptor attached to them before
 * they are allowed to send packets. All RtpReceivers must have a
 * FrameDecryptor attached to them before they are able to receive packets.
 */
@property(nonatomic, assign) BOOL sframeRequireFrameEncryption;

/**
 * Initializes CryptoOptions with all possible options set explicitly. This
 * is done when converting from a native RTCConfiguration.crypto_options.
 */
- (instancetype)
         initWithSrtpEnableGcmCryptoSuites:(BOOL)srtpEnableGcmCryptoSuites
       srtpEnableAes128Sha1_32CryptoCipher:
           (BOOL)srtpEnableAes128Sha1_32CryptoCipher
    srtpEnableEncryptedRtpHeaderExtensions:
        (BOOL)srtpEnableEncryptedRtpHeaderExtensions
              sframeRequireFrameEncryption:(BOOL)sframeRequireFrameEncryption
    NS_DESIGNATED_INITIALIZER;

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
