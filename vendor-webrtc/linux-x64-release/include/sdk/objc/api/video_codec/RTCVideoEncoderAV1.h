/*
 *  Copyright 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "RTCVideoEncoder.h"
#import "sdk/objc/base/RTCMacros.h"

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCVideoEncoderAV1) : NSObject

/* This returns a AV1 encoder that can be returned from a RTCVideoEncoderFactory injected into
 * RTCPeerConnectionFactory. Even though it implements the RTCVideoEncoder protocol, it can not be
 * used independently from the RTCPeerConnectionFactory.
 */
+ (nonnull id<RTC_OBJC_TYPE(RTCVideoEncoder)>)av1Encoder;

/* Returns list of scalability modes supported by the encoder that can be
 * created with `av1Encoder` method above.
 */
+ (nonnull NSArray<NSString*>*)supportedScalabilityModes;

+ (bool)isSupported;

+ (nonnull NSArray<NSString *> *)scalabilityModes;

@end
