/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <memory>

#include "api/audio/audio_mixer.h"
#include "api/scoped_refptr.h"
#include "modules/audio_mixer/audio_mixer_impl.h"
#include "modules/audio_processing/audio_buffer.h"
#include "rtc_base/synchronization/mutex.h"
#include "rust/cxx.h"

namespace livekit_ffi {
class AudioMixer;
class NativeAudioFrame;
}  // namespace livekit_ffi

#include "webrtc-sys/src/audio_mixer.rs.h"

namespace livekit_ffi {

class NativeAudioFrame {
 public:
  NativeAudioFrame(webrtc::AudioFrame* frame) : frame_(frame) {}
  void update_frame(uint32_t timestamp,
                    const int16_t* data,
                    size_t samples_per_channel,
                    int sample_rate_hz,
                    size_t num_channels);

 private:
  webrtc::AudioFrame* frame_;
};

class AudioMixerSource : public webrtc::AudioMixer::Source {
 public:
  AudioMixerSource(rust::Box<AudioMixerSourceWrapper> source);

  AudioFrameInfo GetAudioFrameWithInfo(
      int sample_rate_hz,
      webrtc::AudioFrame* audio_frame) override;

  int Ssrc() const override;

  int PreferredSampleRate() const override;

  ~AudioMixerSource() {}

 private:
  rust::Box<AudioMixerSourceWrapper> source_;
};

class AudioMixer {
 public:
  AudioMixer();

  void add_source(rust::Box<AudioMixerSourceWrapper> source);

  void remove_source(int ssrc);

  size_t mix(size_t num_channels);
  const int16_t* data() const;

 private:
  mutable webrtc::Mutex sources_mutex_;
  webrtc::AudioFrame frame_;
  std::vector<std::shared_ptr<AudioMixerSource>> sources_;
  rtc::scoped_refptr<webrtc::AudioMixer> audio_mixer_;
};

std::unique_ptr<AudioMixer> create_audio_mixer();

}  // namespace livekit_ffi
