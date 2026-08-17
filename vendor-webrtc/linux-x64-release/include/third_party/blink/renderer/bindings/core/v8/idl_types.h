// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_IDL_TYPES_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_IDL_TYPES_H_

#include <optional>
#include <type_traits>

#include "base/time/time.h"
#include "third_party/blink/renderer/bindings/core/v8/idl_types_base.h"
#include "third_party/blink/renderer/bindings/core/v8/native_value_traits.h"
#include "third_party/blink/renderer/platform/heap/heap_traits.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class BigInt;
class EventListener;
class ScriptObject;
template <typename T>
class MemberScriptPromise;
template <typename T>
class ScriptPromise;
class ScriptValue;
struct ToV8UndefinedGenerator;

// The type names below are named as "IDL" prefix + Web IDL type name.
// https://webidl.spec.whatwg.org/#dfn-type-name
// undefined
// TODO(japhet): Use IDLUndefined in place of ToV8UndefinedGenerator and delete
// ToV8UndefinedGenerator. Using IDLUndefined here makes calls to
// ScriptPromseResolver::Resolve/Reject ambiguous between the ToV8() variant
// and the ToV8Traits<>::ToV8() variant of those functions.
struct IDLUndefined final : public IDLBaseHelper<ToV8UndefinedGenerator> {};

// any
struct IDLAny final : public IDLBaseHelper<ScriptValue> {};

// boolean
struct IDLBoolean final : public IDLBaseHelper<bool> {};

// bigint
struct IDLBigint final : public IDLBaseHelper<BigInt> {};

// Integer types

namespace bindings {

enum class IDLIntegerConvMode {
  kDefault,
  kClamp,
  kEnforceRange,
};

}  // namespace bindings

template <typename T,
          bindings::IDLIntegerConvMode mode =
              bindings::IDLIntegerConvMode::kDefault>
struct IDLIntegerTypeBase final : public IDLBaseHelper<T> {};

// Integers
using IDLByte = IDLIntegerTypeBase<int8_t>;
using IDLOctet = IDLIntegerTypeBase<uint8_t>;
using IDLShort = IDLIntegerTypeBase<int16_t>;
using IDLUnsignedShort = IDLIntegerTypeBase<uint16_t>;
using IDLLong = IDLIntegerTypeBase<int32_t>;
using IDLUnsignedLong = IDLIntegerTypeBase<uint32_t>;
using IDLLongLong = IDLIntegerTypeBase<int64_t>;
using IDLUnsignedLongLong = IDLIntegerTypeBase<uint64_t>;

// [Clamp] Integers
using IDLByteClamp =
    IDLIntegerTypeBase<int8_t, bindings::IDLIntegerConvMode::kClamp>;
using IDLOctetClamp =
    IDLIntegerTypeBase<uint8_t, bindings::IDLIntegerConvMode::kClamp>;
using IDLShortClamp =
    IDLIntegerTypeBase<int16_t, bindings::IDLIntegerConvMode::kClamp>;
using IDLUnsignedShortClamp =
    IDLIntegerTypeBase<uint16_t, bindings::IDLIntegerConvMode::kClamp>;
using IDLLongClamp =
    IDLIntegerTypeBase<int32_t, bindings::IDLIntegerConvMode::kClamp>;
using IDLUnsignedLongClamp =
    IDLIntegerTypeBase<uint32_t, bindings::IDLIntegerConvMode::kClamp>;
using IDLLongLongClamp =
    IDLIntegerTypeBase<int64_t, bindings::IDLIntegerConvMode::kClamp>;
using IDLUnsignedLongLongClamp =
    IDLIntegerTypeBase<uint64_t, bindings::IDLIntegerConvMode::kClamp>;

// [EnforceRange] Integers
using IDLByteEnforceRange =
    IDLIntegerTypeBase<int8_t, bindings::IDLIntegerConvMode::kEnforceRange>;
using IDLOctetEnforceRange =
    IDLIntegerTypeBase<uint8_t, bindings::IDLIntegerConvMode::kEnforceRange>;
