/*
 * Copyright (C) 2011, 2013 Apple Inc. All rights reserved.
 * Copyright (C) 2014 Samsung Electronics. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SELECTOR_QUERY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SELECTOR_QUERY_H_

#include "base/compiler_specific.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_selector_list.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CSSSelector;
class ContainerNode;
class Document;
class Element;
class ExceptionState;
template <typename NodeType>
class StaticNodeTypeList;
using StaticElementList = StaticNodeTypeList<Element>;

class CORE_EXPORT SelectorQuery : public GarbageCollected<SelectorQuery> {
 public:
  explicit SelectorQuery(CSSSelectorList*);
  SelectorQuery(const SelectorQuery&) = delete;
  SelectorQuery& operator=(const SelectorQuery&) = delete;

  // https://dom.spec.whatwg.org/#dom-element-matches
  bool Matches(Element&) const;

  // https://dom.spec.whatwg.org/#dom-element-closest
  Element* Closest(Element&) const;

  // https://dom.spec.whatwg.org/#dom-parentnode-queryselectorall
  StaticElementList* QueryAll(ContainerNode& root_node) const;

  // https://dom.spec.whatwg.org/#dom-parentnode-queryselector
  Element* QueryFirst(ContainerNode& root_node) const;

  struct QueryStats {
    unsigned total_count;
    unsigned fast_id;
    unsigned fast_class;
    unsigned fast_tag_name;
    unsigned fast_scan;
    unsigned slow_scan;
    unsigned slow_traversing_shadow_tree_scan;
  };
  // Used by unit tests to get information about what paths were taken during
  // the last query. Always reset between queries. This system is disabled in
  // non DCHECK builds to avoid the overhead on the query process.
  static QueryStats LastQueryStats();

  void Trace(Visitor* visitor) const { visitor->Trace(selector_list_); }

 private:
  template <typename SelectorQueryTrait>
  void ExecuteWithId(ContainerNode& root_node,
                     typename SelectorQueryTrait::OutputType&) const;
  template <typename SelectorQueryTrait>
  void FindTraverseRootsAndExecute(
      ContainerNode& root_node,
      typename SelectorQueryTrait::OutputType&) const;
  template <typename SelectorQueryTrait>
  void ExecuteForTraverseRoot(ContainerNode& traverse_root,
                              ContainerNode& root_node,
                              typename SelectorQueryTrait::OutputType&) const;
  template <typename SelectorQueryTrait>
  void ExecuteSlow(ContainerNode& root_node,
                   typename SelectorQueryTrait::OutputType&) const;
  template <typename SelectorQueryTrait>
  void Execute(ContainerNode& root_node,
               typename SelectorQueryTrait::OutputType&) const;

  bool SelectorListMatches(ContainerNode& root_node, Element&) const;

  const CSSSelector* StartOfComplexSelector(unsigned index) const {
    return UNSAFE_TODO(selector_list_->First() +
                       selector_start_offsets_[index]);
  }

  Member<CSSSelectorList> selector_list_;
  // Contains the start of each complex selector (relative to the first selector
  // in selector_list_; we cannot store pointers due to Oilpan restrictions),
  // but without ones that could never match like pseudo elements, div::before.
  // This can be empty, while |selector_list_| will never be empty, as
  // SelectorQueryCache::add would have thrown an exception.
  Vector<unsigned, 4> selector_start_offsets_;
  AtomicString selector_id_;
  bool selector_id_is_rightmost_ : 1;
  bool selector_id_affected_by_sibling_combinator_ : 1;
  bool use_slow_scan_ : 1;
};

class SelectorQueryCache : public GarbageCollected<SelectorQueryCache> {
 public:
  SelectorQuery* Add(const AtomicString&, const Document&, ExceptionState&);
  void Invalidate();

  void Trace(Visitor* visitor) const { visitor->Trace(entries_); }

 private:
  HeapHashMap<AtomicString, Member<SelectorQuery>> entries_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SELECTOR_QUERY_H_
