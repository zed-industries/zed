/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCIceCandidate.h"

#include <memory>

#include "api/jsep.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCIceCandidate)
()

    /**
     * The native IceCandidateInterface representation of this RTCIceCandidate
     * object. This is needed to pass to the underlying C++ APIs.
     */
    @property(nonatomic, readonly)
        std::unique_ptr<webrtc::IceCandidateInterface> nativeCandidate;

/**
 * Initialize an RTCIceCandidate from a native IceCandidateInterface. No
 * ownership is taken of the native candidate.
 */
- (instancetype)initWithNativeCandidate:
    (const webrtc::IceCandidateInterface *)candidate;

@end

NS_ASSUME_NONNULL_END
