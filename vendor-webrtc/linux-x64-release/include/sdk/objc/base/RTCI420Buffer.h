/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <AVFoundation/AVFoundation.h>

#import "RTCYUVPlanarBuffer.h"

NS_ASSUME_NONNULL_BEGIN

/** Protocol for RTCYUVPlanarBuffers containing I420 data */
RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE
(RTCI420Buffer)<RTC_OBJC_TYPE(RTCYUVPlanarBuffer)> @end

NS_ASSUME_NONNULL_END
