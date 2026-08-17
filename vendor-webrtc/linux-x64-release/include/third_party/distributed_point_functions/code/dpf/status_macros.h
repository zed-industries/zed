/*
 * Copyright 2021 Google LLC
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

#ifndef DISTRIBUTED_POINT_FUNCTIONS_DPF_UTIL_STATUS_MACROS_H_
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_UTIL_STATUS_MACROS_H_

// Helper macro that checks if the right hand side (rexpression) evaluates to a
// StatusOr with Status OK, and if so assigns the value to the value on the left
// hand side (lhs), otherwise returns the error status. Example:
//   DPF_ASSIGN_OR_RETURN(lhs, rexpression);
#define DPF_ASSIGN_OR_RETURN(lhs, rexpr) \
  DPF_ASSIGN_OR_RETURN_IMPL_(            \
      DPF_STATUS_MACROS_IMPL_CONCAT_(_status_or_value, __LINE__), lhs, rexpr)

// Internal helper.
#define DPF_ASSIGN_OR_RETURN_IMPL_(statusor, lhs, rexpr) \
  auto statusor = (rexpr);                               \
  if (ABSL_PREDICT_FALSE(!statusor.ok())) {              \
    return std::move(statusor).status();                 \
  }                                                      \
  lhs = std::move(statusor).value()

// Internal helper for concatenating macro values.
#define DPF_STATUS_MACROS_IMPL_CONCAT_INNER_(x, y) x##y
#define DPF_STATUS_MACROS_IMPL_CONCAT_(x, y) \
  DPF_STATUS_MACROS_IMPL_CONCAT_INNER_(x, y)

#define DPF_RETURN_IF_ERROR(expr)                                              \
  DPF_RETURN_IF_ERROR_IMPL_(DPF_STATUS_MACROS_IMPL_CONCAT_(_status, __LINE__), \
                            expr)

#define DPF_RETURN_IF_ERROR_IMPL_(status, expr) \
  auto status = (expr);                         \
  if (ABSL_PREDICT_FALSE(!status.ok())) {       \
    return status;                              \
  }

#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_UTIL_STATUS_MACROS_H_
