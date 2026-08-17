// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_ARRAY_BUFFER_VIEW_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_ARRAY_BUFFER_VIEW_HELPERS_H_

#include <type_traits>

#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer_view.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace blink {

// A wrapper template type that is used to ensure that a TypedArray is not
// backed by a SharedArrayBuffer.  It is usable like a smart pointer.
//
//   void Foo(NotShared<DOMUint32Array> param) {
//     size_t length = param->length();
//     ...
//   }
template <typename T>
class NotShared {
  DISALLOW_NEW();
  static_assert(WTF::IsSubclass<typename std::remove_const<T>::type,
                                DOMArrayBufferView>::value,
                "NotShared<T> must have T as subclass of DOMArrayBufferView");

 public:
  using TypedArrayType = T;

  NotShared() = default;
  NotShared(const NotShared<T>& other) = default;
  // Allow implicit upcasts if U inherits from T.
  template <typename U, std::enable_if_t<std::is_base_of<T, U>::value, int> = 0>
  NotShared(const NotShared<U>& other) : typed_array_(other.Get()) {}

  explicit NotShared(std::nullptr_t) {}
  explicit NotShared(T* typed_array) : typed_array_(typed_array) {
    DCHECK(!typed_array || !typed_array->IsShared());
  }
  template <typename U>
  explicit NotShared(const Member<U>& other) : typed_array_(other.Get()) {
    DCHECK(!other || !other->IsShared());
  }

  NotShared& operator=(const NotShared& other) = default;
  template <typename U>
  NotShared& operator=(const NotShared<U>& other) {
    typed_array_ = static_cast<T*>(other.Get());
    return *this;
  }

  T* Get() const { return GetRaw(); }
  void Clear() { typed_array_ = nullptr; }

  // Returns true if this object represents IDL null.
  bool IsNull() const { return !GetRaw(); }

  explicit operator bool() const { return GetRaw(); }
  T* operator->() const { return GetRaw(); }
  T& operator*() const { return *GetRaw(); }

  void Trace(Visitor* visitor) const { visitor->Trace(typed_array_); }

 private:
  T* GetRaw() const { return typed_array_.Get(); }

  Member<T> typed_array_;
};

// A wrapper template type that specifies that a TypedArray *may* be backed by
// a SharedArrayBuffer.  It is usable like a smart pointer.
//
//   void Foo(MaybeShared<DOMUint32Array> param) {
//     DOMArrayBuffer* buffer = param->buffer();
//     ...
//   }
template <typename T>
class MaybeShared {
  DISALLOW_NEW();
  static_assert(WTF::IsSubclass<typename std::remove_const<T>::type,
                                DOMArrayBufferView>::value,
                "MaybeShared<T> must have T as subclass of DOMArrayBufferView");

 public:
  using TypedArrayType = T;

  MaybeShared() = default;
  MaybeShared(const MaybeShared& other) = default;
  // Allow implicit upcasts if U inherits from T.
  template <typename U, std::enable_if_t<std::is_base_of<T, U>::value, int> = 0>
  MaybeShared(const MaybeShared<U>& other) : typed_array_(other.Get()) {}

  explicit MaybeShared(std::nullptr_t) {}
  // [AllowShared] array buffer view may be a view of non-shared array buffer,
  // so we don't check if the buffer is SharedArrayBuffer or not.
  // https://webidl.spec.whatwg.org/#AllowShared
  explicit MaybeShared(T* typed_array) : typed_array_(typed_array) {}
  template <typename U>
  explicit MaybeShared(const Member<U>& other) : typed_array_(other.Get()) {}

  MaybeShared& operator=(const MaybeShared& other) = default;
  template <typename U>
  MaybeShared& operator=(const MaybeShared<U>& other) {
    typed_array_ = static_cast<T*>(other.Get());
    return *this;
  }

  T* Get() const { return GetRaw(); }
  void Clear() { typed_array_ = nullptr; }

  // Returns true if this object represents IDL null.
  bool IsNull() const { return !GetRaw(); }

  explicit operator bool() const { return GetRaw(); }
  T* operator->() const { return GetRaw(); }
  T& operator*() const { return *GetRaw(); }

  void Trace(Visitor* visitor) const { visitor->Trace(typed_array_); }

 private:
  T* GetRaw() const { return typed_array_.Get(); }

  Member<T> typed_array_;
};

}  // namespace blink

namespace WTF {

// NotShared<T> is essentially Member<T> from the perspective of HeapVector.
template <typename T>
struct VectorTraits<blink::NotShared<T>> : VectorTraits<blink::Member<T>> {};

// MaybeShared<T> is essentially Member<T> from the perspective of HeapVector.
template <typename T>
struct VectorTraits<blink::MaybeShared<T>> : VectorTraits<blink::Member<T>> {};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TYPED_ARRAYS_ARRAY_BUFFER_VIEW_HELPERS_H_
