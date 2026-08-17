// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_COMMAND_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_COMMAND_EVENT_H_

#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/event_interface_names.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CommandEventInit;

class CommandEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static CommandEvent* Create(const AtomicString& type,
                              const CommandEventInit* initializer) {
    return MakeGarbageCollected<CommandEvent>(type, initializer);
  }

  static CommandEvent* Create(const AtomicString& type,
                              const AtomicString& command,
                              Element* source) {
    return MakeGarbageCollected<CommandEvent>(type, command, source);
  }

  CommandEvent(const AtomicString& type, const CommandEventInit* initializer);
  CommandEvent(const AtomicString& type,
               const String& command,
               Element* source);

  const AtomicString& InterfaceName() const override {
    return event_interface_names::kCommandEvent;
  }

  void Trace(Visitor*) const override;

  const String& command() const { return command_; }

  Element* source() const;
  void SetSource(Element* source) { source_ = source; }

 private:
  Member<Element> source_;
  String command_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_COMMAND_EVENT_H_
