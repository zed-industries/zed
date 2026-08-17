/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_ADAPTATION_VIDEO_STREAM_ENCODER_RESOURCE_MANAGER_H_
#define VIDEO_ADAPTATION_VIDEO_STREAM_ENCODER_RESOURCE_MANAGER_H_

#include <atomic>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include "api/adaptation/resource.h"
#include "api/field_trials_view.h"
#include "api/rtp_parameters.h"
#include "api/scoped_refptr.h"
#include "api/task_queue/task_queue_base.h"
#include "api/video/video_adaptation_counters.h"
#include "api/video/video_adaptation_reason.h"
#include "api/video/video_frame.h"
#include "api/video/video_source_interface.h"
#include "api/video_codecs/video_codec.h"
#include "api/video_codecs/video_encoder.h"
#include "call/adaptation/resource_adaptation_processor_interface.h"
#include "call/adaptation/video_stream_adapter.h"
#include "call/adaptation/video_stream_input_state_provider.h"
#include "rtc_base/experiments/quality_scaler_settings.h"
#include "rtc_base/ref_count.h"
#include "rtc_base/strings/string_builder.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"
#include "video/adaptation/balanced_constraint.h"
#include "video/adaptation/bandwidth_quality_scaler_resource.h"
#include "video/adaptation/bitrate_constraint.h"
#include "video/adaptation/encode_usage_resource.h"
#include "video/adaptation/overuse_frame_detector.h"
#include "video/adaptation/pixel_limit_resource.h"
#include "video/adaptation/quality_scaler_resource.h"
#include "video/adaptation/video_stream_encoder_resource.h"
#include "video/config/video_encoder_config.h"
#include "video/video_stream_encoder_observer.h"

