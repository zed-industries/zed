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

#ifndef SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_RESULT_H_
#define SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_RESULT_H_

#include <sqlite3.h>  // IWYU pragma: export
#include <cstdint>
#include <memory>

namespace perfetto::trace_processor::sqlite::result {

// This file contains wraps the sqlite3_result_* functions which tell SQLite
// about the result of executing a function or calling a xColumn on a virtual
// table.

const auto kSqliteStatic = reinterpret_cast<sqlite3_destructor_type>(0);
const auto kSqliteTransient = reinterpret_cast<sqlite3_destructor_type>(-1);

inline void Null(sqlite3_context* ctx) {
  sqlite3_result_null(ctx);
}

inline void Long(sqlite3_context* ctx, int64_t res) {
  sqlite3_result_int64(ctx, res);
}

inline void Double(sqlite3_context* ctx, double res) {
  sqlite3_result_double(ctx, res);
}

inline void RawString(sqlite3_context* ctx,
                      const char* str,
                      int size,
                      sqlite3_destructor_type destructor) {
  sqlite3_result_text(ctx, str, size, destructor);
}
inline void RawString(sqlite3_context* ctx,
                      const char* str,
                      sqlite3_destructor_type destructor) {
  RawString(ctx, str, -1, destructor);
}
inline void StaticString(sqlite3_context* ctx, const char* str) {
  RawString(ctx, str, kSqliteStatic);
}
inline void TransientString(sqlite3_context* ctx, const char* str) {
  RawString(ctx, str, kSqliteTransient);
}

inline void RawBytes(sqlite3_context* ctx,
                     const void* bytes,
                     int size,
                     sqlite3_destructor_type destructor) {
  sqlite3_result_blob(ctx, bytes, size, destructor);
}
inline void StaticBytes(sqlite3_context* ctx, const void* bytes, int size) {
  RawBytes(ctx, bytes, size, kSqliteStatic);
}
inline void TransientBytes(sqlite3_context* ctx, const void* bytes, int size) {
  RawBytes(ctx, bytes, size, kSqliteTransient);
}

inline void Error(sqlite3_context* ctx, const char* error) {
  sqlite3_result_error(ctx, error, -1);
}

inline void Value(sqlite3_context* ctx, sqlite3_value* value) {
  sqlite3_result_value(ctx, value);
}

inline void RawPointer(sqlite3_context* ctx,
                       void* ptr,
                       const char* name,
                       sqlite3_destructor_type destructor) {
  sqlite3_result_pointer(ctx, ptr, name, destructor);
}
inline void StaticPointer(sqlite3_context* ctx, void* ptr, const char* name) {
  RawPointer(ctx, ptr, name, nullptr);
}
template <typename T>
inline void UniquePointer(sqlite3_context* ctx,
                          std::unique_ptr<T> ptr,
                          const char* name) {
  sqlite::result::RawPointer(ctx, ptr.release(), name, [](void* ptr) {
    std::unique_ptr<T>(static_cast<T*>(ptr));
  });
}

}  // namespace perfetto::trace_processor::sqlite::result

#endif  // SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_RESULT_H_
