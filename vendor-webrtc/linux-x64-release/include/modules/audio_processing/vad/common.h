/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_VAD_COMMON_H_
#define MODULES_AUDIO_PROCESSING_VAD_COMMON_H_

#include <stddef.h>

static const int kSampleRateHz = 16000;
static const size_t kLength10Ms = kSampleRateHz / 100;
static const size_t kMaxNumFrames = 4;

struct AudioFeatures {
  double log_pitch_gain[kMaxNumFrames];
  double pitch_lag_hz[kMaxNumFrames];
  double spectral_peak[kMaxNumFrames];
  double rms[kMaxNumFrames];
  size_t num_frames;
  bool silence;
};

#endif  // MODULES_AUDIO_PROCESSING_VAD_COMMON_H_
