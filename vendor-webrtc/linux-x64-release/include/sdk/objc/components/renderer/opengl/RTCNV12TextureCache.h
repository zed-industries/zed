/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <GLKit/GLKit.h>

#import "sdk/objc/base/RTCMacros.h"

@class RTC_OBJC_TYPE(RTCVideoFrame);

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE(RTCNV12TextureCache) : NSObject

@property(nonatomic, readonly) GLuint yTexture;
@property(nonatomic, readonly) GLuint uvTexture;

- (instancetype)init NS_UNAVAILABLE;
- (nullable instancetype)initWithContext:(EAGLContext *)context
    NS_DESIGNATED_INITIALIZER;

- (BOOL)uploadFrameToTextures:(RTC_OBJC_TYPE(RTCVideoFrame) *)frame;

- (void)releaseTextures;

@end

NS_ASSUME_NONNULL_END
