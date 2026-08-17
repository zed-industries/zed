// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_DISPATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_DISPATCHER_H_

#include "base/containers/span.h"
#include "base/memory/raw_ptr.h"
#include "media/midi/midi_service.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class MIDIDispatcher : public GarbageCollected<MIDIDispatcher>,
                       public midi::mojom::blink::MidiSessionClient {
 public:
  class Client : public GarbageCollectedMixin {
   public:
    virtual void DidAddInputPort(const String& id,
                                 const String& manufacturer,
                                 const String& name,
                                 const String& version,
                                 midi::mojom::PortState) = 0;
    virtual void DidAddOutputPort(const String& id,
                                  const String& manufacturer,
                                  const String& name,
                                  const String& version,
                                  midi::mojom::PortState) = 0;
    virtual void DidSetInputPortState(unsigned port_index,
                                      midi::mojom::PortState) = 0;
    virtual void DidSetOutputPortState(unsigned port_index,
                                       midi::mojom::PortState) = 0;

    virtual void DidStartSession(midi::mojom::Result) = 0;
    virtual void DidReceiveMIDIData(unsigned port_index,
                                    base::span<const uint8_t> data,
                                    base::TimeTicks time_stamp) = 0;
  };

  explicit MIDIDispatcher(ExecutionContext* execution_context);
  ~MIDIDispatcher() override;

  void SetClient(Client* client) { client_ = client; }

  void SendMIDIData(uint32_t port,
                    base::span<const uint8_t> data,
                    base::TimeTicks timestamp);

  // midi::mojom::blink::MidiSessionClient implementation.
  // All of the following methods are run on the main thread.
  void AddInputPort(midi::mojom::blink::PortInfoPtr info) override;
  void AddOutputPort(midi::mojom::blink::PortInfoPtr info) override;
  void SetInputPortState(uint32_t port,
                         midi::mojom::blink::PortState state) override;
  void SetOutputPortState(uint32_t port,
                          midi::mojom::blink::PortState state) override;
  void SessionStarted(midi::mojom::blink::Result result) override;
  void AcknowledgeSentData(uint32_t bytes) override;
  void DataReceived(uint32_t port,
                    const Vector<uint8_t>& data,
                    base::TimeTicks timestamp) override;

  void Trace(Visitor* visitor) const;

 private:
  Member<Client> client_;

  bool initialized_ = false;

  // Holds MidiPortInfoList for input ports and output ports.
  Vector<midi::mojom::blink::PortInfo> inputs_;
  Vector<midi::mojom::blink::PortInfo> outputs_;

  // TODO(toyoshim): Consider to have a per-process limit.
  size_t unacknowledged_bytes_sent_ = 0u;

  HeapMojoRemote<midi::mojom::blink::MidiSession> midi_session_;

  HeapMojoReceiver<midi::mojom::blink::MidiSessionClient, MIDIDispatcher>
      receiver_;
  HeapMojoRemote<midi::mojom::blink::MidiSessionProvider>
      midi_session_provider_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_DISPATCHER_H_
