/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_RTP_FILE_SOURCE_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_RTP_FILE_SOURCE_H_

#include <stdio.h>

#include <memory>
#include <optional>
#include <string>

#include "absl/strings/string_view.h"
#include "modules/audio_coding/neteq/tools/packet_source.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"

namespace webrtc {

namespace test {

class RtpFileReader;

class RtpFileSource : public PacketSource {
 public:
  // Creates an RtpFileSource reading from `file_name`. If the file cannot be
  // opened, or has the wrong format, NULL will be returned.
  static RtpFileSource* Create(
      absl::string_view file_name,
      std::optional<uint32_t> ssrc_filter = std::nullopt);

  // Checks whether a files is a valid RTP dump or PCAP (Wireshark) file.
  static bool ValidRtpDump(absl::string_view file_name);
  static bool ValidPcap(absl::string_view file_name);

  ~RtpFileSource() override;

  RtpFileSource(const RtpFileSource&) = delete;
  RtpFileSource& operator=(const RtpFileSource&) = delete;

  // Registers an RTP header extension and binds it to `id`.
  virtual bool RegisterRtpHeaderExtension(RTPExtensionType type, uint8_t id);

  std::unique_ptr<Packet> NextPacket() override;

 private:
  static const int kFirstLineLength = 40;
  static const int kRtpFileHeaderSize = 4 + 4 + 4 + 2 + 2;
  static const size_t kPacketHeaderSize = 8;

  explicit RtpFileSource(std::optional<uint32_t> ssrc_filter);

  bool OpenFile(absl::string_view file_name);

  std::unique_ptr<RtpFileReader> rtp_reader_;
  const std::optional<uint32_t> ssrc_filter_;
  RtpHeaderExtensionMap rtp_header_extension_map_;
};

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_RTP_FILE_SOURCE_H_
