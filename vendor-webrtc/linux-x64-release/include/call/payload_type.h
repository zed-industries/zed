/*
 *  Copyright 2024 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_PAYLOAD_TYPE_H_
#define CALL_PAYLOAD_TYPE_H_

#include <cstdint>
#include <string>

#include "api/rtc_error.h"
#include "media/base/codec.h"
#include "rtc_base/strong_alias.h"

namespace webrtc {

class PayloadType : public StrongAlias<class PayloadTypeTag, uint8_t> {
 public:
  // Non-explicit conversions from and to ints are to be deprecated and
  // removed once calling code is upgraded.
  PayloadType(uint8_t pt) { value_ = pt; }                // NOLINT: explicit
  constexpr operator uint8_t() const& { return value_; }  // NOLINT: Explicit
  static bool IsValid(PayloadType id, bool rtcp_mux) {
    // A payload type is a 7-bit value in the RTP header, so max = 127.
    // If RTCP multiplexing is used, the numbers from 64 to 95 are reserved
    // for RTCP packets.
    if (rtcp_mux && (id > 63 && id < 96)) {
      return false;
    }
    return id >= 0 && id <= 127;
  }
  template <typename Sink>
  friend void AbslStringify(Sink& sink, const PayloadType pt) {
    absl::Format(&sink, "%d", pt.value_);
  }
};

class PayloadTypeSuggester {
 public:
  virtual ~PayloadTypeSuggester() = default;

  // Suggest a payload type for a given codec on a given media section.
  // Media section is indicated by MID.
  // The function will either return a PT already in use on the connection
  // or a newly suggested one.
  virtual RTCErrorOr<PayloadType> SuggestPayloadType(const std::string& mid,
                                                     Codec codec) = 0;
  // Register a payload type as mapped to a specific codec for this MID
  // at this time.
  virtual RTCError AddLocalMapping(const std::string& mid,
                                   PayloadType payload_type,
                                   const Codec& codec) = 0;
};

}  // namespace webrtc

#endif  // CALL_PAYLOAD_TYPE_H_
