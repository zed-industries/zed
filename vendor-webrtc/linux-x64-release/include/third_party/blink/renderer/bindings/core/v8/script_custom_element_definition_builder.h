// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_CUSTOM_ELEMENT_DEFINITION_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_CUSTOM_ELEMENT_DEFINITION_BUILDER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_custom_element_definition_data.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/custom/custom_element_definition_builder.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;

class CORE_EXPORT ScriptCustomElementDefinitionBuilder
    : public CustomElementDefinitionBuilder {
  STACK_ALLOCATED();

 public:
  ScriptCustomElementDefinitionBuilder(ScriptState*,
                                       CustomElementRegistry*,
                                       V8CustomElementConstructor* constructor,
                                       ExceptionState&);

  ScriptCustomElementDefinitionBuilder(
      const ScriptCustomElementDefinitionBuilder&) = delete;
  ScriptCustomElementDefinitionBuilder& operator=(
      const ScriptCustomElementDefinitionBuilder&) = delete;

  ~ScriptCustomElementDefinitionBuilder() = default;

  V8CustomElementConstructor* Constructor() override {
    return data_.constructor_;
  }

  bool CheckConstructorIntrinsics() override;
  bool CheckConstructorNotRegistered() override;
  bool RememberOriginalProperties() override;
  CustomElementDefinition* Build(const CustomElementDescriptor&) override;

 private:
  ScriptState* GetScriptState() { return data_.script_state_; }
  v8::Isolate* Isolate();

  ExceptionState& exception_state_;
  ScriptCustomElementDefinitionData data_;
  // These v8::Local handles on stack make the function objects alive until we
  // finish building the CustomElementDefinition and wrapper-tracing on it gets
  // available.
  v8::Local<v8::Value> v8_connected_callback_;
  v8::Local<v8::Value> v8_disconnected_callback_;
  v8::Local<v8::Value> v8_connected_move_callback_;
  v8::Local<v8::Value> v8_adopted_callback_;
  v8::Local<v8::Value> v8_attribute_changed_callback_;
  v8::Local<v8::Value> v8_form_associated_callback_;
  v8::Local<v8::Value> v8_form_reset_callback_;
  v8::Local<v8::Value> v8_form_disabled_callback_;
  v8::Local<v8::Value> v8_form_state_restore_callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_CUSTOM_ELEMENT_DEFINITION_BUILDER_H_
