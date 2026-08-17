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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_GRAMMAR_PERFETTOSQL_GRAMMAR_INTERFACE_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_GRAMMAR_PERFETTOSQL_GRAMMAR_INTERFACE_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdio.h>

#include "src/trace_processor/perfetto_sql/grammar/perfettosql_grammar.h"

#undef NDEBUG

#ifdef __cplusplus
namespace perfetto::trace_processor {
#endif

// Basic token structure containing source information.
struct PerfettoSqlToken {
  const char* ptr;  // Pointer to start of token in source
  size_t n;         // Length of token
};

// Overall structure to hold the parsing state.
struct PerfettoSqlParserState;

// List structure for arguments
struct PerfettoSqlArgumentList;

// List structure for indexed columns
struct PerfettoSqlIndexedColumnList;

// List structure for macro arguments
struct PerfettoSqlMacroArgumentList;

// Return type for functions.
struct PerfettoSqlFnReturnType;

// Parser allocation/deallocation functions
void* PerfettoSqlParseAlloc(void* (*allocator)(size_t),
                            struct PerfettoSqlParserState*);
void PerfettoSqlParse(void* parser,
                      int token_type,
                      struct PerfettoSqlToken token);
void PerfettoSqlParseFree(void* parser, void (*free_fn)(void*));

// Error handling
void OnPerfettoSqlSyntaxError(struct PerfettoSqlParserState*,
                              struct PerfettoSqlToken*);

// Statement callbacks
void OnPerfettoSqlCreateFunction(struct PerfettoSqlParserState*,
                                 int replace,
                                 struct PerfettoSqlToken* name,
                                 struct PerfettoSqlArgumentList* args,
                                 struct PerfettoSqlFnReturnType* returns,
                                 struct PerfettoSqlToken* body_start,
                                 struct PerfettoSqlToken* body_end);
void OnPerfettoSqlCreateTable(struct PerfettoSqlParserState*,
                              int replace,
                              struct PerfettoSqlToken* name,
                              struct PerfettoSqlToken* table_impl,
                              struct PerfettoSqlArgumentList* args,
                              struct PerfettoSqlToken* body_start,
                              struct PerfettoSqlToken* body_end);
void OnPerfettoSqlCreateView(struct PerfettoSqlParserState*,
                             int replace,
                             struct PerfettoSqlToken* create_token,
                             struct PerfettoSqlToken* name,
                             struct PerfettoSqlArgumentList* args,
                             struct PerfettoSqlToken* body_start,
                             struct PerfettoSqlToken* body_end);
void OnPerfettoSqlCreateMacro(struct PerfettoSqlParserState*,
                              int replace,
                              struct PerfettoSqlToken* name,
                              struct PerfettoSqlMacroArgumentList* args,
                              struct PerfettoSqlToken* returns,
                              struct PerfettoSqlToken* body_start,
                              struct PerfettoSqlToken* body_end);
void OnPerfettoSqlCreateIndex(struct PerfettoSqlParserState*,
                              int replace,
                              struct PerfettoSqlToken* create_token,
                              struct PerfettoSqlToken* name,
                              struct PerfettoSqlToken* table_name,
                              struct PerfettoSqlIndexedColumnList* cols);
void OnPerfettoSqlInclude(struct PerfettoSqlParserState*,
                          struct PerfettoSqlToken*);
void OnPerfettoSqlDropIndex(struct PerfettoSqlParserState*,
                            struct PerfettoSqlToken* name,
                            struct PerfettoSqlToken* table_name);

struct PerfettoSqlArgumentList* OnPerfettoSqlCreateOrAppendArgument(
    struct PerfettoSqlParserState* state,
    struct PerfettoSqlArgumentList* list,
    struct PerfettoSqlToken* name,
    struct PerfettoSqlToken* type);
void OnPerfettoSqlFreeArgumentList(struct PerfettoSqlParserState*,
                                   struct PerfettoSqlArgumentList*);

struct PerfettoSqlIndexedColumnList* OnPerfettoSqlCreateOrAppendIndexedColumn(
    struct PerfettoSqlIndexedColumnList* list,
    struct PerfettoSqlToken* col);
void OnPerfettoSqlFreeIndexedColumnList(struct PerfettoSqlParserState*,
                                        struct PerfettoSqlIndexedColumnList*);

struct PerfettoSqlMacroArgumentList* OnPerfettoSqlCreateOrAppendMacroArgument(
    struct PerfettoSqlParserState* state,
    struct PerfettoSqlMacroArgumentList* list,
    struct PerfettoSqlToken* name,
    struct PerfettoSqlToken* type);
void OnPerfettoSqlFreeMacroArgumentList(struct PerfettoSqlParserState*,
                                        struct PerfettoSqlMacroArgumentList*);

struct PerfettoSqlFnReturnType* OnPerfettoSqlCreateScalarReturnType(
    struct PerfettoSqlToken* type);
struct PerfettoSqlFnReturnType* OnPerfettoSqlCreateTableReturnType(
    struct PerfettoSqlArgumentList* args);
void OnPerfettoSqlFnFreeReturnType(struct PerfettoSqlParserState*,
                                   struct PerfettoSqlFnReturnType* type);

#ifdef __cplusplus
}  // namespace perfetto::trace_processor
#endif

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_GRAMMAR_PERFETTOSQL_GRAMMAR_INTERFACE_H_
