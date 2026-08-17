/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCRtpHeaderExtension.h"

#include "api/rtp_parameters.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCRtpHeaderExtension)
()

    /** Returns the equivalent native RtpExtension structure. */
    @property(nonatomic, readonly) webrtc::RtpExtension nativeParameters;

/** Initialize the object with a native RtpExtension structure. */
- (instancetype)initWithNativeParameters:
    (const webrtc::RtpExtension &)nativeParameters NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
