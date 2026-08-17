/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_TRACE_SUMMARY_SUMMARY_H_
#define SRC_TRACE_PROCESSOR_TRACE_SUMMARY_SUMMARY_H_

#include <cstdint>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/trace_processor/basic_types.h"
#include "perfetto/trace_processor/trace_processor.h"
#include "src/trace_processor/util/descriptors.h"

namespace perfetto::trace_processor::summary {

// Computes a set of v2 metrics.
//
// See the documentation on TraceProcessor: this is just a 1:1 implementation of
// that API.
base::Status Summarize(TraceProcessor* processor,
                       const DescriptorPool& pool,
                       const TraceSummaryComputationSpec& computation,
                       const std::vector<TraceSummarySpecBytes>& specs,
                       std::vector<uint8_t>* output,
                       const TraceSummaryOutputSpec& output_spec);

}  // namespace perfetto::trace_processor::summary

#endif  // SRC_TRACE_PROCESSOR_TRACE_SUMMARY_SUMMARY_H_
