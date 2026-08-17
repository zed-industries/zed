/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef VIDEO_VIDEO_SEND_STREAM_IMPL_H_
#define VIDEO_VIDEO_SEND_STREAM_IMPL_H_

#include <stddef.h>
#include <stdint.h>

#include <atomic>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/environment/environment.h"
#include "api/field_trials_view.h"
#include "api/metronome/metronome.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "api/video/encoded_image.h"
#include "api/video/video_bitrate_allocation.h"
#include "api/video_codecs/video_encoder.h"
#include "call/bitrate_allocator.h"
#include "call/rtp_config.h"
#include "call/rtp_transport_controller_send_interface.h"
#include "call/rtp_video_sender_interface.h"
#include "call/video_send_stream.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/video_coding/include/video_codec_interface.h"
#include "rtc_base/experiments/field_trial_parser.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "rtc_base/thread_annotations.h"
#include "video/config/video_encoder_config.h"
#include "video/encoder_rtcp_feedback.h"
#include "video/send_delay_stats.h"
#include "video/send_statistics_proxy.h"
#include "video/video_stream_encoder_interface.h"

namespace webrtc {

namespace test {
class VideoSendStreamPeer;
}  // namespace test

namespace internal {

// Pacing buffer config; overridden by ALR config if provided.
struct PacingConfig {
  explicit PacingConfig(const FieldTrialsView& field_trials);
  PacingConfig(const PacingConfig&);
  PacingConfig& operator=(const PacingConfig&) = default;
  ~PacingConfig();
  FieldTrialParameter<double> pacing_factor;
  FieldTrialParameter<TimeDelta> max_pacing_delay;
};

// VideoSendStreamImpl implements webrtc::VideoSendStream.
// It is created and destroyed on `worker queue`. The intent is to
// An encoder may deliver frames through the EncodedImageCallback on an
// arbitrary thread.
class VideoSendStreamImpl : public webrtc::VideoSendStream,
                            public webrtc::BitrateAllocatorObserver,
                            public VideoStreamEncoderInterface::EncoderSink {
 public:
  using RtpStateMap = std::map<uint32_t, RtpState>;
  using RtpPayloadStateMap = std::map<uint32_t, RtpPayloadState>;

  VideoSendStreamImpl(const Environment& env,
                      int num_cpu_cores,
                      RtcpRttStats* call_stats,
                      RtpTransportControllerSendInterface* transport,
                      Metronome* metronome,
                      BitrateAllocatorInterface* bitrate_allocator,
                      SendDelayStats* send_delay_stats,
                      VideoSendStream::Config config,
                      VideoEncoderConfig encoder_config,
                      const RtpStateMap& suspended_ssrcs,
                      const RtpPayloadStateMap& suspended_payload_states,
                      std::unique_ptr<FecController> fec_controller,
                      std::unique_ptr<VideoStreamEncoderInterface>
                          video_stream_encoder_for_test = nullptr);
  ~VideoSendStreamImpl() override;

  void DeliverRtcp(const uint8_t* packet, size_t length);

  // webrtc::VideoSendStream implementation.
  void Start() override;
  void Stop() override;
  bool started() override;

  void AddAdaptationResource(scoped_refptr<Resource> resource) override;
  std::vector<scoped_refptr<Resource>> GetAdaptationResources() override;

  void SetSource(VideoSourceInterface<webrtc::VideoFrame>* source,
                 const DegradationPreference& degradation_preference) override;

  void ReconfigureVideoEncoder(VideoEncoderConfig config) override;
  void ReconfigureVideoEncoder(VideoEncoderConfig config,
                               SetParametersCallback callback) override;
  Stats GetStats() override;
  void SetStats(const Stats& stats) override;

  void StopPermanentlyAndGetRtpStates(RtpStateMap* rtp_state_map,
                                      RtpPayloadStateMap* payload_state_map);
  void GenerateKeyFrame(const std::vector<std::string>& rids) override;

  // TODO(holmer): Move these to RtpTransportControllerSend.
  std::map<uint32_t, RtpState> GetRtpStates() const;

  std::map<uint32_t, RtpPayloadState> GetRtpPayloadStates() const;

  const std::optional<float>& configured_pacing_factor() const {
    return configured_pacing_factor_;
  }

 private:
  friend class test::VideoSendStreamPeer;
  class OnSendPacketObserver : public SendPacketObserver {
   public:
    OnSendPacketObserver(SendStatisticsProxy* stats_proxy,
                         SendDelayStats* send_delay_stats)
        : stats_proxy_(*stats_proxy), send_delay_stats_(*send_delay_stats) {}

    void OnSendPacket(std::optional<uint16_t> packet_id,
                      Timestamp capture_time,
                      uint32_t ssrc) override {
      stats_proxy_.OnSendPacket(ssrc, capture_time);
      if (packet_id.has_value()) {
        send_delay_stats_.OnSendPacket(*packet_id, capture_time, ssrc);
      }
    }

   private:
    SendStatisticsProxy& stats_proxy_;
    SendDelayStats& send_delay_stats_;
  };

  std::optional<float> GetPacingFactorOverride() const;
  // Implements BitrateAllocatorObserver.
  uint32_t OnBitrateUpdated(BitrateAllocationUpdate update) override;
  std::optional<DataRate> GetUsedRate() const override;

  // Implements VideoStreamEncoderInterface::EncoderSink
  void OnEncoderConfigurationChanged(
      std::vector<VideoStream> streams,
      bool is_svc,
      VideoEncoderConfig::ContentType content_type,
      int min_transmit_bitrate_bps) override;

  void OnBitrateAllocationUpdated(
      const VideoBitrateAllocation& allocation) override;
  void OnVideoLayersAllocationUpdated(
      VideoLayersAllocation allocation) override;

  // Implements EncodedImageCallback. The implementation routes encoded frames
  // to the `payload_router_` and `config.pre_encode_callback` if set.
  // Called on an arbitrary encoder callback thread.
  EncodedImageCallback::Result OnEncodedImage(
      const EncodedImage& encoded_image,
      const CodecSpecificInfo* codec_specific_info) override;

  // Implements EncodedImageCallback.
  void OnDroppedFrame(EncodedImageCallback::DropReason reason) override;

  // Starts monitoring and sends a keyframe.
  void StartupVideoSendStream();
  // Removes the bitrate observer, stops monitoring and notifies the video
  // encoder of the bitrate update.
  void StopVideoSendStream() RTC_RUN_ON(thread_checker_);

  void ConfigureProtection();
  void ConfigureSsrcs();
  void SignalEncoderTimedOut();
  void SignalEncoderActive();
  // A video send stream is running if VideoSendStream::Start has been invoked
  // and there is an active encoding.
  bool IsRunning() const;
  MediaStreamAllocationConfig GetAllocationConfig() const
      RTC_RUN_ON(thread_checker_);

  const Environment env_;
  RTC_NO_UNIQUE_ADDRESS SequenceChecker thread_checker_;

  RtpTransportControllerSendInterface* const transport_;

  SendStatisticsProxy stats_proxy_;
  OnSendPacketObserver send_packet_observer_;
  const VideoSendStream::Config config_;
  const VideoEncoderConfig::ContentType content_type_;
  std::unique_ptr<VideoStreamEncoderInterface> video_stream_encoder_;
  EncoderRtcpFeedback encoder_feedback_;
  RtpVideoSenderInterface* const rtp_video_sender_;
  bool running_ RTC_GUARDED_BY(thread_checker_) = false;

  const bool has_alr_probing_;
  const PacingConfig pacing_config_;

  TaskQueueBase* const worker_queue_;

  RepeatingTaskHandle check_encoder_activity_task_
      RTC_GUARDED_BY(thread_checker_);

  std::atomic_bool activity_;
  bool timed_out_ RTC_GUARDED_BY(thread_checker_);

  BitrateAllocatorInterface* const bitrate_allocator_;

  bool has_active_encodings_ RTC_GUARDED_BY(thread_checker_);
  bool disable_padding_ RTC_GUARDED_BY(thread_checker_);
  int max_padding_bitrate_ RTC_GUARDED_BY(thread_checker_);
  int encoder_min_bitrate_bps_ RTC_GUARDED_BY(thread_checker_);
  uint32_t encoder_max_bitrate_bps_ RTC_GUARDED_BY(thread_checker_);
  uint32_t encoder_target_rate_bps_ RTC_GUARDED_BY(thread_checker_);
  double encoder_bitrate_priority_ RTC_GUARDED_BY(thread_checker_);
  const int encoder_av1_priority_bitrate_override_bps_
      RTC_GUARDED_BY(thread_checker_);

  ScopedTaskSafety worker_queue_safety_;

  // Context for the most recent and last sent video bitrate allocation. Used to
  // throttle sending of similar bitrate allocations.
  struct VbaSendContext {
    VideoBitrateAllocation last_sent_allocation;
    std::optional<VideoBitrateAllocation> throttled_allocation;
    int64_t last_send_time_ms;
  };
  std::optional<VbaSendContext> video_bitrate_allocation_context_
      RTC_GUARDED_BY(thread_checker_);
  const std::optional<float> configured_pacing_factor_;
};
}  // namespace internal
}  // namespace webrtc
#endif  // VIDEO_VIDEO_SEND_STREAM_IMPL_H_
