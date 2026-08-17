/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_AUDIO_LEVEL_H_
#define AUDIO_AUDIO_LEVEL_H_

#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class AudioFrame;
namespace voe {

// This class is thread-safe. However, TotalEnergy() and TotalDuration() are
// related, so if you call ComputeLevel() on a different thread than you read
// these values, you still need to use lock to read them as a pair.
class AudioLevel {
 public:
  AudioLevel();
  ~AudioLevel();
  void Reset();

  // Returns the current audio level linearly [0,32767], which gets updated
  // every "kUpdateFrequency+1" call to ComputeLevel() based on the maximum
  // audio level of any audio frame, decaying by a factor of 1/4 each time
  // LevelFullRange() gets updated.
  // Called on "API thread(s)" from APIs like VoEBase::CreateChannel(),
  // VoEBase::StopSend().
  int16_t LevelFullRange() const;
  void ResetLevelFullRange();
  // See the description for "totalAudioEnergy" in the WebRTC stats spec
  // (https://w3c.github.io/webrtc-stats/#dom-rtcaudiohandlerstats-totalaudioenergy)
  // In our implementation, the total audio energy increases by the
  // energy-equivalent of LevelFullRange() at the time of ComputeLevel(), rather
  // than the energy of the samples in that specific audio frame. As a result,
  // we may report a higher audio energy and audio level than the spec mandates.
  // TODO(https://crbug.com/webrtc/10784): We should either do what the spec
  // says or update the spec to match our implementation. If we want to have a
  // decaying audio level we should probably update both the spec and the
  // implementation to reduce the complexity of the definition. If we want to
  // continue to have decaying audio we should have unittests covering the
  // behavior of the decay.
  double TotalEnergy() const;
  double TotalDuration() const;

  // Called on a native capture audio thread (platform dependent) from the
  // AudioTransport::RecordedDataIsAvailable() callback.
  // In Chrome, this method is called on the AudioInputDevice thread.
  void ComputeLevel(const AudioFrame& audioFrame, double duration);

 private:
  enum { kUpdateFrequency = 10 };

  mutable Mutex mutex_;

  int16_t abs_max_ RTC_GUARDED_BY(mutex_);
  int16_t count_ RTC_GUARDED_BY(mutex_);
  int16_t current_level_full_range_ RTC_GUARDED_BY(mutex_);

  double total_energy_ RTC_GUARDED_BY(mutex_) = 0.0;
  double total_duration_ RTC_GUARDED_BY(mutex_) = 0.0;
};

}  // namespace voe
}  // namespace webrtc

#endif  // AUDIO_AUDIO_LEVEL_H_
