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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_NODE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webaudio/audio_handler.h"
#include "third_party/blink/renderer/modules/webaudio/inspector_helper_mixin.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"

namespace blink {

class BaseAudioContext;
class AudioHandler;
class AudioNodeOptions;
class AudioParam;
class DeferredTaskHandler;
class ExceptionState;
class V8ChannelCountMode;
class V8ChannelInterpretation;

// An AudioNode is the basic building block for handling audio within an
// BaseAudioContext.  It may be an audio source, an intermediate processing
// module, or an audio destination.  Each AudioNode can have inputs and/or
// outputs. An AudioSourceNode has no inputs and a single output.
// An AudioDestinationNode has one input and no outputs and represents the final
// destination to the audio hardware.  Most processing nodes such as filters
// will have one input and one output, although multiple inputs and outputs are
// possible.

// Each of AudioNode objects owns its dedicated AudioHandler object. AudioNode
// is responsible to provide IDL-accessible interface and its lifetime is
// managed by Oilpan GC. AudioHandler is responsible for anything else. We must
// not touch AudioNode objects in an audio rendering thread.

// AudioHandler is created and owned by an AudioNode almost all the time. When
// the AudioNode is about to die, the ownership of its AudioHandler is
// transferred to DeferredTaskHandler, and it does deref the AudioHandler on the
// main thread.
//
// Be careful to avoid reference cycles. If an AudioHandler has a reference
// cycle including the owner AudioNode, objects in the cycle are never
// collected.
class MODULES_EXPORT AudioNode : public EventTarget,
                                 public InspectorHelperMixin {
  DEFINE_WRAPPERTYPEINFO();
  USING_PRE_FINALIZER(AudioNode, Dispose);

 public:
  ~AudioNode() override;

  void Trace(Visitor*) const override;
  AudioHandler& Handler() const;

  void HandleChannelOptions(const AudioNodeOptions*, ExceptionState&);
  String GetNodeName() const;

  AudioNode* connect(AudioNode*,
                     unsigned output_index,
                     unsigned input_index,
                     ExceptionState&);
  void connect(AudioParam*, unsigned output_index, ExceptionState&);
  void disconnect();
  void disconnect(unsigned output_index, ExceptionState&);
  void disconnect(AudioNode*, ExceptionState&);
  void disconnect(AudioNode*, unsigned output_index, ExceptionState&);
  void disconnect(AudioNode*,
                  unsigned output_index,
                  unsigned input_index,
                  ExceptionState&);
  void disconnect(AudioParam*, ExceptionState&);
  void disconnect(AudioParam*, unsigned output_index, ExceptionState&);
  BaseAudioContext* context() const;
  unsigned numberOfInputs() const;
  unsigned numberOfOutputs() const;
  unsigned channelCount() const;
  void setChannelCount(unsigned, ExceptionState&);
  V8ChannelCountMode channelCountMode() const;
  void setChannelCountMode(const V8ChannelCountMode&, ExceptionState&);
  V8ChannelInterpretation channelInterpretation() const;
  void setChannelInterpretation(const V8ChannelInterpretation&,
                                ExceptionState&);

  // EventTarget
  const AtomicString& InterfaceName() const final;
  ExecutionContext* GetExecutionContext() const final;

  // Called inside AudioHandler constructors.
  void DidAddOutput(unsigned number_of_outputs);

 protected:
  explicit AudioNode(BaseAudioContext&);
  // This should be called in a constructor.
  void SetHandler(scoped_refptr<AudioHandler>);

  // During construction time the handler may not be set properly. Since the
  // garbage collector can call into HasPendingActivity() such calls need to be
  // able to see whether a handle has been set.
  bool ContainsHandler() const;

 private:
  void WarnIfContextClosed() const;
  void Dispose();
  void DisconnectAllFromOutput(unsigned output_index);
  // Returns true if the specified AudioNodeInput was connected.
  bool DisconnectFromOutputIfConnected(unsigned output_index,
                                       AudioNode& destination,
                                       unsigned input_index_of_destination);
  // Returns true if the specified AudioParam was connected.
  bool DisconnectFromOutputIfConnected(unsigned output_index, AudioParam&);

  // Any derived node may implement this method to handle the destination
  // connection.
  virtual void ConnectToDestinationReady() {}

  // https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/media/capture/README.md#logs
  void SendLogMessage(const char* const function_name, const String& message);

  Member<BaseAudioContext> context_;
  scoped_refptr<DeferredTaskHandler> deferred_task_handler_;
  scoped_refptr<AudioHandler> handler_;

  // Represents audio node graph with Oilpan references. N-th HeapHashSet
  // represents a set of AudioNode objects connected to this AudioNode's N-th
  // output.
  HeapVector<Member<GCedHeapHashSet<Member<AudioNode>>>> connected_nodes_;
  // Represents audio node graph with Oilpan references. N-th HeapHashSet
  // represents a set of AudioParam objects connected to this AudioNode's N-th
  // output.
  HeapVector<Member<GCedHeapHashSet<Member<AudioParam>>>> connected_params_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_NODE_H_