using IDLShortEnforceRange =
    IDLIntegerTypeBase<int16_t, bindings::IDLIntegerConvMode::kEnforceRange>;
using IDLUnsignedShortEnforceRange =
    IDLIntegerTypeBase<uint16_t, bindings::IDLIntegerConvMode::kEnforceRange>;
using IDLLongEnforceRange =
    IDLIntegerTypeBase<int32_t, bindings::IDLIntegerConvMode::kEnforceRange>;
using IDLUnsignedLongEnforceRange =
    IDLIntegerTypeBase<uint32_t, bindings::IDLIntegerConvMode::kEnforceRange>;
using IDLLongLongEnforceRange =
    IDLIntegerTypeBase<int64_t, bindings::IDLIntegerConvMode::kEnforceRange>;
using IDLUnsignedLongLongEnforceRange =
    IDLIntegerTypeBase<uint64_t, bindings::IDLIntegerConvMode::kEnforceRange>;

// Floating point number types

namespace bindings {

enum class IDLFloatingPointNumberConvMode {
  kDefault,
  kUnrestricted,
};

}  // namespace bindings

template <typename T,
          bindings::IDLFloatingPointNumberConvMode mode =
              bindings::IDLFloatingPointNumberConvMode::kDefault>
struct IDLFloatingPointNumberTypeBase final : public IDLBaseHelper<T> {};

// float
using IDLFloat = IDLFloatingPointNumberTypeBase<float>;
using IDLUnrestrictedFloat = IDLFloatingPointNumberTypeBase<
    float,
    bindings::IDLFloatingPointNumberConvMode::kUnrestricted>;

// double
using IDLDouble = IDLFloatingPointNumberTypeBase<double>;
using IDLUnrestrictedDouble = IDLFloatingPointNumberTypeBase<
    double,
    bindings::IDLFloatingPointNumberConvMode::kUnrestricted>;

// DOMHighResTimeStamp
// https://w3c.github.io/hr-time/#sec-domhighrestimestamp
using IDLDOMHighResTimeStamp = IDLDouble;

// Strings

namespace bindings {

enum class IDLStringConvMode {
  kDefault,
  kNullable,
  kLegacyNullToEmptyString,
};

}  // namespace bindings

// Base class for IDL string types (except for enumeration types)
struct IDLStringTypeBase : public IDLBaseHelper<String> {};

// ByteString
template <bindings::IDLStringConvMode mode>
struct IDLByteStringBase final : public IDLStringTypeBase {};
using IDLByteString = IDLByteStringBase<bindings::IDLStringConvMode::kDefault>;

// DOMString
template <bindings::IDLStringConvMode mode>
struct IDLStringBase final : public IDLStringTypeBase {};
using IDLString = IDLStringBase<bindings::IDLStringConvMode::kDefault>;
using IDLStringLegacyNullToEmptyString =
    IDLStringBase<bindings::IDLStringConvMode::kLegacyNullToEmptyString>;

// USVString
template <bindings::IDLStringConvMode mode>
struct IDLUSVStringBase final : public IDLStringTypeBase {};
using IDLUSVString = IDLUSVStringBase<bindings::IDLStringConvMode::kDefault>;

// [StringContext=TrustedHTML] DOMString
template <bindings::IDLStringConvMode mode>
struct IDLStringStringContextTrustedHTMLBase final : public IDLStringTypeBase {
};
using IDLStringStringContextTrustedHTML = IDLStringStringContextTrustedHTMLBase<
    bindings::IDLStringConvMode::kDefault>;
using IDLStringLegacyNullToEmptyStringStringContextTrustedHTML =
    IDLStringStringContextTrustedHTMLBase<
        bindings::IDLStringConvMode::kLegacyNullToEmptyString>;

// [StringContext=TrustedScript] DOMString
template <bindings::IDLStringConvMode mode>
struct IDLStringStringContextTrustedScriptBase final
    : public IDLStringTypeBase {};
using IDLStringStringContextTrustedScript =
    IDLStringStringContextTrustedScriptBase<
        bindings::IDLStringConvMode::kDefault>;
