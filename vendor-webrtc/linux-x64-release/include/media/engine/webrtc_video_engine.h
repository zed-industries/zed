/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_ENGINE_WEBRTC_VIDEO_ENGINE_H_
#define MEDIA_ENGINE_WEBRTC_VIDEO_ENGINE_H_

#include <stddef.h>

#include <cstdint>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/crypto/crypto_options.h"
#include "api/crypto/frame_decryptor_interface.h"
#include "api/crypto/frame_encryptor_interface.h"
#include "api/field_trials_view.h"
#include "api/frame_transformer_interface.h"
#include "api/media_types.h"
#include "api/rtc_error.h"
#include "api/rtp_headers.h"
#include "api/rtp_parameters.h"
#include "api/rtp_sender_interface.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "api/transport/bitrate_settings.h"
#include "api/transport/rtp/rtp_source.h"
#include "api/video/recordable_encoded_frame.h"
#include "api/video/video_bitrate_allocator_factory.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video/video_source_interface.h"
#include "api/video/video_stream_encoder_settings.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "call/call.h"
#include "call/flexfec_receive_stream.h"
#include "call/rtp_config.h"
#include "call/video_receive_stream.h"
#include "call/video_send_stream.h"
#include "media/base/codec.h"
#include "media/base/media_channel.h"
#include "media/base/media_channel_impl.h"
#include "media/base/media_config.h"
#include "media/base/media_engine.h"
#include "media/base/stream_params.h"
#include "modules/rtp_rtcp/include/rtp_header_extension_map.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "rtc_base/checks.h"
#include "rtc_base/network/sent_packet.h"
#include "rtc_base/network_route.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"
#include "video/config/video_encoder_config.h"

namespace webrtc {
class VideoDecoderFactory;
class VideoEncoderFactory;
}  // namespace webrtc

namespace webrtc {

// Public for testing.
// Inputs StreamStats for all types of substreams (kMedia, kRtx, kFlexfec) and
// merges any non-kMedia substream stats object into its referenced kMedia-type
// substream. The resulting substreams are all kMedia. This means, for example,
// that packet and byte counters of RTX and FlexFEC streams are accounted for in
// the relevant RTP media stream's stats. This makes the resulting StreamStats
// objects ready to be turned into "outbound-rtp" stats objects for GetStats()
// which does not create separate stream stats objects for complementary
// streams.
std::map<uint32_t, VideoSendStream::StreamStats>
MergeInfoAboutOutboundRtpSubstreamsForTesting(
    const std::map<uint32_t, VideoSendStream::StreamStats>& substreams);

// WebRtcVideoEngine is used for the new native WebRTC Video API (webrtc:1667).
class WebRtcVideoEngine : public VideoEngineInterface {
 public:
  // These video codec factories represents all video codecs, i.e. both software
  // and external hardware codecs.
  WebRtcVideoEngine(std::unique_ptr<VideoEncoderFactory> video_encoder_factory,
                    std::unique_ptr<VideoDecoderFactory> video_decoder_factory,
                    const FieldTrialsView& trials);

  ~WebRtcVideoEngine() override;

  std::unique_ptr<VideoMediaSendChannelInterface> CreateSendChannel(
      Call* call,
      const MediaConfig& config,
      const VideoOptions& options,
      const CryptoOptions& crypto_options,
      VideoBitrateAllocatorFactory* video_bitrate_allocator_factory) override;
  std::unique_ptr<VideoMediaReceiveChannelInterface> CreateReceiveChannel(
      Call* call,
      const MediaConfig& config,
      const VideoOptions& options,
      const CryptoOptions& crypto_options) override;

  // TODO: https://issues.webrtc.org/360058654 - remove Legacy functions.
  std::vector<Codec> LegacySendCodecs() const override {
    return LegacySendCodecs(true);
  }
  std::vector<Codec> LegacyRecvCodecs() const override {
    return LegacyRecvCodecs(true);
  }
  std::vector<Codec> LegacySendCodecs(bool include_rtx) const override;
  std::vector<Codec> LegacyRecvCodecs(bool include_rtx) const override;
  std::vector<RtpHeaderExtensionCapability> GetRtpHeaderExtensions()
      const override;

 private:
  const std::unique_ptr<VideoDecoderFactory> decoder_factory_;
  const std::unique_ptr<VideoEncoderFactory> encoder_factory_;
  const std::unique_ptr<VideoBitrateAllocatorFactory>
      bitrate_allocator_factory_;
  const FieldTrialsView& trials_;
};

struct VideoCodecSettings {
  explicit VideoCodecSettings(const Codec& codec);

  // Checks if all members of |*this| are equal to the corresponding members
  // of `other`.
  bool operator==(const VideoCodecSettings& other) const;
  bool operator!=(const VideoCodecSettings& other) const;

  // Checks if all members of `a`, except `flexfec_payload_type`, are equal
  // to the corresponding members of `b`.
  static bool EqualsDisregardingFlexfec(const VideoCodecSettings& a,
                                        const VideoCodecSettings& b);

