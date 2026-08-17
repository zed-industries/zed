/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2008 Nikolas Zimmermann <zimmermann@kde.org>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_SCRIPT_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_SCRIPT_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/create_element_flags.h"
#include "third_party/blink/renderer/core/html/blocking_attribute.h"
#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/core/probe/async_task_context.h"
#include "third_party/blink/renderer/core/script/script_element_base.h"
#include "third_party/blink/renderer/core/script/script_loader.h"
#include "third_party/blink/renderer/platform/bindings/parkable_string.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class ExceptionState;

class CORE_EXPORT HTMLScriptElement final : public HTMLElement,
                                            public ScriptElementBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static bool supports(const AtomicString&);

  HTMLScriptElement(Document&, const CreateElementFlags);

  // Returns attributes that should be checked against Trusted Types
  const AttrNameToTrustedType& GetCheckedAttributeTypes() const override;

  String text() { return TextFromChildren(); }
  void setText(const String&);
  void setInnerTextForBinding(
      const V8UnionStringLegacyNullToEmptyStringOrTrustedScript*
          string_or_trusted_script,
      ExceptionState& exception_state) override;
  void setTextContentForBinding(const V8UnionStringOrTrustedScript* value,
                                ExceptionState& exception_state) override;
  void setTextContent(const String&) override;

  void setAsync(bool);
  bool async() const;

  BlockingAttribute& blocking() const { return *blocking_attribute_; }

  ScriptLoader* Loader() const final { return loader_.Get(); }

  bool IsScriptElement() const override { return true; }
  Document& GetDocument() const override;
  ExecutionContext* GetExecutionContext() const override;

  V8HTMLOrSVGScriptElement* AsV8HTMLOrSVGScriptElement() override;
  DOMNodeId GetDOMNodeId() override;

  void Trace(Visitor*) const override;

  void FinishParsingChildren() override;

  bool IsPotentiallyRenderBlocking() const override;

 private:
  void ParseAttribute(const AttributeModificationParams&) override;
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode& insertion_point) override;

  void DidNotifySubtreeInsertionsToDocument() override;
  void ChildrenChanged(const ChildrenChange&) override;

  bool IsURLAttribute(const Attribute&) const override;
  bool HasLegalLinkAttribute(const QualifiedName&) const override;

  // ScriptElementBase overrides:
  String SourceAttributeValue() const override;
  String CharsetAttributeValue() const override;
  String TypeAttributeValue() const override;
  String LanguageAttributeValue() const override;
  bool NomoduleAttributeValue() const override;
  String ForAttributeValue() const override;
  String EventAttributeValue() const override;
  String CrossOriginAttributeValue() const override;
  String IntegrityAttributeValue() const override;
  String SignatureAttributeValue() const override;
  String ReferrerPolicyAttributeValue() const override;
  String FetchPriorityAttributeValue() const override;
  String ChildTextContent() override;
  String ScriptTextInternalSlot() const override;
  bool AsyncAttributeValue() const override;
  bool DeferAttributeValue() const override;
  bool HasSourceAttribute() const override;
  bool HasAttributionsrcAttribute() const override;
  bool IsConnected() const override;
  bool HasChildren() const override;
  const AtomicString& GetNonceForElement() const override;
  bool ElementHasDuplicateAttributes() const override {
    return HasDuplicateAttribute();
  }
  bool AllowInlineScriptForCSP(const AtomicString& nonce,
                               const WTF::OrdinalNumber&,
                               const String& script_content) override;
  void DispatchLoadEvent() override;
  void DispatchErrorEvent() override;

  Type GetScriptElementType() override;

  Element& CloneWithoutAttributesAndChildren(Document&) const override;

  // https://w3c.github.io/trusted-types/dist/spec/#script-scripttext
  ParkableString script_text_internal_slot_;
  bool children_changed_by_api_;

  Member<BlockingAttribute> blocking_attribute_;
  Member<ScriptLoader> loader_;

  probe::AsyncTaskContext async_task_context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_SCRIPT_ELEMENT_H_
