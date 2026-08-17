/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_VAD_PITCH_BASED_VAD_H_
#define MODULES_AUDIO_PROCESSING_VAD_PITCH_BASED_VAD_H_

#include <memory>

#include "modules/audio_processing/vad/common.h"
#include "modules/audio_processing/vad/gmm.h"

namespace webrtc {

class VadCircularBuffer;

// Computes the probability of the input audio frame to be active given
// the corresponding pitch-gain and lag of the frame.
class PitchBasedVad {
 public:
  PitchBasedVad();
  ~PitchBasedVad();

  // Compute pitch-based voicing probability, given the features.
  //   features: a structure containing features required for computing voicing
  //             probabilities.
  //
  //   p_combined: an array which contains the combined activity probabilities
  //               computed prior to the call of this function. The method,
  //               then, computes the voicing probabilities and combine them
  //               with the given values. The result are returned in `p`.
  int VoicingProbability(const AudioFeatures& features, double* p_combined);

 private:
  int UpdatePrior(double p);

  // TODO(turajs): maybe defining this at a higher level (maybe enum) so that
  // all the code recognize it as "no-error."
  static const int kNoError = 0;

  GmmParameters noise_gmm_;
  GmmParameters voice_gmm_;

  double p_prior_;

  std::unique_ptr<VadCircularBuffer> circular_buffer_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_VAD_PITCH_BASED_VAD_H_
