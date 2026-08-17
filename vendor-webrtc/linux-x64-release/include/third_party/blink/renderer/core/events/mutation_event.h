/*
 * Copyright (C) 2001 Peter Kelly (pmk@post.com)
 * Copyright (C) 2001 Tobias Anton (anton@stud.fbi.fh-darmstadt.de)
 * Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
 * Copyright (C) 2003, 2004, 2005, 2006, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_MUTATION_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_MUTATION_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/dom/node.h"

namespace blink {

class EventDispatcher;

class MutationEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~MutationEvent() override;

  enum AttrChangeType { kModification = 1, kAddition = 2, kRemoval = 3 };

  static MutationEvent* Create() {
    return MakeGarbageCollected<MutationEvent>();
  }

  static MutationEvent* Create(const AtomicString& type,
                               Bubbles bubbles,
                               Node* related_node = nullptr,
                               const String& prev_value = String(),
                               const String& new_value = String(),
                               const String& attr_name = String(),
                               uint16_t attr_change = 0) {
    return MakeGarbageCollected<MutationEvent>(
        type, bubbles, Cancelable::kNo, related_node, prev_value, new_value,
        attr_name, attr_change);
  }

  MutationEvent();
  MutationEvent(const AtomicString& type,
                Bubbles,
                Cancelable,
                Node* related_node,
                const String& prev_value,
                const String& new_value,
                const String& attr_name,
                uint16_t attr_change);

  void initMutationEvent(const AtomicString& type,
                         bool bubbles,
                         bool cancelable,
                         Node* related_node,
                         const String& prev_value,
                         const String& new_value,
                         const String& attr_name,
                         uint16_t attr_change);

  Node* relatedNode() const { return related_node_.Get(); }
  String prevValue() const { return prev_value_; }
  String newValue() const { return new_value_; }
  String attrName() const { return attr_name_; }
  uint16_t attrChange() const { return attr_change_; }

  const AtomicString& InterfaceName() const override;

  DispatchEventResult DispatchEvent(EventDispatcher&) override;

  void Trace(Visitor*) const override;

 private:
  Member<Node> related_node_;
  String prev_value_;
  String new_value_;
  String attr_name_;
  uint16_t attr_change_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_MUTATION_EVENT_H_
