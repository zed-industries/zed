/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "RTCMTLRenderer.h"
#import "RTCMacros.h"

/** @abstract RGB/BGR renderer.
 *  @discussion This renderer handles both kCVPixelFormatType_32BGRA and
 * kCVPixelFormatType_32ARGB.
 */
NS_AVAILABLE(10_11, 9_0)
@interface RTC_OBJC_TYPE (RTCMTLRGBRenderer): RTC_OBJC_TYPE(RTCMTLRenderer)

@end
