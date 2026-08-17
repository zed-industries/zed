/*
 *  Copyright 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCRtpCodecCapability.h"

#include "api/rtp_parameters.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCRtpCodecCapability)
()

    /**
     * The native RtpCodecCapability representation of this
     * RTCRtpCodecCapability object. This is needed to pass to the underlying
     * C++ APIs.
     */
    @property(nonatomic,
              readonly) webrtc::RtpCodecCapability nativeRtpCodecCapability;

/**
 * Initialize an RTCRtpCodecCapability from a native RtpCodecCapability.
 */
- (instancetype)initWithNativeRtpCodecCapability:
    (const webrtc::RtpCodecCapability &)nativeRtpCodecCapability;

@end

NS_ASSUME_NONNULL_END
