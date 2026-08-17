// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_CPUFREQ_MONITOR_ANDROID_H_
#define BASE_TRACE_EVENT_CPUFREQ_MONITOR_ANDROID_H_

#include <atomic>

#include "base/base_export.h"
#include "base/files/scoped_file.h"
#include "base/memory/scoped_refptr.h"
#include "base/trace_event/trace_log.h"

namespace base {

class SingleThreadTaskRunner;

namespace trace_event {

// A delegate to isolate CPU frequency monitor functionality mainly for testing.
class BASE_EXPORT CPUFreqMonitorDelegate {
 public:
  CPUFreqMonitorDelegate();

  CPUFreqMonitorDelegate(const CPUFreqMonitorDelegate&) = delete;
  CPUFreqMonitorDelegate& operator=(const CPUFreqMonitorDelegate&) = delete;

  virtual ~CPUFreqMonitorDelegate() = default;

  // Returns a vector of the minimal set of CPU IDs that we need to monitor to
  // get CPU frequency information. For CPUs that operate cores in a cluster,
  // i.e. modern Qualcomm 8 cores, this is CPU0 and CPU4.
  virtual void GetCPUIds(std::vector<unsigned int>* ids) const;

  // Reads the kernel_max_cpu file to determine the max CPU ID, i.e. 7 on an
  // 8-core CPU.
  virtual unsigned int GetKernelMaxCPUs() const;

  // Reads the frequency from the CPUs being monitored and records them.
  virtual void RecordFrequency(unsigned int cpu_id, unsigned int freq);

  // Returns whether or not the tracing category our CPU Frequency counters are
  // in is enabled to determine if we should record.
  virtual bool IsTraceCategoryEnabled() const;

  // Gets the path to CPU frequency related files for a particular CPU ID.
  virtual std::string GetScalingCurFreqPathString(unsigned int cpu_id) const;
  virtual std::string GetRelatedCPUsPathString(unsigned int cpu_id) const;

  // Allows us to delay creating a task runner, necessary because many tests
  // don't like us creating one outside of a TaskEnvironment.
  virtual scoped_refptr<SingleThreadTaskRunner> CreateTaskRunner();
};

// A class for monitoring the CPU frequency on unique cores/clusters.
class BASE_EXPORT CPUFreqMonitor : public TraceLog::EnabledStateObserver {
 public:
  // Overhead of reading one cluster on a Nexus 6P is ~0.1ms per CPU. 50ms seems
  // frequent enough to get a general idea of CPU frequency trends.
  static const size_t kDefaultCPUFreqSampleIntervalMs = 50;

  CPUFreqMonitor();

  CPUFreqMonitor(const CPUFreqMonitor&) = delete;
  CPUFreqMonitor& operator=(const CPUFreqMonitor&) = delete;

  ~CPUFreqMonitor() override;

  static CPUFreqMonitor* GetInstance();

  void Start();
  void Stop();

  // TraceLog::EnabledStateObserver.
  void OnTraceLogEnabled() override;
  void OnTraceLogDisabled() override;

  bool IsEnabledForTesting();

 private:
  friend class CPUFreqMonitorTest;

  CPUFreqMonitor(std::unique_ptr<CPUFreqMonitorDelegate> delegate);

  void Sample(std::vector<std::pair<unsigned int, base::ScopedFD>> fds);

  // Uses the delegate's CreateTaskRunner function to lazily create a task
  // runner so we don't illegally create a task runner on Chrome startup for
  // various tests.
  const scoped_refptr<SingleThreadTaskRunner>& GetOrCreateTaskRunner();

  std::atomic<bool> is_enabled_{false};
  scoped_refptr<SingleThreadTaskRunner> task_runner_;
  std::unique_ptr<CPUFreqMonitorDelegate> delegate_;
  base::WeakPtrFactory<CPUFreqMonitor> weak_ptr_factory_{this};
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_CPUFREQ_MONITOR_ANDROID_H_
