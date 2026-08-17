/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

/** The only valid value for the following if set is kRTCFieldTrialEnabledValue. */
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCFieldTrialAudioForceABWENoTWCCKey);
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCFieldTrialFlexFec03AdvertisedKey);
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCFieldTrialFlexFec03Key);
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCFieldTrialH264HighProfileKey);
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCFieldTrialMinimizeResamplingOnMobileKey);
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCFieldTrialUseNWPathMonitor);

/** The valid value for field trials above. */
RTC_EXTERN NSString *const RTC_CONSTANT_TYPE(RTCFieldTrialEnabledValue);

/** Initialize field trials using a dictionary mapping field trial keys to their
 * values. See above for valid keys and values. Must be called before any other
 * call into WebRTC. See: webrtc/system_wrappers/include/field_trial.h
 */
RTC_EXTERN void RTC_OBJC_TYPE(RTCInitFieldTrialDictionary)(NSDictionary<NSString *, NSString *> *fieldTrials);
