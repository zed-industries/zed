// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACING_PERFETTO_TASK_RUNNER_H_
#define BASE_TRACING_PERFETTO_TASK_RUNNER_H_

#include "base/base_export.h"
#include "base/cancelable_callback.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "base/task/sequenced_task_runner.h"
#include "base/timer/timer.h"
#include "build/build_config.h"
#include "third_party/perfetto/include/perfetto/base/task_runner.h"

#if (BUILDFLAG(IS_POSIX) && !BUILDFLAG(IS_NACL)) || BUILDFLAG(IS_FUCHSIA)
// Needed for base::FileDescriptorWatcher::Controller and for implementing
// AddFileDescriptorWatch & RemoveFileDescriptorWatch.
#include <map>

#include "base/files/file_descriptor_watcher_posix.h"
#endif  // (BUILDFLAG(IS_POSIX) && !BUILDFLAG(IS_NACL)) || BUILDFLAG(IS_FUCHSIA)

namespace base {
namespace tracing {

// This wraps a base::TaskRunner implementation to be able
// to provide it to Perfetto.
class BASE_EXPORT PerfettoTaskRunner : public perfetto::base::TaskRunner {
 public:
  explicit PerfettoTaskRunner(scoped_refptr<base::SequencedTaskRunner>);
  ~PerfettoTaskRunner() override;
  PerfettoTaskRunner(const PerfettoTaskRunner&) = delete;
  void operator=(const PerfettoTaskRunner&) = delete;

  // perfetto::base::TaskRunner implementation. Only called by
  // the Perfetto implementation itself.
  void PostTask(std::function<void()> task) override;
  void PostDelayedTask(std::function<void()> task, uint32_t delay_ms) override;
  // This in Chrome would more correctly be called "RunsTasksInCurrentSequence".
  // Perfetto calls this to determine wheather CommitData requests should be
  // flushed synchronously. RunsTasksInCurrentSequence is sufficient for that
  // use case.
  bool RunsTasksOnCurrentThread() const override;

  // These are only used on Android when talking to the system Perfetto service.
  void AddFileDescriptorWatch(perfetto::base::PlatformHandle,
                              std::function<void()>) override;
  void RemoveFileDescriptorWatch(perfetto::base::PlatformHandle) override;

  void ResetTaskRunner(scoped_refptr<base::SequencedTaskRunner>);

  WeakPtr<PerfettoTaskRunner> GetWeakPtr() {
    return weak_factory_.GetWeakPtr();
  }

 private:
  scoped_refptr<base::SequencedTaskRunner> task_runner_;

#if (BUILDFLAG(IS_POSIX) && !BUILDFLAG(IS_NACL)) || BUILDFLAG(IS_FUCHSIA)
  // FDControllerAndCallback keeps track of the state of FD watching:
  // * |controller| has value: FD watching is added. |callback| is nullopt.
  // * |controller| is nullptr: FD watching is pending for add. |callback| has
  //   a value.
  // It's safe to call RemoveFileDescriptorWatch in either of the above states.
  // |controller| and |callback| can't be both non-null after returning from
  // AddFileDescriptorWatch or RemoveFileDescriptorWatch.
  struct FDControllerAndCallback {
    std::unique_ptr<base::FileDescriptorWatcher::Controller> controller;
    base::CancelableOnceClosure callback;

    FDControllerAndCallback();
    ~FDControllerAndCallback();
  };
  std::map<int, FDControllerAndCallback> fd_controllers_;
#endif  // (BUILDFLAG(IS_POSIX) && !BUILDFLAG(IS_NACL)) || BUILDFLAG(IS_FUCHSIA)

  WeakPtrFactory<PerfettoTaskRunner> weak_factory_{this};
};

}  // namespace tracing
}  // namespace base

#endif  // BASE_TRACING_PERFETTO_TASK_RUNNER_H_
