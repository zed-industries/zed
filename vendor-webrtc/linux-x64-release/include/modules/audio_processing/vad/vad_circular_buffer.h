/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_VAD_VAD_CIRCULAR_BUFFER_H_
#define MODULES_AUDIO_PROCESSING_VAD_VAD_CIRCULAR_BUFFER_H_

#include <memory>

namespace webrtc {

// A circular buffer tailored to the need of this project. It stores last
// K samples of the input, and keeps track of the mean of the last samples.
//
// It is used in class "PitchBasedActivity" to keep track of posterior
// probabilities in the past few seconds. The posterior probabilities are used
// to recursively update prior probabilities.
class VadCircularBuffer {
 public:
  static VadCircularBuffer* Create(int buffer_size);
  ~VadCircularBuffer();

  // If buffer is wrapped around.
  bool is_full() const { return is_full_; }
  // Get the oldest entry in the buffer.
  double Oldest() const;
  // Insert new value into the buffer.
  void Insert(double value);
  // Reset buffer, forget the past, start fresh.
  void Reset();

  // The mean value of the elements in the buffer. The return value is zero if
  // buffer is empty, i.e. no value is inserted.
  double Mean();
  // Remove transients. If the values exceed `val_threshold` for a period
  // shorter then or equal to `width_threshold`, then that period is considered
  // transient and set to zero.
  int RemoveTransient(int width_threshold, double val_threshold);

 private:
  explicit VadCircularBuffer(int buffer_size);
  // Get previous values. |index = 0| corresponds to the most recent
  // insertion. |index = 1| is the one before the most recent insertion, and
  // so on.
  int Get(int index, double* value) const;
  // Set a given position to `value`. `index` is interpreted as above.
  int Set(int index, double value);
  // Return the number of valid elements in the buffer.
  int BufferLevel();

  // Convert an index with the interpretation as get() method to the
  // corresponding linear index.
  int ConvertToLinearIndex(int* index) const;

  std::unique_ptr<double[]> buffer_;
  bool is_full_;
  int index_;
  int buffer_size_;
  double sum_;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_PROCESSING_VAD_VAD_CIRCULAR_BUFFER_H_
