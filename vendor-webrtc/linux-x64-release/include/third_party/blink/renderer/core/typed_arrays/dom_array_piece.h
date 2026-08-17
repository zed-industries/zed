// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_ARRAY_PIECE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_ARRAY_PIECE_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"

namespace blink {

class DOMArrayBufferView;
class V8UnionArrayBufferOrArrayBufferView;

// This class is for passing around un-owned bytes as a pointer + length.
// It supports implicit conversion from several other data types.
//
// DOMArrayPiece has the concept of being "null". This is different from an
// empty byte range. It is invalid to call methods other than isNull() on such
// instances.
//
// IMPORTANT: The data contained by DOMArrayPiece is NOT OWNED, so caution must
//            be taken to ensure it is kept alive.
class CORE_EXPORT DOMArrayPiece {
  DISALLOW_NEW();

 public:
  DOMArrayPiece();
  // NOLINTNEXTLINE(google-explicit-constructor)
  DOMArrayPiece(DOMArrayBuffer* buffer);
  // NOLINTNEXTLINE(google-explicit-constructor)
  DOMArrayPiece(DOMArrayBufferView* view);
  // NOLINTNEXTLINE(google-explicit-constructor)
  DOMArrayPiece(
      const V8UnionArrayBufferOrArrayBufferView* array_buffer_or_view);

  bool operator==(const DOMArrayBuffer& other) const {
    return ByteSpan() == other.ByteSpan();
  }

  bool IsNull() const;
  bool IsDetached() const;
  void* Data() const;
  unsigned char* Bytes() const;
  size_t ByteLength() const;
  base::span<uint8_t> ByteSpan() const;

 private:
  void InitWithArrayBuffer(DOMArrayBuffer*);
  void InitWithArrayBufferView(DOMArrayBufferView*);
  void InitWithData(base::span<uint8_t> data);

  void InitNull();

  base::span<uint8_t> data_;
  bool is_null_;
  bool is_detached_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_ARRAY_PIECE_H_
