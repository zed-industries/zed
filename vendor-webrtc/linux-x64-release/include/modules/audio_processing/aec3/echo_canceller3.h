/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_ECHO_CANCELLER3_H_
#define MODULES_AUDIO_PROCESSING_AEC3_ECHO_CANCELLER3_H_

#include <stddef.h>

#include <atomic>
#include <memory>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "api/audio/echo_canceller3_config.h"
#include "api/audio/echo_control.h"
#include "api/environment/environment.h"
#include "api/field_trials_view.h"
#include "modules/audio_processing/aec3/api_call_jitter_metrics.h"
#include "modules/audio_processing/aec3/block_delay_buffer.h"
#include "modules/audio_processing/aec3/block_framer.h"
#include "modules/audio_processing/aec3/block_processor.h"
#include "modules/audio_processing/aec3/config_selector.h"
#include "modules/audio_processing/aec3/frame_blocker.h"
#include "modules/audio_processing/aec3/multi_channel_content_detector.h"
#include "modules/audio_processing/audio_buffer.h"
#include "modules/audio_processing/logging/apm_data_dumper.h"
#include "rtc_base/checks.h"
#include "rtc_base/race_checker.h"
#include "rtc_base/swap_queue.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// Method for adjusting config parameter dependencies.
// Only to be used externally to AEC3 for testing purposes.
// TODO(webrtc:5298): Move this to a separate file.
EchoCanceller3Config AdjustConfig(const EchoCanceller3Config& config,
                                  const FieldTrialsView& field_trials);

// Functor for verifying the invariance of the frames being put into the render
// queue.
class Aec3RenderQueueItemVerifier {
 public:
  Aec3RenderQueueItemVerifier(size_t num_bands,
                              size_t num_channels,
                              size_t frame_length)
      : num_bands_(num_bands),
        num_channels_(num_channels),
        frame_length_(frame_length) {}

  bool operator()(const std::vector<std::vector<std::vector<float>>>& v) const {
    if (v.size() != num_bands_) {
      return false;
    }
    for (const auto& band : v) {
      if (band.size() != num_channels_) {
        return false;
      }
      for (const auto& channel : band) {
        if (channel.size() != frame_length_) {
          return false;
        }
      }
    }
    return true;
  }

 private:
  const size_t num_bands_;
  const size_t num_channels_;
  const size_t frame_length_;
};

// Main class for the echo canceller3.
// It does 4 things:
// -Receives 10 ms frames of band-split audio.
// -Provides the lower level echo canceller functionality with
// blocks of 64 samples of audio data.
// -Partially handles the jitter in the render and capture API
// call sequence.
//
// The class is supposed to be used in a non-concurrent manner apart from the
// AnalyzeRender call which can be called concurrently with the other methods.
class EchoCanceller3 : public EchoControl {
 public:
  EchoCanceller3(const Environment& env,
                 const EchoCanceller3Config& config,
                 const std::optional<EchoCanceller3Config>& multichannel_config,
                 int sample_rate_hz,
                 size_t num_render_channels,
                 size_t num_capture_channels);

  ~EchoCanceller3() override;

  EchoCanceller3(const EchoCanceller3&) = delete;
  EchoCanceller3& operator=(const EchoCanceller3&) = delete;

  // Analyzes and stores an internal copy of the split-band domain render
  // signal.
  void AnalyzeRender(AudioBuffer* render) override { AnalyzeRender(*render); }
  // Analyzes the full-band domain capture signal to detect signal saturation.
  void AnalyzeCapture(AudioBuffer* capture) override {
    AnalyzeCapture(*capture);
  }
  // Processes the split-band domain capture signal in order to remove any echo
  // present in the signal.
  void ProcessCapture(AudioBuffer* capture, bool level_change) override;
  // As above, but also returns the linear filter output.
  void ProcessCapture(AudioBuffer* capture,
                      AudioBuffer* linear_output,
                      bool level_change) override;
  // Collect current metrics from the echo canceller.
  Metrics GetMetrics() const override;
  // Provides an optional external estimate of the audio buffer delay.
  void SetAudioBufferDelay(int delay_ms) override;

  // Specifies whether the capture output will be used. The purpose of this is
  // to allow the echo controller to deactivate some of the processing when the
  // resulting output is anyway not used, for instance when the endpoint is
  // muted.
  void SetCaptureOutputUsage(bool capture_output_used) override;

  bool ActiveProcessing() const override;

