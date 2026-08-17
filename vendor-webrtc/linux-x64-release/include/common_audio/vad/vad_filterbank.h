/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

/*
 * This file includes feature calculating functionality used in vad_core.c.
 */

#ifndef COMMON_AUDIO_VAD_VAD_FILTERBANK_H_
#define COMMON_AUDIO_VAD_VAD_FILTERBANK_H_

#include "common_audio/vad/vad_core.h"

// Takes `data_length` samples of `data_in` and calculates the logarithm of the
// energy of each of the `kNumChannels` = 6 frequency bands used by the VAD:
//        80 Hz - 250 Hz
//        250 Hz - 500 Hz
//        500 Hz - 1000 Hz
//        1000 Hz - 2000 Hz
//        2000 Hz - 3000 Hz
//        3000 Hz - 4000 Hz
//
// The values are given in Q4 and written to `features`. Further, an approximate
// overall energy is returned. The return value is used in
// WebRtcVad_GmmProbability() as a signal indicator, hence it is arbitrary above
// the threshold `kMinEnergy`.
//
// - self         [i/o] : State information of the VAD.
// - data_in      [i]   : Input audio data, for feature extraction.
// - data_length  [i]   : Audio data size, in number of samples.
// - features     [o]   : 10 * log10(energy in each frequency band), Q4.
// - returns            : Total energy of the signal (NOTE! This value is not
//                        exact. It is only used in a comparison.)
int16_t WebRtcVad_CalculateFeatures(VadInstT* self,
                                    const int16_t* data_in,
                                    size_t data_length,
                                    int16_t* features);

#endif  // COMMON_AUDIO_VAD_VAD_FILTERBANK_H_
