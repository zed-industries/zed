/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_CLIPPING_PREDICTOR_LEVEL_BUFFER_H_
#define MODULES_AUDIO_PROCESSING_AGC2_CLIPPING_PREDICTOR_LEVEL_BUFFER_H_

#include <memory>
#include <optional>
#include <vector>

namespace webrtc {

// A circular buffer to store frame-wise `Level` items for clipping prediction.
// The current implementation is not optimized for large buffer lengths.
class ClippingPredictorLevelBuffer {
 public:
  struct Level {
    float average;
    float max;
    bool operator==(const Level& level) const;
  };

  // Recommended maximum capacity. It is possible to create a buffer with a
  // larger capacity, but the implementation is not optimized for large values.
  static constexpr int kMaxCapacity = 100;

  // Ctor. Sets the buffer capacity to max(1, `capacity`) and logs a warning
  // message if the capacity is greater than `kMaxCapacity`.
  explicit ClippingPredictorLevelBuffer(int capacity);
  ~ClippingPredictorLevelBuffer() {}
  ClippingPredictorLevelBuffer(const ClippingPredictorLevelBuffer&) = delete;
  ClippingPredictorLevelBuffer& operator=(const ClippingPredictorLevelBuffer&) =
      delete;

  void Reset();

  // Returns the current number of items stored in the buffer.
  int Size() const { return size_; }

  // Returns the capacity of the buffer.
  int Capacity() const { return data_.size(); }

  // Adds a `level` item into the circular buffer `data_`. Stores at most
  // `Capacity()` items. If more items are pushed, the new item replaces the
  // least recently pushed item.
  void Push(Level level);

  // If at least `num_items` + `delay` items have been pushed, returns the
  // average and maximum value for the `num_items` most recently pushed items
  // from `delay` to `delay` - `num_items` (a delay equal to zero corresponds
  // to the most recently pushed item). The value of `delay` is limited to
  // [0, N] and `num_items` to [1, M] where N + M is the capacity of the buffer.
  std::optional<Level> ComputePartialMetrics(int delay, int num_items) const;

 private:
  int tail_;
  int size_;
  std::vector<Level> data_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_CLIPPING_PREDICTOR_LEVEL_BUFFER_H_
