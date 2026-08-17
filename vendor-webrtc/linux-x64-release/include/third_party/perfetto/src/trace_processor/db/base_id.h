/*
 * Copyright (C) 2022 The Android Open Source Project
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

#include <stdint.h>
#include <type_traits>

#ifndef SRC_TRACE_PROCESSOR_DB_BASE_ID_H_
#define SRC_TRACE_PROCESSOR_DB_BASE_ID_H_

namespace perfetto {
namespace trace_processor {

// Id type which can be used as a base for strongly typed ids.
// TypedColumn has support for storing descendents of BaseId seamlessly
// in a Column.
struct BaseId {
  BaseId() = default;
  explicit constexpr BaseId(uint32_t v) : value(v) {}

  bool operator==(const BaseId& o) const { return o.value == value; }
  bool operator!=(const BaseId& o) const { return !(*this == o); }
  bool operator<(const BaseId& o) const { return value < o.value; }

  uint32_t value;
};
static_assert(std::is_trivially_destructible<BaseId>::value,
              "Inheritance used without trivial destruction");

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_DB_BASE_ID_H_
