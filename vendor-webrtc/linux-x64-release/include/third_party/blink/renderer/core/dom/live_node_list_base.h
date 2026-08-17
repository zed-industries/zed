/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2006, 2007 Apple Inc. All rights reserved.
 * Copyright (C) 2014 Samsung Electronics. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LIVE_NODE_LIST_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LIVE_NODE_LIST_BASE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element_traversal.h"
#include "third_party/blink/renderer/core/dom/node_list_invalidation_type.h"
#include "third_party/blink/renderer/core/html/collection_type.h"
#include "third_party/blink/renderer/core/html_names.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Document;

enum class NodeListSearchRoot {
  kOwnerNode,
  kTreeScope,
};

class CORE_EXPORT LiveNodeListBase : public GarbageCollectedMixin {
 public:
  LiveNodeListBase(ContainerNode& owner_node,
                   NodeListSearchRoot search_root,
                   NodeListInvalidationType invalidation_type,
                   CollectionType collection_type)
      : owner_node_(owner_node),
        search_root_(static_cast<unsigned>(search_root)),
        invalidation_type_(invalidation_type),
        collection_type_(collection_type) {
    DCHECK_EQ(search_root_, static_cast<unsigned>(search_root));
    DCHECK_EQ(invalidation_type_, static_cast<unsigned>(invalidation_type));
    DCHECK_EQ(collection_type_, static_cast<unsigned>(collection_type));
  }

  virtual ~LiveNodeListBase() = default;

  virtual ContainerNode& RootNode() const;

  void DidMoveToDocument(Document& old_document, Document& new_document);
  ALWAYS_INLINE bool IsRootedAtTreeScope() const {
    return search_root_ ==
           static_cast<unsigned>(NodeListSearchRoot::kTreeScope);
  }
  ALWAYS_INLINE NodeListInvalidationType InvalidationType() const {
    return static_cast<NodeListInvalidationType>(invalidation_type_);
  }
  ALWAYS_INLINE CollectionType GetType() const {
    return static_cast<CollectionType>(collection_type_);
  }
  ContainerNode& ownerNode() const { return *owner_node_; }

  virtual void InvalidateCache(Document* old_document = nullptr) const = 0;
  void InvalidateCacheForAttribute(const QualifiedName*) const;

  static bool ShouldInvalidateTypeOnAttributeChange(NodeListInvalidationType,
                                                    const QualifiedName&);

  void Trace(Visitor* visitor) const override { visitor->Trace(owner_node_); }

 protected:
  Document& GetDocument() const { return owner_node_->GetDocument(); }

  template <typename MatchFunc>
  static Element* TraverseMatchingElementsForwardToOffset(
      Element& current_element,
      const ContainerNode* stay_within,
      unsigned offset,
      unsigned& current_offset,
      MatchFunc);
  template <typename MatchFunc>
  static Element* TraverseMatchingElementsBackwardToOffset(
      Element& current_element,
      const ContainerNode* stay_within,
      unsigned offset,
      unsigned& current_offset,
      MatchFunc);

 private:
  Member<ContainerNode> owner_node_;  // Cannot be null.
  const unsigned search_root_ : 1;
  const unsigned invalidation_type_ : 4;
  const unsigned collection_type_ : 5;
};

ALWAYS_INLINE bool LiveNodeListBase::ShouldInvalidateTypeOnAttributeChange(
    NodeListInvalidationType type,
    const QualifiedName& attr_name) {
  switch (type) {
    case kInvalidateOnClassAttrChange:
      return attr_name == html_names::kClassAttr;
    case kInvalidateOnNameAttrChange:
      return attr_name == html_names::kNameAttr;
    case kInvalidateOnIdNameAttrChange:
      return attr_name == html_names::kIdAttr ||
             attr_name == html_names::kNameAttr;
    case kInvalidateOnForAttrChange:
      return attr_name == html_names::kForAttr;
    case kInvalidateForFormControls:
      return attr_name == html_names::kNameAttr ||
             attr_name == html_names::kIdAttr ||
             attr_name == html_names::kForAttr ||
             attr_name == html_names::kFormAttr ||
             attr_name == html_names::kTypeAttr;
    case kInvalidateOnHRefAttrChange:
      return attr_name == html_names::kHrefAttr;
    case kInvalidateOnPopoverInvokerAttrChange:
      return attr_name == html_names::kPopoverAttr ||
             attr_name == html_names::kPopovertargetAttr ||
             attr_name == html_names::kPopovertargetactionAttr;
    case kInvalidateOnCommandInvokerAttrChange:
      return attr_name == html_names::kCommandAttr ||
             attr_name == html_names::kCommandforAttr;
    case kDoNotInvalidateOnAttributeChanges:
      return false;
    case kInvalidateOnAnyAttrChange:
      return true;
  }
  return false;
}

template <typename MatchFunc>
Element* LiveNodeListBase::TraverseMatchingElementsForwardToOffset(
    Element& current_element,
    const ContainerNode* stay_within,
    unsigned offset,
    unsigned& current_offset,
    MatchFunc is_match) {
  DCHECK_LT(current_offset, offset);
  for (Element* next =
           ElementTraversal::Next(current_element, stay_within, is_match);
       next; next = ElementTraversal::Next(*next, stay_within, is_match)) {
    if (++current_offset == offset)
      return next;
  }
  return nullptr;
}

template <typename MatchFunc>
Element* LiveNodeListBase::TraverseMatchingElementsBackwardToOffset(
    Element& current_element,
    const ContainerNode* stay_within,
    unsigned offset,
    unsigned& current_offset,
    MatchFunc is_match) {
  DCHECK_GT(current_offset, offset);
  for (Element* previous =
           ElementTraversal::Previous(current_element, stay_within, is_match);
       previous; previous = ElementTraversal::Previous(*previous, stay_within,
                                                       is_match)) {
    if (--current_offset == offset)
      return previous;
  }
  return nullptr;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LIVE_NODE_LIST_BASE_H_
