// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_NATIVE_VALUE_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_NATIVE_VALUE_TRAITS_H_

#include <concepts>
#include <type_traits>

#include "third_party/blink/renderer/bindings/core/v8/idl_types_base.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;

// Primary template for NativeValueTraits. It is not supposed to be used
// directly: there needs to be a specialization for each type which represents
// a JavaScript type that will be converted to a C++ representation.
// Its main goal is to provide a standard interface for converting JS types
// into C++ ones.
//
// Example:
// template <>
// struct NativeValueTraits<IDLLong> : public NativeValueTraitsBase<IDLLong> {
//   static inline int32_t NativeValue(v8::Isolate* isolate,
//                                     v8::Local<v8::Value> value,
//                                     ExceptionState& exceptionState) {
//     return toInt32(isolate, value, exceptionState, NormalConversion);
//   }
// }
template <typename T>
struct NativeValueTraits;

// This declaration serves only as a blueprint for specializations: the
// return type can change, but all specializations are expected to provide a
// NativeValue() method that takes the 3 arguments below.
//
// template <>
// struct NativeValueTraits<T> : public NativeValueTraitsBase<T> {
//   static inline typename NativeValueTraitsBase<T>::ImplType
//   NativeValue(v8::Isolate*, v8::Local<v8::Value>, ExceptionState&);
// };

namespace bindings {

template <typename T>
struct ImplTypeFor {
  using type = T;
};

template <typename T>
  requires std::derived_from<T, IDLBase>
struct ImplTypeFor<T> {
  using type = typename T::ImplType;
};

}  // namespace bindings

// NativeValueTraitsBase is supposed to be inherited by NativeValueTraits
// classes. They serve as a way to hold the ImplType typedef without requiring
// all NativeValueTraits specializations to declare it.
//
// The primary template below is used by NativeValueTraits specializations with
// types that do not inherit from IDLBase, in which case it is assumed the type
// of the specialization is also |ImplType|. The NativeValueTraitsBase
// specialization is used for IDLBase-based types, which are supposed to have
// their own |ImplType| typedefs.
//
// If present, |NullValue()| will be used when converting from the nullable type
// T?, and should be used if the impl type has an existing "null" state. If not
// present, WTF::Optional will be used to wrap the type.
template <typename T>
struct NativeValueTraitsBase {
  STATIC_ONLY(NativeValueTraitsBase);

  using ImplType = bindings::ImplTypeFor<T>::type;

  // Pointer types have nullptr as IDL null value.
  // ScriptValue, String, and union types have IsNull member function.
  static constexpr bool has_null_value =
      std::is_pointer_v<ImplType> ||
      requires(ImplType value) { value.IsNull(); };

  // This should only be true for certain subclasses of ScriptWrappable
  // that satisfy the assumptions of CreateIDLSequenceFromV8ArraySlow() with
  // regards to how NativeValue() is implemented for the underlying type.
  static constexpr bool supports_scriptwrappable_specific_fast_array_iteration =
      false;

  template <typename... ExtraArgs>
  static decltype(auto) ArgumentValue(v8::Isolate* isolate,
                                      int argument_index,
                                      v8::Local<v8::Value> value,
                                      ExceptionState& exception_state,
                                      ExtraArgs... extra_args) {
    return NativeValueTraits<std::remove_pointer_t<T>>::NativeValue(
        isolate, value, exception_state,
        std::forward<ExtraArgs>(extra_args)...);
  }
};

template <class T>
concept TypeHasAnyNamedPropertiesMethod =
    requires(T a) { a.HasAnyNamedProperties(); };

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_NATIVE_VALUE_TRAITS_H_
