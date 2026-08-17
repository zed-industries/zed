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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_SQL_VALUES_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_SQL_VALUES_H_

#include <cstdint>

#include "src/trace_processor/importers/etm/opencsd.h"
#include "src/trace_processor/tables/etm_tables_py.h"

namespace perfetto::trace_processor::etm {

struct InstructionRangeSqlValue {
  static constexpr const char kPtrType[] = "etm::InstructionRangeSqlValue";
  tables::EtmV4ConfigurationTable::Id config_id;
  ocsd_isa isa;
  uint64_t st_addr;
  const uint8_t* start;
  const uint8_t* end;
};

}  // namespace perfetto::trace_processor::etm

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_SQL_VALUES_H_
