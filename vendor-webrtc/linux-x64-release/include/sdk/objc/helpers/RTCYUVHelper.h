/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "RTCMacros.h"
#import "RTCVideoFrame.h"

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCYUVHelper) : NSObject

- (instancetype)init NS_UNAVAILABLE;

+ (void)I420Rotate:(const uint8_t*)srcY
        srcStrideY:(int)srcStrideY
              srcU:(const uint8_t*)srcU
        srcStrideU:(int)srcStrideU
              srcV:(const uint8_t*)srcV
        srcStrideV:(int)srcStrideV
              dstY:(uint8_t*)dstY
        dstStrideY:(int)dstStrideY
              dstU:(uint8_t*)dstU
        dstStrideU:(int)dstStrideU
              dstV:(uint8_t*)dstV
        dstStrideV:(int)dstStrideV
             width:(int)width
             height:(int)height
              mode:(RTC_OBJC_TYPE(RTCVideoRotation))mode;

+ (int)I420ToNV12:(const uint8_t*)srcY
       srcStrideY:(int)srcStrideY
             srcU:(const uint8_t*)srcU
       srcStrideU:(int)srcStrideU
             srcV:(const uint8_t*)srcV
       srcStrideV:(int)srcStrideV
             dstY:(uint8_t*)dstY
       dstStrideY:(int)dstStrideY
            dstUV:(uint8_t*)dstUV
      dstStrideUV:(int)dstStrideUV
            width:(int)width
            height:(int)height;

+ (int)I420ToNV21:(const uint8_t*)srcY
       srcStrideY:(int)srcStrideY
             srcU:(const uint8_t*)srcU
       srcStrideU:(int)srcStrideU
             srcV:(const uint8_t*)srcV
       srcStrideV:(int)srcStrideV
             dstY:(uint8_t*)dstY
       dstStrideY:(int)dstStrideY
            dstUV:(uint8_t*)dstUV
      dstStrideUV:(int)dstStrideUV
            width:(int)width
            height:(int)height;

+ (int)I420ToARGB:(const uint8_t*)srcY
       srcStrideY:(int)srcStrideY
             srcU:(const uint8_t*)srcU
       srcStrideU:(int)srcStrideU
             srcV:(const uint8_t*)srcV
       srcStrideV:(int)srcStrideV
          dstARGB:(uint8_t*)dstARGB
    dstStrideARGB:(int)dstStrideARGB
            width:(int)width
           height:(int)height;

+ (int)I420ToBGRA:(const uint8_t*)srcY
       srcStrideY:(int)srcStrideY
             srcU:(const uint8_t*)srcU
       srcStrideU:(int)srcStrideU
             srcV:(const uint8_t*)srcV
       srcStrideV:(int)srcStrideV
          dstBGRA:(uint8_t*)dstBGRA
    dstStrideBGRA:(int)dstStrideBGRA
            width:(int)width
           height:(int)height;

+ (int)I420ToABGR:(const uint8_t*)srcY
       srcStrideY:(int)srcStrideY
             srcU:(const uint8_t*)srcU
       srcStrideU:(int)srcStrideU
             srcV:(const uint8_t*)srcV
       srcStrideV:(int)srcStrideV
          dstABGR:(uint8_t*)dstABGR
    dstStrideABGR:(int)dstStrideABGR
            width:(int)width
           height:(int)height;

+ (int)I420ToRGBA:(const uint8_t*)srcY
       srcStrideY:(int)srcStrideY
             srcU:(const uint8_t*)srcU
       srcStrideU:(int)srcStrideU
             srcV:(const uint8_t*)srcV
       srcStrideV:(int)srcStrideV
          dstRGBA:(uint8_t*)dstRGBA
    dstStrideRGBA:(int)dstStrideRGBA
            width:(int)width
           height:(int)height;

+ (int)I420ToRGB24:(const uint8_t*)srcY
        srcStrideY:(int)srcStrideY
              srcU:(const uint8_t*)srcU
        srcStrideU:(int)srcStrideU
              srcV:(const uint8_t*)srcV
        srcStrideV:(int)srcStrideV
          dstRGB24:(uint8_t*)dstRGB24
    dstStrideRGB24:(int)dstStrideRGB24
             width:(int)width
            height:(int)height;

@end
