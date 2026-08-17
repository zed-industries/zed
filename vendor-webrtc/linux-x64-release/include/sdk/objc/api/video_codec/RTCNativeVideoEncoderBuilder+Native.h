/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "sdk/objc/base/RTCMacros.h"

#include <memory>

#include "api/environment/environment.h"
#include "api/video_codecs/video_encoder.h"

@protocol RTC_OBJC_TYPE
(RTCNativeVideoEncoderBuilder)

    - (std::unique_ptr<webrtc::VideoEncoder>)build
    : (const webrtc::Environment&)env;

@end
