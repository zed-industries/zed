/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012, 2013
 * Apple Inc. All rights reserved.
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ELEMENT_TRAVERSAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ELEMENT_TRAVERSAL_H_

#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/node_traversal.h"
#include "third_party/blink/renderer/core/dom/traversal_range.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class HasTagName {
  STACK_ALLOCATED();

 public:
  explicit HasTagName(const QualifiedName& tag_name) : tag_name_(tag_name) {}
  bool operator()(const Element& element) const {
    return element.HasTagName(tag_name_);
  }

 private:
  const QualifiedName tag_name_;
};

// This class is used to traverse the DOM tree. It isn't meant to be
// constructed; instead, callers invoke the static methods, after templating it
// so that ElementType is the type of element they are interested in traversing.
// Traversals can also be predicated on a matcher, which will be used to
// filter the returned elements. A matcher is a callable - an object of a class
// that defines operator(). HasTagName above is an example of a matcher.
//
// For example, a caller could do this:
//   Traversal<Element>::firstChild(some_node,
//                                  HasTagName(html_names::kTitleTag));
//
// This invocation would return the first child of |some_node| (which has to be
// a ContainerNode) for which HasTagName(html_names::kTitleTag) returned true,
// so it would return the first child of |someNode| which is a <title> element.
// If the caller needs to traverse a Node this way, it's necessary to first
// check Node::IsContainerNode() and then use To<ContainerNode>(). Another way
// to achieve same behaviour is to use DynamicTo<ContainerNode>() which
// checks Node::IsContainerNode() and then returns container
// node. If the conditional check fails then it returns nullptr.
// DynamicTo<ContainerNode>() wraps IsContainerNode() so there is no need of
// an explicit conditional check.
//
// When looking for a specific element type, it is more efficient to do this:
//   Traversal<HTMLTitleElement>::firstChild(someNode);
//
// Traversal can also be used to find ancestors and descendants; see the
// documentation in the class body below.
//
// Note that these functions do not traverse into child shadow trees of any
// shadow hosts they encounter. If you need to traverse the shadow DOM, you can
// manually traverse the shadow trees using a second Traversal, or use
// FlatTreeTraversal.
//
// ElementTraversal is a specialized version of Traversal<Element>.
template <class ElementType>
class Traversal {
  STATIC_ONLY(Traversal);

 public:
  using TraversalNodeType = ElementType;
  // First or last ElementType child of the node.
  static ElementType* FirstChild(const ContainerNode& current) {
    return FirstChildTemplate(current);
  }
  static ElementType* FirstChild(const Node& current) {
    return FirstChildTemplate(current);
  }
  template <class MatchFunc>
  static ElementType* FirstChild(const ContainerNode&, MatchFunc);
  static ElementType* LastChild(const ContainerNode& current) {
    return LastChildTemplate(current);
  }
  static ElementType* LastChild(const Node& current) {
    return LastChildTemplate(current);
  }
  template <class MatchFunc>
  static ElementType* LastChild(const ContainerNode&, MatchFunc);

  // First ElementType ancestor of the node.
  static ElementType* FirstAncestor(const Node& current);
  static ElementType* FirstAncestorOrSelf(Node& current) {
    return FirstAncestorOrSelfTemplate(current);
  }
  static ElementType* FirstAncestorOrSelf(Element& current) {
    return FirstAncestorOrSelfTemplate(current);
  }
  static const ElementType* FirstAncestorOrSelf(const Node& current) {
    return FirstAncestorOrSelfTemplate(const_cast<Node&>(current));
  }
  static const ElementType* FirstAncestorOrSelf(const Element& current) {
    return FirstAncestorOrSelfTemplate(const_cast<Element&>(current));
  }

  // First or last ElementType descendant of the node.
  // For pure Elements firstWithin() is always the same as firstChild().
  static ElementType* FirstWithin(const ContainerNode& current) {
    return FirstWithinTemplate(current);
  }
  static ElementType* FirstWithin(const Node& current) {
    return FirstWithinTemplate(current);
  }
  template <typename MatchFunc>
  static ElementType* FirstWithin(const ContainerNode&, MatchFunc);

  static ElementType* InclusiveFirstWithin(Node& current) {
    auto* first = DynamicTo<ElementType>(current);
    return first ? first : FirstWithin(current);
  }

  static ElementType* LastWithin(const ContainerNode& current) {
    return LastWithinTemplate(current);
  }
  static ElementType* LastWithin(const Node& current) {
    return LastWithinTemplate(current);
  }
  template <class MatchFunc>
  static ElementType* LastWithin(const ContainerNode&, MatchFunc);
  static const ElementType* LastWithinOrSelf(const ElementType&);

