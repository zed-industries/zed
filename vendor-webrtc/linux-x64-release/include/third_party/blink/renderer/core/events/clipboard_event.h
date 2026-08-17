/*
 * Copyright (C) 2001 Peter Kelly (pmk@post.com)
 * Copyright (C) 2001 Tobias Anton (anton@stud.fbi.fh-darmstadt.de)
 * Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights
 * reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_CLIPBOARD_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_CLIPBOARD_EVENT_H_

#include "third_party/blink/renderer/core/clipboard/data_transfer.h"
#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class ClipboardEventInit;

class ClipboardEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ClipboardEvent(const AtomicString& type, DataTransfer* clipboard_data);
  ClipboardEvent(const AtomicString& type, const ClipboardEventInit*);
  ~ClipboardEvent() override;

  static ClipboardEvent* Create(const AtomicString& type,
                                DataTransfer* data_transfer) {
    return MakeGarbageCollected<ClipboardEvent>(type, data_transfer);
  }

  static ClipboardEvent* Create(const AtomicString& type,
                                const ClipboardEventInit* initializer) {
    return MakeGarbageCollected<ClipboardEvent>(type, initializer);
  }

  DataTransfer* clipboardData() const { return clipboard_data_.Get(); }

  void Trace(Visitor*) const override;

 private:
  const AtomicString& InterfaceName() const override;
  bool IsClipboardEvent() const override;

  Member<DataTransfer> clipboard_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_CLIPBOARD_EVENT_H_
