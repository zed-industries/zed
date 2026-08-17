/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_UTIL_TRACE_BLOB_VIEW_READER_H_
#define SRC_TRACE_PROCESSOR_UTIL_TRACE_BLOB_VIEW_READER_H_

#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <optional>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/ext/base/circular_queue.h"
#include "perfetto/public/compiler.h"
#include "perfetto/trace_processor/trace_blob_view.h"

namespace perfetto::trace_processor::util {

// Helper class which handles all the complexity of reading pieces of data which
// span across multiple TraceBlobView chunks. It takes care of:
//  1) Buffering data until it can be read.
//  2) Stitching together the cross-chunk spanning pieces.
//  3) Dropping data when it is no longer necessary to be buffered.
class TraceBlobViewReader {
 private:
  struct Entry {
    // File offset of the first byte in `data`.
    size_t start_offset;
    TraceBlobView data;
    size_t end_offset() const { return start_offset + data.size(); }
  };

 public:
  class Iterator {
   public:
    ~Iterator() = default;

    Iterator(const Iterator&) = default;
    Iterator& operator=(const Iterator&) = default;

    Iterator(Iterator&&) = default;
    Iterator& operator=(Iterator&&) = default;

    // Tries to advance the iterator `size` bytes forward. Returns true if
    // the advance was successful and false if it would overflow the iterator.
    // If false is returned, the state of the iterator is not changed.
    bool MaybeAdvance(size_t delta) {
      file_offset_ += delta;
      if (PERFETTO_LIKELY(file_offset_ < iter_->end_offset())) {
        return true;
      }
      if (file_offset_ == end_offset_) {
        return true;
      }
      if (file_offset_ > end_offset_) {
        file_offset_ -= delta;
        return false;
      }
      do {
        ++iter_;
      } while (file_offset_ >= iter_->end_offset());
      return true;
    }

    // Tries to read `size` bytes from the iterator.  Returns a TraceBlobView
    // containing the data if `size` bytes were available and std::nullopt
    // otherwise. If std::nullopt is returned, the state of the iterator is not
    // changed.
    std::optional<TraceBlobView> MaybeRead(size_t delta) {
      std::optional<TraceBlobView> tbv =
          reader_->SliceOff(file_offset(), delta);
      if (PERFETTO_LIKELY(tbv)) {
        PERFETTO_CHECK(MaybeAdvance(delta));
      }
      return tbv;
    }

    // Tries to find a byte equal to |chr| in the iterator and, if found,
    // advance to it. Returns a TraceBlobView containing the data if the byte
    // was found and could be advanced to and std::nullopt if no such byte was
    // found before the end of the iterator. If std::nullopt is returned, the
    // state of the iterator is not changed.
    std::optional<TraceBlobView> MaybeFindAndRead(uint8_t chr) {
      size_t begin = file_offset();
      if (!MaybeFindAndAdvanceInner(chr)) {
        return std::nullopt;
      }
      std::optional<TraceBlobView> tbv =
          reader_->SliceOff(begin, file_offset() - begin);
      PERFETTO_CHECK(tbv);
      PERFETTO_CHECK(MaybeAdvance(1));
      return tbv;
    }

    uint8_t operator*() const {
      PERFETTO_DCHECK(file_offset_ < iter_->end_offset());
      return iter_->data.data()[file_offset_ - iter_->start_offset];
    }

    explicit operator bool() const { return file_offset_ != end_offset_; }

    size_t file_offset() const { return file_offset_; }

   private:
    friend TraceBlobViewReader;

    Iterator(const TraceBlobViewReader* reader,
             base::CircularQueue<Entry>::Iterator iter,
             size_t file_offset,
             size_t end_offset)
        : reader_(reader),
          iter_(iter),
          file_offset_(file_offset),
          end_offset_(end_offset) {}

    // Tries to find a byte equal to |chr| in the iterator and, if found,
    // advance to it. Returns true if the byte was found and could be advanced
    // to and false if no such byte was found before the end of the iterator. If
    // false is returned, the state of the iterator is not changed.
    bool MaybeFindAndAdvanceInner(uint8_t chr) {
      size_t off = file_offset_;
      while (off < end_offset_) {
        size_t iter_off = off - iter_->start_offset;
        size_t iter_rem = iter_->data.size() - iter_off;
        const auto* p = reinterpret_cast<const uint8_t*>(
            memchr(iter_->data.data() + iter_off, chr, iter_rem));
        if (p) {
          file_offset_ =
              iter_->start_offset + static_cast<size_t>(p - iter_->data.data());
          return true;
        }
        off = iter_->end_offset();
        ++iter_;
      }
      return false;
    }

    const TraceBlobViewReader* reader_;
    base::CircularQueue<Entry>::Iterator iter_;
    size_t file_offset_;
    size_t end_offset_;
  };

  Iterator GetIterator() const {
    return {this, data_.begin(), start_offset(), end_offset()};
  }

  // Adds a `TraceBlobView` at the back.
  void PushBack(TraceBlobView);

  // Shrinks the buffer by dropping data from the front of the buffer until the
  // given offset is reached. If not enough data is present as much data as
  // possible will be dropped and `false` will be returned.
  //
  // Note:
  //  * if `offset` < 'file_offset()' this method will CHECK fail.
  //  * calling this function invalidates all iterators created from this
  //  reader.
  bool PopFrontUntil(size_t offset);

  // Shrinks the buffer by dropping `bytes` from the front of the buffer. If not
  // enough data is present as much data as possible will be dropped and `false`
  // will be returned.
  //
  // Note: calling this function invalidates all iterators created from this
  // reader.
  bool PopFrontBytes(size_t bytes) {
    return PopFrontUntil(start_offset() + bytes);
  }

  // Creates a TraceBlobView by slicing this reader starting at |offset| and
  // spanning |length| bytes.
  //
  // If possible, this method will try to avoid copies and simply slice an
  // input TraceBlobView. However, that may not be possible and it so, it will
  // allocate a new chunk of memory and copy over the data instead.
  //
  // NOTE: If `offset` < 'file_offset()' this method will CHECK fail.
  std::optional<TraceBlobView> SliceOff(size_t offset, size_t length) const;

  // Similar to SliceOff but this method will not combine slices but instead
  // potentially return multiple chunks. Useful if we are extracting slices to
  // forward them to a `ChunkedTraceReader`.
  std::vector<TraceBlobView> MultiSliceOff(size_t offset, size_t length) const;

  // Returns the offset to the start of the available data.
  size_t start_offset() const {
    return data_.empty() ? end_offset_ : data_.front().start_offset;
  }

  // Returns the offset to the end of the available data.
  size_t end_offset() const { return end_offset_; }

  // Returns the number of bytes of buffered data.
  size_t avail() const { return end_offset() - start_offset(); }

  bool empty() const { return data_.empty(); }

 private:
  template <typename Visitor>
  auto SliceOffImpl(size_t offset, size_t length, Visitor& visitor) const;

  // CircularQueue has no const_iterator, so mutable is needed to access it from
  // const methods.
  mutable base::CircularQueue<Entry> data_;
  size_t end_offset_ = 0;
};

}  // namespace perfetto::trace_processor::util

#endif  // SRC_TRACE_PROCESSOR_UTIL_TRACE_BLOB_VIEW_READER_H_
