/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_AUDIO_PROCESSING_SIMULATOR_H_
#define MODULES_AUDIO_PROCESSING_TEST_AUDIO_PROCESSING_SIMULATOR_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <fstream>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/strings/string_view.h"
#include "api/audio/audio_processing.h"
#include "api/scoped_refptr.h"
#include "common_audio/channel_buffer.h"
#include "common_audio/include/audio_util.h"
#include "common_audio/wav_file.h"
#include "modules/audio_processing/test/api_call_statistics.h"
#include "modules/audio_processing/test/fake_recording_device.h"
#include "modules/audio_processing/test/test_utils.h"
#include "rtc_base/checks.h"
#include "rtc_base/task_queue_for_test.h"

namespace webrtc {
namespace test {

static const int kChunksPerSecond = 1000 / AudioProcessing::kChunkSizeMs;

struct Int16Frame {
  void SetFormat(int sample_rate_hz, int num_channels) {
    sample_rate_hz_ = sample_rate_hz;
    samples_per_channel_ = CheckedDivExact(sample_rate_hz, kChunksPerSecond);
    num_channels_ = num_channels;
    config = StreamConfig(sample_rate_hz, num_channels);
    data.resize(num_channels * samples_per_channel_);
  }

  void CopyTo(ChannelBuffer<float>* dest) {
    RTC_DCHECK(dest);
    RTC_CHECK_EQ(num_channels_, dest->num_channels());
    RTC_CHECK_EQ(samples_per_channel_, dest->num_frames());
    // Copy the data from the input buffer.
    std::vector<float> tmp(samples_per_channel_ * num_channels_);
    S16ToFloat(data.data(), tmp.size(), tmp.data());
    Deinterleave(tmp.data(), samples_per_channel_, num_channels_,
                 dest->channels());
  }

  void CopyFrom(const ChannelBuffer<float>& src) {
    RTC_CHECK_EQ(src.num_channels(), num_channels_);
    RTC_CHECK_EQ(src.num_frames(), samples_per_channel_);
    data.resize(num_channels_ * samples_per_channel_);
    int16_t* dest_data = data.data();
    for (int ch = 0; ch < num_channels_; ++ch) {
      for (int sample = 0; sample < samples_per_channel_; ++sample) {
        dest_data[sample * num_channels_ + ch] =
            src.channels()[ch][sample] * 32767;
      }
    }
  }

  int sample_rate_hz_;
  int samples_per_channel_;
  int num_channels_;

  StreamConfig config;

  std::vector<int16_t> data;
};

// Holds all the parameters available for controlling the simulation.
struct SimulationSettings {
  SimulationSettings();
  SimulationSettings(const SimulationSettings&);
  ~SimulationSettings();
  std::optional<int> stream_delay;
  std::optional<bool> use_stream_delay;
  std::optional<int> output_sample_rate_hz;
  std::optional<int> output_num_channels;
  std::optional<int> reverse_output_sample_rate_hz;
  std::optional<int> reverse_output_num_channels;
  std::optional<std::string> output_filename;
  std::optional<std::string> reverse_output_filename;
  std::optional<std::string> input_filename;
  std::optional<std::string> reverse_input_filename;
  std::optional<std::string> artificial_nearend_filename;
  std::optional<std::string> linear_aec_output_filename;
  std::optional<bool> use_aec;
  std::optional<bool> use_aecm;
  std::optional<bool> use_ed;  // Residual Echo Detector.
  std::optional<std::string> ed_graph_output_filename;
  std::optional<bool> use_agc;
  std::optional<bool> use_agc2;
  std::optional<bool> use_pre_amplifier;
  std::optional<bool> use_capture_level_adjustment;
  std::optional<bool> use_analog_mic_gain_emulation;
  std::optional<bool> use_hpf;
  std::optional<bool> use_ns;
  std::optional<int> use_ts;
  std::optional<bool> use_analog_agc;
  std::optional<bool> use_all;
  std::optional<bool> analog_agc_use_digital_adaptive_controller;
  std::optional<int> agc_mode;
  std::optional<int> agc_target_level;
  std::optional<bool> use_agc_limiter;
  std::optional<int> agc_compression_gain;
  std::optional<bool> agc2_use_adaptive_gain;
  std::optional<float> agc2_fixed_gain_db;
  std::optional<bool> agc2_use_input_volume_controller;
  std::optional<float> pre_amplifier_gain_factor;
  std::optional<float> pre_gain_factor;
  std::optional<float> post_gain_factor;
  std::optional<float> analog_mic_gain_emulation_initial_level;
  std::optional<int> ns_level;
  std::optional<bool> ns_analysis_on_linear_aec_output;
  std::optional<bool> override_key_pressed;
  std::optional<int> maximum_internal_processing_rate;
  int initial_mic_level;
  bool simulate_mic_gain = false;
  std::optional<bool> multi_channel_render;
  std::optional<bool> multi_channel_capture;
  std::optional<int> simulated_mic_kind;
  std::optional<int> frame_for_sending_capture_output_used_false;
  std::optional<int> frame_for_sending_capture_output_used_true;
  bool report_performance = false;
  std::optional<std::string> performance_report_output_filename;
  bool report_bitexactness = false;
  bool use_verbose_logging = false;
  bool use_quiet_output = false;
  bool discard_all_settings_in_aecdump = true;
  std::optional<std::string> aec_dump_input_filename;
  std::optional<std::string> aec_dump_output_filename;
  bool fixed_interface = false;
  bool store_intermediate_output = false;
  bool print_aec_parameter_values = false;
  bool dump_internal_data = false;
  WavFile::SampleFormat wav_output_format = WavFile::SampleFormat::kInt16;
  std::optional<std::string> dump_internal_data_output_dir;
  std::optional<int> dump_set_to_use;
  std::optional<std::string> call_order_input_filename;
  std::optional<std::string> call_order_output_filename;
  std::optional<std::string> aec_settings_filename;
  std::optional<absl::string_view> aec_dump_input_string;
  std::vector<float>* processed_capture_samples = nullptr;
  bool analysis_only = false;
  std::optional<int> dump_start_frame;
  std::optional<int> dump_end_frame;
  std::optional<int> init_to_process;
};

// Provides common functionality for performing audioprocessing simulations.
class AudioProcessingSimulator {
 public:
  AudioProcessingSimulator(
      const SimulationSettings& settings,
      absl_nonnull scoped_refptr<AudioProcessing> audio_processing);

