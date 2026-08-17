/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in, software
 * distributed under the License is distributed on an "AS IS",
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_MODULE_H_
#define SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_MODULE_H_

#include <sqlite3.h>

#include "perfetto/base/logging.h"

namespace perfetto::trace_processor::sqlite {

// Prototype for a virtual table (vtab) module which can be registered with
// SQLite.
//
// See https://www.sqlite.org/vtab.html for how to implement this class.
template <typename Impl>
struct Module {
  // Specifies the type of module: implementations can override this field by
  // declaring and defining it.
  //
  // Specifying this to kCreateOnly requires that the |Create| and |Destroy|
  // functions are defined.
  //
  // See the SQLite documentation on what these types mean.
  static constexpr enum { kEponymousOnly, kCreateOnly } kType = kCreateOnly;

  // Specifies whether this table is supports making changes to it:
  // implementations can override this field by declaring and defining it.
  //
  // Setting this to true requires the |Update| function to be defined.
  static constexpr bool kSupportsWrites = true;

  // Specifies whether this table supports overloading functions:
  // implementations can override this field by declaring and defining it.
  //
  // Setting this to true requires that the |FindFunction| function is defined.
  static constexpr bool kDoesOverloadFunctions = true;

  // Specifies the type of context for the module. Implementations should define
  // this type to match the context type which is expected to be passed into
  // |sqlite3_create_module|.
  using Context = void;

  // Specifies the type for the vtab created by this module.
  //
  // Implementations should define this type to match the vtab type they use in
  // |Create| and |Connect|.
  using Vtab = sqlite3_vtab;

  // Specifies the type for the cursor created by this module.
  //
  // Implementations should define this type to match the cursor type they use
  // in |Open| and |Close|.
  using Cursor = sqlite3_vtab_cursor;

  // Creates a new instance of a virtual table and its backing storage.
  //
  // Implementations MUST define this function themselves if
  // |kType| == |kCreateOnly|; this function is declared but *not* defined so
  // linker errors will be thrown if not defined.
  static int Create(sqlite3*,
                    void*,
                    int,
                    const char* const*,
                    sqlite3_vtab**,
                    char**);

  // Destroys the virtual table and its backing storage.
  //
  // Implementations MUST define this function themselves if
  // |kType| == |kCreateOnly|; this function is declared but *not* defined so
  // linker errors will be thrown if not defined.
  static int Destroy(sqlite3_vtab*);

  // Creates a new instance of the virtual table, connecting to existing
  // backing storage.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static int Connect(sqlite3*,
                     void*,
                     int,
                     const char* const*,
                     sqlite3_vtab**,
                     char**);

  // Destroys the virtual table but *not* its backing storage.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static int Disconnect(sqlite3_vtab*);

  // Specifies filtering and cost information for the query planner.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static int BestIndex(sqlite3_vtab*, sqlite3_index_info*);

  // Opens a cursor into the given vtab.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static int Open(sqlite3_vtab*, sqlite3_vtab_cursor**);

  // Closes the cursor.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static int Close(sqlite3_vtab_cursor*);

  // Resets this cursor to filter rows matching the provided set of filter
  // constraints and order by clauses.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static int Filter(sqlite3_vtab_cursor*,
                    int,
                    const char*,
                    int,
                    sqlite3_value**);

  // Forwards the cursor to point to the next row.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static int Next(sqlite3_vtab_cursor*);

  // Returns 1 if the cursor has reached its end or 0 otherwise.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static int Eof(sqlite3_vtab_cursor*);

  // Returns the value column at the given index for the current row the cursor
  // points to.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static int Column(sqlite3_vtab_cursor*, sqlite3_context*, int);

  // Returns the rowid for the current row.
  //
  // Implementations MUST define this function themselves; this function is
  // declared but *not* defined so linker errors will be thrown if not defined.
  static int Rowid(sqlite3_vtab_cursor*, sqlite_int64*);

  // Inserts/deletes/updates one row.
  //
  // Implementations MUST define this function themselves if
  // |kSupportsWrites| == |true|; this function is declared but *not* defined so
  // linker errors will be thrown if not defined.
  static int Update(sqlite3_vtab*, int, sqlite3_value**, sqlite_int64*);

  // Overloads a function with the given name when executed with a vtab column
  // as the first argument.
  //
  // Implementations MUST define this function themselves if
  // |kDoesOverloadFunctions| == |true|; this function is declared but *not*
  // defined so linker errors will be thrown if not defined.
  using FindFunctionFn = void(sqlite3_context*, int, sqlite3_value**);
  static int FindFunction(sqlite3_vtab*,
                          int,
                          const char*,
                          FindFunctionFn**,
                          void**);

  // Helper function to cast the module context pointer to the correct type.
  static auto GetContext(void* ctx) {
    return static_cast<typename Impl::Context*>(ctx);
  }

  // Helper function to cast the vtab pointer to the correct type.
  static auto GetVtab(sqlite3_vtab* vtab) {
    return static_cast<typename Impl::Vtab*>(vtab);
  }

  // Helper function to cast the cursor pointer to the correct type.
  static auto GetCursor(sqlite3_vtab_cursor* cursor) {
    return static_cast<typename Impl::Cursor*>(cursor);
  }

  // Returns sqlite3_module object corresponding to the module. Used to pass
  // information about this module to SQLite.
  static constexpr sqlite3_module CreateModule() {
    sqlite3_module module{};
    module.xBestIndex = &Impl::BestIndex;
    module.xOpen = &Impl::Open;
    module.xClose = &Impl::Close;
    module.xFilter = &Impl::Filter;
    module.xNext = &Impl::Next;
    module.xEof = &Impl::Eof;
    module.xColumn = &Impl::Column;
    module.xRowid = &Impl::Rowid;
    if constexpr (Impl::kType == kCreateOnly) {
      module.xCreate = &Impl::Create;
      module.xDestroy = &Impl::Destroy;
      module.xConnect = &Impl::Connect;
      module.xDisconnect = &Impl::Disconnect;
    } else {
      module.xCreate = nullptr;
      module.xDestroy = [](sqlite3_vtab*) -> int {
        PERFETTO_FATAL("Should not be reachable");
      };
      module.xConnect = &Impl::Connect;
      module.xDisconnect = &Impl::Disconnect;
    }
    if constexpr (Impl::kSupportsWrites) {
      module.xUpdate = &Impl::Update;
    }
    if constexpr (Impl::kDoesOverloadFunctions) {
      module.xFindFunction = &Impl::FindFunction;
    }
    return module;
  }
};

}  // namespace perfetto::trace_processor::sqlite

#endif  // SRC_TRACE_PROCESSOR_SQLITE_BINDINGS_SQLITE_MODULE_H_
