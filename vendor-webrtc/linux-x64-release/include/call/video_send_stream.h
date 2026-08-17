/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_VIDEO_SEND_STREAM_H_
#define CALL_VIDEO_SEND_STREAM_H_

#include <stdint.h>

#include <map>
#include <optional>
#include <string>
#include <vector>

#include "api/adaptation/resource.h"
#include "api/call/transport.h"
#include "api/crypto/crypto_options.h"
#include "api/frame_transformer_interface.h"
#include "api/rtp_parameters.h"
#include "api/rtp_sender_interface.h"
#include "api/scoped_refptr.h"
#include "api/units/data_rate.h"
#include "api/video/video_content_type.h"
#include "api/video/video_frame.h"
#include "api/video/video_source_interface.h"
#include "api/video/video_stream_encoder_settings.h"
#include "api/video_codecs/scalability_mode.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "call/rtp_config.h"
#include "common_video/frame_counts.h"
#include "common_video/include/quality_limitation_reason.h"
#include "modules/rtp_rtcp/include/report_block_data.h"
#include "modules/rtp_rtcp/include/rtcp_statistics.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "rtc_base/checks.h"
#include "video/config/video_encoder_config.h"

namespace webrtc {

class FrameEncryptorInterface;

class VideoSendStream {
 public:
  // Multiple StreamStats objects are present if simulcast is used (multiple
  // kMedia streams) or if RTX or FlexFEC is negotiated. Multiple SVC layers, on
  // the other hand, does not cause additional StreamStats.
  struct StreamStats {
    enum class StreamType {
      // A media stream is an RTP stream for audio or video. Retransmissions and
      // FEC is either sent over the same SSRC or negotiated to be sent over
      // separate SSRCs, in which case separate StreamStats objects exist with
      // references to this media stream's SSRC.
      kMedia,
      // RTX streams are streams dedicated to retransmissions. They have a
      // dependency on a single kMedia stream: `referenced_media_ssrc`.
      kRtx,
      // FlexFEC streams are streams dedicated to FlexFEC. They have a
      // dependency on a single kMedia stream: `referenced_media_ssrc`.
      kFlexfec,
    };

    StreamStats();
    ~StreamStats();

    std::string ToString() const;

    StreamType type = StreamType::kMedia;
    // If `type` is kRtx or kFlexfec this value is present. The referenced SSRC
    // is the kMedia stream that this stream is performing retransmissions or
    // FEC for. If `type` is kMedia, this value is null.
    std::optional<uint32_t> referenced_media_ssrc;
    FrameCounts frame_counts;
    int width = 0;
    int height = 0;
    // TODO(holmer): Move bitrate_bps out to the webrtc::Call layer.
    int total_bitrate_bps = 0;
    int retransmit_bitrate_bps = 0;
    // `avg_delay_ms` and `max_delay_ms` are only used in tests. Consider
    // deleting.
    int avg_delay_ms = 0;
    int max_delay_ms = 0;
    StreamDataCounters rtp_stats;
    RtcpPacketTypeCounter rtcp_packet_type_counts;
    // A snapshot of the most recent Report Block with additional data of
    // interest to statistics. Used to implement RTCRemoteInboundRtpStreamStats.
    std::optional<ReportBlockData> report_block_data;
    double encode_frame_rate = 0.0;
    int frames_encoded = 0;
    std::optional<uint64_t> qp_sum;
    uint64_t total_encode_time_ms = 0;
    uint64_t total_encoded_bytes_target = 0;
    uint32_t huge_frames_sent = 0;
    std::optional<ScalabilityMode> scalability_mode;
    // The target bitrate is what we tell the encoder to produce. What the
    // encoder actually produces is the sum of encoded bytes.
    std::optional<DataRate> target_bitrate;
  };

  struct Stats {
    Stats();
    ~Stats();
    std::string ToString(int64_t time_ms) const;
    std::optional<std::string> encoder_implementation_name;
    double input_frame_rate = 0;
    int encode_frame_rate = 0;
    int avg_encode_time_ms = 0;
    int encode_usage_percent = 0;
    uint32_t frames_encoded = 0;
    // https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-totalencodetime
    uint64_t total_encode_time_ms = 0;
    // https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-totalencodedbytestarget
    uint64_t total_encoded_bytes_target = 0;
    uint32_t frames = 0;
    uint32_t frames_dropped_by_capturer = 0;
    uint32_t frames_dropped_by_bad_timestamp = 0;
    uint32_t frames_dropped_by_encoder_queue = 0;
    uint32_t frames_dropped_by_rate_limiter = 0;
    uint32_t frames_dropped_by_congestion_window = 0;
    uint32_t frames_dropped_by_encoder = 0;
    // Metric only used by legacy getStats()'s BWE.
    // - Similar to `StreamStats::target_bitrate` except this is for the whole
    //   stream as opposed to being per substream (per SSRC).
    // - Unlike what you would expect, it is not equal to the sum of all
    //   substream targets and may sometimes over-report e.g. webrtc:392424845.
    int target_media_bitrate_bps = 0;
    // Bitrate the encoder is actually producing.
    int media_bitrate_bps = 0;
    bool suspended = false;
    bool bw_limited_resolution = false;
    bool cpu_limited_resolution = false;
    bool bw_limited_framerate = false;
    bool cpu_limited_framerate = false;
    // https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-qualitylimitationreason
    QualityLimitationReason quality_limitation_reason =
        QualityLimitationReason::kNone;
    // https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-qualitylimitationdurations
    std::map<QualityLimitationReason, int64_t> quality_limitation_durations_ms;
    // https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-qualitylimitationresolutionchanges
    uint32_t quality_limitation_resolution_changes = 0;
    // Total number of times resolution as been requested to be changed due to
    // CPU/quality adaptation.
    int number_of_cpu_adapt_changes = 0;
    int number_of_quality_adapt_changes = 0;
    bool has_entered_low_resolution = false;
    std::map<uint32_t, StreamStats> substreams;
    webrtc::VideoContentType content_type =
        webrtc::VideoContentType::UNSPECIFIED;
    uint32_t frames_sent = 0;
    uint32_t huge_frames_sent = 0;
    std::optional<bool> power_efficient_encoder;
  };

