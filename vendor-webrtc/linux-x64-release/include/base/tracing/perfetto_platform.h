// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACING_PERFETTO_PLATFORM_H_
#define BASE_TRACING_PERFETTO_PLATFORM_H_

#include "base/base_export.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/threading/thread_local_storage.h"
#include "third_party/perfetto/include/perfetto/base/thread_utils.h"
#include "third_party/perfetto/include/perfetto/tracing/platform.h"

namespace base {

namespace tracing {

class PerfettoTaskRunner;

class BASE_EXPORT PerfettoPlatform : public perfetto::Platform {
 public:
  struct Options final {
    // The prefix to return in GetCurrentProcessName(). This makes it possible
    // to customize the Perfetto "producer name", which can then be used as
    // a filter in the Perfetto trace config (see
    // TraceConfig.DataSource.producer_name_filter).
    std::string process_name_prefix = "org.chromium-";

    // Work around https://bugs.llvm.org/show_bug.cgi?id=36684
    static Options Default() { return {}; }
  };

  explicit PerfettoPlatform(scoped_refptr<base::SequencedTaskRunner>,
                            Options options = Options::Default());
  ~PerfettoPlatform() override;

  // perfetto::Platform implementation:
  ThreadLocalObject* GetOrCreateThreadLocalObject() override;
  std::unique_ptr<perfetto::base::TaskRunner> CreateTaskRunner(
      const CreateTaskRunnerArgs&) override;
  std::string GetCurrentProcessName() override;

  // Chrome uses different thread IDs than Perfetto on Mac. So we need to
  // override this method to keep Perfetto tracks consistent with Chrome
  // thread IDs.
  perfetto::base::PlatformThreadId GetCurrentThreadId() override;

  void ResetTaskRunner(scoped_refptr<base::SequencedTaskRunner> task_runner);

 private:
  const std::string process_name_prefix_;
  WeakPtr<PerfettoTaskRunner> perfetto_task_runner_;
  scoped_refptr<base::SequencedTaskRunner> task_runner_;
  ThreadLocalStorage::Slot thread_local_object_;
};

}  // namespace tracing
}  // namespace base

#endif  // BASE_TRACING_PERFETTO_PLATFORM_H_
