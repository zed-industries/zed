/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains the class RtpFormatVp8TestHelper. The class is
// responsible for setting up a fake VP8 bitstream according to the
// RTPVideoHeaderVP8 header. The packetizer can then be provided to this helper
// class, which will then extract all packets and compare to the expected
// outcome.

#ifndef MODULES_RTP_RTCP_SOURCE_RTP_FORMAT_VP8_TEST_HELPER_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_FORMAT_VP8_TEST_HELPER_H_

#include <cstddef>
#include <cstdint>

#include "api/array_view.h"
#include "modules/rtp_rtcp/source/rtp_format_vp8.h"
#include "modules/video_coding/codecs/vp8/include/vp8_globals.h"
#include "rtc_base/buffer.h"

namespace webrtc {

class RtpFormatVp8TestHelper {
 public:
  RtpFormatVp8TestHelper(const RTPVideoHeaderVP8* hdr, size_t payload_len);
  ~RtpFormatVp8TestHelper();

  RtpFormatVp8TestHelper(const RtpFormatVp8TestHelper&) = delete;
  RtpFormatVp8TestHelper& operator=(const RtpFormatVp8TestHelper&) = delete;

  void GetAllPacketsAndCheck(RtpPacketizerVp8* packetizer,
                             ArrayView<const size_t> expected_sizes);

  ArrayView<const uint8_t> payload() const { return payload_; }
  size_t payload_size() const { return payload_.size(); }

 private:
  // Returns header size, i.e. payload offset.
  int CheckHeader(ArrayView<const uint8_t> rtp_payload, bool first);
  void CheckPictureID(ArrayView<const uint8_t> rtp_payload, int* offset);
  void CheckTl0PicIdx(ArrayView<const uint8_t> rtp_payload, int* offset);
  void CheckTIDAndKeyIdx(ArrayView<const uint8_t> rtp_payload, int* offset);
  void CheckPayload(const uint8_t* data_ptr);

  const RTPVideoHeaderVP8* const hdr_info_;
  Buffer payload_;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTP_FORMAT_VP8_TEST_HELPER_H_