  struct Config {
   public:
    Config() = delete;
    Config(Config&&);
    explicit Config(Transport* send_transport);

    Config& operator=(Config&&);
    Config& operator=(const Config&) = delete;

    ~Config();

    // Mostly used by tests.  Avoid creating copies if you can.
    Config Copy() const { return Config(*this); }

    std::string ToString() const;

    RtpConfig rtp;

    VideoStreamEncoderSettings encoder_settings;

    // Time interval between RTCP report for video
    int rtcp_report_interval_ms = 1000;

    // Transport for outgoing packets.
    Transport* send_transport = nullptr;

    // Expected delay needed by the renderer, i.e. the frame will be delivered
    // this many milliseconds, if possible, earlier than expected render time.
    // Only valid if `local_renderer` is set.
    int render_delay_ms = 0;

    // Target delay in milliseconds. A positive value indicates this stream is
    // used for streaming instead of a real-time call.
    int target_delay_ms = 0;

    // True if the stream should be suspended when the available bitrate fall
    // below the minimum configured bitrate. If this variable is false, the
    // stream may send at a rate higher than the estimated available bitrate.
    bool suspend_below_min_bitrate = false;

    // Enables periodic bandwidth probing in application-limited region.
    bool periodic_alr_bandwidth_probing = false;

    // An optional custom frame encryptor that allows the entire frame to be
    // encrypted in whatever way the caller chooses. This is not required by
    // default.
    scoped_refptr<webrtc::FrameEncryptorInterface> frame_encryptor;

    // An optional encoder selector provided by the user.
    // Overrides VideoEncoderFactory::GetEncoderSelector().
    // Owned by RtpSenderBase.
    VideoEncoderFactory::EncoderSelectorInterface* encoder_selector = nullptr;

    // Per PeerConnection cryptography options.
    CryptoOptions crypto_options;

    scoped_refptr<webrtc::FrameTransformerInterface> frame_transformer;

   private:
    // Access to the copy constructor is private to force use of the Copy()
    // method for those exceptional cases where we do use it.
    Config(const Config&);
  };

  // Starts stream activity.
  // When a stream is active, it can receive, process and deliver packets.
  virtual void Start() = 0;

  // Stops stream activity.
  // When a stream is stopped, it can't receive, process or deliver packets.
  virtual void Stop() = 0;

  // Accessor for determining if the stream is active. This is an inexpensive
  // call that must be made on the same thread as `Start()` and `Stop()` methods
  // are called on and will return `true` iff activity has been started
  // via `Start()`.
  virtual bool started() = 0;

  // If the resource is overusing, the VideoSendStream will try to reduce
  // resolution or frame rate until no resource is overusing.
  // TODO(https://crbug.com/webrtc/11565): When the ResourceAdaptationProcessor
  // is moved to Call this method could be deleted altogether in favor of
  // Call-level APIs only.
  virtual void AddAdaptationResource(scoped_refptr<Resource> resource) = 0;
  virtual std::vector<scoped_refptr<Resource>> GetAdaptationResources() = 0;

  virtual void SetSource(
      VideoSourceInterface<webrtc::VideoFrame>* source,
      const DegradationPreference& degradation_preference) = 0;

  // Set which streams to send. Must have at least as many SSRCs as configured
  // in the config. Encoder settings are passed on to the encoder instance along
  // with the VideoStream settings.
  virtual void ReconfigureVideoEncoder(VideoEncoderConfig config) = 0;

  virtual void ReconfigureVideoEncoder(VideoEncoderConfig config,
                                       SetParametersCallback callback) = 0;

  virtual Stats GetStats() = 0;

  // TODO: webrtc:40644448 - Make this pure virtual.
  virtual void SetStats(const Stats& stats) { RTC_CHECK_NOTREACHED(); }

  virtual void GenerateKeyFrame(const std::vector<std::string>& rids) = 0;

 protected:
  virtual ~VideoSendStream() {}
};

}  // namespace webrtc

#endif  // CALL_VIDEO_SEND_STREAM_H_
