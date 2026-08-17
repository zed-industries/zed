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

#ifndef SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_WINDOW_FUNCTION_H_
#define SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_WINDOW_FUNCTION_H_

struct sqlite3_context;
struct sqlite3_value;

namespace perfetto::trace_processor {

// Prototype for a window function which can be registered with SQLite.
//
// See https://www.sqlite.org/windowfunctions.html#udfwinfunc for details on how
// to implement the methods of this class.
class SqliteWindowFunction {
 public:
  // The type of the context object which will be passed to the function.
  // Can be redefined in any sub-classes to override the context.
  using Context = void;

  // The xStep function which will be executed by SQLite to add a row of values
  // to the current window.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static void Step(sqlite3_context*, int argc, sqlite3_value** argv);

  // The xStep function which will be executed by SQLite to remove a row of
  // values from the current window.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static void Inverse(sqlite3_context* ctx, int argc, sqlite3_value** argv);

  // The xValue function which will be executed by SQLite to obtain the current
  // value of the aggregate.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static void Value(sqlite3_context* ctx);

  // The xInverse function which will be executed by SQLite to obtain the
  // current value of the aggregate *and* free all resources allocated by
  // previous calls to Step, Inverse and Value.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static void Final(sqlite3_context* ctx);
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_WINDOW_FUNCTION_H_
