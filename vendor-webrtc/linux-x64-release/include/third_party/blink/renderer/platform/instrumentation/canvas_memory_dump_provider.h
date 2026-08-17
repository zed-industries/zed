// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_CANVAS_MEMORY_DUMP_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_CANVAS_MEMORY_DUMP_PROVIDER_H_

#include "base/synchronization/lock.h"
#include "base/trace_event/memory_dump_provider.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

class PLATFORM_EXPORT CanvasMemoryDumpClient {
 public:
  virtual void OnMemoryDump(base::trace_event::ProcessMemoryDump*) = 0;
  virtual size_t GetSize() const = 0;

  ~CanvasMemoryDumpClient() = default;
};

class PLATFORM_EXPORT CanvasMemoryDumpProvider final
    : public base::trace_event::MemoryDumpProvider {
  USING_FAST_MALLOC(CanvasMemoryDumpProvider);

 public:
  static CanvasMemoryDumpProvider* Instance();
  CanvasMemoryDumpProvider(const CanvasMemoryDumpProvider&) = delete;
  CanvasMemoryDumpProvider& operator=(const CanvasMemoryDumpProvider&) = delete;
  ~CanvasMemoryDumpProvider() override = default;

  // MemoryDumpProvider implementation.
  bool OnMemoryDump(const base::trace_event::MemoryDumpArgs&,
                    base::trace_event::ProcessMemoryDump*) override;

  void RegisterClient(CanvasMemoryDumpClient*);
  void UnregisterClient(CanvasMemoryDumpClient*);

 private:
  CanvasMemoryDumpProvider() = default;

  base::Lock lock_;
  WTF::HashSet<CanvasMemoryDumpClient*> clients_ GUARDED_BY(lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_CANVAS_MEMORY_DUMP_PROVIDER_H_