using IDLStringLegacyNullToEmptyStringStringContextTrustedScript =
    IDLStringStringContextTrustedScriptBase<
        bindings::IDLStringConvMode::kLegacyNullToEmptyString>;

// [StringContext=TrustedScriptURL] USVString
template <bindings::IDLStringConvMode mode>
struct IDLUSVStringStringContextTrustedScriptURLBase final
    : public IDLStringTypeBase {};
using IDLUSVStringStringContextTrustedScriptURL =
    IDLUSVStringStringContextTrustedScriptURLBase<
        bindings::IDLStringConvMode::kDefault>;

// object
struct IDLObject final : public IDLBaseHelper<ScriptObject> {};

// Promise types
template <typename T>
struct IDLPromise final : public IDLBaseHelper<ScriptPromise<T>> {};

template <typename T>
struct IsPromiseType {
  static constexpr bool value = false;
  using IDLPromiseResultType = void;
};

template <typename T>
struct IsPromiseType<IDLPromise<T>> {
  static constexpr bool value = true;
  using IDLPromiseResultType = T;
};

// Sequence types
template <typename T>
struct IDLSequence final : public IDLBase {
  using ImplType =
      VectorOf<std::remove_pointer_t<typename NativeValueTraits<T>::ImplType>>;
};

// Frozen array types
template <typename T>
struct IDLArray final : public IDLBase {
  // IDL FrozenArray is implemented as FrozenArray<IDLType>, but it's convenient
  // for NativeValueTraits<IDLArray<T>> to use (Heap)Vector<T>. Generally, for
  // inputs (attribute setters, operation arguments), (Heap)Vector is convenient
  // while FrozenArray<IDLType> should be used for outputs (attribute getters,
  // operation return values).
  //
  // Since IDLType is tightly bound to NativeValueTraits rather than ToV8Traits,
  // IDLArray<T>::ImplType is defined as (Heap)Vector<T> rather than
  // FrozenArray<T>.
  using ImplType =
      VectorOf<std::remove_pointer_t<typename NativeValueTraits<T>::ImplType>>;
};

// Record types
template <typename Key, typename Value>
struct IDLRecord final : public IDLBase {
  static_assert(std::is_same<typename Key::ImplType, String>::value,
                "IDLRecord keys must be of a WebIDL string type");
  static_assert(
      std::is_same<typename NativeValueTraits<Key>::ImplType, String>::value,
      "IDLRecord keys must be of a WebIDL string type");

  using ValueImplType = std::conditional_t<
      IsPromiseType<Value>::value,
      MemberScriptPromise<typename IsPromiseType<Value>::IDLPromiseResultType>,
      typename NativeValueTraits<Value>::ImplType>;

  using ImplType = VectorOfPairs<String, std::remove_pointer_t<ValueImplType>>;
};

// Nullable types
template <typename T>
struct IDLNullable final : public IDLBase {
  using ImplType = std::conditional_t<
      NativeValueTraits<T>::has_null_value,
      typename NativeValueTraits<T>::ImplType,
      std::optional<typename NativeValueTraits<T>::ImplType>>;
};

// EventHandler types
struct IDLEventHandler final : public IDLBaseHelper<EventListener*> {};
struct IDLOnBeforeUnloadEventHandler final
    : public IDLBaseHelper<EventListener*> {};
struct IDLOnErrorEventHandler final : public IDLBaseHelper<EventListener*> {};

// [BufferSourceTypeNoSizeLimit]
template <typename T>
struct IDLBufferSourceTypeNoSizeLimit {};

// [AllowResizable]
template <typename T>
struct IDLAllowResizable {};

// IDL optional types
//
// IDLOptional represents an optional argument and supports a conversion from
// ES undefined to "missing" special value.  The "missing" value might be
// represented in Blink as std::nullopt, nullptr, 0, etc. depending on a Blink
// type.
//
// Note that IDLOptional is not meant to represent an optional dictionary
// member.
template <typename T>
struct IDLOptional final : public IDLBase {
  using ImplType = typename NativeValueTraits<T>::ImplType;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_IDL_TYPES_H_
