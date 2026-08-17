// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WORKLET_ANIMATION_AND_PAINT_WORKLET_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WORKLET_ANIMATION_AND_PAINT_WORKLET_THREAD_H_

#include <memory>
#include "third_party/blink/renderer/core/workers/worker_thread.h"
#include "third_party/blink/renderer/core/workers/worklet_thread_holder.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class WorkerReportingProxy;

// Represents the shared backing thread that is used by both animation worklets
// and off-thread paint worklets. This thread participates in the Blink garbage
// collection process.
class MODULES_EXPORT AnimationAndPaintWorkletThread final
    : public WorkerThread {
 public:
  static std::unique_ptr<AnimationAndPaintWorkletThread>
  CreateForAnimationWorklet(WorkerReportingProxy&);
  static std::unique_ptr<AnimationAndPaintWorkletThread> CreateForPaintWorklet(
      WorkerReportingProxy&);
  ~AnimationAndPaintWorkletThread() override;

  WorkerBackingThread& GetWorkerBackingThread() override;

  // This may block the main thread.
  static void CollectAllGarbageForTesting();

  static WorkletThreadHolder<AnimationAndPaintWorkletThread>*
  GetWorkletThreadHolderForTesting();

 private:
  enum class WorkletType {
    kAnimation,
    kPaint,
  };

  explicit AnimationAndPaintWorkletThread(WorkletType, WorkerReportingProxy&);

  WorkerOrWorkletGlobalScope* CreateWorkerGlobalScope(
      std::unique_ptr<GlobalScopeCreationParams>) final;

  bool IsOwningBackingThread() const override { return false; }

  ThreadType GetThreadType() const override {
    return ThreadType::kAnimationAndPaintWorkletThread;
  }

  void EnsureSharedBackingThread();
  void ClearSharedBackingThread();

  WorkletType worklet_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WORKLET_ANIMATION_AND_PAINT_WORKLET_THREAD_H_
