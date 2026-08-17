/*
 * Copyright (C) 2010 Google, Inc. All Rights Reserved.
 * Copyright (C) 2011 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL GOOGLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_ELEMENT_STACK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_ELEMENT_STACK_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/html/parser/html_stack_item.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ContainerNode;
class Element;

enum class DOMPartsAllowed {
  kNever,
  kInsideParseParts,
  kAlways,
};

// NOTE: The HTML5 spec uses a backwards (grows downward) stack.  We're using
// more standard (grows upwards) stack terminology here.
class HTMLElementStack {
  DISALLOW_NEW();

 public:
  HTMLElementStack();
  HTMLElementStack(const HTMLElementStack&) = delete;
  HTMLElementStack& operator=(const HTMLElementStack&) = delete;

  unsigned StackDepth() const { return stack_depth_; }

  // Inlining this function is a (small) performance win on the parsing
  // benchmark.
  Element* Top() const {
    DCHECK(top_->GetElement());
    return top_->GetElement();
  }

  ContainerNode* TopNode() const {
    DCHECK(top_);
    DCHECK(top_->GetNode());
    return top_->GetNode();
  }

  HTMLStackItem* TopStackItem() const {
    DCHECK(top_);
    return top_.Get();
  }

  HTMLStackItem* OneBelowTop() const;
  HTMLStackItem* Find(Element*) const;
  HTMLStackItem* FurthestBlockForFormattingElement(Element*) const;
  HTMLStackItem* Topmost(html_names::HTMLTag tag) const;

  void InsertAbove(HTMLStackItem*, HTMLStackItem*);

  void Push(HTMLStackItem*);
  void PushRootNode(HTMLStackItem*);
  void PushHTMLHtmlElement(HTMLStackItem*);
  void PushHTMLHeadElement(HTMLStackItem*);
  void PushHTMLBodyElement(HTMLStackItem*);
  void Replace(HTMLStackItem* old_item, HTMLStackItem* new_item);

  void Pop();
  void PopUntil(html_names::HTMLTag tag);
  void PopUntil(Element*);
  void PopUntilPopped(Element*);
  void PopUntilPopped(html_names::HTMLTag tag);

  void PopUntilNumberedHeaderElementPopped();

  // "clear the stack back to a table context" in the spec.
  void PopUntilTableScopeMarker();

  // "clear the stack back to a table body context" in the spec.
  void PopUntilTableBodyScopeMarker();

  // "clear the stack back to a table row context" in the spec.
  void PopUntilTableRowScopeMarker();

  void PopUntilForeignContentScopeMarker();
  void PopHTMLHeadElement();
  void PopHTMLBodyElement();
  void PopAll();

  static bool IsMathMLTextIntegrationPoint(HTMLStackItem*);
  static bool IsHTMLIntegrationPoint(HTMLStackItem*);

  void Remove(Element*);
  void RemoveHTMLHeadElement(Element*);

  bool Contains(Element*) const;

  bool InScope(Element*) const;
  bool InScope(html_names::HTMLTag tag) const;
  bool InListItemScope(html_names::HTMLTag tag) const;
  bool InTableScope(html_names::HTMLTag tag) const;
  bool InButtonScope(html_names::HTMLTag tag) const;
  bool InSelectScope(html_names::HTMLTag tag) const;
  bool InParsePartsScope() const {
    DCHECK(RuntimeEnabledFeatures::DOMPartsAPIEnabled() || !parse_parts_count_);
    return parse_parts_count_;
  }
  void SetDOMPartsAllowedState(DOMPartsAllowed state) {
    DCHECK(RuntimeEnabledFeatures::DOMPartsAPIEnabled());
    dom_parts_allowed_state_ = state;
    if (state == DOMPartsAllowed::kAlways) {
      parse_parts_count_ = 1;
    }
  }

  bool HasNumberedHeaderElementInScope() const;

  bool HasOnlyOneElement() const;
  bool SecondElementIsHTMLBodyElement() const;
  bool HasTemplateInHTMLScope() const;
  Element* HtmlElement() const;
  Element* HeadElement() const;
  Element* BodyElement() const;

  ContainerNode* RootNode() const;

  void Trace(Visitor*) const;

#ifndef NDEBUG
  void Show();
#endif

 private:
  void PushCommon(HTMLStackItem*);
  void PushRootNodeCommon(HTMLStackItem*);
  void PopCommon();
  void RemoveNonTopCommon(Element*);

  unsigned parse_parts_count_{0};
  DOMPartsAllowed dom_parts_allowed_state_{DOMPartsAllowed::kInsideParseParts};

  Member<HTMLStackItem> top_;

  // We remember the root node, <head> and <body> as they are pushed. The root
  // node is never popped.
  // FIXME: We don't currently require type-specific information about these
  // elements so we haven't yet bothered to plumb the types all the way down
  // through createElement, etc.
  Member<ContainerNode> root_node_;
  Member<Element> head_element_;
  Member<Element> body_element_;
  unsigned stack_depth_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_ELEMENT_STACK_H_
