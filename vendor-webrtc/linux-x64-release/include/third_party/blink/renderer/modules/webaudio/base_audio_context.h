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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_BASE_AUDIO_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_BASE_AUDIO_CONTEXT_H_

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_audio_context_state.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_decode_error_callback.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_decode_success_callback.h"
#include "third_party/blink/renderer/core/dom/events/event_listener.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_state_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webaudio/async_audio_decoder.h"
#include "third_party/blink/renderer/modules/webaudio/audio_destination_node.h"
#include "third_party/blink/renderer/modules/webaudio/deferred_task_handler.h"
#include "third_party/blink/renderer/modules/webaudio/inspector_helper_mixin.h"
#include "third_party/blink/renderer/platform/audio/audio_callback_metric_reporter.h"
#include "third_party/blink/renderer/platform/audio/audio_io_callback.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/wtf/threading.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class AnalyserNode;
class AudioBuffer;
class AudioBufferSourceNode;
class AudioListener;
class AudioWorklet;
class BiquadFilterNode;
class ChannelMergerNode;
class ChannelSplitterNode;
class ConstantSourceNode;
class ConvolverNode;
class DelayNode;
class DynamicsCompressorNode;
class ExceptionState;
class LocalDOMWindow;
class GainNode;
class IIRFilterNode;
class OscillatorNode;
class PannerNode;
class PeriodicWave;
class PeriodicWaveConstraints;
class ScriptProcessorNode;
class ScriptState;
class SecurityOrigin;
class StereoPannerNode;
class WaveShaperNode;
class WorkerThread;

// BaseAudioContext is the cornerstone of the web audio API and all AudioNodes
// are created from it.  For thread safety between the audio thread and the main
// thread, it has a rendering graph locking mechanism.

