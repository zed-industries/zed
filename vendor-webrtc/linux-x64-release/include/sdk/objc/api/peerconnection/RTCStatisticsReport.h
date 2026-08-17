/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

@class RTC_OBJC_TYPE(RTCStatistics);

NS_ASSUME_NONNULL_BEGIN

/** A statistics report. Encapsulates a number of RTCStatistics objects. */
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCStatisticsReport) : NSObject

/** The timestamp of the report in microseconds since 1970-01-01T00:00:00Z. */
@property(nonatomic, readonly) CFTimeInterval timestamp_us;

/** RTCStatistics objects by id. */
@property(nonatomic, readonly)
    NSDictionary<NSString *, RTC_OBJC_TYPE(RTCStatistics) *> *statistics;

- (instancetype)init NS_UNAVAILABLE;

@end

/** A part of a report (a subreport) covering a certain area. */
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCStatistics) : NSObject

/** The id of this subreport, e.g. "RTCMediaStreamTrack_receiver_2". */
@property(nonatomic, readonly) NSString *id;

/** The timestamp of the subreport in microseconds since 1970-01-01T00:00:00Z.
 */
@property(nonatomic, readonly) CFTimeInterval timestamp_us;

/** The type of the subreport, e.g. "track", "codec". */
@property(nonatomic, readonly) NSString *type;

/** The keys and values of the subreport, e.g. "totalFramesDuration = 5.551".
    The values are either NSNumbers or NSStrings or NSArrays encapsulating
   NSNumbers or NSStrings, or NSDictionary of NSString keys to NSNumber values.
 */
@property(nonatomic, readonly) NSDictionary<NSString *, NSObject *> *values;

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
