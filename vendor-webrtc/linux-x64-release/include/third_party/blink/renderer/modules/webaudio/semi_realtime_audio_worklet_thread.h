// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_SEMI_REALTIME_AUDIO_WORKLET_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_SEMI_REALTIME_AUDIO_WORKLET_THREAD_H_

#include "third_party/blink/renderer/core/workers/worker_thread.h"
#include "third_party/blink/renderer/core/workers/worklet_thread_holder.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class WorkerReportingProxy;

// SemiRealtimeAudioWorkletThread is a per-AudioWorkletGlobalScope object that
// has a reference count to the backing thread that performs AudioWorklet tasks.
// This object is used by an AudioWorklet spawned by non-MainFrame and its
// backing thread has DISPLAY priority,
class MODULES_EXPORT SemiRealtimeAudioWorkletThread final : public WorkerThread {
 public:
  explicit SemiRealtimeAudioWorkletThread(WorkerReportingProxy&);
  ~SemiRealtimeAudioWorkletThread() final;

  WorkerBackingThread& GetWorkerBackingThread() final;

  static void ClearSharedBackingThread();

 private:
  WorkerOrWorkletGlobalScope* CreateWorkerGlobalScope(
      std::unique_ptr<GlobalScopeCreationParams>) final;
  bool IsOwningBackingThread() const final { return false; }
  ThreadType GetThreadType() const final {
    return ThreadType::kSemiRealtimeAudioWorkletThread;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_SEMI_REALTIME_AUDIO_WORKLET_THREAD_H_
