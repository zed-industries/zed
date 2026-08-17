// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_FROZEN_ARRAY_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_FROZEN_ARRAY_BASE_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink::bindings {

// FrozenArrayBase is the common base class of all the IDL frozen array classes.
// Most importantly this class provides a way of type dispatching (e.g. overload
// resolutions, SFINAE technique, etc.) so that it's possible to distinguish
// IDL frozen array types from anything else. Also it provides a common
// implementation of IDL frozen array types.
//
// Note that NativeValueTraits<IDLArray<T>> does _not_ return FrozenArray<T>,
// it returns (Heap)Vector<T>, because it's convenient for Blink implementation
// because it's easy (and cheap when the move semantics is used) to convert a
// (Heap)Vector<T> to a FrozenArray<T> while the reverse conversion is not.
// ToV8Traits<IDLArray<T>> takes a FrozenArray<T> as normally.
class PLATFORM_EXPORT FrozenArrayBase : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~FrozenArrayBase() override = default;

  v8::Local<v8::Value> ToV8(ScriptState* script_state) const;
  v8::Local<v8::Value> ToV8(ScriptState* script_state);

  // ScriptWrappable overrides:
  v8::Local<v8::Value> Wrap(ScriptState* script_state) override;
  [[nodiscard]] v8::Local<v8::Object> AssociateWithWrapper(
      v8::Isolate* isolate,
      const WrapperTypeInfo* wrapper_type_info,
      v8::Local<v8::Object> wrapper) override;

 protected:
  FrozenArrayBase() = default;

  // `FrozenArrayBase::Wrap` implements the common part of the wrapper creation.
  // `FrozenArray<T>` overrides `MakeV8ArrayToBeFrozen` and implements the
  // type-T-dependent part of the wrapper creation (which doesn't include
  // "freeze").
  virtual v8::Local<v8::Value> MakeV8ArrayToBeFrozen(
      ScriptState* script_state) const = 0;
};

}  // namespace blink::bindings

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_FROZEN_ARRAY_BASE_H_
