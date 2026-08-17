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

NS_ASSUME_NONNULL_BEGIN

/** Holds information to identify a codec. Corresponds to
 * webrtc::SdpVideoFormat. */
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCVideoCodecInfo) : NSObject <NSCoding>

- (instancetype)init NS_UNAVAILABLE;

- (instancetype)initWithName:(NSString *)name;

- (instancetype)initWithName:(NSString *)name
                  parameters:(nullable NSDictionary<NSString *, NSString *> *)
                                 parameters;

- (instancetype)initWithName:(NSString *)name
                  parameters:(nullable NSDictionary<NSString *, NSString *> *)parameters
             scalabilityModes:(nullable NSArray<NSString *> *)scalabilityModes
    NS_DESIGNATED_INITIALIZER;

- (BOOL)isEqualToCodecInfo:(RTC_OBJC_TYPE(RTCVideoCodecInfo) *)info;

@property(nonatomic, readonly) NSString *name;
@property(nonatomic, readonly) NSDictionary<NSString *, NSString *> *parameters;
@property(nonatomic, readonly) NSArray<NSString *> *scalabilityModes;

@end

NS_ASSUME_NONNULL_END
