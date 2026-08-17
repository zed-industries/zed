// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_INSTANCE_COUNTERS_MEMORY_DUMP_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_INSTANCE_COUNTERS_MEMORY_DUMP_PROVIDER_H_

#include "base/trace_event/memory_dump_provider.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class PLATFORM_EXPORT InstanceCountersMemoryDumpProvider final
    : public base::trace_event::MemoryDumpProvider {
  USING_FAST_MALLOC(InstanceCountersMemoryDumpProvider);

 public:
  static InstanceCountersMemoryDumpProvider* Instance();
  InstanceCountersMemoryDumpProvider(
      const InstanceCountersMemoryDumpProvider&) = delete;
  InstanceCountersMemoryDumpProvider& operator=(
      const InstanceCountersMemoryDumpProvider&) = delete;
  ~InstanceCountersMemoryDumpProvider() override = default;

  // MemoryDumpProvider implementation.
  bool OnMemoryDump(const base::trace_event::MemoryDumpArgs&,
                    base::trace_event::ProcessMemoryDump*) override;

 private:
  InstanceCountersMemoryDumpProvider() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_INSTANCE_COUNTERS_MEMORY_DUMP_PROVIDER_H_
