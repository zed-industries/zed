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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_GECKO_GECKO_EVENT_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_GECKO_GECKO_EVENT_H_

#include <cstdint>
#include <variant>

#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/tables/profiler_tables_py.h"

namespace perfetto::trace_processor::gecko_importer {

struct alignas(8) GeckoEvent {
  struct ThreadMetadata {
    uint32_t tid;
    uint32_t pid;
    StringPool::Id name;
  };
  struct StackSample {
    uint32_t tid;
    tables::StackProfileCallsiteTable::Id callsite_id;
  };
  using OneOf = std::variant<ThreadMetadata, StackSample>;
  OneOf oneof;
};

}  // namespace perfetto::trace_processor::gecko_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_GECKO_GECKO_EVENT_H_
