/*
 *  Copyright 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCRtpSource.h"

#include "api/transport/rtp/rtp_source.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCRtpSource)
()

    /** Initialize an RTCRtpSource with a native RtpSource. */
    - (instancetype)initWithNativeRtpSource
    : (const webrtc::RtpSource&)nativeRtpSource NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
