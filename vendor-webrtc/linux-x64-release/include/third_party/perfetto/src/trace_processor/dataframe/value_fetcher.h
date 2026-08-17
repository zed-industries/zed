/*
 * Copyright (C) 2025 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_VALUE_FETCHER_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_VALUE_FETCHER_H_

#include <cstddef>
#include <cstdint>

namespace perfetto::trace_processor::dataframe {

// Fetcher for values from an aribtrary indexed source. The meaning of the index
// in each of the *Value methods varies depending on where this class is used.
//
// Note: all the methods in this class are declared but not defined as this
// class is simply an interface which needs to be subclassed and all
// methods/variables implemented. The methods are intentionally not defined to
// cause link errors if not implemented.
struct ValueFetcher {
  using Type = int;
  static const Type kInt64;
  static const Type kDouble;
  static const Type kString;
  static const Type kNull;

  // Fetches an int64_t value at the given index.
  int64_t GetInt64Value(uint32_t);
  // Fetches a double value at the given index.
  double GetDoubleValue(uint32_t);
  // Fetches a string value at the given index.
  const char* GetStringValue(uint32_t);
  // Fetches the type of the value at the given index.
  Type GetValueType(uint32_t);
};

}  // namespace perfetto::trace_processor::dataframe

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_VALUE_FETCHER_H_
