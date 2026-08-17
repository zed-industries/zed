/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_GAIN_CONTROL_IMPL_H_
#define MODULES_AUDIO_PROCESSING_GAIN_CONTROL_IMPL_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "modules/audio_processing/agc/gain_control.h"

namespace webrtc {

class ApmDataDumper;
class AudioBuffer;

class GainControlImpl : public GainControl {
 public:
  GainControlImpl();
  GainControlImpl(const GainControlImpl&) = delete;
  GainControlImpl& operator=(const GainControlImpl&) = delete;

  ~GainControlImpl() override;

  void ProcessRenderAudio(ArrayView<const int16_t> packed_render_audio);
  int AnalyzeCaptureAudio(const AudioBuffer& audio);
  int ProcessCaptureAudio(AudioBuffer* audio, bool stream_has_echo);

  void Initialize(size_t num_proc_channels, int sample_rate_hz);

  static void PackRenderAudioBuffer(const AudioBuffer& audio,
                                    std::vector<int16_t>* packed_buffer);

  // GainControl implementation.
  int stream_analog_level() const override;
  bool is_limiter_enabled() const override { return limiter_enabled_; }
  Mode mode() const override { return mode_; }
  int set_mode(Mode mode) override;
  int compression_gain_db() const override { return compression_gain_db_; }
  int set_analog_level_limits(int minimum, int maximum) override;
  int set_compression_gain_db(int gain) override;
  int set_target_level_dbfs(int level) override;
  int enable_limiter(bool enable) override;
  int set_stream_analog_level(int level) override;

 private:
  struct MonoAgcState;

  // GainControl implementation.
  int target_level_dbfs() const override { return target_level_dbfs_; }
  int analog_level_minimum() const override { return minimum_capture_level_; }
  int analog_level_maximum() const override { return maximum_capture_level_; }
  bool stream_is_saturated() const override { return stream_is_saturated_; }

  int Configure();

  std::unique_ptr<ApmDataDumper> data_dumper_;

  Mode mode_;
  int minimum_capture_level_;
  int maximum_capture_level_;
  bool limiter_enabled_;
  int target_level_dbfs_;
  int compression_gain_db_;
  int analog_capture_level_ = 0;
  bool was_analog_level_set_;
  bool stream_is_saturated_;

  std::vector<std::unique_ptr<MonoAgcState>> mono_agcs_;
  std::vector<int> capture_levels_;

  std::optional<size_t> num_proc_channels_;
  std::optional<int> sample_rate_hz_;

  static int instance_counter_;
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_GAIN_CONTROL_IMPL_H_
