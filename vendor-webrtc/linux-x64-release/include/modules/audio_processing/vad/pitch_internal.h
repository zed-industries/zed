/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_VAD_PITCH_INTERNAL_H_
#define MODULES_AUDIO_PROCESSING_VAD_PITCH_INTERNAL_H_

namespace webrtc {

// TODO(turajs): Write a description of this function. Also be consistent with
// usage of `sampling_rate_hz` vs `kSamplingFreqHz`.
void GetSubframesPitchParameters(int sampling_rate_hz,
                                 double* gains,
                                 double* lags,
                                 int num_in_frames,
                                 int num_out_frames,
                                 double* log_old_gain,
                                 double* old_lag,
                                 double* log_pitch_gain,
                                 double* pitch_lag_hz);

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_VAD_PITCH_INTERNAL_H_
