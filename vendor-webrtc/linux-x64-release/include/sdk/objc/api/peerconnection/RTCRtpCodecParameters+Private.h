/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCRtpCodecParameters.h"

#include "api/rtp_parameters.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCRtpCodecParameters)
()

    /** Returns the equivalent native RtpCodecParameters structure. */
    @property(nonatomic, readonly) webrtc::RtpCodecParameters nativeParameters;

/** Initialize the object with a native RtpCodecParameters structure. */
- (instancetype)initWithNativeParameters:
    (const webrtc::RtpCodecParameters &)nativeParameters
    NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
