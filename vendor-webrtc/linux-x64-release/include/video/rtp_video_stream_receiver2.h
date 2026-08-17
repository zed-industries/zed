/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_RTP_VIDEO_STREAM_RECEIVER2_H_
#define VIDEO_RTP_VIDEO_STREAM_RECEIVER2_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <variant>
#include <vector>

#include "api/crypto/frame_decryptor_interface.h"
#include "api/environment/environment.h"
#include "api/frame_transformer_interface.h"
#include "api/rtp_headers.h"
#include "api/rtp_packet_info.h"
#include "api/rtp_parameters.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "api/transport/rtp/dependency_descriptor.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "api/video/color_space.h"
#include "api/video/encoded_frame.h"
#include "api/video/video_codec_constants.h"
#include "api/video/video_codec_type.h"
#include "call/rtp_packet_sink_interface.h"
#include "call/syncable.h"
#include "call/video_receive_stream.h"
#include "common_video/frame_instrumentation_data.h"
#include "modules/include/module_common_types.h"
#include "modules/rtp_rtcp/include/receive_statistics.h"
#include "modules/rtp_rtcp/include/recovered_packet_receiver.h"
#include "modules/rtp_rtcp/include/remote_ntp_time_estimator.h"
#include "modules/rtp_rtcp/include/rtcp_statistics.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/absolute_capture_time_interpolator.h"
#include "modules/rtp_rtcp/source/capture_clock_offset_updater.h"
#include "modules/rtp_rtcp/source/frame_object.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "modules/rtp_rtcp/source/rtp_rtcp_impl2.h"
#include "modules/rtp_rtcp/source/rtp_rtcp_interface.h"
#include "modules/rtp_rtcp/source/rtp_video_header.h"
#include "modules/rtp_rtcp/source/rtp_video_stream_receiver_frame_transformer_delegate.h"
#include "modules/rtp_rtcp/source/video_rtp_depacketizer.h"
#include "modules/video_coding/h264_sps_pps_tracker.h"
#include "modules/video_coding/h26x_packet_buffer.h"
#include "modules/video_coding/loss_notification_controller.h"
#include "modules/video_coding/nack_requester.h"
#include "modules/video_coding/packet_buffer.h"
#include "modules/video_coding/rtp_frame_reference_finder.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/experiments/field_trial_parser.h"
#include "rtc_base/numerics/sequence_number_unwrapper.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"
#include "video/buffered_frame_decryptor.h"
#include "video/unique_timestamp_counter.h"

