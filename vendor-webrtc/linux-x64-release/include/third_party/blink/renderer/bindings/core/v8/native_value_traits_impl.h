// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_NATIVE_VALUE_TRAITS_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_NATIVE_VALUE_TRAITS_IMPL_H_

#include <concepts>
#include <optional>
#include <type_traits>

#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/bindings/core/v8/native_value_traits.h"
#include "third_party/blink/renderer/bindings/core/v8/pass_as_span.h"
#include "third_party/blink/renderer/bindings/core/v8/script_iterator.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_core.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_trusted_html.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_trusted_script.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_trusted_script_url.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/trustedtypes/trusted_html.h"
#include "third_party/blink/renderer/core/trustedtypes/trusted_script.h"
#include "third_party/blink/renderer/core/trustedtypes/trusted_script_url.h"
#include "third_party/blink/renderer/core/trustedtypes/trusted_types_util.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_data_view.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/platform/bindings/bigint.h"
#include "third_party/blink/renderer/platform/bindings/exception_messages.h"
#include "third_party/blink/renderer/platform/heap/heap_traits.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-fast-api-calls.h"

namespace blink {

class CallbackFunctionBase;
class CallbackInterfaceBase;
class EventListener;
class GPUColorTargetState;
class GPURenderPassColorAttachment;
class GPUVertexBufferLayout;
class ScriptWrappable;
struct WrapperTypeInfo;

struct ToV8UndefinedGenerator {
  DISALLOW_NEW();
  using ImplType = ToV8UndefinedGenerator;
};

namespace bindings {

class EnumerationBase;
class InputDictionaryBase;
class UnionBase;
CORE_EXPORT void NativeValueTraitsInterfaceNotOfType(
    const WrapperTypeInfo* wrapper_type_info,
    ExceptionState& exception_state);

CORE_EXPORT void NativeValueTraitsInterfaceNotOfType(
    const WrapperTypeInfo* wrapper_type_info,
    int argument_index,
    ExceptionState& exception_state);

// Class created for IDLAny types. Converts to either ScriptValue or
// v8::Local<v8::Value>.
class CORE_EXPORT NativeValueTraitsAnyAdapter {
  STACK_ALLOCATED();

 public:
  NativeValueTraitsAnyAdapter() = default;
  NativeValueTraitsAnyAdapter(const NativeValueTraitsAnyAdapter&) = delete;
  NativeValueTraitsAnyAdapter(NativeValueTraitsAnyAdapter&&) = default;
  explicit NativeValueTraitsAnyAdapter(v8::Isolate* isolate,
                                       v8::Local<v8::Value> value)
      : isolate_(isolate), v8_value_(value) {}

