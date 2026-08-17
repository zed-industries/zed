/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC_STANDALONE_VAD_H_
#define MODULES_AUDIO_PROCESSING_AGC_STANDALONE_VAD_H_

#include <stddef.h>
#include <stdint.h>

#include "common_audio/vad/include/webrtc_vad.h"
#include "modules/audio_processing/vad/common.h"

namespace webrtc {

class StandaloneVad {
 public:
  static StandaloneVad* Create();
  ~StandaloneVad();

  // Outputs
  //   p: a buffer where probabilities are written to.
  //   length_p: number of elements of `p`.
  //
  // return value:
  //    -1: if no audio is stored or VAD returns error.
  //     0: in success.
  // In case of error the content of `activity` is unchanged.
  //
  // Note that due to a high false-positive (VAD decision is active while the
  // processed audio is just background noise) rate, stand-alone VAD is used as
  // a one-sided indicator. The activity probability is 0.5 if the frame is
  // classified as active, and the probability is 0.01 if the audio is
  // classified as passive. In this way, when probabilities are combined, the
  // effect of the stand-alone VAD is neutral if the input is classified as
  // active.
  int GetActivity(double* p, size_t length_p);

  // Expecting 10 ms of 16 kHz audio to be pushed in.
  int AddAudio(const int16_t* data, size_t length);

  // Set aggressiveness of VAD, 0 is the least aggressive and 3 is the most
  // aggressive mode. Returns -1 if the input is less than 0 or larger than 3,
  // otherwise 0 is returned.
  int set_mode(int mode);
  // Get the agressiveness of the current VAD.
  int mode() const { return mode_; }

 private:
  explicit StandaloneVad(VadInst* vad);

  static const size_t kMaxNum10msFrames = 3;

  // TODO(turajs): Is there a way to use scoped-pointer here?
  VadInst* vad_;
  int16_t buffer_[kMaxNum10msFrames * kLength10Ms];
  size_t index_;
  int mode_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC_STANDALONE_VAD_H_
