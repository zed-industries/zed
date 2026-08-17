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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_TEXT_PERF_TEXT_SAMPLE_LINE_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_TEXT_PERF_TEXT_SAMPLE_LINE_PARSER_H_

#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>
#include <string_view>

namespace perfetto::trace_processor::perf_text_importer {

struct SampleLine {
  std::string comm;
  std::optional<uint32_t> pid;
  uint32_t tid;
  std::optional<uint32_t> cpu;
  int64_t ts;
};

// Given a single line of a perf text sample, parses it into its components and
// returns the result. If parsing was not possible, returns std::nullopt.
std::optional<SampleLine> ParseSampleLine(std::string_view line);

// Given a chunk of a trace file (starting at `ptr` and containing `size`
// bytes), returns whether the file is a perf text format trace.
bool IsPerfTextFormatTrace(const uint8_t* ptr, size_t size);

}  // namespace perfetto::trace_processor::perf_text_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_TEXT_PERF_TEXT_SAMPLE_LINE_PARSER_H_
