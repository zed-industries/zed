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

#import "RTCVideoCodecInfo.h"
#import "RTCVideoDecoder.h"
#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

/** RTCVideoDecoderFactory is an Objective-C version of
 * webrtc::VideoDecoderFactory.
 */
RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE
(RTCVideoDecoderFactory)<NSObject>

    - (nullable id<RTC_OBJC_TYPE(RTCVideoDecoder)>)createDecoder
    : (RTC_OBJC_TYPE(RTCVideoCodecInfo) *)info;
- (NSArray<RTC_OBJC_TYPE(RTCVideoCodecInfo) *> *)
    supportedCodecs;  // TODO(andersc): "supportedFormats" instead?

@end

NS_ASSUME_NONNULL_END
