/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_FRAME_DUMPING_ENCODER_H_
#define VIDEO_FRAME_DUMPING_ENCODER_H_

#include <memory>

#include "api/field_trials_view.h"
#include "api/video_codecs/video_encoder.h"

namespace webrtc {

// Creates an encoder that wraps another passed encoder and dumps its encoded
// frames out into a unique IVF file into the directory specified by the
// "WebRTC-EncoderDataDumpDirectory" field trial. Each file generated is
// suffixed by the simulcast index of the encoded frames. If the passed encoder
// is nullptr, or the field trial is not setup, the function just returns the
// passed encoder. The directory specified by the field trial parameter should
// be delimited by ';'.
std::unique_ptr<VideoEncoder> MaybeCreateFrameDumpingEncoderWrapper(
    std::unique_ptr<VideoEncoder> encoder,
    const FieldTrialsView& field_trials);

}  // namespace webrtc

#endif  // VIDEO_FRAME_DUMPING_ENCODER_H_
