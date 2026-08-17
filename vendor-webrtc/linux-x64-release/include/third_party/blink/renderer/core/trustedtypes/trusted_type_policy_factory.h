// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TRUSTEDTYPES_TRUSTED_TYPE_POLICY_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TRUSTEDTYPES_TRUSTED_TYPE_POLICY_FACTORY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class ScriptObject;
class ScriptState;
class ScriptValue;
class TrustedHTML;
class TrustedScript;
class TrustedTypePolicy;
class TrustedTypePolicyOptions;

class CORE_EXPORT TrustedTypePolicyFactory final
    : public EventTarget,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit TrustedTypePolicyFactory(ExecutionContext*);

  // TrustedTypePolicyFactory.idl
  TrustedTypePolicy* createPolicy(const String&, ExceptionState&);
  TrustedTypePolicy* createPolicy(const String&,
                                  const TrustedTypePolicyOptions*,
                                  ExceptionState&);

  TrustedTypePolicy* defaultPolicy() const;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(beforecreatepolicy, kBeforecreatepolicy)

  bool isHTML(ScriptState*, const ScriptValue&);
  bool isScript(ScriptState*, const ScriptValue&);
  bool isScriptURL(ScriptState*, const ScriptValue&);

  TrustedHTML* emptyHTML() const;

  TrustedScript* emptyScript() const;

  String getPropertyType(const String& tagName,
                         const String& propertyName,
                         const String& elementNS) const;
  String getAttributeType(const String& tagName,
                          const String& attributeName,
                          const String& tagNS,
                          const String& attributeNS) const;

  ScriptObject getTypeMapping(ScriptState*) const;
  ScriptObject getTypeMapping(ScriptState*, const String& ns) const;

  // Count whether a Trusted Type error occured during DOM operations.
  // (We aggregate this here to get a count per document, so that we can
  //  relate it to the total number of TT enabled documents.)
  void CountTrustedTypeAssignmentError();

  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;
  void Trace(Visitor*) const override;

  // Check whether a given attribute is considered an event handler.
  //
  // This function is largely unrelated to the TrustedTypePolicyFactory, but
  // it reuses the data from getTypeMapping, which is why we have defined it
  // here.
  static bool IsEventHandlerAttributeName(const AtomicString& attributeName);

 private:
  Member<TrustedHTML> empty_html_;
  Member<TrustedScript> empty_script_;
  HeapHashMap<String, Member<TrustedTypePolicy>> policy_map_;

  bool hadAssignmentError = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TRUSTEDTYPES_TRUSTED_TYPE_POLICY_FACTORY_H_
