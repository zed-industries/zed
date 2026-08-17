/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_JIT_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_JIT_TRACKER_H_

#include <cstdint>
#include <memory>

#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/importers/common/address_range.h"
#include "src/trace_processor/importers/common/stack_profile_tracker.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor {

class JitCache;

// Keeps track of Jitted code.
class JitTracker : public Destructible {
 public:
  static JitTracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->jit_tracker) {
      context->jit_tracker.reset(new JitTracker(context));
    }
    return static_cast<JitTracker*>(context->jit_tracker.get());
  }

  ~JitTracker() override;

  // Creates a JitCache. Any frame interning request for the given pid in the
  // given address range will be forwarded from the StackProfileTracker to this
  // cache.
  JitCache* CreateJitCache(std::string name,
                           UniquePid upid,
                           AddressRange range);

 private:
  explicit JitTracker(TraceProcessorContext* context);

  FrameId InternUnknownFrame(MappingId mapping_id, uint64_t rel_pc);

  TraceProcessorContext* const context_;

  base::FlatHashMap<UniquePid, AddressRangeMap<std::unique_ptr<JitCache>>>
      caches_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_JIT_TRACKER_H_
