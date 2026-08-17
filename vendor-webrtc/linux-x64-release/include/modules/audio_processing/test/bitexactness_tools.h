
/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_BITEXACTNESS_TOOLS_H_
#define MODULES_AUDIO_PROCESSING_TEST_BITEXACTNESS_TOOLS_H_

#include <string>

#include "api/array_view.h"
#include "modules/audio_coding/neteq/tools/input_audio_file.h"
#include "test/gtest.h"

namespace webrtc {
namespace test {

// Returns test vector to use for the render signal in an
// APM bitexactness test.
std::string GetApmRenderTestVectorFileName(int sample_rate_hz);

// Returns test vector to use for the capture signal in an
// APM bitexactness test.
std::string GetApmCaptureTestVectorFileName(int sample_rate_hz);

// Extract float samples of up to two channels from a pcm file.
void ReadFloatSamplesFromStereoFile(size_t samples_per_channel,
                                    size_t num_channels,
                                    InputAudioFile* stereo_pcm_file,
                                    ArrayView<float> data);

// Verifies a frame against a reference and returns the results as an
// AssertionResult.
::testing::AssertionResult VerifyDeinterleavedArray(
    size_t samples_per_channel,
    size_t num_channels,
    ArrayView<const float> reference,
    ArrayView<const float> output,
    float element_error_bound);

// Verifies a vector against a reference and returns the results as an
// AssertionResult.
::testing::AssertionResult VerifyArray(ArrayView<const float> reference,
                                       ArrayView<const float> output,
                                       float element_error_bound);

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_BITEXACTNESS_TOOLS_H_
