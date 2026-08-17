// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_FRAME_WORKER_ANIMATION_FRAME_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_FRAME_WORKER_ANIMATION_FRAME_PROVIDER_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/frame_request_callback_collection.h"
#include "third_party/blink/renderer/platform/graphics/begin_frame_provider.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class OffscreenCanvas;

// WorkerAnimationFrameProvider is a member of WorkerGlobalScope and it provides
// RequestAnimationFrame capabilities to Workers.
//
// It's responsible for registering and dealing with callbacks.
// And maintains a connection with the Display process, through
// CompositorFrameSink, that is used to v-sync with the display.
//
// OffscreenCanvases can notify when there's been a change on any
// OffscreenCanvas that is connected to a Canvas, and this class signals
// OffscreenCanvases when it's time to dispatch frames.
class CORE_EXPORT WorkerAnimationFrameProvider
    : public GarbageCollected<WorkerAnimationFrameProvider>,
      public BeginFrameProviderClient {
 public:
  WorkerAnimationFrameProvider(
      ExecutionContext* context,
      const BeginFrameProviderParams& begin_frame_provider_params);
  WorkerAnimationFrameProvider(const WorkerAnimationFrameProvider&) = delete;
  WorkerAnimationFrameProvider& operator=(const WorkerAnimationFrameProvider&) =
      delete;

  int RegisterCallback(FrameCallback* callback);
  void CancelCallback(int id);

  void Trace(Visitor* visitor) const override;

  // BeginFrameProviderClient
  void BeginFrame(const viz::BeginFrameArgs&) override;
  scoped_refptr<base::SingleThreadTaskRunner> GetCompositorTaskRunner()
      override;

  void RegisterOffscreenCanvas(OffscreenCanvas*);
  void DeregisterOffscreenCanvas(OffscreenCanvas*);

  static const int kInvalidCallbackId = -1;

 private:
  const Member<BeginFrameProvider> begin_frame_provider_;
  FrameRequestCallbackCollection callback_collection_;

  HeapLinkedHashSet<WeakMember<OffscreenCanvas>> offscreen_canvases_;

  Member<ExecutionContext> context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_FRAME_WORKER_ANIMATION_FRAME_PROVIDER_H_
