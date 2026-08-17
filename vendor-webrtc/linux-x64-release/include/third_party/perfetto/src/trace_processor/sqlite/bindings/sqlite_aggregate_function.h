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

#ifndef SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_AGGREGATE_FUNCTION_H_
#define SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_AGGREGATE_FUNCTION_H_

#include <sqlite3.h>

#include "perfetto/ext/base/scoped_file.h"

namespace perfetto::trace_processor {

// Prototype for an aggregate context which can be fetched from an aggregate
// function in SQLite.
template <typename Impl>
struct SqliteAggregateContext {
  static int ScopedDestructor(Impl* impl) {
    if (impl) {
      impl->~Impl();
    }
    return 0;
  }
  using ScopedContext = base::ScopedResource<Impl*, ScopedDestructor, nullptr>;

  // Function which should be called from |Step| to retrieve the context.
  static Impl& GetOrCreateContextForStep(sqlite3_context* ctx) {
    // Fast path: the context is already allocated and initialized. Just fetch
    // it (by passing 0 to SQLite to suppress any allocations) and return it.
    if (auto* ptr = sqlite3_aggregate_context(ctx, 0); ptr) {
      return *static_cast<Impl*>(ptr);
    }

    // Slow path: we need to actually allocate the memory and then initialize
    // it.
    auto* raw_ptr = sqlite3_aggregate_context(ctx, sizeof(Impl));
    new (raw_ptr) Impl();
    return *static_cast<Impl*>(raw_ptr);
  }

  // Function which should be called from |Final| to retrieve the context.
  // Returns null if no previous call to |Step| was made.
  static ScopedContext GetContextOrNullForFinal(sqlite3_context* ctx) {
    return ScopedContext(static_cast<Impl*>(sqlite3_aggregate_context(ctx, 0)));
  }
};

// Prototype for a aggregate function which can be registered with SQLite.
//
// See https://www.sqlite.org/c3ref/create_function.html for details on how to
// implement the methods of this class.
template <typename Impl>
struct SqliteAggregateFunction {
  // The type of the context object which will be passed to the function.
  // Can be redefined in any sub-classes to override the context.
  using UserDataContext = void;

  // The xStep function which will be executed by SQLite to add a row of values
  // to the aggregate.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static void Step(sqlite3_context*, int argc, sqlite3_value** argv);

  // The xFinal function which will be executed by SQLite to obtain the current
  // value of the aggregate *and* free all resources allocated by previous calls
  // to Step.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static void Final(sqlite3_context* ctx);

  // Returns the pointer to the user data structure which is passed when
  // creating the function.
  static auto GetUserData(sqlite3_context* ctx) {
    return static_cast<typename Impl::UserDataContext*>(sqlite3_user_data(ctx));
  }
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_AGGREGATE_FUNCTION_H_
