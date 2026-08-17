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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_CONNECTION_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_CONNECTION_EVENT_H_

#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/webmidi/midi_port.h"

namespace blink {

class MIDIConnectionEventInit;

class MIDIConnectionEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static MIDIConnectionEvent* Create(MIDIPort* port) {
    return MakeGarbageCollected<MIDIConnectionEvent>(port);
  }

  static MIDIConnectionEvent* Create(
      const AtomicString& type,
      const MIDIConnectionEventInit* initializer) {
    return MakeGarbageCollected<MIDIConnectionEvent>(type, initializer);
  }
  MIDIConnectionEvent(MIDIPort* port)
      : Event(event_type_names::kStatechange, Bubbles::kNo, Cancelable::kNo),
        port_(port) {}
  MIDIConnectionEvent(const AtomicString&, const MIDIConnectionEventInit*);

  MIDIPort* port() { return port_.Get(); }

  const AtomicString& InterfaceName() const override {
    return event_interface_names::kMIDIConnectionEvent;
  }

  void Trace(Visitor*) const override;

 private:
  Member<MIDIPort> port_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_CONNECTION_EVENT_H_
