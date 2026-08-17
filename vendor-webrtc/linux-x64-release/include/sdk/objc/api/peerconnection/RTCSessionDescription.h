/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

/**
 * Represents the session description type. This exposes the same types that are
 * in C++, which doesn't include the rollback type that is in the W3C spec.
 */
typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCSdpType)) {
  RTC_OBJC_TYPE(RTCSdpTypeOffer),
  RTC_OBJC_TYPE(RTCSdpTypePrAnswer),
  RTC_OBJC_TYPE(RTCSdpTypeAnswer),
  RTC_OBJC_TYPE(RTCSdpTypeRollback),
};

NS_ASSUME_NONNULL_BEGIN

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCSessionDescription) : NSObject

/** The type of session description. */
@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCSdpType) type;

/** The SDP string representation of this session description. */
@property(nonatomic, readonly) NSString *sdp;

- (instancetype)init NS_UNAVAILABLE;

/** Initialize a session description with a type and SDP string. */
- (instancetype)initWithType:(RTC_OBJC_TYPE(RTCSdpType))type sdp:(NSString *)sdp NS_DESIGNATED_INITIALIZER;

+ (NSString *)stringForType:(RTC_OBJC_TYPE(RTCSdpType))type;

+ (RTC_OBJC_TYPE(RTCSdpType))typeForString:(NSString *)string;

@end

NS_ASSUME_NONNULL_END
