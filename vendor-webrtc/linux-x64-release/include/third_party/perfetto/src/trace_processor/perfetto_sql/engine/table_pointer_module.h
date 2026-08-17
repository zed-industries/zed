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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_ENGINE_TABLE_POINTER_MODULE_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_ENGINE_TABLE_POINTER_MODULE_H_

#include <array>
#include <cstdint>
#include <optional>

#include "src/trace_processor/db/table.h"
#include "src/trace_processor/sqlite/bindings/sqlite_module.h"

namespace perfetto::trace_processor {

// SQLite module which allows iteration over a table pointer (i.e. a instance of
// Table which is being directly passed in as a SQL value). This allows for a
// dynamic, schema-less iteration over table pointers. This is generally not
// possible as SQLite requires the schema to be defined upfront but this class
// works around that by having a fixed schema but then allowing "binding" table
// pointer columns to SQLite columns dynamically at query time.
//
// Example:
// ```
//  -- Renaming the static columns defined by this table to the particular
//  -- column names for this query.
//  SELECT c0 AS node_id, c1 AS parent_node_id
//  -- The call to this class
//  FROM __intrinsic_table_ptr((
//    -- An aggregate function which returns the table pointer we want to
//    -- iterate over.
//    SELECT __intrinsic_dfs(g.source_node_id, g.dest_node_id, $start_node_id)
//    FROM $graph_table g
//  ))
//  -- Informs this class about which SQLite column corresponds to which
//  -- SQLite column. The SQLite columns bindings should be dense starting from
//  -- 0.
//  WHERE __intrinsic_table_ptr_bind(c0, 'node_id')
//    AND __intrinsic_table_ptr_bind(c1, 'parent_node_id')
// ```
//
// Note: this class is *not* intended to be used directly by end users. It is
// a building block intended for use by very low-level macros in the standard
// library.
struct TablePointerModule : sqlite::Module<TablePointerModule> {
  static constexpr int kBindConstraint = SQLITE_INDEX_CONSTRAINT_FUNCTION + 1;
  static constexpr int kBindableColumnCount = 16;
  static constexpr int kTableColumnIndex = kBindableColumnCount;
  static constexpr int kRowColumnIndex = kBindableColumnCount + 1;
  static constexpr int kTableArgvIndex = 1;
  static constexpr int kBoundColumnArgvOffset = 2;

  using Context = void;
  struct Vtab : sqlite::Module<TablePointerModule>::Vtab {};
  struct Cursor : sqlite::Module<TablePointerModule>::Cursor {
    const Table* table = nullptr;
    std::array<uint32_t, kBindableColumnCount> bound_col_to_table_index{};
    uint32_t col_count = 0;
    std::optional<Table::Iterator> iterator;
  };

  static constexpr auto kType = kEponymousOnly;
  static constexpr bool kSupportsWrites = false;

  static int Connect(sqlite3*,
                     void*,
                     int,
                     const char* const*,
                     sqlite3_vtab**,
                     char**);
  static int Disconnect(sqlite3_vtab*);

  static int BestIndex(sqlite3_vtab*, sqlite3_index_info*);

  static int Open(sqlite3_vtab*, sqlite3_vtab_cursor**);
  static int Close(sqlite3_vtab_cursor*);

  static int Filter(sqlite3_vtab_cursor*,
                    int,
                    const char*,
                    int,
                    sqlite3_value**);
  static int Next(sqlite3_vtab_cursor*);
  static int Eof(sqlite3_vtab_cursor*);
  static int Column(sqlite3_vtab_cursor*, sqlite3_context*, int);
  static int Rowid(sqlite3_vtab_cursor*, sqlite_int64*);

  static int FindFunction(sqlite3_vtab*,
                          int,
                          const char*,
                          FindFunctionFn**,
                          void**);

  // This needs to happen at the end as it depends on the functions
  // defined above.
  static constexpr sqlite3_module kModule = CreateModule();
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_ENGINE_TABLE_POINTER_MODULE_H_
