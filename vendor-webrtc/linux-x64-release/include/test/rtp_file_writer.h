/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_RTP_FILE_WRITER_H_
#define TEST_RTP_FILE_WRITER_H_

#include <string>

#include "test/rtp_file_reader.h"

namespace webrtc {
namespace test {
class RtpFileWriter {
 public:
  enum FileFormat {
    kRtpDump,
  };

  virtual ~RtpFileWriter() {}
  static RtpFileWriter* Create(FileFormat format, const std::string& filename);

  virtual bool WritePacket(const RtpPacket* packet) = 0;
};
}  // namespace test
}  // namespace webrtc
#endif  // TEST_RTP_FILE_WRITER_H_
