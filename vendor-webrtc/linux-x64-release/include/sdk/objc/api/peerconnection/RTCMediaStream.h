/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
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

@class RTC_OBJC_TYPE(RTCAudioTrack);
@class RTC_OBJC_TYPE(RTCPeerConnectionFactory);
@class RTC_OBJC_TYPE(RTCVideoTrack);

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCMediaStream) : NSObject

/** The audio tracks in this stream. */
@property(nonatomic, strong, readonly) NSArray<RTC_OBJC_TYPE(RTCAudioTrack) *> *audioTracks;

/** The video tracks in this stream. */
@property(nonatomic, strong, readonly)
    NSArray<RTC_OBJC_TYPE(RTCVideoTrack) *> *videoTracks;

/** An identifier for this media stream. */
@property(nonatomic, readonly) NSString *streamId;

- (instancetype)init NS_UNAVAILABLE;

/** Adds the given audio track to this media stream. */
- (void)addAudioTrack:(RTC_OBJC_TYPE(RTCAudioTrack) *)audioTrack;

/** Adds the given video track to this media stream. */
- (void)addVideoTrack:(RTC_OBJC_TYPE(RTCVideoTrack) *)videoTrack;

/** Removes the given audio track to this media stream. */
- (void)removeAudioTrack:(RTC_OBJC_TYPE(RTCAudioTrack) *)audioTrack;

/** Removes the given video track to this media stream. */
- (void)removeVideoTrack:(RTC_OBJC_TYPE(RTCVideoTrack) *)videoTrack;

@end

NS_ASSUME_NONNULL_END
