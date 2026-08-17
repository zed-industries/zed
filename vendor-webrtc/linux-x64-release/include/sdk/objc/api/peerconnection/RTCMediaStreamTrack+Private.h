/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCMediaStreamTrack.h"

#include "api/media_stream_interface.h"

typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCMediaStreamTrackType)) {
  RTC_OBJC_TYPE(RTCMediaStreamTrackTypeAudio),
  RTC_OBJC_TYPE(RTCMediaStreamTrackTypeVideo),
};

NS_ASSUME_NONNULL_BEGIN

@class RTC_OBJC_TYPE(RTCPeerConnectionFactory);

@interface RTC_OBJC_TYPE (RTCMediaStreamTrack)
()

        @property(nonatomic, readonly) RTC_OBJC_TYPE(RTCPeerConnectionFactory) *
    factory;

/**
 * The native MediaStreamTrackInterface passed in or created during
 * construction.
 */
@property(nonatomic, readonly)
    webrtc::scoped_refptr<webrtc::MediaStreamTrackInterface>
        nativeTrack;

/**
 * Initialize an RTCMediaStreamTrack from a native MediaStreamTrackInterface.
 */
- (instancetype)initWithFactory:(RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory
                    nativeTrack:(webrtc::scoped_refptr<webrtc::MediaStreamTrackInterface>)nativeTrack
                           type:(RTC_OBJC_TYPE(RTCMediaStreamTrackType))type NS_DESIGNATED_INITIALIZER;

- (instancetype)
    initWithFactory:(RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory
        nativeTrack:(webrtc::scoped_refptr<webrtc::MediaStreamTrackInterface>)
                        nativeTrack;

- (BOOL)isEqualToTrack:(RTC_OBJC_TYPE(RTCMediaStreamTrack) *)track;

+ (webrtc::MediaStreamTrackInterface::TrackState)nativeTrackStateForState:
    (RTC_OBJC_TYPE(RTCMediaStreamTrackState))state;

+ (RTC_OBJC_TYPE(RTCMediaStreamTrackState))trackStateForNativeState:
    (webrtc::MediaStreamTrackInterface::TrackState)nativeState;

+ (NSString *)stringForState:(RTC_OBJC_TYPE(RTCMediaStreamTrackState))state;

+ (RTC_OBJC_TYPE(RTCMediaStreamTrack) *)
    mediaTrackForNativeTrack:
        (webrtc::scoped_refptr<webrtc::MediaStreamTrackInterface>)nativeTrack
                     factory:(RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory;

@end

NS_ASSUME_NONNULL_END
