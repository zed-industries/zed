/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Circular buffers for planar or interleaved data.
//
// A circular buffer is an efficient, fixed-size memory structure that reuses
// the same memory for streaming operations. These classes support streaming
// where the input and output block size are unequal, but you should always
// make sure that the requested number of samples are available first using
// buffer.NumReadableEntries().
//
// Reading and writing to the buffer advance the read and write pointers,
// respectivley. It is assumed that the same data is not intended to be read
// from the buffer multiple times.
//
// https://en.wikipedia.org/wiki/Circular_buffer
// https://en.wikipedia.org/wiki/Interleaving_(data)

#ifndef AUDIO_DSP_CIRCULAR_BUFFER_H_
#define AUDIO_DSP_CIRCULAR_BUFFER_H_

#include <vector>

#include "glog/logging.h"
#include "absl/types/span.h"

namespace audio_dsp {

template <typename DataType>
class CircularBuffer {
 public:
  using Span = absl::Span<const DataType>;
  using MutableSpan = absl::Span<DataType>;

  CircularBuffer()
      : capacity_(0 /*uninitialized*/) {}

  // capacity is the amount of data that can be stored in the circular buffer.
  void Init(int capacity) {
    ABSL_CHECK_GT(capacity, 0);
    capacity_ = capacity;
    data_.resize(capacity_);
    Clear();
  }

  // input.size() must be less than size()!
  void Write(Span input) {
    std::pair<MutableSpan, MutableSpan> buffers =
        Write(input.size());
    std::copy(input.begin(), input.begin() + buffers.first.size(),
              buffers.first.begin());
    std::copy(input.begin() + buffers.first.size(), input.end(),
              buffers.second.begin());
  }

  // This signature gives access to an allocated block of internal storage that
  // the client may write to. The size of the two returned arrays totals
  // num_to_write, but either may have any size, even zero. The first
  // array precedes the second in terms of sequential data ordering. The
  // internal pointer is advanced accordingly, possibly overwriting old entries.
  std::pair<MutableSpan, MutableSpan> Write(int num_to_write) {
    ABSL_CHECK_GT(capacity_, 0) << "Did you forget to call Init()?";
    ABSL_DCHECK_LE(num_to_write, capacity());
    if (num_to_write == 0) {
      return {MutableSpan(), MutableSpan()};
    }

    MutableSpan first;
    MutableSpan second;
    if (write_index_ + num_to_write <= capacity_) {
      // We don't need to wrap over the edge of the circular buffer to
      // add the data.
      first = MutableSpan(&data_[write_index_], num_to_write);
    } else {
      int entries_before_wrap = capacity_ - write_index_;
      first = MutableSpan(&data_[write_index_], entries_before_wrap);
      second = MutableSpan(&data_[0], num_to_write - entries_before_wrap);
    }

    write_index_ = (write_index_ + num_to_write) % capacity_;
    if (num_available_data_ + num_to_write > capacity_) {
      num_available_data_ = capacity_;
    } else {
      num_available_data_ += num_to_write;
    }
    is_full_ = write_index_ == read_index_;
    return {first, second};
  }

  // num_to_read must be less than NumReadableEntries()! The read pointer is
  // advanced by output.size() samples.
  void Read(MutableSpan output) {
    std::pair<Span, Span> buffers = Read(output.size());
    CopySpansToBuffer(buffers, output);
  }

  // output will be resized, if necessary.
  // num_to_read must be less than NumReadableEntries()!
  void Read(int num_to_read, std::vector<DataType>* output) {
    output->resize(num_to_read);
    std::pair<Span, Span> buffers = Read(output->size());
    CopySpansToBuffer(buffers, absl::MakeSpan(*output));
  }

  // Similar to above, the returned array slices give direct, immutable access
  // to the requested memory. The size of the two returned arrays totals
  // num_to_read, but either may have any size, even zero. The first
  // array precedes the second in terms of sequential data ordering.
  // This function should be seen as an alterative to Read(int, vector<T>*) for
  // those who want more control over memory management.
  std::pair<Span, Span> Read(int num_to_read) {
    auto array_slices = Peek(num_to_read);
    Advance(num_to_read);
    return array_slices;
  }

