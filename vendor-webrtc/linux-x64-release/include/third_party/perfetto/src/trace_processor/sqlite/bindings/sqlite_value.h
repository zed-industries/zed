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

#ifndef SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_VALUE_H_
#define SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_VALUE_H_

#include <sqlite3.h>  // IWYU pragma: export
#include <cstdint>

#include "src/trace_processor/sqlite/bindings/sqlite_type.h"

namespace perfetto::trace_processor::sqlite::value {

// This file contains wraps the sqlite3_value_* functions which extract values
// from sqlite3_value structs.

inline Type Type(sqlite3_value* value) {
  return static_cast<enum Type>(sqlite3_value_type(value));
}

inline bool IsNull(sqlite3_value* value) {
  return Type(value) == Type::kNull;
}

inline int64_t Int64(sqlite3_value* value) {
  return sqlite3_value_int64(value);
}

inline double Double(sqlite3_value* value) {
  return sqlite3_value_double(value);
}

inline const char* Text(sqlite3_value* value) {
  return reinterpret_cast<const char*>(sqlite3_value_text(value));
}

template <typename T>
inline T* Pointer(sqlite3_value* value, const char* type) {
  return static_cast<T*>(sqlite3_value_pointer(value, type));
}

}  // namespace perfetto::trace_processor::sqlite::value

#endif  // SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_VALUE_H_
