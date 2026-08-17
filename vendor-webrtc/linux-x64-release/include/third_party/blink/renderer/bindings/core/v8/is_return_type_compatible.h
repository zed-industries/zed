// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_IS_RETURN_TYPE_COMPATIBLE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_IS_RETURN_TYPE_COMPATIBLE_H_

#include <optional>
#include <type_traits>

#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_data_view.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace blink {

class KURL;

namespace bindings {

class DictionaryBase;
class UnionBase;

// A fall-through case.
template <typename IDLType, typename ReturnType>
inline constexpr bool IsReturnTypeCompatible = false;

// Any type is compatible to IDLAny (duh!)
template <typename IDLType>
inline constexpr bool IsReturnTypeCompatible<IDLAny, IDLType> = true;

// Any type is compatible to itself.
template <typename SameType>
inline constexpr bool IsReturnTypeCompatible<SameType, SameType> = true;

namespace internal {

// In the normal case, we should not accept a function returning a "const
// ReturnType*" if our expectation from the IDL is that it returns
// "ReturnType*".  Exceptions below.
template <typename ReturnType>
inline constexpr bool CanRemoveCVQualifiers = false;

// Union types are really only meant for JS consumers.  However, they
// currently implement C++ const-ness behavior as though they're a struct,
// meaning that a const union type can return non-const instances of the types
// that it is a union of; it just can't be modified.  So const-ness of unions
// isn't meaningfully reflected in JS.
// TODO(dbaron) TODO(caseq): We should probably stop using const union types
// and remove this exception.
template <typename ReturnType>
  requires std::derived_from<ReturnType, UnionBase>
inline constexpr bool CanRemoveCVQualifiers<ReturnType> = true;

// Dictionary types are copied when converting, so it's fine if the C++
// returns a const dictionary even when the IDL doesn't reflect constness.
template <typename ReturnType>
  requires std::derived_from<ReturnType, DictionaryBase>
inline constexpr bool CanRemoveCVQualifiers<ReturnType> = true;

// This is like std::derived_from, except that std::derived_from always
// removes cv-qualifiers, whereas this only remove cv-qualifiers in the cases
// where we want to allow that removal.
template <class Derived, class Base>
concept DerivedFromAndIDLConvertible =
    std::is_base_of_v<Base, Derived> &&
    std::is_convertible_v<std::conditional_t<CanRemoveCVQualifiers<Derived>,
                                             std::remove_cv_t<Derived>*,
                                             Derived*>,
                          Base*>;

}  // namespace internal

// Returning a wider type is fine.
template <typename IDLType, typename ReturnType>
  requires internal::DerivedFromAndIDLConvertible<
               std::remove_pointer_t<ReturnType>,
               IDLType>
inline constexpr bool IsReturnTypeCompatible<IDLType, ReturnType> = true;

// Types that have null values must be used directly to represent nullables.
// This includes pointer types.
template <typename IDLType, typename ReturnType>
  requires(NativeValueTraits<IDLType>::has_null_value &&
           IsReturnTypeCompatible<IDLType, ReturnType>)
inline constexpr bool IsReturnTypeCompatible<IDLNullable<IDLType>, ReturnType> =
    true;

// std::optional<> is the way to implement nullable for types without null
// values.
template <typename IDLType, typename ReturnType>
  requires(!NativeValueTraits<IDLType>::has_null_value &&
           IsReturnTypeCompatible<IDLType, ReturnType>)
inline constexpr bool
    IsReturnTypeCompatible<IDLNullable<IDLType>, std::optional<ReturnType>> =
        true;

// Nullable sequence and array types can also be represented as pointers
// or with std::optional<> if their has_null_value is true.  (The case above
// accepts std::optional<> when has_null_value is false.)
// TODO(dbaron): We might consider removing the std::optional<> option here,
// since std::optional<HeapVector<>> is somewhat suspicious, and disallowed on
// the heap.
template <typename IDLType, typename ReturnType>
  requires(NativeValueTraits<IDLSequence<IDLType>>::has_null_value &&
           IsReturnTypeCompatible<IDLSequence<IDLType>,
                                  std::remove_cv_t<ReturnType>>)
inline constexpr bool
    IsReturnTypeCompatible<IDLNullable<IDLSequence<IDLType>>, ReturnType*> =
        true;

template <typename IDLType, typename ReturnType>
  requires(NativeValueTraits<IDLArray<IDLType>>::has_null_value &&
           IsReturnTypeCompatible<IDLArray<IDLType>,
                                  std::remove_cv_t<ReturnType>>)
inline constexpr bool
    IsReturnTypeCompatible<IDLNullable<IDLArray<IDLType>>, ReturnType*> = true;

template <typename IDLType, typename ReturnType>
  requires(NativeValueTraits<IDLSequence<IDLType>>::has_null_value &&
           IsReturnTypeCompatible<IDLSequence<IDLType>, ReturnType>)
inline constexpr bool IsReturnTypeCompatible<IDLNullable<IDLSequence<IDLType>>,
                                             std::optional<ReturnType>> = true;

template <typename IDLType, typename ReturnType>
  requires(NativeValueTraits<IDLArray<IDLType>>::has_null_value &&
           IsReturnTypeCompatible<IDLArray<IDLType>, ReturnType>)
inline constexpr bool IsReturnTypeCompatible<IDLNullable<IDLArray<IDLType>>,
                                             std::optional<ReturnType>> = true;

namespace internal {

// An helper similar in spirit to `std::remove_pointer<>`.
template <typename T>
struct RemoveMember {
  using type = T;
};

template <typename T>
struct RemoveMember<Member<T>> {
  using type = T;
};

}  // namespace internal

// Anything that looks like a range of compatible types is compatible to an
// IDLSequence.
template <typename IDLType, typename ReturnType>
  requires std::ranges::range<ReturnType> &&
               IsReturnTypeCompatible<
                   IDLType,
                   typename internal::RemoveMember<
                       std::ranges::range_value_t<ReturnType>>::type>
inline constexpr bool
    IsReturnTypeCompatible<blink::IDLSequence<IDLType>, ReturnType> = true;

// ... as well as to an IDLArray.
template <typename IDLType, typename ReturnType>
  requires std::ranges::range<ReturnType> &&
               IsReturnTypeCompatible<
                   IDLType,
                   typename internal::RemoveMember<
                       std::ranges::range_value_t<ReturnType>>::type>
inline constexpr bool
    IsReturnTypeCompatible<blink::IDLArray<IDLType>, ReturnType> = true;

// IDL types derived from IDLBaseHelper already have associated impl type,
// so just use that.
template <typename IDLType, typename ReturnType>
  requires internal::DerivedFromAndIDLConvertible<IDLType,
                                                  IDLBaseHelper<ReturnType>>
inline constexpr bool IsReturnTypeCompatible<IDLType, ReturnType> = true;

// TODO(caseq): this is an exception as it loses type checks, remove and take
// care of broken call sites!
template <typename IDLType>
inline constexpr bool IsReturnTypeCompatible<blink::IDLArray<IDLType>,
                                             v8::LocalVector<v8::Value>> = true;

// Just ignore NotShared when checking types.
template <typename ReturnType>
  requires std::derived_from<std::remove_pointer_t<ReturnType>,
                             DOMArrayBufferView>
inline constexpr bool
    IsReturnTypeCompatible<blink::NotShared<std::remove_pointer_t<ReturnType>>,
                           ReturnType> = true;

// IDLRecords are compatible to arrays of pairs, as long as key/value types
// are compatible between records and pairs.
template <typename IDLKey,
          typename IDLValue,
          typename ReturnKey,
          typename ReturnValue>
inline constexpr bool
    IsReturnTypeCompatible<blink::IDLRecord<IDLKey, IDLValue>,
                           WTF::Vector<std::pair<ReturnKey, ReturnValue>>> =
        IsReturnTypeCompatible<IDLKey, ReturnKey> &&
        IsReturnTypeCompatible<IDLValue, ReturnValue>;

template <typename IDLKey,
          typename IDLValue,
          typename ReturnKey,
          typename ReturnValue>
inline constexpr bool IsReturnTypeCompatible<
    blink::IDLRecord<IDLKey, IDLValue>,
    HeapVector<std::pair<ReturnKey, Member<ReturnValue>>>> =
    IsReturnTypeCompatible<IDLKey, ReturnKey> &&
    IsReturnTypeCompatible<IDLValue, ReturnValue>;

template <>
inline constexpr bool IsReturnTypeCompatible<IDLObject, v8::Local<v8::Object>> =
    true;

// TODO(caseq): this shouldn't really be allowed, as v8::Value may carry
// values that are not objects, but keep it for now.
template <>
inline constexpr bool IsReturnTypeCompatible<IDLObject, v8::Local<v8::Value>> =
    true;

// Any IDL strings are compatible to any blink strings.
template <typename IDLStringType>
  requires std::derived_from<IDLStringType, IDLStringTypeBase>
inline constexpr bool IsReturnTypeCompatible<IDLStringType, WTF::String> = true;

template <typename IDLStringType>
  requires std::derived_from<IDLStringType, IDLStringTypeBase>
inline constexpr bool IsReturnTypeCompatible<IDLStringType, WTF::AtomicString> =
    true;

// TODO(caseq): note we do not differentiate between double and float here. Not
// sure if there's a point considering representation would be the same on the
// JS side.
template <typename FloatType,
          blink::bindings::IDLFloatingPointNumberConvMode conv_mode>
inline constexpr bool IsReturnTypeCompatible<
    blink::IDLFloatingPointNumberTypeBase<FloatType, conv_mode>,
    float> = true;

template <typename FloatType,
          blink::bindings::IDLFloatingPointNumberConvMode conv_mode>
inline constexpr bool IsReturnTypeCompatible<
    blink::IDLFloatingPointNumberTypeBase<FloatType, conv_mode>,
    double> = true;

// Be relaxed about int types for now.
template <typename IDLIntType,
          typename NativeIntType,
          blink::bindings::IDLIntegerConvMode conv_mode>
inline constexpr bool
    IsReturnTypeCompatible<blink::IDLIntegerTypeBase<IDLIntType, conv_mode>,
                           NativeIntType> =
        std::is_convertible<NativeIntType, IDLIntType>::value ||
        std::is_enum<NativeIntType>::value;

// TODO(caseq): should we restrict KURLs to strings of particular type? Forbid
// implicit conversion altogether?
template <typename IDLStringType>
  requires std::derived_from<IDLStringType, IDLStringTypeBase>
inline constexpr bool IsReturnTypeCompatible<IDLStringType, blink::KURL> = true;

}  // namespace bindings
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_IS_RETURN_TYPE_COMPATIBLE_H_
