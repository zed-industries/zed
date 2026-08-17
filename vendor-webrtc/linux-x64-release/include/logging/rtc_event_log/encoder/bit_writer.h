/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_ENCODER_BIT_WRITER_H_
#define LOGGING_RTC_EVENT_LOG_ENCODER_BIT_WRITER_H_

#include <stddef.h>
#include <stdint.h>

#include <string>

#include "absl/strings/string_view.h"
#include "rtc_base/bit_buffer.h"
#include "rtc_base/checks.h"

namespace webrtc {

// Wrap BitBufferWriter and extend its functionality by (1) keeping track of
// the number of bits written and (2) owning its buffer.
class BitWriter final {
 public:
  explicit BitWriter(size_t byte_count)
      : buffer_(byte_count, '\0'),
        bit_writer_(reinterpret_cast<uint8_t*>(&buffer_[0]), buffer_.size()),
        written_bits_(0),
        valid_(true) {
    RTC_DCHECK_GT(byte_count, 0);
  }

  BitWriter(const BitWriter&) = delete;
  BitWriter& operator=(const BitWriter&) = delete;

  void WriteBits(uint64_t val, size_t bit_count);

  void WriteBits(absl::string_view input);

  // Returns everything that was written so far.
  // Nothing more may be written after this is called.
  std::string GetString();

 private:
  std::string buffer_;
  BitBufferWriter bit_writer_;
  // Note: Counting bits instead of bytes wraps around earlier than it has to,
  // which means the maximum length is lower than it could be. We don't expect
  // to go anywhere near the limit, though, so this is good enough.
  size_t written_bits_;
  bool valid_;
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_ENCODER_BIT_WRITER_H_
