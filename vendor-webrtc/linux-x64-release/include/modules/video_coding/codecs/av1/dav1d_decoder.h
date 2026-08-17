/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_VIDEO_CODING_CODECS_AV1_DAV1D_DECODER_H_
#define MODULES_VIDEO_CODING_CODECS_AV1_DAV1D_DECODER_H_

#include <memory>

#include "api/environment/environment.h"
#include "api/video_codecs/video_decoder.h"

namespace webrtc {

// TODO: b/405341160 - Delete after downstream projects switched to version with
// `Environment`.
std::unique_ptr<VideoDecoder> CreateDav1dDecoder();

std::unique_ptr<VideoDecoder> CreateDav1dDecoder(const Environment& env);

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_AV1_DAV1D_DECODER_H_
