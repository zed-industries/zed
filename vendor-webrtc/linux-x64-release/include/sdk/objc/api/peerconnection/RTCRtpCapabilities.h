/*
 *  Copyright 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

@class RTC_OBJC_TYPE(RTCRtpCodecCapability);
@class RTC_OBJC_TYPE(RTCRtpHeaderExtensionCapability);

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCRtpCapabilities) : NSObject

@property(nonatomic, copy) NSArray<RTC_OBJC_TYPE(RTCRtpCodecCapability) *> *codecs;
@property(nonatomic, copy)
    NSArray<RTC_OBJC_TYPE(RTCRtpHeaderExtensionCapability) *> *headerExtensions;

- (instancetype)init;

@end

NS_ASSUME_NONNULL_END
