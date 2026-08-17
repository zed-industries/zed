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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ETM_V4_STREAM_DEMULTIPLEXER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ETM_V4_STREAM_DEMULTIPLEXER_H_

#include <memory>

#include "perfetto/ext/base/status_or.h"
namespace perfetto::trace_processor {
class TraceProcessorContext;
namespace perf_importer {
class AuxDataTokenizer;
struct AuxtraceInfoRecord;
}  // namespace perf_importer
namespace etm {

base::StatusOr<std::unique_ptr<perf_importer::AuxDataTokenizer>>
CreateEtmV4StreamDemultiplexer(TraceProcessorContext* context,
                               perf_importer::AuxtraceInfoRecord info);

}  // namespace etm
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ETM_V4_STREAM_DEMULTIPLEXER_H_
