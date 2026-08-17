/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC_AGC_H_
#define MODULES_AUDIO_PROCESSING_AGC_AGC_H_

#include <memory>

#include "api/array_view.h"
#include "modules/audio_processing/vad/voice_activity_detector.h"

namespace webrtc {

class LoudnessHistogram;

class Agc {
 public:
  Agc();
  virtual ~Agc();

  // `audio` must be mono; in a multi-channel stream, provide the first (usually
  // left) channel.
  virtual void Process(ArrayView<const int16_t> audio);

  // Retrieves the difference between the target RMS level and the current
  // signal RMS level in dB. Returns true if an update is available and false
  // otherwise, in which case `error` should be ignored and no action taken.
  virtual bool GetRmsErrorDb(int* error);
  virtual void Reset();

  virtual int set_target_level_dbfs(int level);
  virtual int target_level_dbfs() const;
  virtual float voice_probability() const;

 private:
  double target_level_loudness_;
  int target_level_dbfs_;
  std::unique_ptr<LoudnessHistogram> histogram_;
  std::unique_ptr<LoudnessHistogram> inactive_histogram_;
  VoiceActivityDetector vad_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC_AGC_H_
