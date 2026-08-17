/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "base/RTCVideoEncoder.h"
#import "sdk/objc/base/RTCMacros.h"

// NativeVideoEncoder pretends to conform to RTCVideoEncoder protocol, but
// expects its methods won't be called.
@interface RTC_OBJC_TYPE (RTCNativeVideoEncoder) : NSObject <RTC_OBJC_TYPE (RTCVideoEncoder)>

@end
