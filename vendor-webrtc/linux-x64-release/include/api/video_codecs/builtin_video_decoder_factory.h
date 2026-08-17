/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_BUILTIN_VIDEO_DECODER_FACTORY_H_
#define API_VIDEO_CODECS_BUILTIN_VIDEO_DECODER_FACTORY_H_

#include <memory>

#include "api/video_codecs/video_decoder_factory.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Creates a new factory that can create the built-in types of video decoders.
RTC_EXPORT std::unique_ptr<VideoDecoderFactory>
CreateBuiltinVideoDecoderFactory();

}  // namespace webrtc

#endif  // API_VIDEO_CODECS_BUILTIN_VIDEO_DECODER_FACTORY_H_
