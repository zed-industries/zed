/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_TESTING_DATA_GENERATOR_H_
#define NET_DCSCTP_TESTING_DATA_GENERATOR_H_

#include <cstdint>
#include <optional>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/common/internal_types.h"
#include "net/dcsctp/packet/data.h"

namespace dcsctp {

struct DataGeneratorOptions {
  StreamID stream_id = StreamID(1);
  std::optional<MID> mid = std::nullopt;
  PPID ppid = PPID(53);
};

// Generates Data with correct sequence numbers, and used only in unit tests.
class DataGenerator {
 public:
  explicit DataGenerator(MID start_mid = MID(0)) : mid_(start_mid) {}

  // Generates ordered "data" with the provided `payload` and flags, which can
  // contain "B" for setting the "is_beginning" flag, and/or "E" for setting the
  // "is_end" flag.
  Data Ordered(std::vector<uint8_t> payload,
               absl::string_view flags = "",
               DataGeneratorOptions opts = {});

  // Generates unordered "data" with the provided `payload` and flags, which can
  // contain "B" for setting the "is_beginning" flag, and/or "E" for setting the
  // "is_end" flag.
  Data Unordered(std::vector<uint8_t> payload,
                 absl::string_view flags = "",
                 DataGeneratorOptions opts = {});

  // Resets the Message ID identifier - simulating a "stream reset".
  void ResetStream() { mid_ = MID(0); }

 private:
  MID mid_;
  FSN fsn_ = FSN(0);
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_TESTING_DATA_GENERATOR_H_
