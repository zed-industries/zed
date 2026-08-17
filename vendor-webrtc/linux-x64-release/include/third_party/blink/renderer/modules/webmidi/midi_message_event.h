/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_MESSAGE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_MESSAGE_EVENT_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/event_type_names.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/event_modules.h"

namespace blink {

class MIDIMessageEventInit;

class MIDIMessageEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static MIDIMessageEvent* Create(const AtomicString& type,
                                  const MIDIMessageEventInit* initializer) {
    return MakeGarbageCollected<MIDIMessageEvent>(type, initializer);
  }

  MIDIMessageEvent(base::TimeTicks time_stamp, DOMUint8Array* data)
      : Event(event_type_names::kMidimessage,
              Bubbles::kYes,
              Cancelable::kNo,
              time_stamp),
        data_(data) {}
  MIDIMessageEvent(const AtomicString& type,
                   const MIDIMessageEventInit* initializer);

  NotShared<DOMUint8Array> data() { return data_; }

  const AtomicString& InterfaceName() const override {
    return event_interface_names::kMIDIMessageEvent;
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(data_);
    Event::Trace(visitor);
  }

 private:
  NotShared<DOMUint8Array> data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_MESSAGE_EVENT_H_
