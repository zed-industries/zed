// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_CRASH_MEMORY_METRICS_REPORTER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_CRASH_MEMORY_METRICS_REPORTER_IMPL_H_

#include "base/files/scoped_file.h"
#include "base/gtest_prod_util.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "third_party/blink/public/common/oom_intervention/oom_intervention_types.h"
#include "third_party/blink/public/mojom/crash/crash_memory_metrics_reporter.mojom-blink.h"
#include "third_party/blink/renderer/controller/controller_export.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

// Writes data about renderer into shared memory that will be read by browser.
class CONTROLLER_EXPORT CrashMemoryMetricsReporterImpl
    : public mojom::blink::CrashMemoryMetricsReporter {
 public:
  static CrashMemoryMetricsReporterImpl& Instance();
  static void Bind(
      mojo::PendingReceiver<mojom::blink::CrashMemoryMetricsReporter> receiver);

  ~CrashMemoryMetricsReporterImpl() override;

  // mojom::CrashMemoryMetricsReporter implementations:
  void SetSharedMemory(
      base::UnsafeSharedMemoryRegion shared_metrics_buffer) override;

  // This method tracks when an allocation failure occurs. It should be hooked
  // into all platform allocation failure handlers in a process such as
  // base::TerminateBecauseOutOfMemory() and OOM_CRASH() in PartitionAlloc.
  // TODO(yuzus): Now only called from OOM_CRASH(). Call this from malloc/new
  // failures and base::TerminateBecauseOutOfMemory(), too.
  static void OnOOMCallback();

 protected:
  CrashMemoryMetricsReporterImpl();

 private:
  FRIEND_TEST_ALL_PREFIXES(OomInterventionImplTest, CalculateProcessFootprint);

  void WriteIntoSharedMemory();
  void SampleMemoryState(TimerBase*);

  OomInterventionMetrics last_reported_metrics_;
  base::WritableSharedMemoryMapping shared_metrics_mapping_;
  mojo::Receiver<mojom::blink::CrashMemoryMetricsReporter> receiver_{this};
  TaskRunnerTimer<CrashMemoryMetricsReporterImpl> timer_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_CRASH_MEMORY_METRICS_REPORTER_IMPL_H_
