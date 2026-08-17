/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_CODECS_H264_H264_COLOR_SPACE_H_
#define MODULES_VIDEO_CODING_CODECS_H264_H264_COLOR_SPACE_H_

// Everything declared in this header is only required when WebRTC is
// build with H264 support, please do not move anything out of the
// #ifdef unless needed and tested.
#ifdef WEBRTC_USE_H264

#if defined(WEBRTC_WIN) && !defined(__clang__)
#error "See: bugs.webrtc.org/9213#c13."
#endif

extern "C" {
#include <libavcodec/avcodec.h>
}  // extern "C"

#include "api/video/color_space.h"

namespace webrtc {

// Helper class for extracting color space information from H264 stream.
ColorSpace ExtractH264ColorSpace(AVCodecContext* codec);

}  // namespace webrtc

#endif  // WEBRTC_USE_H264

#endif  // MODULES_VIDEO_CODING_CODECS_H264_H264_COLOR_SPACE_H_
