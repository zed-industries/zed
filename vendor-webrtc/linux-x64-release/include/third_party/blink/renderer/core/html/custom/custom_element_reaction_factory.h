// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_FACTORY_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CustomElementDefinition;
class CustomElementReaction;
class Document;
class HTMLFormElement;
class QualifiedName;

class CustomElementReactionFactory {
  STATIC_ONLY(CustomElementReactionFactory);

 public:
  static CustomElementReaction& CreateUpgrade(
      CustomElementDefinition& definition);
  static CustomElementReaction& CreateConnected(
      CustomElementDefinition& definition);
  static CustomElementReaction& CreateDisconnected(
      CustomElementDefinition& definition);
  static CustomElementReaction& CreateConnectedMove(
      CustomElementDefinition& definition);
  static CustomElementReaction& CreateAdopted(
      CustomElementDefinition& definition,
      Document& old_owner,
      Document& new_owner);
  static CustomElementReaction& CreateAttributeChanged(
      CustomElementDefinition& definition,
      const QualifiedName& name,
      const AtomicString& old_value,
      const AtomicString& new_value);
  static CustomElementReaction& CreateFormAssociated(
      CustomElementDefinition& definition,
      HTMLFormElement* nullable_form);
  static CustomElementReaction& CreateFormReset(
      CustomElementDefinition& definition);
  static CustomElementReaction& CreateFormDisabled(
      CustomElementDefinition& definition,
      bool is_disabled);
  static CustomElementReaction& CreateFormStateRestore(
      CustomElementDefinition& definition,
      const V8ControlValue* value,
      const WTF::String& mode);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_FACTORY_H_
