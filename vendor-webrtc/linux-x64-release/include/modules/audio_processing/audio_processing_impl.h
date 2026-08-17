/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AUDIO_PROCESSING_IMPL_H_
#define MODULES_AUDIO_PROCESSING_AUDIO_PROCESSING_IMPL_H_

#include <stdio.h>

#include <atomic>
#include <list>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/audio/audio_processing.h"
#include "api/audio/audio_processing_statistics.h"
#include "api/environment/environment.h"
#include "api/function_view.h"
#include "api/task_queue/task_queue_base.h"
#include "modules/audio_processing/aec3/echo_canceller3.h"
#include "modules/audio_processing/agc/agc_manager_direct.h"
#include "modules/audio_processing/agc/gain_control.h"
#include "modules/audio_processing/agc2/input_volume_stats_reporter.h"
#include "modules/audio_processing/audio_buffer.h"
#include "modules/audio_processing/capture_levels_adjuster/capture_levels_adjuster.h"
#include "modules/audio_processing/echo_control_mobile_impl.h"
#include "modules/audio_processing/gain_control_impl.h"
#include "modules/audio_processing/gain_controller2.h"
#include "modules/audio_processing/high_pass_filter.h"
#include "modules/audio_processing/include/aec_dump.h"
#include "modules/audio_processing/include/audio_frame_proxies.h"
#include "modules/audio_processing/ns/noise_suppressor.h"
#include "modules/audio_processing/render_queue_item_verifier.h"
#include "modules/audio_processing/rms_level.h"
#include "rtc_base/gtest_prod_util.h"
#include "rtc_base/swap_queue.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class ApmDataDumper;
class AudioConverter;

constexpr int RuntimeSettingQueueSize() {
  return 100;
}

class AudioProcessingImpl : public AudioProcessing {
 public:
  // Methods forcing APM to run in a single-threaded manner.
  // Acquires both the render and capture locks.
  explicit AudioProcessingImpl(const Environment& env);
  AudioProcessingImpl(const Environment& env,
                      const AudioProcessing::Config& config,
                      std::unique_ptr<CustomProcessing> capture_post_processor,
                      std::unique_ptr<CustomProcessing> render_pre_processor,
                      std::unique_ptr<EchoControlFactory> echo_control_factory,
                      scoped_refptr<EchoDetector> echo_detector,
                      std::unique_ptr<CustomAudioAnalyzer> capture_analyzer);
  ~AudioProcessingImpl() override;
  int Initialize() override;
  int Initialize(const ProcessingConfig& processing_config) override;
  void ApplyConfig(const AudioProcessing::Config& config) override;
  bool CreateAndAttachAecDump(absl::string_view file_name,
                              int64_t max_log_size_bytes,
                              TaskQueueBase* absl_nonnull
                                  worker_queue) override;
  bool CreateAndAttachAecDump(FILE* handle,
                              int64_t max_log_size_bytes,
                              TaskQueueBase* absl_nonnull
                                  worker_queue) override;
  // TODO(webrtc:5298) Deprecated variant.
  void AttachAecDump(std::unique_ptr<AecDump> aec_dump) override;
  void DetachAecDump() override;
  void SetRuntimeSetting(RuntimeSetting setting) override;
  bool PostRuntimeSetting(RuntimeSetting setting) override;