  // Signals whether an external detector has detected echo leakage from the
  // echo canceller.
  // Note that in the case echo leakage has been flagged, it should be unflagged
  // once it is no longer occurring.
  void UpdateEchoLeakageStatus(bool leakage_detected) {
    RTC_DCHECK_RUNS_SERIALIZED(&capture_race_checker_);
    block_processor_->UpdateEchoLeakageStatus(leakage_detected);
  }

 private:
  friend class EchoCanceller3Tester;
  FRIEND_TEST_ALL_PREFIXES(EchoCanceller3, DetectionOfProperStereo);
  FRIEND_TEST_ALL_PREFIXES(EchoCanceller3,
                           DetectionOfProperStereoUsingThreshold);
  FRIEND_TEST_ALL_PREFIXES(EchoCanceller3,
                           DetectionOfProperStereoUsingHysteresis);
  FRIEND_TEST_ALL_PREFIXES(EchoCanceller3,
                           StereoContentDetectionForMonoSignals);

  class RenderWriter;

  // (Re-)Initializes the selected subset of the EchoCanceller3 fields, at
  // creation as well as during reconfiguration.
  void Initialize();

  // Only for testing. Replaces the internal block processor.
  void SetBlockProcessorForTesting(
      std::unique_ptr<BlockProcessor> block_processor);

  // Only for testing. Returns whether stereo processing is active.
  bool StereoRenderProcessingActiveForTesting() const {
    return multichannel_content_detector_.IsProperMultiChannelContentDetected();
  }

  // Only for testing.
  const EchoCanceller3Config& GetActiveConfigForTesting() const {
    return config_selector_.active_config();
  }

  // Empties the render SwapQueue.
  void EmptyRenderQueue();

  // Analyzes and stores an internal copy of the split-band domain render
  // signal.
  void AnalyzeRender(const AudioBuffer& render);
  // Analyzes the full-band domain capture signal to detect signal saturation.
  void AnalyzeCapture(const AudioBuffer& capture);

  const Environment env_;
  RaceChecker capture_race_checker_;
  RaceChecker render_race_checker_;

  // State that is accessed by the AnalyzeRender call.
  std::unique_ptr<RenderWriter> render_writer_
      RTC_GUARDED_BY(render_race_checker_);

  // State that may be accessed by the capture thread.
  static std::atomic<int> instance_count_;
  std::unique_ptr<ApmDataDumper> data_dumper_;
  const EchoCanceller3Config config_;
  const int sample_rate_hz_;
  const int num_bands_;
  const size_t num_render_input_channels_;
  size_t num_render_channels_to_aec_;
  const size_t num_capture_channels_;
  ConfigSelector config_selector_;
  MultiChannelContentDetector multichannel_content_detector_;
  std::unique_ptr<BlockFramer> linear_output_framer_
      RTC_GUARDED_BY(capture_race_checker_);
  BlockFramer output_framer_ RTC_GUARDED_BY(capture_race_checker_);
  FrameBlocker capture_blocker_ RTC_GUARDED_BY(capture_race_checker_);
  std::unique_ptr<FrameBlocker> render_blocker_
      RTC_GUARDED_BY(capture_race_checker_);
  SwapQueue<std::vector<std::vector<std::vector<float>>>,
            Aec3RenderQueueItemVerifier>
      render_transfer_queue_;
  std::unique_ptr<BlockProcessor> block_processor_
      RTC_GUARDED_BY(capture_race_checker_);
  std::vector<std::vector<std::vector<float>>> render_queue_output_frame_
      RTC_GUARDED_BY(capture_race_checker_);
  bool saturated_microphone_signal_ RTC_GUARDED_BY(capture_race_checker_) =
      false;
  Block render_block_ RTC_GUARDED_BY(capture_race_checker_);
  std::unique_ptr<Block> linear_output_block_
      RTC_GUARDED_BY(capture_race_checker_);
  Block capture_block_ RTC_GUARDED_BY(capture_race_checker_);
  std::vector<std::vector<ArrayView<float>>> render_sub_frame_view_
      RTC_GUARDED_BY(capture_race_checker_);
  std::vector<std::vector<ArrayView<float>>> linear_output_sub_frame_view_
      RTC_GUARDED_BY(capture_race_checker_);
  std::vector<std::vector<ArrayView<float>>> capture_sub_frame_view_
      RTC_GUARDED_BY(capture_race_checker_);
  std::unique_ptr<BlockDelayBuffer> block_delay_buffer_
      RTC_GUARDED_BY(capture_race_checker_);
  ApiCallJitterMetrics api_call_metrics_ RTC_GUARDED_BY(capture_race_checker_);
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_ECHO_CANCELLER3_H_
