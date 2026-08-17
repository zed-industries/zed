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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SPE_RECORD_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SPE_RECORD_PARSER_H_

#include <array>
#include <cstddef>
#include <cstdint>

#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/common/trace_parser.h"
#include "src/trace_processor/importers/common/virtual_memory_mapping.h"
#include "src/trace_processor/importers/perf/reader.h"
#include "src/trace_processor/importers/perf/spe.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/perf_tables_py.h"

namespace perfetto::trace_processor {
class TraceProcessorContext;
namespace perf_importer {

class SpeRecordParserImpl : public SpeRecordParser {
 public:
  explicit SpeRecordParserImpl(TraceProcessorContext* context);

  void ParseSpeRecord(int64_t, TraceBlobView) override;

 private:
  template <typename Enum>
  class CachedStringIdArray {
   public:
    static constexpr size_t size = static_cast<size_t>(Enum::kMax) + 1;
    explicit CachedStringIdArray() { cache_.fill(kNullStringId); }
    StringId& operator[](Enum e) { return cache_[static_cast<size_t>(e)]; }

   private:
    std::array<StringId, size> cache_;
  };

  struct InflightSpeRecord {
    std::optional<spe::InstructionVirtualAddress> instruction_address;
  };

  enum class OperationName {
    kOther,
    kSveVecOp,
    kLoad,
    kStore,
    kBranch,
    kUnknown,
    kMax = kUnknown
  };

  static const char* ToString(OperationName name);
  static const char* ToString(spe::ExceptionLevel el);
  static const char* ToString(spe::DataSource ds);

  StringId ToStringId(OperationName name);
  StringId ToStringId(spe::ExceptionLevel el);
  StringId ToStringId(spe::DataSource ds);

  void ReadShortPacket(spe::ShortHeader short_header);
  void ReadExtendedPacket(spe::ExtendedHeader extended_header);

  void ReadAddressPacket(spe::AddressIndex index);
  void ReadCounterPacket(spe::CounterIndex index);

  void ReadEventsPacket(spe::ShortHeader short_header);
  void ReadContextPacket(spe::ShortHeader short_header);
  void ReadOperationTypePacket(spe::ShortHeader short_header);
  void ReadDataSourcePacket(spe::ShortHeader short_header);

  uint64_t ReadPayload(spe::ShortHeader short_header);

  OperationName GetOperationName(spe::ShortHeader short_header,
                                 uint8_t payload) const;

  VirtualMemoryMapping* GetDummyMapping();

  TraceProcessorContext* const context_;
  CachedStringIdArray<OperationName> operation_name_strings_;
  CachedStringIdArray<spe::DataSource> data_source_strings_;
  CachedStringIdArray<spe::ExceptionLevel> exception_level_strings_;

  Reader reader_;
  tables::SpeRecordTable::Row inflight_row_;
  InflightSpeRecord inflight_record_;

  VirtualMemoryMapping* dummy_mapping_ = nullptr;
};

}  // namespace perf_importer
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SPE_RECORD_PARSER_H_
