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

#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCMetricsSampleInfo) : NSObject

/**
 * Example of RTCMetricsSampleInfo:
 * name: "WebRTC.Video.InputFramesPerSecond"
 * min: 1
 * max: 100
 * bucketCount: 50
 * samples: [29]:2 [30]:1
 */

/** The name of the histogram. */
@property(nonatomic, readonly) NSString *name;

/** The minimum bucket value. */
@property(nonatomic, readonly) int min;

/** The maximum bucket value. */
@property(nonatomic, readonly) int max;

/** The number of buckets. */
@property(nonatomic, readonly) int bucketCount;

/** A dictionary holding the samples <value, # of events>. */
@property(nonatomic, readonly) NSDictionary<NSNumber *, NSNumber *> *samples;

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
