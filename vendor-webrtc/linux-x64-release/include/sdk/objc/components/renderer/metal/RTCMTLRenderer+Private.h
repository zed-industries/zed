/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Metal/Metal.h>

#import "RTCMTLRenderer.h"

#define MTL_STRINGIFY(s) @ #s

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE(RTCMTLRenderer) (Private)
- (nullable id<MTLDevice>)currentMetalDevice;
- (NSString *)shaderSource;
- (BOOL)setupTexturesForFrame:(nonnull RTC_OBJC_TYPE(RTCVideoFrame) *)frame;
- (void)uploadTexturesToRenderEncoder:
    (id<MTLRenderCommandEncoder>)renderEncoder;
- (void)getWidth:(nonnull int *)width
          height:(nonnull int *)height
       cropWidth:(nonnull int *)cropWidth
      cropHeight:(nonnull int *)cropHeight
           cropX:(nonnull int *)cropX
           cropY:(nonnull int *)cropY
         ofFrame:(nonnull RTC_OBJC_TYPE(RTCVideoFrame) *)frame;
@end

NS_ASSUME_NONNULL_END
