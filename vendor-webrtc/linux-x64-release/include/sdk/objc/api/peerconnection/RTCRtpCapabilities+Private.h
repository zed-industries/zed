/*
 *  Copyright 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCRtpCapabilities.h"

#include "api/rtp_parameters.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCRtpCapabilities)
()

    /**
     * The native RtpCapabilities representation of this RTCRtpCapabilities
     * object. This is needed to pass to the underlying C++ APIs.
     */
    @property(nonatomic,
              readonly) webrtc::RtpCapabilities nativeRtpCapabilities;

/**
 * Initialize an RTCRtpCapabilities from a native RtpCapabilities.
 */
- (instancetype)initWithNativeRtpCapabilities:
    (const webrtc::RtpCapabilities &)rtpCapabilities;

@end

NS_ASSUME_NONNULL_END
