// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_FROZEN_ARRAY_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_FROZEN_ARRAY_H_

#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/bindings/core/v8/to_v8_traits.h"
#include "third_party/blink/renderer/platform/bindings/frozen_array_base.h"
#include "third_party/blink/renderer/platform/heap/heap_traits.h"

namespace blink {

// FrozenArray<IDLType> implements IDL frozen array types. The instances of
// this class behave as an immutable object, and provide read-only access to
// the internal vector object.
//
// V8 wrapper objects for each world are created from the immutable internal
// vector object. It's guaranteed that, given a frozen_array,
//   ToV8Traits<IDLArray<IDLType>>::ToV8(script_state, frozen_array)
//   == ToV8Traits<IDLArray<IDLType>>::ToV8(script_state, frozen_array)
// i.e. repeated conversions from a Blink object to a V8 value return the same
// V8 value. (Note that IDLSequence<IDLType> doesn't support this equivalence.)
//
// About NativeValueTraits<FrozenArray<IDLType>>, see the class comment of
// `FrozenArrayBase`.
template <typename IDLType>
class FrozenArray final : public bindings::FrozenArrayBase {
 public:
  using VectorType = VectorOf<typename IDLTypeToBlinkImplType<IDLType>::type>;
  using size_type = typename VectorType::size_type;
  using value_type = typename VectorType::value_type;
  using const_reference = typename VectorType::const_reference;
  using const_pointer = typename VectorType::const_pointer;
  using const_iterator = typename VectorType::const_iterator;
  using const_reverse_iterator = typename VectorType::const_reverse_iterator;

  FrozenArray() = default;
  explicit FrozenArray(VectorType array) : array_(std::move(array)) {}
  ~FrozenArray() override = default;

  // Vector-compatible APIs
  size_type size() const { return array_.size(); }
  bool empty() const { return array_.empty(); }
  const_reference at(size_type index) const { return array_.at(index); }
  const_reference operator[](size_type index) const { return array_[index]; }
  const value_type* data() const { return array_.data(); }
  const_iterator begin() const { return array_.begin(); }
  const_iterator end() const { return array_.end(); }
  const_reverse_iterator rbegin() const { return array_.rbegin(); }
  const_reverse_iterator rend() const { return array_.rend(); }
  const_reference front() const { return array_.front(); }
  const_reference back() const { return array_.back(); }

  const VectorType& AsVector() const { return array_; }

  void Trace(Visitor* visitor) const override {
    FrozenArrayBase::Trace(visitor);
    TraceIfNeeded<VectorType>::Trace(visitor, array_);
  }

 protected:
  // FrozenArrayBase overrides:
  v8::Local<v8::Value> MakeV8ArrayToBeFrozen(
      ScriptState* script_state) const override {
    return ToV8Traits<IDLSequence<IDLType>>::ToV8(script_state, array_);
  }

 private:
  const VectorType array_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_FROZEN_ARRAY_H_
