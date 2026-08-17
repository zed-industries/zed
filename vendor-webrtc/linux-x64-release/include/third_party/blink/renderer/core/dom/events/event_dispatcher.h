/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2008 Nokia Corporation and/or its subsidiary(-ies)
 * Copyright (C) 2009 Torch Mobile Inc. All rights reserved.
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_DISPATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_DISPATCHER_H_

#include "base/dcheck_is_on.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/dom/events/event_dispatch_result.h"
#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Event;
class EventDispatchHandlingState;
class LocalFrameView;
class Node;
class HTMLInputElement;

class EventDispatchHandlingState
    : public GarbageCollected<EventDispatchHandlingState> {
 public:
  virtual void Trace(Visitor* visitor) const {}
};

enum EventDispatchContinuation { kContinueDispatching, kDoneDispatching };

class EventDispatcher {
  STACK_ALLOCATED();

 public:
  static DispatchEventResult DispatchEvent(Node&, Event&);
  static void DispatchScopedEvent(Node&, Event&);

  static void DispatchSimulatedClick(Node&,
                                     const Event* underlying_event,
                                     SimulatedClickCreationScope);
  static void DispatchSimulatedEnterEvent(HTMLInputElement& input_element);

  DispatchEventResult Dispatch();
  Node& GetNode() const { return *node_; }
  Event& GetEvent() const { return *event_; }

 private:
  EventDispatcher(Node&, Event&);

  EventDispatchContinuation DispatchEventPreProcess(
      Node* activation_target,
      EventDispatchHandlingState*&);
  EventDispatchContinuation DispatchEventAtCapturing();
  void DispatchEventAtBubbling();
  void DispatchEventPostProcess(Node* activation_target,
                                EventDispatchHandlingState*);

  Node* node_;
  Event* event_;
  LocalFrameView* view_;
#if DCHECK_IS_ON()
  bool event_dispatched_ = false;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_DISPATCHER_H_
