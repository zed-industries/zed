// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_PRIVATE_MEMORY_FOOTPRINT_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_PRIVATE_MEMORY_FOOTPRINT_PROVIDER_H_

#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/controller/controller_export.h"
#include "third_party/blink/renderer/controller/memory_usage_monitor.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace base {
class SingleThreadTaskRunner;
class TickClock;
}  // namespace base

namespace blink {

// Provides this renderer process' private memory footprint for browser process.
class CONTROLLER_EXPORT PrivateMemoryFootprintProvider
    : public MemoryUsageMonitor::Observer {
  USING_FAST_MALLOC(PrivateMemoryFootprintProvider);

 public:
  // Initializes the shared instance. Has no effect if called more than once.
  static void Initialize(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

 private:
  explicit PrivateMemoryFootprintProvider(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  PrivateMemoryFootprintProvider(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner_for_testing,
      const base::TickClock* clock);

  // MemoryUsageMonitor::Observer:
  void OnMemoryPing(MemoryUsage) override;

  // Use mojom to provide private memory footprint for this RendererHost in
  // the browser process.
  void SetPrivateMemoryFootprint(uint64_t private_memory_footprint_bytes);

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  raw_ptr<const base::TickClock> clock_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_PRIVATE_MEMORY_FOOTPRINT_PROVIDER_H_