  // Capture-side exclusive methods possibly running APM in a
  // multi-threaded manner. Acquire the capture lock.
  int ProcessStream(const int16_t* const src,
                    const StreamConfig& input_config,
                    const StreamConfig& output_config,
                    int16_t* const dest) override;
  int ProcessStream(const float* const* src,
                    const StreamConfig& input_config,
                    const StreamConfig& output_config,
                    float* const* dest) override;
  bool GetLinearAecOutput(
      ArrayView<std::array<float, 160>> linear_output) const override;
  void set_output_will_be_muted(bool muted) override;
  bool get_output_will_be_muted() override;
  void HandleCaptureOutputUsedSetting(bool capture_output_used)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);
  int set_stream_delay_ms(int delay) override;
  void set_stream_key_pressed(bool key_pressed) override;
  void set_stream_analog_level(int level) override;
  int recommended_stream_analog_level() const
      RTC_LOCKS_EXCLUDED(mutex_capture_) override;

  // Render-side exclusive methods possibly running APM in a
  // multi-threaded manner. Acquire the render lock.
  int ProcessReverseStream(const int16_t* const src,
                           const StreamConfig& input_config,
                           const StreamConfig& output_config,
                           int16_t* const dest) override;
  int AnalyzeReverseStream(const float* const* data,
                           const StreamConfig& reverse_config) override;
  int ProcessReverseStream(const float* const* src,
                           const StreamConfig& input_config,
                           const StreamConfig& output_config,
                           float* const* dest) override;

  // Methods only accessed from APM submodules or
  // from AudioProcessing tests in a single-threaded manner.
  // Hence there is no need for locks in these.
  int proc_sample_rate_hz() const override;
  int proc_split_sample_rate_hz() const override;
  size_t num_input_channels() const override;
  size_t num_proc_channels() const override;
  size_t num_output_channels() const override;
  size_t num_reverse_channels() const override;
  int stream_delay_ms() const override;

  AudioProcessingStats GetStatistics(bool /* has_remote_tracks */) override {
    return GetStatistics();
  }
  AudioProcessingStats GetStatistics() override {
    return stats_reporter_.GetStatistics();
  }

  AudioProcessing::Config GetConfig() const override;

 protected:
  // Overridden in a mock.
  virtual void InitializeLocked()
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_render_, mutex_capture_);
  void AssertLockedForTest()
      RTC_ASSERT_EXCLUSIVE_LOCK(mutex_render_, mutex_capture_) {
    mutex_render_.AssertHeld();
    mutex_capture_.AssertHeld();
  }

 private:
  // TODO(peah): These friend classes should be removed as soon as the new
  // parameter setting scheme allows.
  FRIEND_TEST_ALL_PREFIXES(ApmConfiguration, DefaultBehavior);
  FRIEND_TEST_ALL_PREFIXES(ApmConfiguration, ValidConfigBehavior);
  FRIEND_TEST_ALL_PREFIXES(ApmConfiguration, InValidConfigBehavior);

  void set_stream_analog_level_locked(int level)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);
  void UpdateRecommendedInputVolumeLocked()
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);

  // Class providing thread-safe message pipe functionality for
  // `runtime_settings_`.
  class RuntimeSettingEnqueuer {
   public:
    explicit RuntimeSettingEnqueuer(
        SwapQueue<RuntimeSetting>* runtime_settings);
    ~RuntimeSettingEnqueuer();

    // Enqueue setting and return whether the setting was successfully enqueued.
    bool Enqueue(RuntimeSetting setting);

   private:
    SwapQueue<RuntimeSetting>& runtime_settings_;
  };

  const Environment env_;
  const std::unique_ptr<ApmDataDumper> data_dumper_;
  static std::atomic<int> instance_count_;
  const bool use_setup_specific_default_aec3_config_;

  SwapQueue<RuntimeSetting> capture_runtime_settings_;
  SwapQueue<RuntimeSetting> render_runtime_settings_;

  RuntimeSettingEnqueuer capture_runtime_settings_enqueuer_;
  RuntimeSettingEnqueuer render_runtime_settings_enqueuer_;

  // EchoControl factory.
  const std::unique_ptr<EchoControlFactory> echo_control_factory_;

  class SubmoduleStates {
   public:
    SubmoduleStates(bool capture_post_processor_enabled,
                    bool render_pre_processor_enabled,
                    bool capture_analyzer_enabled);
    // Updates the submodule state and returns true if it has changed.
    bool Update(bool high_pass_filter_enabled,
                bool mobile_echo_controller_enabled,
                bool noise_suppressor_enabled,
                bool adaptive_gain_controller_enabled,
                bool gain_controller2_enabled,
                bool gain_adjustment_enabled,
                bool echo_controller_enabled);
    bool CaptureMultiBandSubModulesActive() const;
    bool CaptureMultiBandProcessingPresent() const;
    bool CaptureMultiBandProcessingActive(bool ec_processing_active) const;
    bool CaptureFullBandProcessingActive() const;
    bool CaptureAnalyzerActive() const;
    bool RenderMultiBandSubModulesActive() const;
    bool RenderFullBandProcessingActive() const;
    bool RenderMultiBandProcessingActive() const;
    bool HighPassFilteringRequired() const;

   private:
    const bool capture_post_processor_enabled_ = false;
    const bool render_pre_processor_enabled_ = false;
    const bool capture_analyzer_enabled_ = false;
    bool high_pass_filter_enabled_ = false;
    bool mobile_echo_controller_enabled_ = false;
    bool noise_suppressor_enabled_ = false;
    bool adaptive_gain_controller_enabled_ = false;
    bool gain_controller2_enabled_ = false;
    bool gain_adjustment_enabled_ = false;
    bool echo_controller_enabled_ = false;
    bool first_update_ = true;
  };

  // Methods for modifying the formats struct that is used by both
  // the render and capture threads. The check for whether modifications are
  // needed is done while holding a single lock only, thereby avoiding that the
  // capture thread blocks the render thread.
  // Called by render: Holds the render lock when reading the format struct and
  // acquires both locks if reinitialization is required.
  void MaybeInitializeRender(const StreamConfig& input_config,
                             const StreamConfig& output_config)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_render_);
  // Called by capture: Acquires and releases the capture lock to read the
  // format struct and acquires both locks if reinitialization is needed.
  void MaybeInitializeCapture(const StreamConfig& input_config,
                              const StreamConfig& output_config);

  // Method for updating the state keeping track of the active submodules.
  // Returns a bool indicating whether the state has changed.
  bool UpdateActiveSubmoduleStates()
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);

  // Methods requiring APM running in a single-threaded manner, requiring both
  // the render and capture lock to be acquired.
  void InitializeLocked(const ProcessingConfig& config)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_render_, mutex_capture_);
  void InitializeResidualEchoDetector()
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_render_, mutex_capture_);
  void InitializeEchoController()
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_render_, mutex_capture_);

  // Initializations of capture-only sub-modules, requiring the capture lock
  // already acquired.
  void InitializeHighPassFilter(bool forced_reset)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);
  void InitializeGainController1() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);
  // Initializes the `GainController2` sub-module. If the sub-module is enabled,
  // recreates it.
  void InitializeGainController2() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);
  void InitializeNoiseSuppressor() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);
  void InitializeCaptureLevelsAdjuster()
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);
  void InitializePostProcessor() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);
  void InitializeAnalyzer() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);

  // Initializations of render-only submodules, requiring the render lock
  // already acquired.
  void InitializePreProcessor() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_render_);

  // Sample rate used for the fullband processing.
  int proc_fullband_sample_rate_hz() const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);

  // Empties and handles the respective RuntimeSetting queues.
  void HandleCaptureRuntimeSettings()
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);
  void HandleRenderRuntimeSettings()
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_render_);

  void EmptyQueuedRenderAudio() RTC_LOCKS_EXCLUDED(mutex_capture_);
  void EmptyQueuedRenderAudioLocked()
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);
  void AllocateRenderQueue()
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_render_, mutex_capture_);
  void QueueBandedRenderAudio(AudioBuffer* audio)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_render_);
  void QueueNonbandedRenderAudio(AudioBuffer* audio)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_render_);

  // Capture-side exclusive methods possibly running APM in a multi-threaded
  // manner that are called with the render lock already acquired.
  int ProcessCaptureStreamLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);

  // Render-side exclusive methods possibly running APM in a multi-threaded
  // manner that are called with the render lock already acquired.
  int AnalyzeReverseStreamLocked(const float* const* src,
                                 const StreamConfig& input_config,
                                 const StreamConfig& output_config)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_render_);
  int ProcessRenderStreamLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_render_);

  // Collects configuration settings from public and private
  // submodules to be saved as an audioproc::Config message on the
  // AecDump if it is attached.  If not `forced`, only writes the current
  // config if it is different from the last saved one; if `forced`,
  // writes the config regardless of the last saved.
  void WriteAecDumpConfigMessage(bool forced)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);

  // Notifies attached AecDump of current configuration and capture data.
  void RecordUnprocessedCaptureStream(const float* const* capture_stream)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);

  void RecordUnprocessedCaptureStream(const int16_t* const data,
                                      const StreamConfig& config)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);

  // Notifies attached AecDump of current configuration and
  // processed capture data and issues a capture stream recording
  // request.
  void RecordProcessedCaptureStream(
      const float* const* processed_capture_stream)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);

  void RecordProcessedCaptureStream(const int16_t* const data,
                                    const StreamConfig& config)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);

  // Notifies attached AecDump about current state (delay, drift, etc).
  void RecordAudioProcessingState()
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);

  // Ensures that overruns in the capture runtime settings queue is properly
  // handled by the code, providing safe-fallbacks to mitigate the implications
  // of any settings being missed.
  void HandleOverrunInCaptureRuntimeSettingsQueue()
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_capture_);

  // AecDump instance used for optionally logging APM config, input
  // and output to file in the AEC-dump format defined in debug.proto.
  std::unique_ptr<AecDump> aec_dump_;

  // Hold the last config written with AecDump for avoiding writing
  // the same config twice.
  InternalAPMConfig apm_config_for_aec_dump_ RTC_GUARDED_BY(mutex_capture_);

  // Critical sections.
  mutable Mutex mutex_render_ RTC_ACQUIRED_BEFORE(mutex_capture_);
  mutable Mutex mutex_capture_;

  // Struct containing the Config specifying the behavior of APM.
  AudioProcessing::Config config_;

  // Class containing information about what submodules are active.
  SubmoduleStates submodule_states_;

  // Struct containing the pointers to the submodules.
  struct Submodules {
    Submodules(std::unique_ptr<CustomProcessing> capture_post_processor,
               std::unique_ptr<CustomProcessing> render_pre_processor,
               scoped_refptr<EchoDetector> echo_detector,
               std::unique_ptr<CustomAudioAnalyzer> capture_analyzer)
        : echo_detector(std::move(echo_detector)),
          capture_post_processor(std::move(capture_post_processor)),
          render_pre_processor(std::move(render_pre_processor)),
          capture_analyzer(std::move(capture_analyzer)) {}
    // Accessed internally from capture or during initialization.
    const scoped_refptr<EchoDetector> echo_detector;
    const std::unique_ptr<CustomProcessing> capture_post_processor;
    const std::unique_ptr<CustomProcessing> render_pre_processor;
    const std::unique_ptr<CustomAudioAnalyzer> capture_analyzer;
    std::unique_ptr<AgcManagerDirect> agc_manager;
    std::unique_ptr<GainControlImpl> gain_control;
    std::unique_ptr<GainController2> gain_controller2;
    std::unique_ptr<HighPassFilter> high_pass_filter;
    std::unique_ptr<EchoControl> echo_controller;
    std::unique_ptr<EchoControlMobileImpl> echo_control_mobile;
    std::unique_ptr<NoiseSuppressor> noise_suppressor;
    std::unique_ptr<CaptureLevelsAdjuster> capture_levels_adjuster;
  } submodules_;

  // State that is written to while holding both the render and capture locks
  // but can be read without any lock being held.
  // As this is only accessed internally of APM, and all internal methods in APM
  // either are holding the render or capture locks, this construct is safe as
  // it is not possible to read the variables while writing them.
  struct ApmFormatState {
    ApmFormatState()
        :  // Format of processing streams at input/output call sites.
          api_format({{{kSampleRate16kHz, 1},
                       {kSampleRate16kHz, 1},
                       {kSampleRate16kHz, 1},
                       {kSampleRate16kHz, 1}}}),
          render_processing_format(kSampleRate16kHz, 1) {}
    ProcessingConfig api_format;
    StreamConfig render_processing_format;
  } formats_;

  // APM constants.
  const struct ApmConstants {
    ApmConstants(bool multi_channel_render_support,
                 bool multi_channel_capture_support,
                 bool enforce_split_band_hpf,
                 bool minimize_processing_for_unused_output)
        : multi_channel_render_support(multi_channel_render_support),
          multi_channel_capture_support(multi_channel_capture_support),
          enforce_split_band_hpf(enforce_split_band_hpf),
          minimize_processing_for_unused_output(
              minimize_processing_for_unused_output) {}
    bool multi_channel_render_support;
    bool multi_channel_capture_support;
    bool enforce_split_band_hpf;
    bool minimize_processing_for_unused_output;
  } constants_;

  struct ApmCaptureState {
    ApmCaptureState();
    ~ApmCaptureState();
    bool was_stream_delay_set;
    bool capture_output_used;
    bool capture_output_used_last_frame;
    bool key_pressed;
    std::unique_ptr<AudioBuffer> capture_audio;
    std::unique_ptr<AudioBuffer> capture_fullband_audio;
    std::unique_ptr<AudioBuffer> linear_aec_output;
    // Only the rate and samples fields of capture_processing_format_ are used
    // because the capture processing number of channels is mutable and is
    // tracked by the capture_audio_.
    StreamConfig capture_processing_format;
    int split_rate;
    bool echo_path_gain_change;
    float prev_pre_adjustment_gain;
    int playout_volume;
    int prev_playout_volume;
    AudioProcessingStats stats;
    // Input volume applied on the audio input device when the audio is
    // acquired. Unspecified when unknown.
    std::optional<int> applied_input_volume;
    bool applied_input_volume_changed;
    // Recommended input volume to apply on the audio input device the next time
    // that audio is acquired. Unspecified when no input volume can be
    // recommended.
    std::optional<int> recommended_input_volume;
  } capture_ RTC_GUARDED_BY(mutex_capture_);

  struct ApmCaptureNonLockedState {
    ApmCaptureNonLockedState()
        : capture_processing_format(kSampleRate16kHz),
          split_rate(kSampleRate16kHz),
          stream_delay_ms(0) {}
    // Only the rate and samples fields of capture_processing_format_ are used
    // because the forward processing number of channels is mutable and is
    // tracked by the capture_audio_.
    StreamConfig capture_processing_format;
    int split_rate;
    int stream_delay_ms;
    bool echo_controller_enabled = false;
  } capture_nonlocked_;

  struct ApmRenderState {
    ApmRenderState();
    ~ApmRenderState();
    std::unique_ptr<AudioConverter> render_converter;
    std::unique_ptr<AudioBuffer> render_audio;
  } render_ RTC_GUARDED_BY(mutex_render_);

  // Class for statistics reporting. The class is thread-safe and no lock is
  // needed when accessing it.
  class ApmStatsReporter {
   public:
    ApmStatsReporter();
    ~ApmStatsReporter();

    // Returns the most recently reported statistics.
    AudioProcessingStats GetStatistics();

    // Update the cached statistics.
    void UpdateStatistics(const AudioProcessingStats& new_stats);

   private:
    Mutex mutex_stats_;
    AudioProcessingStats cached_stats_ RTC_GUARDED_BY(mutex_stats_);
    SwapQueue<AudioProcessingStats> stats_message_queue_;
  } stats_reporter_;

  std::vector<int16_t> aecm_render_queue_buffer_ RTC_GUARDED_BY(mutex_render_);
  std::vector<int16_t> aecm_capture_queue_buffer_
      RTC_GUARDED_BY(mutex_capture_);

  size_t agc_render_queue_element_max_size_ RTC_GUARDED_BY(mutex_render_)
      RTC_GUARDED_BY(mutex_capture_) = 0;
  std::vector<int16_t> agc_render_queue_buffer_ RTC_GUARDED_BY(mutex_render_);
  std::vector<int16_t> agc_capture_queue_buffer_ RTC_GUARDED_BY(mutex_capture_);

  size_t red_render_queue_element_max_size_ RTC_GUARDED_BY(mutex_render_)
      RTC_GUARDED_BY(mutex_capture_) = 0;
  std::vector<float> red_render_queue_buffer_ RTC_GUARDED_BY(mutex_render_);
  std::vector<float> red_capture_queue_buffer_ RTC_GUARDED_BY(mutex_capture_);

  RmsLevel capture_input_rms_ RTC_GUARDED_BY(mutex_capture_);
  RmsLevel capture_output_rms_ RTC_GUARDED_BY(mutex_capture_);
  int capture_rms_interval_counter_ RTC_GUARDED_BY(mutex_capture_) = 0;

  InputVolumeStatsReporter applied_input_volume_stats_reporter_
      RTC_GUARDED_BY(mutex_capture_);
  InputVolumeStatsReporter recommended_input_volume_stats_reporter_
      RTC_GUARDED_BY(mutex_capture_);

  // Lock protection not needed.
  std::unique_ptr<
      SwapQueue<std::vector<int16_t>, RenderQueueItemVerifier<int16_t>>>
      aecm_render_signal_queue_;
  std::unique_ptr<
      SwapQueue<std::vector<int16_t>, RenderQueueItemVerifier<int16_t>>>
      agc_render_signal_queue_;
  std::unique_ptr<SwapQueue<std::vector<float>, RenderQueueItemVerifier<float>>>
      red_render_signal_queue_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AUDIO_PROCESSING_IMPL_H_
