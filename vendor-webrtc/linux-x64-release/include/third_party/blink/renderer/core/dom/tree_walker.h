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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TREE_WALKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TREE_WALKER_H_

#include "third_party/blink/renderer/core/dom/node_iterator_base.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class ExceptionState;

class TreeWalker final : public ScriptWrappable, public NodeIteratorBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  TreeWalker(Node*, unsigned what_to_show, V8NodeFilter*);

  Node* currentNode() const { return current_.Get(); }
  void setCurrentNode(Node*);

  Node* parentNode(ExceptionState&);
  Node* firstChild(ExceptionState&);
  Node* lastChild(ExceptionState&);
  Node* previousSibling(ExceptionState&);
  Node* nextSibling(ExceptionState&);
  Node* previousNode(ExceptionState&);
  Node* nextNode(ExceptionState&);

  void Trace(Visitor*) const override;

 private:
  Node* SetCurrent(Node*);
  template <typename Strategy>
  Node* TraverseSiblings(ExceptionState&);

  Member<Node> current_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TREE_WALKER_H_
