/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_SYMMETRIC_MATRIX_BUFFER_H_
#define MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_SYMMETRIC_MATRIX_BUFFER_H_

#include <algorithm>
#include <array>
#include <cstring>
#include <utility>

#include "api/array_view.h"
#include "rtc_base/checks.h"
#include "rtc_base/numerics/safe_compare.h"

namespace webrtc {
namespace rnn_vad {

// Data structure to buffer the results of pair-wise comparisons between items
// stored in a ring buffer. Every time that the oldest item is replaced in the
// ring buffer, the new one is compared to the remaining items in the ring
// buffer. The results of such comparisons need to be buffered and automatically
// removed when one of the two corresponding items that have been compared is
// removed from the ring buffer. It is assumed that the comparison is symmetric
// and that comparing an item with itself is not needed.
template <typename T, int S>
class SymmetricMatrixBuffer {
  static_assert(S > 2, "");

 public:
  SymmetricMatrixBuffer() = default;
  SymmetricMatrixBuffer(const SymmetricMatrixBuffer&) = delete;
  SymmetricMatrixBuffer& operator=(const SymmetricMatrixBuffer&) = delete;
  ~SymmetricMatrixBuffer() = default;
  // Sets the buffer values to zero.
  void Reset() {
    static_assert(std::is_arithmetic<T>::value,
                  "Integral or floating point required.");
    buf_.fill(0);
  }
  // Pushes the results from the comparison between the most recent item and
  // those that are still in the ring buffer. The first element in `values` must
  // correspond to the comparison between the most recent item and the second
  // most recent one in the ring buffer, whereas the last element in `values`
  // must correspond to the comparison between the most recent item and the
  // oldest one in the ring buffer.
  void Push(ArrayView<T, S - 1> values) {
    // Move the lower-right sub-matrix of size (S-2) x (S-2) one row up and one
    // column left.
    std::memmove(buf_.data(), buf_.data() + S, (buf_.size() - S) * sizeof(T));
    // Copy new values in the last column in the right order.
    for (int i = 0; SafeLt(i, values.size()); ++i) {
      const int index = (S - 1 - i) * (S - 1) - 1;
      RTC_DCHECK_GE(index, 0);
      RTC_DCHECK_LT(index, buf_.size());
      buf_[index] = values[i];
    }
  }
  // Reads the value that corresponds to comparison of two items in the ring
  // buffer having delay `delay1` and `delay2`. The two arguments must not be
  // equal and both must be in {0, ..., S - 1}.
  T GetValue(int delay1, int delay2) const {
    int row = S - 1 - delay1;
    int col = S - 1 - delay2;
    RTC_DCHECK_NE(row, col) << "The diagonal cannot be accessed.";
    if (row > col)
      std::swap(row, col);  // Swap to access the upper-right triangular part.
    RTC_DCHECK_LE(0, row);
    RTC_DCHECK_LT(row, S - 1) << "Not enforcing row < col and row != col.";
    RTC_DCHECK_LE(1, col) << "Not enforcing row < col and row != col.";
    RTC_DCHECK_LT(col, S);
    const int index = row * (S - 1) + (col - 1);
    RTC_DCHECK_LE(0, index);
    RTC_DCHECK_LT(index, buf_.size());
    return buf_[index];
  }

 private:
  // Encode an upper-right triangular matrix (excluding its diagonal) using a
  // square matrix. This allows to move the data in Push() with one single
  // operation.
  std::array<T, (S - 1) * (S - 1)> buf_{};
};

}  // namespace rnn_vad
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_SYMMETRIC_MATRIX_BUFFER_H_
