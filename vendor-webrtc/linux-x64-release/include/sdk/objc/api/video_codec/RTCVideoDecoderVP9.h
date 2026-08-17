/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "RTCVideoDecoder.h"
#import "sdk/objc/base/RTCMacros.h"

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCVideoDecoderVP9) : NSObject

/* This returns a VP9 decoder that can be returned from a RTCVideoDecoderFactory injected into
 * RTCPeerConnectionFactory. Even though it implements the RTCVideoDecoder protocol, it can not be
 * used independently from the RTCPeerConnectionFactory.
 */
+ (id<RTC_OBJC_TYPE(RTCVideoDecoder)>)vp9Decoder;

+ (bool)isSupported;

@end
