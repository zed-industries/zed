/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_SQL_FUNCTION_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_SQL_FUNCTION_H_

#include <sqlite3.h>
#include <memory>

#include "perfetto/base/status.h"
#include "perfetto/trace_processor/basic_types.h"

namespace perfetto {
namespace trace_processor {

// Prototype for a C++ function which can be registered with SQLite.
//
// Usage
//
// Define a subclass of this struct as follows:
// struct YourFunction : public SqlFunction {
//   // Optional if you want a custom context object (i.e. an object
//   // passed in at registration time which will be passed to Run on
//   // every invocation)
//   struct YourContext { /* define context fields here */ };
//
//   static base::Status Run(/* see parameters below */) {
//     /* function body here */
//   }
//
//   static base::Status Cleanup(/* see parameters below */) {
//     /* function body here */
//   }
// }
//
// Then, register this function with SQLite using RegisterFunction (see below);
// you'll likely want to do this in TraceProcessorImpl:
// RegisterFunction<YourFunction>(/* see arguments below */)
struct SqlFunction {
  // The type of the context object which will be passed to the function.
  // Can be redefined in any sub-classes to override the context.
  using Context = void;

  // Indicates whether this function is "void" (i.e. doesn't actually want
  // to return a value). While the function will still return null in SQL
  // (because SQLite does not actually allow null functions), for accounting
  // purposes, this null will be ignored when verifying whether this statement
  // has any output.
  // Can be redefined in any sub-classes to override it.
  // If this is set to true, subclasses must not modify |out| or |destructors|.
  static constexpr bool kVoidReturn = false;

  // Struct which holds destructors for strings/bytes returned from the
  // function. Passed as an argument to |Run| to allow implementations to
  // override the destructors.
  struct Destructors {
    // This matches SQLITE_TRANSIENT constant which we cannot use because it
    // expands to a C-style cast, causing compiler warnings.
    sqlite3_destructor_type string_destructor =
        reinterpret_cast<sqlite3_destructor_type>(-1);
    sqlite3_destructor_type bytes_destructor =
        reinterpret_cast<sqlite3_destructor_type>(-1);
  };

  // The function which will be executed with the arguments from SQL.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  //
  // |ctx|:         the context object passed at registration time.
  // |argc|:        number of arguments.
  // |argv|:        arguments to the function.
  // |out|:         the return value of the function.
  // |destructors|: destructors for string/bytes return values.
  static base::Status Run(Context* ctx,
                          size_t argc,
                          sqlite3_value** argv,
                          SqlValue& out,
                          Destructors& destructors);

  // Executed after the result from |Run| is reported to SQLite.
  // Allows implementations to verify post-conditions without needing to worry
  // about overwriting return types.
  //
  // Implementations do not need to define this function; a default no-op
  // implementation will be used in this case.
  static base::Status VerifyPostConditions(Context*);

  // Executed after the result from |Run| is reported to SQLite.
  // Allows any pending state to be cleaned up post-copy of results by SQLite:
  // this function will be called even if |Run| or |PostRun| returned errors.
  //
  // Implementations do not need to define this function; a default no-op
  // implementation will be used in this case.
  static void Cleanup(Context*);
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_SQL_FUNCTION_H_
