/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_FRAME_DUMPING_DECODER_H_
#define VIDEO_FRAME_DUMPING_DECODER_H_

#include <memory>

#include "api/video_codecs/video_decoder.h"
#include "rtc_base/system/file_wrapper.h"

namespace webrtc {

// Creates a decoder wrapper that writes the encoded frames to an IVF file.
std::unique_ptr<VideoDecoder> CreateFrameDumpingDecoderWrapper(
    std::unique_ptr<VideoDecoder> decoder,
    FileWrapper file);

}  // namespace webrtc

#endif  // VIDEO_FRAME_DUMPING_DECODER_H_
