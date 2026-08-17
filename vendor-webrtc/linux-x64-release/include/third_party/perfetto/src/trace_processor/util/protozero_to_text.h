/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_UTIL_PROTOZERO_TO_TEXT_H_
#define SRC_TRACE_PROCESSOR_UTIL_PROTOZERO_TO_TEXT_H_

#include <cstdint>
#include <string>
#include <vector>

#include "perfetto/protozero/field.h"

namespace perfetto::trace_processor {

class DescriptorPool;

namespace protozero_to_text {

// If |new_lines_modes| == kIncludeNewLines, new lines will be used between
// fields, otherwise only a space will be used.
enum NewLinesMode {
  kIncludeNewLines = 0,
  kSkipNewLines,
};

// Given a protozero message |protobytes| which is of fully qualified name
// |type|, convert this into a text proto format string. All types used in
// message definition of |type| must be available in |pool|.
std::string ProtozeroToText(
    const DescriptorPool& pool,
    const std::string& type,
    protozero::ConstBytes protobytes,
    NewLinesMode new_lines_mode = NewLinesMode::kIncludeNewLines,
    uint32_t initial_indent_depth = 0);

std::string ProtozeroToText(const DescriptorPool& pool,
                            const std::string& type,
                            const std::vector<uint8_t>& protobytes,
                            NewLinesMode new_lines_mode);

}  // namespace protozero_to_text
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_UTIL_PROTOZERO_TO_TEXT_H_