namespace webrtc {

class NackRequester;
class PacketRouter;
class ReceiveStatistics;
class RtcpRttStats;
class RtpPacketReceived;
class Transport;
class UlpfecReceiver;

class RtpVideoStreamReceiver2 : public LossNotificationSender,
                                public RecoveredPacketReceiver,
                                public RtpPacketSinkInterface,
                                public KeyFrameRequestSender,
                                public NackSender,
                                public OnDecryptedFrameCallback,
                                public OnDecryptionStatusChangeCallback,
                                public RtpVideoFrameReceiver {
 public:
  // A complete frame is a frame which has received all its packets and all its
  // references are known.
  class OnCompleteFrameCallback {
   public:
    virtual ~OnCompleteFrameCallback() {}
    virtual void OnCompleteFrame(std::unique_ptr<EncodedFrame> frame) = 0;
  };

  RtpVideoStreamReceiver2(
      const Environment& env,
      TaskQueueBase* current_queue,
      Transport* transport,
      RtcpRttStats* rtt_stats,
      // The packet router is optional; if provided, the RtpRtcp module for this
      // stream is registered as a candidate for sending REMB and transport
      // feedback.
      PacketRouter* packet_router,
      const VideoReceiveStreamInterface::Config* config,
      ReceiveStatistics* rtp_receive_statistics,
      RtcpPacketTypeCounterObserver* rtcp_packet_type_counter_observer,
      RtcpCnameCallback* rtcp_cname_callback,
      NackPeriodicProcessor* nack_periodic_processor,
      // The KeyFrameRequestSender is optional; if not provided, key frame
      // requests are sent via the internal RtpRtcp module.
      OnCompleteFrameCallback* complete_frame_callback,
      scoped_refptr<FrameDecryptorInterface> frame_decryptor,
      scoped_refptr<FrameTransformerInterface> frame_transformer);
  ~RtpVideoStreamReceiver2() override;

  void AddReceiveCodec(uint8_t payload_type,
                       VideoCodecType video_codec,
                       const webrtc::CodecParameterMap& codec_params,
                       bool raw_payload);

  // Clears state for all receive codecs added via `AddReceiveCodec`.
  void RemoveReceiveCodecs();

  void StartReceive();
  void StopReceive();

  // Produces the transport-related timestamps; current_delay_ms is left unset.
  std::optional<Syncable::Info> GetSyncInfo() const;

  bool DeliverRtcp(const uint8_t* rtcp_packet, size_t rtcp_packet_length);

  void FrameContinuous(int64_t seq_num);

  void FrameDecoded(int64_t seq_num);

  void SignalNetworkState(NetworkState state);

  // Returns number of different frames seen.
  int GetUniqueFramesSeen() const {
    RTC_DCHECK_RUN_ON(&packet_sequence_checker_);
    return frame_counter_.GetUniqueSeen();
  }

  // Implements RtpPacketSinkInterface.
  void OnRtpPacket(const RtpPacketReceived& packet) override;

  // Public only for tests.
  // Returns true if the packet should be stashed and retried at a later stage.
  bool OnReceivedPayloadData(CopyOnWriteBuffer codec_payload,
                             const RtpPacketReceived& rtp_packet,
                             const RTPVideoHeader& video,
                             int times_nacked);

  // Implements RecoveredPacketReceiver.
  void OnRecoveredPacket(const RtpPacketReceived& packet) override;

  // Send an RTCP keyframe request.
  void RequestKeyFrame() override;

  // Implements NackSender.
  void SendNack(const std::vector<uint16_t>& sequence_numbers,
                bool buffering_allowed) override;

  // Implements LossNotificationSender.
  void SendLossNotification(uint16_t last_decoded_seq_num,
                            uint16_t last_received_seq_num,
                            bool decodability_flag,
                            bool buffering_allowed) override;

  // Returns true if a decryptor is attached and frames can be decrypted.
  // Updated by OnDecryptionStatusChangeCallback. Note this refers to Frame
  // Decryption not SRTP.
  bool IsDecryptable() const;

  // Implements OnDecryptedFrameCallback.
  void OnDecryptedFrame(std::unique_ptr<RtpFrameObject> frame) override;

  // Implements OnDecryptionStatusChangeCallback.
  void OnDecryptionStatusChange(
      FrameDecryptorInterface::Status status) override;

  // Optionally set a frame decryptor after a stream has started. This will not
  // reset the decoder state.
  void SetFrameDecryptor(
      scoped_refptr<FrameDecryptorInterface> frame_decryptor);

  // Sets a frame transformer after a stream has started, if no transformer
  // has previously been set. Does not reset the decoder state.
  void SetDepacketizerToDecoderFrameTransformer(
      scoped_refptr<FrameTransformerInterface> frame_transformer);

  // Called by VideoReceiveStreamInterface when stats are updated.
  void UpdateRtt(int64_t max_rtt_ms);

  // Called when the local_ssrc is changed to match with a sender.
  void OnLocalSsrcChange(uint32_t local_ssrc);

  // Forwards the call to set rtcp_sender_ to the RTCP mode of the rtcp sender.
  void SetRtcpMode(RtcpMode mode);

  void SetReferenceTimeReport(bool enabled);

  // Sets or clears the callback sink that gets called for RTP packets. Used for
  // packet handlers such as FlexFec. Must be called on the packet delivery
  // thread (same context as `OnRtpPacket` is called on).
  // TODO(bugs.webrtc.org/11993): Packet delivery thread today means `worker
  // thread` but will be `network thread`.
  void SetPacketSink(RtpPacketSinkInterface* packet_sink);

  // Turns on/off loss notifications. Must be called on the packet delivery
  // thread.
  void SetLossNotificationEnabled(bool enabled);

  void SetNackHistory(TimeDelta history);

  int ulpfec_payload_type() const;
  int red_payload_type() const;
  void SetProtectionPayloadTypes(int red_payload_type, int ulpfec_payload_type);

  std::optional<int64_t> LastReceivedPacketMs() const;
  std::optional<uint32_t> LastReceivedFrameRtpTimestamp() const;
  std::optional<int64_t> LastReceivedKeyframePacketMs() const;

  std::optional<RtpRtcpInterface::SenderReportStats> GetSenderReportStats()
      const;

 private:
  // Implements RtpVideoFrameReceiver.
  void ManageFrame(std::unique_ptr<RtpFrameObject> frame) override;

  void OnCompleteFrames(RtpFrameReferenceFinder::ReturnVector frame)
      RTC_RUN_ON(packet_sequence_checker_);

  // Used for buffering RTCP feedback messages and sending them all together.
  // Note:
  // 1. Key frame requests and NACKs are mutually exclusive, with the
  //    former taking precedence over the latter.
  // 2. Loss notifications are orthogonal to either. (That is, may be sent
  //    alongside either.)
  class RtcpFeedbackBuffer : public KeyFrameRequestSender,
                             public NackSender,
                             public LossNotificationSender {
   public:
    RtcpFeedbackBuffer(KeyFrameRequestSender* key_frame_request_sender,
                       NackSender* nack_sender,
                       LossNotificationSender* loss_notification_sender);

    ~RtcpFeedbackBuffer() override = default;

    // KeyFrameRequestSender implementation.
    void RequestKeyFrame() override;

    // NackSender implementation.
    void SendNack(const std::vector<uint16_t>& sequence_numbers,
                  bool buffering_allowed) override;

    // LossNotificationSender implementation.
    void SendLossNotification(uint16_t last_decoded_seq_num,
                              uint16_t last_received_seq_num,
                              bool decodability_flag,
                              bool buffering_allowed) override;

    // Send all RTCP feedback messages buffered thus far.
    void SendBufferedRtcpFeedback();

    void ClearLossNotificationState();

   private:
    // LNTF-related state.
    struct LossNotificationState {
      LossNotificationState(uint16_t last_decoded_seq_num,
                            uint16_t last_received_seq_num,
                            bool decodability_flag)
          : last_decoded_seq_num(last_decoded_seq_num),
            last_received_seq_num(last_received_seq_num),
            decodability_flag(decodability_flag) {}

      uint16_t last_decoded_seq_num;
      uint16_t last_received_seq_num;
      bool decodability_flag;
    };

    RTC_NO_UNIQUE_ADDRESS SequenceChecker packet_sequence_checker_;
    KeyFrameRequestSender* const key_frame_request_sender_;
    NackSender* const nack_sender_;
    LossNotificationSender* const loss_notification_sender_;

    // Key-frame-request-related state.
    bool request_key_frame_ RTC_GUARDED_BY(packet_sequence_checker_);

    // NACK-related state.
    std::vector<uint16_t> nack_sequence_numbers_
        RTC_GUARDED_BY(packet_sequence_checker_);

    std::optional<LossNotificationState> lntf_state_
        RTC_GUARDED_BY(packet_sequence_checker_);
  };
  enum ParseGenericDependenciesResult {
    kStashPacket,
    kDropPacket,
    kHasGenericDescriptor,
    kNoGenericDescriptor
  };

  // Entry point doing non-stats work for a received packet. Called
  // for the same packet both before and after RED decapsulation.
  void ReceivePacket(const RtpPacketReceived& packet)
      RTC_RUN_ON(packet_sequence_checker_);

  // Parses and handles RED headers.
  // This function assumes that it's being called from only one thread.
  void ParseAndHandleEncapsulatingHeader(const RtpPacketReceived& packet)
      RTC_RUN_ON(packet_sequence_checker_);
  void NotifyReceiverOfEmptyPacket(uint16_t seq_num,
                                   std::optional<VideoCodecType> codec)
      RTC_RUN_ON(packet_sequence_checker_);
  bool IsRedEnabled() const;
  void InsertSpsPpsIntoTracker(uint8_t payload_type)
      RTC_RUN_ON(packet_sequence_checker_);
  void OnInsertedPacket(video_coding::PacketBuffer::InsertResult result)
      RTC_RUN_ON(packet_sequence_checker_);
  ParseGenericDependenciesResult ParseGenericDependenciesExtension(
      const RtpPacketReceived& rtp_packet,
      RTPVideoHeader* video_header) RTC_RUN_ON(packet_sequence_checker_);
  void OnAssembledFrame(std::unique_ptr<RtpFrameObject> frame)
      RTC_RUN_ON(packet_sequence_checker_);
  void UpdatePacketReceiveTimestamps(const RtpPacketReceived& packet,
                                     bool is_keyframe)
      RTC_RUN_ON(packet_sequence_checker_);
  void SetLastCorruptionDetectionIndex(
      const std::variant<FrameInstrumentationSyncData,
                         FrameInstrumentationData>& frame_instrumentation_data,
      int spatial_idx);

  std::optional<VideoCodecType> GetCodecFromPayloadType(
      uint8_t payload_type) const RTC_RUN_ON(packet_sequence_checker_);
  bool UseH26xPacketBuffer(std::optional<VideoCodecType> codec) const
      RTC_RUN_ON(packet_sequence_checker_);

  const Environment env_;
  TaskQueueBase* const worker_queue_;

  // Ownership of this object lies with VideoReceiveStreamInterface, which owns
  // `this`.
  const VideoReceiveStreamInterface::Config& config_;
  PacketRouter* const packet_router_;

  RemoteNtpTimeEstimator ntp_estimator_;

  // Set by the field trial WebRTC-ForcePlayoutDelay to override any playout
  // delay that is specified in the received packets.
  FieldTrialOptional<int> forced_playout_delay_max_ms_;
  FieldTrialOptional<int> forced_playout_delay_min_ms_;
  ReceiveStatistics* const rtp_receive_statistics_;
  std::unique_ptr<UlpfecReceiver> ulpfec_receiver_
      RTC_GUARDED_BY(packet_sequence_checker_);
  int red_payload_type_ RTC_GUARDED_BY(packet_sequence_checker_);

  RTC_NO_UNIQUE_ADDRESS SequenceChecker worker_task_checker_;
  // TODO(bugs.webrtc.org/11993): This checker conceptually represents
  // operations that belong to the network thread. The Call class is currently
  // moving towards handling network packets on the network thread and while
  // that work is ongoing, this checker may in practice represent the worker
  // thread, but still serves as a mechanism of grouping together concepts
  // that belong to the network thread. Once the packets are fully delivered
  // on the network thread, this comment will be deleted.
  RTC_NO_UNIQUE_ADDRESS SequenceChecker packet_sequence_checker_;
  RtpPacketSinkInterface* packet_sink_ RTC_GUARDED_BY(packet_sequence_checker_);
  bool receiving_ RTC_GUARDED_BY(packet_sequence_checker_);
  int64_t last_packet_log_ms_ RTC_GUARDED_BY(packet_sequence_checker_);

  const std::unique_ptr<ModuleRtpRtcpImpl2> rtp_rtcp_;

  NackPeriodicProcessor* const nack_periodic_processor_;
  OnCompleteFrameCallback* complete_frame_callback_;
  const KeyFrameReqMethod keyframe_request_method_;

  RtcpFeedbackBuffer rtcp_feedback_buffer_;
  // TODO(tommi): Consider std::optional<NackRequester> instead of unique_ptr
  // since nack is usually configured.
  std::unique_ptr<NackRequester> nack_module_
      RTC_GUARDED_BY(packet_sequence_checker_);
  std::unique_ptr<LossNotificationController> loss_notification_controller_
      RTC_GUARDED_BY(packet_sequence_checker_);

  video_coding::PacketBuffer packet_buffer_
      RTC_GUARDED_BY(packet_sequence_checker_);
  // h26x_packet_buffer_ is applicable to H.264 and H.265. For H.265 it is
  // always used but for H.264 it is only used if WebRTC-Video-H26xPacketBuffer
  // is enabled, see condition inside UseH26xPacketBuffer().
  std::unique_ptr<H26xPacketBuffer> h26x_packet_buffer_
      RTC_GUARDED_BY(packet_sequence_checker_);
  UniqueTimestampCounter frame_counter_
      RTC_GUARDED_BY(packet_sequence_checker_);
  SeqNumUnwrapper<uint16_t> frame_id_unwrapper_
      RTC_GUARDED_BY(packet_sequence_checker_);

  // Video structure provided in the dependency descriptor in a first packet
  // of a key frame. It is required to parse dependency descriptor in the
  // following delta packets.
  std::unique_ptr<FrameDependencyStructure> video_structure_
      RTC_GUARDED_BY(packet_sequence_checker_);
  // Frame id of the last frame with the attached video structure.
  // std::nullopt when `video_structure_ == nullptr`;
  std::optional<int64_t> video_structure_frame_id_
      RTC_GUARDED_BY(packet_sequence_checker_);
  Timestamp last_logged_failed_to_parse_dd_
      RTC_GUARDED_BY(packet_sequence_checker_) = Timestamp::MinusInfinity();

  std::unique_ptr<RtpFrameReferenceFinder> reference_finder_
      RTC_GUARDED_BY(packet_sequence_checker_);
  std::optional<VideoCodecType> current_codec_
      RTC_GUARDED_BY(packet_sequence_checker_);
  uint32_t last_assembled_frame_rtp_timestamp_
      RTC_GUARDED_BY(packet_sequence_checker_);

  std::map<int64_t, uint16_t> last_seq_num_for_pic_id_
      RTC_GUARDED_BY(packet_sequence_checker_);
  video_coding::H264SpsPpsTracker tracker_
      RTC_GUARDED_BY(packet_sequence_checker_);

  // Maps payload id to the depacketizer.
  std::map<uint8_t, std::unique_ptr<VideoRtpDepacketizer>> payload_type_map_
      RTC_GUARDED_BY(packet_sequence_checker_);

  // TODO(johan): Remove pt_codec_params_ once
  // https://bugs.chromium.org/p/webrtc/issues/detail?id=6883 is resolved.
  // Maps a payload type to a map of out-of-band supplied codec parameters.
  std::map<uint8_t, webrtc::CodecParameterMap> pt_codec_params_
      RTC_GUARDED_BY(packet_sequence_checker_);

  // Maps payload type to the VideoCodecType.
  std::map<uint8_t, webrtc::VideoCodecType> pt_codec_
      RTC_GUARDED_BY(packet_sequence_checker_);

  int16_t last_payload_type_ RTC_GUARDED_BY(packet_sequence_checker_) = -1;

  bool has_received_frame_ RTC_GUARDED_BY(packet_sequence_checker_);

  std::optional<uint32_t> last_received_rtp_timestamp_
      RTC_GUARDED_BY(packet_sequence_checker_);
  std::optional<uint32_t> last_received_keyframe_rtp_timestamp_
      RTC_GUARDED_BY(packet_sequence_checker_);
  std::optional<Timestamp> last_received_rtp_system_time_
      RTC_GUARDED_BY(packet_sequence_checker_);
  std::optional<Timestamp> last_received_keyframe_rtp_system_time_
      RTC_GUARDED_BY(packet_sequence_checker_);

  // Handles incoming encrypted frames and forwards them to the
  // rtp_reference_finder if they are decryptable.
  std::unique_ptr<BufferedFrameDecryptor> buffered_frame_decryptor_
      RTC_PT_GUARDED_BY(packet_sequence_checker_);
  bool frames_decryptable_ RTC_GUARDED_BY(worker_task_checker_);
  std::optional<ColorSpace> last_color_space_;

  AbsoluteCaptureTimeInterpolator absolute_capture_time_interpolator_
      RTC_GUARDED_BY(packet_sequence_checker_);

  CaptureClockOffsetUpdater capture_clock_offset_updater_
      RTC_GUARDED_BY(packet_sequence_checker_);

  int64_t last_completed_picture_id_ = 0;

  scoped_refptr<RtpVideoStreamReceiverFrameTransformerDelegate>
      frame_transformer_delegate_;

  SeqNumUnwrapper<uint16_t> rtp_seq_num_unwrapper_
      RTC_GUARDED_BY(packet_sequence_checker_);
  std::map<int64_t, RtpPacketInfo> packet_infos_
      RTC_GUARDED_BY(packet_sequence_checker_);
  std::vector<RtpPacketReceived> stashed_packets_
      RTC_GUARDED_BY(packet_sequence_checker_);

  Timestamp next_keyframe_request_for_missing_video_structure_ =
      Timestamp::MinusInfinity();
  bool sps_pps_idr_is_h264_keyframe_ = false;

  struct CorruptionDetectionLayerState {
    int sequence_index = 0;
    std::optional<uint32_t> timestamp;
  };
  std::array<CorruptionDetectionLayerState, kMaxSpatialLayers>
      last_corruption_detection_state_by_layer_;
};

}  // namespace webrtc

#endif  // VIDEO_RTP_VIDEO_STREAM_RECEIVER2_H_
