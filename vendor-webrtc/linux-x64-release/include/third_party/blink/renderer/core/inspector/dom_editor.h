/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DOM_EDITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DOM_EDITOR_H_

#include "third_party/blink/renderer/core/inspector/protocol/forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ContainerNode;
class Element;
class ExceptionState;
class InspectorHistory;
class Node;

class DOMEditor final : public GarbageCollected<DOMEditor> {
 public:
  explicit DOMEditor(InspectorHistory*);
  DOMEditor(const DOMEditor&) = delete;
  DOMEditor& operator=(const DOMEditor&) = delete;

  void Trace(Visitor*) const;

  bool InsertBefore(ContainerNode* parent_node,
                    Node*,
                    Node* anchor_node,
                    ExceptionState&);
  bool RemoveChild(ContainerNode* parent_node, Node*, ExceptionState&);
  bool SetAttribute(Element*,
                    const String& name,
                    const String& value,
                    ExceptionState&);
  bool RemoveAttribute(Element*, const String& name, ExceptionState&);
  bool SetOuterHTML(Node*,
                    const String& html,
                    Node** new_node,
                    ExceptionState&);
  bool ReplaceChild(ContainerNode* parent_node,
                    Node* new_node,
                    Node* old_node,
                    ExceptionState&);
  bool SetNodeValue(Node* parent_node, const String& value, ExceptionState&);

  protocol::Response InsertBefore(ContainerNode* parent_node,
                                  Node*,
                                  Node* anchor_node);
  protocol::Response RemoveChild(ContainerNode* parent_node, Node*);
  protocol::Response SetAttribute(Element*,
                                  const String& name,
                                  const String& value);
  protocol::Response RemoveAttribute(Element*, const String& name);
  protocol::Response SetOuterHTML(Node*, const String& html, Node** new_node);
  protocol::Response SetNodeValue(Node* parent_node, const String& value);

 private:
  class DOMAction;
  class RemoveChildAction;
  class InsertBeforeAction;
  class RemoveAttributeAction;
  class SetAttributeAction;
  class SetOuterHTMLAction;
  class ReplaceChildNodeAction;
  class SetNodeValueAction;

  Member<InspectorHistory> history_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DOM_EDITOR_H_
