// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REGISTRY_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_custom_element_constructor.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_custom_element_constructor_hash.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_union_element_shadowroot.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/custom/custom_element_definition.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class CustomElementDefinitionBuilder;
class CustomElementDescriptor;
class Document;
class Element;
class ElementDefinitionOptions;
class ExceptionState;
class LocalDOMWindow;
class ScriptState;
class ScriptValue;

class CORE_EXPORT CustomElementRegistry final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static CustomElementRegistry* Create(ScriptState*);

  explicit CustomElementRegistry(const LocalDOMWindow*);
  CustomElementRegistry(const CustomElementRegistry&) = delete;
  CustomElementRegistry& operator=(const CustomElementRegistry&) = delete;
  ~CustomElementRegistry() override = default;

  CustomElementDefinition* define(ScriptState*,
                                  const AtomicString& name,
                                  V8CustomElementConstructor* constructor,
                                  const ElementDefinitionOptions*,
                                  ExceptionState&);

  // Access to custom element definitions. Name here refers to the
  // https://whatwg.org/C/#concept-custom-element-definition-name.

  ScriptValue get(const AtomicString& name);
  const AtomicString& getName(V8CustomElementConstructor* constructor);
  bool NameIsDefined(const AtomicString& name) const;
  Vector<AtomicString> DefinedNames() const;
  CustomElementDefinition* DefinitionForName(const AtomicString& name) const;
  CustomElementDefinition* DefinitionForConstructor(
      V8CustomElementConstructor*) const;
  CustomElementDefinition* DefinitionForConstructor(
      v8::Local<v8::Object> constructor) const;

  // TODO(dominicc): Switch most callers of definitionForName to
  // definitionFor when implementing type extensions.
  CustomElementDefinition* DefinitionFor(const CustomElementDescriptor&) const;

  // TODO(dominicc): Consider broadening this API when type extensions are
  // implemented.
  void AddCandidate(Element&);
  ScriptPromise<V8CustomElementConstructor>
  whenDefined(ScriptState*, const AtomicString& name, ExceptionState&);
  void upgrade(Node* root);

  const LocalDOMWindow* GetOwnerWindow() const { return owner_.Get(); }

  bool IsGlobalRegistry() const;

  void AssociatedWith(Document& document);

  void initialize(V8UnionElementOrShadowRoot* element_or_shadowroot) {}

  void Trace(Visitor*) const override;

 private:
  CustomElementDefinition* DefineInternal(ScriptState*,
                                          const AtomicString& name,
                                          CustomElementDefinitionBuilder&,
                                          const ElementDefinitionOptions*,
                                          ExceptionState&);

  void CollectCandidates(const CustomElementDescriptor&,
                         HeapVector<Member<Element>>*);

  bool element_definition_is_running_;

  using ConstructorMap = HeapHashMap<Member<V8CustomElementConstructor>,
                                     Member<CustomElementDefinition>,
                                     V8CustomElementConstructorHashTraits>;
  ConstructorMap constructor_map_;

  using NameMap = HeapHashMap<AtomicString, Member<CustomElementDefinition>>;
  NameMap name_map_;

  Member<const LocalDOMWindow> owner_;

  using UpgradeCandidateSet = GCedHeapHashSet<WeakMember<Element>>;
  using UpgradeCandidateMap =
      GCedHeapHashMap<AtomicString, Member<UpgradeCandidateSet>>;

  // Candidate elements that can be upgraded with this registry later.
  // To make implementation simpler, we maintain a superset here, and remove
  // non-candidates before upgrading.
  Member<UpgradeCandidateMap> upgrade_candidates_;

  using WhenDefinedPromiseMap =
      HeapHashMap<AtomicString,
                  Member<ScriptPromiseResolver<V8CustomElementConstructor>>>;
  WhenDefinedPromiseMap when_defined_promise_map_;

  // Weak ordered set of all documents where this registry is used, in the order
  // of association between this registry and any tree scope in the document.
  using AssociatedDocumentSet = GCedHeapLinkedHashSet<WeakMember<Document>>;
  Member<AssociatedDocumentSet> associated_documents_;

  FRIEND_TEST_ALL_PREFIXES(
      CustomElementTest,
      CreateElement_TagNameCaseHandlingCreatingCustomElement);
  friend class CustomElementRegistryTest;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REGISTRY_H_