  Codec codec;
  UlpfecConfig ulpfec;
  int flexfec_payload_type;  // -1 if absent.
  int rtx_payload_type;      // -1 if absent.
  std::optional<int> rtx_time;
};

class WebRtcVideoSendChannel : public MediaChannelUtil,
                               public VideoMediaSendChannelInterface,
                               public EncoderSwitchRequestCallback {
 public:
  WebRtcVideoSendChannel(
      Call* call,
      const MediaConfig& config,
      const VideoOptions& options,
      const CryptoOptions& crypto_options,
      VideoEncoderFactory* encoder_factory,
      VideoDecoderFactory* decoder_factory,
      VideoBitrateAllocatorFactory* bitrate_allocator_factory);
  ~WebRtcVideoSendChannel() override;

  MediaType media_type() const override { return MediaType::VIDEO; }
  // Type manipulations
  VideoMediaSendChannelInterface* AsVideoSendChannel() override { return this; }
  VoiceMediaSendChannelInterface* AsVoiceSendChannel() override {
    RTC_CHECK_NOTREACHED();
    return nullptr;
  }
  // Functions imported from MediaChannelUtil
  bool HasNetworkInterface() const override {
    return MediaChannelUtil::HasNetworkInterface();
  }
  void SetExtmapAllowMixed(bool extmap_allow_mixed) override {
    MediaChannelUtil::SetExtmapAllowMixed(extmap_allow_mixed);
  }
  bool ExtmapAllowMixed() const override {
    return MediaChannelUtil::ExtmapAllowMixed();
  }

  // Common functions between sender and receiver
  void SetInterface(MediaChannelNetworkInterface* iface) override;
  // VideoMediaSendChannelInterface implementation
  bool SetSenderParameters(const VideoSenderParameters& params) override;
  RTCError SetRtpSendParameters(uint32_t ssrc,
                                const RtpParameters& parameters,
                                SetParametersCallback callback) override;
  RtpParameters GetRtpSendParameters(uint32_t ssrc) const override;
  std::optional<Codec> GetSendCodec() const override;
  bool SetSend(bool send) override;
  bool SetVideoSend(uint32_t ssrc,
                    const VideoOptions* options,
                    VideoSourceInterface<VideoFrame>* source) override;
  bool AddSendStream(const StreamParams& sp) override;
  bool RemoveSendStream(uint32_t ssrc) override;
  void FillBitrateInfo(BandwidthEstimationInfo* bwe_info) override;
  bool GetStats(VideoMediaSendInfo* info) override;

  void OnPacketSent(const SentPacketInfo& sent_packet) override;
  void OnReadyToSend(bool ready) override;
  void OnNetworkRouteChanged(absl::string_view transport_name,
                             const NetworkRoute& network_route) override;

  // Set a frame encryptor to a particular ssrc that will intercept all
  // outgoing video frames and attempt to encrypt them and forward the result
  // to the packetizer.
  void SetFrameEncryptor(
      uint32_t ssrc,
      scoped_refptr<FrameEncryptorInterface> frame_encryptor) override;

  // note: The encoder_selector object must remain valid for the lifetime of the
  // MediaChannel, unless replaced.
  void SetEncoderSelector(
      uint32_t ssrc,
      VideoEncoderFactory::EncoderSelectorInterface* encoder_selector) override;

  void SetSendCodecChangedCallback(
      absl::AnyInvocable<void()> callback) override {
    send_codec_changed_callback_ = std::move(callback);
  }

  void SetSsrcListChangedCallback(
      absl::AnyInvocable<void(const std::set<uint32_t>&)> callback) override {
    ssrc_list_changed_callback_ = std::move(callback);
  }

  // Implemented for VideoMediaChannelTest.
  bool sending() const {
    RTC_DCHECK_RUN_ON(&thread_checker_);
    return sending_;
  }

  // AdaptReason is used for expressing why a WebRtcVideoSendStream request
  // a lower input frame size than the currently configured camera input frame
  // size. There can be more than one reason OR:ed together.
  enum AdaptReason {
    ADAPTREASON_NONE = 0,
    ADAPTREASON_CPU = 1,
    ADAPTREASON_BANDWIDTH = 2,
  };

  // Implements webrtc::EncoderSwitchRequestCallback.
  void RequestEncoderFallback() override;
  void RequestEncoderSwitch(const SdpVideoFormat& format,
                            bool allow_default_fallback) override;

  void GenerateSendKeyFrame(uint32_t ssrc,
                            const std::vector<std::string>& rids) override;

  void SetEncoderToPacketizerFrameTransformer(
      uint32_t ssrc,
      scoped_refptr<FrameTransformerInterface> frame_transformer) override;
  // Information queries to support SetReceiverFeedbackParameters
  RtcpMode SendCodecRtcpMode() const override {
    RTC_DCHECK_RUN_ON(&thread_checker_);
    return send_params_.rtcp.reduced_size ? RtcpMode::kReducedSize
                                          : RtcpMode::kCompound;
  }

  bool SendCodecHasLntf() const override {
    RTC_DCHECK_RUN_ON(&thread_checker_);
    if (!send_codec()) {
      return false;
    }
    return webrtc::HasLntf(send_codec()->codec);
  }
  bool SendCodecHasNack() const override {
    RTC_DCHECK_RUN_ON(&thread_checker_);
    if (!send_codec()) {
      return false;
    }
    return webrtc::HasNack(send_codec()->codec);
  }
  std::optional<int> SendCodecRtxTime() const override {
    RTC_DCHECK_RUN_ON(&thread_checker_);
    if (!send_codec()) {
      return std::nullopt;
    }
    return send_codec()->rtx_time;
  }

 private:
  struct ChangedSenderParameters {
    // These optionals are unset if not changed.
    std::optional<VideoCodecSettings> send_codec;
    std::optional<std::vector<VideoCodecSettings>> negotiated_codecs;
    std::optional<std::vector<VideoCodecSettings>> send_codecs;
    std::optional<std::vector<RtpExtension>> rtp_header_extensions;
    std::optional<std::string> mid;
    std::optional<bool> extmap_allow_mixed;
    std::optional<int> max_bandwidth_bps;
    std::optional<bool> conference_mode;
    std::optional<RtcpMode> rtcp_mode;
  };

  bool GetChangedSenderParameters(const VideoSenderParameters& params,
                                  ChangedSenderParameters* changed_params) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);
  bool ApplyChangedParams(const ChangedSenderParameters& changed_params);
  bool ValidateSendSsrcAvailability(const StreamParams& sp) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);

  // Populates `rtx_associated_payload_types`, `raw_payload_types` and
  // `decoders` based on codec settings provided by `recv_codecs`.
  // `recv_codecs` must be non-empty and all other parameters must be empty.
  static void ExtractCodecInformation(
      ArrayView<const VideoCodecSettings> recv_codecs,
      std::map<int, int>& rtx_associated_payload_types,
      std::set<int>& raw_payload_types,
      std::vector<VideoReceiveStreamInterface::Decoder>& decoders);

  // Wrapper for the sender part.
  class WebRtcVideoSendStream {
   public:
    WebRtcVideoSendStream(
        Call* call,
        const StreamParams& sp,
        VideoSendStream::Config config,
        const VideoOptions& options,
        bool enable_cpu_overuse_detection,
        int max_bitrate_bps,
        const std::optional<VideoCodecSettings>& codec_settings,
        const std::vector<VideoCodecSettings>& codec_settings_list,
        const std::optional<std::vector<RtpExtension>>& rtp_extensions,
        const VideoSenderParameters& send_params);
    ~WebRtcVideoSendStream();

    void SetSenderParameters(const ChangedSenderParameters& send_params);
    RTCError SetRtpParameters(const RtpParameters& parameters,
                              SetParametersCallback callback);
    RtpParameters GetRtpParameters() const;

    void SetFrameEncryptor(
        scoped_refptr<FrameEncryptorInterface> frame_encryptor);

    bool SetVideoSend(const VideoOptions* options,
                      VideoSourceInterface<VideoFrame>* source);

    // note: The encoder_selector object must remain valid for the lifetime of
    // the MediaChannel, unless replaced.
    void SetEncoderSelector(
        VideoEncoderFactory::EncoderSelectorInterface* encoder_selector);

    void SetSend(bool send);

    const std::vector<uint32_t>& GetSsrcs() const;
    // Returns per ssrc VideoSenderInfos. Useful for simulcast scenario.
    std::vector<VideoSenderInfo> GetPerLayerVideoSenderInfos(bool log_stats);
    // Aggregates per ssrc VideoSenderInfos to single VideoSenderInfo for
    // legacy reasons. Used in old GetStats API and track stats.
    VideoSenderInfo GetAggregatedVideoSenderInfo(
        const std::vector<VideoSenderInfo>& infos) const;
    void FillBitrateInfo(BandwidthEstimationInfo* bwe_info);

    void SetEncoderToPacketizerFrameTransformer(
        scoped_refptr<FrameTransformerInterface> frame_transformer);
    void GenerateKeyFrame(const std::vector<std::string>& rids);

   private:
    // Parameters needed to reconstruct the underlying stream.
    // webrtc::VideoSendStream doesn't support setting a lot of options on the
    // fly, so when those need to be changed we tear down and reconstruct with
    // similar parameters depending on which options changed etc.
    struct VideoSendStreamParameters {
      VideoSendStreamParameters(
          VideoSendStream::Config config,
          const VideoOptions& options,
          int max_bitrate_bps,
          const std::optional<VideoCodecSettings>& codec_settings,
          const std::vector<VideoCodecSettings>& codec_settings_list);
      VideoSendStream::Config config;
      VideoOptions options;
      int max_bitrate_bps;
      bool conference_mode;
      std::optional<VideoCodecSettings> codec_settings;
      std::vector<VideoCodecSettings> codec_settings_list;
      // Sent resolutions + bitrates etc. by the underlying VideoSendStream,
      // typically changes when setting a new resolution or reconfiguring
      // bitrates.
      VideoEncoderConfig encoder_config;
    };

    scoped_refptr<VideoEncoderConfig::EncoderSpecificSettings>
    ConfigureVideoEncoderSettings(const Codec& codec);
    void SetCodec(const VideoCodecSettings& codec,
                  const std::vector<VideoCodecSettings>& codec_settings_list);
    void RecreateWebRtcStream();
    VideoEncoderConfig CreateVideoEncoderConfig(const Codec& codec) const;
    void ReconfigureEncoder(SetParametersCallback callback);

    // Calls Start or Stop according to whether or not `sending_` is true.
    void UpdateSendState();

    DegradationPreference GetDegradationPreference() const
        RTC_EXCLUSIVE_LOCKS_REQUIRED(&thread_checker_);

    RTC_NO_UNIQUE_ADDRESS SequenceChecker thread_checker_;
    TaskQueueBase* const worker_thread_;
    const std::vector<uint32_t> ssrcs_ RTC_GUARDED_BY(&thread_checker_);
    const std::vector<SsrcGroup> ssrc_groups_ RTC_GUARDED_BY(&thread_checker_);
    Call* const call_;
    const bool enable_cpu_overuse_detection_;
    VideoSourceInterface<VideoFrame>* source_ RTC_GUARDED_BY(&thread_checker_);

    VideoSendStream* stream_ RTC_GUARDED_BY(&thread_checker_);

    // Contains settings that are the same for all streams in the MediaChannel,
    // such as codecs, header extensions, and the global bitrate limit for the
    // entire channel.
    VideoSendStreamParameters parameters_ RTC_GUARDED_BY(&thread_checker_);
    // Contains settings that are unique for each stream, such as max_bitrate.
    // Does *not* contain codecs, however.
    // TODO(skvlad): Move ssrcs_ and ssrc_groups_ into rtp_parameters_.
    // TODO(skvlad): Combine parameters_ and rtp_parameters_ once we have only
    // one stream per MediaChannel.
    RtpParameters rtp_parameters_ RTC_GUARDED_BY(&thread_checker_);

    bool sending_ RTC_GUARDED_BY(&thread_checker_);

    // TODO(asapersson): investigate why setting
    // DegrationPreferences::MAINTAIN_RESOLUTION isn't sufficient to disable
    // downscaling everywhere in the pipeline.
    const bool disable_automatic_resize_;
  };

  void Construct(Call* call, WebRtcVideoEngine* engine);

  // Get all codecs that are compatible with the receiver.
  std::vector<VideoCodecSettings> SelectSendVideoCodecs(
      const std::vector<VideoCodecSettings>& remote_mapped_codecs) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);

  void FillSenderStats(VideoMediaSendInfo* info, bool log_stats)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);
  void FillBandwidthEstimationStats(const Call::Stats& stats,
                                    VideoMediaInfo* info)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);
  void FillSendCodecStats(VideoMediaSendInfo* video_media_info)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);

  // Accessor function for send_codec_. Introduced in order to ensure
  // that a receive channel does not touch the send codec directly.
  // Can go away once these are different classes.
  // TODO(bugs.webrtc.org/13931): Remove this function
  std::optional<VideoCodecSettings>& send_codec() { return send_codec_; }
  const std::optional<VideoCodecSettings>& send_codec() const {
    return send_codec_;
  }
  TaskQueueBase* const worker_thread_;
  ScopedTaskSafety task_safety_;
  RTC_NO_UNIQUE_ADDRESS SequenceChecker network_thread_checker_{
      SequenceChecker::kDetached};
  RTC_NO_UNIQUE_ADDRESS SequenceChecker thread_checker_;

  uint32_t rtcp_receiver_report_ssrc_ RTC_GUARDED_BY(thread_checker_);
  bool sending_ RTC_GUARDED_BY(thread_checker_);
  bool receiving_ RTC_GUARDED_BY(&thread_checker_);
  Call* const call_;

  VideoSinkInterface<VideoFrame>* default_sink_ RTC_GUARDED_BY(thread_checker_);

  // Delay for unsignaled streams, which may be set before the stream exists.
  int default_recv_base_minimum_delay_ms_ RTC_GUARDED_BY(thread_checker_) = 0;

  const MediaConfig::Video video_config_ RTC_GUARDED_BY(thread_checker_);

  // Using primary-ssrc (first ssrc) as key.
  std::map<uint32_t, WebRtcVideoSendStream*> send_streams_
      RTC_GUARDED_BY(thread_checker_);
  // When the channel and demuxer get reconfigured, there is a window of time
  // where we have to be prepared for packets arriving based on the old demuxer
  // criteria because the streams live on the worker thread and the demuxer
  // lives on the network thread. Because packets are posted from the network
  // thread to the worker thread, they can still be in-flight when streams are
  // reconfgured. This can happen when `demuxer_criteria_id_` and
  // `demuxer_criteria_completed_id_` don't match. During this time, we do not
  // want to create unsignalled receive streams and should instead drop the
  // packets. E.g:
  // * If RemoveRecvStream(old_ssrc) was recently called, there may be packets
  //   in-flight for that ssrc. This happens when a receiver becomes inactive.
  // * If we go from one to many m= sections, the demuxer may change from
  //   forwarding all packets to only forwarding the configured ssrcs, so there
  //   is a risk of receiving ssrcs for other, recently added m= sections.
  uint32_t demuxer_criteria_id_ RTC_GUARDED_BY(thread_checker_) = 0;
  uint32_t demuxer_criteria_completed_id_ RTC_GUARDED_BY(thread_checker_) = 0;
  std::optional<int64_t> last_unsignalled_ssrc_creation_time_ms_
      RTC_GUARDED_BY(thread_checker_);
  std::set<uint32_t> send_ssrcs_ RTC_GUARDED_BY(thread_checker_);
  std::set<uint32_t> receive_ssrcs_ RTC_GUARDED_BY(thread_checker_);

  std::optional<VideoCodecSettings> send_codec_ RTC_GUARDED_BY(thread_checker_);
  std::vector<VideoCodecSettings> negotiated_codecs_
      RTC_GUARDED_BY(thread_checker_);
  std::vector<VideoCodecSettings> send_codecs_ RTC_GUARDED_BY(thread_checker_);

  std::vector<RtpExtension> send_rtp_extensions_
      RTC_GUARDED_BY(thread_checker_);

  VideoEncoderFactory* const encoder_factory_ RTC_GUARDED_BY(thread_checker_);
  VideoDecoderFactory* const decoder_factory_ RTC_GUARDED_BY(thread_checker_);
  VideoBitrateAllocatorFactory* const bitrate_allocator_factory_
      RTC_GUARDED_BY(thread_checker_);
  std::vector<VideoCodecSettings> recv_codecs_ RTC_GUARDED_BY(thread_checker_);
  RtpHeaderExtensionMap recv_rtp_extension_map_ RTC_GUARDED_BY(thread_checker_);
  std::vector<RtpExtension> recv_rtp_extensions_
      RTC_GUARDED_BY(thread_checker_);
  // See reason for keeping track of the FlexFEC payload type separately in
  // comment in WebRtcVideoChannel::ChangedReceiverParameters.
  int recv_flexfec_payload_type_ RTC_GUARDED_BY(thread_checker_);
  BitrateConstraints bitrate_config_ RTC_GUARDED_BY(thread_checker_);
  // TODO(deadbeef): Don't duplicate information between
  // send_params/recv_params, rtp_extensions, options, etc.
  VideoSenderParameters send_params_ RTC_GUARDED_BY(thread_checker_);
  VideoOptions default_send_options_ RTC_GUARDED_BY(thread_checker_);
  VideoReceiverParameters recv_params_ RTC_GUARDED_BY(thread_checker_);
  int64_t last_send_stats_log_ms_ RTC_GUARDED_BY(thread_checker_);
  int64_t last_receive_stats_log_ms_ RTC_GUARDED_BY(thread_checker_);
  const bool discard_unknown_ssrc_packets_ RTC_GUARDED_BY(thread_checker_);
  // This is a stream param that comes from the remote description, but wasn't
  // signaled with any a=ssrc lines. It holds information that was signaled
  // before the unsignaled receive stream is created when the first packet is
  // received.
  StreamParams unsignaled_stream_params_ RTC_GUARDED_BY(thread_checker_);
  // Per peer connection crypto options that last for the lifetime of the peer
  // connection.
  const CryptoOptions crypto_options_ RTC_GUARDED_BY(thread_checker_);

  // Optional frame transformer set on unsignaled streams.
  scoped_refptr<FrameTransformerInterface> unsignaled_frame_transformer_
      RTC_GUARDED_BY(thread_checker_);

  // RTP parameters that need to be set when creating a video receive stream.
  // Only used in Receiver mode - in Both mode, it reads those things from the
  // codec.
  VideoReceiveStreamInterface::Config::Rtp rtp_config_;

  // Callback invoked whenever the send codec changes.
  // TODO(bugs.webrtc.org/13931): Remove again when coupling isn't needed.
  absl::AnyInvocable<void()> send_codec_changed_callback_;
  // Callback invoked whenever the list of SSRCs changes.
  absl::AnyInvocable<void(const std::set<uint32_t>&)>
      ssrc_list_changed_callback_;
};

