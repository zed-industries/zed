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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_PORT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_PORT_H_

#include "media/midi/midi_service.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_midi_port_connection_state.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_midi_port_type.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class MIDIAccess;
class V8MIDIPortDeviceState;
using MIDIPortConnectionState = V8MIDIPortConnectionState::Enum;
using MIDIPortType = V8MIDIPortType::Enum;

class MIDIPort : public EventTarget,
                 public ActiveScriptWrappable<MIDIPort>,
                 public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~MIDIPort() override = default;

  V8MIDIPortConnectionState connection() const;
  String id() const { return id_; }
  String manufacturer() const { return manufacturer_; }
  String name() const { return name_; }
  V8MIDIPortDeviceState state() const;
  V8MIDIPortType type() const;
  String version() const { return version_; }

  ScriptPromise<MIDIPort> open(ScriptState*);
  ScriptPromise<MIDIPort> close(ScriptState*);

  midi::mojom::PortState GetState() const { return state_; }
  void SetState(midi::mojom::PortState);
  MIDIPortConnectionState GetConnection() const { return connection_; }

  void Trace(Visitor*) const override;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(statechange, kStatechange)

  // EventTarget
  const AtomicString& InterfaceName() const override {
    return event_target_names::kMIDIPort;
  }
  ExecutionContext* GetExecutionContext() const final;

  // ScriptWrappable
  bool HasPendingActivity() const final;

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

 protected:
  MIDIPort(MIDIAccess*,
           const String& id,
           const String& manufacturer,
           const String& name,
           MIDIPortType,
           const String& version,
           midi::mojom::PortState);

  void open();
  bool IsOpening() { return running_open_count_; }
  MIDIAccess* midiAccess() const { return access_.Get(); }

 private:
  void OpenAsynchronously(ScriptPromiseResolver<MIDIPort>*);
  virtual void DidOpen(bool opened) {}
  void CloseAsynchronously(ScriptPromiseResolver<MIDIPort>*);

  ScriptPromise<MIDIPort> Accept(ScriptState*);

  void SetStates(midi::mojom::PortState, MIDIPortConnectionState);

  String id_;
  String manufacturer_;
  String name_;
  MIDIPortType type_;
  String version_;
  Member<MIDIAccess> access_;
  midi::mojom::PortState state_;
  MIDIPortConnectionState connection_;
  unsigned running_open_count_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_PORT_H_
