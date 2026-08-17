// Copyright (C) 2022 The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef INCLUDE_PERFETTO_TRACE_PROCESSOR_METATRACE_CONFIG_H_
#define INCLUDE_PERFETTO_TRACE_PROCESSOR_METATRACE_CONFIG_H_

#include <cstddef>
#include <cstdint>

namespace perfetto {
namespace trace_processor {
namespace metatrace {

enum MetatraceCategories : uint32_t {
  // Category for low-frequency events which provide a high-level timeline of
  // SQL query execution.
  QUERY_TIMELINE = 1 << 0,

  // Category for high-frequency events which provide details about SQL query
  // execution.
  QUERY_DETAILED = 1 << 1,

  // Category for high-frequency events which provide details about SQL function
  // calls.
  FUNCTION_CALL = 1 << 2,

  // Category for high-frequency events which provide details about the columnar
  // database operations.
  DB = 1 << 3,

  // Category for low-frequency events which provide a high-level timeline of
  // SQL query execution.
  API_TIMELINE = 1 << 4,

  // Alias for turning off all other categories.
  NONE = 0,

  // Alias for turning on all other categories.
  ALL = QUERY_TIMELINE | QUERY_DETAILED | FUNCTION_CALL | DB | API_TIMELINE,
};

struct MetatraceConfig {
  MetatraceConfig();

  MetatraceCategories categories = static_cast<MetatraceCategories>(
      MetatraceCategories::QUERY_TIMELINE | MetatraceCategories::API_TIMELINE);

  // Requested buffer size. The implemenation may choose to allocate a larger
  // buffer size for efficiency.
  size_t override_buffer_size = 0;
};

}  // namespace metatrace
}  // namespace trace_processor
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACE_PROCESSOR_METATRACE_CONFIG_H_
