/*
 * Copyright (C) 2012, Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_OFFLINE_AUDIO_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_OFFLINE_AUDIO_CONTEXT_H_

#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webaudio/base_audio_context.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {
class AudioBuffer;
class ExceptionState;
class OfflineAudioContextOptions;
class OfflineAudioDestinationHandler;

class MODULES_EXPORT OfflineAudioContext final : public BaseAudioContext {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static OfflineAudioContext* Create(ExecutionContext*,
                                     unsigned number_of_channels,
                                     unsigned number_of_frames,
                                     float sample_rate,
                                     ExceptionState&);

  static OfflineAudioContext* Create(ExecutionContext*,
                                     const OfflineAudioContextOptions*,
                                     ExceptionState&);

  OfflineAudioContext(LocalDOMWindow*,
                      unsigned number_of_channels,
                      uint32_t number_of_frames,
                      float sample_rate,
                      ExceptionState&);
  ~OfflineAudioContext() override;

  void Trace(Visitor*) const override;

  uint32_t length() const { return total_render_frames_; }

  ScriptPromise<AudioBuffer> startOfflineRendering(ScriptState*,
                                                   ExceptionState&);

  ScriptPromise<IDLUndefined> suspendContext(ScriptState*,
                                             double,
                                             ExceptionState&);
  ScriptPromise<IDLUndefined> resumeContext(ScriptState*, ExceptionState&);

  void RejectPendingResolvers() override;

  bool HasRealtimeConstraint() final { return false; }

  bool IsPullingAudioGraph() const final;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(complete, kComplete)

  // Fire completion event when the rendering is finished.
  void FireCompletionEvent();

  bool HandlePreRenderTasks(uint32_t frames_to_process,
                            const AudioIOPosition* output_position,
                            const AudioCallbackMetric* metric,
                            base::TimeDelta playout_delay,
                            const media::AudioGlitchInfo& glitch_info) final;
  void HandlePostRenderTasks() final;

  // Resolve a suspend scheduled at the specified frame. With this specified
  // frame as a unique key, the associated promise resolver can be retrieved
  // from the map (m_scheduledSuspends) and resolved.
  void ResolveSuspendOnMainThread(size_t);

  // OfflineAudioContext is not affected by Autoplay, so this MUST do nothing.
  void NotifySourceNodeStart() final {}

  // The HashMap with 'zero' key is needed because `CurrentSampleFrame()` can be
  // zero.
  using SuspendMap = HeapHashMap<size_t,
                                 Member<ScriptPromiseResolver<IDLUndefined>>,
                                 IntWithZeroKeyHashTraits<size_t>>;

  bool HasPendingActivity() const final;

 private:
  // Fetch directly the destination handler.
  OfflineAudioDestinationHandler& DestinationHandler();

  // Check if the rendering needs to be suspended.
  bool ShouldSuspend();

  // This map is to store the timing of scheduled suspends (frame) and the
  // associated promise resolver. This storage can only be modified by the
  // main thread and accessed by the audio thread with the graph lock.
  //
  // The map consists of key-value pairs of:
  // { size_t quantized_frame: ScriptPromiseResolverBase resolver }
  //
  // Note that `quantized_frame` is a unique key, since you can have only one
  // suspend scheduled for a certain frame. Accessing to this must be
  // protected by the offline context lock.
  SuspendMap scheduled_suspends_;

  base::Lock suspend_frames_lock_;
  // Holds copies of `quantized_frame` in `scheduled_suspends_` to ensure
  // a safe access from the audio thread.
  HashSet<size_t, IntWithZeroKeyHashTraits<size_t>> scheduled_suspend_frames_
      GUARDED_BY(suspend_frames_lock_);

  Member<ScriptPromiseResolver<AudioBuffer>> complete_resolver_;

  // This flag is necessary to indicate the rendering has actually started or
  // running. Note that initial state of context is 'Suspended', which is the
  // same state when the context is suspended, so we cannot utilize it for this
  // purpose.
  bool is_rendering_started_ = false;

  // Total render sample length.
  uint32_t total_render_frames_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_OFFLINE_AUDIO_CONTEXT_H_
