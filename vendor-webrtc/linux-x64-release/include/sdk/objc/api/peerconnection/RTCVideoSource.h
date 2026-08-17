/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "RTCMediaSource.h"
#import "RTCVideoCapturer.h"
#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

RTC_OBJC_EXPORT

@interface RTC_OBJC_TYPE (RTCVideoSource) : RTC_OBJC_TYPE(RTCMediaSource) <RTC_OBJC_TYPE(RTCVideoCapturerDelegate)>

- (instancetype)init NS_UNAVAILABLE;

/**
 * Calling this function will cause frames to be scaled down to the
 * requested resolution. Also, frames will be cropped to match the
 * requested aspect ratio, and frames will be dropped to match the
 * requested fps. The requested aspect ratio is orientation agnostic and
 * will be adjusted to maintain the input orientation, so it doesn't
 * matter if e.g. 1280x720 or 720x1280 is requested.
 */
- (void)adaptOutputFormatToWidth:(int)width height:(int)height fps:(int)fps;

@end

NS_ASSUME_NONNULL_END
