// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_MEMORY_DUMP_PROVIDER_H_
#define BASE_TRACE_EVENT_MEMORY_DUMP_PROVIDER_H_

#include "base/base_export.h"
#include "base/process/process_handle.h"
#include "base/trace_event/memory_dump_request_args.h"

namespace base {
namespace trace_event {

class ProcessMemoryDump;

// The contract interface that memory dump providers must implement.
class BASE_EXPORT MemoryDumpProvider {
 public:
  // Optional arguments for MemoryDumpManager::RegisterDumpProvider().
  struct Options {
    Options() : dumps_on_single_thread_task_runner(false) {}

    // |dumps_on_single_thread_task_runner| is true if the dump provider runs on
    // a SingleThreadTaskRunner, which is usually the case. It is faster to run
    // all providers that run on the same thread together without thread hops.
    bool dumps_on_single_thread_task_runner;
  };

  MemoryDumpProvider(const MemoryDumpProvider&) = delete;
  MemoryDumpProvider& operator=(const MemoryDumpProvider&) = delete;
  virtual ~MemoryDumpProvider() = default;

  // Called by the MemoryDumpManager when generating memory dumps.
  // The |args| specify if the embedder should generate light/heavy dumps on
  // dump requests. The embedder should return true if the |pmd| was
  // successfully populated, false if something went wrong and the dump should
  // be considered invalid.
  // (Note, the MemoryDumpManager has a fail-safe logic which will disable the
  // MemoryDumpProvider for the entire trace session if it fails consistently).
  virtual bool OnMemoryDump(const MemoryDumpArgs& args,
                            ProcessMemoryDump* pmd) = 0;

 protected:
  MemoryDumpProvider() = default;
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_MEMORY_DUMP_PROVIDER_H_
