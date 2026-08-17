/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCAudioSource.h"

#import "RTCMediaSource+Private.h"

@interface RTC_OBJC_TYPE (RTCAudioSource)
()

    /**
     * The AudioSourceInterface object passed to this RTCAudioSource during
     * construction.
     */
    @property(nonatomic, readonly)
        webrtc::scoped_refptr<webrtc::AudioSourceInterface>
            nativeAudioSource;

/** Initialize an RTCAudioSource from a native AudioSourceInterface. */
- (instancetype)
      initWithFactory:(RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory
    nativeAudioSource:
        (webrtc::scoped_refptr<webrtc::AudioSourceInterface>)nativeAudioSource
    NS_DESIGNATED_INITIALIZER;

- (instancetype)
      initWithFactory:(RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory
    nativeMediaSource:
        (webrtc::scoped_refptr<webrtc::MediaSourceInterface>)nativeMediaSource
                 type:(RTC_OBJC_TYPE(RTCMediaSourceType))type NS_UNAVAILABLE;

@end
