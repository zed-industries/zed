/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCMediaConstraints.h"

#include <memory>

#include "sdk/media_constraints.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCMediaConstraints)
()

    /**
     * A MediaConstraints representation of this RTCMediaConstraints object.
     * This is needed to pass to the underlying C++ APIs.
     */
    - (std::unique_ptr<webrtc::MediaConstraints>)nativeConstraints;

/** Return a native Constraints object representing these constraints */
+ (webrtc::MediaConstraints::Constraints)nativeConstraintsForConstraints:
    (NSDictionary<NSString*, NSString*>*)constraints;

@end

NS_ASSUME_NONNULL_END
