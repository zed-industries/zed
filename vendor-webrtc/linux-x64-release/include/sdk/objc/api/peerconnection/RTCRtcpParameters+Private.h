/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCRtcpParameters.h"

#include "api/rtp_parameters.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCRtcpParameters)
()

    /** Returns the equivalent native RtcpParameters structure. */
    @property(nonatomic, readonly) webrtc::RtcpParameters nativeParameters;

/** Initialize the object with a native RtcpParameters structure. */
- (instancetype)initWithNativeParameters:
    (const webrtc::RtcpParameters &)nativeParameters NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
