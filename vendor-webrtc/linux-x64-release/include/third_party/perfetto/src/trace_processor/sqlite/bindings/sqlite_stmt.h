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

#ifndef SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_STMT_H_
#define SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_STMT_H_

#include <sqlite3.h>  // IWYU pragma: export

namespace perfetto::trace_processor::sqlite::stmt {

// This file contains wraps the SQLite functions which operate on sqlite3_stmt
// objects.

inline int Reset(sqlite3_stmt* stmt) {
  return sqlite3_reset(stmt);
}

}  // namespace perfetto::trace_processor::sqlite::stmt

#endif  // SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_STMT_H_
