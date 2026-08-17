/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_FUZZERS_AUDIO_ENCODER_FUZZER_H_
#define TEST_FUZZERS_AUDIO_ENCODER_FUZZER_H_

#include <memory>

#include "api/array_view.h"
#include "api/audio_codecs/audio_encoder.h"

namespace webrtc {

void FuzzAudioEncoder(ArrayView<const uint8_t> data_view,
                      std::unique_ptr<AudioEncoder> encoder);

}  // namespace webrtc

#endif  // TEST_FUZZERS_AUDIO_ENCODER_FUZZER_H_
