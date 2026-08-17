/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_TRACE_PROCESSOR_TRACE_BLOB_VIEW_H_
#define INCLUDE_PERFETTO_TRACE_PROCESSOR_TRACE_BLOB_VIEW_H_

#include <stddef.h>
#include <stdint.h>

#include <limits>
#include <memory>

#include "perfetto/base/logging.h"
#include "perfetto/trace_processor/ref_counted.h"
#include "perfetto/trace_processor/trace_blob.h"

namespace perfetto {
namespace trace_processor {

// A read-only view of a TraceBlob.
// This class is an equivalent of std::string_view for trace binary data, with
// a twist: it supports turning a TraceBlob into a refcounted reference. In this
// case the TraceBlobView acts directly as a shared_ptr, without requiring extra
// layers of indirection.
// The underlying TraceBlob will be freed once all the TraceBlobViews that refer
// to the same buffer have passed through the pipeline and been parsed.
// The overall idea is that a TraceBlob is passed around until it's written.
// When writing is done it transforms into a shared refcounted object which is
// held onto by one or more read-only TraceBlobView instances.
//
// In summary:
//  - TraceBlob: writable, move-only, single-instance.
//  - TraceBlobView: readable, copyable, multiple-instances can hold onto
//                   (different sub-slices of) the same refcounted TraceBlob.
class alignas(8) TraceBlobView {
 public:
  // Takes ownership of the passed |blob|.
  static constexpr size_t kWholeBlob = std::numeric_limits<size_t>::max();
  explicit TraceBlobView(TraceBlob blob,
                         size_t offset = 0,
                         size_t length = kWholeBlob) {
    PERFETTO_DCHECK(offset <= std::numeric_limits<uint32_t>::max());
    data_ = blob.data() + offset;
    if (length == kWholeBlob) {
      length_ = static_cast<uint32_t>(blob.size() - offset);
    } else {
      PERFETTO_DCHECK(length <= std::numeric_limits<uint32_t>::max());
      PERFETTO_DCHECK(offset + length_ <= blob.size());
      length_ = static_cast<uint32_t>(length);
    }
    blob_.reset(new TraceBlob(std::move(blob)));
  }

  TraceBlobView(RefPtr<TraceBlob> blob, size_t offset, uint32_t length)
      : blob_(std::move(blob)), data_(blob_->data() + offset), length_(length) {
    PERFETTO_DCHECK(offset + length_ <= blob_->size());
  }

  // Trivial empty ctor.
  TraceBlobView() : data_(nullptr), length_(0) {}

  ~TraceBlobView() = default;

  // Allow std::move().
  TraceBlobView(TraceBlobView&& other) noexcept { *this = std::move(other); }

  TraceBlobView& operator=(TraceBlobView&& other) noexcept {
    data_ = other.data_;
    length_ = other.length_;
    blob_ = std::move(other.blob_);
    return *this;
  }

  // Disable copy operators. Use x.Copy() to get a copy.
  TraceBlobView(const TraceBlobView&) = delete;
  TraceBlobView& operator=(const TraceBlobView&) = delete;

  // [data, data+length] must be <= the current TraceBlobView.
  TraceBlobView slice(const uint8_t* data, size_t length) const {
    PERFETTO_DCHECK(data >= data_);
    PERFETTO_DCHECK(data + length <= data_ + length_);
    return TraceBlobView(blob_, data, static_cast<uint32_t>(length));
  }

  // Like slice() but takes an offset rather than a pointer as 1st argument.
  TraceBlobView slice_off(size_t off, size_t length) const {
    PERFETTO_DCHECK(off + length <= length_);
    return TraceBlobView(blob_, data_ + off, static_cast<uint32_t>(length));
  }

  TraceBlobView copy() const { return slice(data_, length_); }

  bool operator==(const TraceBlobView& rhs) const {
    return (data_ == rhs.data_) && (length_ == rhs.length_) &&
           (blob_ == rhs.blob_);
  }
  bool operator!=(const TraceBlobView& rhs) const { return !(*this == rhs); }

  const uint8_t* data() const { return data_; }
  size_t offset() const { return static_cast<size_t>(data_ - blob_->data()); }
  size_t length() const { return length_; }
  size_t size() const { return length_; }
  RefPtr<TraceBlob> blob() const { return blob_; }

 private:
  TraceBlobView(RefPtr<TraceBlob> blob, const uint8_t* data, uint32_t length)
      : blob_(std::move(blob)), data_(data), length_(length) {}

  RefPtr<TraceBlob> blob_;
  const uint8_t* data_ = nullptr;
  uint32_t length_ = 0;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACE_PROCESSOR_TRACE_BLOB_VIEW_H_