  NativeValueTraitsAnyAdapter& operator=(const NativeValueTraitsAnyAdapter&) =
      delete;
  NativeValueTraitsAnyAdapter& operator=(NativeValueTraitsAnyAdapter&&) =
      default;
  NativeValueTraitsAnyAdapter& operator=(const ScriptValue& value) {
    isolate_ = value.GetIsolate();
    v8_value_ = value.V8Value();
    return *this;
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  operator v8::Local<v8::Value>() const { return v8_value_; }
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator ScriptValue() const { return ScriptValue(isolate_, v8_value_); }

 private:
  v8::Isolate* isolate_ = nullptr;
  v8::Local<v8::Value> v8_value_;
};

}  // namespace bindings

// any
template <>
struct CORE_EXPORT NativeValueTraits<IDLAny>
    : public NativeValueTraitsBase<IDLAny> {
  static bindings::NativeValueTraitsAnyAdapter NativeValue(
      v8::Isolate* isolate,
      v8::Local<v8::Value> value,
      ExceptionState& exception_state) {
    return bindings::NativeValueTraitsAnyAdapter(isolate, value);
  }
};

// IDLNullable<IDLAny> must not be used.
template <>
struct NativeValueTraits<IDLNullable<IDLAny>>;

template <>
struct CORE_EXPORT NativeValueTraits<IDLOptional<IDLAny>>
    : public NativeValueTraitsBase<IDLOptional<IDLAny>> {
  static bindings::NativeValueTraitsAnyAdapter NativeValue(
      v8::Isolate* isolate,
      v8::Local<v8::Value> value,
      ExceptionState& exception_state) {
    return bindings::NativeValueTraitsAnyAdapter(isolate, value);
  }
};

// undefined
template <>
struct CORE_EXPORT NativeValueTraits<IDLUndefined>
    : public NativeValueTraitsBase<IDLUndefined> {
  static ToV8UndefinedGenerator NativeValue(v8::Isolate*,
                                            v8::Local<v8::Value>,
                                            ExceptionState&) {
    return ToV8UndefinedGenerator();
  }
};

// boolean
template <>
struct CORE_EXPORT NativeValueTraits<IDLBoolean>
    : public NativeValueTraitsBase<IDLBoolean> {
  static bool NativeValue(v8::Isolate* isolate,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) {
    return ToBoolean(isolate, value, exception_state);
  }
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLOptional<IDLBoolean>>
    : public NativeValueTraitsBase<IDLOptional<IDLBoolean>> {
  static bool NativeValue(v8::Isolate* isolate,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) {
    return ToBoolean(isolate, value, exception_state);
  }
};

// bigint
template <>
struct CORE_EXPORT NativeValueTraits<IDLBigint>
    : public NativeValueTraitsBase<IDLBigint> {
  static BigInt NativeValue(v8::Isolate* isolate,
                            v8::Local<v8::Value> value,
                            ExceptionState& exception_state) {
    return ToBigInt(isolate, value, exception_state);
  }
};

// Integer types
#define DEFINE_NATIVE_VALUE_TRAITS_INTEGER_TYPE(T, Func)             \
  template <bindings::IDLIntegerConvMode mode>                       \
  struct NativeValueTraits<IDLIntegerTypeBase<T, mode>>              \
      : public NativeValueTraitsBase<IDLIntegerTypeBase<T, mode>> {  \
    static T NativeValue(v8::Isolate* isolate,                       \
                         v8::Local<v8::Value> value,                 \
                         ExceptionState& exception_state) {          \
      return Func(isolate, value,                                    \
                  static_cast<IntegerConversionConfiguration>(mode), \
                  exception_state);                                  \
    }                                                                \
  }
DEFINE_NATIVE_VALUE_TRAITS_INTEGER_TYPE(int8_t, ToInt8);
DEFINE_NATIVE_VALUE_TRAITS_INTEGER_TYPE(uint8_t, ToUInt8);
DEFINE_NATIVE_VALUE_TRAITS_INTEGER_TYPE(int16_t, ToInt16);
DEFINE_NATIVE_VALUE_TRAITS_INTEGER_TYPE(uint16_t, ToUInt16);
DEFINE_NATIVE_VALUE_TRAITS_INTEGER_TYPE(int32_t, ToInt32);
DEFINE_NATIVE_VALUE_TRAITS_INTEGER_TYPE(uint32_t, ToUInt32);
DEFINE_NATIVE_VALUE_TRAITS_INTEGER_TYPE(int64_t, ToInt64);
DEFINE_NATIVE_VALUE_TRAITS_INTEGER_TYPE(uint64_t, ToUInt64);
#undef DEFINE_NATIVE_VALUE_TRAITS_INTEGER_TYPE

// Floats and doubles
template <>
struct CORE_EXPORT NativeValueTraits<IDLDouble>
    : public NativeValueTraitsBase<IDLDouble> {
  static double NativeValue(v8::Isolate* isolate,
                            v8::Local<v8::Value> value,
                            ExceptionState& exception_state) {
    return ToRestrictedDouble(isolate, value, exception_state);
  }
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLUnrestrictedDouble>
    : public NativeValueTraitsBase<IDLUnrestrictedDouble> {
  static double NativeValue(v8::Isolate* isolate,
                            v8::Local<v8::Value> value,
                            ExceptionState& exception_state) {
    return ToDouble(isolate, value, exception_state);
  }
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLFloat>
    : public NativeValueTraitsBase<IDLFloat> {
  static float NativeValue(v8::Isolate* isolate,
                           v8::Local<v8::Value> value,
                           ExceptionState& exception_state) {
    return ToRestrictedFloat(isolate, value, exception_state);
  }
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLUnrestrictedFloat>
    : public NativeValueTraitsBase<IDLUnrestrictedFloat> {
  static float NativeValue(v8::Isolate* isolate,
                           v8::Local<v8::Value> value,
                           ExceptionState& exception_state) {
    return ToFloat(isolate, value, exception_state);
  }
};

// Strings

namespace bindings {

// ToBlinkString implements AtomicString- and String-specific conversions from
// v8::String.  NativeValueTraitsStringAdapter helps select the best conversion.
//
// Example:
//   void F(const AtomicString& s);
//   void G(const String& s);
//
//   const NativeValueTraitsStringAdapter& x = ...;
//   F(x);  // ToBlinkString<AtomicString> is used.
//   G(x);  // ToBlinkString<String> is used.
class CORE_EXPORT NativeValueTraitsStringAdapter {
  STACK_ALLOCATED();

 public:
  NativeValueTraitsStringAdapter() = default;
  NativeValueTraitsStringAdapter(const NativeValueTraitsStringAdapter&) =
      delete;
  NativeValueTraitsStringAdapter(NativeValueTraitsStringAdapter&&) = default;
  explicit NativeValueTraitsStringAdapter(v8::Isolate* isolate,
                                          v8::Local<v8::String> value)
      : v8_string_(value), isolate_(isolate) {}
  explicit NativeValueTraitsStringAdapter(v8::Isolate* isolate,
                                          const String& value)
      : isolate_(isolate), wtf_string_(value) {}
  explicit NativeValueTraitsStringAdapter(v8::Isolate* isolate, int32_t value)
      : isolate_(isolate), wtf_string_(ToBlinkString(value)) {}

  NativeValueTraitsStringAdapter& operator=(
      const NativeValueTraitsStringAdapter&) = delete;
  NativeValueTraitsStringAdapter& operator=(NativeValueTraitsStringAdapter&&) =
      default;
  NativeValueTraitsStringAdapter& operator=(const String& value) {
    v8_string_.Clear();
    wtf_string_ = value;
    return *this;
  }

  void Init(v8::Isolate* isolate, v8::Local<v8::String> value) {
    DCHECK(v8_string_.IsEmpty());
    DCHECK(wtf_string_.IsNull());
    v8_string_ = value;
    isolate_ = isolate;
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  operator String() const { return ToString<String>(); }
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator AtomicString() const { return ToString<AtomicString>(); }
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator StringView() const& { return ToStringView(); }
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator StringView() const&& = delete;

 private:
  template <class StringType>
  StringType ToString() const {
    if (!v8_string_.IsEmpty()) [[likely]] {
      return ToBlinkString<StringType>(isolate_, v8_string_, kExternalize);
    }
    return StringType(wtf_string_);
  }

  StringView ToStringView() const& {
    if (!v8_string_.IsEmpty()) [[likely]] {
      return ToBlinkStringView(isolate_, v8_string_, string_view_backing_store_,
                               kExternalize);
    }
    return wtf_string_;
  }

  // Careful here, ordering some of the members here (mainly the isolate) may
  // be important in the hot path. Having the isolate the second member showed
  // a performance gain on MacOS arm (see crbug.com/1482549).
  v8::Local<v8::String> v8_string_;
  v8::Isolate* isolate_ = nullptr;
  String wtf_string_;
  mutable StringView::StackBackingStore string_view_backing_store_;
};

}  // namespace bindings

template <bindings::IDLStringConvMode mode>
struct NativeValueTraits<IDLByteStringBase<mode>>
    : public NativeValueTraitsBase<IDLByteStringBase<mode>> {
  // https://webidl.spec.whatwg.org/#es-ByteString
  static bindings::NativeValueTraitsStringAdapter NativeValue(
      v8::Isolate* isolate,
      v8::Local<v8::Value> value,
      ExceptionState& exception_state) {
    if (value->IsString() and value.As<v8::String>()->ContainsOnlyOneByte()) {
      return bindings::NativeValueTraitsStringAdapter(isolate,
                                                      value.As<v8::String>());
    }
    if (value->IsInt32()) {
      return bindings::NativeValueTraitsStringAdapter(
          isolate, value.As<v8::Int32>()->Value());
    }

    if (mode == bindings::IDLStringConvMode::kNullable) {
      if (value->IsNullOrUndefined())
        return bindings::NativeValueTraitsStringAdapter();
    }

    TryRethrowScope rethrow_scope(isolate, exception_state);
    v8::Local<v8::String> v8_string;
    if (!value->ToString(isolate->GetCurrentContext()).ToLocal(&v8_string)) {
      return bindings::NativeValueTraitsStringAdapter();
    }
    if (!v8_string->ContainsOnlyOneByte()) {
      exception_state.ThrowTypeError(
          "String contains non ISO-8859-1 code point.");
      return bindings::NativeValueTraitsStringAdapter();
    }
    return bindings::NativeValueTraitsStringAdapter(isolate, v8_string);
  }
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLNullable<IDLByteString>>
    : public NativeValueTraitsBase<IDLNullable<IDLByteString>> {
  static decltype(auto) NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state) {
    return NativeValueTraits<IDLByteStringBase<
        bindings::IDLStringConvMode::kNullable>>::NativeValue(isolate, value,
                                                              exception_state);
  }
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLOptional<IDLByteString>>
    : public NativeValueTraitsBase<IDLOptional<IDLByteString>> {
  static decltype(auto) NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state) {
    if (value->IsUndefined())
      return bindings::NativeValueTraitsStringAdapter();
    return NativeValueTraits<IDLByteString>::NativeValue(isolate, value,
                                                         exception_state);
  }
};

template <bindings::IDLStringConvMode mode>
struct NativeValueTraits<IDLStringBase<mode>>
    : public NativeValueTraitsBase<IDLStringBase<mode>> {
  // https://webidl.spec.whatwg.org/#es-DOMString
  static bindings::NativeValueTraitsStringAdapter NativeValue(
      v8::Isolate* isolate,
      v8::Local<v8::Value> value,
      ExceptionState& exception_state) {
    if (value->IsString()) {
      return bindings::NativeValueTraitsStringAdapter(isolate,
                                                      value.As<v8::String>());
    }
    if (value->IsInt32()) {
      return bindings::NativeValueTraitsStringAdapter(
          isolate, value.As<v8::Int32>()->Value());
    }

    if (mode == bindings::IDLStringConvMode::kNullable) {
      if (value->IsNullOrUndefined())
        return bindings::NativeValueTraitsStringAdapter();
    }
    if (mode == bindings::IDLStringConvMode::kLegacyNullToEmptyString) {
      if (value->IsNull()) {
        return bindings::NativeValueTraitsStringAdapter(isolate,
                                                        g_empty_string);
      }
    }

    TryRethrowScope rethrow_scope(isolate, exception_state);
    v8::Local<v8::String> v8_string;
    if (!value->ToString(isolate->GetCurrentContext()).ToLocal(&v8_string)) {
      return bindings::NativeValueTraitsStringAdapter();
    }
    return bindings::NativeValueTraitsStringAdapter(isolate, v8_string);
  }
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLNullable<IDLString>>
    : public NativeValueTraitsBase<IDLNullable<IDLString>> {
  static decltype(auto) NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state) {
    return NativeValueTraits<IDLStringBase<
        bindings::IDLStringConvMode::kNullable>>::NativeValue(isolate, value,
                                                              exception_state);
  }
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLOptional<IDLString>>
    : public NativeValueTraitsBase<IDLOptional<IDLString>> {
  static decltype(auto) NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state) {
    if (value->IsUndefined())
      return bindings::NativeValueTraitsStringAdapter();
    return NativeValueTraits<IDLString>::NativeValue(isolate, value,
                                                     exception_state);
  }
};

template <bindings::IDLStringConvMode mode>
struct NativeValueTraits<IDLUSVStringBase<mode>>
    : public NativeValueTraitsBase<IDLUSVStringBase<mode>> {
  // https://webidl.spec.whatwg.org/#es-USVString
  static bindings::NativeValueTraitsStringAdapter NativeValue(
      v8::Isolate* isolate,
      v8::Local<v8::Value> value,
      ExceptionState& exception_state) {
    String string = NativeValueTraits<IDLStringBase<mode>>::NativeValue(
        isolate, value, exception_state);
    if (exception_state.HadException())
      return bindings::NativeValueTraitsStringAdapter();

    return bindings::NativeValueTraitsStringAdapter(
        isolate, ReplaceUnmatchedSurrogates(string));
  }
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLNullable<IDLUSVString>>
    : public NativeValueTraitsBase<IDLNullable<IDLUSVString>> {
  static decltype(auto) NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state) {
    return NativeValueTraits<IDLUSVStringBase<
        bindings::IDLStringConvMode::kNullable>>::NativeValue(isolate, value,
                                                              exception_state);
  }
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLOptional<IDLUSVString>>
    : public NativeValueTraitsBase<IDLOptional<IDLUSVString>> {
  static decltype(auto) NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state) {
    if (value->IsUndefined())
      return bindings::NativeValueTraitsStringAdapter();
    return NativeValueTraits<IDLUSVString>::NativeValue(isolate, value,
                                                        exception_state);
  }
};

template <bindings::IDLStringConvMode mode>
struct NativeValueTraits<IDLStringStringContextTrustedHTMLBase<mode>>
    : public NativeValueTraitsBase<
          IDLStringStringContextTrustedHTMLBase<mode>> {
  static String NativeValue(v8::Isolate* isolate,
                            v8::Local<v8::Value> value,
                            ExceptionState& exception_state,
                            const char* interface_name,
                            const char* property_name,
                            ExecutionContext* execution_context) {
    if (TrustedHTML* trusted_html =
            V8TrustedHTML::ToWrappable(isolate, value)) {
      return trusted_html->toString();
    }

    auto&& string = NativeValueTraits<IDLStringBase<mode>>::NativeValue(
        isolate, value, exception_state);
    if (exception_state.HadException())
      return g_null_atom;
    return TrustedTypesCheckForHTML(string, execution_context, interface_name,
                                    property_name, exception_state);
  }
};

template <>
struct CORE_EXPORT
    NativeValueTraits<IDLNullable<IDLStringStringContextTrustedHTML>>
    : public NativeValueTraitsBase<
          IDLNullable<IDLStringStringContextTrustedHTML>> {
  static String NativeValue(v8::Isolate* isolate,
                            v8::Local<v8::Value> value,
                            ExceptionState& exception_state,
                            const char* interface_name,
                            const char* property_name,
                            ExecutionContext* execution_context) {
    return NativeValueTraits<IDLStringStringContextTrustedHTMLBase<
        bindings::IDLStringConvMode::kNullable>>::
        NativeValue(isolate, value, exception_state, interface_name,
                    property_name, execution_context);
  }
};

template <bindings::IDLStringConvMode mode>
struct NativeValueTraits<IDLStringStringContextTrustedScriptBase<mode>>
    : public NativeValueTraitsBase<
          IDLStringStringContextTrustedScriptBase<mode>> {
  static String NativeValue(v8::Isolate* isolate,
                            v8::Local<v8::Value> value,
                            ExceptionState& exception_state,
                            const char* interface_name,
                            const char* property_name,
                            ExecutionContext* execution_context) {
    if (TrustedScript* trusted_script =
            V8TrustedScript::ToWrappable(isolate, value)) {
      return trusted_script->toString();
    }

    auto&& string = NativeValueTraits<IDLStringBase<mode>>::NativeValue(
        isolate, value, exception_state);
    if (exception_state.HadException())
      return g_null_atom;
    return TrustedTypesCheckForScript(string, execution_context, interface_name,
                                      property_name, exception_state);
  }
};

template <>
struct CORE_EXPORT
    NativeValueTraits<IDLNullable<IDLStringStringContextTrustedScript>>
    : public NativeValueTraitsBase<
          IDLNullable<IDLStringStringContextTrustedScript>> {
  static String NativeValue(v8::Isolate* isolate,
                            v8::Local<v8::Value> value,
                            ExceptionState& exception_state,
                            const char* interface_name,
                            const char* property_name,
                            ExecutionContext* execution_context) {
    return NativeValueTraits<IDLStringStringContextTrustedScriptBase<
        bindings::IDLStringConvMode::kNullable>>::
        NativeValue(isolate, value, exception_state, interface_name,
                    property_name, execution_context);
  }
};

template <bindings::IDLStringConvMode mode>
struct NativeValueTraits<IDLUSVStringStringContextTrustedScriptURLBase<mode>>
    : public NativeValueTraitsBase<
          IDLUSVStringStringContextTrustedScriptURLBase<mode>> {
  static String NativeValue(v8::Isolate* isolate,
                            v8::Local<v8::Value> value,
                            ExceptionState& exception_state,
                            const char* interface_name,
                            const char* property_name,
                            ExecutionContext* execution_context) {
    if (TrustedScriptURL* trusted_script_url =
            V8TrustedScriptURL::ToWrappable(isolate, value)) {
      return trusted_script_url->toString();
    }

    auto&& string = NativeValueTraits<IDLUSVStringBase<mode>>::NativeValue(
        isolate, value, exception_state);
    if (exception_state.HadException())
      return g_null_atom;
    return TrustedTypesCheckForScriptURL(string, execution_context,
                                         interface_name, property_name,
                                         exception_state);
  }
};

template <>
struct CORE_EXPORT
    NativeValueTraits<IDLNullable<IDLUSVStringStringContextTrustedScriptURL>>
    : public NativeValueTraitsBase<
          IDLNullable<IDLUSVStringStringContextTrustedScriptURL>> {
  static String NativeValue(v8::Isolate* isolate,
                            v8::Local<v8::Value> value,
                            ExceptionState& exception_state,
                            const char* interface_name,
                            const char* property_name,
                            ExecutionContext* execution_context) {
    return NativeValueTraits<IDLUSVStringStringContextTrustedScriptURLBase<
        bindings::IDLStringConvMode::kNullable>>::
        NativeValue(isolate, value, exception_state, interface_name,
                    property_name, execution_context);
  }
};

// Buffer source types
template <>
struct CORE_EXPORT NativeValueTraits<DOMArrayBuffer>
    : public NativeValueTraitsBase<DOMArrayBuffer*> {
  static DOMArrayBuffer* NativeValue(v8::Isolate* isolate,
                                     v8::Local<v8::Value> value,
                                     ExceptionState& exception_state);

  static DOMArrayBuffer* ArgumentValue(v8::Isolate* isolate,
                                       int argument_index,
                                       v8::Local<v8::Value> value,
                                       ExceptionState& exception_state);
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLNullable<DOMArrayBuffer>>
    : public NativeValueTraitsBase<DOMArrayBuffer*> {
  static DOMArrayBuffer* NativeValue(v8::Isolate* isolate,
                                     v8::Local<v8::Value> value,
                                     ExceptionState& exception_state);

  static DOMArrayBuffer* ArgumentValue(v8::Isolate* isolate,
                                       int argument_index,
                                       v8::Local<v8::Value> value,
                                       ExceptionState& exception_state);
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLAllowResizable<DOMArrayBuffer>>
    : public NativeValueTraitsBase<DOMArrayBuffer*> {
  static DOMArrayBuffer* NativeValue(v8::Isolate* isolate,
                                     v8::Local<v8::Value> value,
                                     ExceptionState& exception_state);

  static DOMArrayBuffer* ArgumentValue(v8::Isolate* isolate,
                                       int argument_index,
                                       v8::Local<v8::Value> value,
                                       ExceptionState& exception_state);
};

template <>
struct CORE_EXPORT NativeValueTraits<DOMSharedArrayBuffer>
    : public NativeValueTraitsBase<DOMSharedArrayBuffer*> {
  static DOMSharedArrayBuffer* NativeValue(v8::Isolate* isolate,
                                           v8::Local<v8::Value> value,
                                           ExceptionState& exception_state);

  static DOMSharedArrayBuffer* ArgumentValue(v8::Isolate* isolate,
                                             int argument_index,
                                             v8::Local<v8::Value> value,
                                             ExceptionState& exception_state);
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLNullable<DOMSharedArrayBuffer>>
    : public NativeValueTraitsBase<DOMSharedArrayBuffer*> {
  static DOMSharedArrayBuffer* NativeValue(v8::Isolate* isolate,
                                           v8::Local<v8::Value> value,
                                           ExceptionState& exception_state);

  static DOMSharedArrayBuffer* ArgumentValue(v8::Isolate* isolate,
                                             int argument_index,
                                             v8::Local<v8::Value> value,
                                             ExceptionState& exception_state);
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLAllowResizable<DOMSharedArrayBuffer>>
    : public NativeValueTraitsBase<DOMSharedArrayBuffer*> {
  static DOMSharedArrayBuffer* NativeValue(v8::Isolate* isolate,
                                           v8::Local<v8::Value> value,
                                           ExceptionState& exception_state);

  static DOMSharedArrayBuffer* ArgumentValue(v8::Isolate* isolate,
                                             int argument_index,
                                             v8::Local<v8::Value> value,
                                             ExceptionState& exception_state);
};

// DOMArrayBufferBase is the common base class of DOMArrayBuffer and
// DOMSharedArrayBuffer, so it behaves as "[AllowShared] ArrayBuffer" in
// Web IDL.
template <>
struct CORE_EXPORT NativeValueTraits<DOMArrayBufferBase>
    : public NativeValueTraitsBase<DOMArrayBufferBase*> {
  static DOMArrayBufferBase* NativeValue(v8::Isolate* isolate,
                                         v8::Local<v8::Value> value,
                                         ExceptionState& exception_state);

  static DOMArrayBufferBase* ArgumentValue(v8::Isolate* isolate,
                                           int argument_index,
                                           v8::Local<v8::Value> value,
                                           ExceptionState& exception_state);
};

template <>
struct CORE_EXPORT
    NativeValueTraits<IDLBufferSourceTypeNoSizeLimit<DOMArrayBufferBase>>
    : public NativeValueTraitsBase<DOMArrayBufferBase*> {
  // BufferSourceTypeNoSizeLimit must be used only as arguments.
  static DOMArrayBufferBase* NativeValue(v8::Isolate* isolate,
                                         v8::Local<v8::Value> value,
                                         ExceptionState& exception_state) =
      delete;

  static DOMArrayBufferBase* ArgumentValue(v8::Isolate* isolate,
                                           int argument_index,
                                           v8::Local<v8::Value> value,
                                           ExceptionState& exception_state);
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLNullable<DOMArrayBufferBase>>
    : public NativeValueTraitsBase<DOMArrayBufferBase*> {
  static DOMArrayBufferBase* NativeValue(v8::Isolate* isolate,
                                         v8::Local<v8::Value> value,
                                         ExceptionState& exception_state);

  static DOMArrayBufferBase* ArgumentValue(v8::Isolate* isolate,
                                           int argument_index,
                                           v8::Local<v8::Value> value,
                                           ExceptionState& exception_state);
};

template <>
struct CORE_EXPORT NativeValueTraits<
    IDLNullable<IDLBufferSourceTypeNoSizeLimit<DOMArrayBufferBase>>>
    : public NativeValueTraitsBase<DOMArrayBufferBase*> {
  // BufferSourceTypeNoSizeLimit must be used only as arguments.
  static DOMArrayBufferBase* NativeValue(v8::Isolate* isolate,
                                         v8::Local<v8::Value> value,
                                         ExceptionState& exception_state) =
      delete;

  static DOMArrayBufferBase* ArgumentValue(v8::Isolate* isolate,
                                           int argument_index,
                                           v8::Local<v8::Value> value,
                                           ExceptionState& exception_state);
};

template <typename T>
  requires std::derived_from<T, DOMArrayBufferView>
struct NativeValueTraits<T> {
  // NotShared<T> or MaybeShared<T> should be used instead.
  static T* NativeValue(v8::Isolate* isolate,
                        v8::Local<v8::Value> value,
                        ExceptionState& exception_state) = delete;
  static T* ArgumentValue(v8::Isolate* isolate,
                          int argument_index,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) = delete;
};

template <typename T>
  requires std::derived_from<T, DOMArrayBufferView>
struct NativeValueTraits<IDLNullable<T>> {
  // NotShared<T> or MaybeShared<T> should be used instead.
  static T* NativeValue(v8::Isolate* isolate,
                        v8::Local<v8::Value> value,
                        ExceptionState& exception_state) = delete;
  static T* ArgumentValue(v8::Isolate* isolate,
                          int argument_index,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) = delete;
};

template <typename T>
  requires std::derived_from<T, DOMArrayBufferView>
struct NativeValueTraits<NotShared<T>>
    : public NativeValueTraitsBase<NotShared<T>> {
  static NotShared<T> NativeValue(v8::Isolate* isolate,
                                  v8::Local<v8::Value> value,
                                  ExceptionState& exception_state);

  static NotShared<T> ArgumentValue(v8::Isolate* isolate,
                                    int argument_index,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state);
};

template <typename T>
  requires std::derived_from<T, DOMArrayBufferView>
struct NativeValueTraits<IDLNullable<NotShared<T>>>
    : public NativeValueTraitsBase<NotShared<T>> {
  static NotShared<T> NativeValue(v8::Isolate* isolate,
                                  v8::Local<v8::Value> value,
                                  ExceptionState& exception_state);

  static NotShared<T> ArgumentValue(v8::Isolate* isolate,
                                    int argument_index,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state);
};

template <typename T>
  requires std::derived_from<T, DOMArrayBufferView>
struct NativeValueTraits<MaybeShared<T>>
    : public NativeValueTraitsBase<MaybeShared<T>> {
  static MaybeShared<T> NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state);

  static MaybeShared<T> ArgumentValue(v8::Isolate* isolate,
                                      int argument_index,
                                      v8::Local<v8::Value> value,
                                      ExceptionState& exception_state);
};

template <typename T>
  requires std::derived_from<T, DOMArrayBufferView>
struct NativeValueTraits<IDLBufferSourceTypeNoSizeLimit<MaybeShared<T>>>
    : public NativeValueTraitsBase<MaybeShared<T>> {
  static MaybeShared<T> NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state) = delete;

  static MaybeShared<T> ArgumentValue(v8::Isolate* isolate,
                                      int argument_index,
                                      v8::Local<v8::Value> value,
                                      ExceptionState& exception_state);
};

template <typename T>
  requires std::derived_from<T, DOMArrayBufferView>
struct NativeValueTraits<IDLNullable<MaybeShared<T>>>
    : public NativeValueTraitsBase<MaybeShared<T>> {
  static MaybeShared<T> NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state);

  static MaybeShared<T> ArgumentValue(v8::Isolate* isolate,
                                      int argument_index,
                                      v8::Local<v8::Value> value,
                                      ExceptionState& exception_state);
};

template <typename T>
  requires std::derived_from<T, DOMArrayBufferView>
struct NativeValueTraits<
    IDLNullable<IDLBufferSourceTypeNoSizeLimit<MaybeShared<T>>>>
    : public NativeValueTraitsBase<MaybeShared<T>> {
  // BufferSourceTypeNoSizeLimit must be used only as arguments.
  static MaybeShared<T> NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state) = delete;

  static MaybeShared<T> ArgumentValue(v8::Isolate* isolate,
                                      int argument_index,
                                      v8::Local<v8::Value> value,
                                      ExceptionState& exception_state);
};

// object
template <>
struct CORE_EXPORT NativeValueTraits<IDLObject>
    : public NativeValueTraitsBase<IDLObject> {
  static ScriptObject NativeValue(v8::Isolate* isolate,
                                  v8::Local<v8::Value> value,
                                  ExceptionState& exception_state) {
    if (value->IsObject()) [[likely]] {
      return ScriptObject(isolate, value);
    }
    exception_state.ThrowTypeError(
        ExceptionMessages::FailedToConvertJSValue("object"));
    return ScriptObject();
  }

  static ScriptObject ArgumentValue(v8::Isolate* isolate,
                                    int argument_index,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state) {
    if (value->IsObject()) [[likely]] {
      return ScriptObject(isolate, value);
    }
    exception_state.ThrowTypeError(
        ExceptionMessages::ArgumentNotOfType(argument_index, "object"));
    return ScriptObject();
  }
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLNullable<IDLObject>>
    : public NativeValueTraitsBase<IDLNullable<IDLObject>> {
  static ScriptObject NativeValue(v8::Isolate* isolate,
                                  v8::Local<v8::Value> value,
                                  ExceptionState& exception_state) {
    if (value->IsObject()) {
      return ScriptObject(isolate, value);
    }
    if (value->IsNullOrUndefined()) {
      return ScriptObject::CreateNull(isolate);
    }
    exception_state.ThrowTypeError(
        ExceptionMessages::FailedToConvertJSValue("object"));
    return ScriptObject();
  }

  static ScriptObject ArgumentValue(v8::Isolate* isolate,
                                    int argument_index,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state) {
    if (value->IsObject()) {
      return ScriptObject(isolate, value);
    }
    if (value->IsNullOrUndefined()) {
      return ScriptObject::CreateNull(isolate);
    }
    exception_state.ThrowTypeError(
        ExceptionMessages::ArgumentNotOfType(argument_index, "object"));
    return ScriptObject();
  }
};

// IDLNullable<IDLPromise> must not be used.
template <typename T>
struct NativeValueTraits<IDLNullable<IDLPromise<T>>>;

// Sequence types

// IDLSequence's implementation is a little tricky due to a historical reason.
// The following type mapping is used for IDLSequence and its variants.
//
// tl;dr: Only IDLNullable<IDLSequence<traceable_type>> is a reference type.
//   The others are value types.
//
// - IDLSequence<T> where T is not traceable
//   => Vector<T> as a value type
// - IDLSequence<T> where T is traceable
//   => HeapVector<T> as a value type despite that HeapVector is
//      GarbageCollected because HeapVector had been implemented as a non-GC
//      type (a value type) for years until 2021 January.  This point is very
//      inconsistent but kept unchanged so far.
// - IDLNullable<IDLSequence<T>> where T is not traceable
//   => std::optional<Vector<T>> as a value type
// - IDLNullable<IDLSequence<T>> where T is traceable
//   => HeapVector<T>* as a reference type.  std::optional<HeapVector<T>> is
//      not an option because it's not appropriately traceable despite that
//      the content HeapVector needs tracing.  As same as other
//      GarbageCollected types, pointer type is used to represent IDL nullable
//      type.
template <typename T>
struct NativeValueTraits<IDLSequence<T>>
    : public NativeValueTraitsBase<IDLSequence<T>> {
  // Nondependent types need to be explicitly qualified to be accessible.
  using typename NativeValueTraitsBase<IDLSequence<T>>::ImplType;

  // HeapVector is GarbageCollected, so HeapVector<T>* is used for IDLNullable
  // while std::optional<Vector<T>> is used for IDLNullable<Vector<T>>.
  static constexpr bool has_null_value = WTF::IsTraceable<T>::value;

  // https://webidl.spec.whatwg.org/#es-sequence
  static ImplType NativeValue(v8::Isolate* isolate,
                              v8::Local<v8::Value> value,
                              ExceptionState& exception_state);
};

namespace bindings {

// Slow case: follow WebIDL's "Creating a sequence from an iterable" steps to
// iterate through each element.
template <typename T>
typename NativeValueTraits<IDLSequence<T>>::ImplType
CreateIDLSequenceFromIterator(v8::Isolate* isolate,
                              ScriptIterator script_iterator,
                              ExceptionState& exception_state) {
  // https://webidl.spec.whatwg.org/#create-sequence-from-iterable
  ExecutionContext* execution_context =
      ToExecutionContext(isolate->GetCurrentContext());
  typename NativeValueTraits<IDLSequence<T>>::ImplType result;
  // 3. Repeat:
  while (script_iterator.Next(execution_context, exception_state)) {
    // 3.1. Let next be ? IteratorStep(iter).
    DCHECK(!exception_state.HadException());
    // 3.3. Let nextItem be ? IteratorValue(next).
    //
    // The value should already be non-empty, as guaranteed by the call to
    // Next() and the |exception_state| check above.
    v8::Local<v8::Value> v8_element =
        script_iterator.GetValue().ToLocalChecked();
    // 3.4. Initialize Si to the result of converting nextItem to an IDL value
    //   of type T.
    auto&& element =
        NativeValueTraits<T>::NativeValue(isolate, v8_element, exception_state);
    if (exception_state.HadException())
      return {};
    result.push_back(std::move(element));
  }
  if (exception_state.HadException())
    return {};
  // 3.2. If next is false, then return an IDL sequence value of type
  //   sequence<T> of length i, where the value of the element at index j is Sj.
  return result;
}

// Faster case: non template-specialized implementation that iterates over an
// Array that adheres to %ArrayIteratorPrototype%'s protocol.
template <typename T>
typename NativeValueTraits<IDLSequence<T>>::ImplType
CreateIDLSequenceFromV8ArraySlow(v8::Isolate* isolate,
                                 v8::Local<v8::Array> v8_array,
                                 ExceptionState& exception_state) {
  // https://webidl.spec.whatwg.org/#create-sequence-from-iterable
  const uint32_t length = v8_array->Length();
  if (length > NativeValueTraits<IDLSequence<T>>::ImplType::MaxCapacity()) {
    exception_state.ThrowRangeError("Array length exceeds supported limit.");
    return {};
  }

  using ResultType = typename NativeValueTraits<IDLSequence<T>>::ImplType;
  ResultType result;
  result.ReserveInitialCapacity(length);
  v8::Local<v8::Context> current_context = isolate->GetCurrentContext();
  TryRethrowScope rethrow_scope(isolate, exception_state);

  // Fast path -- we're creating a sequence of script wrappables, which can be
  // done by directly getting underlying object as long as array types are
  // homogeneous. With ScriptWrappables, we don't expect to enter JS during
  // iteration, so we can rely on v8::Array::Iterate() which is much faster than
  // iterating an array on the client side of the v8. Additionally, for most
  // subsptyes of ScriptWrappables, we can speed up type checks (see more on
  // that below next to supports_scriptwrappable_specific_fast_array_iteration
  // check.
  if constexpr (std::is_base_of_v<ScriptWrappable, T>) {
    struct CallbackData {
      STACK_ALLOCATED();

     public:
      v8::Isolate* isolate;
      v8::TypecheckWitness witness;
      ResultType& result;
      ExceptionState& exception_state;
      CallbackData(v8::Isolate* isolate,
                   ResultType& result,
                   ExceptionState& exception_state)
          : isolate(isolate),
            witness(isolate),
            result(result),
            exception_state(exception_state) {}
    };

    CallbackData callback_data(isolate, result, exception_state);
    v8::Array::IterationCallback callback = [](uint32_t index,
                                               v8::Local<v8::Value> v8_element,
                                               void* data) {
      CallbackData* callback_data = reinterpret_cast<CallbackData*>(data);
      v8::Isolate* isolate = callback_data->isolate;
      // 3.4. Initialize Si to the result of converting nextItem to an IDL value
      //   of type T.
      v8::TypecheckWitness& witness = callback_data->witness;
      // We can speed up type check by taking advantage of V8's type witness,
      // provided traits' NativeValue implementation doesn't have additional
      // logic beyond checking the type and calling ToScriptWrappable().
      if constexpr (
          NativeValueTraits<
              T>::supports_scriptwrappable_specific_fast_array_iteration) {
        if (witness.Matches(v8_element)) {
          callback_data->result.push_back(
              ToScriptWrappable<T>(isolate, v8_element.As<v8::Object>()));
          return v8::Array::CallbackResult::kContinue;
        }
      }
      auto&& element = NativeValueTraits<T>::NativeValue(
          isolate, v8_element, callback_data->exception_state);
      if (callback_data->exception_state.HadException()) {
        // It doesn't matter whether we return `kException` or `kBreak` here,
        // as that only affects the return value of `v8_array->Iterate()`,
        // which we are ignoring.
        return v8::Array::CallbackResult::kException;
      }
      if constexpr (
          NativeValueTraits<
              T>::supports_scriptwrappable_specific_fast_array_iteration) {
        witness.Update(v8_element);
      }
      callback_data->result.push_back(std::move(element));
      return v8::Array::CallbackResult::kContinue;
    };
    if (!v8_array->Iterate(current_context, callback, &callback_data)
             .IsJust()) {
      DCHECK(exception_state.HadException());
      return {};
    }
    return result;
  }

  // Array length may change if array is mutated during iteration.
  for (uint32_t i = 0; i < v8_array->Length(); ++i) {
    v8::Local<v8::Value> v8_element;
    if (!v8_array->Get(current_context, i).ToLocal(&v8_element)) {
      return {};
    }
    // 3.4. Initialize Si to the result of converting nextItem to an IDL value
    //   of type T.
    auto&& element =
        NativeValueTraits<T>::NativeValue(isolate, v8_element, exception_state);
    if (exception_state.HadException())
      return {};
    result.push_back(std::move(element));
  }

  // 3.2. If next is false, then return an IDL sequence value of type
  //   sequence<T> of length i, where the value of the element at index j is Sj.
  return result;
}

// Fastest case: template-specialized implementation that directly copies the
// contents of this JavaScript array into a C++ buffer.
template <typename T>
typename NativeValueTraits<IDLSequence<T>>::ImplType
CreateIDLSequenceFromV8Array(v8::Isolate* isolate,
                             v8::Local<v8::Array> v8_array,
                             ExceptionState& exception_state) {
  return CreateIDLSequenceFromV8ArraySlow<T>(isolate, v8_array,
                                             exception_state);
}

template <>
CORE_EXTERN_TEMPLATE_EXPORT
    typename NativeValueTraits<IDLSequence<IDLLong>>::ImplType
    CreateIDLSequenceFromV8Array<IDLLong>(v8::Isolate* isolate,
                                          v8::Local<v8::Array> v8_array,
                                          ExceptionState& exception_state);

}  // namespace bindings

template <typename T>
typename NativeValueTraits<IDLSequence<T>>::ImplType
NativeValueTraits<IDLSequence<T>>::NativeValue(
    v8::Isolate* isolate,
    v8::Local<v8::Value> value,
    ExceptionState& exception_state) {
  // TODO(https://crbug.com/715122): Checking for IsArray() may not be
  // enough. Other engines also prefer regular array iteration over a custom
  // @@iterator when the latter is defined, but it is not clear if this is a
  // valid optimization.
  if (value->IsArray()) {
    return bindings::CreateIDLSequenceFromV8Array<T>(
        isolate, value.As<v8::Array>(), exception_state);
  }

  // 1. If Type(V) is not Object, throw a TypeError.
  if (!value->IsObject()) {
    exception_state.ThrowTypeError(
        "The provided value cannot be converted to a sequence.");
    return ImplType();
  }

  // 2. Let method be ? GetMethod(V, @@iterator).
  // 3. If method is undefined, throw a TypeError.
  // 4. Return the result of creating a sequence from V and method.
  auto script_iterator = ScriptIterator::FromIterable(
      isolate, value.As<v8::Object>(), exception_state,
      ScriptIterator::Kind::kSync);
  if (exception_state.HadException())
    return ImplType();
  if (script_iterator.IsNull()) {
    // A null ScriptIterator with an empty |exception_state| means the
    // object is lacking a callable @@iterator property.
    exception_state.ThrowTypeError(
        "The object must have a callable @@iterator property.");
    return ImplType();
  }
  return bindings::CreateIDLSequenceFromIterator<T>(
      isolate, std::move(script_iterator), exception_state);
}

template <typename T>
  requires NativeValueTraits<IDLSequence<T>>::has_null_value
struct NativeValueTraits<IDLNullable<IDLSequence<T>>>
    : public NativeValueTraitsBase<HeapVector<AddMemberIfNeeded<T>>*> {
  using ImplType = typename NativeValueTraits<IDLSequence<T>>::ImplType*;

  static ImplType NativeValue(v8::Isolate* isolate,
                              v8::Local<v8::Value> value,
                              ExceptionState& exception_state) {
    if (value->IsNullOrUndefined())
      return nullptr;

    auto on_stack = NativeValueTraits<IDLSequence<T>>::NativeValue(
        isolate, value, exception_state);
    if (exception_state.HadException())
      return nullptr;
    auto* on_heap = MakeGarbageCollected<HeapVector<AddMemberIfNeeded<T>>>();
    on_heap->swap(on_stack);
    return on_heap;
  }

  static ImplType ArgumentValue(v8::Isolate* isolate,
                                int argument_index,
                                v8::Local<v8::Value> value,
                                ExceptionState& exception_state) {
    if (value->IsNullOrUndefined())
      return nullptr;

    auto on_stack = NativeValueTraits<IDLSequence<T>>::ArgumentValue(
        isolate, argument_index, value, exception_state);
    if (exception_state.HadException())
      return nullptr;
    auto* on_heap = MakeGarbageCollected<HeapVector<AddMemberIfNeeded<T>>>();
    on_heap->swap(on_stack);
    return on_heap;
  }
};

template <typename T>
struct NativeValueTraits<IDLOptional<IDLSequence<T>>>
    : public NativeValueTraitsBase<IDLOptional<IDLSequence<T>>> {
  static typename NativeValueTraits<IDLSequence<T>>::ImplType NativeValue(
      v8::Isolate* isolate,
      v8::Local<v8::Value> value,
      ExceptionState& exception_state) {
    if (value->IsUndefined())
      return {};
    return NativeValueTraits<IDLSequence<T>>::NativeValue(isolate, value,
                                                          exception_state);
  }

  static typename NativeValueTraits<IDLSequence<T>>::ImplType ArgumentValue(
      v8::Isolate* isolate,
      int argument_index,
      v8::Local<v8::Value> value,
      ExceptionState& exception_state) {
    if (value->IsUndefined())
      return {};
    return NativeValueTraits<IDLSequence<T>>::ArgumentValue(
        isolate, argument_index, value, exception_state);
  }
};

// Frozen array types
//
// Just for convenience, NativeValueTraits<IDLArray<T>> returns a mutable
// (Heap)Vector<T> rather than an immutable FrozenArray<T>. It's easy (and cheap
// when the move semantics is used) to convert a (Heap)Vector<T> to a
// FrozenArray<T>, but the reverse conversion is not.
//
// Note that it's possible that Blink implementation wants to make some
// modifications on the sequence before making it frozen. Thus this returns
// a mutable (Heap)Vector.
template <typename T>
struct NativeValueTraits<IDLArray<T>>
    : public NativeValueTraits<IDLSequence<T>> {};

template <typename T>
  requires NativeValueTraits<IDLSequence<T>>::has_null_value
struct NativeValueTraits<IDLNullable<IDLArray<T>>>
    : public NativeValueTraits<IDLNullable<IDLSequence<T>>> {};

// Record types
template <typename K, typename V>
struct NativeValueTraits<IDLRecord<K, V>>
    : public NativeValueTraitsBase<IDLRecord<K, V>> {
  // Nondependent types need to be explicitly qualified to be accessible.
  using typename NativeValueTraitsBase<IDLRecord<K, V>>::ImplType;

  // Converts a JavaScript value |O| to an IDL record<K, V> value. In C++, a
  // record is represented as a Vector<std::pair<k, v>> (or a HeapVector if
  // necessary). See https://webidl.spec.whatwg.org/#es-record.
  static ImplType NativeValue(v8::Isolate* isolate,
                              v8::Local<v8::Value> v8_value,
                              ExceptionState& exception_state) {
    v8::Local<v8::Context> context = isolate->GetCurrentContext();

    // "1. If Type(O) is not Object, throw a TypeError."
    if (!v8_value->IsObject()) {
      exception_state.ThrowTypeError(
          "Only objects can be converted to record<K,V> types");
      return ImplType();
    }
    v8::Local<v8::Object> v8_object = v8::Local<v8::Object>::Cast(v8_value);
    TryRethrowScope rethrow_scope(isolate, exception_state);

    // "3. Let keys be ? O.[[OwnPropertyKeys]]()."
    v8::Local<v8::Array> keys;
    // While we could pass v8::ONLY_ENUMERABLE below, doing so breaks
    // web-platform-tests' headers-record.html. It might be worthwhile to try
    // changing the test.
    if (!v8_object
             ->GetOwnPropertyNames(context,
                                   static_cast<v8::PropertyFilter>(
                                       v8::PropertyFilter::ALL_PROPERTIES),
                                   v8::KeyConversionMode::kConvertToString)
             .ToLocal(&keys)) {
      return ImplType();
    }
    if (keys->Length() > ImplType::MaxCapacity()) {
      exception_state.ThrowRangeError("Array length exceeds supported limit.");
      return ImplType();
    }

    // "2. Let result be a new empty instance of record<K, V>."
    ImplType result;
    result.ReserveInitialCapacity(keys->Length());

    // The conversion algorithm needs a data structure with fast insertion at
    // the end while at the same time requiring fast checks for previous insert
    // of a given key. |seenKeys| is a key/position in |result| map that aids in
    // the latter part.
    HashMap<String, uint32_t> seen_keys;

    for (uint32_t i = 0; i < keys->Length(); ++i) {
      // "4. Repeat, for each element key of keys in List order:"
      v8::Local<v8::Value> key;
      if (!keys->Get(context, i).ToLocal(&key)) {
        return ImplType();
      }

      // "4.1. Let desc be ? O.[[GetOwnProperty]](key)."
      v8::Local<v8::Value> desc;
      if (!v8_object->GetOwnPropertyDescriptor(context, key.As<v8::Name>())
               .ToLocal(&desc)) {
        return ImplType();
      }

      // "4.2. If desc is not undefined and desc.[[Enumerable]] is true:"
      // We can call ToLocalChecked() and ToChecked() here because
      // GetOwnPropertyDescriptor is responsible for catching any exceptions
      // and failures, and if we got to this point of the code we have a proper
      // object that was not created by a user.
      if (desc->IsUndefined())
        continue;
      DCHECK(desc->IsObject());
      v8::Local<v8::Value> enumerable =
          v8::Local<v8::Object>::Cast(desc)
              ->Get(context, V8AtomicString(isolate, "enumerable"))
              .ToLocalChecked();
      if (!enumerable->BooleanValue(isolate))
        continue;

      // "4.2.1. Let typedKey be key converted to an IDL value of type K."
      String typed_key =
          NativeValueTraits<K>::NativeValue(isolate, key, exception_state);
      if (exception_state.HadException())
        return ImplType();

      // "4.2.2. Let value be ? Get(O, key)."
      v8::Local<v8::Value> value;
      if (!v8_object->Get(context, key).ToLocal(&value)) {
        return ImplType();
      }

      // "4.2.3. Let typedValue be value converted to an IDL value of type V."
      typename ImplType::ValueType::second_type typed_value =
          NativeValueTraits<V>::NativeValue(isolate, value, exception_state);
      if (exception_state.HadException())
        return ImplType();

      if (seen_keys.Contains(typed_key)) {
        // "4.2.4. If typedKey is already a key in result, set its value to
        //         typedValue.
        //         Note: This can happen when K is USVString and key contains
        //         unpaired surrogates."
        const uint32_t pos = seen_keys.at(typed_key);
        result[pos].second = std::move(typed_value);
      } else {
        // "4.2.5. Otherwise, append to result a mapping (typedKey,
        // typedValue)."
        // Note we can take this shortcut because we are always appending.
        const uint32_t pos = result.size();
        seen_keys.Set(typed_key, pos);
        result.UncheckedAppend(
            std::make_pair(std::move(typed_key), std::move(typed_value)));
      }
    }
    // "5. Return result."
    return result;
  }
};

// Callback function types
template <typename T>
  requires std::derived_from<T, CallbackFunctionBase>
struct NativeValueTraits<T> : public NativeValueTraitsBase<T*> {
  static T* NativeValue(v8::Isolate* isolate,
                        v8::Local<v8::Value> value,
                        ExceptionState& exception_state) {
    if (value->IsFunction())
      return T::Create(value.As<v8::Function>());
    exception_state.ThrowTypeError("The given value is not a function.");
    return nullptr;
  }

  static T* ArgumentValue(v8::Isolate* isolate,
                          int argument_index,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) {
    if (value->IsFunction())
      return T::Create(value.As<v8::Function>());
    exception_state.ThrowTypeError(
        ExceptionMessages::ArgumentNotOfType(argument_index, "Function"));
    return nullptr;
  }
};

template <typename T>
  requires std::derived_from<T, CallbackFunctionBase>
struct NativeValueTraits<IDLNullable<T>>
    : public NativeValueTraitsBase<IDLNullable<T>> {
  static T* NativeValue(v8::Isolate* isolate,
                        v8::Local<v8::Value> value,
                        ExceptionState& exception_state) {
    if (value->IsFunction())
      return T::Create(value.As<v8::Function>());
    if (value->IsNullOrUndefined())
      return nullptr;
    exception_state.ThrowTypeError("The given value is not a function.");
    return nullptr;
  }

  static T* ArgumentValue(v8::Isolate* isolate,
                          int argument_index,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) {
    if (value->IsFunction())
      return T::Create(value.As<v8::Function>());
    if (value->IsNullOrUndefined())
      return nullptr;
    exception_state.ThrowTypeError(
        ExceptionMessages::ArgumentNotOfType(argument_index, "Function"));
    return nullptr;
  }
};

// Callback interface types
template <typename T>
  requires std::derived_from<T, CallbackInterfaceBase>
struct NativeValueTraits<T> : public NativeValueTraitsBase<T*> {
  static T* NativeValue(v8::Isolate* isolate,
                        v8::Local<v8::Value> value,
                        ExceptionState& exception_state) {
    if (value->IsObject())
      return T::Create(value.As<v8::Object>());
    exception_state.ThrowTypeError("The given value is not an object.");
    return nullptr;
  }

  static T* ArgumentValue(v8::Isolate* isolate,
                          int argument_index,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) {
    if (value->IsObject())
      return T::Create(value.As<v8::Object>());
    exception_state.ThrowTypeError(
        ExceptionMessages::ArgumentNotOfType(argument_index, "Object"));
    return nullptr;
  }
};

// Interface types
template <typename T>
  requires std::derived_from<T, CallbackInterfaceBase>
struct NativeValueTraits<IDLNullable<T>>
    : public NativeValueTraitsBase<IDLNullable<T>> {
  static T* NativeValue(v8::Isolate* isolate,
                        v8::Local<v8::Value> value,
                        ExceptionState& exception_state) {
    if (value->IsObject())
      return T::Create(value.As<v8::Object>());
    if (value->IsNullOrUndefined())
      return nullptr;
    exception_state.ThrowTypeError("The given value is not an object.");
    return nullptr;
  }

  static T* ArgumentValue(v8::Isolate* isolate,
                          int argument_index,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) {
    if (value->IsObject())
      return T::Create(value.As<v8::Object>());
    if (value->IsNullOrUndefined())
      return nullptr;
    exception_state.ThrowTypeError(
        ExceptionMessages::ArgumentNotOfType(argument_index, "Object"));
    return nullptr;
  }
};

// Dictionary types
template <typename T>
  requires std::derived_from<T, bindings::InputDictionaryBase>
struct NativeValueTraits<T> : public NativeValueTraitsBase<T*> {
  static T* NativeValue(v8::Isolate* isolate,
                        v8::Local<v8::Value> value,
                        ExceptionState& exception_state) {
    return T::Create(isolate, value, exception_state);
  }
};

// We don't support nullable dictionary types in general since it's quite
// confusing and often misused.
template <typename T>
  requires std::derived_from<T, bindings::InputDictionaryBase> &&
           (std::same_as<T, GPUColorTargetState> ||
            std::same_as<T, GPURenderPassColorAttachment> ||
            std::same_as<T, GPUVertexBufferLayout>)
struct NativeValueTraits<IDLNullable<T>> : public NativeValueTraitsBase<T*> {
  static T* NativeValue(v8::Isolate* isolate,
                        v8::Local<v8::Value> value,
                        ExceptionState& exception_state) {
    if (value->IsNullOrUndefined())
      return nullptr;
    return T::Create(isolate, value, exception_state);
  }
};

// Enumeration types
template <typename T>
  requires std::derived_from<T, bindings::EnumerationBase>
struct NativeValueTraits<T> : public NativeValueTraitsBase<T> {
  static T NativeValue(v8::Isolate* isolate,
                       v8::Local<v8::Value> value,
                       ExceptionState& exception_state) {
    return T::Create(isolate, value, exception_state);
  }
};

// Interface types
template <typename T>
  requires std::derived_from<T, ScriptWrappable>
struct NativeValueTraits<T> : public NativeValueTraitsBase<T*> {
  // This signifies that CreateIDLSequenceFromV8ArraySlow() may apply
  // certain optimization based on assumptions about `NativeValue()`
  // implementation below. For subclasses of ScriptWrappable that have
  // different implementation of NativeValue(), this should remain false.
  static constexpr bool supports_scriptwrappable_specific_fast_array_iteration =
      true;

  static inline T* NativeValue(v8::Isolate* isolate,
                               v8::Local<v8::Value> value,
                               ExceptionState& exception_state) {
    const WrapperTypeInfo* wrapper_type_info = T::GetStaticWrapperTypeInfo();
    if (V8PerIsolateData::From(isolate)->HasInstance(wrapper_type_info,
                                                     value)) {
      return ToScriptWrappable<T>(isolate, value.As<v8::Object>());
    }

    bindings::NativeValueTraitsInterfaceNotOfType(wrapper_type_info,
                                                  exception_state);
    return nullptr;
  }

  static inline T* ArgumentValue(v8::Isolate* isolate,
                                 int argument_index,
                                 v8::Local<v8::Value> value,
                                 ExceptionState& exception_state) {
    const WrapperTypeInfo* wrapper_type_info = T::GetStaticWrapperTypeInfo();
    if (V8PerIsolateData::From(isolate)->HasInstance(wrapper_type_info,
                                                     value)) {
      return ToScriptWrappable<T>(isolate, value.As<v8::Object>());
    }

    bindings::NativeValueTraitsInterfaceNotOfType(
        wrapper_type_info, argument_index, exception_state);
    return nullptr;
  }
};

template <typename T>
  requires std::derived_from<T, ScriptWrappable>
struct NativeValueTraits<IDLNullable<T>>
    : public NativeValueTraitsBase<IDLNullable<T>> {
  static inline T* NativeValue(v8::Isolate* isolate,
                               v8::Local<v8::Value> value,
                               ExceptionState& exception_state) {
    const WrapperTypeInfo* wrapper_type_info = T::GetStaticWrapperTypeInfo();
    if (V8PerIsolateData::From(isolate)->HasInstance(wrapper_type_info,
                                                     value)) {
      return ToScriptWrappable<T>(isolate, value.As<v8::Object>());
    }

    if (value->IsNullOrUndefined())
      return nullptr;

    bindings::NativeValueTraitsInterfaceNotOfType(wrapper_type_info,
                                                  exception_state);
    return nullptr;
  }

  static inline T* ArgumentValue(v8::Isolate* isolate,
                                 int argument_index,
                                 v8::Local<v8::Value> value,
                                 ExceptionState& exception_state) {
    const WrapperTypeInfo* wrapper_type_info = T::GetStaticWrapperTypeInfo();
    if (V8PerIsolateData::From(isolate)->HasInstance(wrapper_type_info,
                                                     value)) {
      return ToScriptWrappable<T>(isolate, value.As<v8::Object>());
    }

    if (value->IsNullOrUndefined())
      return nullptr;

    bindings::NativeValueTraitsInterfaceNotOfType(
        wrapper_type_info, argument_index, exception_state);
    return nullptr;
  }
};

template <typename T>
  requires std::derived_from<T, bindings::UnionBase>
struct NativeValueTraits<T> : public NativeValueTraitsBase<T*> {
  static T* NativeValue(v8::Isolate* isolate,
                        v8::Local<v8::Value> value,
                        ExceptionState& exception_state) {
    return T::Create(isolate, value, exception_state);
  }

  static T* ArgumentValue(v8::Isolate* isolate,
                          int argument_index,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) {
    return T::Create(isolate, value, exception_state);
  }
};

template <typename T>
  requires std::derived_from<T, bindings::UnionBase>
struct NativeValueTraits<IDLNullable<T>> : public NativeValueTraitsBase<T*> {
  static T* NativeValue(v8::Isolate* isolate,
                        v8::Local<v8::Value> value,
                        ExceptionState& exception_state) {
    if (value->IsNullOrUndefined())
      return nullptr;
    return T::Create(isolate, value, exception_state);
  }

  static T* ArgumentValue(v8::Isolate* isolate,
                          int argument_index,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) {
    if (value->IsNullOrUndefined())
      return nullptr;
    return T::Create(isolate, value, exception_state);
  }
};

// Nullable types
template <typename InnerType>
  requires(!NativeValueTraits<InnerType>::has_null_value)
struct NativeValueTraits<IDLNullable<InnerType>>
    : public NativeValueTraitsBase<IDLNullable<InnerType>> {
  // https://webidl.spec.whatwg.org/#es-nullable-type
  using ImplType =
      std::optional<typename NativeValueTraits<InnerType>::ImplType>;

  static ImplType NativeValue(v8::Isolate* isolate,
                              v8::Local<v8::Value> value,
                              ExceptionState& exception_state) {
    if (value->IsNullOrUndefined())
      return std::nullopt;
    return NativeValueTraits<InnerType>::NativeValue(isolate, value,
                                                     exception_state);
  }

  static ImplType ArgumentValue(v8::Isolate* isolate,
                                int argument_index,
                                v8::Local<v8::Value> value,
                                ExceptionState& exception_state) {
    if (value->IsNullOrUndefined())
      return std::nullopt;
    return NativeValueTraits<InnerType>::ArgumentValue(isolate, argument_index,
                                                       value, exception_state);
  }
};

// IDLNullable<IDLNullable<T>> must not be used.
template <typename T>
struct NativeValueTraits<IDLNullable<IDLNullable<T>>>;

// Optional types
template <typename T>
  requires std::is_arithmetic_v<typename NativeValueTraits<T>::ImplType>
struct NativeValueTraits<IDLOptional<T>>
    : public NativeValueTraitsBase<typename NativeValueTraits<T>::ImplType> {
  using ImplType = typename NativeValueTraits<T>::ImplType;

  static ImplType NativeValue(v8::Isolate* isolate,
                              v8::Local<v8::Value> value,
                              ExceptionState& exception_state) {
    // Just let ES undefined to be converted into 0.
    return NativeValueTraits<T>::NativeValue(isolate, value, exception_state);
  }

  static ImplType ArgumentValue(v8::Isolate* isolate,
                                int argument_index,
                                v8::Local<v8::Value> value,
                                ExceptionState& exception_state) {
    // Just let ES undefined to be converted into 0.
    return NativeValueTraits<T>::ArgumentValue(isolate, argument_index, value,
                                               exception_state);
  }
};

template <typename T>
  requires std::is_pointer_v<typename NativeValueTraits<T>::ImplType>
struct NativeValueTraits<IDLOptional<T>>
    : public NativeValueTraitsBase<typename NativeValueTraits<T>::ImplType> {
  using ImplType = typename NativeValueTraits<T>::ImplType;

  static ImplType NativeValue(v8::Isolate* isolate,
                              v8::Local<v8::Value> value,
                              ExceptionState& exception_state) {
    if (value->IsUndefined())
      return nullptr;
    return NativeValueTraits<T>::NativeValue(isolate, value, exception_state);
  }

  static ImplType ArgumentValue(v8::Isolate* isolate,
                                int argument_index,
                                v8::Local<v8::Value> value,
                                ExceptionState& exception_state) {
    if (value->IsUndefined())
      return nullptr;
    return NativeValueTraits<T>::ArgumentValue(isolate, argument_index, value,
                                               exception_state);
  }
};

// EventHandler
template <>
struct CORE_EXPORT NativeValueTraits<IDLEventHandler>
    : public NativeValueTraitsBase<IDLEventHandler> {
  static EventListener* NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state);
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLOnBeforeUnloadEventHandler>
    : public NativeValueTraitsBase<IDLOnBeforeUnloadEventHandler> {
  static EventListener* NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state);
};

template <>
struct CORE_EXPORT NativeValueTraits<IDLOnErrorEventHandler>
    : public NativeValueTraitsBase<IDLOnErrorEventHandler> {
  static EventListener* NativeValue(v8::Isolate* isolate,
                                    v8::Local<v8::Value> value,
                                    ExceptionState& exception_state);
};

// EventHandler and its family are nullable, so IDLNullable<IDLEventHandler>
// must not be used.
template <>
struct NativeValueTraits<IDLNullable<IDLEventHandler>>;
template <>
struct NativeValueTraits<IDLNullable<IDLOnBeforeUnloadEventHandler>>;
template <>
struct NativeValueTraits<IDLNullable<IDLOnErrorEventHandler>>;

template <typename T>
  requires std::derived_from<T, PassAsSpanMarkerBase> && (!T::is_typed)
struct NativeValueTraits<T> : public NativeValueTraitsBase<T> {
  static void NativeValue(v8::Isolate* isolate,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) = delete;

  static bindings::internal::ByteSpanWithInlineStorage ArgumentValue(
      v8::Isolate* isolate,
      int argument_index,
      v8::Local<v8::Value> value,
      ExceptionState& exception_state) {
    bindings::internal::ByteSpanWithInlineStorage result;
    if (value->IsArrayBuffer()) {
      result.Assign(
          bindings::internal::GetArrayData(value.As<v8::ArrayBuffer>()));
      return result;
    }
    if (T::allow_shared && value->IsSharedArrayBuffer()) {
      result.Assign(
          bindings::internal::GetArrayData(value.As<v8::SharedArrayBuffer>()));
      return result;
    }
    if (value->IsArrayBufferView()) {
      v8::Local<v8::ArrayBufferView> view = value.As<v8::ArrayBufferView>();
      if (!T::allow_shared && view->HasBuffer() &&
          view->Buffer()->GetBackingStore()->IsShared()) [[unlikely]] {
        exception_state.ThrowTypeError(
            "The provided ArrayBufferView value must not be shared.");
        return result;
      }
      result.Assign(view->GetContents(result.GetInlineStorage()));
      return result;
    }
    exception_state.ThrowTypeError(
        ExceptionMessages::ArgumentNotOfType(argument_index, "ArrayBuffer"));
    return result;
  }
};

template <typename T>
  requires std::derived_from<T, PassAsSpanMarkerBase> && T::is_typed
struct NativeValueTraits<T> : public NativeValueTraitsBase<T> {
  using ElementType = typename T::ElementType;

  static void NativeValue(v8::Isolate* isolate,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) = delete;

  static typename T::ReturnType ArgumentValue(v8::Isolate* isolate,
                                              int argument_index,
                                              v8::Local<v8::Value> value,
                                              ExceptionState& exception_state) {
    typename T::ReturnType result;
    using Traits = bindings::internal::TypedArrayElementTraits<ElementType>;
    if (Traits::IsViewOfType(value)) [[likely]] {
      v8::Local<v8::ArrayBufferView> view = value.As<v8::ArrayBufferView>();
      if (!T::allow_shared && view->HasBuffer() &&
          view->Buffer()->GetBackingStore()->IsShared()) [[unlikely]] {
        exception_state.ThrowTypeError(
            "The provided ArrayBufferView value must not be shared.");
        return result;
      }
      result.Assign(view->GetContents(result.GetInlineStorage()));
      return result;
    }
    if constexpr (T::allow_sequence) {
      auto&& vec = NativeValueTraits<IDLSequence<typename Traits::IDLType>>::
          ArgumentValue(isolate, argument_index, value, exception_state);
      if (!exception_state.HadException()) [[likely]] {
        result.Assign(std::move(vec));
      }
      return result;
    }
    exception_state.ThrowTypeError(
        ExceptionMessages::ArgumentNotOfType(argument_index, "TypedArray"));
    return result;
  }
};

template <typename T>
  requires std::derived_from<T, PassAsSpanMarkerBase>
struct NativeValueTraits<IDLOptional<T>> : public NativeValueTraitsBase<T> {
  // PassAsSpan is only applicable to arguments.
  static void NativeValue(v8::Isolate* isolate,
                          v8::Local<v8::Value> value,
                          ExceptionState& exception_state) = delete;

  static std::optional<typename T::ReturnType> ArgumentValue(
      v8::Isolate* isolate,
      int argument_index,
      v8::Local<v8::Value> value,
      ExceptionState& exception_state) {
    if (value->IsUndefined()) {
      return std::nullopt;
    }
    return NativeValueTraits<T>::ArgumentValue(isolate, argument_index, value,
                                               exception_state);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_NATIVE_VALUE_TRAITS_IMPL_H_
