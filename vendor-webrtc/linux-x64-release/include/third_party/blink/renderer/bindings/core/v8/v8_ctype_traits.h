// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_CTYPE_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_CTYPE_TRAITS_H_

#include <type_traits>

#include "third_party/blink/renderer/bindings/core/v8/idl_types_base.h"
#include "v8/include/v8-fast-api-calls.h"

namespace blink {

template <typename T>
class V8CTypeTraits;

template <>
class V8CTypeTraits<IDLBoolean> {
 public:
  static constexpr v8::CTypeInfo kCTypeInfo =
      v8::CTypeInfoBuilder<bool>().Build();
};

template <typename T, bindings::IDLFloatingPointNumberConvMode mode>
class V8CTypeTraits<IDLFloatingPointNumberTypeBase<T, mode>> {
  static constexpr v8::CTypeInfo::Flags GetCTypeInfoFlags() {
    switch (mode) {
      case bindings::IDLFloatingPointNumberConvMode::kDefault:
        return v8::CTypeInfo::Flags::kIsRestrictedBit;
      case bindings::IDLFloatingPointNumberConvMode::kUnrestricted:
        return v8::CTypeInfo::Flags::kNone;
    }
  }

 public:
  static constexpr v8::CTypeInfo kCTypeInfo =
      v8::CTypeInfoBuilder<T, GetCTypeInfoFlags()>().Build();
};

template <typename T, bindings::IDLIntegerConvMode mode>
class V8CTypeTraits<IDLIntegerTypeBase<T, mode>> {
  static constexpr v8::CTypeInfo::Flags GetCTypeInfoFlags() {
    switch (mode) {
      case bindings::IDLIntegerConvMode::kDefault:
        return v8::CTypeInfo::Flags::kNone;
      case bindings::IDLIntegerConvMode::kClamp:
        return v8::CTypeInfo::Flags::kClampBit;
      case bindings::IDLIntegerConvMode::kEnforceRange:
        return v8::CTypeInfo::Flags::kEnforceRangeBit;
    }
  }

 public:
  static constexpr v8::CTypeInfo kCTypeInfo =
      v8::CTypeInfoBuilder<T, GetCTypeInfoFlags()>().Build();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_CTYPE_TRAITS_H_
