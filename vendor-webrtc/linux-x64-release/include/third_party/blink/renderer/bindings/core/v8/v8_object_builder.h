// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_OBJECT_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_OBJECT_BUILDER_H_

#include "third_party/blink/renderer/bindings/core/v8/to_v8_traits.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class ScriptState;
class ScriptObject;

class CORE_EXPORT V8ObjectBuilder final {
  STACK_ALLOCATED();

 public:
  explicit V8ObjectBuilder(ScriptState*);

  ScriptState* GetScriptState() const { return script_state_; }

  V8ObjectBuilder& Add(const StringView& name, const V8ObjectBuilder&);

  V8ObjectBuilder& AddNull(const StringView& name);
  V8ObjectBuilder& AddBoolean(const StringView& name, bool value);
  V8ObjectBuilder& AddNumber(const StringView& name, double value);
  V8ObjectBuilder& AddNumberOrNull(const StringView& name,
                                   std::optional<double> value);
  V8ObjectBuilder& AddInteger(const StringView& name, uint64_t value);
  V8ObjectBuilder& AddString(const StringView& name, const StringView& value);
  V8ObjectBuilder& AddStringOrNull(const StringView& name,
                                   const StringView& value);
  V8ObjectBuilder& AddV8Value(const StringView& name, v8::Local<v8::Value>);

  template <typename T>
    requires std::derived_from<T, bindings::DictionaryBase> ||
             std::derived_from<T, ScriptWrappable>
  V8ObjectBuilder& Add(const StringView& name, T* value) {
    AddInternal(name, ToV8Traits<T>::ToV8(script_state_, value));
    return *this;
  }

  template <typename T, typename Container>
  V8ObjectBuilder& AddVector(const StringView& name, const Container& value) {
    AddInternal(name, ToV8Traits<IDLSequence<T>>::ToV8(script_state_, value));
    return *this;
  }

  ScriptObject ToScriptObject() const;
  v8::Local<v8::Object> V8Object() const { return object_; }

 private:
  void AddInternal(const StringView& name, v8::Local<v8::Value>);

  ScriptState* script_state_;
  v8::Local<v8::Object> object_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_OBJECT_BUILDER_H_