class WebRtcVideoReceiveChannel : public MediaChannelUtil,
                                  public VideoMediaReceiveChannelInterface {
 public:
  WebRtcVideoReceiveChannel(Call* call,
                            const MediaConfig& config,
                            const VideoOptions& options,
                            const CryptoOptions& crypto_options,
                            VideoDecoderFactory* decoder_factory);
  ~WebRtcVideoReceiveChannel() override;

 public:
  MediaType media_type() const override { return MediaType::VIDEO; }
  VideoMediaReceiveChannelInterface* AsVideoReceiveChannel() override {
    return this;
  }
  VoiceMediaReceiveChannelInterface* AsVoiceReceiveChannel() override {
    RTC_CHECK_NOTREACHED();
    return nullptr;
  }

  // Common functions between sender and receiver
  void SetInterface(MediaChannelNetworkInterface* iface) override;
  // VideoMediaReceiveChannelInterface implementation
  bool SetReceiverParameters(const VideoReceiverParameters& params) override;
  RtpParameters GetRtpReceiverParameters(uint32_t ssrc) const override;
  RtpParameters GetDefaultRtpReceiveParameters() const override;
  void SetReceive(bool receive) override;
  bool AddRecvStream(const StreamParams& sp) override;
  bool AddDefaultRecvStreamForTesting(const StreamParams& sp) override {
    // Invokes private AddRecvStream variant function
    return AddRecvStream(sp, true);
  }
  bool RemoveRecvStream(uint32_t ssrc) override;
  void ResetUnsignaledRecvStream() override;
  std::optional<uint32_t> GetUnsignaledSsrc() const override;
  void OnDemuxerCriteriaUpdatePending() override;
  void OnDemuxerCriteriaUpdateComplete() override;
  bool SetSink(uint32_t ssrc, VideoSinkInterface<VideoFrame>* sink) override;
  void SetDefaultSink(VideoSinkInterface<VideoFrame>* sink) override;
  bool GetStats(VideoMediaReceiveInfo* info) override;
  void OnPacketReceived(const RtpPacketReceived& packet) override;
  bool SetBaseMinimumPlayoutDelayMs(uint32_t ssrc, int delay_ms) override;

  std::optional<int> GetBaseMinimumPlayoutDelayMs(uint32_t ssrc) const override;

  // Choose one of the available SSRCs (or default if none) as the current
  // receiver report SSRC.
  void ChooseReceiverReportSsrc(const std::set<uint32_t>& choices) override;

  // E2E Encrypted Video Frame API
  // Set a frame decryptor to a particular ssrc that will intercept all
  // incoming video frames and attempt to decrypt them before forwarding the
  // result.
  void SetFrameDecryptor(
      uint32_t ssrc,
      scoped_refptr<FrameDecryptorInterface> frame_decryptor) override;
  void SetRecordableEncodedFrameCallback(
      uint32_t ssrc,
      std::function<void(const RecordableEncodedFrame&)> callback) override;
  void ClearRecordableEncodedFrameCallback(uint32_t ssrc) override;
  void RequestRecvKeyFrame(uint32_t ssrc) override;
  void SetDepacketizerToDecoderFrameTransformer(
      uint32_t ssrc,
      scoped_refptr<FrameTransformerInterface> frame_transformer) override;
  std::vector<RtpSource> GetSources(uint32_t ssrc) const override;

  void SetReceiverFeedbackParameters(bool lntf_enabled,
                                     bool nack_enabled,
                                     RtcpMode rtcp_mode,
                                     std::optional<int> rtx_time) override;
  void StartReceive(uint32_t ssrc) override;
  void StopReceive(uint32_t ssrc) override;
 private:
  class WebRtcVideoReceiveStream;
  struct ChangedReceiverParameters {
    // These optionals are unset if not changed.
    std::optional<std::vector<VideoCodecSettings>> codec_settings;
    std::optional<std::vector<RtpExtension>> rtp_header_extensions;
    // Keep track of the FlexFEC payload type separately from `codec_settings`.
    // This allows us to recreate the FlexfecReceiveStream separately from the
    // VideoReceiveStreamInterface when the FlexFEC payload type is changed.
    std::optional<int> flexfec_payload_type;
  };

  // Finds VideoReceiveStreamInterface corresponding to ssrc. Aware of
  // unsignalled ssrc handling.
  WebRtcVideoReceiveStream* FindReceiveStream(uint32_t ssrc)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);

  void ProcessReceivedPacket(RtpPacketReceived packet)
      RTC_RUN_ON(thread_checker_);

  // Expected to be invoked once per packet that belongs to this channel that
  // can not be demuxed.
  // Returns true if a new default stream has been created.
  bool MaybeCreateDefaultReceiveStream(const RtpPacketReceived& parsed_packet)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);
  void ReCreateDefaultReceiveStream(uint32_t ssrc,
                                    std::optional<uint32_t> rtx_ssrc);
  // Add a receive stream. Used for testing.
  bool AddRecvStream(const StreamParams& sp, bool default_stream);

  void ConfigureReceiverRtp(VideoReceiveStreamInterface::Config* config,
                            FlexfecReceiveStream::Config* flexfec_config,
                            const StreamParams& sp) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);
  bool ValidateReceiveSsrcAvailability(const StreamParams& sp) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);
  void DeleteReceiveStream(WebRtcVideoReceiveStream* stream)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);

  // Called when the local ssrc changes. Sets `rtcp_receiver_report_ssrc_` and
  // updates the receive streams.
  void SetReceiverReportSsrc(uint32_t ssrc) RTC_RUN_ON(&thread_checker_);

  // Wrapper for the receiver part, contains configs etc. that are needed to
  // reconstruct the underlying VideoReceiveStreamInterface.
  class WebRtcVideoReceiveStream : public VideoSinkInterface<VideoFrame> {
   public:
    WebRtcVideoReceiveStream(
        Call* call,
        const StreamParams& sp,
        VideoReceiveStreamInterface::Config config,
        bool default_stream,
        const std::vector<VideoCodecSettings>& recv_codecs,
        const FlexfecReceiveStream::Config& flexfec_config);
    ~WebRtcVideoReceiveStream();

    VideoReceiveStreamInterface& stream();
    // Return value may be nullptr.
    FlexfecReceiveStream* flexfec_stream();

    const std::vector<uint32_t>& GetSsrcs() const;

    std::vector<RtpSource> GetSources();

    // Does not return codecs, nor header extensions,  they are filled by the
    // owning WebRtcVideoChannel.
    RtpParameters GetRtpParameters() const;

    // TODO(deadbeef): Move these feedback parameters into the recv parameters.
    void SetFeedbackParameters(bool lntf_enabled,
                               bool nack_enabled,
                               RtcpMode rtcp_mode,
                               std::optional<int> rtx_time);
    void SetReceiverParameters(const ChangedReceiverParameters& recv_params);

    void OnFrame(const VideoFrame& frame) override;
    bool IsDefaultStream() const;

    void SetFrameDecryptor(
        scoped_refptr<FrameDecryptorInterface> frame_decryptor);

    bool SetBaseMinimumPlayoutDelayMs(int delay_ms);

    int GetBaseMinimumPlayoutDelayMs() const;

    void SetSink(VideoSinkInterface<VideoFrame>* sink);

    VideoReceiverInfo GetVideoReceiverInfo(bool log_stats);

    void SetRecordableEncodedFrameCallback(
        std::function<void(const RecordableEncodedFrame&)> callback);
    void ClearRecordableEncodedFrameCallback();
    void GenerateKeyFrame();

    void SetDepacketizerToDecoderFrameTransformer(
        scoped_refptr<FrameTransformerInterface> frame_transformer);

    void SetLocalSsrc(uint32_t local_ssrc);
    void UpdateRtxSsrc(uint32_t ssrc);
    void StartReceiveStream();
    void StopReceiveStream();

   private:
    // Attempts to reconfigure an already existing `flexfec_stream_`, create
    // one if the configuration is now complete or remove a flexfec stream
    // when disabled.
    void SetFlexFecPayload(int payload_type);

    void RecreateReceiveStream();
    void CreateReceiveStream();

    // Applies a new receive codecs configration to `config_`. Returns true
    // if the internal stream needs to be reconstructed, or false if no changes
    // were applied.
    bool ReconfigureCodecs(const std::vector<VideoCodecSettings>& recv_codecs);

    Call* const call_;
    const StreamParams stream_params_;

    // Both `stream_` and `flexfec_stream_` are managed by `this`. They are
    // destroyed by calling call_->DestroyVideoReceiveStream and
    // call_->DestroyFlexfecReceiveStream, respectively.
    VideoReceiveStreamInterface* stream_;
    const bool default_stream_;
    VideoReceiveStreamInterface::Config config_;
    FlexfecReceiveStream::Config flexfec_config_;
    FlexfecReceiveStream* flexfec_stream_;

    Mutex sink_lock_;
    VideoSinkInterface<VideoFrame>* sink_ RTC_GUARDED_BY(sink_lock_);
    int64_t first_frame_timestamp_ RTC_GUARDED_BY(sink_lock_);
    // Start NTP time is estimated as current remote NTP time (estimated from
    // RTCP) minus the elapsed time, as soon as remote NTP time is available.
    int64_t estimated_remote_start_ntp_time_ms_ RTC_GUARDED_BY(sink_lock_);

    RTC_NO_UNIQUE_ADDRESS SequenceChecker thread_checker_;
    bool receiving_ RTC_GUARDED_BY(&thread_checker_);
  };
  bool GetChangedReceiverParameters(const VideoReceiverParameters& params,
                                    ChangedReceiverParameters* changed_params)
      const RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);

  std::map<uint32_t, WebRtcVideoReceiveStream*> receive_streams_
      RTC_GUARDED_BY(thread_checker_);
  void FillReceiverStats(VideoMediaReceiveInfo* info, bool log_stats)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);
  void FillReceiveCodecStats(VideoMediaReceiveInfo* video_media_info)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(thread_checker_);

  StreamParams unsignaled_stream_params() {
    RTC_DCHECK_RUN_ON(&thread_checker_);
    return unsignaled_stream_params_;
  }
  // Variables.
  TaskQueueBase* const worker_thread_;
  ScopedTaskSafety task_safety_;
  RTC_NO_UNIQUE_ADDRESS SequenceChecker network_thread_checker_{
      SequenceChecker::kDetached};
  RTC_NO_UNIQUE_ADDRESS SequenceChecker thread_checker_;

  uint32_t rtcp_receiver_report_ssrc_ RTC_GUARDED_BY(thread_checker_);
  bool receiving_ RTC_GUARDED_BY(&thread_checker_);
  Call* const call_;

  VideoSinkInterface<VideoFrame>* default_sink_ RTC_GUARDED_BY(thread_checker_);

  // Delay for unsignaled streams, which may be set before the stream exists.
  int default_recv_base_minimum_delay_ms_ RTC_GUARDED_BY(thread_checker_) = 0;

  const MediaConfig::Video video_config_ RTC_GUARDED_BY(thread_checker_);

  // When the channel and demuxer get reconfigured, there is a window of time
  // where we have to be prepared for packets arriving based on the old demuxer
  // criteria because the streams live on the worker thread and the demuxer
  // lives on the network thread. Because packets are posted from the network
  // thread to the worker thread, they can still be in-flight when streams are
  // reconfgured. This can happen when `demuxer_criteria_id_` and
  // `demuxer_criteria_completed_id_` don't match. During this time, we do not
  // want to create unsignalled receive streams and should instead drop the
  // packets. E.g:
  // * If RemoveRecvStream(old_ssrc) was recently called, there may be packets
  //   in-flight for that ssrc. This happens when a receiver becomes inactive.
  // * If we go from one to many m= sections, the demuxer may change from
  //   forwarding all packets to only forwarding the configured ssrcs, so there
  //   is a risk of receiving ssrcs for other, recently added m= sections.
  uint32_t demuxer_criteria_id_ RTC_GUARDED_BY(thread_checker_) = 0;
  uint32_t demuxer_criteria_completed_id_ RTC_GUARDED_BY(thread_checker_) = 0;
  std::optional<int64_t> last_unsignalled_ssrc_creation_time_ms_
      RTC_GUARDED_BY(thread_checker_);
  std::set<uint32_t> send_ssrcs_ RTC_GUARDED_BY(thread_checker_);
  std::set<uint32_t> receive_ssrcs_ RTC_GUARDED_BY(thread_checker_);

  std::optional<VideoCodecSettings> send_codec_ RTC_GUARDED_BY(thread_checker_);
  std::vector<VideoCodecSettings> negotiated_codecs_
      RTC_GUARDED_BY(thread_checker_);

  std::vector<RtpExtension> send_rtp_extensions_
      RTC_GUARDED_BY(thread_checker_);

  VideoDecoderFactory* const decoder_factory_ RTC_GUARDED_BY(thread_checker_);
  std::vector<VideoCodecSettings> recv_codecs_ RTC_GUARDED_BY(thread_checker_);
  RtpHeaderExtensionMap recv_rtp_extension_map_ RTC_GUARDED_BY(thread_checker_);
  std::vector<RtpExtension> recv_rtp_extensions_
      RTC_GUARDED_BY(thread_checker_);
  // See reason for keeping track of the FlexFEC payload type separately in
  // comment in WebRtcVideoChannel::ChangedReceiverParameters.
  int recv_flexfec_payload_type_ RTC_GUARDED_BY(thread_checker_);
  BitrateConstraints bitrate_config_ RTC_GUARDED_BY(thread_checker_);
  // TODO(deadbeef): Don't duplicate information between
  // send_params/recv_params, rtp_extensions, options, etc.
  VideoSenderParameters send_params_ RTC_GUARDED_BY(thread_checker_);
  VideoOptions default_send_options_ RTC_GUARDED_BY(thread_checker_);
  VideoReceiverParameters recv_params_ RTC_GUARDED_BY(thread_checker_);
  int64_t last_receive_stats_log_ms_ RTC_GUARDED_BY(thread_checker_);
  const bool discard_unknown_ssrc_packets_ RTC_GUARDED_BY(thread_checker_);
  // This is a stream param that comes from the remote description, but wasn't
  // signaled with any a=ssrc lines. It holds information that was signaled
  // before the unsignaled receive stream is created when the first packet is
  // received.
  StreamParams unsignaled_stream_params_ RTC_GUARDED_BY(thread_checker_);
  // Per peer connection crypto options that last for the lifetime of the peer
  // connection.
  const CryptoOptions crypto_options_ RTC_GUARDED_BY(thread_checker_);

  // Optional frame transformer set on unsignaled streams.
  scoped_refptr<FrameTransformerInterface> unsignaled_frame_transformer_
      RTC_GUARDED_BY(thread_checker_);

  // RTP parameters that need to be set when creating a video receive stream.
  // Only used in Receiver mode - in Both mode, it reads those things from the
  // codec.
  VideoReceiveStreamInterface::Config::Rtp rtp_config_;

  // Callback invoked whenever the send codec changes.
  // TODO(bugs.webrtc.org/13931): Remove again when coupling isn't needed.
  absl::AnyInvocable<void()> send_codec_changed_callback_;
  // Callback invoked whenever the list of SSRCs changes.
  absl::AnyInvocable<void(const std::set<uint32_t>&)>
      ssrc_list_changed_callback_;

  const int receive_buffer_size_;
};

// Keeping the old name "WebRtcVideoChannel" around because some external
// customers are using webrtc::WebRtcVideoChannel::AdaptReason
// TODO(bugs.webrtc.org/15216): Move this enum to an interface class and
// delete this workaround.
class WebRtcVideoChannel : public WebRtcVideoSendChannel {
 public:
  // Make all the values of AdaptReason available as
  // WebRtcVideoChannel::ADAPT_xxx.
  using WebRtcVideoSendChannel::AdaptReason;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::MergeInfoAboutOutboundRtpSubstreamsForTesting;
using ::webrtc::VideoCodecSettings;
using ::webrtc::WebRtcVideoChannel;
using ::webrtc::WebRtcVideoEngine;
using ::webrtc::WebRtcVideoReceiveChannel;
using ::webrtc::WebRtcVideoSendChannel;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_ENGINE_WEBRTC_VIDEO_ENGINE_H_
