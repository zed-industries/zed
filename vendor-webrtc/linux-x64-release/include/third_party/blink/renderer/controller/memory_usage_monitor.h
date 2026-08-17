// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_MEMORY_USAGE_MONITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_MEMORY_USAGE_MONITOR_H_

#include "base/observer_list.h"
#include "base/timer/timer.h"
#include "third_party/blink/renderer/controller/controller_export.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace base {
class SingleThreadTaskRunner;
class TickClock;
}  // namespace base

namespace blink {

// nan means data not available.
struct MemoryUsage {
  double private_footprint_bytes = std::numeric_limits<double>::quiet_NaN();
  double swap_bytes = std::numeric_limits<double>::quiet_NaN();
  double vm_size_bytes = std::numeric_limits<double>::quiet_NaN();
  double peak_resident_bytes = std::numeric_limits<double>::quiet_NaN();
};

// Periodically checks the memory usage and notifies its observers. Monitoring
// automatically starts/stops depending on whether an observer exists.
class CONTROLLER_EXPORT MemoryUsageMonitor {
  USING_FAST_MALLOC(MemoryUsageMonitor);

 public:
  static MemoryUsageMonitor& Instance();
  static void SetInstanceForTesting(MemoryUsageMonitor*);

  class Observer : public base::CheckedObserver {
   public:
    virtual void OnMemoryPing(MemoryUsage) = 0;
  };

  MemoryUsageMonitor();
  virtual ~MemoryUsageMonitor() = default;

  // Returns the current memory usage.
  virtual MemoryUsage GetCurrentMemoryUsage();

  // Ensures that an observer is only added once.
  void AddObserver(Observer*);
  // Observers must be removed before they are destroyed.
  void RemoveObserver(Observer*);
  bool HasObserver(Observer*);

  bool TimerIsActive() const { return timer_.IsRunning(); }

 protected:
  MemoryUsageMonitor(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner_for_testing,
      const base::TickClock* clock_for_testing);

  // Adds V8 related memory usage data to the given struct.
  void GetV8MemoryUsage(MemoryUsage&);
  // Adds Blink related memory usage data to the given struct.
  void GetBlinkMemoryUsage(MemoryUsage&);
  // Adds process related memory usage data to the given struct.
  virtual void GetProcessMemoryUsage(MemoryUsage&) {}

 private:
  virtual void StartMonitoringIfNeeded();
  virtual void StopMonitoring();

  void TimerFired();

  base::RepeatingTimer timer_;
  base::ObserverList<Observer> observers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_MEMORY_USAGE_MONITOR_H_
