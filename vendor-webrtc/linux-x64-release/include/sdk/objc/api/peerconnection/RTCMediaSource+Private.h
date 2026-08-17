/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCMediaSource.h"

#include "api/media_stream_interface.h"

NS_ASSUME_NONNULL_BEGIN

@class RTC_OBJC_TYPE(RTCPeerConnectionFactory);

typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCMediaSourceType)) {
  RTC_OBJC_TYPE(RTCMediaSourceTypeAudio),
  RTC_OBJC_TYPE(RTCMediaSourceTypeVideo),
};

@interface RTC_OBJC_TYPE (RTCMediaSource)
()

    @property(nonatomic, readonly)
        webrtc::scoped_refptr<webrtc::MediaSourceInterface>
            nativeMediaSource;

- (instancetype)initWithFactory:(RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory
              nativeMediaSource:(webrtc::scoped_refptr<webrtc::MediaSourceInterface>)nativeMediaSource
                           type:(RTC_OBJC_TYPE(RTCMediaSourceType))type NS_DESIGNATED_INITIALIZER;

+ (webrtc::MediaSourceInterface::SourceState)nativeSourceStateForState:(RTC_OBJC_TYPE(RTCSourceState))state;

+ (RTC_OBJC_TYPE(RTCSourceState))sourceStateForNativeState:(webrtc::MediaSourceInterface::SourceState)nativeState;

+ (NSString *)stringForState:(RTC_OBJC_TYPE(RTCSourceState))state;

@end

NS_ASSUME_NONNULL_END
