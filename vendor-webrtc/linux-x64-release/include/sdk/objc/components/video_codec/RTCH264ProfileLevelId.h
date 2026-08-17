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

#import "sdk/objc/base/RTCMacros.h"

RTC_OBJC_EXPORT extern NSString *const RTC_CONSTANT_TYPE(RTCVideoCodecH264Name);
RTC_OBJC_EXPORT extern NSString *const RTC_CONSTANT_TYPE(RTCLevel31ConstrainedHigh);
RTC_OBJC_EXPORT extern NSString *const RTC_CONSTANT_TYPE(RTCLevel31ConstrainedBaseline);
RTC_OBJC_EXPORT extern NSString *const RTC_CONSTANT_TYPE(RTCMaxSupportedH264ProfileLevelConstrainedHigh);
RTC_OBJC_EXPORT extern NSString *const RTC_CONSTANT_TYPE(RTCMaxSupportedH264ProfileLevelConstrainedBaseline);

/** H264 Profiles and levels. */
typedef NS_ENUM(NSUInteger, RTC_OBJC_TYPE(RTCH264Profile)) {
  RTC_OBJC_TYPE(RTCH264ProfileConstrainedBaseline),
  RTC_OBJC_TYPE(RTCH264ProfileBaseline),
  RTC_OBJC_TYPE(RTCH264ProfileMain),
  RTC_OBJC_TYPE(RTCH264ProfileConstrainedHigh),
  RTC_OBJC_TYPE(RTCH264ProfileHigh),
};

typedef NS_ENUM(NSUInteger, RTC_OBJC_TYPE(RTCH264Level)) {
  RTC_OBJC_TYPE(RTCH264Level1_b) = 0,
  RTC_OBJC_TYPE(RTCH264Level1) = 10,
  RTC_OBJC_TYPE(RTCH264Level1_1) = 11,
  RTC_OBJC_TYPE(RTCH264Level1_2) = 12,
  RTC_OBJC_TYPE(RTCH264Level1_3) = 13,
  RTC_OBJC_TYPE(RTCH264Level2) = 20,
  RTC_OBJC_TYPE(RTCH264Level2_1) = 21,
  RTC_OBJC_TYPE(RTCH264Level2_2) = 22,
  RTC_OBJC_TYPE(RTCH264Level3) = 30,
  RTC_OBJC_TYPE(RTCH264Level3_1) = 31,
  RTC_OBJC_TYPE(RTCH264Level3_2) = 32,
  RTC_OBJC_TYPE(RTCH264Level4) = 40,
  RTC_OBJC_TYPE(RTCH264Level4_1) = 41,
  RTC_OBJC_TYPE(RTCH264Level4_2) = 42,
  RTC_OBJC_TYPE(RTCH264Level5) = 50,
  RTC_OBJC_TYPE(RTCH264Level5_1) = 51,
  RTC_OBJC_TYPE(RTCH264Level5_2) = 52
};

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCH264ProfileLevelId) : NSObject

@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCH264Profile) profile;
@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCH264Level) level;
@property(nonatomic, readonly) NSString *hexString;

- (instancetype)initWithHexString:(NSString *)hexString;
- (instancetype)initWithProfile:(RTC_OBJC_TYPE(RTCH264Profile))profile level:(RTC_OBJC_TYPE(RTCH264Level))level;

@end
