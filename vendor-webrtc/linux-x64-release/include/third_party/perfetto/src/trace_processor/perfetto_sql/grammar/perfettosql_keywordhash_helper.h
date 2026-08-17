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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_GRAMMAR_PERFETTOSQL_KEYWORDHASH_HELPER_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_GRAMMAR_PERFETTOSQL_KEYWORDHASH_HELPER_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <assert.h>
#include <ctype.h>

#include "src/trace_processor/perfetto_sql/grammar/perfettosql_grammar.h"

typedef unsigned char u8;

#define SQLITE_OK 0
#define SQLITE_ERROR 1
#define SQLITE_ASCII 1

static inline int charMap(char c) {
  return tolower(c);
}

static inline void testcase(int X) {}

#ifdef __cplusplus
}
#endif

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_GRAMMAR_PERFETTOSQL_KEYWORDHASH_HELPER_H_
