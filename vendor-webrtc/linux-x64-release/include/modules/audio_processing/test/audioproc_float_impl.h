/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_AUDIOPROC_FLOAT_IMPL_H_
#define MODULES_AUDIO_PROCESSING_TEST_AUDIOPROC_FLOAT_IMPL_H_

#include <memory>

#include "absl/base/nullability.h"
#include "api/audio/audio_processing.h"
#include "api/audio/builtin_audio_processing_builder.h"

namespace webrtc {
namespace test {

int AudioprocFloatImpl(
    absl_nonnull std::unique_ptr<BuiltinAudioProcessingBuilder> ap_builder,
    int argc,
    char* argv[]);

int AudioprocFloatImpl(
    absl_nonnull std::unique_ptr<AudioProcessingBuilderInterface> ap_builder,
    int argc,
    char* argv[]);

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_AUDIOPROC_FLOAT_IMPL_H_
