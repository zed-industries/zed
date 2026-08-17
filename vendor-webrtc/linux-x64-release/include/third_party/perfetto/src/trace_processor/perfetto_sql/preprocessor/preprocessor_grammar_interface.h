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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_PREPROCESSOR_PREPROCESSOR_GRAMMAR_INTERFACE_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_PREPROCESSOR_PREPROCESSOR_GRAMMAR_INTERFACE_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdio.h>

#include "src/trace_processor/perfetto_sql/preprocessor/preprocessor_grammar.h"

#undef NDEBUG

#ifdef __cplusplus
namespace perfetto::trace_processor {
namespace {
#endif

struct PreprocessorGrammarState;

struct PreprocessorGrammarToken {
  const char* ptr;
  size_t n;
  int major;
};

struct PreprocessorGrammarTokenBounds {
  struct PreprocessorGrammarToken start;
  struct PreprocessorGrammarToken end;
};

struct PreprocessorGrammarApplyList;

void* PreprocessorGrammarParseAlloc(void* (*)(size_t),
                                    struct PreprocessorGrammarState*);
void PreprocessorGrammarParse(void* parser,
                              int,
                              struct PreprocessorGrammarToken);
void PreprocessorGrammarParseFree(void* parser, void (*)(void*));
void PreprocessorGrammarParseTrace(FILE*, char*);

void OnPreprocessorSyntaxError(struct PreprocessorGrammarState*,
                               struct PreprocessorGrammarToken*);
void OnPreprocessorApply(struct PreprocessorGrammarState*,
                         struct PreprocessorGrammarToken* name,
                         struct PreprocessorGrammarToken* join,
                         struct PreprocessorGrammarToken* prefix,
                         struct PreprocessorGrammarApplyList*,
                         struct PreprocessorGrammarApplyList*);
void OnPreprocessorVariable(struct PreprocessorGrammarState*,
                            struct PreprocessorGrammarToken* var);
void OnPreprocessorMacroId(struct PreprocessorGrammarState*,
                           struct PreprocessorGrammarToken* name);
void OnPreprocessorMacroArg(struct PreprocessorGrammarState*,
                            struct PreprocessorGrammarTokenBounds*);
void OnPreprocessorMacroEnd(struct PreprocessorGrammarState*,
                            struct PreprocessorGrammarToken* name,
                            struct PreprocessorGrammarToken* rp);
void OnPreprocessorEnd(struct PreprocessorGrammarState*);

struct PreprocessorGrammarApplyList* OnPreprocessorCreateApplyList();
struct PreprocessorGrammarApplyList* OnPreprocessorAppendApplyList(
    struct PreprocessorGrammarApplyList*,
    struct PreprocessorGrammarTokenBounds*);
void OnPreprocessorFreeApplyList(struct PreprocessorGrammarState*,
                                 struct PreprocessorGrammarApplyList*);

#ifdef __cplusplus
}
}
}
#endif

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_PREPROCESSOR_PREPROCESSOR_GRAMMAR_INTERFACE_H_
