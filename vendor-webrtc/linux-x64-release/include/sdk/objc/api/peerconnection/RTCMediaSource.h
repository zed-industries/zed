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

typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCSourceState)) {
  RTC_OBJC_TYPE(RTCSourceStateInitializing),
  RTC_OBJC_TYPE(RTCSourceStateLive),
  RTC_OBJC_TYPE(RTCSourceStateEnded),
  RTC_OBJC_TYPE(RTCSourceStateMuted),
};

NS_ASSUME_NONNULL_BEGIN

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCMediaSource) : NSObject

/** The current state of the RTCMediaSource. */
@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCSourceState) state;

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
