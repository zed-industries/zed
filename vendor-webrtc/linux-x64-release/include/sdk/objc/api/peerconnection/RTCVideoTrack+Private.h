/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCVideoTrack.h"

#include "api/media_stream_interface.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCVideoTrack)
()

    /** VideoTrackInterface created or passed in at construction. */
    @property(nonatomic, readonly)
        webrtc::scoped_refptr<webrtc::VideoTrackInterface>
            nativeVideoTrack;

/** Initialize an RTCVideoTrack with its source and an id. */
- (instancetype)initWithFactory:
                    (RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory
                         source:(RTC_OBJC_TYPE(RTCVideoSource) *)source
                        trackId:(NSString *)trackId;

@end

NS_ASSUME_NONNULL_END
