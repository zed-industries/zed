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

#import "sdk/objc/base/RTCMacros.h"

NS_ASSUME_NONNULL_BEGIN

@protocol RTC_OBJC_TYPE
(RTCVideoRenderer);
@class RTC_OBJC_TYPE(RTCPeerConnectionFactory);
@class RTC_OBJC_TYPE(RTCVideoSource);

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCVideoTrack) : RTC_OBJC_TYPE(RTCMediaStreamTrack)

/** The video source for this video track. */
@property(nonatomic, readonly) RTC_OBJC_TYPE(RTCVideoSource) *source;

/** The receive state, if this is a remote video track. */
@property(nonatomic, assign) BOOL shouldReceive;

- (instancetype)init NS_UNAVAILABLE;

/** Register a renderer that will render all frames received on this track. */
- (void)addRenderer:(id<RTC_OBJC_TYPE(RTCVideoRenderer)>)renderer;

/** Deregister a renderer. */
- (void)removeRenderer:(id<RTC_OBJC_TYPE(RTCVideoRenderer)>)renderer;

@end

NS_ASSUME_NONNULL_END
