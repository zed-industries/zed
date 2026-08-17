/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Header file including the delay estimator handle used for testing.

#ifndef MODULES_AUDIO_PROCESSING_UTILITY_DELAY_ESTIMATOR_INTERNAL_H_
#define MODULES_AUDIO_PROCESSING_UTILITY_DELAY_ESTIMATOR_INTERNAL_H_

#include "modules/audio_processing/utility/delay_estimator.h"

namespace webrtc {

typedef union {
  float float_;
  int32_t int32_;
} SpectrumType;

typedef struct {
  // Pointers to mean values of spectrum.
  SpectrumType* mean_far_spectrum;
  // `mean_far_spectrum` initialization indicator.
  int far_spectrum_initialized;

  int spectrum_size;

  // Far-end part of binary spectrum based delay estimation.
  BinaryDelayEstimatorFarend* binary_farend;
} DelayEstimatorFarend;

typedef struct {
  // Pointers to mean values of spectrum.
  SpectrumType* mean_near_spectrum;
  // `mean_near_spectrum` initialization indicator.
  int near_spectrum_initialized;

  int spectrum_size;

  // Binary spectrum based delay estimator
  BinaryDelayEstimator* binary_handle;
} DelayEstimator;

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_UTILITY_DELAY_ESTIMATOR_INTERNAL_H_
