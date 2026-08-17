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

#include "livekit/apm.h"

#include "api/audio/builtin_audio_processing_builder.h"
#include "api/environment/environment_factory.h"

#include <memory>

namespace livekit_ffi {

namespace {

webrtc::AudioProcessing::Config ToWebrtcConfig(
    const AudioProcessingConfig& c) {
  webrtc::AudioProcessing::Config out;
  out.echo_canceller.enabled = c.echo_canceller_enabled;
  out.high_pass_filter.enabled = c.high_pass_filter_enabled;
  out.noise_suppression.enabled = c.noise_suppression_enabled;

  auto& g = out.gain_controller2;
  g.enabled = c.gain_controller2.enabled;
  g.input_volume_controller.enabled =
      c.gain_controller2.input_volume_controller_enabled;
  g.fixed_digital.gain_db = c.gain_controller2.fixed_digital_gain_db;

  auto& ad = g.adaptive_digital;
  ad.enabled = c.gain_controller2.adaptive_digital.enabled;
  ad.headroom_db = c.gain_controller2.adaptive_digital.headroom_db;
  ad.max_gain_db = c.gain_controller2.adaptive_digital.max_gain_db;
  ad.initial_gain_db = c.gain_controller2.adaptive_digital.initial_gain_db;
  ad.max_gain_change_db_per_second =
      c.gain_controller2.adaptive_digital.max_gain_change_db_per_second;
  ad.max_output_noise_level_dbfs =
      c.gain_controller2.adaptive_digital.max_output_noise_level_dbfs;

  return out;
}

}  // namespace

AudioProcessingModule::AudioProcessingModule(
    const AudioProcessingConfig& config) {
  apm_ = webrtc::BuiltinAudioProcessingBuilder()
             .Build(webrtc::CreateEnvironment());

  apm_->ApplyConfig(ToWebrtcConfig(config));
  apm_->Initialize();
}

int AudioProcessingModule::process_stream(const int16_t* src,
                                          size_t src_len,
                                          int16_t* dst,
                                          size_t dst_len,
                                          int sample_rate,
                                          int num_channels) {
  webrtc::StreamConfig stream_cfg(sample_rate, num_channels);
  return apm_->ProcessStream(src, stream_cfg, stream_cfg, dst);
}

int AudioProcessingModule::process_reverse_stream(const int16_t* src,
                                                  size_t src_len,
                                                  int16_t* dst,
                                                  size_t dst_len,
                                                  int sample_rate,
                                                  int num_channels) {
  webrtc::StreamConfig stream_cfg(sample_rate, num_channels);
  return apm_->ProcessReverseStream(src, stream_cfg, stream_cfg, dst);
}

int AudioProcessingModule::set_stream_delay_ms(int delay_ms) {
  return apm_->set_stream_delay_ms(delay_ms);
}

std::unique_ptr<AudioProcessingModule> create_apm(
    const AudioProcessingConfig& config) {
  return std::make_unique<AudioProcessingModule>(config);
}

}  // namespace livekit_ffi
