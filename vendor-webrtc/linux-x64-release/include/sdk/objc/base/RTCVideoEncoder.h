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

#import "RTCCodecSpecificInfo.h"
#import "RTCEncodedImage.h"
#import "RTCVideoEncoderQpThresholds.h"
#import "RTCVideoEncoderSettings.h"
#import "RTCVideoFrame.h"
#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

/** Callback block for encoder. */
typedef BOOL (^RTCVideoEncoderCallback)(
    RTC_OBJC_TYPE(RTCEncodedImage) * frame,
    id<RTC_OBJC_TYPE(RTCCodecSpecificInfo)> info);

/** Protocol for encoder implementations. */
RTC_OBJC_EXPORT
@protocol RTC_OBJC_TYPE
(RTCVideoEncoder)<NSObject>

    - (void)setCallback : (nullable RTCVideoEncoderCallback)callback;
- (NSInteger)startEncodeWithSettings:
                 (RTC_OBJC_TYPE(RTCVideoEncoderSettings) *)settings
                       numberOfCores:(int)numberOfCores;
- (NSInteger)releaseEncoder;
- (NSInteger)encode:(RTC_OBJC_TYPE(RTCVideoFrame) *)frame
    codecSpecificInfo:(nullable id<RTC_OBJC_TYPE(RTCCodecSpecificInfo)>)info
           frameTypes:(NSArray<NSNumber *> *)frameTypes;
- (int)setBitrate:(uint32_t)bitrateKbit framerate:(uint32_t)framerate;
- (NSString *)implementationName;

/** Returns QP scaling settings for encoder. The quality scaler adjusts the
 * resolution in order to keep the QP from the encoded images within the given
 * range. Returning nil from this function disables quality scaling. */
- (nullable RTC_OBJC_TYPE(RTCVideoEncoderQpThresholds) *)scalingSettings;

/** Resolutions should be aligned to this value. */
@property(nonatomic, readonly) NSInteger resolutionAlignment;

/** If enabled, resolution alignment is applied to all simulcast layers
   simultaneously so that when scaled, all resolutions comply with
   'resolutionAlignment'. */
@property(nonatomic, readonly) BOOL applyAlignmentToAllSimulcastLayers;

/** If YES, the receiver is expected to resample/scale the source texture to the
   expected output size. */
@property(nonatomic, readonly) BOOL supportsNativeHandle;

@end

NS_ASSUME_NONNULL_END
