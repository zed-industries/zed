/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCMediaStream.h"

#include "api/media_stream_interface.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCMediaStream)
()

    /**
     * MediaStreamInterface representation of this RTCMediaStream object. This
     * is needed to pass to the underlying C++ APIs.
     */
    @property(nonatomic, readonly)
        webrtc::scoped_refptr<webrtc::MediaStreamInterface>
            nativeMediaStream;

/** Initialize an RTCMediaStream with an id. */
- (instancetype)initWithFactory:
                    (RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory
                       streamId:(NSString *)streamId;

/** Initialize an RTCMediaStream from a native MediaStreamInterface. */
- (instancetype)
      initWithFactory:(RTC_OBJC_TYPE(RTCPeerConnectionFactory) *)factory
    nativeMediaStream:
        (webrtc::scoped_refptr<webrtc::MediaStreamInterface>)nativeMediaStream;

@end

NS_ASSUME_NONNULL_END
