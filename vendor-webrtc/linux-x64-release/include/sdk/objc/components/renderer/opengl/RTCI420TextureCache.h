/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCOpenGLDefines.h"
#import "base/RTCVideoFrame.h"

@interface RTC_OBJC_TYPE(RTCI420TextureCache) : NSObject

@property(nonatomic, readonly) GLuint yTexture;
@property(nonatomic, readonly) GLuint uTexture;
@property(nonatomic, readonly) GLuint vTexture;

- (instancetype)init NS_UNAVAILABLE;
- (instancetype)initWithContext:(GlContextType *)context
    NS_DESIGNATED_INITIALIZER;

- (void)uploadFrameToTextures:(RTC_OBJC_TYPE(RTCVideoFrame) *)frame;

@end
