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

typedef NS_ENUM(NSUInteger, RTC_OBJC_TYPE(RTCVideoCodecMode)) {
  RTC_OBJC_TYPE(RTCVideoCodecModeRealtimeVideo),
  RTC_OBJC_TYPE(RTCVideoCodecModeScreensharing),
};

/** Settings for encoder. Corresponds to webrtc::VideoCodec. */
RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCVideoEncoderSettings) : NSObject

@property(nonatomic, strong) NSString *name;

@property(nonatomic, assign) unsigned short width;
@property(nonatomic, assign) unsigned short height;

@property(nonatomic, assign) unsigned int startBitrate;  // kilobits/sec.
@property(nonatomic, assign) unsigned int maxBitrate;
@property(nonatomic, assign) unsigned int minBitrate;

@property(nonatomic, assign) uint32_t maxFramerate;

@property(nonatomic, assign) unsigned int qpMax;
@property(nonatomic, assign) RTC_OBJC_TYPE(RTCVideoCodecMode) mode;

@end

NS_ASSUME_NONNULL_END
