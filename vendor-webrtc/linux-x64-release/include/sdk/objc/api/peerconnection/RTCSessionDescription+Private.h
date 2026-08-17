/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCSessionDescription.h"

#include "api/jsep.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCSessionDescription)
()

    /**
     * The native SessionDescriptionInterface representation of this
     * RTCSessionDescription object. This is needed to pass to the underlying
     * C++ APIs.
     */
    @property(nonatomic, readonly)
        std::unique_ptr<webrtc::SessionDescriptionInterface> nativeDescription;

/**
 * Initialize an RTCSessionDescription from a native
 * SessionDescriptionInterface. No ownership is taken of the native session
 * description.
 */
- (instancetype)initWithNativeDescription:
    (const webrtc::SessionDescriptionInterface *)nativeDescription;

+ (std::string)stdStringForType:(RTC_OBJC_TYPE(RTCSdpType))type;

+ (RTC_OBJC_TYPE(RTCSdpType))typeForStdString:(const std::string &)string;

@end

NS_ASSUME_NONNULL_END
