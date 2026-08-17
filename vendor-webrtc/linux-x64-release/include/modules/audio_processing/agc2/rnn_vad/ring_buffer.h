/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_RING_BUFFER_H_
#define MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_RING_BUFFER_H_

#include <array>
#include <cstring>
#include <type_traits>

#include "api/array_view.h"

namespace webrtc {
namespace rnn_vad {

// Ring buffer for N arrays of type T each one with size S.
template <typename T, int S, int N>
class RingBuffer {
  static_assert(S > 0, "");
  static_assert(N > 0, "");
  static_assert(std::is_arithmetic<T>::value,
                "Integral or floating point required.");

 public:
  RingBuffer() : tail_(0) {}
  RingBuffer(const RingBuffer&) = delete;
  RingBuffer& operator=(const RingBuffer&) = delete;
  ~RingBuffer() = default;
  // Set the ring buffer values to zero.
  void Reset() { buffer_.fill(0); }
  // Replace the least recently pushed array in the buffer with `new_values`.
  void Push(ArrayView<const T, S> new_values) {
    std::memcpy(buffer_.data() + S * tail_, new_values.data(), S * sizeof(T));
    tail_ += 1;
    if (tail_ == N)
      tail_ = 0;
  }
  // Return an array view onto the array with a given delay. A view on the last
  // and least recently push array is returned when `delay` is 0 and N - 1
  // respectively.
  ArrayView<const T, S> GetArrayView(int delay) const {
    RTC_DCHECK_LE(0, delay);
    RTC_DCHECK_LT(delay, N);
    int offset = tail_ - 1 - delay;
    if (offset < 0)
      offset += N;
    return {buffer_.data() + S * offset, S};
  }

 private:
  int tail_;  // Index of the least recently pushed sub-array.
  std::array<T, S * N> buffer_{};
};

}  // namespace rnn_vad
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_RING_BUFFER_H_
