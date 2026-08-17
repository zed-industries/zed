/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCNetworkMonitor.h"
#import "RTCMacros.h"

#include "sdk/objc/native/src/network_monitor_observer.h"

@interface RTC_OBJC_TYPE (RTCNetworkMonitor)
()

    /** `observer` is a raw pointer and should be kept alive
     *  for this object's lifetime.
     */
    - (instancetype)initWithObserver
    : (webrtc::NetworkMonitorObserver *)observer NS_DESIGNATED_INITIALIZER;

/** Stops the receiver from posting updates to `observer`. */
- (void)stop;

@end
