// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_REALTIME_AUDIO_WORKLET_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_REALTIME_AUDIO_WORKLET_THREAD_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/workers/worker_thread.h"
#include "third_party/blink/renderer/core/workers/worklet_thread_holder.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class WorkerReportingProxy;

// RealtimeAudioWorkletThread is a per-AudioWorklet object that has a hybrid
// threading management system. Up to 3 instances of AudioWorklet, this class
// provides a dedicated backing thread for more performant audio processing,
// but starting from the 4th and subsequent instances will use a shared
// backing thread managed with reference counting.
class MODULES_EXPORT RealtimeAudioWorkletThread final : public WorkerThread {
 public:
  RealtimeAudioWorkletThread(WorkerReportingProxy& worker_reporting_proxy,
                             base::TimeDelta realtime_buffer_duration);
  ~RealtimeAudioWorkletThread() final;

  WorkerBackingThread& GetWorkerBackingThread() final;

  private:
  WorkerOrWorkletGlobalScope* CreateWorkerGlobalScope(
      std::unique_ptr<GlobalScopeCreationParams>) final;
  bool IsOwningBackingThread() const final {
    return worker_backing_thread_ != nullptr;
  }
  ThreadType GetThreadType() const final {
    return ThreadType::kRealtimeAudioWorkletThread;
  }

  // For the instance that uses a shared backing thread, this is nullptr.
  std::unique_ptr<WorkerBackingThread> worker_backing_thread_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_REALTIME_AUDIO_WORKLET_THREAD_H_
