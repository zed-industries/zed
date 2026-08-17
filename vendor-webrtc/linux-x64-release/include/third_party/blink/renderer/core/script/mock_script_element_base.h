// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MOCK_SCRIPT_ELEMENT_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MOCK_SCRIPT_ELEMENT_BASE_H_

#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/script/script_element_base.h"

namespace blink {

class MockScriptElementBase : public GarbageCollected<MockScriptElementBase>,
                              public ScriptElementBase {
 public:
  static MockScriptElementBase* Create() {
    return MakeGarbageCollected<testing::StrictMock<MockScriptElementBase>>();
  }
  virtual ~MockScriptElementBase() {}

  MOCK_METHOD0(DispatchLoadEvent, void());
  MOCK_METHOD0(DispatchErrorEvent, void());
  MOCK_CONST_METHOD0(AsyncAttributeValue, bool());
  MOCK_CONST_METHOD0(CharsetAttributeValue, String());
  MOCK_CONST_METHOD0(CrossOriginAttributeValue, String());
  MOCK_CONST_METHOD0(DeferAttributeValue, bool());
  MOCK_CONST_METHOD0(EventAttributeValue, String());
  MOCK_CONST_METHOD0(ForAttributeValue, String());
  MOCK_CONST_METHOD0(IntegrityAttributeValue, String());
  MOCK_CONST_METHOD0(SignatureAttributeValue, String());
  MOCK_CONST_METHOD0(ReferrerPolicyAttributeValue, String());
  MOCK_CONST_METHOD0(FetchPriorityAttributeValue, String());
  MOCK_CONST_METHOD0(LanguageAttributeValue, String());
  MOCK_CONST_METHOD0(NomoduleAttributeValue, bool());
  MOCK_CONST_METHOD0(SourceAttributeValue, String());
  MOCK_CONST_METHOD0(TypeAttributeValue, String());
  MOCK_METHOD0(ChildTextContent, String());
  MOCK_CONST_METHOD0(ScriptTextInternalSlot, String());
  MOCK_CONST_METHOD0(HasSourceAttribute, bool());
  MOCK_CONST_METHOD0(HasAttributionsrcAttribute, bool());
  MOCK_CONST_METHOD0(IsConnected, bool());
  MOCK_CONST_METHOD0(HasChildren, bool());
  MOCK_CONST_METHOD0(GetNonceForElement, const AtomicString&());
  MOCK_CONST_METHOD0(ElementHasDuplicateAttributes, bool());
  MOCK_CONST_METHOD0(InitiatorName, AtomicString());
  MOCK_CONST_METHOD0(IsPotentiallyRenderBlocking, bool());
  MOCK_METHOD3(AllowInlineScriptForCSP,
               bool(const AtomicString&,
                    const WTF::OrdinalNumber&,
                    const String&));
  MOCK_CONST_METHOD0(GetDocument, Document&());
  MOCK_CONST_METHOD0(GetExecutionContext, ExecutionContext*());
  MOCK_METHOD0(AsV8HTMLOrSVGScriptElement, V8HTMLOrSVGScriptElement*());
  MOCK_METHOD0(GetDOMNodeId, DOMNodeId());
  MOCK_METHOD1(SetScriptElementForBinding,
               void(HTMLScriptElementOrSVGScriptElement&));
  MOCK_CONST_METHOD0(Loader, ScriptLoader*());

  ScriptElementBase::Type GetScriptElementType() override {
    return ScriptElementBase::Type::kHTMLScriptElement;
  }
  void Trace(Visitor* visitor) const override {
    ScriptElementBase::Trace(visitor);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MOCK_SCRIPT_ELEMENT_BASE_H_
