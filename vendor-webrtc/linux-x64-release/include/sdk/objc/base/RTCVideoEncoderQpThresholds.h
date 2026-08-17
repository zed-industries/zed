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

/** QP thresholds for encoder. Corresponds to
 * webrtc::VideoEncoder::QpThresholds. */
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCVideoEncoderQpThresholds) : NSObject

- (instancetype)initWithThresholdsLow:(NSInteger)low high:(NSInteger)high;

@property(nonatomic, readonly) NSInteger low;
@property(nonatomic, readonly) NSInteger high;

@end

NS_ASSUME_NONNULL_END
