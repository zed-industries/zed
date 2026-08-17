// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_JAVA_HEAP_DUMP_PROVIDER_ANDROID_H_
#define BASE_TRACE_EVENT_JAVA_HEAP_DUMP_PROVIDER_ANDROID_H_

#include "base/memory/singleton.h"
#include "base/trace_event/memory_dump_provider.h"

namespace base {
namespace trace_event {

// Dump provider which collects process-wide memory stats.
class BASE_EXPORT JavaHeapDumpProvider : public MemoryDumpProvider {
 public:
  static JavaHeapDumpProvider* GetInstance();

  JavaHeapDumpProvider(const JavaHeapDumpProvider&) = delete;
  JavaHeapDumpProvider& operator=(const JavaHeapDumpProvider&) = delete;

  // MemoryDumpProvider implementation.
  bool OnMemoryDump(const MemoryDumpArgs& args,
                    ProcessMemoryDump* pmd) override;

 private:
  friend struct DefaultSingletonTraits<JavaHeapDumpProvider>;

  JavaHeapDumpProvider() = default;
  ~JavaHeapDumpProvider() override = default;
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_JAVA_HEAP_DUMP_PROVIDER_ANDROID_H_
