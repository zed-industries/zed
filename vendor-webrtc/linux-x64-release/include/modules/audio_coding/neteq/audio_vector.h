/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_AUDIO_VECTOR_H_
#define MODULES_AUDIO_CODING_NETEQ_AUDIO_VECTOR_H_

#include <string.h>

#include <cstdint>
#include <memory>

#include "rtc_base/checks.h"

namespace webrtc {

class AudioVector {
 public:
  // Creates an empty AudioVector.
  AudioVector();

  // Creates an AudioVector with an initial size.
  explicit AudioVector(size_t initial_size);

  virtual ~AudioVector();

  AudioVector(const AudioVector&) = delete;
  AudioVector& operator=(const AudioVector&) = delete;

  // Deletes all values and make the vector empty.
  virtual void Clear();

  // Copies all values from this vector to `copy_to`. Any contents in `copy_to`
  // are deleted before the copy operation. After the operation is done,
  // `copy_to` will be an exact replica of this object.
  virtual void CopyTo(AudioVector* copy_to) const;

  // Copies `length` values from `position` in this vector to `copy_to`.
  virtual void CopyTo(size_t length, size_t position, int16_t* copy_to) const;

  // Prepends the contents of AudioVector `prepend_this` to this object. The
  // length of this object is increased with the length of `prepend_this`.
  virtual void PushFront(const AudioVector& prepend_this);

  // Same as above, but with an array `prepend_this` with `length` elements as
  // source.
  virtual void PushFront(const int16_t* prepend_this, size_t length);

  // Same as PushFront but will append to the end of this object.
  virtual void PushBack(const AudioVector& append_this);

  // Appends a segment of `append_this` to the end of this object. The segment
  // starts from `position` and has `length` samples.
  virtual void PushBack(const AudioVector& append_this,
                        size_t length,
                        size_t position);

  // Same as PushFront but will append to the end of this object.
  virtual void PushBack(const int16_t* append_this, size_t length);

  // Removes `length` elements from the beginning of this object.
  virtual void PopFront(size_t length);

  // Removes `length` elements from the end of this object.
  virtual void PopBack(size_t length);

  // Extends this object with `extra_length` elements at the end. The new
  // elements are initialized to zero.
  virtual void Extend(size_t extra_length);

  // Inserts `length` elements taken from the array `insert_this` and insert
  // them at `position`. The length of the AudioVector is increased by `length`.
  // `position` = 0 means that the new values are prepended to the vector.
  // `position` = Size() means that the new values are appended to the vector.
  virtual void InsertAt(const int16_t* insert_this,
                        size_t length,
                        size_t position);

  // Like InsertAt, but inserts `length` zero elements at `position`.
  virtual void InsertZerosAt(size_t length, size_t position);

  // Overwrites `length` elements of this AudioVector starting from `position`
  // with first values in `AudioVector`. The definition of `position`
  // is the same as for InsertAt(). If `length` and `position` are selected
  // such that the new data extends beyond the end of the current AudioVector,
  // the vector is extended to accommodate the new data.
  virtual void OverwriteAt(const AudioVector& insert_this,
                           size_t length,
                           size_t position);

  // Overwrites `length` elements of this AudioVector with values taken from the
  // array `insert_this`, starting at `position`. The definition of `position`
  // is the same as for InsertAt(). If `length` and `position` are selected
  // such that the new data extends beyond the end of the current AudioVector,
  // the vector is extended to accommodate the new data.
  virtual void OverwriteAt(const int16_t* insert_this,
                           size_t length,
                           size_t position);

  // Appends `append_this` to the end of the current vector. Lets the two
  // vectors overlap by `fade_length` samples, and cross-fade linearly in this
  // region.
  virtual void CrossFade(const AudioVector& append_this, size_t fade_length);

  // Returns the number of elements in this AudioVector.
  virtual size_t Size() const;

  // Returns true if this AudioVector is empty.
  virtual bool Empty() const;

  // Accesses and modifies an element of AudioVector.
  inline const int16_t& operator[](size_t index) const {
    return array_[WrapIndex(index, begin_index_, capacity_)];
  }

  inline int16_t& operator[](size_t index) {
    return array_[WrapIndex(index, begin_index_, capacity_)];
  }

 private:
  static const size_t kDefaultInitialSize = 10;

  // This method is used by the [] operators to calculate an index within the
  // capacity of the array, but without using the modulo operation (%).
  static inline size_t WrapIndex(size_t index,
                                 size_t begin_index,
                                 size_t capacity) {
    RTC_DCHECK_LT(index, capacity);
    RTC_DCHECK_LT(begin_index, capacity);
    size_t ix = begin_index + index;
    RTC_DCHECK_GE(ix, index);  // Check for overflow.
    if (ix >= capacity) {
      ix -= capacity;
    }
    RTC_DCHECK_LT(ix, capacity);
    return ix;
  }

  void Reserve(size_t n);

  void InsertByPushBack(const int16_t* insert_this,
                        size_t length,
                        size_t position);

  void InsertByPushFront(const int16_t* insert_this,
                         size_t length,
                         size_t position);

  void InsertZerosByPushBack(size_t length, size_t position);

  void InsertZerosByPushFront(size_t length, size_t position);

  std::unique_ptr<int16_t[]> array_;

  size_t capacity_;  // Allocated number of samples in the array.

  // The index of the first sample in `array_`, except when
  // |begin_index_ == end_index_|, which indicates an empty buffer.
  size_t begin_index_;

  // The index of the sample after the last sample in `array_`.
  size_t end_index_;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_AUDIO_VECTOR_H_
