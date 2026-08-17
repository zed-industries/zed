/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2009 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_FRAGMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_FRAGMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/container_node.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/parser_content_policy.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class DocumentPartRoot;

class CORE_EXPORT DocumentFragment : public ContainerNode {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static DocumentFragment* Create(Document&);

  DocumentFragment(Document*, ConstructionType);

  void ParseHTML(const String&,
                 Element* context_element,
                 ParserContentPolicy = kAllowScriptingContent);
  bool ParseXML(const String&,
                Element* context_element,
                ExceptionState& exception_state,
                ParserContentPolicy = kAllowScriptingContent);

  bool CanContainRangeEndPoint() const final { return true; }
  virtual bool IsTemplateContent() const { return false; }

  // These represent whether this DocumentFragment is one that holds
  // children that have not had Node::InsertedInto called.  We use such
  // DocumentFragment objects internally for temporary storage of
  // parsing results.  This requires that when we move the children
  // *out* of the DocumentFragment we also not call RemovedFrom.
  bool HoldsUnnotifiedChildren() const { return holds_unnotified_children_; }
  void SetHoldsUnnotifiedChildren(bool v) { holds_unnotified_children_ = v; }

  // When HoldsUnnotifiedChildren() is true, a caller that is taking the
  // children can call ForgetChildren to disconnect them without any
  // notifications.
  void ForgetChildren();

  // This will catch anyone doing an unnecessary check.
  bool IsDocumentFragment() const = delete;

  // https://crbug.com/1453291
  // The DOM Parts API:
  // https://github.com/WICG/webcomponents/blob/gh-pages/proposals/DOM-Parts.md.
  DocumentPartRoot& getPartRoot();

  void Trace(Visitor* visitor) const override;

 protected:
  String nodeName() const final;

 private:
  Node* Clone(Document& factory,
              NodeCloningData& data,
              ContainerNode* append_to,
              ExceptionState& append_exception_state) const override;
  bool ChildTypeAllowed(NodeType) const override;

  Member<DocumentPartRoot> document_part_root_;
  bool holds_unnotified_children_ = false;
};

template <>
struct DowncastTraits<DocumentFragment> {
  static bool AllowFrom(const Node& node) { return node.IsDocumentFragment(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_FRAGMENT_H_
