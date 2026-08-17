// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COUNTER_STYLE_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COUNTER_STYLE_MAP_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/counter_style.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class Document;
class TreeScope;
class CounterStyle;
class RuleSet;

class CORE_EXPORT CounterStyleMap : public GarbageCollected<CounterStyleMap> {
 public:
  static CounterStyleMap* GetUACounterStyleMap();
  static CounterStyleMap* GetUserCounterStyleMap(Document&);
  static CounterStyleMap* GetAuthorCounterStyleMap(const TreeScope&);

  static CounterStyleMap* CreateUserCounterStyleMap(Document&);
  static CounterStyleMap* CreateAuthorCounterStyleMap(TreeScope&);

  CounterStyle* FindCounterStyleAcrossScopes(const AtomicString& name) const;

  void AddCounterStyles(const RuleSet&);

  void ResolveReferences(HeapHashSet<Member<CounterStyleMap>>& resolved_maps);
  static void ResolveAllReferences(Document&,
                                   const HeapHashSet<Member<TreeScope>>&);

  void MarkDirtyCounterStyles(HeapHashSet<Member<CounterStyle>>& visited);
  static void MarkAllDirtyCounterStyles(Document&,
                                        const HeapHashSet<Member<TreeScope>>&);

  void Dispose();

  CounterStyleMap(Document* document, TreeScope* tree_scope);
  void Trace(Visitor*) const;

 private:
  CounterStyleMap* GetAncestorMap() const;

  void ResolveExtendsFor(CounterStyle&);
  void ResolveFallbackFor(CounterStyle&);
  void ResolveSpeakAsReferenceFor(CounterStyle&);

  static CounterStyleMap* CreateUACounterStyleMap();
  CounterStyle& CreateUACounterStyle(const AtomicString& name);

  // Null means these are user-agent rules.
  Member<Document> owner_document_;

  // Null tree scope and non-null document means these are user rules.
  Member<TreeScope> tree_scope_;

  HeapHashMap<AtomicString, Member<CounterStyle>> counter_styles_;

  friend class CounterStyleMapTest;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COUNTER_STYLE_MAP_H_
