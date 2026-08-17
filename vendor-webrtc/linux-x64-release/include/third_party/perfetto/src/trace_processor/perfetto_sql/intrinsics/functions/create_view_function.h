/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_CREATE_VIEW_FUNCTION_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_CREATE_VIEW_FUNCTION_H_

#include <sqlite3.h>
#include <cstddef>

#include "perfetto/base/status.h"
#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/perfetto_sql/intrinsics/functions/sql_function.h"

namespace perfetto::trace_processor {

class PerfettoSqlEngine;

// Implementation of CREATE_VIEW_FUNCTION SQL function.
// See https://perfetto.dev/docs/analysis/metrics#metric-helper-functions for
// usage of this function.
struct CreateViewFunction : public SqlFunction {
  using Context = PerfettoSqlEngine;

  static constexpr bool kVoidReturn = true;

  static base::Status Run(Context* ctx,
                          size_t argc,
                          sqlite3_value** argv,
                          SqlValue& out,
                          Destructors&);
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_CREATE_VIEW_FUNCTION_H_
