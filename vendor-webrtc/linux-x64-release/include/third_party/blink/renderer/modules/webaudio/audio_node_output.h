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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_NODE_OUTPUT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_NODE_OUTPUT_H_

#include "base/memory/raw_ref.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/platform/audio/audio_bus.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

class AudioNodeInput;

// AudioNodeOutput represents a single output for an AudioNode.
// It may be connected to one or more AudioNodeInputs.
class MODULES_EXPORT AudioNodeOutput final {
  USING_FAST_MALLOC(AudioNodeOutput);

 public:
  // It's OK to pass 0 for numberOfChannels in which case
  // setNumberOfChannels() must be called later on.
  AudioNodeOutput(AudioHandler*, unsigned number_of_channels);

  void Dispose();

  // Causes our AudioNode to process if it hasn't already for this render
  // quantum.  It returns the bus containing the processed audio for this
  // output, returning inPlaceBus if in-place processing was possible.  Called
  // from context's audio thread.
  AudioBus* Pull(AudioBus* in_place_bus, uint32_t frames_to_process);

  // bus() will contain the rendered audio after pull() is called for each
  // rendering time quantum.
  // Called from context's audio thread.
  AudioBus* Bus() const;

  // renderingFanOutCount() is the number of AudioNodeInputs that we're
  // connected to during rendering.  Unlike fanOutCount() it will not change
  // during the course of a render quantum.
  unsigned RenderingFanOutCount() const;

  // Returns the number of AudioParams that this output is connected to
  // during rendering. Unlike `ParamFanOutCount()` this will not change
  // during a render quantum. MUST be called from the audio thread.
  unsigned RenderingParamFanOutCount() const;

  // Return true if either `RenderingFanOutCount()` or
  // `RenderingParamFanOutCount()` is greater than zero.
  bool IsConnectedDuringRendering() const;

  // Must be called with the context's graph lock.
  void DisconnectAll();

  // Disconnect a specific input or AudioParam.
  void DisconnectAudioParam(AudioParamHandler&);

  void SetNumberOfChannels(unsigned);
  unsigned NumberOfChannels() const { return number_of_channels_; }
  bool IsChannelCountKnown() const { return NumberOfChannels() > 0; }

  bool IsConnected() { return FanOutCount() > 0 || ParamFanOutCount() > 0; }

  // Disable/Enable happens when there are still JavaScript references to a
  // node, but it has otherwise "finished" its work.  For example, when a note
  // has finished playing.  It is kept around, because it may be played again at
  // a later time.  They must be called with the context's graph lock.
  void Disable();
  void Enable();

  // updateRenderingState() is called in the audio thread at the start or end of
  // the render quantum to handle any recent changes to the graph state.
  // It must be called with the context's graph lock.
  void UpdateRenderingState();

 private:
  // Can be called from any thread.
  AudioHandler& Handler() const { return *handler_; }
  DeferredTaskHandler& GetDeferredTaskHandler() const {
    return handler_->GetDeferredTaskHandler();
  }

  // This reference is safe because the AudioHandler owns this AudioNodeOutput
  // object.
  const raw_ref<AudioHandler> handler_;

  // fanOutCount() is the number of AudioNodeInputs that we're connected to.
  // This method should not be called in audio thread rendering code, instead
  // renderingFanOutCount() should be used.
  // It must be called with the context's graph lock.
  unsigned FanOutCount();

  // Similar to `FanOutCount()`, `ParamFanOutCount()` is the number of
  // AudioParams that this output is connected to.  This method MUST be
  // called from the main thread with the context graph lock.
  // For audio thread, use `RenderingParamFanOutCount()` instead.
  unsigned ParamFanOutCount();

  // Must be called with the context's graph lock.
  void DisconnectAllInputs();
  void DisconnectAllParams();

  // updateInternalBus() updates m_internalBus appropriately for the number of
  // channels.  It is called in the constructor or in the audio thread with the
  // context's graph lock.
  void UpdateInternalBus();

  // Announce to any nodes we're connected to that we changed our channel count
  // for its input.
  // It must be called in the audio thread with the context's graph lock.
  void PropagateChannelCount();

  // updateNumberOfChannels() is called in the audio thread at the start or end
  // of the render quantum to pick up channel changes.
  // It must be called with the context's graph lock.
  void UpdateNumberOfChannels();

  // m_numberOfChannels will only be changed in the audio thread.
  // The main thread sets m_desiredNumberOfChannels which will later get picked
  // up in the audio thread in updateNumberOfChannels().
  unsigned number_of_channels_;
  unsigned desired_number_of_channels_;

  // m_internalBus and m_inPlaceBus must only be changed in the audio thread
  // with the context's graph lock (or constructor).
  scoped_refptr<AudioBus> internal_bus_;
  scoped_refptr<AudioBus> in_place_bus_;
  // If m_isInPlace is true, use m_inPlaceBus as the valid AudioBus; If false,
  // use the default m_internalBus.
  bool is_in_place_ = false;

  // This HashSet holds connection references. We must call
  // AudioNode::makeConnection when we add an AudioNodeInput to this, and must
  // call AudioNode::breakConnection() when we remove an AudioNodeInput from
  // this.
  HashSet<AudioNodeInput*> inputs_;
  bool is_enabled_ = true;

  bool did_call_dispose_ = false;

  // For the purposes of rendering, keeps track of the number of inputs and
  // AudioParams we're connected to.  These value should only be changed at the
  // very start or end of the rendering quantum.
  unsigned rendering_fan_out_count_ = 0;
  unsigned rendering_param_fan_out_count_ = 0;

  // This collection of raw pointers is safe because they are retained by
  // AudioParam objects retained by m_connectedParams of the owner AudioNode.
  HashSet<AudioParamHandler*> params_;

  friend class AudioNodeWiring;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_NODE_OUTPUT_H_
