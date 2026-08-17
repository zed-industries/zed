/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_VIDEO_RECEIVE_STREAM_H_
#define CALL_VIDEO_RECEIVE_STREAM_H_

#include <cstdint>
#include <functional>
#include <limits>
#include <map>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "api/call/transport.h"
#include "api/crypto/crypto_options.h"
#include "api/crypto/frame_decryptor_interface.h"
#include "api/frame_transformer_interface.h"
#include "api/rtp_headers.h"
#include "api/scoped_refptr.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "api/video/recordable_encoded_frame.h"
#include "api/video/video_content_type.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video/video_timing.h"
#include "api/video_codecs/sdp_video_format.h"
#include "call/receive_stream.h"
#include "call/rtp_config.h"
#include "common_video/frame_counts.h"
#include "modules/rtp_rtcp/include/rtcp_statistics.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"

namespace webrtc {

class RtpPacketSinkInterface;
class VideoDecoderFactory;

class VideoReceiveStreamInterface : public MediaReceiveStreamInterface {
 public:
  // Class for handling moving in/out recording state.
  struct RecordingState {
    RecordingState() = default;
    explicit RecordingState(
        std::function<void(const RecordableEncodedFrame&)> callback)
        : callback(std::move(callback)) {}

    // Callback stored from the VideoReceiveStreamInterface. The
    // VideoReceiveStreamInterface client should not interpret the attribute.
    std::function<void(const RecordableEncodedFrame&)> callback;
    // Memento of when a keyframe request was last sent. The
    // VideoReceiveStreamInterface client should not interpret the attribute.
    std::optional<int64_t> last_keyframe_request_ms;
  };

  // TODO(mflodman) Move all these settings to VideoDecoder and move the
  // declaration to common_types.h.
  struct Decoder {
    Decoder(SdpVideoFormat video_format, int payload_type);
    Decoder();
    Decoder(const Decoder&);
    ~Decoder();

    bool operator==(const Decoder& other) const;

    std::string ToString() const;

    SdpVideoFormat video_format;

    // Received RTP packets with this payload type will be sent to this decoder
    // instance.
    int payload_type = 0;
  };

  struct Stats {
    Stats();
    ~Stats();
    std::string ToString(int64_t time_ms) const;

    int network_frame_rate = 0;
    int decode_frame_rate = 0;
    int render_frame_rate = 0;
    uint32_t frames_rendered = 0;

    // Decoder stats.
    std::optional<std::string> decoder_implementation_name;
    std::optional<bool> power_efficient_decoder;
    FrameCounts frame_counts;
    int decode_ms = 0;
    int max_decode_ms = 0;
    int current_delay_ms = 0;
    int target_delay_ms = 0;
    int jitter_buffer_ms = 0;
    // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-jitterbufferdelay
    TimeDelta jitter_buffer_delay = TimeDelta::Zero();
    // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-jitterbuffertargetdelay
    TimeDelta jitter_buffer_target_delay = TimeDelta::Zero();
    // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-jitterbufferemittedcount
    uint64_t jitter_buffer_emitted_count = 0;
    // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-jitterbufferminimumdelay
    TimeDelta jitter_buffer_minimum_delay = TimeDelta::Zero();
    int min_playout_delay_ms = 0;
    int render_delay_ms = 10;
    int64_t interframe_delay_max_ms = -1;
    // Frames dropped due to decoding failures or if the system is too slow.
    // https://www.w3.org/TR/webrtc-stats/#dom-rtcvideoreceiverstats-framesdropped
    uint32_t frames_dropped = 0;
    uint32_t frames_decoded = 0;
    // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-totaldecodetime
    TimeDelta total_decode_time = TimeDelta::Zero();
    // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-totalprocessingdelay
    TimeDelta total_processing_delay = TimeDelta::Zero();

    // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-totalassemblytime
    TimeDelta total_assembly_time = TimeDelta::Zero();
    uint32_t frames_assembled_from_multiple_packets = 0;

    // Total inter frame delay in seconds.
    // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-totalinterframedelay
    double total_inter_frame_delay = 0;
    // Total squared inter frame delay in seconds^2.
    // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-totalsqauredinterframedelay
    double total_squared_inter_frame_delay = 0;
    int64_t first_frame_received_to_decoded_ms = -1;
    std::optional<uint64_t> qp_sum;