namespace webrtc {

// The assumed input frame size if we have not yet received a frame.
// TODO(hbos): This is 144p - why are we assuming super low quality? Seems like
// a bad heuristic.
extern const int kDefaultInputPixelsWidth;
extern const int kDefaultInputPixelsHeight;

// Owns adaptation-related Resources pertaining to a single VideoStreamEncoder
// and passes on the relevant input from the encoder to the resources. The
// resources provide resource usage states to the ResourceAdaptationProcessor
// which is responsible for reconfiguring streams in order not to overuse
// resources.
//
// The manager is also involved with various mitigations not part of the
// ResourceAdaptationProcessor code such as the initial frame dropping.
class VideoStreamEncoderResourceManager
    : public VideoSourceRestrictionsListener,
      public ResourceLimitationsListener {
 public:
  VideoStreamEncoderResourceManager(
      VideoStreamInputStateProvider* input_state_provider,
      VideoStreamEncoderObserver* encoder_stats_observer,
      Clock* clock,
      bool experiment_cpu_load_estimator,
      std::unique_ptr<OveruseFrameDetector> overuse_detector,
      DegradationPreferenceProvider* degradation_preference_provider,
      const FieldTrialsView& field_trials);
  ~VideoStreamEncoderResourceManager() override;

  void Initialize(TaskQueueBase* encoder_queue);
  void SetAdaptationProcessor(
      ResourceAdaptationProcessorInterface* adaptation_processor,
      VideoStreamAdapter* stream_adapter);

  // TODO(https://crbug.com/webrtc/11563): The degradation preference is a
  // setting of the Processor, it does not belong to the Manager - can we get
  // rid of this?
  void SetDegradationPreferences(DegradationPreference degradation_preference);
  DegradationPreference degradation_preference() const;

  void ConfigureEncodeUsageResource();
  // Initializes the pixel limit resource if the "WebRTC-PixelLimitResource"
  // field trial is enabled. This can be used for testing.
  void MaybeInitializePixelLimitResource();
  // Stops the encode usage and quality scaler resources if not already stopped.
  // If the pixel limit resource was created it is also stopped and nulled.
  void StopManagedResources();

  // Settings that affect the VideoStreamEncoder-specific resources.
  void SetEncoderSettings(EncoderSettings encoder_settings);
  void SetStartBitrate(DataRate start_bitrate);
  void SetTargetBitrate(DataRate target_bitrate);
  void SetEncoderRates(
      const VideoEncoder::RateControlParameters& encoder_rates);
  // TODO(https://crbug.com/webrtc/11338): This can be made private if we
  // configure on SetDegredationPreference and SetEncoderSettings.
  void ConfigureQualityScaler(const VideoEncoder::EncoderInfo& encoder_info);
  void ConfigureBandwidthQualityScaler(
      const VideoEncoder::EncoderInfo& encoder_info);

  // Methods corresponding to different points in the encoding pipeline.
  void OnFrameDroppedDueToSize();
  void OnMaybeEncodeFrame();
  void OnEncodeStarted(const VideoFrame& cropped_frame,
                       int64_t time_when_first_seen_us);
  void OnEncodeCompleted(const EncodedImage& encoded_image,
                         int64_t time_sent_in_us,
                         std::optional<int> encode_duration_us,
                         DataSize frame_size);
  void OnFrameDropped(EncodedImageCallback::DropReason reason);

  // Resources need to be mapped to an AdaptReason (kCpu or kQuality) in order
  // to update legacy getStats().
  void AddResource(scoped_refptr<Resource> resource,
                   VideoAdaptationReason reason);
  void RemoveResource(scoped_refptr<Resource> resource);
  std::vector<AdaptationConstraint*> AdaptationConstraints() const;
  // If true, the VideoStreamEncoder should execute its logic to maybe drop
  // frames based on size and bitrate.
  bool DropInitialFrames() const;
  std::optional<uint32_t> SingleActiveStreamPixels() const;
  std::optional<uint32_t> UseBandwidthAllocationBps() const;

  // VideoSourceRestrictionsListener implementation.
  // Updates `video_source_restrictions_`.
  void OnVideoSourceRestrictionsUpdated(
      VideoSourceRestrictions restrictions,
      const VideoAdaptationCounters& adaptation_counters,
      scoped_refptr<Resource> reason,
      const VideoSourceRestrictions& unfiltered_restrictions) override;
  void OnResourceLimitationChanged(
      scoped_refptr<Resource> resource,
      const std::map<scoped_refptr<Resource>, VideoAdaptationCounters>&
          resource_limitations) override;

  static bool IsSimulcastOrMultipleSpatialLayers(
      const VideoEncoderConfig& encoder_config,
      const VideoCodec& video_codec);

 private:
  class InitialFrameDropper;

  VideoAdaptationReason GetReasonFromResource(
      scoped_refptr<Resource> resource) const;

  CpuOveruseOptions GetCpuOveruseOptions() const;
  int LastFrameSizeOrDefault() const;

  // Calculates an up-to-date value of the target frame rate and informs the
  // `encode_usage_resource_` of the new value.
  void MaybeUpdateTargetFrameRate();

  // Use nullopt to disable quality scaling.
  void UpdateQualityScalerSettings(
      std::optional<VideoEncoder::QpThresholds> qp_thresholds);

  void UpdateBandwidthQualityScalerSettings(
      bool bandwidth_quality_scaling_allowed,
      const std::vector<VideoEncoder::ResolutionBitrateLimits>&
          resolution_bitrate_limits,
      VideoCodecType codec_type);

  void UpdateStatsAdaptationSettings() const;

  static std::string ActiveCountsToString(
      const std::map<VideoAdaptationReason, VideoAdaptationCounters>&
          active_counts);

  const FieldTrialsView& field_trials_;
  DegradationPreferenceProvider* const degradation_preference_provider_;
  std::unique_ptr<BitrateConstraint> bitrate_constraint_
      RTC_GUARDED_BY(encoder_queue_);
  const std::unique_ptr<BalancedConstraint> balanced_constraint_
      RTC_GUARDED_BY(encoder_queue_);
  const scoped_refptr<EncodeUsageResource> encode_usage_resource_;
  const scoped_refptr<QualityScalerResource> quality_scaler_resource_;
  scoped_refptr<PixelLimitResource> pixel_limit_resource_;
  const scoped_refptr<BandwidthQualityScalerResource>
      bandwidth_quality_scaler_resource_;

  TaskQueueBase* encoder_queue_;
  VideoStreamInputStateProvider* const input_state_provider_
      RTC_GUARDED_BY(encoder_queue_);
  ResourceAdaptationProcessorInterface* adaptation_processor_;
  VideoStreamAdapter* stream_adapter_ RTC_GUARDED_BY(encoder_queue_);
  // Thread-safe.
  VideoStreamEncoderObserver* const encoder_stats_observer_;

  DegradationPreference degradation_preference_ RTC_GUARDED_BY(encoder_queue_);
  VideoSourceRestrictions video_source_restrictions_
      RTC_GUARDED_BY(encoder_queue_);

  VideoAdaptationCounters current_adaptation_counters_
      RTC_GUARDED_BY(encoder_queue_);

  const BalancedDegradationSettings balanced_settings_;
  Clock* clock_ RTC_GUARDED_BY(encoder_queue_);
  const bool experiment_cpu_load_estimator_ RTC_GUARDED_BY(encoder_queue_);
  const std::unique_ptr<InitialFrameDropper> initial_frame_dropper_
      RTC_GUARDED_BY(encoder_queue_);
  const bool quality_scaling_experiment_enabled_ RTC_GUARDED_BY(encoder_queue_);
  const bool pixel_limit_resource_experiment_enabled_
      RTC_GUARDED_BY(encoder_queue_);
  std::optional<uint32_t> encoder_target_bitrate_bps_
      RTC_GUARDED_BY(encoder_queue_);
  std::optional<VideoEncoder::RateControlParameters> encoder_rates_
      RTC_GUARDED_BY(encoder_queue_);
  std::optional<EncoderSettings> encoder_settings_
      RTC_GUARDED_BY(encoder_queue_);

  // Ties a resource to a reason for statistical reporting. This AdaptReason is
  // also used by this module to make decisions about how to adapt up/down.
  std::map<scoped_refptr<Resource>, VideoAdaptationReason> resources_
      RTC_GUARDED_BY(encoder_queue_);
};

}  // namespace webrtc

#endif  // VIDEO_ADAPTATION_VIDEO_STREAM_ENCODER_RESOURCE_MANAGER_H_
