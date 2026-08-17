/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_FUZZERS_UTILS_RTP_REPLAYER_H_
#define TEST_FUZZERS_UTILS_RTP_REPLAYER_H_

#include <stdio.h>

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "api/test/video/function_video_decoder_factory.h"
#include "api/video_codecs/video_decoder.h"
#include "call/call.h"
#include "media/engine/internal_decoder_factory.h"
#include "rtc_base/fake_clock.h"
#include "rtc_base/time_utils.h"
#include "test/null_transport.h"
#include "test/rtp_file_reader.h"
#include "test/test_video_capturer.h"
#include "test/video_renderer.h"

namespace webrtc {
namespace test {

// The RtpReplayer is a utility for fuzzing the RTP/RTCP receiver stack in
// WebRTC. It achieves this by accepting a set of Receiver configurations and
// an RtpDump (consisting of both RTP and RTCP packets). The `rtp_dump` is
// passed in as a buffer to allow simple mutation fuzzing directly on the dump.
class RtpReplayer final {
 public:
  // Holds all the important stream information required to emulate the WebRTC
  // rtp receival code path.
  struct StreamState {
    test::NullTransport transport;
    std::vector<std::unique_ptr<VideoSinkInterface<VideoFrame>>> sinks;
    std::vector<VideoReceiveStreamInterface*> receive_streams;
    std::unique_ptr<VideoDecoderFactory> decoder_factory;
  };

  // Construct an RtpReplayer from a JSON replay configuration file.
  static void Replay(const std::string& replay_config_filepath,
                     const uint8_t* rtp_dump_data,
                     size_t rtp_dump_size);

  // Construct an RtpReplayer from  a set of
  // VideoReceiveStreamInterface::Configs. Note the stream_state.transport must
  // be set for each receiver stream.
  static void Replay(
      std::unique_ptr<StreamState> stream_state,
      std::vector<VideoReceiveStreamInterface::Config> receive_stream_config,
      const uint8_t* rtp_dump_data,
      size_t rtp_dump_size);

 private:
  // Reads the replay configuration from Json.
  static std::vector<VideoReceiveStreamInterface::Config> ReadConfigFromFile(
      const std::string& replay_config,
      Transport* transport);

  // Configures the stream state based on the receiver configurations.
  static void SetupVideoStreams(
      std::vector<VideoReceiveStreamInterface::Config>* receive_stream_configs,
      StreamState* stream_state,
      Call* call);

  // Creates a new RtpReader which can read the RtpDump
  static std::unique_ptr<test::RtpFileReader> CreateRtpReader(
      const uint8_t* rtp_dump_data,
      size_t rtp_dump_size);

  // Replays each packet to from the RtpDump.
  static void ReplayPackets(FakeClock* clock,
                            Call* call,
                            test::RtpFileReader* rtp_reader,
                            const RtpHeaderExtensionMap& extensions);
};  // class RtpReplayer

}  // namespace test
}  // namespace webrtc

#endif  // TEST_FUZZERS_UTILS_RTP_REPLAYER_H_
