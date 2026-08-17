// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_TEST_HELPERS_H_

#include <utility>

#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_testing.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/core/html/custom/ce_reactions_scope.h"
#include "third_party/blink/renderer/core/html/custom/custom_element_definition.h"
#include "third_party/blink/renderer/core/html/custom/custom_element_definition_builder.h"
#include "third_party/blink/renderer/core/html/custom/custom_element_registry.h"
#include "third_party/blink/renderer/core/html/html_document.h"
#include "third_party/blink/renderer/core/testing/null_execution_context.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// Supports creating test custom element definitions to be added into the global
// registry. Does not support scoped registries.
class CustomElementTestingScope : public V8TestingScope {
  STACK_ALLOCATED();

 public:
  static CustomElementTestingScope& GetInstance() {
    DCHECK(instance_)
        << "Custom element unit tests require CustomElementTestingScope";
    return *instance_;
  }

  CustomElementTestingScope() {
    // We should never create nested testing scopes.
    DCHECK(!instance_);
    instance_ = this;
  }

  ~CustomElementTestingScope() { instance_ = nullptr; }

  CustomElementRegistry& Registry();

 private:
  static CustomElementTestingScope* instance_;
};

class TestCustomElementDefinitionBuilder
    : public CustomElementDefinitionBuilder {
  STACK_ALLOCATED();

 public:
  TestCustomElementDefinitionBuilder();
  TestCustomElementDefinitionBuilder(
      const TestCustomElementDefinitionBuilder&) = delete;
  TestCustomElementDefinitionBuilder& operator=(
      const TestCustomElementDefinitionBuilder&) = delete;

  V8CustomElementConstructor* Constructor() override { return constructor_; }
  bool CheckConstructorIntrinsics() override { return true; }
  bool CheckConstructorNotRegistered() override { return true; }
  bool RememberOriginalProperties() override { return true; }
  CustomElementDefinition* Build(const CustomElementDescriptor&) override;

 private:
  V8CustomElementConstructor* constructor_;
};

class TestCustomElementDefinition : public CustomElementDefinition {
 public:
  explicit TestCustomElementDefinition(
      const CustomElementDescriptor& descriptor);

  TestCustomElementDefinition(const CustomElementDescriptor& descriptor,
                              V8CustomElementConstructor* constructor);

  TestCustomElementDefinition(const CustomElementDescriptor& descriptor,
                              V8CustomElementConstructor* constructor,
                              HashSet<AtomicString>&& observed_attributes,
                              const Vector<String>& disabled_features);

  TestCustomElementDefinition(const TestCustomElementDefinition&) = delete;
  TestCustomElementDefinition& operator=(const TestCustomElementDefinition&) =
      delete;

  ~TestCustomElementDefinition() override = default;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(constructor_);
    CustomElementDefinition::Trace(visitor);
  }

  ScriptValue GetConstructorForScript() override { return ScriptValue(); }

  V8CustomElementConstructor* GetV8CustomElementConstructor() override {
    return constructor_.Get();
  }

  bool RunConstructor(Element& element) override;

  HTMLElement* CreateAutonomousCustomElementSync(
      Document& document,
      const QualifiedName&) override {
    return CreateElementForConstructor(document);
  }

  bool HasConnectedCallback() const override { return false; }
  bool HasDisconnectedCallback() const override { return false; }
  bool HasConnectedMoveCallback() const override { return false; }
  bool HasAdoptedCallback() const override { return false; }
  bool HasFormAssociatedCallback() const override { return false; }
  bool HasFormResetCallback() const override { return false; }
  bool HasFormDisabledCallback() const override { return false; }
  bool HasFormStateRestoreCallback() const override { return false; }

  void RunConnectedCallback(Element&) override {
    NOTREACHED() << "definition does not have connected callback";
  }

  void RunDisconnectedCallback(Element&) override {
    NOTREACHED() << "definition does not have disconnected callback";
  }

  void RunConnectedMoveCallback(Element&) override {
    NOTREACHED() << "definition does not have disconnected callback";
  }

  void RunAdoptedCallback(Element&,
                          Document& old_owner,
                          Document& new_owner) override {
    NOTREACHED() << "definition does not have adopted callback";
  }

  void RunAttributeChangedCallback(Element&,
                                   const QualifiedName&,
                                   const AtomicString& old_value,
                                   const AtomicString& new_value) override {
    NOTREACHED() << "definition does not have attribute changed callback";
  }
  void RunFormAssociatedCallback(Element& element,
                                 HTMLFormElement* nullable_form) override {
    NOTREACHED() << "definition does not have formAssociatedCallback";
  }

  void RunFormResetCallback(Element& element) override {
    NOTREACHED() << "definition does not have formResetCallback";
  }

  void RunFormDisabledCallback(Element& element, bool is_disabled) override {
    NOTREACHED() << "definition does not have disabledStateChangedCallback";
  }

  void RunFormStateRestoreCallback(Element& element,
                                   const V8ControlValue* value,
                                   const String& mode) override {
    NOTREACHED() << "definition does not have restoreValueCallback";
  }

 private:
  Member<V8CustomElementConstructor> constructor_;
};

class CreateElement {
  STACK_ALLOCATED();

 public:
  CreateElement(const AtomicString& local_name)
      : namespace_uri_(html_names::xhtmlNamespaceURI),
        local_name_(local_name) {}

  CreateElement& InDocument(Document* document) {
    document_ = document;
    return *this;
  }

  CreateElement& InNamespace(const AtomicString& uri) {
    namespace_uri_ = uri;
    return *this;
  }

  CreateElement& WithId(const AtomicString& id) {
    attributes_.push_back(std::make_pair(html_names::kIdAttr, id));
    return *this;
  }

  CreateElement& WithIsValue(const AtomicString& value) {
    is_value_ = value;
    return *this;
  }

  operator Element*() const {
    Document* document = document_;
    if (!document) {
      document =
          HTMLDocument::CreateForTest(execution_context_.GetExecutionContext());
    }
    NonThrowableExceptionState no_exceptions;
    Element* element = document->CreateElement(
        QualifiedName(g_null_atom, local_name_, namespace_uri_),
        CreateElementFlags::ByCreateElement(), is_value_);
    for (const auto& attribute : attributes_)
      element->setAttribute(attribute.first, attribute.second);
    return element;
  }

 private:
  ScopedNullExecutionContext execution_context_;
  Document* document_ = nullptr;
  AtomicString namespace_uri_;
  AtomicString local_name_;
  AtomicString is_value_;
  Vector<std::pair<QualifiedName, AtomicString>> attributes_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_TEST_HELPERS_H_
