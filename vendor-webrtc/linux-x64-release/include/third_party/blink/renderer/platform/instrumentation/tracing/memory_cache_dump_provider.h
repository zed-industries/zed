// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_TRACING_MEMORY_CACHE_DUMP_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_TRACING_MEMORY_CACHE_DUMP_PROVIDER_H_

#include "base/trace_event/memory_dump_provider.h"
#include "base/trace_event/process_memory_dump.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/instrumentation/tracing/web_process_memory_dump.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/wtf.h"

namespace blink {

class PLATFORM_EXPORT MemoryCacheDumpClient : public GarbageCollectedMixin {
 public:
  virtual ~MemoryCacheDumpClient() = default;
  virtual bool OnMemoryDump(WebMemoryDumpLevelOfDetail,
                            WebProcessMemoryDump*) = 0;

  void Trace(Visitor*) const override;
};

// This class is wrapper around MemoryCache to take memory snapshots. It dumps
// the stats of cache only after the cache is created.
class PLATFORM_EXPORT MemoryCacheDumpProvider final
    : public base::trace_event::MemoryDumpProvider {
  USING_FAST_MALLOC(MemoryCacheDumpProvider);

 public:
  // This class is singleton since there is a global MemoryCache object.
  static MemoryCacheDumpProvider* Instance();
  MemoryCacheDumpProvider(const MemoryCacheDumpProvider&) = delete;
  MemoryCacheDumpProvider& operator=(const MemoryCacheDumpProvider&) = delete;
  ~MemoryCacheDumpProvider() override;

  // base::trace_event::MemoryDumpProvider implementation.
  bool OnMemoryDump(const base::trace_event::MemoryDumpArgs&,
                    base::trace_event::ProcessMemoryDump*) override;

  void SetMemoryCache(MemoryCacheDumpClient* client) {
    DCHECK(IsMainThread());
    client_ = client;
  }

 private:
  MemoryCacheDumpProvider();

  WeakPersistent<MemoryCacheDumpClient> client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_TRACING_MEMORY_CACHE_DUMP_PROVIDER_H_
