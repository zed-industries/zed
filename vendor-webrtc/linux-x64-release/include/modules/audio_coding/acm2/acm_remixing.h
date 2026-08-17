/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_ACM2_ACM_REMIXING_H_
#define MODULES_AUDIO_CODING_ACM2_ACM_REMIXING_H_

#include <vector>

#include "api/array_view.h"
#include "api/audio/audio_frame.h"

namespace webrtc {

// Stereo-to-mono downmixing. The length of the output must equal to the number
// of samples per channel in the input.
void DownMixFrame(const AudioFrame& input, ArrayView<int16_t> output);

// Remixes the interleaved input frame to an interleaved output data vector. The
// remixed data replaces the data in the output vector which is resized if
// needed. The remixing supports any combination of input and output channels,
// as well as any number of samples per channel.
void ReMixFrame(const AudioFrame& input,
                size_t num_output_channels,
                std::vector<int16_t>* output);

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_ACM2_ACM_REMIXING_H_
