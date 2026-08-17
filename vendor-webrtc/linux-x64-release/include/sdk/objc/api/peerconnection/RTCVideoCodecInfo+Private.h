/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "base/RTCVideoCodecInfo.h"

#include "api/video_codecs/sdp_video_format.h"

NS_ASSUME_NONNULL_BEGIN

/* Interface for converting to/from internal C++ formats. */
@interface RTC_OBJC_TYPE (RTCVideoCodecInfo)
(Private)

    - (instancetype)initWithNativeSdpVideoFormat
    : (webrtc::SdpVideoFormat)format;
- (webrtc::SdpVideoFormat)nativeSdpVideoFormat;

@end

NS_ASSUME_NONNULL_END
