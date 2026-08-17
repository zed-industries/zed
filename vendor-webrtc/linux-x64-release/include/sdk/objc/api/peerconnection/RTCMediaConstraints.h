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

/** Constraint keys for media sources. */
/** The value for this key should be a base64 encoded string containing
 *  the data from the serialized configuration proto.
 */
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCMediaConstraintsAudioNetworkAdaptorConfig);

/** Constraint keys for generating offers and answers. */
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCMediaConstraintsIceRestart);
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCMediaConstraintsOfferToReceiveAudio);
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCMediaConstraintsOfferToReceiveVideo);
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCMediaConstraintsVoiceActivityDetection);

/** Constraint values for Boolean parameters. */
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCMediaConstraintsValueTrue);
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCMediaConstraintsValueFalse);

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCMediaConstraints) : NSObject

- (instancetype)init NS_UNAVAILABLE;

/** Initialize with mandatory and/or optional constraints. */
- (instancetype)initWithMandatoryConstraints:
                    (nullable NSDictionary<NSString *, NSString *> *)mandatory
                         optionalConstraints:
                             (nullable NSDictionary<NSString *, NSString *> *)
                                 optional NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
