// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_TYPED_ARRAY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_TYPED_ARRAY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer_view.h"
#include "v8/include/v8.h"

namespace blink {

template <typename T, typename V8TypedArray, bool clamped = false>
class DOMTypedArray final : public DOMArrayBufferView {
  typedef DOMTypedArray<T, V8TypedArray, clamped> ThisType;
  DEFINE_WRAPPERTYPEINFO();
  static const WrapperTypeInfo wrapper_type_info_body_;

 public:
  using ValueType = T;

  static ThisType* Create(DOMArrayBufferBase* buffer,
                          size_t byte_offset,
                          size_t length) {
    return MakeGarbageCollected<ThisType>(buffer, byte_offset, length);
  }

  static ThisType* Create(size_t length) {
    DOMArrayBuffer* buffer = DOMArrayBuffer::Create(length, sizeof(ValueType));
    return Create(buffer, 0, length);
  }

  static ThisType* Create(base::span<const ValueType> array)
    requires std::is_trivially_copyable_v<ValueType>
  {
    // Intentionally avoids using `as_bytes`, since that requires
    // `std::has_unique_object_representations_v<ValueType>`, which we neither
    // need here nor can guarantee.
    DOMArrayBuffer* buffer =
        DOMArrayBuffer::Create(array.data(), array.size_bytes());
    return Create(buffer, 0, array.size());
  }

  static ThisType* CreateOrNull(size_t length) {
    DOMArrayBuffer* buffer =
        DOMArrayBuffer::CreateOrNull(length, sizeof(ValueType));
    return buffer ? Create(buffer, 0, length) : nullptr;
  }

  static ThisType* CreateOrNull(base::span<const ValueType> array)
    requires std::is_trivially_copyable_v<ValueType>
  {
    // Intentionally avoids using `as_bytes`, since that requires
    // `std::has_unique_object_representations_v<ValueType>`, which we neither
    // need here nor can guarantee.
    DOMArrayBuffer* buffer =
        DOMArrayBuffer::CreateOrNull(array.data(), array.size_bytes());
    return buffer ? Create(buffer, 0, array.size()) : nullptr;
  }

  static ThisType* CreateUninitializedOrNull(size_t length) {
    DOMArrayBuffer* buffer =
        DOMArrayBuffer::CreateUninitializedOrNull(length, sizeof(ValueType));
    return buffer ? Create(buffer, 0, length) : nullptr;
  }

  DOMTypedArray(DOMArrayBufferBase* dom_array_buffer,
                size_t byte_offset,
                size_t length)
      : DOMArrayBufferView(dom_array_buffer, byte_offset), raw_length_(length) {
    CHECK(VerifySubRange(dom_array_buffer, byte_offset, length));
  }

  ValueType* Data() const { return static_cast<ValueType*>(BaseAddress()); }

  ValueType* DataMaybeShared() const {
    return reinterpret_cast<ValueType*>(BaseAddressMaybeShared());
  }

  base::span<ValueType> AsSpan() const {
    // SAFETY: Data() and length() guarantee the span is valid
    return UNSAFE_BUFFERS(
        base::span(static_cast<ValueType*>(Data()), length()));
  }

  base::span<ValueType> AsSpanMaybeShared() const {
    // SAFETY: DataMaybeShared() and length() guarantee the span is valid
    return UNSAFE_BUFFERS(
        base::span(static_cast<ValueType*>(DataMaybeShared()), length()));
  }

  size_t length() const { return !IsDetached() ? raw_length_ : 0; }

  size_t byteLength() const final { return length() * sizeof(ValueType); }

  unsigned TypeSize() const final { return sizeof(ValueType); }

  DOMArrayBufferView::ViewType GetType() const override;

  // Invoked by the indexed getter. Does not perform range checks; caller
  // is responsible for doing so and returning undefined as necessary.
  ValueType Item(size_t index) const { return AsSpan()[index]; }

  v8::Local<v8::Value> Wrap(ScriptState*) override;

 private:
  // Helper to verify that a given sub-range of an ArrayBuffer is within range.
  static bool VerifySubRange(const DOMArrayBufferBase* buffer,
                             size_t byte_offset,
                             size_t num_elements) {
    if (!buffer)
      return false;
    if (sizeof(T) > 1 && byte_offset % sizeof(T))
      return false;
    if (byte_offset > buffer->ByteLength())
      return false;
    size_t remaining_elements =
        (buffer->ByteLength() - byte_offset) / sizeof(T);
    if (num_elements > remaining_elements)
      return false;
    return true;
  }

  // It may be stale after Detach. Use length() instead.
  size_t raw_length_;
};

#define DOMTYPEDARRAY_FOREACH_VIEW_TYPE(V) \
  V(int8_t, Int8, false)                   \
  V(int16_t, Int16, false)                 \
  V(int32_t, Int32, false)                 \
  V(uint8_t, Uint8, false)                 \
  V(uint8_t, Uint8Clamped, true)           \
  V(uint16_t, Uint16, false)               \
  V(uint32_t, Uint32, false)               \
  V(uint16_t, Float16, false)              \
  V(float, Float32, false)                 \
  V(double, Float64, false)                \
  V(int64_t, BigInt64, false)              \
  V(uint64_t, BigUint64, false)

#define DOMTYPEDARRAY_DECLARE_WRAPPERTYPEINFO(val_t, Type, clamped)            \
  template <>                                                                  \
  const WrapperTypeInfo                                                        \
      DOMTypedArray<val_t, v8::Type##Array, clamped>::wrapper_type_info_body_; \
  template <>                                                                  \
  const WrapperTypeInfo&                                                       \
      DOMTypedArray<val_t, v8::Type##Array, clamped>::wrapper_type_info_;
DOMTYPEDARRAY_FOREACH_VIEW_TYPE(DOMTYPEDARRAY_DECLARE_WRAPPERTYPEINFO)
#undef DOMTYPEDARRAY_DECLARE_WRAPPERTYPEINFO

#define DOMTYPEDARRAY_DEFINE_GETTYPE(val_t, Type, clamped)          \
  template <>                                                       \
  inline DOMArrayBufferView::ViewType                               \
  DOMTypedArray<val_t, v8::Type##Array, clamped>::GetType() const { \
    return DOMArrayBufferView::kType##Type;                         \
  }
DOMTYPEDARRAY_FOREACH_VIEW_TYPE(DOMTYPEDARRAY_DEFINE_GETTYPE)
#undef DOMTYPEDARRAY_DEFINE_GETTYPE

#define DOMTYPEDARRAY_DECLARE_EXTERN_TEMPLATE(val_t, Type, clamped) \
  extern template class CORE_EXTERN_TEMPLATE_EXPORT                 \
      DOMTypedArray<val_t, v8::Type##Array, clamped>;
DOMTYPEDARRAY_FOREACH_VIEW_TYPE(DOMTYPEDARRAY_DECLARE_EXTERN_TEMPLATE)
#undef DOMTYPEDARRAY_DECLARE_EXTERN_TEMPLATE

#define DOMTYPEDARRAY_DEFINE_TYPEDEFNAME(val_t, Type, clamped) \
  using DOM##Type##Array = DOMTypedArray<val_t, v8::Type##Array, clamped>;
DOMTYPEDARRAY_FOREACH_VIEW_TYPE(DOMTYPEDARRAY_DEFINE_TYPEDEFNAME)
#undef DOMTYPEDARRAY_DEFINE_TYPEDEFNAME

#undef DOMTYPEDARRAY_FOREACH_VIEW_TYPE

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_DOM_TYPED_ARRAY_H_