    // Corruption score, indicating the probability of corruption. Its value is
    // between 0 and 1, where 0 means no corruption and 1 means that the
    // compressed frame is corrupted.
    // However, note that the corruption score may not accurately reflect
    // corruption. E.g. even if the corruption score is 0, the compressed frame
    // may still be corrupted and vice versa.
    std::optional<double> corruption_score_sum;
    std::optional<double> corruption_score_squared_sum;
    // Number of frames the `corruption_score` was calculated on. This is
    // usually not the same as `frames_decoded`.
    uint32_t corruption_score_count = 0;

    int current_payload_type = -1;

    int total_bitrate_bps = 0;

    int width = 0;
    int height = 0;

    uint32_t freeze_count = 0;
    uint32_t pause_count = 0;
    uint32_t total_freezes_duration_ms = 0;
    uint32_t total_pauses_duration_ms = 0;

    VideoContentType content_type = VideoContentType::UNSPECIFIED;

    // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-estimatedplayouttimestamp
    std::optional<int64_t> estimated_playout_ntp_timestamp_ms;
    int sync_offset_ms = std::numeric_limits<int>::max();

    uint32_t ssrc = 0;
    std::string c_name;
    RtpReceiveStats rtp_stats;
    RtcpPacketTypeCounter rtcp_packet_type_counts;
    std::optional<RtpReceiveStats> rtx_rtp_stats;

    // Timing frame info: all important timestamps for a full lifetime of a
    // single 'timing frame'.
    std::optional<webrtc::TimingFrameInfo> timing_frame_info;

    // Remote outbound stats derived by the received RTCP sender reports.
    // https://w3c.github.io/webrtc-stats/#remoteoutboundrtpstats-dict*
    std::optional<Timestamp> last_sender_report_timestamp;
    // TODO: bugs.webrtc.org/370535296 - Remove the utc timestamp when linked
    // issue is fixed.
    std::optional<Timestamp> last_sender_report_utc_timestamp;
    std::optional<Timestamp> last_sender_report_remote_utc_timestamp;
    uint32_t sender_reports_packets_sent = 0;
    uint64_t sender_reports_bytes_sent = 0;
    uint64_t sender_reports_reports_count = 0;
  };

  struct Config {
   private:
    // Access to the copy constructor is private to force use of the Copy()
    // method for those exceptional cases where we do use it.
    Config(const Config&);

   public:
    Config() = delete;
    Config(Config&&);
    Config(Transport* rtcp_send_transport,
           VideoDecoderFactory* decoder_factory = nullptr);
    Config& operator=(Config&&);
    Config& operator=(const Config&) = delete;
    ~Config();

    // Mostly used by tests.  Avoid creating copies if you can.
    Config Copy() const { return Config(*this); }

    std::string ToString() const;

    // Decoders for every payload that we can receive.
    std::vector<Decoder> decoders;

    // Ownership stays with WebrtcVideoEngine (delegated from PeerConnection).
    VideoDecoderFactory* decoder_factory = nullptr;

    // Receive-stream specific RTP settings.
    struct Rtp : public ReceiveStreamRtpConfig {
      Rtp();
      Rtp(const Rtp&);
      ~Rtp();
      std::string ToString() const;

      // See NackConfig for description.
      NackConfig nack;

      // See RtcpMode for description.
      RtcpMode rtcp_mode = RtcpMode::kCompound;

      // Extended RTCP settings.
      struct RtcpXr {
        // True if RTCP Receiver Reference Time Report Block extension
        // (RFC 3611) should be enabled.
        bool receiver_reference_time_report = false;
      } rtcp_xr;

      // How to request keyframes from a remote sender. Applies only if lntf is
      // disabled.
      KeyFrameReqMethod keyframe_method = KeyFrameReqMethod::kPliRtcp;

      // See LntfConfig for description.
      LntfConfig lntf;

      // Payload types for ULPFEC and RED, respectively.
      int ulpfec_payload_type = -1;
      int red_payload_type = -1;

      // SSRC for retransmissions.
      uint32_t rtx_ssrc = 0;

      // Set if the stream is protected using FlexFEC.
      bool protected_by_flexfec = false;

