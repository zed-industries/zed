// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_FAST_SHARED_BUFFER_READER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_FAST_SHARED_BUFFER_READER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/image-decoders/segment_reader.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// This class is used by image decoders to avoid memory consolidation and
// therefore minimizes the cost of memory copying when the decoders
// repeatedly read from a buffer that is continually growing due to network
// traffic.
class PLATFORM_EXPORT FastSharedBufferReader final {
  DISALLOW_NEW();

 public:
  explicit FastSharedBufferReader(scoped_refptr<SegmentReader> data);
  FastSharedBufferReader(const FastSharedBufferReader&) = delete;
  FastSharedBufferReader& operator=(const FastSharedBufferReader&) = delete;
  ~FastSharedBufferReader();

  void SetData(scoped_refptr<SegmentReader>);

  // Returns a consecutive buffer that carries the data starting
  // at |data_position| with |length| bytes.
  // This method returns a pointer to a memory segment stored in
  // |data_| if such a consecutive buffer can be found.
  // Otherwise copies into |buffer| and returns it.
  // Caller must ensure there are enough bytes in |data_| and |buffer|.
  const char* GetConsecutiveData(size_t data_position,
                                 size_t length,
                                 char* buffer) const;

  // Wraps SegmentReader::GetSomeData().
  size_t GetSomeData(const char*& some_data, size_t data_position) const;

  // Returns a byte at |data_position|.
  // Caller must ensure there are enough bytes in |data_|.
  inline char GetOneByte(size_t data_position) const {
    return *GetConsecutiveData(data_position, 1, nullptr);
  }

  size_t size() const { return data_->size(); }

  // This class caches the last access for faster subsequent reads. This
  // method clears that cache in case the SegmentReader has been modified
  // (e.g. with MergeSegmentsIntoBuffer on a wrapped SharedBuffer).
  void ClearCache();

 private:
  void GetSomeDataInternal(size_t data_position) const;

  scoped_refptr<SegmentReader> data_;

  // Caches the last segment of |data_| accessed, since subsequent reads are
  // likely to re-access it.
  mutable const char* segment_;
  mutable size_t segment_length_;

  // Data position in |data_| pointed to by |segment_|.
  mutable size_t data_position_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_FAST_SHARED_BUFFER_READER_H_
