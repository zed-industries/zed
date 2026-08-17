/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_TOOLS_RTP_GENERATOR_RTP_GENERATOR_H_
#define RTC_TOOLS_RTP_GENERATOR_RTP_GENERATOR_H_

#include <memory>
#include <string>
#include <vector>

#include "api/call/transport.h"
#include "api/environment/environment.h"
#include "api/media_types.h"
#include "api/video/builtin_video_bitrate_allocator_factory.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "call/call.h"
#include "call/rtp_config.h"
#include "call/video_send_stream.h"
#include "media/engine/webrtc_video_engine.h"
#include "test/frame_generator_capturer.h"
#include "test/rtp_file_reader.h"
#include "test/rtp_file_writer.h"
#include "video/config/video_encoder_config.h"

namespace webrtc {

// Specifies all the configurable options to pass to the corpus generator.
// If modified please update the JSON parser as well as all.
struct RtpGeneratorOptions {
  struct VideoSendStreamConfig {
    // The time to record the RtpDump for.
    int duration_ms = 10000;
    // The video resolution width.
    int video_width = 640;
    // The video resolution height.
    int video_height = 480;
    // The video fps.
    int video_fps = 24;
    // The number of squares to render.
    int num_squares = 128;
    // The individual RTP configuration.
    RtpConfig rtp;
  };
  // Multiple senders can be active at once on an rtp channel.
  std::vector<VideoSendStreamConfig> video_streams;
};

// Attempts to parse RtpGeneratorOptions from a JSON file. Any failures
// will result in std::nullopt.
std::optional<RtpGeneratorOptions> ParseRtpGeneratorOptionsFromFile(
    const std::string& options_file);

// The RtpGenerator allows generating of corpus material intended to be
// used by fuzzers. It accepts a simple Json configuration file that allows the
// user to configure the codec, extensions and error correction mechanisms. It
// will then proceed to generate an rtpdump for the specified duration using
// that configuration that can be replayed by the video_replayer. The receiver
// configuration JSON will also be output and can be replayed as follows:
// ./rtp_generator --config_file sender_config --output_rtpdump my.rtpdump
//                 --output_config receiver_config.json
// ./video_replay --config_file receiver_config.json --output_file my.rtpdump
//
// It achieves this by creating a VideoStreamSender, configuring it as requested
// by the user and then intercepting all outgoing RTP packets and writing them
// to a file instead of out of the network. It then uses this sender
// configuration to generate a mirror receiver configuration that can be read by
// the video_replay program.
class RtpGenerator final : public webrtc::Transport {
 public:
  // Construct a new RtpGenerator using the specified options.
  explicit RtpGenerator(const RtpGeneratorOptions& options);

  RtpGenerator() = delete;
  RtpGenerator(const RtpGenerator&) = delete;
  RtpGenerator& operator=(const RtpGenerator&) = delete;

  // Cleans up the VideoSendStream.
  ~RtpGenerator() override;
  // Generates an rtp_dump that is written out to
  void GenerateRtpDump(const std::string& rtp_dump_path);

 private:
  // webrtc::Transport implementation
  // Captured RTP packets are written to the RTPDump file instead of over the
  // network.
  bool SendRtp(ArrayView<const uint8_t> packet,
               const webrtc::PacketOptions& options) override;
  // RTCP packets are ignored for now.
  bool SendRtcp(ArrayView<const uint8_t> packet) override;
  // Returns the maximum duration
  int GetMaxDuration() const;
  // Waits until all video streams have finished.
  void WaitUntilAllVideoStreamsFinish();
  // Converts packet data into an RtpPacket.
  test::RtpPacket DataToRtpPacket(const uint8_t* packet, size_t packet_len);

  const RtpGeneratorOptions options_;
  const Environment env_;
  std::unique_ptr<VideoEncoderFactory> video_encoder_factory_;
  std::unique_ptr<VideoDecoderFactory> video_decoder_factory_;
  std::unique_ptr<VideoBitrateAllocatorFactory>
      video_bitrate_allocator_factory_;
  std::unique_ptr<Call> call_;
  std::unique_ptr<test::RtpFileWriter> rtp_dump_writer_;
  std::vector<std::unique_ptr<test::FrameGeneratorCapturer>> frame_generators_;
  std::vector<VideoSendStream*> video_send_streams_;
  std::vector<uint32_t> durations_ms_;
  uint32_t start_ms_ = 0;
};

}  // namespace webrtc

#endif  // RTC_TOOLS_RTP_GENERATOR_RTP_GENERATOR_H_
