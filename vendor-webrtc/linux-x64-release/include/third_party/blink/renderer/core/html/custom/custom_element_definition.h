// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_DEFINITION_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/create_element_flags.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/core/html/custom/custom_element_descriptor.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class Document;
class Element;
class ExceptionState;
class HTMLElement;
class HTMLFormElement;
class QualifiedName;
class V8CustomElementConstructor;

enum class FormAssociationFlag {
  kNo,
  kYes,
};

class CORE_EXPORT CustomElementDefinition
    : public GarbageCollected<CustomElementDefinition>,
      public NameClient,
      public ElementRareDataField {
 public:
  CustomElementDefinition(const CustomElementDefinition&) = delete;
  CustomElementDefinition& operator=(const CustomElementDefinition&) = delete;
  ~CustomElementDefinition() override;

  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override {
    return "CustomElementDefinition";
  }

  CustomElementRegistry& GetRegistry() { return *registry_; }

  const CustomElementDescriptor& Descriptor() { return descriptor_; }

  // TODO(yosin): To support Web Modules, introduce an abstract
  // class |CustomElementConstructor| to allow us to have JavaScript
  // and C++ constructors and ask the binding layer to convert
  // |CustomElementConstructor| to |ScriptValue|. Replace
  // |getConstructorForScript()| by |getConstructor() ->
  // CustomElementConstructor|.
  virtual ScriptValue GetConstructorForScript() = 0;

  virtual V8CustomElementConstructor* GetV8CustomElementConstructor() = 0;

  HTMLElement* CreateElementForConstructor(Document&);
  virtual HTMLElement* CreateAutonomousCustomElementSync(
      Document&,
      const QualifiedName&) = 0;
  HTMLElement* CreateElement(Document&,
                             const QualifiedName&,
                             const CreateElementFlags);

  void Upgrade(Element&);

  virtual bool HasConnectedCallback() const = 0;
  virtual bool HasDisconnectedCallback() const = 0;
  virtual bool HasConnectedMoveCallback() const = 0;
  virtual bool HasAdoptedCallback() const = 0;
  bool HasAttributeChangedCallback(const QualifiedName&) const;
  bool HasStyleAttributeChangedCallback() const;
  virtual bool HasFormAssociatedCallback() const = 0;
  virtual bool HasFormResetCallback() const = 0;
  virtual bool HasFormDisabledCallback() const = 0;
  virtual bool HasFormStateRestoreCallback() const = 0;

  virtual void RunConnectedCallback(Element&) = 0;
  virtual void RunDisconnectedCallback(Element&) = 0;
  virtual void RunConnectedMoveCallback(Element&) = 0;
  virtual void RunAdoptedCallback(Element&,
                                  Document& old_owner,
                                  Document& new_owner) = 0;
  virtual void RunAttributeChangedCallback(Element&,
                                           const QualifiedName&,
                                           const AtomicString& old_value,
                                           const AtomicString& new_value) = 0;
  virtual void RunFormAssociatedCallback(Element& element,
                                         HTMLFormElement* nullable_form) = 0;
  virtual void RunFormResetCallback(Element& element) = 0;
  virtual void RunFormDisabledCallback(Element& element, bool is_disabled) = 0;
  virtual void RunFormStateRestoreCallback(Element& element,
                                           const V8ControlValue* value,
                                           const String& mode) = 0;

  void EnqueueUpgradeReaction(Element&);
  void EnqueueConnectedCallback(Element&);
  void EnqueueDisconnectedCallback(Element&);
  void EnqueueConnectedMoveCallback(Element&);
  void EnqueueAdoptedCallback(Element&,
                              Document& old_owner,
                              Document& new_owner);
  void EnqueueAttributeChangedCallback(Element&,
                                       const QualifiedName&,
                                       const AtomicString& old_value,
                                       const AtomicString& new_value);

  bool DisableShadow() const { return disable_shadow_; }
  bool DisableInternals() const { return disable_internals_; }
  bool IsFormAssociated() const { return is_form_associated_; }

 protected:
  CustomElementDefinition(CustomElementRegistry&,
                          const CustomElementDescriptor&);

  CustomElementDefinition(CustomElementRegistry&,
                          const CustomElementDescriptor&,
                          const HashSet<AtomicString>& observed_attributes,
                          const Vector<String>& disabled_features,
                          FormAssociationFlag form_association_flag);

  virtual bool RunConstructor(Element&) = 0;

  static void CheckConstructorResult(Element*,
                                     Document&,
                                     const QualifiedName&,
                                     ExceptionState&);

 private:
  Member<CustomElementRegistry> registry_;
  const CustomElementDescriptor descriptor_;
  HashSet<AtomicString> observed_attributes_;
  bool has_style_attribute_changed_callback_;
  bool disable_shadow_ = false;
  bool disable_internals_ = false;
  bool is_form_associated_ = false;

  void EnqueueAttributeChangedCallbackForAllAttributes(Element&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_DEFINITION_H_
