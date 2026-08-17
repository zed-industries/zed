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

#include "livekit/audio_mixer.h"

#include <iostream>
#include <memory>

#include "api/audio/audio_frame.h"
#include "api/audio/audio_mixer.h"
#include "modules/audio_mixer/audio_mixer_impl.h"
#include "webrtc-sys/src/audio_mixer.rs.h"

namespace livekit_ffi {

AudioMixer::AudioMixer() {
  audio_mixer_ = webrtc::AudioMixerImpl::Create();
}

void AudioMixer::add_source(rust::Box<AudioMixerSourceWrapper> source) {
  auto native_source = std::make_shared<AudioMixerSource>(std::move(source));

  webrtc::MutexLock lock(&sources_mutex_);
  audio_mixer_->AddSource(native_source.get());
  sources_.push_back(native_source);
}

void AudioMixer::remove_source(int source_ssrc) {
  webrtc::MutexLock lock(&sources_mutex_);
  auto it = std::find_if(
      sources_.begin(), sources_.end(),
      [source_ssrc](const auto& s) { return s->Ssrc() == source_ssrc; });

  if (it != sources_.end()) {
    audio_mixer_->RemoveSource(it->get());
    sources_.erase(it);
  }
}

size_t AudioMixer::mix(size_t number_of_channels) {
  audio_mixer_->Mix(number_of_channels, &frame_);
  return frame_.num_channels() * frame_.samples_per_channel();
}

const int16_t* AudioMixer::data() const {
  return frame_.data();
}

std::unique_ptr<AudioMixer> create_audio_mixer() {
  return std::make_unique<AudioMixer>();
}

AudioMixerSource::AudioMixerSource(rust::Box<AudioMixerSourceWrapper> source)
    : source_(std::move(source)) {}

int AudioMixerSource::Ssrc() const {
  return source_->ssrc();
}

int AudioMixerSource::PreferredSampleRate() const {
  return source_->preferred_sample_rate();
}

webrtc::AudioMixer::Source::AudioFrameInfo
AudioMixerSource::GetAudioFrameWithInfo(int sample_rate,
                                        webrtc::AudioFrame* audio_frame) {
  NativeAudioFrame frame(audio_frame);

  livekit_ffi::AudioFrameInfo result =
      source_->get_audio_frame_with_info(sample_rate, frame);

  if (result == livekit_ffi::AudioFrameInfo::Normal) {
    return webrtc::AudioMixer::Source::AudioFrameInfo::kNormal;
  } else if (result == livekit_ffi::AudioFrameInfo::Muted) {
    return webrtc::AudioMixer::Source::AudioFrameInfo::kMuted;
  } else {
    return webrtc::AudioMixer::Source::AudioFrameInfo::kError;
  }
}

void NativeAudioFrame::update_frame(uint32_t timestamp,
                                    const int16_t* data,
                                    size_t samples_per_channel,
                                    int sample_rate_hz,
                                    size_t num_channels) {
  frame_->UpdateFrame(timestamp, data, samples_per_channel, sample_rate_hz,
                      webrtc::AudioFrame::SpeechType::kNormalSpeech,
                      webrtc::AudioFrame::VADActivity::kVadUnknown,
                      num_channels);
}

}  // namespace livekit_ffi
