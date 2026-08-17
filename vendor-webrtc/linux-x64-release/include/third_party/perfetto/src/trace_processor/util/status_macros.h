/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_UTIL_STATUS_MACROS_H_
#define SRC_TRACE_PROCESSOR_UTIL_STATUS_MACROS_H_

#include "perfetto/trace_processor/status.h"

// Evaluates |expr|, which should return a base::Status. If the status is an
// error status, returns the status from the current function.
#define RETURN_IF_ERROR(expr)                           \
  do {                                                  \
    base::Status status_macro_internal_status = (expr); \
    if (!status_macro_internal_status.ok())             \
      return status_macro_internal_status;              \
  } while (0)

#define PERFETTO_INTERNAL_CONCAT_IMPL(x, y) x##y
#define PERFETTO_INTERNAL_MACRO_CONCAT(x, y) PERFETTO_INTERNAL_CONCAT_IMPL(x, y)

// Evalues |rhs| which should return a base::StatusOr<T> and assigns this
// to |lhs|. If the status is an error status, returns the status from the
// current function.
#define ASSIGN_OR_RETURN(lhs, rhs)                                   \
  PERFETTO_INTERNAL_MACRO_CONCAT(auto status_or, __LINE__) = rhs;    \
  RETURN_IF_ERROR(                                                   \
      PERFETTO_INTERNAL_MACRO_CONCAT(status_or, __LINE__).status()); \
  lhs = std::move(PERFETTO_INTERNAL_MACRO_CONCAT(status_or, __LINE__).value())

#endif  // SRC_TRACE_PROCESSOR_UTIL_STATUS_MACROS_H_
