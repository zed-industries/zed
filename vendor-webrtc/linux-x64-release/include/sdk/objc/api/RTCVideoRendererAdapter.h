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

#import "RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

/*
 * Creates a webrtc::VideoSinkInterface surface for an RTCVideoRenderer. The
 * webrtc::VideoSinkInterface is used by WebRTC rendering code - this
 * adapter adapts calls made to that interface to the RTCVideoRenderer supplied
 * during construction.
 */
@interface RTC_OBJC_TYPE (RTCVideoRendererAdapter): NSObject

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
