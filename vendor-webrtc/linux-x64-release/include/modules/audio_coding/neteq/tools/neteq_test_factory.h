/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_TEST_FACTORY_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_TEST_FACTORY_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <string>

#include "absl/strings/string_view.h"
#include "api/neteq/neteq_factory.h"
#include "modules/audio_coding/neteq/tools/neteq_input.h"
#include "modules/audio_coding/neteq/tools/neteq_test.h"

namespace webrtc {
namespace test {

class SsrcSwitchDetector;
class NetEqStatsGetter;
class NetEqStatsPlotter;

// Note that the NetEqTestFactory needs to be alive when the NetEqTest object is
// used for a simulation.
class NetEqTestFactory {
 public:
  NetEqTestFactory();
  ~NetEqTestFactory();
  struct Config {
    Config();
    Config(const Config& other);
    ~Config();
    // RTP payload type for PCM-u.
    static constexpr int default_pcmu() { return 0; }
    int pcmu = default_pcmu();
    // RTP payload type for PCM-a.
    static constexpr int default_pcma() { return 8; }
    int pcma = default_pcma();
    // RTP payload type for iSAC.
    static constexpr int default_isac() { return 103; }
    int isac = default_isac();
    // RTP payload type for iSAC-swb (32 kHz).
    static constexpr int default_isac_swb() { return 104; }
    int isac_swb = default_isac_swb();
    // RTP payload type for Opus.
    static constexpr int default_opus() { return 111; }
    int opus = default_opus();
    // RTP payload type for PCM16b-nb (8 kHz).
    static constexpr int default_pcm16b() { return 93; }
    int pcm16b = default_pcm16b();
    // RTP payload type for PCM16b-wb (16 kHz).
    static constexpr int default_pcm16b_wb() { return 94; }
    int pcm16b_wb = default_pcm16b_wb();
    // RTP payload type for PCM16b-swb32 (32 kHz).
    static constexpr int default_pcm16b_swb32() { return 95; }
    int pcm16b_swb32 = default_pcm16b_swb32();
    // RTP payload type for PCM16b-swb48 (48 kHz).
    static constexpr int default_pcm16b_swb48() { return 96; }
    int pcm16b_swb48 = default_pcm16b_swb48();
    // RTP payload type for G.722.
    static constexpr int default_g722() { return 9; }
    int g722 = default_g722();
    // RTP payload type for AVT/DTMF (8 kHz).
    static constexpr int default_avt() { return 106; }
    int avt = default_avt();
    // RTP payload type for AVT/DTMF (16 kHz).
    static constexpr int default_avt_16() { return 114; }
    int avt_16 = default_avt_16();
    // RTP payload type for AVT/DTMF (32 kHz).
    static constexpr int default_avt_32() { return 115; }
    int avt_32 = default_avt_32();
    // RTP payload type for AVT/DTMF (48 kHz).
    static constexpr int default_avt_48() { return 116; }
    int avt_48 = default_avt_48();
    // RTP payload type for redundant audio (RED).
    static constexpr int default_red() { return 117; }
    int red = default_red();

    static constexpr int default_opus_red() { return 63; }
    int opus_red = default_opus_red();
    // RTP payload type for comfort noise (8 kHz).
    static constexpr int default_cn_nb() { return 13; }
    int cn_nb = default_cn_nb();
    // RTP payload type for comfort noise (16 kHz).
    static constexpr int default_cn_wb() { return 98; }
    int cn_wb = default_cn_wb();
    // RTP payload type for comfort noise (32 kHz).
    static constexpr int default_cn_swb32() { return 99; }
    int cn_swb32 = default_cn_swb32();
    // RTP payload type for comfort noise (48 kHz).
    static constexpr int default_cn_swb48() { return 100; }
    int cn_swb48 = default_cn_swb48();
    // A PCM file that will be used to populate dummy RTP packets.
    std::string replacement_audio_file;
    // Only use packets with this SSRC.
    std::optional<uint32_t> ssrc_filter;
    // Extension ID for audio level (RFC 6464).
    static constexpr int default_audio_level() { return 1; }
    int audio_level = default_audio_level();
    // Extension ID for absolute sender time.
    static constexpr int default_abs_send_time() { return 3; }
    int abs_send_time = default_abs_send_time();
    // Extension ID for transport sequence number.
    static constexpr int default_transport_seq_no() { return 5; }
    int transport_seq_no = default_transport_seq_no();
    // Extension ID for video content type.
    static constexpr int default_video_content_type() { return 7; }
    int video_content_type = default_video_content_type();
    // Extension ID for video timing.
    static constexpr int default_video_timing() { return 8; }
    int video_timing = default_video_timing();
    // Generate a matlab script for plotting the delay profile.
    bool matlabplot = false;
    // Generates a python script for plotting the delay profile.
    bool pythonplot = false;
    // Prints concealment events.
    bool concealment_events = false;
    // Maximum allowed number of packets in the buffer.
    static constexpr int default_max_nr_packets_in_buffer() { return 200; }
    int max_nr_packets_in_buffer = default_max_nr_packets_in_buffer();
    // Number of dummy packets to put in the packet buffer at the start of the
    // simulation.
    static constexpr int default_initial_dummy_packets() { return 0; }
    int initial_dummy_packets = default_initial_dummy_packets();
    // Number of getAudio events to skip at the start of the simulation.
    static constexpr int default_skip_get_audio_events() { return 0; }
    int skip_get_audio_events = default_skip_get_audio_events();
    // Enables jitter buffer fast accelerate.
    bool enable_fast_accelerate = false;
    // Dumps events that describes the simulation on a step-by-step basis.
    bool textlog = false;
    // If specified and `textlog` is true, the output of `textlog` is written to
    // the specified file name.
    std::optional<std::string> textlog_filename;
    // Base name for the output script files for plotting the delay profile.
    std::optional<std::string> plot_scripts_basename;
    // Path to the output audio file.
    std::optional<std::string> output_audio_filename;
    // Field trials to use during the simulation.
    std::string field_trial_string;
  };

  std::unique_ptr<NetEqTest> InitializeTestFromFile(
      absl::string_view input_filename,
      NetEqFactory* neteq_factory,
      const Config& config);
  std::unique_ptr<NetEqTest> InitializeTestFromString(
      absl::string_view input_string,
      NetEqFactory* neteq_factory,
      const Config& config);

 private:
  std::unique_ptr<NetEqTest> InitializeTest(std::unique_ptr<NetEqInput> input,
                                            NetEqFactory* neteq_factory,
                                            const Config& config);
  std::unique_ptr<SsrcSwitchDetector> ssrc_switch_detector_;
  std::unique_ptr<NetEqStatsPlotter> stats_plotter_;
};

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_TEST_FACTORY_H_
