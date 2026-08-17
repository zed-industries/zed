/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCVideoFrame.h"

#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

@class RTC_OBJC_TYPE(RTCVideoCapturer);

RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE
(RTCVideoCapturerDelegate)<NSObject> -
    (void)capturer
    : (RTC_OBJC_TYPE(RTCVideoCapturer) *)capturer didCaptureVideoFrame
    : (RTC_OBJC_TYPE(RTCVideoFrame) *)frame;
@end

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCVideoCapturer) : NSObject

@property(nonatomic, weak) id<RTC_OBJC_TYPE(RTCVideoCapturerDelegate)> delegate;

- (instancetype)initWithDelegate:
    (id<RTC_OBJC_TYPE(RTCVideoCapturerDelegate)>)delegate;

@end

NS_ASSUME_NONNULL_END