  // Pre-order traversal skipping non-element nodes.
  static ElementType* Next(const ContainerNode& current) {
    return NextTemplate(current);
  }
  static ElementType* Next(const Node& current) {
    return NextTemplate(current);
  }
  static ElementType* Next(const ContainerNode& current,
                           const Node* stay_within) {
    return NextTemplate(current, stay_within);
  }
  static ElementType* Next(const Node& current, const Node* stay_within) {
    return NextTemplate(current, stay_within);
  }
  template <class MatchFunc>
  static ElementType* Next(const ContainerNode& current,
                           const Node* stay_within,
                           MatchFunc);
  static ElementType* Previous(const Node&);
  static ElementType* Previous(const Node&, const Node* stay_within);
  template <class MatchFunc>
  static ElementType* Previous(const ContainerNode& current,
                               const Node* stay_within,
                               MatchFunc);

  // Returns the previous direct sibling of the node, if there is one. If not,
  // it will traverse up the ancestor chain until it finds an ancestor
  // that has a previous sibling, returning that sibling. Or nullptr if none.
  // See comment for |FlatTreeTraversal::PreviousAbsoluteSibling| for details.
  static ElementType* PreviousAbsoluteSibling(const Node&,
                                              const Node* stay_within);

  // Like next, but skips children.  If you're looking for the "Previous"
  // version of this method, see PreviousAbsoluteSibling().
  static ElementType* NextSkippingChildren(const Node&);
  static ElementType* NextSkippingChildren(const Node&,
                                           const Node* stay_within);

  // Pre-order traversal including the pseudo-elements.
  static ElementType* PreviousIncludingPseudo(
      const Node&,
      const Node* stay_within = nullptr);
  static ElementType* NextIncludingPseudo(const Node&,
                                          const Node* stay_within = nullptr);
  static ElementType* NextIncludingPseudoSkippingChildren(
      const Node&,
      const Node* stay_within = nullptr);

  // Utility function to traverse only the element and pseudo-element siblings
  // of a node.
  static ElementType* PseudoAwarePreviousSibling(const Node&);

  // Previous / Next sibling.
  static ElementType* PreviousSibling(const Node&);
  template <class MatchFunc>
  static ElementType* PreviousSibling(const Node&, MatchFunc);
  static ElementType* NextSibling(const Node&);
  template <class MatchFunc>
  static ElementType* NextSibling(const Node&, MatchFunc);

  static TraversalSiblingRange<Traversal<ElementType>> ChildrenOf(const Node&);
  static TraversalDescendantRange<Traversal<ElementType>> DescendantsOf(
      const Node&);
  static TraversalInclusiveDescendantRange<Traversal<ElementType>>
  InclusiveDescendantsOf(const ElementType&);
  static TraversalNextRange<Traversal<ElementType>> StartsAt(
      const ElementType&);
  static TraversalNextRange<Traversal<ElementType>> StartsAfter(const Node&);

 private:
  template <class NodeType>
  static ElementType* FirstChildTemplate(NodeType&);
  template <class NodeType>
  static ElementType* LastChildTemplate(NodeType&);
  template <class NodeType>
  static ElementType* FirstAncestorOrSelfTemplate(NodeType&);
  template <class NodeType>
  static ElementType* FirstWithinTemplate(NodeType&);
  template <class NodeType>
  static ElementType* LastWithinTemplate(NodeType&);
  template <class NodeType>
  static ElementType* NextTemplate(NodeType&);
  template <class NodeType>
  static ElementType* NextTemplate(NodeType&, const Node* stay_within);
};

typedef Traversal<Element> ElementTraversal;

template <class ElementType>
inline TraversalSiblingRange<Traversal<ElementType>>
Traversal<ElementType>::ChildrenOf(const Node& start) {
  return TraversalSiblingRange<Traversal<ElementType>>(
      Traversal<ElementType>::FirstChild(start));
}

template <class ElementType>
inline TraversalDescendantRange<Traversal<ElementType>>
Traversal<ElementType>::DescendantsOf(const Node& root) {
  return TraversalDescendantRange<Traversal<ElementType>>(&root);
}

template <class ElementType>
inline TraversalInclusiveDescendantRange<Traversal<ElementType>>
Traversal<ElementType>::InclusiveDescendantsOf(const ElementType& root) {
  return TraversalInclusiveDescendantRange<Traversal<ElementType>>(&root);
}

template <class ElementType>
inline TraversalNextRange<Traversal<ElementType>>
Traversal<ElementType>::StartsAt(const ElementType& start) {
  return TraversalNextRange<Traversal<ElementType>>(&start);
}

