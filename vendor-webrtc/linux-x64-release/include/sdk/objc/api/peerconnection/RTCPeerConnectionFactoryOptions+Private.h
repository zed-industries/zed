/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCPeerConnectionFactoryOptions.h"

#include "api/peer_connection_interface.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCPeerConnectionFactoryOptions)
()

    /** Returns the equivalent native PeerConnectionFactoryInterface::Options
     * structure. */
    @property(nonatomic, readonly)
        webrtc::PeerConnectionFactoryInterface::Options nativeOptions;

@end

NS_ASSUME_NONNULL_END
