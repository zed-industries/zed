/*
 * Copyright (C) 2010, 2011 Nokia Corporation and/or its subsidiary(-ies)
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_DETAILS_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_DETAILS_ELEMENT_H_

#include "third_party/blink/renderer/core/events/toggle_event.h"
#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"

namespace blink {

class HTMLDetailsElement final : public HTMLElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  void ToggleOpen();

  explicit HTMLDetailsElement(Document&);
  ~HTMLDetailsElement() override;

  Element* FindMainSummary() const;

  void ManuallyAssignSlots() override;

  void Trace(Visitor*) const override;

  // Walks up the ancestor chain and expands all <details> elements found along
  // the way by setting the open attribute. If any were expanded, returns true.
  // This method may run script because of the mutation events fired when
  // setting the open attribute.
  static bool ExpandDetailsAncestors(const Node&);

  bool IsValidBuiltinCommand(HTMLElement& invoker,
                             CommandEventType command) override;
  bool HandleCommandInternal(HTMLElement& invoker,
                             CommandEventType command) override;

  // The name attribute for grouping of related details; empty string
  // means no grouping.
  const AtomicString& GetName() const {
    return FastGetAttribute(html_names::kNameAttr);
  }

 private:
  void DispatchPendingEvent(const AttributeModificationReason);

  // Return all the <details> elements in the group created by the name
  // attribute, excluding |this|, in tree order.  If there is no such group
  // (e.g., because there is no name attribute), returns an empty list.
  HeapVector<Member<HTMLDetailsElement>> OtherElementsInNameGroup();
  void MaybeCloseForExclusivity();

  void ParseAttribute(const AttributeModificationParams&) override;
  void AttributeChanged(const AttributeModificationParams&) override;
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void DidAddUserAgentShadowRoot(ShadowRoot&) override;
  bool IsInteractiveContent() const override;

  bool is_open_ = false;
  TaskHandle pending_event_task_;
  Member<ToggleEvent> pending_toggle_event_;
  Member<HTMLSlotElement> summary_slot_;
  Member<HTMLSlotElement> content_slot_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_DETAILS_ELEMENT_H_
