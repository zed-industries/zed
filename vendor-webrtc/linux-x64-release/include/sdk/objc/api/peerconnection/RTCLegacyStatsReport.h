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

NS_ASSUME_NONNULL_BEGIN

/** This does not currently conform to the spec. */
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCLegacyStatsReport) : NSObject

/** Time since 1970-01-01T00:00:00Z in milliseconds. */
@property(nonatomic, readonly) CFTimeInterval timestamp;

/** The type of stats held by this object. */
@property(nonatomic, readonly) NSString *type;

/** The identifier for this object. */
@property(nonatomic, readonly) NSString *reportId;

/** A dictionary holding the actual stats. */
@property(nonatomic, readonly) NSDictionary<NSString *, NSString *> *values;

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
