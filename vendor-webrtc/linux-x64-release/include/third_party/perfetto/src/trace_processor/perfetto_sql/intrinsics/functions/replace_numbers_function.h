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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_REPLACE_NUMBERS_FUNCTION_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_REPLACE_NUMBERS_FUNCTION_H_

#include <sqlite3.h>

#include "perfetto/base/status.h"

namespace perfetto {
namespace trace_processor {

class PerfettoSqlEngine;
class TraceProcessorContext;

// Registers the following functions:
//
// __intrinsic_strip_hex(name STRING, min_repeated_digits LONG)
//
// Description:
//   Replaces hexadecimal sequences (with at least one digit) in a string with
//   "<num>" based on specified criteria.
//
// Parameters:
//   name STRING: The input string.
//   min_repeated_digits LONG: MINIMUM consecutive hex characters for
//   replacement.
//
// Replacement Criteria:
//   - Replaces hex/num sequences [0-9a-fA-F] with at least one occurrence of a
//     digit preceded by:
//      - Start of the string
//      - Defined prefix ("0x", "0X")
//      - Non-alphanumeric character
//   -  Replaces only sequences with length >= 'min_repeated_digits'
//
// Return Value:
//   The string with replaced hex sequences.
base::Status RegisterStripHexFunction(PerfettoSqlEngine* engine,
                                      TraceProcessorContext* context);

// Implementation of __intrinsic_strip_hex function
// Visible for testing
std::string SqlStripHex(std::string input, int64_t min_repeated_digits);
}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_REPLACE_NUMBERS_FUNCTION_H_
