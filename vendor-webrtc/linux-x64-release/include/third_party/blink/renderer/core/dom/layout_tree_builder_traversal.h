/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LAYOUT_TREE_BUILDER_TRAVERSAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LAYOUT_TREE_BUILDER_TRAVERSAL_H_

#include <cstdint>
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LayoutObject;

class CORE_EXPORT LayoutTreeBuilderTraversal {
  STATIC_ONLY(LayoutTreeBuilderTraversal);

 public:
  static const int32_t kTraverseAllSiblings = -2;

  static ContainerNode* Parent(const Node&);
  static ContainerNode* LayoutParent(const Node&);
  static Node* FirstChild(const Node&);
  static Node* LastChild(const Node&);
  static Node* NextSibling(const Node&);
  static Node* NextLayoutSibling(const Node& node) {
    int32_t limit = kTraverseAllSiblings;
    return NextLayoutSibling(node, limit);
  }
  static Node* PreviousLayoutSibling(const Node& node) {
    int32_t limit = kTraverseAllSiblings;
    return PreviousLayoutSibling(node, limit);
  }
  static Node* FirstLayoutChild(const Node&);

  static Node* PreviousSibling(const Node&);
  static Node* Previous(const Node&, const Node* stay_within);
  static Node* Next(const Node&, const Node* stay_within);
  static Node* NextSkippingChildren(const Node&, const Node* stay_within);
  static LayoutObject* ParentLayoutObject(const Node&);
  static LayoutObject* NextSiblingLayoutObject(
      const Node&,
      int32_t limit = kTraverseAllSiblings);
  static LayoutObject* PreviousSiblingLayoutObject(
      const Node&,
      int32_t limit = kTraverseAllSiblings);
  static LayoutObject* NextInTopLayer(const Element&);

  static inline Element* ParentElement(const Node& node) {
    return DynamicTo<Element>(Parent(node));
  }
  static inline Element* LayoutParentElement(const Node& node) {
    return DynamicTo<Element>(LayoutParent(node));
  }
  static bool IsLayoutParent(const Node& node);
  // Compares positions of two nodes in preorder tree traversal.
  // Return -1 if the first one goes first, 0 if they are the same
  // and 1 if the second goes first.
  static int ComparePreorderTreePosition(const Node&, const Node&);

 private:
  static Node* NextLayoutSibling(const Node&, int32_t& limit);
  static Node* PreviousLayoutSibling(const Node&, int32_t& limit);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LAYOUT_TREE_BUILDER_TRAVERSAL_H_
