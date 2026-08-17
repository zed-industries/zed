/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCRtpEncodingParameters.h"

#include "api/rtp_parameters.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCRtpEncodingParameters)
()

    /** Returns the equivalent native RtpEncodingParameters structure. */
    @property(nonatomic,
              readonly) webrtc::RtpEncodingParameters nativeParameters;

/** Initialize the object with a native RtpEncodingParameters structure. */
- (instancetype)initWithNativeParameters:
    (const webrtc::RtpEncodingParameters &)nativeParameters
    NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
