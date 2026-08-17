/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCPeerConnectionFactory.h"

#include "api/peer_connection_interface.h"
#include "api/scoped_refptr.h"
#include "rtc_base/thread.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCPeerConnectionFactory)
()

    /**
     * PeerConnectionFactoryInterface created and held by this
     * RTCPeerConnectionFactory object. This is needed to pass to the underlying
     * C++ APIs.
     */
    @property(nonatomic, readonly)
        webrtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface>
            nativeFactory;

@property(nonatomic, readonly) webrtc::Thread* signalingThread;
@property(nonatomic, readonly) webrtc::Thread* workerThread;
@property(nonatomic, readonly) webrtc::Thread* networkThread;

@end

NS_ASSUME_NONNULL_END
