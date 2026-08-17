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

#ifndef SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_COLUMN_H_
#define SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_COLUMN_H_

#include <sqlite3.h>  // IWYU pragma: export
#include <cstdint>

#include "src/trace_processor/sqlite/bindings/sqlite_type.h"

namespace perfetto::trace_processor::sqlite::column {

// This file contains wraps the SQLite functions which operate on stmt
// columns and start with sqlite3_column_*.

inline const char* Name(sqlite3_stmt* stmt, uint32_t N) {
  return sqlite3_column_name(stmt, static_cast<int>(N));
}

inline uint32_t Count(sqlite3_stmt* stmt) {
  return static_cast<uint32_t>(sqlite3_column_count(stmt));
}

inline sqlite::Type Type(sqlite3_stmt* stmt, uint32_t N) {
  return static_cast<sqlite::Type>(
      sqlite3_column_type(stmt, static_cast<int>(N)));
}

inline int64_t Int64(sqlite3_stmt* stmt, uint32_t N) {
  return sqlite3_column_int64(stmt, static_cast<int>(N));
}

inline const char* Text(sqlite3_stmt* stmt, uint32_t N) {
  return reinterpret_cast<const char*>(
      sqlite3_column_text(stmt, static_cast<int>(N)));
}

inline double Double(sqlite3_stmt* stmt, uint32_t N) {
  return sqlite3_column_double(stmt, static_cast<int>(N));
}

inline sqlite3_value* Value(sqlite3_stmt* stmt, uint32_t N) {
  return sqlite3_column_value(stmt, static_cast<int>(N));
}

using PointerDestructor = void(void*);
inline int BindPointer(sqlite3_stmt* stmt,
                       uint32_t N,
                       void* ptr,
                       const char* name,
                       PointerDestructor destructor) {
  return sqlite3_bind_pointer(stmt, static_cast<int>(N), ptr, name, destructor);
}

}  // namespace perfetto::trace_processor::sqlite::column

#endif  // SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_COLUMN_H_
