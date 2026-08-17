/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_AUDIO_MULTI_VECTOR_H_
#define MODULES_AUDIO_CODING_NETEQ_AUDIO_MULTI_VECTOR_H_

#include <stdint.h>
#include <string.h>

#include <vector>

#include "api/array_view.h"
#include "modules/audio_coding/neteq/audio_vector.h"

namespace webrtc {

class AudioMultiVector {
 public:
  // Creates an empty AudioMultiVector with `N` audio channels. `N` must be
  // larger than 0.
  explicit AudioMultiVector(size_t N);

  // Creates an AudioMultiVector with `N` audio channels, each channel having
  // an initial size. `N` must be larger than 0.
  AudioMultiVector(size_t N, size_t initial_size);

  virtual ~AudioMultiVector();

  AudioMultiVector(const AudioMultiVector&) = delete;
  AudioMultiVector& operator=(const AudioMultiVector&) = delete;

  // Deletes all values and make the vector empty.
  virtual void Clear();

  // Clears the vector and inserts `length` zeros into each channel.
  virtual void Zeros(size_t length);

  // Copies all values from this vector to `copy_to`. Any contents in `copy_to`
  // are deleted. After the operation is done, `copy_to` will be an exact
  // replica of this object. The source and the destination must have the same
  // number of channels.
  virtual void CopyTo(AudioMultiVector* copy_to) const;

  // Appends the contents of `append_this` to the end of this object. The array
  // is assumed to be channel-interleaved. The length must be an even multiple
  // of this object's number of channels. The length of this object is increased
  // with the length of the array divided by the number of channels.
  void PushBackInterleaved(ArrayView<const int16_t> append_this);

  // Appends the contents of AudioMultiVector `append_this` to this object. The
  // length of this object is increased with the length of `append_this`.
  virtual void PushBack(const AudioMultiVector& append_this);

  // Appends the contents of AudioMultiVector `append_this` to this object,
  // taken from `index` up until the end of `append_this`. The length of this
  // object is increased.
  virtual void PushBackFromIndex(const AudioMultiVector& append_this,
                                 size_t index);

  // Removes `length` elements from the beginning of this object, from each
  // channel.
  virtual void PopFront(size_t length);

  // Removes `length` elements from the end of this object, from each
  // channel.
  virtual void PopBack(size_t length);

  // Reads `length` samples from each channel and writes them interleaved to
  // `destination`. The total number of elements written to `destination` is
  // returned, i.e., `length` * number of channels. If the AudioMultiVector
  // contains less than `length` samples per channel, this is reflected in the
  // return value.
  virtual size_t ReadInterleaved(size_t length, int16_t* destination) const;

  // Like ReadInterleaved() above, but reads from `start_index` instead of from
  // the beginning.
  virtual size_t ReadInterleavedFromIndex(size_t start_index,
                                          size_t length,
                                          int16_t* destination) const;

  // Like ReadInterleaved() above, but reads from the end instead of from
  // the beginning.
  virtual size_t ReadInterleavedFromEnd(size_t length,
                                        int16_t* destination) const;

  // Overwrites each channel in this AudioMultiVector with values taken from
  // `insert_this`. The values are taken from the beginning of `insert_this` and
  // are inserted starting at `position`. `length` values are written into each
  // channel. If `length` and `position` are selected such that the new data
  // extends beyond the end of the current AudioVector, the vector is extended
  // to accommodate the new data. `length` is limited to the length of
  // `insert_this`.
  virtual void OverwriteAt(const AudioMultiVector& insert_this,
                           size_t length,
                           size_t position);

  // Appends `append_this` to the end of the current vector. Lets the two
  // vectors overlap by `fade_length` samples (per channel), and cross-fade
  // linearly in this region.
  virtual void CrossFade(const AudioMultiVector& append_this,
                         size_t fade_length);

  // Returns the number of channels.
  virtual size_t Channels() const;

  // Returns the number of elements per channel in this AudioMultiVector.
  virtual size_t Size() const;

  // Verify that each channel can hold at least `required_size` elements. If
  // not, extend accordingly.
  virtual void AssertSize(size_t required_size);

  virtual bool Empty() const;

  // Copies the data between two channels in the AudioMultiVector. The method
  // does not add any new channel. Thus, `from_channel` and `to_channel` must
  // both be valid channel numbers.
  virtual void CopyChannel(size_t from_channel, size_t to_channel);

  // Accesses and modifies a channel (i.e., an AudioVector object) of this
  // AudioMultiVector.
  const AudioVector& operator[](size_t index) const;
  AudioVector& operator[](size_t index);

 protected:
  std::vector<AudioVector*> channels_;
  size_t num_channels_;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_AUDIO_MULTI_VECTOR_H_
