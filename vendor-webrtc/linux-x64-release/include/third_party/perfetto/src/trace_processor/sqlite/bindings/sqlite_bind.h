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

#ifndef SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_BIND_H_
#define SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_BIND_H_

#include <sqlite3.h>  // IWYU pragma: export
#include <cstdint>
#include <memory>

namespace perfetto::trace_processor::sqlite::bind {

// This file contains wraps the SQLite functions which operate on stmt
// bindings and start with sqlite3_bind_*.

using PointerDestructor = void(void*);
inline int Pointer(sqlite3_stmt* stmt,
                   uint32_t N,
                   void* ptr,
                   const char* name,
                   PointerDestructor destructor) {
  return sqlite3_bind_pointer(stmt, static_cast<int>(N), ptr, name, destructor);
}

template <typename T>
inline int UniquePointer(sqlite3_stmt* stmt,
                         uint32_t N,
                         std::unique_ptr<T> ptr,
                         const char* name) {
  return sqlite3_bind_pointer(
      stmt, static_cast<int>(N), ptr.release(), name,
      [](void* tab) { std::unique_ptr<T>(static_cast<T*>(tab)); });
}

}  // namespace perfetto::trace_processor::sqlite::bind

#endif  // SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_BIND_H_
