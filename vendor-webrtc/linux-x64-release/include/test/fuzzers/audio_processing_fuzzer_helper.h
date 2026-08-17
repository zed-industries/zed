/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_FUZZERS_AUDIO_PROCESSING_FUZZER_HELPER_H_
#define TEST_FUZZERS_AUDIO_PROCESSING_FUZZER_HELPER_H_

#include <memory>

#include "api/audio/audio_processing.h"
#include "test/fuzzers/fuzz_data_helper.h"
namespace webrtc {

void FuzzAudioProcessing(test::FuzzDataHelper* fuzz_data,
                         scoped_refptr<AudioProcessing> apm);

}  // namespace webrtc

#endif  // TEST_FUZZERS_AUDIO_PROCESSING_FUZZER_HELPER_H_
