/*
 * Copyright (C) 2010, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_NODE_INPUT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_NODE_INPUT_H_

#include "base/memory/raw_ref.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/audio_summing_junction.h"
#include "third_party/blink/renderer/platform/audio/audio_bus.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

class AudioNodeOutput;

// An AudioNodeInput represents an input to an AudioNode and can be connected
// from one or more AudioNodeOutputs.  In the case of multiple connections, the
// input will act as a unity-gain summing junction, mixing all the outputs.  The
// number of channels of the input's bus is the maximum of the number of
// channels of all its connections.
//
// Use AudioNodeWiring to connect the AudioNodeOutput of another node to this,
// and to disconnect, enable or disable that connection afterward.
class MODULES_EXPORT AudioNodeInput final : public AudioSummingJunction {
  USING_FAST_MALLOC(AudioNodeInput);

 public:
  explicit AudioNodeInput(AudioHandler&);
  ~AudioNodeInput() override;

  // AudioSummingJunction
  void DidUpdate() override;

  // Can be called from any thread.
  AudioHandler& Handler() const { return *handler_; }

  // pull() processes all of the AudioNodes connected to us.
  // In the case of multiple connections it sums the result into an internal
  // summing bus.  In the single connection case, it allows in-place processing
  // where possible using inPlaceBus.  It returns the bus which it rendered
  // into, returning inPlaceBus if in-place processing was performed.
  // Called from context's audio thread.
  scoped_refptr<AudioBus> Pull(AudioBus* in_place_bus,
                               uint32_t frames_to_process);

  // bus() contains the rendered audio after pull() has been called for each
  // time quantum.
  // Called from context's audio thread.
  scoped_refptr<AudioBus> Bus();

  // updateInternalBus() updates m_internalSummingBus appropriately for the
  // number of channels.  This must be called when we own the context's graph
  // lock in the audio thread at the very start or end of the render quantum.
  void UpdateInternalBus();

  // The number of channels of the connection with the largest number of
  // channels.
  unsigned NumberOfChannels() const;

 private:
  // This reference is safe because the AudioHandler owns this AudioNodeInput
  // object.
  const raw_ref<AudioHandler> handler_;

  // m_disabledOutputs contains the AudioNodeOutputs which are disabled (will
  // not be processed) by the audio graph rendering.  But, from JavaScript's
  // perspective, these outputs are still connected to us.
  // Generally, these represent disabled connections from "notes" which have
  // finished playing but are not yet garbage collected.
  // These raw pointers are safe. Owner AudioNodes of these AudioNodeOutputs
  // manage their lifetime, and AudioNode::dispose() disconnects all of
  // connections.
  HashSet<AudioNodeOutput*> disabled_outputs_;

  // Called from context's audio thread.
  scoped_refptr<AudioBus> InternalSummingBus();
  void SumAllConnections(scoped_refptr<AudioBus> summing_bus,
                         uint32_t frames_to_process);

  scoped_refptr<AudioBus> internal_summing_bus_;

  // Used to connect inputs and outputs together.
  friend class AudioNodeWiring;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_NODE_INPUT_H_
