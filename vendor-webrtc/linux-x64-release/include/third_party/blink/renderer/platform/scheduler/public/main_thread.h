// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_MAIN_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_MAIN_THREAD_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"

namespace blink {

// This class restricts access to the `GetTaskRunner`. Only authorized callees
// are allowed. Before you add a restriction consider using a frame or worker
// based API instead.
class MainThreadTaskRunnerRestricted {
 private:
  // Permitted users of `MainThread::GetTaskRunner`.
  friend class BlinkCategorizedWorkerPoolDelegate;
  friend class BlinkInitializer;
  friend class BlobBytesProvider;
  friend class CachedStorageArea;
  friend class DevToolsAgent;
  friend class FontCache;
  friend class InspectorNetworkAgent;
  friend class MemoryCache;
  friend class ParkableImageManager;
  friend class ParkableStringManager;
  friend class RendererResourceCoordinatorImpl;
  friend class SharedGpuContext;
  friend class SharedWorkerReportingProxy;
  friend class ThreadedIconLoader;
  friend class V8WorkerMemoryReporter;
  friend class WebGLWebCodecsVideoFrame;
  friend class WebRtcVideoFrameAdapter;
  friend class WorkerGlobalScope;
  friend class CanvasHibernationHandler;
  friend class HibernatedCanvasMemoryDumpProvider;
  friend class MainThreadTaskRunnerRestrictedForTesting;
  friend MainThreadTaskRunnerRestricted AccessMainThreadForGpuFactories();
  friend MainThreadTaskRunnerRestricted
  AccessMainThreadForWebGraphicsContext3DProvider();
  friend MainThreadTaskRunnerRestricted
  AccessMainThreadForGpuMemoryBufferManager();

  MainThreadTaskRunnerRestricted() = default;
};

class MainThreadTaskRunnerRestrictedForTesting
    : public MainThreadTaskRunnerRestricted {};

// The interface of a main thread in Blink.
//
// This class will has a restricted GetTaskRunner method.
//
class PLATFORM_EXPORT MainThread : public Thread {
 public:
  friend class Platform;  // For SetMainThread() and IsSimpleMainThread().
  friend class ScopedMainThreadOverrider;  // For SetMainThread().

  // Task runner for the main thread. This should only be used in
  // specific scenarios. Likely you want a frame or worker based task runner
  // instead.
  virtual scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(
      MainThreadTaskRunnerRestricted) const = 0;

 private:
  // For Platform and ScopedMainThreadOverrider. Return the thread object
  // previously set (if any).
  //
  // This is done this way because we need to be able to "override" the main
  // thread temporarily for ScopedTestingPlatformSupport.
  static std::unique_ptr<MainThread> SetMainThread(std::unique_ptr<MainThread>);

  // This is used to identify the actual Thread instance. This should be
  // used only in Platform, and other users should ignore this.
  virtual bool IsSimpleMainThread() const { return false; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_MAIN_THREAD_H_
