// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_TRACING_WEB_PROCESS_MEMORY_DUMP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_TRACING_WEB_PROCESS_MEMORY_DUMP_H_

#include <memory>
#include <unordered_map>

#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/trace_event/heap_profiler_allocation_context.h"
#include "base/trace_event/memory_dump_request_args.h"
#include "third_party/blink/renderer/platform/instrumentation/tracing/web_memory_allocator_dump.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

class SkTraceMemoryDump;

namespace base {
class DiscardableMemory;
namespace trace_event {
class MemoryAllocatorDump;
class ProcessMemoryDump;
}  // namespace base
}  // namespace trace_event

namespace skia {
class SkiaTraceMemoryDumpImpl;
}  // namespace skia

namespace blink {

// Used to specify the type of memory dump the WebProcessMemoryDump should
// generate on dump requests.
// TODO(hajimehoshi): Remove this and use base::trace_event::
// MemoryDumpLevelOfDetail instead.
enum class WebMemoryDumpLevelOfDetail { kBackground, kLight, kDetailed };

// A container which holds all the dumps for the various allocators for a given
// process. Embedders of WebMemoryDumpProvider are expected to populate a
// WebProcessMemoryDump instance with the stats of their allocators.
class PLATFORM_EXPORT WebProcessMemoryDump final {
  USING_FAST_MALLOC(WebProcessMemoryDump);

 public:
  // Creates a standalone WebProcessMemoryDump, which owns the underlying
  // ProcessMemoryDump.
  WebProcessMemoryDump();
  WebProcessMemoryDump(const WebProcessMemoryDump&) = delete;
  WebProcessMemoryDump& operator=(const WebProcessMemoryDump&) = delete;

  // Wraps (without owning) an existing ProcessMemoryDump.
  explicit WebProcessMemoryDump(
      base::trace_event::MemoryDumpLevelOfDetail level_of_detail,
      base::trace_event::ProcessMemoryDump* process_memory_dump);

  ~WebProcessMemoryDump();

  // Creates a new MemoryAllocatorDump with the given name and returns the
  // empty object back to the caller. |absoluteName| uniquely identifies the
  // dump within the scope of a ProcessMemoryDump. It is possible to express
  // nesting by means of a slash-separated path naming (e.g.,
  // "allocator_name/arena_1/subheap_X").
  // |guid| is  an optional identifier, unique among all processes within the
  // scope of a global dump. This is only relevant when using
  // addOwnershipEdge(). If omitted, it will be automatically generated.
  blink::WebMemoryAllocatorDump* CreateMemoryAllocatorDump(
      const String& absolute_name);
  blink::WebMemoryAllocatorDump* CreateMemoryAllocatorDump(
      const String& absolute_name,
      blink::WebMemoryAllocatorDumpGuid guid);

  // Gets a previously created MemoryAllocatorDump given its name.
  blink::WebMemoryAllocatorDump* GetMemoryAllocatorDump(
      const String& absolute_name) const;

  // Removes all the WebMemoryAllocatorDump(s) contained in this instance.
  // This WebProcessMemoryDump can be safely reused as if it was new once this
  // method returns.
  void Clear();

  // Merges all WebMemoryAllocatorDump(s) contained in |other| inside this
  // WebProcessMemoryDump, transferring their ownership to this instance.
  // |other| will be an empty WebProcessMemoryDump after this method returns
  // and can be reused as if it was new.
  void TakeAllDumpsFrom(blink::WebProcessMemoryDump* other);

  // Adds an ownership relationship between two MemoryAllocatorDump(s) with
  // the semantics: |source| owns |target|, and has the effect of attributing
  // the memory usage of |target| to |source|. |importance| is optional and
  // relevant only for the cases of co-ownership, where it acts as a z-index:
  // the owner with the highest importance will be attributed |target|'s
  // memory.
  void AddOwnershipEdge(blink::WebMemoryAllocatorDumpGuid source,
                        blink::WebMemoryAllocatorDumpGuid target,
                        int importance);
  void AddOwnershipEdge(blink::WebMemoryAllocatorDumpGuid source,
                        blink::WebMemoryAllocatorDumpGuid target);

  // Utility method to add a suballocation relationship with the following
  // semantics: |source| is suballocated from |target_node_name|.
  // This creates a child node of |target_node_name| and adds an ownership
  // edge between |source| and the new child node. As a result, the UI will
  // not account the memory of |source| in the target node.
  void AddSuballocation(blink::WebMemoryAllocatorDumpGuid source,
                        const String& target_node_name);

  // Returns the SkTraceMemoryDump proxy interface that can be passed to Skia
  // to dump into this WebProcessMemoryDump. Multiple SkTraceMemoryDump
  // objects can be created using this method. The created dumpers are owned
  // by WebProcessMemoryDump and cannot outlive the WebProcessMemoryDump
  // object owning them. |dumpNamePrefix| is prefix appended to each dump
  // created by the SkTraceMemoryDump implementation, if the dump should be
  // placed under different namespace and not "skia".
  SkTraceMemoryDump* CreateDumpAdapterForSkia(const String& dump_name_prefix);

  const base::trace_event::ProcessMemoryDump* process_memory_dump() const {
    return process_memory_dump_;
  }

  blink::WebMemoryAllocatorDump* CreateDiscardableMemoryAllocatorDump(
      const std::string& name,
      base::DiscardableMemory* discardable);

 private:
  FRIEND_TEST_ALL_PREFIXES(WebProcessMemoryDumpTest, IntegrationTest);

  blink::WebMemoryAllocatorDump* CreateWebMemoryAllocatorDump(
      base::trace_event::MemoryAllocatorDump* memory_allocator_dump);

  // Only for the case of ProcessMemoryDump being owned (i.e. the default ctor).
  std::unique_ptr<base::trace_event::ProcessMemoryDump>
      owned_process_memory_dump_;

  // The underlying ProcessMemoryDump instance to which the
  // createMemoryAllocatorDump() calls will be proxied to.
  raw_ptr<base::trace_event::ProcessMemoryDump>
      process_memory_dump_;  // Not owned.

  // TODO(ssid): Remove it once this information is added to ProcessMemoryDump.
  base::trace_event::MemoryDumpLevelOfDetail level_of_detail_;

  // Reverse index of MemoryAllocatorDump -> WebMemoryAllocatorDump wrapper.
  // By design WebMemoryDumpProvider(s) are not supposed to hold the pointer
  // to the WebProcessMemoryDump passed as argument of the onMemoryDump() call.
  // Those pointers are valid only within the scope of the call and can be
  // safely torn down once the WebProcessMemoryDump itself is destroyed.
  HashMap<base::trace_event::MemoryAllocatorDump*,
          std::unique_ptr<WebMemoryAllocatorDump>>
      memory_allocator_dumps_;

  // Stores SkTraceMemoryDump for the current ProcessMemoryDump.
  Vector<std::unique_ptr<skia::SkiaTraceMemoryDumpImpl>> sk_trace_dump_list_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_TRACING_WEB_PROCESS_MEMORY_DUMP_H_
