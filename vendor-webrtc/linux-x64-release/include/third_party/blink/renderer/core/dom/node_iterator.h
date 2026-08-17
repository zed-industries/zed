/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2000 Frederik Holljen (frederik.holljen@hig.no)
 * Copyright (C) 2001 Peter Kelly (pmk@post.com)
 * Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
 * Copyright (C) 2004, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_ITERATOR_H_

#include "third_party/blink/renderer/core/dom/node_iterator_base.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;

class NodeIterator final : public ScriptWrappable, public NodeIteratorBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  NodeIterator(Node*, unsigned what_to_show, V8NodeFilter*);

  Node* nextNode(ExceptionState&);
  Node* previousNode(ExceptionState&);
  void detach();

  Node* referenceNode() const { return reference_node_.node.Get(); }
  bool pointerBeforeReferenceNode() const {
    return reference_node_.is_pointer_before_node;
  }

  // This function is called before any node is removed from the document tree.
  void NodeWillBeRemoved(Node&);

  void Trace(Visitor*) const override;

 private:
  class NodePointer {
    DISALLOW_NEW();

   public:
    NodePointer();
    NodePointer(Node*, bool);

    void Clear();
    bool MoveToNext(Node* root);
    bool MoveToPrevious(Node* root);

    Member<Node> node;
    bool is_pointer_before_node;

    void Trace(Visitor* visitor) const { visitor->Trace(node); }
  };

  void UpdateForNodeRemoval(Node& node_to_be_removed, NodePointer&) const;

  NodePointer reference_node_;
  NodePointer candidate_node_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_ITERATOR_H_
