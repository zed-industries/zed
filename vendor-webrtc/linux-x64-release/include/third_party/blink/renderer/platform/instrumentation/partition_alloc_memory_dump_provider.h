// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_PARTITION_ALLOC_MEMORY_DUMP_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_PARTITION_ALLOC_MEMORY_DUMP_PROVIDER_H_

#include "base/trace_event/memory_dump_provider.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class PLATFORM_EXPORT PartitionAllocMemoryDumpProvider final
    : public base::trace_event::MemoryDumpProvider {
  // TODO(tasak): PartitionAllocMemoryDumpProvider should be
  // USING_FAST_MALLOC. c.f. crbug.com/584196

 public:
  static PartitionAllocMemoryDumpProvider* Instance();
  PartitionAllocMemoryDumpProvider(const PartitionAllocMemoryDumpProvider&) =
      delete;
  PartitionAllocMemoryDumpProvider& operator=(
      const PartitionAllocMemoryDumpProvider&) = delete;
  ~PartitionAllocMemoryDumpProvider() override;

  // MemoryDumpProvider implementation.
  bool OnMemoryDump(const base::trace_event::MemoryDumpArgs&,
                    base::trace_event::ProcessMemoryDump*) override;

 private:
  PartitionAllocMemoryDumpProvider();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_PARTITION_ALLOC_MEMORY_DUMP_PROVIDER_H_