  AudioProcessingSimulator() = delete;
  AudioProcessingSimulator(const AudioProcessingSimulator&) = delete;
  AudioProcessingSimulator& operator=(const AudioProcessingSimulator&) = delete;

  virtual ~AudioProcessingSimulator();

  // Processes the data in the input.
  virtual void Process() = 0;

  // Returns the execution times of all AudioProcessing calls.
  const ApiCallStatistics& GetApiCallStatistics() const {
    return api_call_statistics_;
  }

  // Analyzes the data in the input and reports the resulting statistics.
  virtual void Analyze() = 0;

  // Reports whether the processed recording was bitexact.
  bool OutputWasBitexact() { return bitexact_output_; }

  size_t get_num_process_stream_calls() { return num_process_stream_calls_; }
  size_t get_num_reverse_process_stream_calls() {
    return num_reverse_process_stream_calls_;
  }

 protected:
  void ProcessStream(bool fixed_interface);
  void ProcessReverseStream(bool fixed_interface);
  void ConfigureAudioProcessor();
  void DetachAecDump();
  void SetupBuffersConfigsOutputs(int input_sample_rate_hz,
                                  int output_sample_rate_hz,
                                  int reverse_input_sample_rate_hz,
                                  int reverse_output_sample_rate_hz,
                                  int input_num_channels,
                                  int output_num_channels,
                                  int reverse_input_num_channels,
                                  int reverse_output_num_channels);
  void SelectivelyToggleDataDumping(int init_index,
                                    int capture_frames_since_init) const;

  const SimulationSettings settings_;
  scoped_refptr<AudioProcessing> ap_;

  std::unique_ptr<ChannelBuffer<float>> in_buf_;
  std::unique_ptr<ChannelBuffer<float>> out_buf_;
  std::unique_ptr<ChannelBuffer<float>> reverse_in_buf_;
  std::unique_ptr<ChannelBuffer<float>> reverse_out_buf_;
  std::vector<std::array<float, 160>> linear_aec_output_buf_;
  StreamConfig in_config_;
  StreamConfig out_config_;
  StreamConfig reverse_in_config_;
  StreamConfig reverse_out_config_;
  std::unique_ptr<ChannelBufferWavReader> buffer_reader_;
  std::unique_ptr<ChannelBufferWavReader> reverse_buffer_reader_;
  Int16Frame rev_frame_;
  Int16Frame fwd_frame_;
  bool bitexact_output_ = true;
  std::optional<int> aec_dump_applied_input_level_ = 0;

 protected:
  size_t output_reset_counter_ = 0;

 private:
  void SetupOutput();

  size_t num_process_stream_calls_ = 0;
  size_t num_reverse_process_stream_calls_ = 0;
  std::unique_ptr<ChannelBufferWavWriter> buffer_file_writer_;
  std::unique_ptr<ChannelBufferWavWriter> reverse_buffer_file_writer_;
  std::unique_ptr<ChannelBufferVectorWriter> buffer_memory_writer_;
  std::unique_ptr<WavWriter> linear_aec_output_file_writer_;
  ApiCallStatistics api_call_statistics_;
  std::ofstream residual_echo_likelihood_graph_writer_;
  int applied_input_volume_;
  FakeRecordingDevice fake_recording_device_;

  TaskQueueForTest worker_queue_;
};

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_AUDIO_PROCESSING_SIMULATOR_H_