template <class ElementType>
inline TraversalNextRange<Traversal<ElementType>>
Traversal<ElementType>::StartsAfter(const Node& start) {
  return TraversalNextRange<Traversal<ElementType>>(
      Traversal<ElementType>::Next(start));
}

// Specialized for pure Element to exploit the fact that Elements parent is
// always either another Element or the root.
template <>
template <class NodeType>
inline Element* Traversal<Element>::FirstWithinTemplate(NodeType& current) {
  return FirstChildTemplate(current);
}

template <>
template <class NodeType>
inline Element* Traversal<Element>::NextTemplate(NodeType& current) {
  Node* node = NodeTraversal::Next(current);
  while (node && !node->IsElementNode())
    node = NodeTraversal::NextSkippingChildren(*node);
  return To<Element>(node);
}

template <>
template <class NodeType>
inline Element* Traversal<Element>::NextTemplate(NodeType& current,
                                                 const Node* stay_within) {
  Node* node = NodeTraversal::Next(current, stay_within);
  while (node && !node->IsElementNode())
    node = NodeTraversal::NextSkippingChildren(*node, stay_within);
  return To<Element>(node);
}

// Generic versions.
template <class ElementType>
template <class NodeType>
inline ElementType* Traversal<ElementType>::FirstChildTemplate(
    NodeType& current) {
  for (Node* node = current.firstChild(); node; node = node->nextSibling()) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
template <class MatchFunc>
inline ElementType* Traversal<ElementType>::FirstChild(
    const ContainerNode& current,
    MatchFunc is_match) {
  ElementType* element = Traversal<ElementType>::FirstChild(current);
  while (element && !is_match(*element))
    element = Traversal<ElementType>::NextSibling(*element);
  return element;
}

template <class ElementType>
inline ElementType* Traversal<ElementType>::FirstAncestor(const Node& current) {
  for (ContainerNode* ancestor = current.parentNode(); ancestor;
       ancestor = ancestor->parentNode()) {
    if (auto* element = DynamicTo<ElementType>(*ancestor)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
template <class NodeType>
inline ElementType* Traversal<ElementType>::FirstAncestorOrSelfTemplate(
    NodeType& current) {
  if (auto* element = DynamicTo<ElementType>(current)) {
    return element;
  }
  return FirstAncestor(current);
}

template <class ElementType>
template <class NodeType>
inline ElementType* Traversal<ElementType>::LastChildTemplate(
    NodeType& current) {
  for (Node* node = current.lastChild(); node; node = node->previousSibling()) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
template <class MatchFunc>
inline ElementType* Traversal<ElementType>::LastChild(
    const ContainerNode& current,
    MatchFunc is_match) {
  ElementType* element = Traversal<ElementType>::LastChild(current);
  while (element && !is_match(*element))
    element = Traversal<ElementType>::PreviousSibling(*element);
  return element;
}

template <class ElementType>
template <class NodeType>
inline ElementType* Traversal<ElementType>::FirstWithinTemplate(
    NodeType& current) {
  for (Node* node = current.firstChild(); node;
       node = NodeTraversal::Next(*node, &current)) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
template <typename MatchFunc>
inline ElementType* Traversal<ElementType>::FirstWithin(
    const ContainerNode& current,
    MatchFunc is_match) {
  ElementType* element = Traversal<ElementType>::FirstWithin(current);
  while (element && !is_match(*element))
    element = Traversal<ElementType>::Next(*element, &current, is_match);
  return element;
}

template <class ElementType>
template <class NodeType>
inline ElementType* Traversal<ElementType>::LastWithinTemplate(
    NodeType& current) {
  for (Node* node = NodeTraversal::LastWithin(current); node;
       node = NodeTraversal::Previous(*node, &current)) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
template <class MatchFunc>
inline ElementType* Traversal<ElementType>::LastWithin(
    const ContainerNode& current,
    MatchFunc is_match) {
  ElementType* element = Traversal<ElementType>::LastWithin(current);
  while (element && !is_match(*element))
    element = Traversal<ElementType>::Previous(*element, &current, is_match);
  return element;
}

template <class ElementType>
inline const ElementType* Traversal<ElementType>::LastWithinOrSelf(
    const ElementType& current) {
  if (auto* last_descendant = LastWithin(current)) {
    return last_descendant;
  }
  return &current;
}

template <class ElementType>
template <class NodeType>
inline ElementType* Traversal<ElementType>::NextTemplate(NodeType& current) {
  for (Node* node = NodeTraversal::Next(current); node;
       node = NodeTraversal::Next(*node)) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
template <class NodeType>
inline ElementType* Traversal<ElementType>::NextTemplate(
    NodeType& current,
    const Node* stay_within) {
  for (Node* node = NodeTraversal::Next(current, stay_within); node;
       node = NodeTraversal::Next(*node, stay_within)) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
template <class MatchFunc>
inline ElementType* Traversal<ElementType>::Next(const ContainerNode& current,
                                                 const Node* stay_within,
                                                 MatchFunc is_match) {
  ElementType* element = Traversal<ElementType>::Next(current, stay_within);
  while (element && !is_match(*element))
    element = Traversal<ElementType>::Next(*element, stay_within);
  return element;
}

template <class ElementType>
inline ElementType* Traversal<ElementType>::Previous(const Node& current) {
  for (Node* node = NodeTraversal::Previous(current); node;
       node = NodeTraversal::Previous(*node)) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
inline ElementType* Traversal<ElementType>::Previous(const Node& current,
                                                     const Node* stay_within) {
  for (Node* node = NodeTraversal::Previous(current, stay_within); node;
       node = NodeTraversal::Previous(*node, stay_within)) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
template <class MatchFunc>
inline ElementType* Traversal<ElementType>::Previous(
    const ContainerNode& current,
    const Node* stay_within,
    MatchFunc is_match) {
  ElementType* element = Traversal<ElementType>::Previous(current, stay_within);
  while (element && !is_match(*element))
    element = Traversal<ElementType>::Previous(*element, stay_within);
  return element;
}

template <class ElementType>
inline ElementType* Traversal<ElementType>::PreviousAbsoluteSibling(
    const Node& current,
    const Node* stay_within) {
  for (Node* node =
           NodeTraversal::PreviousAbsoluteSibling(current, stay_within);
       node;
       node = NodeTraversal::PreviousAbsoluteSibling(*node, stay_within)) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }

  return nullptr;
}

template <class ElementType>
inline ElementType* Traversal<ElementType>::NextSkippingChildren(
    const Node& current) {
  for (Node* node = NodeTraversal::NextSkippingChildren(current); node;
       node = NodeTraversal::NextSkippingChildren(*node)) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
inline ElementType* Traversal<ElementType>::NextSkippingChildren(
    const Node& current,
    const Node* stay_within) {
  for (Node* node = NodeTraversal::NextSkippingChildren(current, stay_within);
       node; node = NodeTraversal::NextSkippingChildren(*node, stay_within)) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
inline ElementType* Traversal<ElementType>::PreviousIncludingPseudo(
    const Node& current,
    const Node* stay_within) {
  for (Node* node =
           NodeTraversal::PreviousIncludingPseudo(current, stay_within);
       node;
       node = NodeTraversal::PreviousIncludingPseudo(*node, stay_within)) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
inline ElementType* Traversal<ElementType>::NextIncludingPseudo(
    const Node& current,
    const Node* stay_within) {
  for (Node* node = NodeTraversal::NextIncludingPseudo(current, stay_within);
       node; node = NodeTraversal::NextIncludingPseudo(*node, stay_within)) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
inline ElementType* Traversal<ElementType>::NextIncludingPseudoSkippingChildren(
    const Node& current,
    const Node* stay_within) {
  for (Node* node = NodeTraversal::NextIncludingPseudoSkippingChildren(
           current, stay_within);
       node; node = NodeTraversal::NextIncludingPseudoSkippingChildren(
                 *node, stay_within)) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
inline ElementType* Traversal<ElementType>::PseudoAwarePreviousSibling(
    const Node& current) {
  for (Node* node = current.PseudoAwarePreviousSibling(); node;
       node = node->PseudoAwarePreviousSibling()) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
inline ElementType* Traversal<ElementType>::PreviousSibling(
    const Node& current) {
  for (Node* node = current.previousSibling(); node;
       node = node->previousSibling()) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
template <class MatchFunc>
inline ElementType* Traversal<ElementType>::PreviousSibling(
    const Node& current,
    MatchFunc is_match) {
  ElementType* element = Traversal<ElementType>::PreviousSibling(current);
  while (element && !is_match(*element))
    element = Traversal<ElementType>::PreviousSibling(*element);
  return element;
}

template <class ElementType>
inline ElementType* Traversal<ElementType>::NextSibling(const Node& current) {
  for (Node* node = current.nextSibling(); node; node = node->nextSibling()) {
    if (auto* element = DynamicTo<ElementType>(*node)) {
      return element;
    }
  }
  return nullptr;
}

template <class ElementType>
template <class MatchFunc>
inline ElementType* Traversal<ElementType>::NextSibling(const Node& current,
                                                        MatchFunc is_match) {
  ElementType* element = Traversal<ElementType>::NextSibling(current);
  while (element && !is_match(*element))
    element = Traversal<ElementType>::NextSibling(*element);
  return element;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ELEMENT_TRAVERSAL_H_
