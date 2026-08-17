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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_STORAGE_HANDLE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_STORAGE_HANDLE_H_

#include <memory>

#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/tables/etm_tables_py.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor {
class TraceStorage;
namespace etm {

class Configuration;

enum EtmV4ConfigurationTableFlag : uint64_t {
  ETM_V4_CONFIGURATION_TABLE_FLAG_HAS_CYCLE_COUNT = 1ull << 0,
  ETM_V4_CONFIGURATION_TABLE_FLAG_TS_ENABLED = 1ull << 1,
};

class StorageHandle {
 public:
  explicit StorageHandle(TraceProcessorContext* context)
      : storage_(context->storage.get()) {}
  explicit StorageHandle(TraceStorage* storage) : storage_(storage) {}

  void StoreEtmV4Config(tables::EtmV4ConfigurationTable::Id id,
                        std::unique_ptr<Configuration> config);
  const Configuration& GetEtmV4Config(
      tables::EtmV4ConfigurationTable::Id) const;

  void StoreTrace(tables::EtmV4TraceTable::Id id, TraceBlobView trace);
  const TraceBlobView& GetTrace(tables::EtmV4TraceTable::Id id) const;

 private:
  TraceStorage* storage_;
};

}  // namespace etm
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_STORAGE_HANDLE_H_
