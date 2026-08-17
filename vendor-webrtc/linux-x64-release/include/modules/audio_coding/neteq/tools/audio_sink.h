/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_AUDIO_SINK_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_AUDIO_SINK_H_

#include "api/audio/audio_frame.h"

namespace webrtc {
namespace test {

// Interface class for an object receiving raw output audio from test
// applications.
class AudioSink {
 public:
  AudioSink() {}
  virtual ~AudioSink() {}

  AudioSink(const AudioSink&) = delete;
  AudioSink& operator=(const AudioSink&) = delete;

  // Writes `num_samples` from `audio` to the AudioSink. Returns true if
  // successful, otherwise false.
  virtual bool WriteArray(const int16_t* audio, size_t num_samples) = 0;

  // Writes `audio_frame` to the AudioSink. Returns true if successful,
  // otherwise false.
  bool WriteAudioFrame(const AudioFrame& audio_frame) {
    return WriteArray(audio_frame.data(), audio_frame.samples_per_channel_ *
                                              audio_frame.num_channels_);
  }
};

// Forks the output audio to two AudioSink objects.
class AudioSinkFork : public AudioSink {
 public:
  AudioSinkFork(AudioSink* left, AudioSink* right)
      : left_sink_(left), right_sink_(right) {}

  AudioSinkFork(const AudioSinkFork&) = delete;
  AudioSinkFork& operator=(const AudioSinkFork&) = delete;

  bool WriteArray(const int16_t* audio, size_t num_samples) override;

 private:
  AudioSink* left_sink_;
  AudioSink* right_sink_;
};

// An AudioSink implementation that does nothing.
class VoidAudioSink : public AudioSink {
 public:
  VoidAudioSink() = default;

  VoidAudioSink(const VoidAudioSink&) = delete;
  VoidAudioSink& operator=(const VoidAudioSink&) = delete;

  bool WriteArray(const int16_t* audio, size_t num_samples) override;
};

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_AUDIO_SINK_H_
