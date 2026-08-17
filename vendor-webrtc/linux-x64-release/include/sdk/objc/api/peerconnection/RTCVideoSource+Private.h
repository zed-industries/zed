/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCVideoSource.h"

#import "RTCMediaSource+Private.h"

#include "api/media_stream_interface.h"
#include "rtc_base/thread.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCVideoSource)
()

    /**
     * The VideoTrackSourceInterface object passed to this RTCVideoSource during
     * construction.
     */
    @property(nonatomic, readonly)
        webrtc::scoped_refptr<webrtc::VideoTrackSourceInterface>
            nativeVideoSource;

/** Initialize an RTCVideoSource from a native VideoTrackSourceInterface. */
- (instancetype)
      initWithFactory:(RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory
    nativeVideoSource:(webrtc::scoped_refptr<webrtc::VideoTrackSourceInterface>)
                          nativeVideoSource NS_DESIGNATED_INITIALIZER;

- (instancetype)
      initWithFactory:(RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory
    nativeMediaSource:
        (webrtc::scoped_refptr<webrtc::MediaSourceInterface>)nativeMediaSource
                 type:(RTC_OBJC_TYPE(RTCMediaSourceType))type NS_UNAVAILABLE;

- (instancetype)initWithFactory:
                    (RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory
                signalingThread:(webrtc::Thread *)signalingThread
                   workerThread:(webrtc::Thread *)workerThread;

- (instancetype)initWithFactory:
                    (RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory
                signalingThread:(webrtc::Thread *)signalingThread
                   workerThread:(webrtc::Thread *)workerThread
                   isScreenCast:(BOOL)isScreenCast;

@end

NS_ASSUME_NONNULL_END
