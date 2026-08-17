/*
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_BYTE_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_BYTE_STREAM_H_

#include <memory>

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hasher.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

template <typename DataType>
union ByteType {
  DataType value;
  unsigned char bytes[sizeof(DataType)];
};

class SVGPathByteStream {
  USING_FAST_MALLOC(SVGPathByteStream);

 public:
  using Data = Vector<unsigned char>;
  using DataIterator = Data::const_iterator;

  SVGPathByteStream() = default;
  explicit SVGPathByteStream(const Vector<unsigned char, 1024>& other) {
    // NOTE: This does not use `_data(other)` as that triggers an
    // allocation at the capacity of `other`, which may be far too large.
    data_.ReserveInitialCapacity(other.size());
    data_.InsertVector(0, other);
  }

  std::unique_ptr<SVGPathByteStream> Clone() const {
    return base::WrapUnique(new SVGPathByteStream(data_));
  }

  DataIterator begin() const { return data_.begin(); }
  DataIterator end() const { return data_.end(); }
  void clear() { data_.clear(); }
  void ReserveInitialCapacity(wtf_size_t size) {
    data_.ReserveInitialCapacity(size);
  }
  void ShrinkToFit() { data_.shrink_to_fit(); }
  bool IsEmpty() const { return data_.empty(); }
  unsigned size() const { return data_.size(); }

  bool operator==(const SVGPathByteStream& other) const {
    return data_ == other.data_;
  }
  unsigned Hash() const { return StringHasher::HashMemory(data_); }

 private:
  explicit SVGPathByteStream(const Data& data) : data_(data) {}

  Data data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_BYTE_STREAM_H_
