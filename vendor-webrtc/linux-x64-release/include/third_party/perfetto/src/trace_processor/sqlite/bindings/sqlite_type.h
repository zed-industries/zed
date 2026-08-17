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

#ifndef SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_TYPE_H_
#define SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_TYPE_H_

#include <sqlite3.h>  // IWYU pragma: export

namespace perfetto::trace_processor::sqlite {

enum class Type : int {
  kNull = SQLITE_NULL,
  kInteger = SQLITE_INTEGER,
  kText = SQLITE_TEXT,
  kFloat = SQLITE_FLOAT,
  kBlob = SQLITE_BLOB,
};

}  // namespace perfetto::trace_processor::sqlite

#endif  // SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_TYPE_H_
