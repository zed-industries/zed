/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <AVFoundation/AVFoundation.h>

#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

@protocol RTC_OBJC_TYPE
(RTCI420Buffer);

// RTCVideoFrameBuffer is an ObjectiveC version of webrtc::VideoFrameBuffer.
RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE
(RTCVideoFrameBuffer)<NSObject>

    @property(nonatomic, readonly) int width;
@property(nonatomic, readonly) int height;

- (id<RTC_OBJC_TYPE(RTCI420Buffer)>)toI420;

@optional
- (id<RTC_OBJC_TYPE(RTCVideoFrameBuffer)>)cropAndScaleWith:(int)offsetX
                                                   offsetY:(int)offsetY
                                                 cropWidth:(int)cropWidth
                                                cropHeight:(int)cropHeight
                                                scaleWidth:(int)scaleWidth
                                               scaleHeight:(int)scaleHeight;

@end

NS_ASSUME_NONNULL_END