  // Peek is same as Read, but without altering the buffer contents or
  // modifying the read pointer. num_to_read must be less than
  // NumReadableEntries()!
  void Peek(MutableSpan output) const {
    std::pair<Span, Span> buffers = Peek(output.size());
    CopySpansToBuffer(buffers, output);
  }

  // output will be resized, if necessary.
  void Peek(int num_to_read, std::vector<DataType>* output) const {
    output->resize(num_to_read);
    std::pair<Span, Span> buffers = Peek(output->size());
    CopySpansToBuffer(buffers, absl::MakeSpan(*output));
  }

  std::pair<Span, Span> Peek(int num_to_read) const {
    ABSL_CHECK_GT(capacity_, 0) << "Did you forget to call Init()?";
    if (num_to_read == 0) {
      return {Span(), Span()};
    }
    ABSL_DCHECK_GT(num_to_read, 0);
    ABSL_DCHECK_LE(num_to_read, NumReadableEntries());

    Span first;
    Span second;
    int end_of_read_section = (read_index_ + num_to_read) % capacity_;
    if (read_index_ < end_of_read_section) {
      // We can read without wrapping around the edge of the buffer.
      first = Span(&data_[read_index_], end_of_read_section - read_index_);
    } else {
      int samples_before_wrap = capacity_ - read_index_;
      first = Span(&data_[read_index_], data_.size() - read_index_);
      second = Span(&data_[0], num_to_read - samples_before_wrap);
    }
    return {first, second};
  }

  // Advance the read pointer. advance_by must be less than
  // NumReadableEntries().
  void Advance(int advance_by) {
    ABSL_CHECK_LE(advance_by, num_available_data_);
    is_full_ = false;
    read_index_ = (read_index_ + advance_by) % capacity_;
    num_available_data_ -= advance_by;
  }

  // Clears the buffer
  void Clear() {
    is_full_ = false;
    read_index_ = 0;
    write_index_ = 0;
    num_available_data_ = 0;
  }

  int capacity() const { return capacity_; }

  // Returns whether or not data can be written into the buffer without
  // overwriting old, unread data.
  bool IsFull() const { return is_full_; }

  // Returns the number of entries (per channel) that are in the buffer that
  // are available to read.
  int NumReadableEntries() const { return num_available_data_; }

 private:
  void CopySpansToBuffer(std::pair<Span, Span> buffers,
                         MutableSpan output) const {
    std::copy(buffers.first.begin(), buffers.first.end(),
              output.begin());
    std::copy(buffers.second.begin(), buffers.second.end(),
              output.begin() + buffers.first.size());
  }

  int capacity_;
  bool is_full_;
  int num_available_data_;
  std::vector<DataType> data_;

  // Internal state variables.
  int read_index_;
  int write_index_;
};

template <typename DataType>
class PlanarCircularBuffer {
 public:
  PlanarCircularBuffer()
      : num_channels_(0 /*uninitialized*/) {}

  void Init(int num_channels, int capacity) {
    num_channels_ = num_channels;
    buffers_.resize(num_channels_);
    for (int i = 0; i < num_channels_; ++i) {
      buffers_[i].Init(capacity);
    }
  }

  // input.size() / num_channels must be less than size()!
  void Write(const absl::Span<std::vector<DataType>> input) {
    for (int i = 0; i < num_channels_; ++i) {
      ABSL_DCHECK_EQ(input[i].size(), input[0].size());
      buffers_[i].Write(input[i]);
    }
  }

  // num_to_read must be less than NumReadableEntries()!
  void Read(int num_to_read, std::vector<std::vector<DataType>>* output) {
    output->resize(num_channels_);
    for (int i = 0; i < num_channels_; ++i) {
      buffers_[i].Read(num_to_read, &(*output)[i]);
    }
  }

  int GetNumChannels() const { return num_channels_; }

  int capacity() const {
    return buffers_[0].capacity();
  }
    // Returns whether or not data can be written into the buffer without
  // overwriting old, unread data.
  bool IsFull() const { return buffers_[0].IsFull(); }

  // Returns the number of entries (per channel) that are in the buffer that
  // are available to read.
  int NumReadableEntries() const {
    return buffers_[0].NumReadableEntries();
  }

 private:
  int num_channels_;
  std::vector<CircularBuffer<DataType>> buffers_;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_CIRCULAR_BUFFER_H_