class MODULES_EXPORT BaseAudioContext
    : public EventTarget,
      public ActiveScriptWrappable<BaseAudioContext>,
      public ExecutionContextLifecycleStateObserver,
      public InspectorHelperMixin {
  DEFINE_WRAPPERTYPEINFO();
  USING_PRE_FINALIZER(BaseAudioContext, Dispose);

 public:
  ~BaseAudioContext() override;

  void Trace(Visitor*) const override;

  // EventTarget
  const AtomicString& InterfaceName() const final;
  ExecutionContext* GetExecutionContext() const final;

  // ActiveScriptWrappable
  bool HasPendingActivity() const override;

  // ExecutionContextLifecycleStateObserver
  void ContextLifecycleStateChanged(mojom::blink::FrameLifecycleState) override;
  void ContextDestroyed() override;

  // InspectorHelperMixin
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

  // https://webaudio.github.io/web-audio-api/#BaseAudioContext
  // Cannot be called from the audio thread.
  AudioDestinationNode* destination() const;
  float sampleRate() const { return destination_handler_->SampleRate(); }
  double currentTime() const { return destination_handler_->CurrentTime(); }
  AudioListener* listener() { return listener_.Get(); }
  V8AudioContextState state() const;
  AudioWorklet* audioWorklet() const;
  DEFINE_ATTRIBUTE_EVENT_LISTENER(statechange, kStatechange)
  AnalyserNode* createAnalyser(ExceptionState&);
  BiquadFilterNode* createBiquadFilter(ExceptionState&);
  AudioBuffer* createBuffer(uint32_t number_of_channels,
                            uint32_t number_of_frames,
                            float sample_rate,
                            ExceptionState&);
  AudioBufferSourceNode* createBufferSource(ExceptionState&);
  ChannelMergerNode* createChannelMerger(ExceptionState&);
  ChannelMergerNode* createChannelMerger(uint32_t number_of_inputs,
                                         ExceptionState&);
  ChannelSplitterNode* createChannelSplitter(ExceptionState&);
  ChannelSplitterNode* createChannelSplitter(uint32_t number_of_outputs,
                                             ExceptionState&);
  ConstantSourceNode* createConstantSource(ExceptionState&);
  ConvolverNode* createConvolver(ExceptionState&);
  DelayNode* createDelay(ExceptionState&);
  DelayNode* createDelay(double max_delay_time, ExceptionState&);
  DynamicsCompressorNode* createDynamicsCompressor(ExceptionState&);
  GainNode* createGain(ExceptionState&);
  IIRFilterNode* createIIRFilter(Vector<double> feedforward_coef,
                                 Vector<double> feedback_coef,
                                 ExceptionState&);
  OscillatorNode* createOscillator(ExceptionState&);
  PannerNode* createPanner(ExceptionState&);
  PeriodicWave* createPeriodicWave(const Vector<float>& real,
                                   const Vector<float>& imag,
                                   ExceptionState&);
  PeriodicWave* createPeriodicWave(const Vector<float>& real,
                                   const Vector<float>& imag,
                                   const PeriodicWaveConstraints*,
                                   ExceptionState&);
  ScriptProcessorNode* createScriptProcessor(ExceptionState&);
  ScriptProcessorNode* createScriptProcessor(uint32_t buffer_size,
                                             ExceptionState&);
  ScriptProcessorNode* createScriptProcessor(uint32_t buffer_size,
                                             uint32_t number_of_input_channels,
                                             ExceptionState&);
  ScriptProcessorNode* createScriptProcessor(uint32_t buffer_size,
                                             uint32_t number_of_input_channels,
                                             uint32_t number_of_output_channels,
                                             ExceptionState&);
  StereoPannerNode* createStereoPanner(ExceptionState&);
  WaveShaperNode* createWaveShaper(ExceptionState&);
  ScriptPromise<AudioBuffer> decodeAudioData(ScriptState*,
                                             DOMArrayBuffer* audio_data,
                                             V8DecodeSuccessCallback*,
                                             V8DecodeErrorCallback*,
                                             ExceptionState&);
  ScriptPromise<AudioBuffer> decodeAudioData(ScriptState*,
                                             DOMArrayBuffer* audio_data,
                                             ExceptionState&);
  ScriptPromise<AudioBuffer> decodeAudioData(ScriptState*,
                                             DOMArrayBuffer* audio_data,
                                             V8DecodeSuccessCallback*,
                                             ExceptionState&);

  // Is the destination node initialized and ready to handle audio?
  bool IsDestinationInitialized() const {
    AudioDestinationNode* dest = destination();
    return dest ? dest->GetAudioDestinationHandler().IsInitialized() : false;
  }

  void Dispose();


  size_t CurrentSampleFrame() const {
    return destination_handler_->CurrentSampleFrame();
  }

  V8AudioContextState::Enum ContextState() const {
    return control_thread_state_;
  }

  // Warn user when creating a node on a closed context.  The node can't do
  // anything useful because the context is closed.
  void WarnIfContextClosed(const AudioHandler*) const;

  // Warn user when connecting two nodes on a closed context. The connection
  // does nothing useful because the context is closed.
  void WarnForConnectionIfContextClosed() const;

  // Return true if the destination is pulling on the audio graph.  Otherwise
  // return false.
  virtual bool IsPullingAudioGraph() const = 0;

  // Handles the promise and callbacks when `.decodeAudioData()` is finished
  // decoding.
  void HandleDecodeAudioData(AudioBuffer*,
                             ScriptPromiseResolver<AudioBuffer>*,
                             V8DecodeSuccessCallback*,
                             V8DecodeErrorCallback*,
                             ExceptionContext);


  virtual bool HasRealtimeConstraint() = 0;

  // When a source node has started processing and needs to be protected,
  // this method tells the context to protect the node.
  //
  // The context itself keeps a reference to all source nodes.  The source
  // nodes, then reference all nodes they're connected to.  In turn, these
  // nodes reference all nodes they're connected to.  All nodes are ultimately
  // connected to the AudioDestinationNode.  When the context release a source
  // node, it will be deactivated from the rendering graph along with all
  // other nodes it is uniquely connected to.
  void NotifySourceNodeStartedProcessing(AudioNode*);
  // When a source node has no more processing to do (has finished playing),
  // this method tells the context to release the corresponding node.
  void NotifySourceNodeFinishedProcessing(AudioHandler*);

  // Called at the start of each render quantum.
  //
  // For an AudioContext:
  //   - `output_position` must be a valid pointer to an AudioIOPosition
  //   - The return value is ignored.
  //
  // For an OfflineAudioContext, we have the following conditions:
  //   - `output_position` must be nullptr because there is no defined
  //   AudioIOPosition.
  //   - The return value indicates whether the context needs to be suspended or
  //   not after rendering.
  virtual bool HandlePreRenderTasks(
      uint32_t frames_to_process,
      const AudioIOPosition* output_position,
      const AudioCallbackMetric* metric,
      base::TimeDelta playout_delay,
      const media::AudioGlitchInfo& glitch_info) = 0;

  // Called at the end of each render quantum.
  virtual void HandlePostRenderTasks() = 0;

  DeferredTaskHandler& GetDeferredTaskHandler() const {
    return *deferred_task_handler_;
  }
  //
  // Thread Safety and Graph Locking:
  //
  // The following functions call corresponding functions of
  // DeferredTaskHandler.
  bool IsAudioThread() const {
    return GetDeferredTaskHandler().IsAudioThread();
  }
  // NO_THREAD_SAFETY_ANALYSIS_FIXME: Stopping here, since the callers (and
  // derived classes are not annotated).
  void lock() NO_THREAD_SAFETY_ANALYSIS_FIXME {
    GetDeferredTaskHandler().lock();
  }
  bool TryLock() { return GetDeferredTaskHandler().TryLock(); }
  void unlock() {
    GetDeferredTaskHandler().AssertGraphOwner();
    GetDeferredTaskHandler().unlock();
  }

  // In DCHECK builds, fails if this thread does not own the context's lock.
  void AssertGraphOwner() const { GetDeferredTaskHandler().AssertGraphOwner(); }

  // Returns the maximum numuber of channels we can support.
  static uint32_t MaxNumberOfChannels() { return kMaxNumberOfChannels; }

  virtual void StartRendering();

  void NotifyStateChange();

  // A context is considered closed if:
  //  - closeContext() has been called.
  //  - it has been stopped by its execution context.
  virtual bool IsContextCleared() const { return is_cleared_; }

  // Get the security origin for this audio context.
  const SecurityOrigin* GetSecurityOrigin() const;

  // Get the PeriodicWave for the specified oscillator type.  The table is
  // initialized internally if necessary.
  PeriodicWave* GetPeriodicWave(int type);

  // Called by handlers of AudioScheduledSourceNode and AudioBufferSourceNode to
  // notify their associated AudioContext when start() is called.
  virtual void NotifySourceNodeStart() = 0;


  // Callback from AudioWorklet, invoked when the associated
  // AudioWorkletGlobalScope is created and the worklet operation is ready after
  // the first script evaluation.
  void NotifyWorkletIsReady();

  // Update the information in AudioWorkletGlobalScope synchronously on the
  // worklet rendering thread. Must be called from the rendering thread.
  // Does nothing when the worklet global scope does not exist.
  void UpdateWorkletGlobalScopeOnRenderingThread();

  // Returns -1 if the destination node is unavailable or any other condition
  // occurs preventing us from determining the count.
  int32_t MaxChannelCount();

  // Returns the platform-specific callback buffer size for Devtools.
  // Returns -1 if the destination node is unavailable or any other condition
  // occurs preventing us from determining the count.
  int32_t CallbackBufferSize();

  // TODO(crbug.com/1055983): Remove this when the execution context validity
  // check is not required in the AudioNode factory methods. Returns false
  // if the execution context does not exist.
  bool CheckExecutionContextAndThrowIfNecessary(ExceptionState&);

 protected:
  enum class ContextType { kRealtimeContext, kOfflineContext };

  explicit BaseAudioContext(LocalDOMWindow*, ContextType);

  void Initialize();
  virtual void Uninitialize();

  void SetContextState(V8AudioContextState::Enum);

  // Tries to handle AudioBufferSourceNodes that were started but became
  // disconnected or was never connected. Because these never get pulled
  // anymore, they will stay around forever. So if we can, try to stop them so
  // they can be collected.
  void HandleStoppableSourceNodes();

  void RejectPendingDecodeAudioDataResolvers();

  // When the context goes away, reject any pending script promise resolvers.
  virtual void RejectPendingResolvers();

  // Returns the window with which the instance is associated.
  LocalDOMWindow* GetWindow() const;

  // The audio thread relies on the main thread to perform some operations over
  // the objects that it owns and controls; this method posts the task to
  // initiate those.
  void ScheduleMainThreadCleanup();

  // Handles promise resolving, stopping and finishing up of audio source nodes
  // etc. Actions that should happen, but can happen asynchronously to the
  // audio thread making rendering progress.
  void PerformCleanupOnMainThread();

  // https://webaudio.github.io/web-audio-api/#dom-baseaudiocontext-pending-promises-slot
  HeapVector<Member<ScriptPromiseResolver<IDLUndefined>>>
      pending_promises_resolvers_;

  Member<AudioDestinationNode> destination_node_;

  // True if we're in the process of resolving promises for resume().  Resolving
  // can take some time and the audio context process loop is very fast, so we
  // don't want to call resolve an excessive number of times.
  bool is_resolving_resume_promises_ = false;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

 private:
  // This is considering 32 is large enough for multiple channels audio.
  // It is somewhat arbitrary and could be increased if necessary.
  static constexpr uint32_t kMaxNumberOfChannels = 32;

  void Clear();

  // When the context goes away, there might still be some sources which
  // haven't finished playing.  Make sure to release them here.
  void ReleaseActiveSourceNodes();

  // The state of an audio context.  On creation, the state is Suspended. The
  // state is Running if audio is being processed (audio graph is being pulled
  // for data). The state is Closed if the audio context has been closed.  The
  // valid transitions are from Suspended to either Running or Closed; Running
  // to Suspended or Closed. Once Closed, there are no valid transitions.
  // https://webaudio.github.io/web-audio-api/#dom-baseaudiocontext-control-thread-state-slot
  V8AudioContextState::Enum control_thread_state_ =
      V8AudioContextState::Enum::kSuspended;

  bool is_cleared_ = false;

  // Listener for the PannerNodes
  Member<AudioListener> listener_;

  // Set to `true` by the audio thread when it posts a main-thread task to
  // perform delayed state sync'ing updates that needs to be done on the main
  // thread. Cleared by the main thread task once it has run.
  bool has_posted_cleanup_task_ = false;

  // Graph locking.
  scoped_refptr<DeferredTaskHandler> deferred_task_handler_;

  AsyncAudioDecoder audio_decoder_;

  // Vector of promises created by decodeAudioData.  This keeps the resolvers
  // alive until decodeAudioData finishes decoding and can tell the main thread
  // to resolve them.
  HeapHashSet<Member<ScriptPromiseResolverBase>> decode_audio_resolvers_;

  // PeriodicWave's for the builtin oscillator types.  These only depend on the
  // sample rate. so they can be shared with all OscillatorNodes in the context.
  // To conserve memory, these are lazily initialized on first use.
  Member<PeriodicWave> periodic_wave_sine_;
  Member<PeriodicWave> periodic_wave_square_;
  Member<PeriodicWave> periodic_wave_sawtooth_;
  Member<PeriodicWave> periodic_wave_triangle_;

  // The handler associated with the above `destination_node_`.
  scoped_refptr<AudioDestinationHandler> destination_handler_;

  Member<AudioWorklet> audio_worklet_;

  // In order to update some information (e.g. current frame) in
  // AudioWorkletGlobalScope *synchronously*, the context needs to keep the
  // reference to the WorkerThread associated with the AudioWorkletGlobalScope.
  // This cannot be nullptr once it is assigned from AudioWorkletThread until
  // the BaseAudioContext goes away.
  raw_ptr<WorkerThread, DanglingUntriaged> audio_worklet_thread_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_BASE_AUDIO_CONTEXT_H_
