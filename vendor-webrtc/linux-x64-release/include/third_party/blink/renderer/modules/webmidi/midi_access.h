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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_ACCESS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_ACCESS_H_

#include "base/notreached.h"
#include "media/midi/midi_service.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/webmidi/midi_access_initializer.h"
#include "third_party/blink/renderer/modules/webmidi/midi_dispatcher.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExecutionContext;
class MIDIInput;
class MIDIInputMap;
class MIDIOutput;
class MIDIOutputMap;

class MIDIAccess final : public EventTarget,
                         public ActiveScriptWrappable<MIDIAccess>,
                         public ExecutionContextLifecycleObserver,
                         public MIDIDispatcher::Client {
  DEFINE_WRAPPERTYPEINFO();

 public:
  MIDIAccess(MIDIDispatcher*,
             bool sysex_enabled,
             const Vector<MIDIAccessInitializer::PortDescriptor>&,
             ExecutionContext*);
  ~MIDIAccess() override;

  MIDIInputMap* inputs() const;
  MIDIOutputMap* outputs() const;

  EventListener* onstatechange();
  void setOnstatechange(EventListener*);

  bool sysexEnabled() const { return sysex_enabled_; }

  // EventTarget
  const AtomicString& InterfaceName() const override {
    return event_target_names::kMIDIAccess;
  }
  ExecutionContext* GetExecutionContext() const override {
    return ExecutionContextLifecycleObserver::GetExecutionContext();
  }

  // ScriptWrappable
  bool HasPendingActivity() const final;

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override {}

  // MIDIDispatcher::Client
  void DidAddInputPort(const String& id,
                       const String& manufacturer,
                       const String& name,
                       const String& version,
                       midi::mojom::PortState) override;
  void DidAddOutputPort(const String& id,
                        const String& manufacturer,
                        const String& name,
                        const String& version,
                        midi::mojom::PortState) override;
  void DidSetInputPortState(unsigned port_index,
                            midi::mojom::PortState) override;
  void DidSetOutputPortState(unsigned port_index,
                             midi::mojom::PortState) override;
  void DidStartSession(midi::mojom::Result) override {
    // This method is for MIDIAccess initialization: MIDIAccessInitializer
    // has the implementation.
    NOTREACHED();
  }
  void DidReceiveMIDIData(unsigned port_index,
                          base::span<const uint8_t> data,
                          base::TimeTicks time_stamp) override;

  // |timeStampInMilliseconds| is in the same time coordinate system as
  // performance.now().
  void SendMIDIData(unsigned port_index,
                    base::span<const uint8_t> data,
                    base::TimeTicks time_stamp);

  // Eager finalization needed to promptly release m_accessor. Otherwise
  // its client back reference could end up being unsafely used during
  // the lazy sweeping phase.
  void Trace(Visitor*) const override;

 private:
  Member<MIDIDispatcher> dispatcher_;
  bool sysex_enabled_;
  bool has_pending_activity_;
  HeapVector<Member<MIDIInput>> inputs_;
  HeapVector<Member<MIDIOutput>> outputs_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_ACCESS_H_
