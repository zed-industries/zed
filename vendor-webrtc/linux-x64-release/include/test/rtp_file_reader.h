/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_RTP_FILE_READER_H_
#define TEST_RTP_FILE_READER_H_

#include <cstdint>
#include <set>
#include <string>

#include "absl/strings/string_view.h"

namespace webrtc {
namespace test {

struct RtpPacket {
  // Accommodate for 50 ms packets of 32 kHz PCM16 samples (3200 bytes) plus
  // some overhead.
  static const size_t kMaxPacketBufferSize = 3500;
  uint8_t data[kMaxPacketBufferSize];
  size_t length;
  // The length the packet had on wire. Will be different from `length` when
  // reading a header-only RTP dump.
  size_t original_length;

  uint32_t time_ms;
};

class RtpFileReader {
 public:
  enum FileFormat { kPcap, kRtpDump, kLengthPacketInterleaved };

  virtual ~RtpFileReader() {}
  static RtpFileReader* Create(FileFormat format,
                               const uint8_t* data,
                               size_t size,
                               const std::set<uint32_t>& ssrc_filter);
  static RtpFileReader* Create(FileFormat format, absl::string_view filename);
  static RtpFileReader* Create(FileFormat format,
                               absl::string_view filename,
                               const std::set<uint32_t>& ssrc_filter);
  virtual bool NextPacket(RtpPacket* packet) = 0;
};
}  // namespace test
}  // namespace webrtc
#endif  // TEST_RTP_FILE_READER_H_