      // Optional callback sink to support additional packet handlers such as
      // FlexFec.
      RtpPacketSinkInterface* packet_sink_ = nullptr;

      // Map from rtx payload type -> media payload type.
      // For RTX to be enabled, both an SSRC and this mapping are needed.
      std::map<int, int> rtx_associated_payload_types;

      // Payload types that should be depacketized using raw depacketizer
      // (payload header will not be parsed and must not be present, additional
      // meta data is expected to be present in generic frame descriptor
      // RTP header extension).
      std::set<int> raw_payload_types;
    } rtp;

    // Transport for outgoing packets (RTCP).
    Transport* rtcp_send_transport = nullptr;

    // Must always be set.
    VideoSinkInterface<VideoFrame>* renderer = nullptr;

    // Expected delay needed by the renderer, i.e. the frame will be delivered
    // this many milliseconds, if possible, earlier than the ideal render time.
    int render_delay_ms = 10;

    // If false, pass frames on to the renderer as soon as they are
    // available.
    bool enable_prerenderer_smoothing = true;

    // Identifier for an A/V synchronization group. Empty string to disable.
    // TODO(pbos): Synchronize streams in a sync group, not just video streams
    // to one of the audio streams.
    std::string sync_group;

    // An optional custom frame decryptor that allows the entire frame to be
    // decrypted in whatever way the caller choses. This is not required by
    // default.
    scoped_refptr<webrtc::FrameDecryptorInterface> frame_decryptor;

    // Per PeerConnection cryptography options.
    CryptoOptions crypto_options;

    scoped_refptr<webrtc::FrameTransformerInterface> frame_transformer;
  };

  // TODO(pbos): Add info on currently-received codec to Stats.
  virtual Stats GetStats() const = 0;

  // Sets a base minimum for the playout delay. Base minimum delay sets lower
  // bound on minimum delay value determining lower bound on playout delay.
  //
  // Returns true if value was successfully set, false overwise.
  virtual bool SetBaseMinimumPlayoutDelayMs(int delay_ms) = 0;

  // Returns current value of base minimum delay in milliseconds.
  virtual int GetBaseMinimumPlayoutDelayMs() const = 0;

  // Sets and returns recording state. The old state is moved out
  // of the video receive stream and returned to the caller, and `state`
  // is moved in. If the state's callback is set, it will be called with
  // recordable encoded frames as they arrive.
  // If `generate_key_frame` is true, the method will generate a key frame.
  // When the function returns, it's guaranteed that all old callouts
  // to the returned callback has ceased.
  // Note: the client should not interpret the returned state's attributes, but
  // instead treat it as opaque data.
  virtual RecordingState SetAndGetRecordingState(RecordingState state,
                                                 bool generate_key_frame) = 0;

  // Cause eventual generation of a key frame from the sender.
  virtual void GenerateKeyFrame() = 0;

  // Sets or clears a flexfec RTP sink. This affects `rtp.packet_sink_` and
  // `rtp.protected_by_flexfec` parts of the configuration. Must be called on
  // the packet delivery thread.
  // TODO(bugs.webrtc.org/11993): Packet delivery thread today means `worker
  // thread` but will be `network thread`.
  virtual void SetFlexFecProtection(RtpPacketSinkInterface* flexfec_sink) = 0;

  // Turns on/off loss notifications. Must be called on the packet delivery
  // thread.
  virtual void SetLossNotificationEnabled(bool enabled) = 0;

  // Modify `rtp.nack.rtp_history_ms` post construction. Setting this value
  // to 0 disables nack.
  // Must be called on the packet delivery thread.
  virtual void SetNackHistory(TimeDelta history) = 0;

  virtual void SetProtectionPayloadTypes(int red_payload_type,
                                         int ulpfec_payload_type) = 0;

  virtual void SetRtcpXr(Config::Rtp::RtcpXr rtcp_xr) = 0;

  virtual void SetAssociatedPayloadTypes(
      std::map<int, int> associated_payload_types) = 0;

  virtual void UpdateRtxSsrc(uint32_t ssrc) = 0;

 protected:
  virtual ~VideoReceiveStreamInterface() {}
};

}  // namespace webrtc

#endif  // CALL_VIDEO_RECEIVE_STREAM_H_
