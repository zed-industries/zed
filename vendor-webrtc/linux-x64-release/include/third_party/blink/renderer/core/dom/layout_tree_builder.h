/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc. All
 * rights reserved.
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LAYOUT_TREE_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LAYOUT_TREE_BUILDER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/flat_tree_traversal.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/text.h"
#include "third_party/blink/renderer/core/layout/layout_inline.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "third_party/blink/renderer/core/layout/layout_text.h"

namespace blink {

class ComputedStyle;

// The LayoutTreeBuilder class uses takes a DOM node and its computed CSS styles
// as input to create a LayoutObject which is then used as input to layout.
//
// The layout tree building is done traversing the flattened DOM tree from
// StyleEngine::RebuildLayoutTree() which calls AttachLayoutTree for the nodes
// which need to have their layout boxes re-attached. AttachLayoutTree then
// calls CreateLayoutObject on LayoutTreeBuilderForElement and
// LayoutTreeBuilderForText for elements and text nodes respectively.
template <typename NodeType>
class LayoutTreeBuilder {
  STACK_ALLOCATED();

 protected:
  LayoutTreeBuilder(NodeType& node,
                    Node::AttachContext& context,
                    const ComputedStyle* style)
      : node_(&node), context_(context), style_(style) {
    DCHECK(!node.GetLayoutObject());
    DCHECK(node.GetDocument().InStyleRecalc() ||
           node.GetDocument().GetStyleEngine().InScrollMarkersAttachment());
    DCHECK(node.InActiveDocument());
    DCHECK(context.parent);
  }

  LayoutObject* NextLayoutObject() const {
    if (!context_.next_sibling_valid) {
      context_.next_sibling =
          LayoutTreeBuilderTraversal::NextSiblingLayoutObject(*node_);
      context_.next_sibling_valid = true;
    }
    LayoutObject* next = context_.next_sibling;
    // If a text node is wrapped in an anonymous inline for display:contents
    // (see CreateInlineWrapperForDisplayContents()), use the wrapper as the
    // next layout object. Otherwise we would need to add code to various
    // AddChild() implementations to walk up the tree to find the correct
    // layout tree parent/siblings.
    if (!next || !next->IsText())
      return next;
    auto* const parent = next->Parent();
    if (!IsAnonymousInline(parent))
      return next;
    // Should return a normal result for display:ruby though it can be
    // an anonymous inline.
    if (parent->IsInlineRuby()) [[unlikely]] {
      return next;
    }
    if (!parent->IsLayoutTextCombine()) [[unlikely]] {
      return parent;
    }
    auto* const text_combine_parent = parent->Parent();
    if (IsAnonymousInline(text_combine_parent))
      return text_combine_parent;
    return parent;
  }

  static bool IsAnonymousInline(const LayoutObject* layout_object) {
    return layout_object && layout_object->IsAnonymous() &&
           layout_object->IsInline();
  }

  NodeType* node_;
  Node::AttachContext& context_;
  const ComputedStyle* style_;
};

class LayoutTreeBuilderForElement : public LayoutTreeBuilder<Element> {
 public:
  LayoutTreeBuilderForElement(Element&,
                              Node::AttachContext&,
                              const ComputedStyle*);

  void CreateLayoutObject();

 private:
  LayoutObject* ParentLayoutObject() const;
  LayoutObject* NextLayoutObject() const;
};

class LayoutTreeBuilderForText : public LayoutTreeBuilder<Text> {
 public:
  LayoutTreeBuilderForText(Text& text,
                           Node::AttachContext& context,
                           const ComputedStyle* style_from_parent)
      : LayoutTreeBuilder(text, context, style_from_parent) {}

  void CreateLayoutObject();

 private:
  const ComputedStyle* CreateInlineWrapperStyleForDisplayContentsIfNeeded()
      const;
  LayoutObject* CreateInlineWrapperForDisplayContentsIfNeeded(
      const ComputedStyle* wrapper_style) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LAYOUT_TREE_BUILDER_H_
