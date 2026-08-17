// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_ARRAY_BUFFER_VIEW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_ARRAY_BUFFER_VIEW_H_

#include "base/containers/span.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_shared_array_buffer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class CORE_EXPORT DOMArrayBufferView : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();
  static const WrapperTypeInfo wrapper_type_info_body_;

 public:
  enum ViewType {
    kTypeInt8,
    kTypeUint8,
    kTypeUint8Clamped,
    kTypeInt16,
    kTypeUint16,
    kTypeInt32,
    kTypeUint32,
    kTypeFloat16,
    kTypeFloat32,
    kTypeFloat64,
    kTypeBigInt64,
    kTypeBigUint64,
    kTypeDataView
  };

  ~DOMArrayBufferView() override = default;

  DOMArrayBuffer* buffer() const {
    DCHECK(!IsShared());
    DCHECK(dom_array_buffer_);
    return static_cast<DOMArrayBuffer*>(dom_array_buffer_.Get());
  }

  DOMSharedArrayBuffer* BufferShared() const {
    DCHECK(IsShared());
    DCHECK(dom_array_buffer_);
    return static_cast<DOMSharedArrayBuffer*>(dom_array_buffer_.Get());
  }

  DOMArrayBufferBase* BufferBase() const {
    if (IsShared())
      return BufferShared();

    return buffer();
  }

  virtual ViewType GetType() const = 0;

  const char* TypeName() {
    switch (GetType()) {
      case kTypeInt8:
        return "Int8";
      case kTypeUint8:
        return "UInt8";
      case kTypeUint8Clamped:
        return "UInt8Clamped";
      case kTypeInt16:
        return "Int16";
      case kTypeUint16:
        return "UInt16";
      case kTypeInt32:
        return "Int32";
      case kTypeUint32:
        return "Uint32";
      case kTypeBigInt64:
        return "BigInt64";
      case kTypeBigUint64:
        return "BigUint64";
      case kTypeFloat16:
        return "Float16";
      case kTypeFloat32:
        return "Float32";
      case kTypeFloat64:
        return "Float64";
      case kTypeDataView:
        return "DataView";
    }
  }

  void* BaseAddress() const {
    DCHECK(!IsShared());
    return BaseAddressMaybeShared();
  }

  size_t byteOffset() const { return !IsDetached() ? raw_byte_offset_ : 0; }

  // Must return the number of valid bytes at `BaseAddress()`.
  virtual size_t byteLength() const = 0;

  base::span<uint8_t> ByteSpan() const {
    // SAFETY: `byteLength()` returns the number of bytes at `BaseAddress()`.
    return UNSAFE_BUFFERS(
        base::span(static_cast<uint8_t*>(BaseAddress()), byteLength()));
  }

  virtual unsigned TypeSize() const = 0;
  bool IsShared() const { return dom_array_buffer_->IsShared(); }

  void* BaseAddressMaybeShared() const {
    return !IsDetached() ? raw_base_address_ : nullptr;
  }

  base::span<uint8_t> ByteSpanMaybeShared() const {
    // SAFETY: `byteLength()` returns the number of bytes at `BaseAddress()`.
    return UNSAFE_BUFFERS(base::span(
        static_cast<uint8_t*>(BaseAddressMaybeShared()), byteLength()));
  }

  // ScriptWrappable overrides:
  v8::Local<v8::Value> Wrap(ScriptState*) override { NOTREACHED(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(dom_array_buffer_);
    ScriptWrappable::Trace(visitor);
  }

  void DetachForTesting() { dom_array_buffer_->Detach(); }

  bool IsDetached() const { return dom_array_buffer_->IsDetached(); }

 protected:
  DOMArrayBufferView(DOMArrayBufferBase* dom_array_buffer, size_t byte_offset)
      : raw_byte_offset_(byte_offset), dom_array_buffer_(dom_array_buffer) {
    DCHECK(dom_array_buffer_);
    // SAFETY: It is the subclasses' responsibility to ensure that the
    // invariants here are maintained.
    raw_base_address_ = UNSAFE_BUFFERS(
        static_cast<char*>(dom_array_buffer_->DataMaybeShared()) + byte_offset);
  }

 private:
  // The raw_* fields may be stale after Detach. Use getters instead.
  // This is the address of the ArrayBuffer's storage, plus the byte offset.
  void* raw_base_address_;
  size_t raw_byte_offset_;

  mutable Member<DOMArrayBufferBase> dom_array_buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_ARRAY_BUFFER_VIEW_H_
